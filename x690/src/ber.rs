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
use asn1::utils::read_i64;
use asn1::{
    InstanceOf, ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME, ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME,
    ENUMERATED, INSTANCE_OF, NULL,
};
use std::borrow::Borrow;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use simdutf8::basic::from_utf8;
use crate::parsing::_parse_sequence;
use crate::{
    ber_cst, deconstruct, x690_write_bit_string_value, x690_write_bmp_string_value,
    x690_write_boolean_value, x690_write_character_string_value, x690_write_date_time_value,
    x690_write_date_value, x690_write_duration_value, x690_write_embedded_pdv_value,
    x690_write_enum_value, x690_write_external_value, x690_write_generalized_time_value,
    x690_write_integer_value, x690_write_object_descriptor_value,
    x690_write_object_identifier_value, x690_write_octet_string_value, x690_write_real_value,
    x690_write_relative_oid_value, x690_write_string_value, x690_write_tag,
    x690_write_time_of_day_value, x690_write_time_value, x690_write_universal_string_value,
    x690_write_utc_time_value, x690_write_utf8_string_value, X690Element, X690Encoding,
    X690_REAL_BASE10, X690_REAL_BASE_16, X690_REAL_BASE_2, X690_REAL_BASE_8, X690_REAL_BASE_MASK,
    X690_REAL_BINARY_SCALING_MASK, X690_REAL_EXPONENT_FORMAT_1_OCTET,
    X690_REAL_EXPONENT_FORMAT_2_OCTET, X690_REAL_EXPONENT_FORMAT_3_OCTET,
    X690_REAL_EXPONENT_FORMAT_MASK, X690_REAL_EXPONENT_FORMAT_VAR_OCTET, X690_REAL_NR1,
    X690_REAL_NR2, X690_REAL_NR3, X690_REAL_SPECIAL, X690_SPECIAL_REAL_MINUS_INFINITY,
    X690_SPECIAL_REAL_MINUS_ZERO, X690_SPECIAL_REAL_NOT_A_NUMBER, X690_SPECIAL_REAL_PLUS_INFINITY,
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
    // Intentionally not validating this. Most integers are small and correct.
    // If they have padding, its obvious how to handle that.
    Ok(Vec::from(value_bytes))
}

pub fn ber_decode_i64_value(value_bytes: ByteSlice) -> ASN1Result<i64> {
    match read_i64(value_bytes) {
        Ok(v) => Ok(v),
        Err(_) => Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
    }
}

pub fn ber_decode_enum_value(value_bytes: ByteSlice) -> ASN1Result<ENUMERATED> {
    ber_decode_i64_value(value_bytes)
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
    // In pre-allocating, we assume the average OID arc consumes two bytes.
    let mut nodes: Vec<u32> = Vec::with_capacity(len << 1);
    nodes.push(arc1);
    nodes.push(arc2);
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
    Ok(OBJECT_IDENTIFIER(nodes))
}

pub fn ber_decode_relative_oid_value(value_bytes: ByteSlice) -> ASN1Result<RELATIVE_OID> {
    let len = value_bytes.len();
    // In pre-allocating, we assume the average OID arc consumes two bytes.
    let mut nodes: Vec<u32> = Vec::with_capacity(len << 1);
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
    Ok(RELATIVE_OID(nodes))
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
        X690_REAL_SPECIAL => match value_bytes[0] & 0b0011_1111 {
            X690_SPECIAL_REAL_PLUS_INFINITY => return Ok(f64::INFINITY),
            X690_SPECIAL_REAL_MINUS_INFINITY => return Ok(f64::NEG_INFINITY),
            X690_SPECIAL_REAL_NOT_A_NUMBER => return Ok(f64::NAN),
            X690_SPECIAL_REAL_MINUS_ZERO => return Ok(-0.000000),
            _ => return Err(ASN1Error::new(ASN1ErrorCode::urecognized_real_format)),
        },
        X690_REAL_BASE10 => {
            let str_ = match from_utf8(&value_bytes[1..]) {
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
                        ASN1ErrorCode::base_10_real_string_malformed(str_.to_owned().into_bytes()),
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
    UTCTime::try_from(value_bytes)
}

pub fn ber_decode_generalized_time_value(value_bytes: ByteSlice) -> ASN1Result<GeneralizedTime> {
    GeneralizedTime::try_from(value_bytes)
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
    DATE::try_from(value_bytes)
}

pub fn ber_decode_time_of_day_value(value_bytes: ByteSlice) -> ASN1Result<TIME_OF_DAY> {
    TIME_OF_DAY::try_from(value_bytes)
}

pub fn ber_decode_date_time_value(value_bytes: ByteSlice) -> ASN1Result<DATE_TIME> {
    DATE_TIME::try_from(value_bytes)
}

pub fn ber_decode_duration_value(value_bytes: ByteSlice) -> ASN1Result<DURATION> {
    DURATION::try_from(value_bytes)
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

pub fn ber_decode_i64(el: &X690Element) -> ASN1Result<i64> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_i64_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_enumerated(el: &X690Element) -> ASN1Result<ENUMERATED> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_enum_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_bit_string(el: &X690Element) -> ASN1Result<BIT_STRING> {
    deconstruct_bit_string(&el)
}

pub fn ber_decode_octet_string(el: &X690Element) -> ASN1Result<OCTET_STRING> {
    Ok(deconstruct(el)?.into_owned())
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
        X690Encoding::IMPLICIT(bytes) => ber_decode_real_value(bytes.as_slice()),
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
        X690Encoding::IMPLICIT(bytes) => ber_decode_relative_oid_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_sequence(el: &X690Element) -> ASN1Result<SEQUENCE> {
    match el.value.borrow() {
        X690Encoding::Constructed(components) => {
            let mut values: Vec<ASN1Value> = Vec::with_capacity(components.len());
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
            let mut values: Vec<ASN1Value> = Vec::with_capacity(components.len());
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
    ber_decode_graphic_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_utf8_string(el: &X690Element) -> ASN1Result<UTF8String> {
    match String::from_utf8(deconstruct(el)?.into_owned()) {
        Ok(x) => Ok(x),
        Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
    }
}

pub fn ber_decode_numeric_string(el: &X690Element) -> ASN1Result<NumericString> {
    ber_decode_numeric_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_printable_string(el: &X690Element) -> ASN1Result<PrintableString> {
    ber_decode_printable_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_t61_string(el: &X690Element) -> ASN1Result<T61String> {
    Ok(deconstruct(el)?.into_owned())
}

pub fn ber_decode_videotex_string(el: &X690Element) -> ASN1Result<VideotexString> {
    Ok(deconstruct(el)?.into_owned())
}

pub fn ber_decode_ia5_string(el: &X690Element) -> ASN1Result<IA5String> {
    ber_decode_ia5_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_utc_time(el: &X690Element) -> ASN1Result<UTCTime> {
    ber_decode_utc_time_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_generalized_time(el: &X690Element) -> ASN1Result<GeneralizedTime> {
    ber_decode_generalized_time_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_graphic_string(el: &X690Element) -> ASN1Result<GraphicString> {
    ber_decode_graphic_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_visible_string(el: &X690Element) -> ASN1Result<VisibleString> {
    ber_decode_visible_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_general_string(el: &X690Element) -> ASN1Result<GeneralString> {
    ber_decode_general_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_universal_string(el: &X690Element) -> ASN1Result<UniversalString> {
    ber_decode_universal_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_bmp_string(el: &X690Element) -> ASN1Result<BMPString> {
    ber_decode_bmp_string_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_date(el: &X690Element) -> ASN1Result<DATE> {
    ber_decode_date_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_time_of_day(el: &X690Element) -> ASN1Result<TIME_OF_DAY> {
    ber_decode_time_of_day_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_date_time(el: &X690Element) -> ASN1Result<DATE_TIME> {
    ber_decode_date_time_value(deconstruct(el)?.as_ref())
}

pub fn ber_decode_duration(el: &X690Element) -> ASN1Result<DURATION> {
    ber_decode_duration_value(deconstruct(el)?.as_ref())
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
        ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED => match ber_decode_enumerated(el) {
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
    let mut out: Bytes = Vec::with_capacity(1);
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

// TODO: Make this generic across numeric types?
pub fn ber_encode_i64(value: &i64) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_enum_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_enumerated(value: &ENUMERATED) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(2); // Most enums are small.
    x690_write_enum_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_bit_string(value: &BIT_STRING) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.bytes.len() + 1);
    x690_write_bit_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_octet_string(value: &OCTET_STRING) -> ASN1Result<X690Element> {
    // Slight optimization to skip all this.
    // let mut out: Bytes = Vec::with_capacity(value.len());
    // x690_write_octet_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING,
        Arc::new(X690Encoding::IMPLICIT(value.clone())),
    ))
}

pub fn ber_encode_null(_value: &NULL) -> ASN1Result<X690Element> {
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_NULL,
        Arc::new(X690Encoding::IMPLICIT(vec![])),
    ))
}

pub fn ber_encode_object_identifier(value: &OBJECT_IDENTIFIER) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.0.len() << 2); // We assume, on average, each arc takes two bytes.
    x690_write_object_identifier_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_external(value: &EXTERNAL) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_tag(
        &mut out,
        TagClass::UNIVERSAL,
        true,
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
    )?;
    x690_write_external_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
        Arc::new(X690Encoding::AlreadyEncoded(out)),
    ))
}

pub fn ber_encode_instance_of(value: &INSTANCE_OF) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_tag(
        &mut out,
        TagClass::UNIVERSAL,
        true,
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
    )?;
    let external = EXTERNAL {
        identification: ExternalIdentification::syntax(value.type_id.clone()),
        data_value_descriptor: None,
        data_value: ExternalEncoding::single_ASN1_type(value.value.clone()),
    };
    x690_write_external_value(&mut out, &external)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
        Arc::new(X690Encoding::AlreadyEncoded(out)),
    ))
}

pub fn ber_encode_real(value: &REAL) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(32); // This should cover most values.
    x690_write_real_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_REAL,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_embedded_pdv(value: &EMBEDDED_PDV) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_tag(
        &mut out,
        TagClass::UNIVERSAL,
        true,
        ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV,
    )?;
    x690_write_embedded_pdv_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV,
        Arc::new(X690Encoding::AlreadyEncoded(out)),
    ))
}

pub fn ber_encode_character_string(value: &CHARACTER_STRING) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_tag(
        &mut out,
        TagClass::UNIVERSAL,
        true,
        ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING,
    )?;
    x690_write_character_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING,
        Arc::new(X690Encoding::AlreadyEncoded(out)),
    ))
}

pub fn ber_encode_relative_oid(value: &RELATIVE_OID) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.0.len() << 2); // We assume, on average, each arc takes two bytes.
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
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_object_descriptor_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_utf8_string(value: &str) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len() << 1); // TODO: Should this pre-allocate double for non-ASCII?
    x690_write_utf8_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_numeric_string(value: &str) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_printable_string(value: &str) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_t61_string(value: &T61String) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_octet_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_videotex_string(value: &VideotexString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_octet_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_ia5_string(value: &IA5String) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_utc_time(value: &UTCTime) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(17); // This is the max length of a UTCTime.
    x690_write_utc_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_generalized_time(value: &GeneralizedTime) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(32); // There is no defined max length, but this is very generous capacity.
    x690_write_generalized_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_graphic_string(value: &GraphicString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_visible_string(value: &VisibleString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_general_string(value: &GeneralString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_universal_string(value: &UniversalString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len() << 2); // Four bytes for every character
    x690_write_universal_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_bmp_string(value: &BMPString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len() << 1); // Two bytes for every character
    x690_write_bmp_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_date(value: &DATE) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(10); // YYYY-MM-DD
    x690_write_date_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_DATE,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_time_of_day(value: &TIME_OF_DAY) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(8); // HH:MM:SS
    x690_write_time_of_day_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_date_time(value: &DATE_TIME) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(19); // 1951-10-14T15:30:00
    x690_write_date_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_duration(value: &DURATION) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(16); // There is no guaranteed size, but 16 is a reasonable pre-allocation.
    x690_write_duration_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_DURATION,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_oid_iri(value: &OID_IRI) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_relative_oid_iri(value: &OID_IRI) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_time(value: &TIME) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::with_capacity(value.len());
    x690_write_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_validate_boolean_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() != 1 {
        return Err(ASN1Error::new(ASN1ErrorCode::x690_boolean_not_one_byte));
    }
    Ok(())
}

pub fn ber_validate_integer_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
    }
    if content_octets.len() == 1 {
        return Ok(());
    }
    if ((content_octets[0] == 0xFF) && (content_octets[1] >= 0b1000_0000))
        || ((content_octets[0] == 0x00) && (content_octets[1] < 0b1000_0000)) {
        return Err(ASN1Error::new(ASN1ErrorCode::int_padding));
    }
    return Ok(());
}

pub fn ber_validate_bit_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
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

pub fn ber_validate_octet_string_value (_content_octets: ByteSlice) -> ASN1Result<()> {
    Ok(())
}

pub fn ber_validate_null_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() == 1 {
        Ok(())
    } else {
        Err(ASN1Error::new(ASN1ErrorCode::malformed_value))
    }
}

pub fn ber_validate_object_identifier_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
    }
    if content_octets.len() > 1 && content_octets[content_octets.len() - 1] >= 0b1000_0000 {
        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
    }
    let mut previous_byte_was_end_of_arc: bool = true;
    for byte in content_octets {
        if previous_byte_was_end_of_arc && *byte == 0b1000_0000 {
            return Err(ASN1Error::new(ASN1ErrorCode::oid_padding));
        }
        previous_byte_was_end_of_arc = *byte < 0b1000_0000;
    }
    Ok(())
}

pub fn ber_validate_object_descriptor_value (content_octets: ByteSlice) -> ASN1Result<()> {
    ber_validate_graphic_string_value(content_octets)
}

pub fn ber_validate_real_value (content_octets: ByteSlice) -> ASN1Result<()> {
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
                return Err(ASN1Error::new(ASN1ErrorCode::truncated));
            }
            if remaining_slice[0] == b'+' || remaining_slice[0] == b'-' {
                start += 1;
                remaining_slice = &content_octets[start..];
            }
            if remaining_slice.len() == 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::truncated));
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
                        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
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
                return Err(ASN1Error::new(ASN1ErrorCode::binary_real_unrecognized_base));
            }
            let exp_encoding = content_octets[0] & 0b0000_0011;
            let exp_len: usize = match exp_encoding {
                0b0000_0000 => 1,
                0b0000_0001 => 2,
                0b0000_0010 => 3,
                0b0000_0011 => {
                    // One byte for prefix, one for exp length, at least one for exp length and at least one for the mantissa.
                    if content_octets.len() < 4 {
                        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
                    }
                    content_octets[1] as usize + 1
                },
                _ => panic!(),
            };
            if content_octets.len() < exp_len + 2 {
                return Err(ASN1Error::new(ASN1ErrorCode::truncated));
            }
        },
        _ => panic!(),
    };
    Ok(())
}

pub fn ber_validate_enumerated_value (content_octets: ByteSlice) -> ASN1Result<()> {
    ber_validate_integer_value(content_octets)
}

pub fn ber_validate_utf8_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    from_utf8(content_octets)
        .map(|_| ())
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8))
}

pub fn ber_validate_relative_object_identifier_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let mut previous_byte_was_end_of_arc: bool = true;
    for byte in content_octets {
        if previous_byte_was_end_of_arc && *byte == 0b1000_0000 {
            return Err(ASN1Error::new(ASN1ErrorCode::oid_padding));
        }
        previous_byte_was_end_of_arc = *byte < 0b1000_0000;
    }
    Ok(())
}

pub fn ber_validate_time_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let maybe_bad_char = content_octets.iter().position(|b| b.is_ascii_graphic());
    if let Some(bad_char_index) = maybe_bad_char {
        let bad_char = content_octets[bad_char_index];
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            bad_char as u32,
            bad_char_index,
        )));
    }
    Ok(())
}

pub fn ber_validate_numeric_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let maybe_bad_char = content_octets.iter().position(|b| b.is_ascii_digit() || *b == b' ');
    if let Some(bad_char_index) = maybe_bad_char {
        let bad_char = content_octets[bad_char_index];
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            bad_char as u32,
            bad_char_index,
        )));
    }
    Ok(())
}

pub fn ber_validate_printable_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    for (i, byte) in content_octets.iter().enumerate() {
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
    Ok(())
}

pub fn ber_validate_t61_string_value (_content_octets: ByteSlice) -> ASN1Result<()> {
    Ok(())
}

pub fn ber_validate_videotex_string_value (_content_octets: ByteSlice) -> ASN1Result<()> {
    Ok(())
}

pub fn ber_validate_ia5_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let maybe_bad_char = content_octets.iter().position(|b| b.is_ascii());
    if let Some(bad_char_index) = maybe_bad_char {
        let bad_char = content_octets[bad_char_index];
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            bad_char as u32,
            bad_char_index,
        )));
    }
    Ok(())
}

// 9604152030Z
pub fn ber_validate_utc_time_value (content_octets: ByteSlice) -> ASN1Result<()> {
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
    let s = from_utf8(&content_octets[0..10]).map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8))?;
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
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        // This isn't technically correct leap-year handling, but it should be good for the next 175 years or so.
        2 => if year % 4 > 0 { 28 } else { 29 },
        _ => 30,
    };
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

// YYYYMMDDHH[MM[SS[.fff]]] e.g. 19960415203000.0-0600
// pub fn ber_validate_generalized_time_value (content_octets: ByteSlice) -> ASN1Result<()> {
//     if content_octets.len() > 32 {
//         return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
//     }
//     if content_octets.len() < 10 {
//         return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
//     }
//     let maybe_bad_char = content_octets[0..10].iter().position(|b| b.is_ascii_digit());
//     if let Some(bad_char_index) = maybe_bad_char {
//         let bad_char = content_octets[bad_char_index];
//         return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
//             bad_char as u32,
//             bad_char_index,
//         )));
//     }
//     let s = unsafe { String::from_utf8_unchecked(content_octets[0..10].to_vec()) };
//     let year = u16::from_str(&s[0..4])
//         .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
//     // I confirmed in a unit test below that u8::from_str() will tolerate leading zeros.
//     let month = u8::from_str(&s[4..6])
//         .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
//     let day = u8::from_str(&s[6..8])
//         .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
//     let hour = u8::from_str(&s[8..10])
//         .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
//     // let minute = u8::from_str(&s[8..10])
//     //     .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
//     if month > 12 || month == 0 {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
//     }
//     let max_day = match month {
//         1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
//         // This isn't technically correct leap-year handling, but it should be good for the next 175 years or so.
//         2 => if year % 4 > 0 { 28 } else { 29 },
//         _ => 30,
//     };
//     if day == 0 || day > max_day {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
//     }
//     if hour > 23 {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
//     }
//     if minute > 59 {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
//     }
//     let mut start_of_time_zone = 10;
//     if content_octets[10].is_ascii_digit() {
//         if content_octets.len() < 12 || !content_octets[11].is_ascii_digit() {
//             return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
//         }
//         let second = u8::from_str(&s[10..12])
//             .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
//         if second > 59 {
//             return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
//         }
//         start_of_time_zone = 12;
//     }
//     if content_octets.len() < start_of_time_zone + 1 {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
//     }
//     if content_octets[start_of_time_zone] == b'Z' {
//         if content_octets.len() > start_of_time_zone + 1 {
//             return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
//         }
//         return Ok(());
//     }
//     if content_octets.len() != start_of_time_zone + 5 {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
//     }
//     if content_octets[start_of_time_zone] != b'+'
//         && content_octets[start_of_time_zone] != b'-' {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
//     }

//     let maybe_bad_char = content_octets[start_of_time_zone + 1..].iter().position(|b| b.is_ascii_digit());
//     if let Some(bad_char_index) = maybe_bad_char {
//         let bad_char = content_octets[bad_char_index];
//         return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
//             bad_char as u32,
//             bad_char_index,
//         )));
//     }
//     let offset_s = unsafe { String::from_utf8_unchecked(content_octets[start_of_time_zone + 1..].to_vec()) };
//     let offset_hour = u8::from_str(&offset_s[6..8])
//         .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
//     let offset_minute = u8::from_str(&offset_s[8..10])
//         .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
//     if offset_hour > 23 || offset_minute > 59 {
//         return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
//     }
//     Ok(())
// }

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0, // Invalid month
    }
}

// Function produced by ChatGPT-4 before a few modifications by me.
pub fn ber_validate_generalized_time_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let s = match from_utf8(content_octets) {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
    };

    // Check for basic length
    if s.len() < 15 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }

    // Extract and validate date and time parts
    let year: u32 = match s[..4].parse() {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_year)),
    };

    let month: u32 = match s[4..6].parse() {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_month)),
    };

    let day: u32 = match s[6..8].parse() {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_day)),
    };

    let hour: u32 = match s[8..10].parse() {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour)),
    };

    let minute: u32 = match s[10..12].parse() {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute)),
    };

    let second: u32 = match s[12..14].parse() {
        Ok(v) => v,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_second)),
    };

    if month == 0 || month > 12 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
    }

    if day == 0 || day > days_in_month(year, month) {
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
            let (fraction, tz) = s.split_at(s.find(|c: char| !c.is_numeric()).unwrap_or(0));

            if fraction.is_empty() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds));
            }

            match tz {
                "Z" => Ok(()),
                tz if tz.starts_with('+') || tz.starts_with('-') => {
                    if tz.len() == 5 && tz[1..].chars().all(|c| c.is_numeric()) {
                        Ok(())
                    } else {
                        Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds))
                    }
                }
                _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
            }
        }
        tz if tz.starts_with('+') || tz.starts_with('-') => {
            if tz.len() == 5 && tz[1..].chars().all(|c| c.is_numeric()) {
                Ok(())
            } else {
                Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset))
            }
        }
        _ => Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    }
}

// TODO: Define a macro to reduce this boilerplate.
pub fn ber_validate_graphic_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let maybe_bad_char = content_octets.iter().position(|b| b.is_ascii_graphic() || *b == b' ');
    if let Some(bad_char_index) = maybe_bad_char {
        let bad_char = content_octets[bad_char_index];
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            bad_char as u32,
            bad_char_index,
        )));
    }
    Ok(())
}

pub fn ber_validate_visible_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let maybe_bad_char = content_octets.iter().position(|b| !b.is_ascii() || *b == 0x7F);
    if let Some(bad_char_index) = maybe_bad_char {
        let bad_char = content_octets[bad_char_index];
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            bad_char as u32,
            bad_char_index,
        )));
    }
    Ok(())
}

pub fn ber_validate_general_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    let maybe_bad_char = content_octets.iter().position(|b| !b.is_ascii());
    if let Some(bad_char_index) = maybe_bad_char {
        let bad_char = content_octets[bad_char_index];
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            bad_char as u32,
            bad_char_index,
        )));
    }
    Ok(())
}

pub fn ber_validate_universal_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() % 4 > 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
    }
    // Theoretically, you could validate that every uint32 is a valid Unicode
    // code point as well, but that might be overkill.
    Ok(())
}

// TODO: CHARACTER STRING	Constructed	29	1D

pub fn ber_validate_bmp_string_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() % 2 > 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
    }
    // TODO: Do you need to do any validation with a BOM?
    Ok(())
}

pub fn ber_validate_date_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() != 10 { // YYYY-MM-DD
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    if
        content_octets[4] != b'-'
        || content_octets[6] != b'-'
        || !content_octets[0..4].iter().all(|b| b.is_ascii_digit())
        || !content_octets[5..7].iter().all(|b| b.is_ascii_digit())
        || !content_octets[8..].iter().all(|b| b.is_ascii_digit())
    {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let s = unsafe { std::str::from_utf8_unchecked(&content_octets) };
    let year = u16::from_str(&s[0..4])
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
    let month = u8::from_str(&s[5..7])
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
    let day = u8::from_str(&s[8..])
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
    if month > 12 || month == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        // This isn't technically correct leap-year handling, but it should be good for the next 175 years or so.
        2 => if year % 4 > 0 { 28 } else { 29 },
        _ => 30,
    };
    if day == 0 || day > max_day {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
    }
    // TODO: Do you need to do any validation with a BOM?
    Ok(())
}

pub fn ber_validate_time_of_day_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() != 8 { // HH:MM:SS
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    if
        content_octets[2] != b':'
        || content_octets[5] != b':'
        || !content_octets[0..2].iter().all(|b| b.is_ascii_digit())
        || !content_octets[3..5].iter().all(|b| b.is_ascii_digit())
        || !content_octets[6..].iter().all(|b| b.is_ascii_digit())
    {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let s = unsafe { std::str::from_utf8_unchecked(&content_octets) };
    let hour = u8::from_str(&s[0..2])
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
    let minute = u8::from_str(&s[3..5])
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
    let second = u8::from_str(&s[6..])
        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?;
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

pub fn ber_validate_date_time_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() != 19 { // 1951-10-14T15:30:00
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    if content_octets[10] != b'T' {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    ber_validate_date_value(&content_octets[0..10])?;
    ber_validate_time_of_day_value(&content_octets[11..])
}

// Before some tweaking, this was produced by ChatGPT.
pub fn ber_validate_duration_value (content_octets: ByteSlice) -> ASN1Result<()> {
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

pub fn ber_validate_oid_iri_value (content_octets: ByteSlice) -> ASN1Result<()> {
    if content_octets.len() < 2 || content_octets[0] != b'/' {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    Ok(())
}

pub fn ber_validate_relative_oid_iri_value (_content_octets: ByteSlice) -> ASN1Result<()> {
    Ok(())
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
                assert!(oid.0.starts_with(&[2, 5, 4, 3]));
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

    #[test]
    fn make_sure_u8_from_str_handles_leading_zeros () {
        let num = u8::from_str("05").unwrap();
        assert_eq!(num, 5);
    }
}
