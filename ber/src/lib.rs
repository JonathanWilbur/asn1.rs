use std::io::{Write, Result, Error, ErrorKind};
use std::str::FromStr;
use asn1::types::{
    BOOLEAN,
    INTEGER,
    ByteSlice,
    BIT_STRING,
    ASN1Value,
    OBJECT_IDENTIFIER,
    TagClass,
    ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
    ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
    RELATIVE_OID,
    REAL,
    OCTET_STRING,
    SEQUENCE,
    // SEQUENCE_OF,
    SET,
    // SET_OF,
    NumericString,
    PrintableString,
    VisibleString,
    VideotexString,
    T61String,
    TIME,
    TIME_OF_DAY,
    DATE,
    DATE_TIME,
    DURATION,
    // EXTERNAL,
    // EMBEDDED_PDV,
    GeneralString,
    // GeneralizedTime,
    UTCTime,
    UTF8String,
    UTCOffset,
    // CHARACTER_STRING,
    // CharacterString,
    // RELATIVE_OID_IRI,
    OID_IRI,
    IA5String,
    // INSTANCE_OF,
    BMPString,
    ObjectDescriptor,
    GraphicString,
    MAX_IA5_STRING_CHAR_CODE,
    UniversalString,
    DURATION_EQUIVALENT,
    ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT,
    ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_NULL,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
    ASN1_UNIVERSAL_TAG_NUMBER_REAL,
    ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED,
    ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_SET,
    ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_DURATION,
    ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI,
};
use asn1::bitstring::join_bit_strings;
use bitvec::macros::internal::funty::Fundamental;
use x690::{
    ber_cst,
    X690Encoding,
    X690Element,
    X690_REAL_BASE_MASK,
    X690_REAL_BINARY_SCALING_MASK,
    X690_REAL_EXPONENT_FORMAT_MASK,
    X690_REAL_EXPONENT_FORMAT_1_OCTET,
    X690_REAL_EXPONENT_FORMAT_2_OCTET,
    X690_REAL_EXPONENT_FORMAT_3_OCTET,
    X690_REAL_EXPONENT_FORMAT_VAR_OCTET,
    X690_REAL_SPECIAL,
    X690_SPECIAL_REAL_PLUS_INFINITY,
    X690_SPECIAL_REAL_MINUS_INFINITY,
    X690_SPECIAL_REAL_NOT_A_NUMBER,
    X690_SPECIAL_REAL_MINUS_ZERO,
    X690_REAL_NR1,
    X690_REAL_NR2,
    X690_REAL_NR3,
    X690_REAL_BASE10,
    X690_REAL_BASE_2,
    X690_REAL_BASE_8,
    X690_REAL_BASE_16,
    deconstruct,
};

// BIT STRING is constructed in a such a way that the octets of each subelement
// cannot simply be concatenated. As such, this function deconstructed a
// constructed BIT STRING to obtain a single BIT STRING.
pub fn deconstruct_bit_string (el: X690Element) -> Result<BIT_STRING> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => {
            match ber_decode_bit_string_value(bytes.as_slice()) {
                Ok(decoded) => return Ok(decoded),
                Err(e) => return Err(e),
            }
        },
        X690Encoding::EXPLICIT(_) => return Err(Error::new(ErrorKind::InvalidData, "asdf")),
        X690Encoding::AlreadyEncoded(bytes) => {
            match ber_cst(&bytes) {
                Ok((_, cst)) => {
                    return deconstruct_bit_string(cst);
                },
                Err(e) => return Err(e),
            }
        },
        X690Encoding::Constructed(children) => {
            let mut substituent_bit_strings: Vec<BIT_STRING> = Vec::new();
            for child in children {
                if child.tag_class != el.tag_class || child.tag_number != el.tag_number {
                    return Err(Error::new(ErrorKind::InvalidData, "asdf")); 
                }
                match deconstruct_bit_string(child) {
                    Ok(deconstructed_child) => {
                        substituent_bit_strings.push(deconstructed_child);
                    },
                    Err(e) => return Err(e),
                }
            }
            return Ok(join_bit_strings(&substituent_bit_strings.as_slice()));
        },
    }
}

pub fn ber_encode_boolean_value <T> (output: &mut T, value: BOOLEAN) -> Result<usize>
    where T : Write {
    if value {
       return output.write(&[ 0x01, 0x01, 0xFF ]);
    } else {
        return output.write(&[ 0x01, 0x01, 0x00 ]);
    }
}

pub fn ber_decode_boolean_value (value_bytes: ByteSlice) -> Result<BOOLEAN> {
    if value_bytes.len() != 1 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    Ok(value_bytes[0] > 0)
}

pub fn ber_decode_integer_value (value_bytes: ByteSlice) -> Result<INTEGER> {
    let len = value_bytes.len();
    match len {
        1 => Ok(value_bytes[0] as i8 as INTEGER),
        2 => Ok(i16::from_be_bytes([ value_bytes[0], value_bytes[1] ]) as i64),
        3 => Ok(i32::from_be_bytes([
            if value_bytes[0] & 0b1000_0000 > 0 { 0xFF } else { 0x00 },
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
        ]) as i64),
        4 => Ok(i32::from_be_bytes([ value_bytes[0], value_bytes[1], value_bytes[2], value_bytes[3] ]) as i64),
        5..=8 => {
            let mut buf: [u8; 8] = [0; 8];
            buf[8-len..].copy_from_slice(value_bytes);
            Ok( i64::from_be_bytes(buf))
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_bit_string_value (value_bytes: ByteSlice) -> Result<BIT_STRING> {
    let len = value_bytes.len();
    if len < 1 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let trailing_bits = value_bytes[0];
    if trailing_bits > 7 {
        return Err(Error::from(ErrorKind::InvalidData)); 
    }
    Ok(BIT_STRING{
        bytes: Vec::from(&value_bytes[1..]),
        trailing_bits,
    })
}

pub fn ber_decode_octet_string_value (value_bytes: ByteSlice) -> Result<OCTET_STRING> {
    Ok(Vec::from(value_bytes))
}

pub fn ber_decode_object_identifier_value (value_bytes: ByteSlice) -> Result<OBJECT_IDENTIFIER> {
    let len = value_bytes.len();
    if len < 1 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let arc1 = (value_bytes[0] / 40) as u32;
    let arc2 = (value_bytes[0] % 40) as u32;
    let mut nodes: Vec<u32> = vec![ arc1, arc2 ];
    let mut current_node: u32 = 0;
    for byte in value_bytes[1..].iter() {
        current_node <<= 7;
        current_node += (byte & 0b0111_1111) as u32;
        if (byte & 0b1000_0000) == 0 {
            nodes.push(current_node);
            current_node = 0;
        }
    }
    if current_node > 0 { // Truncated.
        return Err(Error::from(ErrorKind::InvalidData));
    }
    Ok(nodes)
}

pub fn ber_decode_relative_oid_value (value_bytes: ByteSlice) -> Result<RELATIVE_OID> {
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
    if current_node > 0 { // Truncated.
        return Err(Error::from(ErrorKind::InvalidData));
    }
    Ok(nodes)
}

pub fn ber_read_var_length_u64 (bytes: ByteSlice) -> u64 {
    match bytes.len() {
        0 => 0,
        1 => bytes[0] as u8 as u64,
        2 => u16::from_be_bytes([ bytes[0], bytes[1] ]) as u64,
        3 => u32::from_be_bytes([ 0x00, bytes[0], bytes[1], bytes[2] ]) as u64,
        4 => u32::from_be_bytes([ bytes[0], bytes[1], bytes[2], bytes[3] ]) as u64,
        5 => u64::from_be_bytes([ 0x00, 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4] ]),
        6 => u64::from_be_bytes([ 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5] ]),
        7 => u64::from_be_bytes([ 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6] ]),
        8 => u64::from_be_bytes([ bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7] ]),
        _ => 0,
    }
}

pub fn ber_decode_real_value (value_bytes: ByteSlice) -> Result<REAL> {
    if value_bytes.len() == 0 {
        return Ok(0.000000);
    }
    match value_bytes[0] & 0b1100_0000 {
        X690_REAL_SPECIAL => match value_bytes[0] {
            X690_SPECIAL_REAL_PLUS_INFINITY => return Ok(f64::INFINITY),
            X690_SPECIAL_REAL_MINUS_INFINITY => return Ok(f64::NEG_INFINITY),
            X690_SPECIAL_REAL_NOT_A_NUMBER => return Ok(f64::NAN),
            X690_SPECIAL_REAL_MINUS_ZERO => return Ok(-0.000000),
            _ => return Err(Error::from(ErrorKind::InvalidData)),
        },
        X690_REAL_BASE10 => {
            let str_ = match String::from_utf8(value_bytes[1..].to_vec()) {
                Ok(v) => String::from(v.trim_start()),
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            };
            let f64_value = match f64::from_str(&str_) {
                Ok(f) => f,
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            };
            // FIXME: Wrong formatting.
            return match value_bytes[0] & 0b0011_1111 {
                X690_REAL_NR1 => Ok(f64_value),
                X690_REAL_NR2 => Ok(f64_value),
                X690_REAL_NR3 => Ok(f64_value),
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            };
        },
        _ => { // Binary encoding
            let negative = (value_bytes[0] & 0b0100_0000) > 0;
            let base: u8 = match value_bytes[0] & X690_REAL_BASE_MASK {
                X690_REAL_BASE_2 => 2,
                X690_REAL_BASE_8 => 8,
                X690_REAL_BASE_16 => 16,
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            };
            let scale: u8 = (value_bytes[0] & X690_REAL_BINARY_SCALING_MASK).overflowing_shr(2).0;
            let mantissa: u64;
            let exponent: i32;
            match value_bytes[0] & X690_REAL_EXPONENT_FORMAT_MASK {
                X690_REAL_EXPONENT_FORMAT_1_OCTET => {
                    if value_bytes.len() < 3 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    exponent = value_bytes[1] as i8 as i32;
                    mantissa = ber_read_var_length_u64(&value_bytes[2..])
                },
                X690_REAL_EXPONENT_FORMAT_2_OCTET => {
                    if value_bytes.len() < 4 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    if value_bytes.len() > 4 + 6 { // Mantissa too big.
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    exponent = i32::from_be_bytes([ 0, 0, value_bytes[1], value_bytes[2] ]);
                    mantissa = ber_read_var_length_u64(&value_bytes[3..])
                },
                X690_REAL_EXPONENT_FORMAT_3_OCTET => {
                    if value_bytes.len() < 5 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    if value_bytes.len() > 5 + 6 { // Mantissa too big.
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    exponent = i32::from_be_bytes([ 0, value_bytes[1], value_bytes[2], value_bytes[3] ]);
                    mantissa = ber_read_var_length_u64(&value_bytes[4..])
                },
                X690_REAL_EXPONENT_FORMAT_VAR_OCTET => {
                    if value_bytes.len() < 3 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    let exponent_len = value_bytes[1];
                    if exponent_len > 2 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    if value_bytes.len() > (3 + exponent_len).into() { // Mantissa too big.
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    if exponent_len == 1 {
                        exponent = value_bytes[2] as i8 as i32;
                        mantissa = ber_read_var_length_u64(&value_bytes[3..]);
                    } else { // The exponent must have length 2.
                        exponent = i32::from_be_bytes([ 0, 0, value_bytes[2], value_bytes[3] ]);
                        mantissa = ber_read_var_length_u64(&value_bytes[4..]);
                    }
                },
                _ => return Err(Error::from(ErrorKind::InvalidData)), // This should never happen.
            }
            let unsigned_value = mantissa.as_f64()
                * (2u8.pow(scale.into())).as_f64()
                * ((base as f64).powi(exponent)).as_f64();
            if negative {
                return Ok(-1.0 * unsigned_value);
            } else {
                return Ok(unsigned_value);
            }
        },
    }
}

pub fn ber_decode_numeric_string_value (value_bytes: ByteSlice) -> Result<NumericString> {
    for byte in value_bytes {
        if !byte.is_ascii_digit() && *byte != 0x20 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    unsafe {
        Ok(String::from_utf8_unchecked(value_bytes.to_vec()))
    }
}

pub fn ber_decode_printable_string_value (value_bytes: ByteSlice) -> Result<PrintableString> {
    for byte in value_bytes {
        let b = *byte as char;
        if byte.is_ascii_alphanumeric()
            || (b >= '\x27' && b < '0' && b != '*') // '()+,-./ BUT NOT *
            || b == ' '
            || b == ':'
            || b == '='
            || b == '?' {
            continue;
        }
        return Err(Error::from(ErrorKind::InvalidData));
    }
    unsafe {
        Ok(String::from_utf8_unchecked(value_bytes.to_vec()))
    }
}

pub fn ber_decode_ia5_string_value (value_bytes: ByteSlice) -> Result<IA5String> {
    for byte in value_bytes {
        if *byte > MAX_IA5_STRING_CHAR_CODE { // FIXME: This is impossible.
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    // FIXME: This will not decode correctly.
    unsafe {
        Ok(String::from_utf8_unchecked(value_bytes.to_vec()))
    }
}

pub fn ber_decode_utc_time_value (value_bytes: ByteSlice) -> Result<UTCTime> {
    let len = value_bytes.len();
    if len < 10 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    if len > 17 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    for byte in value_bytes[0..10].iter() {
        if !byte.is_ascii_digit() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    let s = match String::from_utf8(value_bytes.to_vec()) {
        Ok(r) => r,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    // TODO: Validate month, day, hour, minute, second.
    let mut ret = UTCTime::new();
    ret.year = match u8::from_str(&s[0..2]) {
        Ok(u) => u,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    ret.month = match u8::from_str(&s[2..4]) {
        Ok(u) => u,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    ret.day = match u8::from_str(&s[4..6]) {
        Ok(u) => u,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    ret.hour = match u8::from_str(&s[6..8]) {
        Ok(u) => u,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    ret.minute = match u8::from_str(&s[8..10]) {
        Ok(u) => u,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    if (len > 12) && value_bytes[10].is_ascii_digit() {
        // Seconds component is present.
        if !value_bytes[11].is_ascii_digit() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        ret.second = match u8::from_str(&s[10..12]) {
            Ok(u) => Some(u),
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };
    }
    if value_bytes[len - 1] as char != 'Z' {
        if (value_bytes[len - 5] as char != '+') && (value_bytes[len - 5] as char != '-') {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        for byte in value_bytes[len-4..len].iter() {
            if !byte.is_ascii_digit() {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }
        let offset_hour = match i8::from_str(&s[len-4..len-2]) {
            Ok(u) => u,
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        let offset_minute = match u8::from_str(&s[len-2..len]) {
            Ok(u) => u,
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        ret.utc_offset = Some(UTCOffset {
            hour: if value_bytes[len - 5] as char == '-' { -1 * offset_hour } else { offset_hour },
            minute: offset_minute,
        });
    }
    Ok(ret)
}

// FIXME:
// pub fn ber_decode_generalized_time_value (value_bytes: ByteSlice) -> Result<GeneralizedTime> {

// }

pub fn ber_decode_graphic_string_value (value_bytes: ByteSlice) -> Result<GraphicString> {
    for byte in value_bytes {
        if !byte.is_ascii_graphic() && (*byte as char != ' ') {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    unsafe {
        Ok(String::from_utf8_unchecked(value_bytes.to_vec()))
    }
}

pub fn ber_decode_visible_string_value (value_bytes: ByteSlice) -> Result<VisibleString> {
    for byte in value_bytes {
        if !byte.is_ascii() || *byte == 0x7F {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    unsafe {
        Ok(String::from_utf8_unchecked(value_bytes.to_vec()))
    }
}

pub fn ber_decode_general_string_value (value_bytes: ByteSlice) -> Result<GeneralString> {
    for byte in value_bytes {
        if !byte.is_ascii() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    unsafe {
        Ok(String::from_utf8_unchecked(value_bytes.to_vec()))
    }
}

pub fn ber_decode_universal_string_value (value_bytes: ByteSlice) -> Result<UniversalString> {
    if (value_bytes.len() % 4) != 0 {
        return Err(Error::from(ErrorKind::InvalidData));
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
            None => return Err(Error::from(ErrorKind::InvalidData)), 
        }
    }
    return Ok(ret);
}

pub fn ber_decode_bmp_string_value (value_bytes: ByteSlice) -> Result<BMPString> {
    if (value_bytes.len() % 2) != 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let char_length = value_bytes.len() / 4;
    let mut ret = String::with_capacity(char_length);
    for i in 0..char_length {
        let code_point = u16::from_be_bytes([
            value_bytes[(i * 4) + 0],
            value_bytes[(i * 4) + 1],
        ]);
        match char::from_u32(code_point as u32) {
            Some(c) => ret.push(c),
            None => return Err(Error::from(ErrorKind::InvalidData)), 
        }
    }
    return Ok(ret);
}

pub fn ber_decode_date_value (value_bytes: ByteSlice) -> Result<DATE> {
    if value_bytes.len() != 10 { // "YYYY-MM-DD".len()
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_ = match String::from_utf8(value_bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let year = match u16::from_str(&str_[0..4]) {
        Ok(x) => x,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let month = match u8::from_str(&str_[5..7]) {
        Ok(x) => x,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let day = match u8::from_str(&str_[8..]) {
        Ok(x) => x,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    return Ok(DATE { year, month, day });
}

pub fn ber_decode_time_of_day_value (value_bytes: ByteSlice) -> Result<TIME_OF_DAY> {
    if value_bytes.len() != 8 { // "HH:MM:SS".len()
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_ = match String::from_utf8(value_bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let hour = match u8::from_str(&str_[0..2]) {
        Ok(x) => x,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let minute = match u8::from_str(&str_[3..5]) {
        Ok(x) => x,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let second = match u8::from_str(&str_[6..]) {
        Ok(x) => x,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    return Ok(TIME_OF_DAY { hour, minute, second });
}

pub fn ber_decode_date_time_value (value_bytes: ByteSlice) -> Result<DATE_TIME> {
    if value_bytes.len() != 19 { // "YYYY-MM-DDTHH:MM:SS".len()
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let date = match ber_decode_date_value(&value_bytes[0..10]) {
        Ok(d) => d,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let time = match ber_decode_time_of_day_value(&value_bytes[11..19]) {
        Ok(t) => t,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    return Ok(DATE_TIME {
        date,
        time,
    });
}

const DURATION_COMPONENT_YEARS: u8      = 0b0000_0001;
const DURATION_COMPONENT_MONTHS: u8     = 0b0000_0010;
const DURATION_COMPONENT_WEEKS: u8      = 0b0000_0100;
const DURATION_COMPONENT_DAYS: u8       = 0b0000_1000;
const DURATION_COMPONENT_HOURS: u8      = 0b0001_0000;
const DURATION_COMPONENT_MINUTES: u8    = 0b0010_0000;
const DURATION_COMPONENT_SECONDS: u8    = 0b0100_0000;

pub fn ber_decode_duration_value (value_bytes: ByteSlice) -> Result<DURATION> {
    if value_bytes.len() < 3 { // The smallest duration string, e.g. P1Y
        return Err(Error::from(ErrorKind::InvalidData));
    }
    if value_bytes[0] as char != 'P' {
        return Err(Error::from(ErrorKind::InvalidData));
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
                return Err(Error::from(ErrorKind::InvalidData));
            }
            match char_ as char {
                '.' => {
                    index_of_period = i;
                },
                'Y' | 'W' | 'M' | 'D' | 'H' | 'S' => {
                    if index_of_period > 0 {
                        if i != (value_bytes.len() - 1) {
                            // Extra data after the last permitted unit.
                            return Err(Error::from(ErrorKind::InvalidData));
                        }
                    }
                    let end_index = if index_of_period > 0 { index_of_period } else { i };
                    let component_str = match String::from_utf8(value_bytes[start_of_last_digit..end_index].to_vec()) {
                        Ok(s) => s,
                        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
                    };
                    let component_value = match u32::from_str(&component_str) {
                        Ok(v) => v,
                        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
                    };
                    start_of_last_digit = i + 1;
                    match char_ as char {
                        'Y' => {
                            if processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > 0 {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_YEARS;
                            ret.years = component_value;
                        },
                        'M' => {
                            if processing_time_components {
                                if encountered > DURATION_COMPONENT_HOURS {
                                    return Err(Error::from(ErrorKind::InvalidData));
                                }
                                encountered |= DURATION_COMPONENT_MINUTES;
                                ret.minutes = component_value;
                            } else {
                                if encountered > DURATION_COMPONENT_YEARS {
                                    return Err(Error::from(ErrorKind::InvalidData));
                                }
                                encountered |= DURATION_COMPONENT_MONTHS;
                                ret.months = component_value;
                            }
                        },
                        'W' => {
                            if processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_MONTHS {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_WEEKS;
                            ret.weeks = component_value;
                        },
                        'D' => {
                            if processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_WEEKS {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_DAYS;
                            ret.days = component_value;
                        },
                        'H' => {
                            if !processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_DAYS {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_HOURS;
                            ret.hours = component_value;
                        },
                        'S' => {
                            if !processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_MINUTES {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_SECONDS;
                            ret.seconds = component_value;
                        },
                        _ => panic!("Impossible code reached."),
                    };
                },
                'T' => {
                    processing_time_components = true;
                },
                _ => (),
            }
        }
    }
    Ok(ret)
}

pub fn ber_decode_boolean (el: X690Element) -> Result<BOOLEAN> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => ber_decode_boolean_value(bytes.as_slice()),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_integer (el: X690Element) -> Result<INTEGER> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => ber_decode_integer_value(bytes.as_slice()),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_bit_string (el: X690Element) -> Result<BIT_STRING> {
    deconstruct_bit_string(el)
}

pub fn ber_decode_octet_string (el: X690Element) -> Result<OCTET_STRING> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => Ok(bytes),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_null (el: X690Element) -> Result<ASN1Value> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => {
            if bytes.len() != 0 {
                return Err(Error::from(ErrorKind::InvalidData))
            }
            Ok(ASN1Value::NullValue)
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_object_identifier (el: X690Element) -> Result<OBJECT_IDENTIFIER> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) =>
            match ber_decode_object_identifier_value(bytes.as_slice()) {
                Ok(decoded) => Ok(decoded),
                Err(e) => Err(e),
            },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_real (el: X690Element) -> Result<REAL> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) =>
            match ber_decode_real_value(bytes.as_slice()) {
                Ok(decoded) => Ok(decoded),
                Err(e) => Err(e),
            },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_relative_oid (el: X690Element) -> Result<RELATIVE_OID> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) =>
            match ber_decode_relative_oid_value(bytes.as_slice()) {
                Ok(decoded) => Ok(decoded),
                Err(e) => Err(e),
            },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_sequence (el: X690Element) -> Result<SEQUENCE> {
    match el.value {
        X690Encoding::Constructed(components) => {
            let mut values: Vec<ASN1Value> = Vec::new();
            for component in components {
                match ber_decode_any(component) {
                    Ok(v) => values.push(v),
                    Err(e) => return Err(e),
                }
            }
            return Ok(values);
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_set (el: X690Element) -> Result<SET> {
    match el.value {
        X690Encoding::Constructed(components) => {
            let mut values: Vec<ASN1Value> = Vec::new();
            for component in components {
                match ber_decode_any(component) {
                    Ok(v) => values.push(v),
                    Err(e) => return Err(e),
                }
            }
            return Ok(values);
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_object_descriptor (el: X690Element) -> Result<ObjectDescriptor> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_graphic_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_utf8_string (el: X690Element) -> Result<UTF8String> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes) {
                    Ok(x) => Ok(x),
                    Err(_) => Err(Error::from(ErrorKind::InvalidData)),
                },
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_numeric_string (el: X690Element) -> Result<NumericString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_numeric_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_printable_string (el: X690Element) -> Result<PrintableString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_printable_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_t61_string (el: X690Element) -> Result<T61String> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => Ok(bytes),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_videotex_string (el: X690Element) -> Result<VideotexString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => Ok(bytes),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_ia5_string (el: X690Element) -> Result<IA5String> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_ia5_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_graphic_string (el: X690Element) -> Result<GraphicString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_graphic_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_visible_string (el: X690Element) -> Result<VisibleString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_visible_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_general_string (el: X690Element) -> Result<GeneralString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_general_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_universal_string (el: X690Element) -> Result<UniversalString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_universal_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_bmp_string (el: X690Element) -> Result<BMPString> {
    let deconstruction = deconstruct(el);
    match deconstruction {
        Ok(deconstructed) => {
            match deconstructed.value {
                X690Encoding::IMPLICIT(bytes) => ber_decode_bmp_string_value(bytes.as_slice()),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn ber_decode_date (el: X690Element) -> Result<DATE> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => ber_decode_date_value(bytes.as_slice()),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_time_of_day (el: X690Element) -> Result<TIME_OF_DAY> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => ber_decode_time_of_day_value(bytes.as_slice()),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_date_time (el: X690Element) -> Result<DATE_TIME> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => ber_decode_date_time_value(bytes.as_slice()),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_duration (el: X690Element) -> Result<DURATION> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => ber_decode_duration_value(bytes.as_slice()),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_oid_iri (el: X690Element) -> Result<OID_IRI> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes) {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::from(ErrorKind::InvalidData)),
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_relative_oid_iri (el: X690Element) -> Result<OID_IRI> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes) {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::from(ErrorKind::InvalidData)),
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_time (el: X690Element) -> Result<TIME> {
    match el.value {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes) {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::from(ErrorKind::InvalidData)),
        },
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn ber_decode_any (el: X690Element) -> Result<ASN1Value> {
    if el.tag_class != TagClass::UNIVERSAL {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    if let X690Encoding::AlreadyEncoded(encoding) = el.value {
        match ber_cst(&encoding) {
            Ok((bytes_read, encoded_el)) => {
                if bytes_read != encoding.len() {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
                return ber_decode_any(encoded_el);
            }
            Err(e) => return Err(e),
        }
    }

    match el.tag_number {
        ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT => {
            return Err(Error::from(ErrorKind::InvalidData)); 
        },
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
        ASN1_UNIVERSAL_TAG_NUMBER_NULL => ber_decode_null(el),
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER => match ber_decode_object_identifier(el) {
            Ok(v) => Ok(ASN1Value::ObjectIdentifierValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR => match ber_decode_object_descriptor(el) {
            Ok(v) => Ok(ASN1Value::ObjectDescriptor(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL => {
        //     match el.value {

        //     }
        // },
        ASN1_UNIVERSAL_TAG_NUMBER_REAL => match ber_decode_real(el) {
            Ok(v) => Ok(ASN1Value::RealValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED => match ber_decode_integer(el) {
            Ok(v) => Ok(ASN1Value::EnumeratedValue(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV => {
        //     match el.value {

        //     }
        // },
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
        // ASN1_UNIVERSAL_TAG_NUMBER_RESERVED_15 => {
        //     match el.value {

        //     }
        // },
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE => match ber_decode_sequence(el) {
            Ok(v) => Ok(ASN1Value::SequenceValue(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF => match ber_decode_sequence(el) {
        //     Ok(v) => Ok(ASN1Value::SequenceOfValue(v)),
        //     Err(e) => Err(e),
        // },
        ASN1_UNIVERSAL_TAG_NUMBER_SET => match ber_decode_set(el) {
            Ok(v) => Ok(ASN1Value::SetValue(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_SET_OF => match ber_decode_set(el) {
        //     Ok(v) => Ok(ASN1Value::SetValue(v)),
        //     Err(e) => Err(e),
        // },
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
        // ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING => {
        //     match el.value {

        //     }
        // },
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
        _ => return Err(Error::from(ErrorKind::InvalidData)),
    }
} 

#[cfg(test)]
mod tests {

    use super::*;
    use x690::{X690Encoding, X690_TAG_CLASS_UNIVERSAL, ber_cst};

    // pub struct AlgorithmIdentifier {
    //     pub algorithm: OBJECT_IDENTIFIER,
    //     pub parameters: OPTIONAL<ASN1Value>,
    // }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_ber_encode_boolean () {
        let mut output: Vec<u8> = Vec::new();
        ber_encode_boolean_value(&mut output, true).unwrap();
        ber_encode_boolean_value(&mut output, false).unwrap();
        assert_eq!(output.len(), 6);
        assert!(output.starts_with(&[ 0x01, 0x01, 0xFF, 0x01, 0x01, 0x00 ]));
    }

    #[test]
    fn test_ber_decode_algorithm_identifier () {
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
        if let X690Encoding::Constructed(children) = el.value {
            assert_eq!(children.len(), 2);
            assert_eq!(children[0].tag_class, TagClass::UNIVERSAL);
            assert_eq!(children[0].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER);
            assert_eq!(children[1].tag_class, TagClass::UNIVERSAL);
            assert_eq!(children[1].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_NULL);
            if let X690Encoding::IMPLICIT(oid_bytes) = &children[0].value {
                let oid = match ber_decode_object_identifier_value(&oid_bytes) {
                    Err(_) => panic!("woriyjh"),
                    Ok(result) => result,
                };
                assert!(oid.starts_with(&[ 2, 5, 4, 3 ]));
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
    fn test_ber_decode_utc_time () {
        let time = "\x17\x11991105223344+0523";
        let value_bytes = Vec::from(time);
        let cst = match ber_cst(value_bytes.as_slice()) {
            Ok((_, el)) => el,
            Err(e) => panic!("{}", e),
        };
        if let X690Encoding::IMPLICIT(bytes) = cst.value {
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
