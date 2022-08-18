use core::slice::SlicePattern;
use std::io::{Write, Result, Error, ErrorKind, IoSlice, Read};
use std::mem::size_of;
use std::fmt::format;
use asn1::types::{
    Bytes,
    ByteSlice,
    OPTIONAL,
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
    UTCTime,
    GeneralizedTime,
    GeneralString,
    BMPString,
    UniversalString,
    CHARACTER_STRING,
    DATE,
    DATE_TIME,
    DURATION,
    TIME_OF_DAY,
    ASN1Value,
    TagClass,
    TagNumber,
    PresentationContextSwitchingTypeIdentification,
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
    TaggedASN1Value, EmbeddedPDV, CharacterString, UTF8String, RELATIVE_OID, TIME, External,
    MAX_IA5_STRING_CHAR_CODE, DURATION_EQUIVALENT,
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

#[derive(Clone, Debug)]
pub enum X690Length {
    Definite(usize),
    Indefinite,
}

#[derive(Clone, Debug)]
pub enum X690Encoding {
    // TODO: Convert to ByteSlice
    IMPLICIT(Bytes), // the value bytes
    EXPLICIT(Box<X690Element>), // the inner TLV tuple
    Constructed(Vec<X690Element>), // an array of inner TLV tuples
    // TODO: Convert to ByteSlice.
    AlreadyEncoded(Bytes), // the already-encoded TLV
}

#[derive(Clone, Debug)]
pub struct X690Tag {
    pub tag_class: TagClass,
    pub constructed: bool,
    pub tag_number: TagNumber,
}

#[derive(Clone, Debug)]
pub struct X690Element {
    pub name: Option<String>,
    pub tag_class: TagClass,
    pub tag_number: TagNumber,
    pub value: X690Encoding,
    last_calculated_length: Option<usize>, // Maybe not pub
}

impl X690Encoding {
    pub fn len (&self) -> usize {
        let ret = match self {
            X690Encoding::IMPLICIT(value_bytes) => value_bytes.len(),
            X690Encoding::EXPLICIT(inner) => inner.len(),
            X690Encoding::Constructed(components) => {
                let mut sum: usize = 0;
                for c in components {
                    sum += c.len();
                }
                sum
            },
            X690Encoding::AlreadyEncoded(encoded_value) => {
                // TODO: Skip over tag and length bytes
                let tag_and_length_bytes = get_x690_tag_and_length_length(encoded_value);
                encoded_value.len() - tag_and_length_bytes
            },
        };
        ret
    }

    // TODO: is_empty()
}

impl X690Element {
    pub fn new (tag_class: TagClass, tag_number: TagNumber, value: X690Encoding) -> X690Element {
        X690Element {
            name: None,
            tag_class,
            tag_number,
            value,
            last_calculated_length: None
        }
    }

    pub fn len (&self) -> usize {
        match self.last_calculated_length {
            Some(l) => return l,
            None => {
                let tag_length: usize = get_written_x690_tag_length(self.tag_class, self.tag_number);
                let value_length = self.value.len();
                let length_length: usize = get_written_x690_length_length(value_length);
                let ret = tag_length + length_length + value_length;
                // self.last_calculated_length = Some(ret);
                ret
            },
        }
    }

    // TODO: is_empty()
}

fn get_x690_tag_and_length_length (bytes: &Bytes) -> usize {
    if bytes.len() == 0 {
        return 0;
    }
    let mut len: usize = 1;
    if (bytes[0] & 0b00011111) == 0b00011111 { // If it is a long tag...
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
    if (length_byte_0 & 0b1000_0000) == 0 { // Short definite form or indefinite form.
        return len;
    }
    (length_byte_0 & 0b0111_1111) as usize
}

// This is a struct with a single field for the sake of extensibility.
pub struct X690ConcreteSyntaxTree {
    pub root: X690Element,
}

fn base_128_len (num: u32) -> usize {
    if num < 128 {
        return 1;
    }

    let mut l = 0;
    let mut i = num;
    while i > 0 {
        l += 1;
        i >>= 7;
    }

    let mut bytes: usize = 0;
    for i in (0..l).rev() {
        bytes += 1;
    }
    return bytes;
}

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

// Just gets the theoretical length of the written tag.
pub fn get_written_x690_tag_length (
    class: TagClass,
    tagnum: TagNumber,
) -> usize {
    if tagnum < 31 { // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return 1;
    }
    base_128_len(tagnum.into()) + 1
}

// Just gets the length of the length bytes for an element
pub fn get_written_x690_length_length (len: usize) -> usize {
    if len <= 127 { // See ITU Rec. X.690 (2021), Section 8.1.3.3, "NOTE"
        return 1;
    }
    let octets_needed: usize = match len {
        0..=255 => 1,
        256..=65535 => 2,
        65536..=16777215 => 3,
        16777216..=4294967295 => 4,
        _ => return 4, // TODO: What do you do about this scenario?
    };
    octets_needed + 1
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

pub fn x690_write_bit_string_value <W> (output: &mut W, value: &BIT_STRING) -> Result<usize>
    where W : Write {
    match output.write(&[value.trailing_bits % 8]) {
        Err(e) => return Err(e),
        _ => (),
    }
    match output.write(&value.bytes) {
        Ok(bytes_written) => return Ok(bytes_written + 1),
        Err(e) => return Err(e),
    }
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
        _ => (),
    };
    bytes_written += 1;
    for arc in value[2..].iter() {
        match write_base_128(output, *arc) {
            Err(e) => { return Err(e) },
            Ok(wrote_bytes) => {
                bytes_written += wrote_bytes;
            }
        };
    }
    Ok(bytes_written)
}

pub fn x690_write_object_descriptor_value <W> (output: &mut W, value: &ObjectDescriptor) -> Result<usize>
    where W : Write {
    // TODO: Validate
    output.write(value.as_bytes())
}

// NOTE: This has to be encoded in a strange way that is detailed in ITU Recommendation X.690, Section 8.18.
pub fn x690_write_external_value <W> (output: &mut W, value: &EXTERNAL) -> Result<usize>
where W : Write {
    let mut inner_elements: Vec<X690Element> = Vec::new();
    match &value.identification {
        ExternalIdentification::syntax(oid) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut bytes, &oid) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                X690Encoding::IMPLICIT(bytes),
            );
            inner_elements.push(element);
        },
        ExternalIdentification::presentation_context_id(pci) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_integer_value(&mut bytes, *pci) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
                X690Encoding::IMPLICIT(bytes),
            );
            inner_elements.push(element);
        },
        ExternalIdentification::context_negotiation(cn) => {
            let mut direct_ref_bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut direct_ref_bytes, &cn.transfer_syntax) {
                Err(e) => return Err(e),
                _ => (),
            };
            let direct_ref_element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                X690Encoding::IMPLICIT(direct_ref_bytes),
            );
            inner_elements.push(direct_ref_element);
            let mut indirect_ref_bytes: Bytes = Vec::new();
            match x690_write_integer_value(&mut indirect_ref_bytes, cn.presentation_context_id) {
                Err(e) => return Err(e),
                _ => (),
            };
            let indirect_ref_element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
                X690Encoding::IMPLICIT(indirect_ref_bytes),
            );
            inner_elements.push(indirect_ref_element);
        },
    };
    match &value.data_value_descriptor {
        Some(dvd) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_object_descriptor_value(&mut bytes, &dvd) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
                X690Encoding::IMPLICIT(bytes),
            );
            inner_elements.push(element);
        },
        None => (),
    };
    let mut data_value_bytes: Bytes = Vec::new();
    match x690_write_octet_string_value(&mut data_value_bytes, &value.data_value) {
        Err(e) => return Err(e),
        _ => (),
    };
    let data_value_element = X690Element::new(
        TagClass::CONTEXT,
        1,
        X690Encoding::IMPLICIT(data_value_bytes),
    );
    inner_elements.push(data_value_element);
    let mut bytes_written: usize = 0;
    for component in inner_elements {
        match write_x690_node(output, &component) {
            Ok(length) => {
                bytes_written += length;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(bytes_written)
}

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

pub fn x690_context_switching_identification_to_cst (id: &PresentationContextSwitchingTypeIdentification) -> Result<X690Element> {
    match id {
        PresentationContextSwitchingTypeIdentification::syntaxes(syntaxes) => {
            let mut abstract_value_bytes: Bytes = Vec::new();
            let mut transfer_value_bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut abstract_value_bytes, &syntaxes.r#abstract) {
                Err(e) => return Err(e),
                _ => (),
            };
            match x690_write_object_identifier_value(&mut transfer_value_bytes, &syntaxes.transfer) {
                Err(e) => return Err(e),
                _ => (),
            };
            let mut syntaxes_elements: Vec<X690Element> = Vec::new();
            syntaxes_elements.push(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::IMPLICIT(abstract_value_bytes),
            ));
            syntaxes_elements.push(X690Element::new(
                TagClass::CONTEXT,
                1,
                X690Encoding::IMPLICIT(transfer_value_bytes),
            ));
            let element = X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::Constructed(syntaxes_elements),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::EXPLICIT(Box::new(element)),
            ));
        },
        PresentationContextSwitchingTypeIdentification::syntax(oid) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut bytes, &oid) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::CONTEXT,
                1,
                X690Encoding::IMPLICIT(bytes),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::EXPLICIT(Box::new(element)),
            ));
        },
        PresentationContextSwitchingTypeIdentification::presentation_context_id(pci) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_integer_value(&mut bytes, *pci) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::CONTEXT,
                2,
                X690Encoding::IMPLICIT(bytes),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::EXPLICIT(Box::new(element)),
            ));
        },
        PresentationContextSwitchingTypeIdentification::context_negotiation(cn) => {
            let mut pci_bytes: Bytes = Vec::new();
            match x690_write_integer_value(&mut pci_bytes, cn.presentation_context_id) {
                Err(e) => return Err(e),
                _ => (),
            };
            let pci_element = X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::IMPLICIT(pci_bytes),
            );
            let mut transfer_syntax_bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut transfer_syntax_bytes, &cn.transfer_syntax) {
                Err(e) => return Err(e),
                _ => (),
            };
            let transfer_syntax_element = X690Element::new(
                TagClass::CONTEXT,
                1,
                X690Encoding::IMPLICIT(transfer_syntax_bytes),
            );
            let cn_elements: Vec<X690Element> = vec![ pci_element, transfer_syntax_element ];
            let element = X690Element::new(
                TagClass::CONTEXT,
                3,
                X690Encoding::Constructed(cn_elements),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::EXPLICIT(Box::new(element)),
            ));
        },
        PresentationContextSwitchingTypeIdentification::transfer_syntax(ts) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut bytes, &ts) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::CONTEXT,
                4,
                X690Encoding::IMPLICIT(bytes),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::EXPLICIT(Box::new(element)),
            ));
        },
        PresentationContextSwitchingTypeIdentification::fixed => {
            let element = X690Element::new(
                TagClass::CONTEXT,
                5,
                X690Encoding::IMPLICIT(Vec::new()),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                X690Encoding::EXPLICIT(Box::new(element)),
            ));
        },
    }
}

pub fn x690_write_embedded_pdv_value <W> (output: &mut W, value: &EmbeddedPDV) -> Result<usize>
where W : Write {
    let id = x690_context_switching_identification_to_cst(&value.identification);
    if let Err(e) = id {
        return Err(e);
    }
    let id_element = id.unwrap();
    let mut data_value_bytes: Bytes = Vec::new();
    match x690_write_octet_string_value(&mut data_value_bytes, &value.data_value) {
        Err(e) => return Err(e),
        _ => (),
    };
    let data_value_element = X690Element::new(
        TagClass::CONTEXT,
        1,
        X690Encoding::IMPLICIT(data_value_bytes),
    );
    let inner_elements: Vec<X690Element> = vec![ id_element, data_value_element ];
    let mut bytes_written: usize = 0;
    for component in inner_elements {
        match write_x690_node(output, &component) {
            Ok(length) => {
                bytes_written += length;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(bytes_written)
}

pub fn x690_write_utf8_string_value <W> (output: &mut W, value: &UTF8String) -> Result<usize>
where W : Write {
    output.write(value.as_bytes())
}

pub fn x690_write_relative_oid_value <W> (output: &mut W, value: &RELATIVE_OID) -> Result<usize>
where W : Write {
    let mut bytes_written: usize = 0;
    for arc in value.iter() {
        match write_base_128(output, *arc) {
            Err(e) => { return Err(e) },
            Ok(wrote_bytes) => {
                bytes_written += wrote_bytes;
            }
        };
    }
    Ok(bytes_written)
}

// TODO: Validate
pub fn x690_write_time_value <W> (output: &mut W, value: &TIME) -> Result<usize>
where W : Write {
    output.write(value.as_bytes())
}

pub fn x690_write_utc_time_value <W> (output: &mut W, value: &UTCTime) -> Result<usize>
where W : Write {
    if
        value.year > 99
        || value.month > 12
        || value.month == 0
        || value.day > 31
        || value.day == 0
        || value.hour > 23
        || value.minute > 59
    {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_form = format!(
        "{:02}{:02}{:02}{:02}{:02}",
        value.year % 100,
        value.month,
        value.day,
        value.hour,
        value.minute,
    );
    match output.write(str_form.as_bytes()) {
        Err(e) => return Err(e),
        _ => (),
    };
    let mut bytes_written: usize = 10;
    match value.second {
        Some(sec) => {
            if sec > 59 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            match output.write(format!("{:02}", sec).as_bytes()) {
                Err(e) => return Err(e),
                _ => (),
            };
            bytes_written += 2;
        },
        _ => (),
    };
    match value.utc_offset {
        Some(offset) => {
            if offset.hour > 23 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            if offset.minute > 59 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            match output.write(format!("{:+03}{:02}", offset.hour, offset.minute).as_bytes()) {
                Err(e) => return Err(e),
                _ => (),
            };
            bytes_written += 5;
        },
        _ => (),
    };
    Ok(bytes_written)
}

pub fn x690_write_generalized_time_value <W> (output: &mut W, value: &GeneralizedTime) -> Result<usize>
where W : Write {
    if
        value.date.month > 12
        || value.date.month == 0
        || value.date.day > 31
        || value.date.day == 0
        || value.hour > 23
    {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_form = format!(
        "{:04}{:02}{:02}{:02}",
        value.date.year % 10000,
        value.date.month,
        value.date.day,
        value.hour,
    );
    match output.write(str_form.as_bytes()) {
        Err(e) => return Err(e),
        _ => (),
    };
    let mut bytes_written: usize = 10;
    match value.minute {
        Some(min) => {
            if min > 59 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            match output.write(format!("{:02}", min).as_bytes()) {
                Err(e) => return Err(e),
                _ => (),
            };
            bytes_written += 2;
            match value.second {
                Some(sec) => {
                    if sec > 59 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    match output.write(format!("{:02}", sec).as_bytes()) {
                        Err(e) => return Err(e),
                        _ => (),
                    };
                    bytes_written += 2;
                    match value.fraction {
                        Some(ms) => {
                            if ms > 999 {
                                return Err(Error::from(ErrorKind::InvalidData));
                            }
                            match output.write(format!(".{:03}", ms).as_bytes()) {
                                Err(e) => return Err(e),
                                _ => (),
                            };
                            bytes_written += 4;
                        },
                        _ => (),
                    };

                },
                _ => (),
            };
        },
        _ => (),
    };
    match value.utc_offset {
        Some(offset) => {
            if offset.hour > 23 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            if offset.minute > 59 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            match output.write(format!("{:+03}{:02}", offset.hour, offset.minute).as_bytes()) {
                Err(e) => return Err(e),
                _ => (),
            };
            bytes_written += 5;
        },
        None => {
            match output.write("Z".as_bytes()) {
                Err(e) => return Err(e),
                _ => (),
            };
            bytes_written += 1;
        },
    };
    Ok(bytes_written)
}

pub fn x690_write_universal_string_value <W> (output: &mut W, value: &UniversalString) -> Result<usize>
where W : Write {
    let bytes: Vec<u8> = value
        .chars()
        .map(|c| c as u32)
        .flat_map(|c| [
            ((c & 0xFF000000) >> 24) as u8,
            ((c & 0x00FF0000) >> 16) as u8,
            ((c & 0x0000FF00) >> 8) as u8,
            (c & 0x000000FF) as u8,
        ])
        .collect();
    output.write(bytes.as_slice())
}

// This is almost the same for EmbeddedPDV.
pub fn x690_write_character_string_value <W> (output: &mut W, value: &CharacterString) -> Result<usize>
where W : Write {
    let id = x690_context_switching_identification_to_cst(&value.identification);
    if let Err(e) = id {
        return Err(e);
    }
    let id_element = id.unwrap();
    let mut data_value_bytes: Bytes = Vec::new();
    match x690_write_octet_string_value(&mut data_value_bytes, &value.string_value) {
        Err(e) => return Err(e),
        _ => (),
    };
    let data_value_element = X690Element::new(
        TagClass::CONTEXT,
        1,
        X690Encoding::IMPLICIT(data_value_bytes),
    );
    let inner_elements: Vec<X690Element> = vec![ id_element, data_value_element ];
    let mut bytes_written: usize = 0;
    for component in inner_elements {
        match write_x690_node(output, &component) {
            Ok(length) => {
                bytes_written += length;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(bytes_written)
}

pub fn x690_write_bmp_string_value <W> (output: &mut W, value: &UniversalString) -> Result<usize>
where W : Write {
    let bytes: Vec<u8> = value
        .chars()
        .map(|c| c as u32)
        .flat_map(|c| [
            // ((c & 0xFF000000) >> 24) as u8,
            // ((c & 0x00FF0000) >> 16) as u8,
            ((c & 0x0000FF00) >> 8) as u8,
            (c & 0x000000FF) as u8,
        ])
        .collect();
    output.write(bytes.as_slice())
}

// This does not do any validation.
#[inline]
pub fn x690_write_string_value <W> (output: &mut W, value: &String) -> Result<usize>
where W : Write {
    output.write(value.as_bytes())
}

pub fn x690_write_date_value <W> (output: &mut W, value: &DATE) -> Result<usize>
where W : Write {
    if
        value.month > 12
        || value.month == 0
        || value.day > 31
        || value.day == 0
    {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_form = format!(
        "{:04}-{:02}-{:02}",
        value.year % 10000,
        value.month,
        value.day,
    );
    output.write(str_form.as_bytes())
}

pub fn x690_write_time_of_day_value <W> (output: &mut W, value: &TIME_OF_DAY) -> Result<usize>
where W : Write {
    if
        value.hour > 23
        || value.minute > 59
        || value.second > 59
    {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_form = format!(
        "{:02}:{:02}:{:02}",
        value.hour,
        value.minute,
        value.second,
    );
    output.write(str_form.as_bytes())
}

pub fn x690_write_date_time_value <W> (output: &mut W, value: &DATE_TIME) -> Result<usize>
where W : Write {
    if
        value.date.year > 9999
        || value.date.month > 12
        || value.date.month == 0
        || value.date.day > 31
        || value.date.day == 0
        || value.time.hour > 23
        || value.time.minute > 59
        || value.time.second > 59
    {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let str_form = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        value.time.hour,
        value.time.minute,
        value.time.second,
    );
    output.write(str_form.as_bytes())
}

pub fn x690_write_duration_value <W> (output: &mut W, value: &DURATION_EQUIVALENT) -> Result<usize>
where W : Write {
    let mut parts: Vec<String> = vec![ String::from("P") ];
    if value.years > 0 {
        parts.push(format!("{}Y", value.years));
    }
    if value.months > 0 {
        parts.push(format!("{}M", value.months));
    }
    if value.weeks > 0 {
        parts.push(format!("{}W", value.weeks));
    }
    if value.days > 0 {
        parts.push(format!("{}D", value.days));
    }
    if value.hours > 0 {
        parts.push(format!("{}H", value.hours));
    }
    if value.minutes > 0 {
        parts.push(format!("{}M", value.minutes));
    }
    if value.seconds > 0 {
        parts.push(format!("{}S", value.seconds));
    }
    // TODO: This definitely needs some testing.
    if let Some(frac) = value.fractional_part {
        let last_part = parts.last_mut();
        match last_part {
            Some(part) => {
                let last_char = part.pop();
                match last_char {
                    Some(c) => {
                        parts.push(format!(".{:>width$}{}", frac.fractional_value, c, width=frac.number_of_digits));
                    },
                    None => return Err(Error::from(ErrorKind::InvalidData)),
                }
            },
            None => {
                parts.push(format!("0.{:>width$}S", frac.fractional_value, width=frac.number_of_digits));
            },
        };
    }
    output.write(str_form.as_bytes())
}

// TODO: Add check for infinite recursion
pub fn create_x690_cst_node <'a> (value: &ASN1Value) -> Result<X690Element> {
    let mut tag_class: TagClass = TagClass::UNIVERSAL;
    let mut tag_number: TagNumber = 0;
    let mut encoded_value: X690Encoding = X690Encoding::IMPLICIT(vec![]);
    match value {
        ASN1Value::TaggedValue(v) => {
            tag_class = v.tag_class;
            tag_number = v.tag_number;
            if v.explicit {
                match create_x690_cst(&v.value) {
                    Ok(cst) => {
                        encoded_value = X690Encoding::EXPLICIT(Box::new(cst.root));
                    }
                    Err(e) => return Err(e),
                };
            } else {
                match create_x690_cst(&v.value) {
                    Ok(cst) => {
                        encoded_value = cst.root.value;
                    }
                    Err(e) => return Err(e),
                };
            }
        },
        ASN1Value::BooleanValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_boolean_value(&mut value_bytes, *v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        // TODO: Handle a BIGINT type
        ASN1Value::IntegerValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_INTEGER;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_integer_value(&mut value_bytes, *v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::BitStringValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_bit_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::OctetStringValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_octet_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::NullValue => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_NULL;
            encoded_value = X690Encoding::IMPLICIT(vec![]);
        },
        ASN1Value::ObjectIdentifierValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_object_identifier_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::ExternalValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_external_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::RealValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_REAL;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_real_value(&mut value_bytes, *v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::EnumeratedValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_integer_value(&mut value_bytes, *v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::EmbeddedPDVValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_embedded_pdv_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::UTF8String(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_utf8_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::RelativeOIDValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_relative_oid_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::TimeValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_TIME;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_time_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::SequenceValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE;
            let mut inner_values: Vec<X690Element> = Vec::new();
            for inner_value in v {
                match create_x690_cst_node(inner_value) {
                    Ok(inner_node) => {
                        inner_values.push(inner_node);
                    }
                    Err(e) => return Err(e),
                };
            }
            encoded_value = X690Encoding::Constructed(inner_values);
        },
        ASN1Value::SequenceOfValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF;
            let mut inner_values: Vec<X690Element> = Vec::new();
            for inner_value in v {
                match create_x690_cst_node(inner_value) {
                    Ok(inner_node) => {
                        inner_values.push(inner_node);
                    }
                    Err(e) => return Err(e),
                };
            }
            encoded_value = X690Encoding::Constructed(inner_values);
        },
        ASN1Value::SetValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SET;
            let mut inner_values: Vec<X690Element> = Vec::new();
            for inner_value in v {
                match create_x690_cst_node(inner_value) {
                    Ok(inner_node) => {
                        inner_values.push(inner_node);
                    }
                    Err(e) => return Err(e),
                };
            }
            encoded_value = X690Encoding::Constructed(inner_values);
        },
        ASN1Value::SetOfValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SET_OF;
            let mut inner_values: Vec<X690Element> = Vec::new();
            for inner_value in v {
                match create_x690_cst_node(inner_value) {
                    Ok(inner_node) => {
                        inner_values.push(inner_node);
                    }
                    Err(e) => return Err(e),
                };
            }
            encoded_value = X690Encoding::Constructed(inner_values);
        },
        ASN1Value::UTCTime(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_utc_time_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::GeneralizedTime(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_generalized_time_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::UniversalString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_universal_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::UnrestrictedCharacterStringValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_character_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::BMPString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_bmp_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::InstanceOfValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL;
            let type_id = ASN1Value::ObjectIdentifierValue(v.type_id);
            let value = ASN1Value::TaggedValue(
                &TaggedASN1Value {
                    tag_class: TagClass::CONTEXT,
                    tag_number: 0,
                    explicit: true,
                    value: *v.value,
                },
            );
            let type_id_element = match create_x690_cst(&type_id) {
                Err(e) => return Err(e),
                Ok(cst) => cst.root,
            };
            let value_element = match create_x690_cst(&value) {
                Err(e) => return Err(e),
                Ok(cst) => cst.root,
            };
            encoded_value = X690Encoding::Constructed(vec![
                type_id_element,
                value_element,
            ]);
        },
        ASN1Value::IRIValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::RelativeIRIValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::GeneralString(v) => {
            if !v.is_ascii() { // GeneralString must be below or at 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::IA5String(v) => {
            for c in v.chars() {
                if c > MAX_IA5_STRING_CHAR_CODE {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::GraphicString(v) => {
            for c in v.chars() {
                if !c.is_ascii_graphic() && (c != ' ') {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::ISO646String(v) => {
            if !v.is_ascii() { // VisibleString / ISO646String must be below 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            for c in v.chars() {
                if c == '\x7F' { // DELETE not allowed.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::VisibleString(v) => {
            if !v.is_ascii() { // VisibleString / ISO646String must be below 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            for c in v.chars() {
                if c == '\x7F' { // DELETE not allowed.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::NumericString(v) => {
            for c in v.chars() {
                if !c.is_ascii_digit() && c != ' ' { // SPACE is allowed in NumericString.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::PrintableString(v) => {
            for c in v.chars() {
                let code = c as u32;
                if c.is_ascii_alphanumeric()
                    || (c >= '\x27' && c < '0' && c != '*') // '()+,-./ BUT NOT *
                    || c == ' '
                    || c == ':'
                    || c == '='
                    || c == '?' {
                    continue;
                }
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_string_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::TeletexString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match value_bytes.write(v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::T61String(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match value_bytes.write(v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::VideotexString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            match value_bytes.write(v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::DATE(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_DATE;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_date_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::TIME_OF_DAY(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_time_of_day_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::DATE_TIME(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_date_time_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::DURATION(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_DURATION;
            let mut value_bytes: Vec<u8> = Vec::new();
            match x690_write_duration_value(&mut value_bytes, v) {
                Err(e) => return Err(e),
                _ => (),
            };
            encoded_value = X690Encoding::IMPLICIT(value_bytes);
        },
        ASN1Value::ChoiceValue(v) => {
            return create_x690_cst_node(v);
        },
    };

    Ok(X690Element::new(tag_class, tag_number, encoded_value))
}

// TODO: Use this in ::new()
pub fn create_x690_cst <'a> (value: &ASN1Value) -> Result<X690ConcreteSyntaxTree> {
    match create_x690_cst_node(value) {
        Ok(root_node) => Ok(X690ConcreteSyntaxTree{
            root: root_node,
        }),
        Err(e) => Err(e),
    }
}

// TODO:
// pub fn get_x690_cst <'a> (bytes: Bytes) -> Result<X690ConcreteSyntaxTree<'a>> {

// }

fn write_x690_encoding <W> (output: &mut W, encoding: &X690Encoding) -> Result<usize>
    where W : Write {
    match encoding {
        X690Encoding::IMPLICIT(value_bytes) => output.write(value_bytes.as_slice()),
        X690Encoding::EXPLICIT(inner) => write_x690_node(output, inner),
        X690Encoding::Constructed(components) => {
            let mut sum: usize = 0;
            for c in components {
                match write_x690_node(output, c) {
                    Ok(bytes_written) => {
                        sum += bytes_written;
                    },
                    Err(e) => return Err(e),
                };
            }
            Ok(sum)
        },
        X690Encoding::AlreadyEncoded(encoded_value) => Ok(0), // This should never happen.
    }
}

pub fn write_x690_node <W> (output: &mut W, node: &X690Element) -> Result<usize>
    where W : Write {
    if let X690Encoding::AlreadyEncoded(already_encoded_bytes) = &node.value {
        return output.write(already_encoded_bytes.as_slice());
    }
    let tag_class = node.tag_class;
    let tag_number = node.tag_number;
    let mut constructed: bool = false;
    if let X690Encoding::Constructed(_) = &node.value {
        constructed = true;
    } else if let X690Encoding::EXPLICIT(_) = &node.value {
        constructed = true;
    }
    let len = node.value.len();
    let mut bytes_written: usize = 0;
    match x690_write_tag(output, tag_class, constructed, tag_number) {
        Ok(wrote) => {
            bytes_written += wrote;
        },
        Err(e) => return Err(e),
    };
    match x690_write_length(output, len) {
        Ok(wrote) => {
            bytes_written += wrote;
        },
        Err(e) => return Err(e),
    };
    match write_x690_encoding(output, &node.value) {
        Ok(wrote) => {
            bytes_written += wrote;
        },
        Err(e) => return Err(e),
    };
    Ok(bytes_written)
}

pub fn ber_encode <W> (output: &mut W, value: &ASN1Value) -> Result<usize>
    where W : Write {
    let cst = match create_x690_cst(value) {
        Ok(cst) => cst,
        Err(e) => return Err(e),
    };
    write_x690_node(output, &cst.root)
}

pub fn ber_decode_tag (bytes: ByteSlice) -> Result<(usize, X690Tag)> {
    if bytes.len() == 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let mut bytes_read = 1;
    let tag_class = match (bytes[0] & 0b1100_0000) >> 6 {
        0 => TagClass::UNIVERSAL,
        1 => TagClass::CONTEXT,
        2 => TagClass::PRIVATE,
        3 => TagClass::APPLICATION,
        _ => panic!("Impossible tag class"),
    };
    let constructed = (bytes[0] & 0b0010_0000) > 0;
    let mut tag_number: TagNumber = 0;

    if (bytes[0] & 0b00011111) == 0b00011111 { // If it is a long tag...
        for byte in bytes[1..].into_iter() {
            let final_byte: bool = ((*byte) & 0b1000_0000) == 0;
            if (tag_number > 0) && !final_byte { // tag_number > 0 means we've already processed one byte.
                // Tag encoded on more than 14 bits / two bytes.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            let seven_bits = ((*byte) & 0b0111_1111) as u16;
            if !final_byte && (seven_bits == 0) {
                // You cannot encode a long tag with padding bytes.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number <<= 7;
            tag_number += seven_bits;
            bytes_read += 1;
            if final_byte {
                break;   
            }
        }
        if tag_number <= 31 { // TODO: Review this number / comparison.
            // This could have been encoded in short form.
            return Err(Error::from(ErrorKind::InvalidData));
        }
    } else {
        tag_number = (bytes[0] & 0b00011111) as TagNumber;
    }

    let tag = X690Tag{tag_class, constructed, tag_number};
    Ok((bytes_read, tag))
}

pub fn ber_decode_length (bytes: ByteSlice) -> Result<(usize, X690Length)> {
    if bytes.len() == 0 {
        // Truncated.
        return Err(Error::new(ErrorKind::InvalidData, "qwer")); // TODO: Find a better error type.
    }
    if bytes[0] < 0b1000_0000 { // Equivalent to ((b[0] & 0b1000_0000) == 0)
        return Ok((1, X690Length::Definite(bytes[0] as usize)));
    }
    if bytes[0] == 0b1000_0000 {
        return Ok((1, X690Length::Indefinite));
    }
    // Otherwise, it is long definite form.
    let length_length = (bytes[0] & 0b0111_1111) as usize;
    if length_length > size_of::<usize>() {
        // Length too big.
        return Err(Error::new(ErrorKind::InvalidData, "tyui")); // TODO: Find a better error type.
    }
    if (bytes.len() - 1) < length_length {
        // Insufficient bytes to read the length.
        return Err(Error::new(ErrorKind::InvalidData, "rutdfg")); // TODO: Find a better error type. 
    }
    let bytes_read = 1 + length_length;
    let len: usize = match length_length {
        1 => bytes[1] as usize,
        2 => u16::from_be_bytes([ bytes[1], bytes[2] ]) as usize,
        3 => u32::from_be_bytes([ 0, bytes[1], bytes[2], bytes[3] ]) as usize,
        4 => u32::from_be_bytes([ bytes[1], bytes[2], bytes[3], bytes[4] ]) as usize,
        5 => u64::from_be_bytes([ 0, 0, 0, bytes[1], bytes[2], bytes[3], bytes[4], bytes[5] ]) as usize,
        6 => u64::from_be_bytes([ 0, 0, bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6] ]) as usize,
        7 => u64::from_be_bytes([ 0, bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7] ]) as usize,
        8 => u64::from_be_bytes([ bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8] ]) as usize,
        _ => 0, // This should never happen.
    };
    Ok((bytes_read, X690Length::Definite(len)))
}

// Get the CST of BER-encoded data.
pub fn ber_cst (bytes: ByteSlice) -> Result<(usize, X690Element)> {
    let tag_encoding_length: usize;
    let tag_class: TagClass;
    let constructed: bool;
    let tag_number: TagNumber;
    match ber_decode_tag(bytes) {
        Ok((len, tag)) => { // REVIEW: Is this idiomatic?
            tag_encoding_length = len;
            tag_class = tag.tag_class;
            constructed = tag.constructed;
            tag_number = tag.tag_number;
        },
        Err(e) => return Err(e),
    };
    let mut bytes_read: usize = tag_encoding_length;
    let value_length;
    match ber_decode_length(&bytes[bytes_read..]) {
        Ok((len_len, len)) => {
            bytes_read += len_len;
            value_length = len;
        },
        Err(e) => return Err(e),
    };
    match value_length {
        X690Length::Definite(len) => {
            if (bytes.len() - bytes_read) < len {
                // Truncated.
                return Err(Error::new(ErrorKind::InvalidData, "eoriyjhe")); // TODO: Find a better error type. 
            }
            if !constructed {
                let el = X690Element::new(
                    tag_class,
                    tag_number,
                    X690Encoding::IMPLICIT(Vec::from(&bytes[bytes_read..bytes_read+len])), // TODO: Indefinite too
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
                match ber_cst(&bytes[bytes_read..]) {
                    Ok((el_len, el)) => {
                        if el_len == 0 {
                            break;
                        }
                        bytes_read += el_len;
                        children.push(el);
                    },
                    Err(e) => return Err(Error::new(ErrorKind::InvalidData, "asdf")),
                };
            };
            let el = X690Element::new(
                tag_class,
                tag_number,
                X690Encoding::Constructed(children),
            );
            // bytes_read += len;
            Ok((bytes_read, el))
        },
        X690Length::Indefinite => {
            if !constructed {
                // Indefinite length must be constructed.
                return Err(Error::from(ErrorKind::InvalidData)); // TODO: Find a better error type. 
            }
            /* We don't know how many child elements this element must have, but
            it is a good guess (optimization) to assume that there is at least
            one. */
            let mut children: Vec<X690Element> = Vec::with_capacity(1);
            let mut value_bytes_read: usize = 0;
            while value_bytes_read < bytes.len() {
                match ber_cst(&bytes[bytes_read+value_bytes_read..]) {
                    Ok((el_len, el)) => {
                        if el_len == 0 {
                            break;
                        }
                        value_bytes_read += el_len;
                        if 
                            el.tag_class == TagClass::UNIVERSAL
                            && (el.tag_number == ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT)
                        {
                            // We do NOT append the EOC element. It is treated like it does not exist.
                            break;
                        }
                        children.push(el);
                    },
                    Err(e) => return Err(e),
                };
            };
            bytes_read += value_bytes_read;
            let el = X690Element::new(
                tag_class,
                tag_number,
                X690Encoding::Constructed(children),
            );
            Ok((bytes_read, el))
        },
    }
}

#[cfg(test)]
mod tests {

    use asn1::types::{
        ASN1Value,
        TagClass,
        TaggedASN1Value,
        ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
        ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE
    };

    use crate::{
        X690_TAG_CLASS_APPLICATION,
        X690_TAG_CLASS_CONTEXT,
        X690Element,
        write_x690_node,
        X690_TAG_CLASS_UNIVERSAL, ber_encode, ber_cst, X690Encoding,
    };

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
            | 0b0000_0000 // Primitive
            | 5,
            0x01, // Length = 5
            0x00, // FALSE
        ]));
    }

    #[test]
    fn test_constructed_encoding () {
        let asn1_data = X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            crate::X690Encoding::Constructed(vec![
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
                    crate::X690Encoding::IMPLICIT(vec![ 0xFF ]),
                ),
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
                    crate::X690Encoding::IMPLICIT(vec![ 0x01, 0x03 ]),
                ),
            ]),
        );
        let mut output = Vec::new();
        match write_x690_node(&mut output, &asn1_data) {
            Ok(bytes_written) => {
                assert_eq!(bytes_written, 9);
            },
            Err(e) => panic!("{}", e),
        }
        assert!(output.starts_with(&[
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE as u8,
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

    #[test]
    fn test_ber_encode_2 () {
        let asn1_data = ASN1Value::SequenceValue(vec![
            ASN1Value::BooleanValue(true),
            ASN1Value::IntegerValue(127),
        ]);
        let mut output = Vec::new();
        match ber_encode(&mut output, &asn1_data) {
            Ok(bytes_written) => {
                assert_eq!(bytes_written, 8);
            },
            Err(e) => panic!("{}", e),
        }
        assert!(output.starts_with(&[
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE as u8,
            0x06,
            0x01,
            0x01,
            0xFF,
            0x02,
            0x01,
            0x7F,
        ]));
    }

    #[test]
    fn test_ber_decode_definite_short () {
        let encoded_data: Vec<u8> = vec![
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE as u8,
            0x06,
            0x01,
            0x01,
            0xFF,
            0x02,
            0x01,
            0x7F,
        ];
        match ber_cst(encoded_data.as_slice()) {
            Ok((bytes_read, el)) => {
                assert_eq!(bytes_read, 8);
                assert_eq!(el.tag_class, TagClass::UNIVERSAL);
                assert_eq!(el.tag_number, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE);
                if let X690Encoding::Constructed(children) = el.value {
                    assert_eq!(children.len(), 2);
                    assert_eq!(children[0].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[1].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[0].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN);
                    assert_eq!(children[1].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER);
                } else {
                    panic!("Decoded non-constructed.");
                }
            },
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    fn test_ber_decode_indefinite () {
        let encoded_data: Vec<u8> = vec![
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE as u8,
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
        match ber_cst(encoded_data.as_slice()) {
            Ok((bytes_read, el)) => {
                assert_eq!(bytes_read, 10);
                assert_eq!(el.tag_class, TagClass::UNIVERSAL);
                assert_eq!(el.tag_number, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE);
                if let X690Encoding::Constructed(children) = el.value {
                    assert_eq!(children.len(), 2);
                    assert_eq!(children[0].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[1].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[0].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN);
                    assert_eq!(children[1].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER);
                } else {
                    panic!("Decoded non-constructed.");
                }
            },
            Err(e) => panic!("{}", e),
        };
    }
}

// TODO: Tests to verify that my error handling from nested matches actually works.