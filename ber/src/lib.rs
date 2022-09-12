use std::io::{Write, Result, Error, ErrorKind};
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
    SEQUENCE_OF,
    SET,
    SET_OF,
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
    EXTERNAL,
    EMBEDDED_PDV,
    GeneralString,
    GeneralizedTime,
    UTCTime,
    UTF8String,
    UTCOffset,
    CHARACTER_STRING,
    CharacterString,
    RELATIVE_OID_IRI,
    OID_IRI,
    IA5String,
    INSTANCE_OF,
    BMPString,
    ObjectDescriptor,
    GraphicString,
    MAX_IA5_STRING_CHAR_CODE, UniversalString, DURATION_EQUIVALENT,
};
use asn1::bitstring::join_bit_strings;
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
                Ok((read, cst)) => {
                    return deconstruct_bit_string(cst);
                },
                Err(e) => return Err(e),
            }
        },
        X690Encoding::Constructed(children) => {
            let substituent_bit_strings: Vec<BIT_STRING> = Vec::new();
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

pub fn ber_decode_octet_string_value (value_bytes: ByteSlice) -> Result<BIT_STRING> {
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
    let mut nodes: Vec<u32> = Vec::new();
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
        2 => u16::from_be_bytes(&[ bytes[0], bytes[1] ]) as u64,
        3 => u32::from_be_bytes(&[ 0x00, bytes[0], bytes[1], bytes[2] ]) as u64,
        4 => u32::from_be_bytes(&bytes) as u64,
        5 => u64::from_be_bytes(&[ 0x00, 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4] ]),
        6 => u64::from_be_bytes(&[ 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5] ]),
        7 => u64::from_be_bytes(&[ 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6] ]),
        8 => u64::from_be_bytes(&bytes),
        _ => 0,
    }
}

pub fn ber_decode_real_value (value_bytes: ByteSlice) -> Result<REAL> {
    if value_bytes.len() == 0 {
        return Ok(0.000000);
    }
    match value_bytes[0] & 0b1100_0000 {
        X690_REAL_SPECIAL => 
            return match value_bytes[0] {
                X690_SPECIAL_REAL_PLUS_INFINITY => return Ok(f64::INFINITY),
                X690_SPECIAL_REAL_MINUS_INFINITY => return Ok(f64::NEG_INFINITY),
                X690_SPECIAL_REAL_NOT_A_NUMBER => return Ok(f64::NAN),
                X690_SPECIAL_REAL_MINUS_ZERO => return Ok(-0.000000),
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            },
        X690_REAL_BASE10 => {
            let str: String = value_bytes[1..].into();
            return match value_bytes[0] & 0b0011_1111 {
                X690_REAL_NR1 => {
                    // skip whitespace
                    return Ok(f64::from(str)); // TODO: Testing.
                },
                X690_REAL_NR2 => {
                    // skip whitespace
                    return Ok(f64::from(str)); // TODO: Testing.
                },
                X690_REAL_NR3 => {
                    // skip whitespace
                    return Ok(f64::from(str)); // TODO: Testing.
                },
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            }
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
            let mut mantissa: u64 = 0;
            let mut exponent: i32 = 0;
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
                    exponent = i16::from_be_bytes(&value_bytes[1..3]);
                    mantissa = ber_read_var_length_u64(&value_bytes[3..])
                },
                X690_REAL_EXPONENT_FORMAT_3_OCTET => {
                    if value_bytes.len() < 5 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    if value_bytes.len() > 5 + 6 { // Mantissa too big.
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    exponent = i16::from_be_bytes(&value_bytes[1..4]);
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
                    if value_bytes.len() > 3 + exponent_len { // Mantissa too big.
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    if exponent_len == 1 {
                        exponent = value_bytes[2] as i8 as i16;
                        mantissa = ber_read_var_length_u64(&value_bytes[3..]);
                    } else { // The exponent must have length 2.
                        exponent = i16::from_be_bytes(&[ value_bytes[2], value_bytes[3] ]);
                        mantissa = ber_read_var_length_u64(&value_bytes[4..]);
                    }
                },
                _ => return Err(Error::from(ErrorKind::InvalidData)), // This should never happen.
            }
            if negative {
                return -1 * mantissa * (2 ** scale) * (base ** exponent);
            } else {
                return mantissa * (2 ** scale) * (base ** exponent);
            }
        },
    }
}

pub fn ber_decode_numeric_string_value (value_bytes: ByteSlice) -> Result<NumericString> {
    for byte in value_bytes {
        if !byte.is_ascii_digit() && byte != 0x20 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    return Ok(String::from(value_bytes));
}

pub fn ber_decode_printable_string_value (value_bytes: ByteSlice) -> Result<PrintableString> {
    for byte in value_bytes {
        if byte.is_ascii_alphanumeric()
            || (byte >= '\x27' && byte < '0' && byte != '*') // '()+,-./ BUT NOT *
            || byte == ' '
            || byte == ':'
            || byte == '='
            || byte == '?' {
            continue;
        }
        return Err(Error::from(ErrorKind::InvalidData));
    }
    return Ok(String::from(value_bytes));
}

pub fn ber_decode_ia5_string_value (value_bytes: ByteSlice) -> Result<IA5String> {
    for byte in value_bytes {
        if byte > MAX_IA5_STRING_CHAR_CODE {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    return Ok(String::from(value_bytes));
}

pub fn ber_decode_utc_time_value (value_bytes: ByteSlice) -> Result<UTCTime> {

}

pub fn ber_decode_generalized_time_value (value_bytes: ByteSlice) -> Result<GeneralizedTime> {

}

pub fn ber_decode_graphic_string_value (value_bytes: ByteSlice) -> Result<GraphicString> {
    for byte in value_bytes {
        if !byte.is_ascii_graphic() && (byte != ' ') {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    return Ok(String::from(value_bytes));
}

pub fn ber_decode_visible_string_value (value_bytes: ByteSlice) -> Result<VisibleString> {
    for byte in value_bytes {
        if !byte.is_ascii() || byte == 0x7F {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    return Ok(String::from(value_bytes));
}

pub fn ber_decode_general_string_value (value_bytes: ByteSlice) -> Result<GeneralString> {
    for byte in value_bytes {
        if !byte.is_ascii() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    return Ok(String::from(value_bytes));
}

pub fn ber_decode_universal_string_value (value_bytes: ByteSlice) -> Result<UniversalString> {
    if (value_bytes.len() % 4) != 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let char_length = value_bytes.len() / 4;
    let mut ret = String::with_capacity(char_length);
    for i in 0..char_length {
        ret.push(char::from_u32(u32::from_be_bytes(&[
            value_bytes[(i * 4) + 0],
            value_bytes[(i * 4) + 1],
            value_bytes[(i * 4) + 2],
            value_bytes[(i * 4) + 3],
        ])));
    }
    return Ok(ret);
}

pub fn ber_decode_bmp_string_value (value_bytes: ByteSlice) -> Result<BMPString> {
    return Ok(String::from_utf16(value_bytes));
}

pub fn ber_decode_date_value (value_bytes: ByteSlice) -> Result<DATE> {
    if value_bytes.len() != 10 { // "YYYY-MM-DD".len()
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_ = String::from(value_bytes);
    let year = u16::from(str_[0..4]);
    let month = u8::from(str_[5..7]);
    let day = u8::from(str_[8..]);
    return Ok(DATE { year, month, day });
}

pub fn ber_decode_time_of_day_value (value_bytes: ByteSlice) -> Result<TIME_OF_DAY> {
    if value_bytes.len() != 8 { // "HH:MM:SS".len()
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_ = String::from(value_bytes);
    let hour = u16::from(str_[0..2]);
    let minute = u8::from(str_[3..5]);
    let second = u8::from(str_[6..]);
    return Ok(TIME_OF_DAY { hour, minute, second });
}

pub fn ber_decode_date_time_value (value_bytes: ByteSlice) -> Result<DATE_TIME> {
    if value_bytes.len() != 19 { // "YYYY-MM-DDTHH:MM:SS".len()
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_ = String::from(value_bytes);
    let year = u16::from(str_[0..4]);
    let month = u8::from(str_[5..7]);
    let day = u8::from(str_[8..10]);
    let hour = u16::from(str_[11..13]);
    let minute = u8::from(str_[14..16]);
    let second = u8::from(str_[17..19]);
    return Ok(DATE_TIME {
        date: DATE { year, month, day },
        time: TIME_OF_DAY { hour, minute, second },
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
    if value_bytes[0] != 'P' {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let ret = DURATION_EQUIVALENT::new();
    let mut start_of_last_digit = 0;
    let mut processing_time_components: bool = false;
    let mut index_of_period = 0; // 0 means NULL in this case.
    for i in 1..value_bytes.len() {
        let char = value_bytes[i];
        if !char.is_ascii_digit() {
            if start_of_last_digit == i {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            match char {
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
                    let component = String::from(value_bytes[start_of_last_digit..end_index]);
                    start_of_last_digit = i + 1;
                    let mut encountered: u8 = 0;
                    match char {
                        'Y' => {
                            if processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > 0 {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_YEARS;
                            ret.years = u32::from(component);
                        },
                        'M' => {
                            if processing_time_components {
                                if encountered > DURATION_COMPONENT_HOURS {
                                    return Err(Error::from(ErrorKind::InvalidData));
                                }
                                encountered |= DURATION_COMPONENT_MINUTES;
                                ret.minutes = u32::from(component);
                            } else {
                                if encountered > DURATION_COMPONENT_YEARS {
                                    return Err(Error::from(ErrorKind::InvalidData));
                                }
                                encountered |= DURATION_COMPONENT_MONTHS;
                                ret.months = u32::from(component);
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
                            ret.weeks = u32::from(component);
                        },
                        'D' => {
                            if processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_WEEKS {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_DAYS;
                            ret.days = u32::from(component);
                        },
                        'H' => {
                            if !processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_DAYS {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_HOURS;
                            ret.hours = u32::from(component);
                        },
                        'S' => {
                            if !processing_time_components {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            if encountered > DURATION_COMPONENT_MINUTES {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            encountered |= DURATION_COMPONENT_SECONDS;
                            ret.seconds = u32::from(component);
                        },
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

/* Decide how you want to decode values:

Bytes to raw values
Bytes to ASN1Values
X690Encoding to raw values
X690Encoding to ASN1Values
X690Element to raw values
X690Element to ASN1Values

I think it should be X690Encoding to raw values, because, in many cases, you
will just want the raw value directly.

// TODO: Convert to ByteSlice
IMPLICIT(Bytes), // the value bytes
EXPLICIT(Box<X690Element>), // the inner TLV tuple
Constructed(Vec<X690Element>), // an array of inner TLV tuples
// TODO: Convert to ByteSlice.
AlreadyEncoded(Bytes), // the already-encoded TLV

Decoding to an ASN1Value might be useful, because you may need to decode some
types to more than one alternative, depending on how they were encoded:

- INTEGER to i64 or bigint
- EXTERNAL to an External or InstanceOf

Then again, I think the choice to return a type should be done by the function
called, not automatically. E.g. a library user should explicitly call
`ber_decode_big_int()` if they expect a large integer. I think this means that
there pretty much can't be a generic `ber_decode()` function.

*/

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
            match ber_decode_real_value(bytes.as_slice()) {
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

// TODO: Pretty much every case in here needs to be moved out to a function
// of the form ber_decode_T (el: X690Element) -> Result<T>.
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
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_REAL => match ber_decode_real(el) {
            Ok(v) => Ok(ASN1Value::RealValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED => match ber_decode_integer(el) {
            Ok(v) => Ok(ASN1Value::EnumeratedValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID => match ber_decode_relative_oid(el) {
            Ok(v) => Ok(ASN1Value::RelativeOIDValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_TIME => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_RESERVED_15 => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE => match ber_decode_sequence(el) {
            Ok(v) => Ok(ASN1Value::SequenceOfValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_SET => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING => match ber_decode_numeric_string(el) {
            Ok(v) => Ok(ASN1Value::NumericString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_DATE => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_DURATION => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI => {
            match el.value {

            }
        },
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI => {
            match el.value {

            }
        },
        _ => return Err(Error::from(ErrorKind::InvalidData)),
    }
} 

#[cfg(test)]
mod tests {

    use super::*;
    use asn1::types::{
        TagClass,
        ASN1_UNIVERSAL_TAG_NUMBER_NULL,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
        OPTIONAL,
    };
    use x690::{X690Encoding, X690_TAG_CLASS_UNIVERSAL, ber_cst};

    pub struct AlgorithmIdentifier <'a> {
        pub algorithm: OBJECT_IDENTIFIER,
        pub parameters: OPTIONAL<ASN1Value<'a>>,
    }

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


}
