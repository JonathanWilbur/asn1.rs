use std::io::{Write, Result, Error, ErrorKind};
use asn1::types::{BOOLEAN, Bytes, INTEGER, ByteSlice, BIT_STRING, ASN1Value, OBJECT_IDENTIFIER};

pub fn ber_encode_boolean_value <T> (output: &mut T, value: BOOLEAN) -> Result<usize>
    where T : Write {
    if value {
       return output.write(&[ 0x01, 0x01, 0xFF ]);
    } else {
        return output.write(&[ 0x01, 0x01, 0x00 ]);
    }
}

pub fn ber_decode_boolean_value (value_bytes: Bytes) -> Result<BOOLEAN> {
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

// fn ber_decode (value: ByteSlice) -> Result<ASN1Value> {
//     let (bytes_read, el) = match ber_cst(value) {
//         Err(e) => return Err(e),
//         Ok(result) => result,
//     };
//     el.
// } 

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
                let alg_id = AlgorithmIdentifier{
                    algorithm: oid,
                    parameters: Some(children[1]),
                };
            } else {
                panic!("teuye");
            }
        } else {
            panic!("zxcv");
        }
    }
}
