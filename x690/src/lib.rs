use std::io::{Write, Result, Error, ErrorKind};
use std::mem::{size_of};
use asn1::types::{BOOLEAN, INTEGER, BIT_STRING, OCTET_STRING, OBJECT_IDENTIFIER, ObjectDescriptor, REAL};
use num::{Zero, Signed};

const X690_SPECIAL_REAL_PLUS_INFINITY: u8 = 0b1000_0000;
const X690_SPECIAL_REAL_MINUS_INFINITY: u8 = 0b1000_0001;
const X690_SPECIAL_REAL_NOT_A_NUMBER: u8 = 0b1000_0010;
const X690_SPECIAL_REAL_MINUS_ZERO: u8 = 0b1000_0011;
const X690_REAL_BINARY: u8 = 0b1000_0000;
const X690_REAL_POSITIVE: u8 = 0b0000_0000;
const X690_REAL_NEGATIVE: u8 = 0b0100_0000;
const X690_REAL_SIGN_MASK: u8 = 0b0100_0000;
const X690_REAL_BASE_MASK: u8 = 0b0011_0000;
const X690_REAL_BASE_2: u8 = 0b0000_0000;
const X690_REAL_BASE_8: u8 = 0b0001_0000;
const X690_REAL_BASE_16: u8 = 0b0010_0000;
const X690_REAL_BASE_RESERVED: u8 = 0b0011_0000;
const X690_REAL_BINARY_SCALING_MASK: u8 = 0b0000_1100;
const X690_REAL_EXPONENT_FORMAT_MASK: u8 = 0b0000_0011;
const X690_REAL_EXPONENT_FORMAT_1_OCTET: u8 = 0b0000_0000;
const X690_REAL_EXPONENT_FORMAT_2_OCTET: u8 = 0b0000_0001;
const X690_REAL_EXPONENT_FORMAT_3_OCTET: u8 = 0b0000_0010;
const X690_REAL_EXPONENT_FORMAT_VAR_OCTET: u8 = 0b0000_0011;
// const IEEE_754_DPFP_SIGN_MASK

fn write_base_128 <W> (output: &mut W, num: u32) -> Result<usize>
    where W : Write {
    if num < 128 {
        return output.write(&[num as u8]);
    }

    let mut l = 0;
    let mut i = num;
    while i > 0 {
        l += 1;
        i >>= 7;
    }

    let mut encoded: Vec<u8> = Vec::new();
    for i in (0..l).rev() {
        let mut o = (num >> (i * 7)) as u8;
        o &= 0x7f;
        if i != 0 {
            o |= 0x80;
        }
        encoded.push(o);
    }
    return output.write(&encoded);
}

pub fn x690_encode_boolean_value <W> (output: &mut W, value: BOOLEAN) -> Result<usize>
    where W : Write {
    if value {
       return output.write(&[0xFF]);
    } else {
        return output.write(&[0x00]);
    }
}

pub fn x690_encode_integer_value <W> (output: &mut W, value: INTEGER) -> Result<usize>
    where W : Write {
    let bytes = value.to_be_bytes();
    let padding_byte: u8 = if value >= 0 { 0x00 } else { 0xFF };
    let mut number_of_padding_bytes: usize = 0;
    for byte in bytes {
        if byte == padding_byte {
            number_of_padding_bytes += 1;
        }
    }
    if number_of_padding_bytes == size_of::<INTEGER>() {
        return output.write(&[padding_byte]);
    }
    return output.write(&(bytes[number_of_padding_bytes..size_of::<INTEGER>()]));
}

pub fn x690_encode_bit_string_value <W> (output: &mut W, value: &mut BIT_STRING) -> Result<usize>
    where W : Write {
    let padding_bits_result = output.write(&[8u8 - (value.len() % 8) as u8]);
    return match padding_bits_result {
        Err(e) => { Err(e) },
        _ => {
            let mut clone = value.clone();
            clone.set_uninitialized(false);
            let mut bit_bytes_written = 1;
            for chunk in clone.into_vec().iter() {
                let bytes = chunk.to_le_bytes();
                match output.write(&bytes) {
                    Err(e) => return Err(e),
                    _ => {
                        bit_bytes_written += bytes.len();
                    }
                }
            }
            return Ok(bit_bytes_written);
        },
    };
}

pub fn x690_encode_octet_string_value <W> (output: &mut W, value: &OCTET_STRING) -> Result<usize>
    where W : Write {
    output.write(value)
}

pub fn x690_encode_object_identifier_value <W> (output: &mut W, value: &OBJECT_IDENTIFIER) -> Result<usize>
    where W : Write {
    // TODO: Should object identifier be a type that forces a first and second arc?
    if value.len() < 2 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let node0 = value[0] ;
    let node1 = value[1];
    let byte0 = (node0 * 40) + node1;
    let mut bytes_written = 0;
    match output.write(&[ byte0 as u8 ]) {
        Err(e) => return Err(e),
        _ => {
            bytes_written += 1;
        }
    }
    for arc in value[2..].iter() {
        match write_base_128(output, *arc) {
            Err(e) => { return Err(e) },
            Ok(wrote_bytes) => {
                bytes_written += wrote_bytes;
            }
        }
    }
    Ok(bytes_written)
}

pub fn x690_encode_object_descriptor_value <W> (output: &mut W, value: &ObjectDescriptor) -> Result<usize>
    where W : Write {
    // TODO: Validate
    output.write(value.as_bytes())
}

pub fn ber_encode_real_value <W> (output: &mut W, value: REAL) -> Result<usize>
    where W : Write {
    // If the real value is the value plus zero, there shall be no contents octets in the encoding.
    if value.is_zero() {
        return Ok(0);
    }
    // If the real value is the value minus zero, then it shall be encoded as specified in 8.5.9.
    if value.is_zero() && value.is_sign_negative() {
        return output.write(&[ X690_SPECIAL_REAL_MINUS_ZERO ]);
    }

    if value.is_nan() {
        return output.write(&[ X690_SPECIAL_REAL_NOT_A_NUMBER ]);
    }

    if value.is_infinite() {
        if value.is_sign_negative() {
            return output.write(&[ X690_SPECIAL_REAL_MINUS_INFINITY ]);
        } else {
            return output.write(&[ X690_SPECIAL_REAL_PLUS_INFINITY ]);
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
    let mut mantissa: u64 = u64::from_be_bytes(
        [ 0x00, 0x08 & bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7] ]
    );
    let mut exponent: u16 = u16::from_be_bytes([ bytes[0] & 0x7F, bytes[1] & 0b1110_0000 ]) >> 5;
    while (mantissa > 0) && !((mantissa % 2) > 0) {
        mantissa = mantissa >> 1;
        exponent += 1;
    }
    let e_bytes = exponent.to_be_bytes();
    let mut bytes_written: usize = 0;
    if exponent > u8::MAX as u16 {
        let byte0: u8 = X690_REAL_BINARY | sign_bit | base_bits | scaling_factor | X690_REAL_EXPONENT_FORMAT_2_OCTET;
        match output.write(&[ byte0, e_bytes[0], e_bytes[1] ]) {
            Err(e) => return Err(e),
            Ok(wrote) => {
                bytes_written += wrote;
            },
        }
    } else {
        let byte0: u8 = X690_REAL_BINARY | sign_bit | base_bits | scaling_factor | X690_REAL_EXPONENT_FORMAT_1_OCTET;
        match output.write(&[ byte0, e_bytes[1] ]) {
            Err(e) => return Err(e),
            Ok(wrote) => {
                bytes_written += wrote;
            },
        }
    };
    return match x690_encode_integer_value(output, mantissa as i64) {
        Err(e) => return Err(e),
        Ok(wrote) => Ok(wrote + bytes_written),
    };
}

// BitStringValue
// CharacterStringValue
// EmbeddedPDVValue
// EnumeratedValue
// ExternalValue
// InstanceOfValue
// IRIValue
// RealValue
// RelativeIRIValue
// RelativeOIDValue
// SequenceValue
// SequenceOfValue
// SetValue
// SetOfValue
// PrefixedValue
// TimeValue

#[cfg(test)]
mod tests {

    #[test]
    fn test_x690_encode_boolean_value () {
        let mut output: Vec<u8> = Vec::new();
        crate::x690_encode_boolean_value(&mut output, true).unwrap();
        crate::x690_encode_boolean_value(&mut output, false).unwrap();
        assert_eq!(output.len(), 2);
        assert!(output.starts_with(&[ 0xFF, 0x00 ]));
    }

    #[test]
    fn test_x690_encode_integer_value () {
        let mut output: Vec<u8> = Vec::new();
        let mut i = 0;
        for value in -128i8..127i8 {
            crate::x690_encode_integer_value(&mut output, i64::from(value)).unwrap();
            assert_eq!(output[i] as i8, value);
            i += 1;
        }
        assert_eq!(output.len(), 255);
    }

    // #[test]
    // fn test_x690_encode_bit_string_value () {
    //     use bitvec::prelude::*;
    //     let mut output: Vec<u8> = Vec::new();
    //     let mut bits = bitvec![usize, Lsb0; 0, 1, 0, 0, 1];
    //     crate::x690_encode_bit_string_value(&mut output, &mut bits).unwrap();
    //     // assert_eq!(output.len(), 2);
    //     assert_eq!(output[0], 3);
    //     assert_eq!(output[1], 0b0100_1000);
    // }

    #[test]
    fn test_x690_encode_octet_string_value () {
        let mut output: Vec<u8> = Vec::new();
        let bytes: Vec<u8> = vec![ 1, 3, 5, 7, 9 ];
        crate::x690_encode_octet_string_value(&mut output, &bytes).unwrap();
        assert_eq!(output.len(), 5);
        assert!(output.starts_with(&[ 1, 3, 5, 7, 9 ]));
    }

    #[test]
    fn test_x690_encode_object_identifier_value () {
        let mut output: Vec<u8> = Vec::new();
        let oid: asn1::types::OBJECT_IDENTIFIER = vec![ 2, 5, 4, 3 ];
        crate::x690_encode_object_identifier_value(&mut output, &oid).unwrap();
        assert_eq!(output.len(), 3);
        assert!(output.starts_with(&[ 0x55, 0x04, 0x03 ]));
    }

    #[test]
    fn test_x690_encode_object_descriptor_value () {
        let mut output: Vec<u8> = Vec::new();
        let value = String::from("commonName");
        crate::x690_encode_object_descriptor_value(&mut output, &value).unwrap();
        assert_eq!(output.len(), value.len());
        assert_eq!(String::from_utf8(output).unwrap(), String::from("commonName"));
    }
}
