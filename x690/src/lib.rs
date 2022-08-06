use std::io::{Write, Result, Error, ErrorKind, IoSlice};
use std::mem::{size_of};
use asn1::types::{
    BOOLEAN,
    INTEGER,
    BIT_STRING,
    OCTET_STRING,
    OBJECT_IDENTIFIER,
    ObjectDescriptor,
    EXTERNAL,
    REAL,
    ExternalIdentification,
    SEQUENCE,
    ASN1Value,
    TagClass,
    TagNumber,
    ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT,
    ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
    ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
    ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_NULL,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
    ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
    ASN1_UNIVERSAL_TAG_NUMBER_REAL,
    ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED,
    ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV,
    ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_RESERVED_15,
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
    ASN1_UNIVERSAL_TAG_NUMBER_SET,
    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
    ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_DURATION,
    ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI,
    TaggedASN1Value,
};
use num::Zero;

pub const X690_TAG_CLASS_UNIVERSAL: u8 = 0b0000_0000;
pub const X690_TAG_CLASS_APPLICATION: u8 = 0b0100_0000;
pub const X690_TAG_CLASS_CONTEXT: u8 = 0b1000_0000;
pub const X690_TAG_CLASS_PRIVATE: u8 = 0b1100_0000;
pub const X690_SPECIAL_REAL_PLUS_INFINITY: u8 = 0b1000_0000;
pub const X690_SPECIAL_REAL_MINUS_INFINITY: u8 = 0b1000_0001;
pub const X690_SPECIAL_REAL_NOT_A_NUMBER: u8 = 0b1000_0010;
pub const X690_SPECIAL_REAL_MINUS_ZERO: u8 = 0b1000_0011;
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

pub fn x690_write_tag <W> (
    output: &mut W,
    class: TagClass,
    constructed: bool,
    tagnum: TagNumber,
) -> Result<usize>
    where W : Write {
    let k: u8 = match class {
        TagClass::UNIVERSAL => X690_TAG_CLASS_UNIVERSAL,
        TagClass::APPLICATION => X690_TAG_CLASS_APPLICATION,
        TagClass::CONTEXT => X690_TAG_CLASS_CONTEXT,
        TagClass::PRIVATE => X690_TAG_CLASS_PRIVATE,
    };
    // TODO: Review: LT or LTE?
    if tagnum < 31 { // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return output.write(&[
            k
            | if constructed { 0b0010_0000 } else { 0b0000_0000 }
            | tagnum as u8
        ]);
    } else {
        let first_byte_result = output.write(&[
            k
            | if constructed { 0b0010_0000 } else { 0b0000_0000 }
            | 0b0001_1111u8
        ]);
        if let Err(e) = first_byte_result {
            return Err(e);
        }
        return write_base_128(output, tagnum.into());
    }
}

pub fn x690_write_length <W> (
    output: &mut W,
    length: usize,
) -> Result<usize>
    where W : Write {
    if length <= 127 { // See ITU Rec. X.690 (2021), Section 8.1.3.3, "NOTE"
        return output.write(&[ length as u8 ]);
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
        let first_byte_result = output.write(&[
            0b1000_0000
            | octets_needed
        ]);
        output.write(length.to_be_bytes().as_slice())
    }
}

pub fn x690_write_boolean_value <W> (output: &mut W, value: BOOLEAN) -> Result<usize>
    where W : Write {
    if value {
       return output.write(&[0xFF]);
    } else {
        return output.write(&[0x00]);
    }
}

pub fn x690_write_integer_value <W> (output: &mut W, value: INTEGER) -> Result<usize>
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

pub fn x690_write_bit_string_value <W> (output: &mut W, value: &mut BIT_STRING) -> Result<usize>
    where W : Write {
    let padding_bits_result = output.write(&[8u8 - (value.len() % 8) as u8]);
    // let bytes = value.into_vec();
    // let bytes1: Vec<u8> = bytes
    // let bytes = value.to_vec::<u8>();
    return match padding_bits_result {
        Err(e) => { Err(e) },
        _ => {
            // let mut clone = value.clone();
            // clone.set_uninitialized(false);
            value.set_uninitialized(false);
            let mut bit_bytes_written = 1;
            for chunk in value.into_vec().iter() {
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

pub fn x690_write_octet_string_value <W> (output: &mut W, value: &OCTET_STRING) -> Result<usize>
    where W : Write {
    output.write(value)
}

pub fn x690_write_object_identifier_value <W> (output: &mut W, value: &OBJECT_IDENTIFIER) -> Result<usize>
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

pub fn x690_write_object_descriptor_value <W> (output: &mut W, value: &ObjectDescriptor) -> Result<usize>
    where W : Write {
    // TODO: Validate
    output.write(value.as_bytes())
}

// pub fn x690_write_external_value <W> (output: &mut W, value: &EXTERNAL) -> Result<usize>
// where W : Write {
//     let mut bytes_written = 0;
//     match value.identification {
//         ExternalIdentification::syntax(oid) => {
//             match output.write(&[ 0x06,  ]) {
//                 Err(e) => return Err(e),
//                 Ok(wrote_bytes) => {
//                     bytes_written += wrote_bytes;
//                 },
//             };
//             match x690_write_object_identifier_value(output, &oid) {
//                 Err(e) => return Err(e),
//                 Ok(wrote_bytes) => {
//                     bytes_written += wrote_bytes;
//                 },
//             };
//         },
//         ExternalIdentification::presentation_context_id(pci) => 
//             match x690_write_integer_value(output, pci) {
//                 Err(e) => return Err(e),
//                 Ok(wrote_bytes) => {
//                     bytes_written += wrote_bytes;
//                 },
//             },
//         ExternalIdentification::context_negotiation(cn) =>
//             match x690_write_integer_value(output, pci) {
//                 Err(e) => return Err(e),
//                 Ok(wrote_bytes) => {
//                     bytes_written += wrote_bytes;
//                 },
//             },
//     };
// }

pub fn x690_write_real_value <W> (output: &mut W, value: REAL) -> Result<usize>
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
    return match x690_write_integer_value(output, mantissa as i64) {
        Err(e) => return Err(e),
        Ok(wrote) => Ok(wrote + bytes_written),
    };
}

// pub fn x690_write_sequence_value <W> (output: &mut W, value: &SEQUENCE) -> Result<usize>
//     where W : Write {
    
// }

// ber_encode(out, value)
// ber_encode_explicit(out, value)
// ber_encode_implicit(out, value)

pub fn ber_encode <W> (output: &mut W, value: &ASN1Value) -> Result<usize>
    where W : Write {

    if let ASN1Value::TaggedValue(tv) = value {
        if tv.explicit {
            let mut outer_tag_buffer: Vec<u8> = Vec::new();
            let mut inner_buffer: Vec<u8> = Vec::new();
            let inner_result = ber_encode(&mut inner_buffer, &tv.value);
            if let Err(e) = inner_result {
                return Err(e);
            }
            let tag_result = x690_write_tag(
                &mut outer_tag_buffer,
                tv.tag_class,
                true,
                tv.tag_number,
            );
            if let Err(e) = tag_result {
                return Err(e);
            }
            let length_result = x690_write_length(
                &mut outer_tag_buffer,
                inner_buffer.len(),
            );
            if let Err(e) = length_result {
                return Err(e);
            }

            let mut bytes_written: usize = 0;
            match output.write(outer_tag_buffer.as_slice()) {
                Ok(wrote_bytes) => {
                    bytes_written += wrote_bytes;
                },
                Err(e) => return Err(e),
            };
            match output.write(inner_buffer.as_slice()) {
                Ok(wrote_bytes) => {
                    bytes_written += wrote_bytes;
                },
                Err(e) => return Err(e),
            };
            return Ok(bytes_written);
        }
    }

    let mut tag_class: TagClass = TagClass::UNIVERSAL;
    let mut constructed: bool = false;
    let mut tag_number: TagNumber = 0;
    let mut value_buffer: Vec<u8> = Vec::new();
    let mut encodable_value = value;
    // let mut tagged: Option<&TaggedASN1Value> = None;
    while let ASN1Value::TaggedValue(&inner_value) = encodable_value {
        // tagged = Some(*inner_value);
        encodable_value = &ASN1Value::TaggedValue(*inner_value);
    }

    // Write to value buffer
    match encodable_value {
        ASN1Value::BooleanValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN;
            match x690_write_boolean_value(&mut value_buffer, *v) {
                Err(e) => return Err(e),
                _ => (),
            }
        }
        _ => (),
    };

    // If tagged and implicit, overwrite tags
    if let ASN1Value::TaggedValue(tv) = value {
        if !tv.explicit { // IMPLICIT
            tag_class = tv.tag_class;
            tag_number = tv.tag_number;
        }
    }

    let tag_result = x690_write_tag(
        output,
        tag_class,
        constructed,
        tag_number,
    );
    if let Err(e) = tag_result {
        return Err(e);
    }
    let length_result = x690_write_length(
        output,
        value_buffer.len(),
    );
    if let Err(e) = length_result {
        return Err(e);
    }
    output.write(value_buffer.as_slice())
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

/*

WHAT I AM CURRENTLY STUCK ON:

A chicken-and-egg problem. I need to know the length of the encoded value before
I can write the tag and length, but I need to write the tag and length before I
write the encoded value.

NON SOLUTIONS:

I could write functions for each type that determine how big the encoded values
will be, but this will be very susceptible to bugs: the "size estimator" could
calculate that an INTEGER will be encoded on five bytes, but then it gets
encoded on four. What then?

You also cannot leave the length field blank and go back and correct it. Once
you've written it, it is gone. It is also variable-size.

POSSIBLE SOLUTION:

The general `ber_encode()` function will return `(Bytes, Bytes)`. The first
vector will be the bytes of the tag and length, and the second will be the bytes
of the encoded value. This would also make using IMPLICIT tags much more
straight-forward.

Actually, I think the only right way for this to be done is to return `Bytes`
for every type-specific _value_ encoder. The actual element encoder should just
return `Result<usize>`. Unfortunately, this means that you must allocate space
for every value to be encoded right away, but you can then use it right away.

This would also allow manipulation of values prior to transmission, which could
be useful.

Also, you can optimize `ber_write()` to write tags and values directly if their
length can be easily calculated, which should help. This should be easy to
calculate for things like BOOLEAN, OCTET STRING, and string types.

Now that I think about it, I don't even have to change the code I already have:
Rust does not allow you to return new references: you have to pass them in
anyway. So you have to pass in a writer anyway.

*/

#[cfg(test)]
mod tests {

    #[test]
    fn test_x690_write_boolean_value () {
        let mut output: Vec<u8> = Vec::new();
        crate::x690_write_boolean_value(&mut output, true).unwrap();
        crate::x690_write_boolean_value(&mut output, false).unwrap();
        assert_eq!(output.len(), 2);
        assert!(output.starts_with(&[ 0xFF, 0x00 ]));
    }

    #[test]
    fn test_x690_write_integer_value () {
        let mut output: Vec<u8> = Vec::new();
        let mut i = 0;
        for value in -128i8..127i8 {
            crate::x690_write_integer_value(&mut output, i64::from(value)).unwrap();
            assert_eq!(output[i] as i8, value);
            i += 1;
        }
        assert_eq!(output.len(), 255);
    }

    // #[test]
    // fn test_x690_write_bit_string_value () {
    //     use bitvec::prelude::*;
    //     let mut output: Vec<u8> = Vec::new();
    //     let mut bits = bitvec![usize, Lsb0; 0, 1, 0, 0, 1];
    //     crate::x690_write_bit_string_value(&mut output, &mut bits).unwrap();
    //     // assert_eq!(output.len(), 2);
    //     assert_eq!(output[0], 3);
    //     assert_eq!(output[1], 0b0100_1000);
    // }

    use asn1::types::{ASN1Value, TagClass, TaggedASN1Value, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN};

    use crate::{X690_TAG_CLASS_APPLICATION, X690_TAG_CLASS_CONTEXT};

    #[test]
    fn test_x690_write_octet_string_value () {
        let mut output: Vec<u8> = Vec::new();
        let bytes: Vec<u8> = vec![ 1, 3, 5, 7, 9 ];
        crate::x690_write_octet_string_value(&mut output, &bytes).unwrap();
        assert_eq!(output.len(), 5);
        assert!(output.starts_with(&[ 1, 3, 5, 7, 9 ]));
    }

    #[test]
    fn test_x690_write_object_identifier_value () {
        let mut output: Vec<u8> = Vec::new();
        let oid: asn1::types::OBJECT_IDENTIFIER = vec![ 2, 5, 4, 3 ];
        crate::x690_write_object_identifier_value(&mut output, &oid).unwrap();
        assert_eq!(output.len(), 3);
        assert!(output.starts_with(&[ 0x55, 0x04, 0x03 ]));
    }

    #[test]
    fn test_x690_write_object_descriptor_value () {
        let mut output: Vec<u8> = Vec::new();
        let value = String::from("commonName");
        crate::x690_write_object_descriptor_value(&mut output, &value).unwrap();
        assert_eq!(output.len(), value.len());
        assert_eq!(String::from_utf8(output).unwrap(), String::from("commonName"));
    }

    #[test]
    fn test_x690_write_real_value () {
        let mut output: Vec<u8> = Vec::new();
        let value = 1.2345;
        crate::x690_write_real_value(&mut output, value).unwrap();
    }

    #[test]
    fn test_ber_encode () {
        let mut output: Vec<u8> = Vec::new();
        let value: ASN1Value = ASN1Value::TaggedValue(&TaggedASN1Value{
            tag_class: TagClass::APPLICATION,
            explicit: true,
            tag_number: 5,
            value: ASN1Value::BooleanValue(true),
        });
        let result = crate::ber_encode(&mut output, &value).unwrap();
        assert_eq!(result, output.len());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_ber_encode_deep_tagging_1 () {
        let mut output: Vec<u8> = Vec::new();
        let value: ASN1Value = ASN1Value::TaggedValue(&TaggedASN1Value{
            tag_class: TagClass::APPLICATION,
            explicit: true,
            tag_number: 5,
            value: ASN1Value::TaggedValue(&TaggedASN1Value{
                tag_class: TagClass::CONTEXT,
                explicit: true,
                tag_number: 7,
                value: ASN1Value::BooleanValue(false),
            }),
        });
        let result = crate::ber_encode(&mut output, &value).unwrap();
        assert_eq!(result, output.len());
        assert_eq!(result, 7);
        assert!(output.starts_with(&[
            X690_TAG_CLASS_APPLICATION
            | 0b0010_0000 // Constructed
            | 5,
            0x05, // Length = 5
            X690_TAG_CLASS_CONTEXT
            | 0b0010_0000 // Constructed
            | 7,
            0x03, // Length = 3
            ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN as u8,
            0x01, // Length = 1
            0x00, // FALSE
        ]));
    }

    #[test]
    fn test_ber_encode_deep_tagging_2 () {
        let mut output: Vec<u8> = Vec::new();
        let value: ASN1Value = ASN1Value::TaggedValue(&TaggedASN1Value{
            tag_class: TagClass::APPLICATION,
            explicit: false,
            tag_number: 5,
            value: ASN1Value::TaggedValue(&TaggedASN1Value{
                tag_class: TagClass::CONTEXT,
                explicit: false,
                tag_number: 7,
                value: ASN1Value::BooleanValue(false),
            }),
        });
        let result = crate::ber_encode(&mut output, &value).unwrap();
        assert_eq!(result, output.len());
        assert_eq!(result, 3);
        assert!(output.starts_with(&[
            X690_TAG_CLASS_APPLICATION
            | 0b0010_0000 // Constructed
            | 5,
            0x01, // Length = 5
            0x00, // FALSE
        ]));
    }
}

// TODO: Tests to verify that my error handling from nested matches actually works.