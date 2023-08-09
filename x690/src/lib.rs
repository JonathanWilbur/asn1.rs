use asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use asn1::types::{
    ASN1Value, ByteSlice, Bytes, CharacterString, EmbeddedPDV, ExternalEncoding,
    ExternalIdentification, GeneralizedTime, ObjectDescriptor,
    PresentationContextSwitchingTypeIdentification, Tag, TagClass, TagNumber, TaggedASN1Value,
    UTCTime, UniversalString, ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
    ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING, ASN1_UNIVERSAL_TAG_NUMBER_DATE,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME, ASN1_UNIVERSAL_TAG_NUMBER_DURATION,
    ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV, ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT,
    ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED, ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
    ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME, ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING, ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_INTEGER, ASN1_UNIVERSAL_TAG_NUMBER_NULL,
    ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER, ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI, ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_REAL, ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, ASN1_UNIVERSAL_TAG_NUMBER_SET,
    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF, ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME, ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY,
    ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING, ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING, ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING, BIT_STRING, BOOLEAN, DATE, DATE_TIME,
    DURATION_EQUIVALENT, EXTERNAL, INTEGER, MAX_IA5_STRING_CHAR_CODE, OBJECT_IDENTIFIER,
    OCTET_STRING, REAL, RELATIVE_OID, TIME, TIME_OF_DAY,
};
use asn1::ENUMERATED;
use std::borrow::{Borrow, Cow};
use std::io::{Error, ErrorKind, Result, Write};
use std::mem::size_of;
use std::sync::Arc;
use smallvec::{smallvec, SmallVec};

pub mod ber;
pub mod parsing;

pub use crate::ber::*;
pub use crate::parsing::*;

pub const X690_TAG_CLASS_UNIVERSAL: u8 = 0b0000_0000;
pub const X690_TAG_CLASS_APPLICATION: u8 = 0b0100_0000;
pub const X690_TAG_CLASS_CONTEXT: u8 = 0b1000_0000;
pub const X690_TAG_CLASS_PRIVATE: u8 = 0b1100_0000;
pub const X690_SPECIAL_REAL_PLUS_INFINITY: u8 = 0b0000_0000;
pub const X690_SPECIAL_REAL_MINUS_INFINITY: u8 = 0b0000_0001;
pub const X690_SPECIAL_REAL_NOT_A_NUMBER: u8 = 0b0000_0010;
pub const X690_SPECIAL_REAL_MINUS_ZERO: u8 = 0b0000_0011;
pub const X690_REAL_SPECIAL: u8 = 0b0100_0000;
pub const X690_REAL_BASE10: u8 = 0b0000_0000;
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
pub const X690_REAL_NR1: u8 = 1;
pub const X690_REAL_NR2: u8 = 2;
pub const X690_REAL_NR3: u8 = 3;
// const IEEE_754_DPFP_SIGN_MASK

#[derive(Clone, Debug, Hash, Copy, PartialEq, Eq)]
pub enum X690Length {
    Definite(usize),
    Indefinite,
}

#[derive(Clone, Debug, Hash)]
pub enum X690Encoding {
    // TODO: Convert to ByteSlice
    IMPLICIT(Bytes), // the value bytes
    // TODO: Review: Should this be a box or a smart pointer?
    EXPLICIT(Box<X690Element>),    // the inner TLV tuple
    Constructed(Vec<X690Element>), // an array of inner TLV tuples
    AlreadyEncoded(Bytes), // the already-encoded TLV
}

#[derive(Clone, Debug, Hash)]
pub struct X690Tag {
    pub tag_class: TagClass,
    pub constructed: bool,
    pub tag_number: TagNumber,
}

#[derive(Clone, Debug, Hash)]
pub struct X690Element {
    pub name: Option<String>, // Not Rc or Arc, because these are typically small enough where a clone() is not a big deal.
    pub tag_class: TagClass,
    pub tag_number: TagNumber,
    pub value: Arc<X690Encoding>,
    last_calculated_length: Option<usize>, // Maybe not pub
}

impl X690Encoding {
    pub fn len(&self) -> usize {
        let ret = match self {
            X690Encoding::IMPLICIT(value_bytes) => value_bytes.len(),
            X690Encoding::EXPLICIT(inner) => inner.len(),
            X690Encoding::Constructed(components) => {
                let mut sum: usize = 0;
                for c in components {
                    sum += c.len();
                }
                sum
            }
            X690Encoding::AlreadyEncoded(encoded_value) => {
                // TODO: Skip over tag and length bytes
                let tag_and_length_bytes = get_x690_tag_and_length_length(encoded_value);
                encoded_value.len() - tag_and_length_bytes
            }
        };
        ret
    }

    // TODO: is_empty()

    pub fn inner(&self) -> ASN1Result<X690Element> {
        match self {
            X690Encoding::EXPLICIT(e) => Ok((**e).clone()),
            X690Encoding::Constructed(children) => {
                if children.len() != 1 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                Ok(children[0].clone())
            }
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
}

impl X690Element {
    pub fn new(
        tag_class: TagClass,
        tag_number: TagNumber,
        value: Arc<X690Encoding>,
    ) -> X690Element {
        X690Element {
            name: None,
            tag_class,
            tag_number,
            value,
            last_calculated_length: None,
        }
    }

    pub fn len(&self) -> usize {
        match self.last_calculated_length {
            Some(l) => return l,
            None => {
                let tag_length: usize = get_written_x690_tag_length(self.tag_number);
                let value_length = self.value.len();
                let length_length: usize = get_written_x690_length_length(value_length);
                let ret = tag_length + length_length + value_length;
                // self.last_calculated_length = Some(ret);
                ret
            }
        }
    }

    pub fn is_constructed(&self) -> bool {
        if let X690Encoding::IMPLICIT(_) = *self.value {
            return false;
        }
        if let X690Encoding::Constructed(_) = *self.value {
            return true;
        }
        if let X690Encoding::EXPLICIT(_) = *self.value {
            return true;
        }
        if let X690Encoding::AlreadyEncoded(bytes) = self.value.borrow() {
            if bytes.len() == 0 {
                return false;
            }
            return (bytes[0] & 0b0010_0000) > 0;
        }
        false
    }

    pub fn inner(&self) -> ASN1Result<X690Element> {
        match self.value.borrow() {
            X690Encoding::EXPLICIT(e) => Ok((**e).clone()),
            X690Encoding::Constructed(children) => {
                if children.len() != 1 {
                    let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                    err.component_name = self.name.clone();
                    err.tag = Some(Tag::new(self.tag_class, self.tag_number));
                    err.constructed = Some(self.is_constructed());
                    err.length = Some(self.len());
                    return Err(err);
                }
                Ok(children[0].clone())
            }
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.component_name = self.name.clone();
                err.tag = Some(Tag::new(self.tag_class, self.tag_number));
                err.constructed = Some(self.is_constructed());
                err.length = Some(self.len());
                return Err(err);
            }
        }
    }

    // This is expensive because it might clone the content octets!
    // TODO: Make a Cow version of this?
    pub fn content_octets (&self) -> ASN1Result<Vec<u8>> {
        match self.value.borrow() {
            X690Encoding::IMPLICIT(content) => {
                return Ok(content.clone());
            },
            X690Encoding::Constructed(components) => {
                let mut ret: Vec<u8> = Vec::new();
                for component in components {
                    // We simply ignore this error. This should not fail at all,
                    // and if it does,
                    write_x690_node(&mut ret, &component)?;
                }
                Ok(ret)
            },
            X690Encoding::EXPLICIT(inner) => {
                let mut ret: Vec<u8> = Vec::with_capacity(inner.len());
                write_x690_node(&mut ret, &inner)?; // This actually should not fail.
                Ok(ret)
            },
            X690Encoding::AlreadyEncoded(bytes) => {
                let ignore_len = get_x690_tag_and_length_length(bytes);
                return Ok(Vec::from(&bytes[ignore_len..]));
            },
        }
    }

    pub fn to_asn1_error (&self, errcode: ASN1ErrorCode) -> ASN1Error {
        ASN1Error {
            error_code: errcode,
            component_name: self.name.clone(),
            tag: Some(Tag::new(self.tag_class, self.tag_number)),
            length: Some(self.len()),
            constructed: Some(self.is_constructed()),
            value_preview: None,
            bytes_read: None,
            values_read: None,
            io_error: None,
        }
    }
    // TODO: is_empty()
}

impl PartialEq for X690Element {
    fn eq(&self, other: &Self) -> bool {
        let Ok(thing1) = ber_decode_any(self) else {
            return false;
        };
        let Ok(thing2) = ber_decode_any(other) else {
            return false;
        };
        thing1 == thing2
    }
}

pub fn get_x690_tag_and_length_length(bytes: ByteSlice) -> usize {
    if bytes.len() == 0 {
        return 0;
    }
    let mut len: usize = 1;
    if (bytes[0] & 0b00011111) == 0b00011111 {
        // If it is a long tag...
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
    if (length_byte_0 & 0b1000_0000) == 0 {
        // Short definite form or indefinite form.
        return len;
    }
    (length_byte_0 & 0b0111_1111) as usize
}

// This is a struct with a single field for the sake of extensibility.
pub struct X690ConcreteSyntaxTree {
    pub root: X690Element,
}

fn base_128_len(num: u32) -> usize {
    // TODO: Could you get a performance improvement by using a match statement instead?
    if num < 128 {
        return 1;
    }
    let mut l = 0;
    let mut i = num;
    while i > 0 {
        l += 1;
        i >>= 7;
    }
    return l;
}

fn write_base_128<W>(output: &mut W, num: u32) -> Result<usize>
where
    W: Write,
{
    if num < 128 {
        return output.write(&[num as u8]);
    }

    let mut l = 0;
    let mut i = num;
    while i > 0 {
        l += 1;
        i >>= 7;
    }

    // A base-128-encoded u32 can only take up to five bytes.
    let mut encoded: SmallVec<[u8; 5]> = smallvec![];
    // let mut encoded: Vec<u8> = Vec::with_capacity(5);
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
pub fn get_written_x690_tag_length(tagnum: TagNumber) -> usize {
    if tagnum < 31 {
        // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return 1;
    }
    base_128_len(tagnum.into()) + 1
}

// Just gets the length of the length bytes for an element
pub fn get_written_x690_length_length(len: usize) -> usize {
    if len <= 127 {
        // See ITU Rec. X.690 (2021), Section 8.1.3.3, "NOTE"
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

pub fn x690_write_tag<W>(
    output: &mut W,
    class: TagClass,
    constructed: bool,
    tagnum: TagNumber,
) -> Result<usize>
where
    W: Write,
{
    let k: u8 = match class {
        TagClass::UNIVERSAL => X690_TAG_CLASS_UNIVERSAL,
        TagClass::APPLICATION => X690_TAG_CLASS_APPLICATION,
        TagClass::CONTEXT => X690_TAG_CLASS_CONTEXT,
        TagClass::PRIVATE => X690_TAG_CLASS_PRIVATE,
    };
    if tagnum < 31 {
        // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return output.write(&[k
            | if constructed {
                0b0010_0000
            } else {
                0b0000_0000
            }
            | tagnum as u8]);
    } else {
        let first_byte_result = output.write(&[k
            | if constructed {
                0b0010_0000
            } else {
                0b0000_0000
            }
            | 0b0001_1111u8]);
        if let Err(e) = first_byte_result {
            return Err(e);
        }
        return write_base_128(output, tagnum.into());
    }
}

pub fn x690_write_length<W>(output: &mut W, length: usize) -> Result<usize>
where
    W: Write,
{
    if length <= 127 {
        // See ITU Rec. X.690 (2021), Section 8.1.3.3, "NOTE"
        return output.write(&[length as u8]);
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
        output.write(&[0b1000_0000 | octets_needed])?;
        output.write(length.to_be_bytes().as_slice())
    }
}

pub fn x690_write_boolean_value<W>(output: &mut W, value: &BOOLEAN) -> Result<usize>
where
    W: Write,
{
    if *value {
        return output.write(&[0xFF]);
    } else {
        return output.write(&[0x00]);
    }
}

pub fn x690_write_integer_value<W>(output: &mut W, value: &INTEGER) -> Result<usize>
where
    W: Write,
{
    output.write(value)
}

pub fn x690_write_i64_value<W>(output: &mut W, value: i64) -> Result<usize>
where
    W: Write,
{
    let bytes: [u8; 8] = value.to_be_bytes();
    let padding_byte: u8 = if value >= 0 { 0x00 } else { 0xFF };
    let mut number_of_padding_bytes: usize = 0;
    for byte in bytes {
        if byte == padding_byte {
            number_of_padding_bytes += 1;
        } else {
            break;
        }
    }
    let mut bytes_written: usize = 0;
    if (number_of_padding_bytes == size_of::<i64>())
        || (value >= 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) > 0))
        || (value < 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) == 0)) {
        bytes_written += output.write(&[padding_byte])?;
    }
    bytes_written += output.write(&(bytes[number_of_padding_bytes..size_of::<i64>()]))?;
    Ok(bytes_written)
}

pub fn x690_write_enum_value<W>(output: &mut W, value: &ENUMERATED) -> Result<usize>
where
    W: Write,
{
    x690_write_i64_value(output, *value)
}

pub fn x690_write_bit_string_value<W>(output: &mut W, value: &BIT_STRING) -> Result<usize>
where
    W: Write,
{
    output.write(&[value.trailing_bits % 8])?;
    let bytes_written = output.write(&value.bytes)?;
    Ok(bytes_written + 1)
}

pub fn x690_write_octet_string_value<W>(output: &mut W, value: &OCTET_STRING) -> Result<usize>
where
    W: Write,
{
    output.write(value)
}

pub fn x690_write_object_identifier_value<W>(
    output: &mut W,
    value: &OBJECT_IDENTIFIER,
) -> Result<usize>
where
    W: Write,
{
    // TODO: Should object identifier be a type that forces a first and second arc?
    if value.0.len() < 2 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let node0 = value.0[0];
    let node1 = value.0[1];
    let byte0 = (node0 * 40) + node1;
    let mut bytes_written = 0;
    bytes_written += output.write(&[byte0 as u8])?;
    for arc in value.0[2..].iter() {
        bytes_written += write_base_128(output, *arc)?;
    }
    Ok(bytes_written)
}

pub fn x690_write_object_descriptor_value<W>(
    output: &mut W,
    value: &ObjectDescriptor,
) -> Result<usize>
where
    W: Write,
{
    // TODO: Validate
    output.write(value.as_bytes())
}

// NOTE: This has to be encoded in a strange way that is detailed in ITU Recommendation X.690, Section 8.18.
pub fn x690_write_external_value<W>(output: &mut W, value: &EXTERNAL) -> Result<usize>
where
    W: Write,
{
    let mut inner_elements: Vec<X690Element> = Vec::new();
    match &value.identification {
        ExternalIdentification::syntax(oid) => {
            let mut bytes: Bytes = Vec::new();
            x690_write_object_identifier_value(&mut bytes, &oid)?;
            let element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                Arc::new(X690Encoding::IMPLICIT(bytes)),
            );
            inner_elements.push(element);
        }
        ExternalIdentification::presentation_context_id(pci) => {
            let mut bytes: Bytes = Vec::new();
            x690_write_integer_value(&mut bytes, pci)?;
            let element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
                Arc::new(X690Encoding::IMPLICIT(bytes)),
            );
            inner_elements.push(element);
        }
        ExternalIdentification::context_negotiation(cn) => {
            let mut direct_ref_bytes: Bytes = Vec::new();
            x690_write_object_identifier_value(&mut direct_ref_bytes, &cn.transfer_syntax)?;
            let direct_ref_element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                Arc::new(X690Encoding::IMPLICIT(direct_ref_bytes)),
            );
            inner_elements.push(direct_ref_element);
            let mut indirect_ref_bytes: Bytes = Vec::new();
            x690_write_integer_value(&mut indirect_ref_bytes, &cn.presentation_context_id)?;
            let indirect_ref_element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
                Arc::new(X690Encoding::IMPLICIT(indirect_ref_bytes)),
            );
            inner_elements.push(indirect_ref_element);
        }
    };
    match &value.data_value_descriptor {
        Some(dvd) => {
            let mut bytes: Bytes = Vec::new();
            x690_write_object_descriptor_value(&mut bytes, &dvd)?;
            let element = X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
                Arc::new(X690Encoding::IMPLICIT(bytes)),
            );
            inner_elements.push(element);
        }
        None => (),
    };
    let mut data_value_bytes: Bytes = Vec::new();
    match &value.data_value {
        ExternalEncoding::single_ASN1_type(t) => ber_encode(&mut data_value_bytes, t)?,
        ExternalEncoding::octet_aligned(o) => x690_write_octet_string_value(&mut data_value_bytes, o)?,
        ExternalEncoding::arbitrary(b) => x690_write_bit_string_value(&mut data_value_bytes, b)?,
    };
    let data_value_element = X690Element::new(
        TagClass::CONTEXT,
        1,
        Arc::new(X690Encoding::IMPLICIT(data_value_bytes)),
    );
    inner_elements.push(data_value_element);
    let mut bytes_written: usize = 0;
    for component in inner_elements {
        bytes_written += write_x690_node(output, &component)?;
    }
    Ok(bytes_written)
}

pub fn x690_write_real_value<W>(output: &mut W, value: &REAL) -> Result<usize>
where
    W: Write,
{
    // This may seem like a floating precision problem, but this is how the
    // `num` crate does it:
    // https://github.com/rust-num/num-traits/blob/5397a1c27124af874e42d3d185f78d8ce01ecf69/src/identities.rs#L61
    let is_zero = *value == 0.0;
    // If the real value is the value plus zero, there shall be no contents octets in the encoding.
    if is_zero {
        return Ok(0);
    }
    // If the real value is the value minus zero, then it shall be encoded as specified in 8.5.9.
    if is_zero && value.is_sign_negative() {
        return output.write(&[X690_SPECIAL_REAL_MINUS_ZERO]);
    }

    if value.is_nan() {
        return output.write(&[X690_SPECIAL_REAL_NOT_A_NUMBER]);
    }

    if value.is_infinite() {
        if value.is_sign_negative() {
            return output.write(&[X690_SPECIAL_REAL_MINUS_INFINITY]);
        } else {
            return output.write(&[X690_REAL_SPECIAL | X690_SPECIAL_REAL_PLUS_INFINITY]);
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
    let mut mantissa: u64 = u64::from_be_bytes([
        0x00,
        0x08 & bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ]);
    let mut exponent: u16 = u16::from_be_bytes([bytes[0] & 0x7F, bytes[1] & 0b1110_0000]) >> 5;
    while (mantissa > 0) && !((mantissa % 2) > 0) {
        mantissa = mantissa >> 1;
        exponent += 1;
    }
    let e_bytes = exponent.to_be_bytes();
    let mut bytes_written: usize = 0;
    if exponent > u8::MAX as u16 {
        let byte0: u8 = X690_REAL_BINARY
            | sign_bit
            | base_bits
            | scaling_factor
            | X690_REAL_EXPONENT_FORMAT_2_OCTET;
        bytes_written += output.write(&[byte0, e_bytes[0], e_bytes[1]])?;
    } else {
        let byte0: u8 = X690_REAL_BINARY
            | sign_bit
            | base_bits
            | scaling_factor
            | X690_REAL_EXPONENT_FORMAT_1_OCTET;
        bytes_written += output.write(&[byte0, e_bytes[1]])?;
    };
    return match x690_write_i64_value(output, mantissa as i64) {
        Err(e) => return Err(e),
        Ok(wrote) => Ok(wrote + bytes_written),
    };
}

pub fn x690_context_switching_identification_to_cst(
    id: &PresentationContextSwitchingTypeIdentification,
) -> Result<X690Element> {
    match id {
        PresentationContextSwitchingTypeIdentification::syntaxes(syntaxes) => {
            let mut abstract_value_bytes: Bytes = Vec::new();
            let mut transfer_value_bytes: Bytes = Vec::new();
            x690_write_object_identifier_value(
                &mut abstract_value_bytes,
                &syntaxes.r#abstract,
            )?;
            x690_write_object_identifier_value(&mut transfer_value_bytes, &syntaxes.transfer)?;
            let mut syntaxes_elements: Vec<X690Element> = Vec::new();
            syntaxes_elements.push(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::IMPLICIT(abstract_value_bytes)),
            ));
            syntaxes_elements.push(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::IMPLICIT(transfer_value_bytes)),
            ));
            let element = X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::Constructed(syntaxes_elements)),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(element))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::syntax(oid) => {
            // We assume that, on average, each OID arc is encoded on two bytes.
            let mut bytes: Bytes = Vec::with_capacity(oid.0.len() << 1);
            x690_write_object_identifier_value(&mut bytes, &oid)?;
            let element = X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::IMPLICIT(bytes)),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(element))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::presentation_context_id(pci) => {
            let mut bytes: Bytes = Vec::with_capacity(pci.len());
            x690_write_integer_value(&mut bytes, pci)?;
            let element = X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::IMPLICIT(bytes)),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(element))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::context_negotiation(cn) => {
            let mut pci_bytes: Bytes = Vec::new();
            match x690_write_integer_value(&mut pci_bytes, &cn.presentation_context_id) {
                Err(e) => return Err(e),
                _ => (),
            };
            let pci_element = X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::IMPLICIT(pci_bytes)),
            );
            let mut transfer_syntax_bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(
                &mut transfer_syntax_bytes,
                &cn.transfer_syntax,
            ) {
                Err(e) => return Err(e),
                _ => (),
            };
            let transfer_syntax_element = X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::IMPLICIT(transfer_syntax_bytes)),
            );
            let cn_elements: Vec<X690Element> = vec![pci_element, transfer_syntax_element];
            let element = X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::Constructed(cn_elements)),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(element))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::transfer_syntax(ts) => {
            let mut bytes: Bytes = Vec::new();
            match x690_write_object_identifier_value(&mut bytes, &ts) {
                Err(e) => return Err(e),
                _ => (),
            };
            let element = X690Element::new(
                TagClass::CONTEXT,
                4,
                Arc::new(X690Encoding::IMPLICIT(bytes)),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(element))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::fixed => {
            let element = X690Element::new(
                TagClass::CONTEXT,
                5,
                Arc::new(X690Encoding::IMPLICIT(Vec::new())),
            );
            return Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(element))),
            ));
        }
    }
}

pub fn x690_write_embedded_pdv_value<W>(output: &mut W, value: &EmbeddedPDV) -> Result<usize>
where
    W: Write,
{
    let id = x690_context_switching_identification_to_cst(&value.identification)?;
    let mut data_value_bytes: Bytes = Vec::new();
    x690_write_octet_string_value(&mut data_value_bytes, &value.data_value)?;
    let data_value_element = X690Element::new(
        TagClass::CONTEXT,
        1,
        Arc::new(X690Encoding::IMPLICIT(data_value_bytes)),
    );
    let inner_elements: Vec<X690Element> = vec![id, data_value_element];
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

pub fn x690_write_utf8_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

pub fn x690_write_relative_oid_value<W>(output: &mut W, value: &RELATIVE_OID) -> Result<usize>
where
    W: Write,
{
    let mut bytes_written: usize = 0;
    for arc in value.0.iter() {
        bytes_written +=  write_base_128(output, *arc)?;
    }
    Ok(bytes_written)
}

pub fn x690_write_time_value<W>(output: &mut W, value: &TIME) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

pub fn x690_write_utc_time_value<W>(output: &mut W, value: &UTCTime) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_generalized_time_value<W>(
    output: &mut W,
    value: &GeneralizedTime,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_universal_string_value<W>(
    output: &mut W,
    value: &UniversalString,
) -> Result<usize>
where
    W: Write,
{
    let bytes: Vec<u8> = value
        .chars()
        .map(|c| c as u32)
        .flat_map(|c| {
            [
                ((c & 0xFF000000) >> 24) as u8,
                ((c & 0x00FF0000) >> 16) as u8,
                ((c & 0x0000FF00) >> 8) as u8,
                (c & 0x000000FF) as u8,
            ]
        })
        .collect();
    output.write(bytes.as_slice())
}

// This is almost the same for EmbeddedPDV.
pub fn x690_write_character_string_value<W>(
    output: &mut W,
    value: &CharacterString,
) -> Result<usize>
where
    W: Write,
{
    let id = x690_context_switching_identification_to_cst(&value.identification)?;
    let mut data_value_bytes: Bytes = Vec::new();
    x690_write_octet_string_value(&mut data_value_bytes, &value.string_value)?;
    let data_value_element = X690Element::new(
        TagClass::CONTEXT,
        1,
        Arc::new(X690Encoding::IMPLICIT(data_value_bytes)),
    );
    let inner_elements: Vec<X690Element> = vec![id, data_value_element];
    let mut bytes_written: usize = 0;
    for component in inner_elements {
        bytes_written += write_x690_node(output, &component)?;
    }
    Ok(bytes_written)
}

pub fn x690_write_bmp_string_value<W>(output: &mut W, value: &UniversalString) -> Result<usize>
where
    W: Write,
{
    let bytes: Vec<u8> = value
        .chars()
        .map(|c| c as u32)
        .flat_map(|c| {
            [
                // ((c & 0xFF000000) >> 24) as u8,
                // ((c & 0x00FF0000) >> 16) as u8,
                ((c & 0x0000FF00) >> 8) as u8,
                (c & 0x000000FF) as u8,
            ]
        })
        .collect();
    output.write(bytes.as_slice())
}

// This does not do any validation.
#[inline]
pub fn x690_write_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

pub fn x690_write_date_value<W>(output: &mut W, value: &DATE) -> Result<usize>
where
    W: Write,
{
    if value.month > 12 || value.month == 0 || value.day > 31 || value.day == 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_time_of_day_value<W>(output: &mut W, value: &TIME_OF_DAY) -> Result<usize>
where
    W: Write,
{
    if value.hour > 23 || value.minute > 59 || value.second > 59 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_date_time_value<W>(output: &mut W, value: &DATE_TIME) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_duration_value<W>(output: &mut W, value: &DURATION_EQUIVALENT) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

// TODO: Add check for infinite recursion
pub fn create_x690_cst_node(value: &ASN1Value) -> Result<X690Element> {
    let mut tag_class: TagClass = TagClass::UNIVERSAL;
    let mut tag_number: TagNumber = 0;
    let encoded_value: Arc<X690Encoding>;
    match value {
        ASN1Value::UnknownBytes(v) => {
            // TODO: Review
            encoded_value = Arc::new(X690Encoding::IMPLICIT((*v.clone()).clone()));
        }
        ASN1Value::TaggedValue(v) => {
            tag_class = v.tag.tag_class;
            tag_number = v.tag.tag_number;
            if v.explicit {
                let cst = create_x690_cst(&v.value)?;
                encoded_value = Arc::new(X690Encoding::EXPLICIT(Box::new(cst.root)));
            } else {
                let cst = create_x690_cst(&v.value)?;
                // TODO: arc_unwrap_or_clone, when that is non-experimental.
                encoded_value = cst.root.value.clone();
            }
        }
        ASN1Value::BooleanValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(vec![ if *v { 0xFF } else { 0x00 }]));
        }
        // TODO: Handle a BIGINT type
        ASN1Value::IntegerValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_INTEGER;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_integer_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::BitStringValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.bytes.len() + 1);
            x690_write_bit_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::OctetStringValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(v.clone()));
        }
        ASN1Value::NullValue => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_NULL;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(vec![]));
        }
        ASN1Value::ObjectIdentifierValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER;
            // We assume the average arc gets encoded on two bytes.
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.0.len() << 1);
            x690_write_object_identifier_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::ExternalValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL;
            let mut value_bytes: Vec<u8> = Vec::new();
            x690_write_external_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::RealValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_REAL;
            let mut value_bytes: Vec<u8> = Vec::new();
            x690_write_real_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::EnumeratedValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(8);
            x690_write_enum_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::EmbeddedPDVValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV;
            let mut value_bytes: Vec<u8> = Vec::new();
            x690_write_embedded_pdv_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::UTF8String(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_utf8_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::RelativeOIDValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.0.len() << 1);
            x690_write_relative_oid_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::TimeValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_TIME;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_time_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::SequenceValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = Arc::new(X690Encoding::Constructed(inner_values));
        }
        ASN1Value::SequenceOfValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = Arc::new(X690Encoding::Constructed(inner_values));
        }
        ASN1Value::SetValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SET;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = Arc::new(X690Encoding::Constructed(inner_values));
        }
        ASN1Value::SetOfValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_SET_OF;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = Arc::new(X690Encoding::Constructed(inner_values));
        }
        ASN1Value::UTCTime(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(17); // This is the max length of a UTCTime.
            x690_write_utc_time_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::GeneralizedTime(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(32); // This should cover most values.
            x690_write_generalized_time_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::UniversalString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len() << 2);
            x690_write_universal_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::UnrestrictedCharacterStringValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING;
            let mut value_bytes: Vec<u8> = Vec::new();
            x690_write_character_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::BMPString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len() << 1);
            x690_write_bmp_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::InstanceOfValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL;
            let type_id = ASN1Value::ObjectIdentifierValue(v.type_id.clone());
            let val = TaggedASN1Value {
                tag: Tag::new(TagClass::CONTEXT, 0),
                explicit: true,
                value: v.value.clone(),
            };
            let value = ASN1Value::TaggedValue(val);
            let type_id_element = match create_x690_cst(&type_id) {
                Err(e) => return Err(e),
                Ok(cst) => cst.root,
            };
            let value_element = match create_x690_cst(&value) {
                Err(e) => return Err(e),
                Ok(cst) => cst.root,
            };
            encoded_value = Arc::new(X690Encoding::Constructed(vec![
                type_id_element,
                value_element,
            ]));
        }
        ASN1Value::IRIValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::RelativeIRIValue(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::GeneralString(v) => {
            if !v.is_ascii() {
                // GeneralString must be below or at 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::IA5String(v) => {
            for c in v.chars() {
                if c > MAX_IA5_STRING_CHAR_CODE as char {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::GraphicString(v) => {
            for c in v.chars() {
                if !c.is_ascii_graphic() && (c != ' ') {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::ISO646String(v) => {
            if !v.is_ascii() {
                // VisibleString / ISO646String must be below 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            for c in v.chars() {
                if c == '\x7F' {
                    // DELETE not allowed.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::VisibleString(v) => {
            if !v.is_ascii() {
                // VisibleString / ISO646String must be below 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            for c in v.chars() {
                if c == '\x7F' {
                    // DELETE not allowed.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::NumericString(v) => {
            for c in v.chars() {
                if !c.is_ascii_digit() && c != ' ' {
                    // SPACE is allowed in NumericString.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::PrintableString(v) => {
            for c in v.chars() {
                if c.is_ascii_alphanumeric()
                    || (c >= '\x27' && c < '0' && c != '*') // '()+,-./ BUT NOT *
                    || c == ' '
                    || c == ':'
                    || c == '='
                    || c == '?'
                {
                    continue;
                }
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::TeletexString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(v.clone()));
        }
        ASN1Value::T61String(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(v.clone()));
        }
        ASN1Value::VideotexString(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(v.clone()));
        }
        ASN1Value::DATE(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_DATE;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(10);
            x690_write_date_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::TIME_OF_DAY(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(8);
            x690_write_time_of_day_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::DATE_TIME(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(19); // 1951-10-14T15:30:00
            x690_write_date_time_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::DURATION(v) => {
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_DURATION;
            // There is no guaranteed size, but 16 is a reasonable pre-allocation.
            let mut value_bytes: Vec<u8> = Vec::with_capacity(16);
            x690_write_duration_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::ObjectDescriptor(v) => {
            for c in v.chars() {
                if !c.is_ascii_graphic() && (c != ' ') {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING;
            let mut value_bytes: Vec<u8> = Vec::with_capacity(v.len());
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = Arc::new(X690Encoding::IMPLICIT(value_bytes));
        }
        ASN1Value::ChoiceValue(v) => {
            return create_x690_cst_node(v);
        }
    };

    Ok(X690Element::new(tag_class, tag_number, encoded_value))
}

// TODO: Use this in ::new()
pub fn create_x690_cst<'a>(value: &ASN1Value) -> Result<X690ConcreteSyntaxTree> {
    match create_x690_cst_node(value) {
        Ok(root_node) => Ok(X690ConcreteSyntaxTree { root: root_node }),
        Err(e) => Err(e),
    }
}

// TODO:
// pub fn get_x690_cst <'a> (bytes: Bytes) -> Result<X690ConcreteSyntaxTree<'a>> {

// }

fn write_x690_encoding<W>(output: &mut W, encoding: &X690Encoding) -> Result<usize>
where
    W: Write,
{
    match encoding {
        X690Encoding::IMPLICIT(value_bytes) => output.write(value_bytes.as_slice()),
        X690Encoding::EXPLICIT(inner) => write_x690_node(output, inner),
        X690Encoding::Constructed(components) => {
            let mut sum: usize = 0;
            for c in components {
                sum += write_x690_node(output, c)?;
            }
            Ok(sum)
        }
        X690Encoding::AlreadyEncoded(_) => Ok(0), // This should never happen.
    }
}

pub fn write_x690_node<W>(output: &mut W, node: &X690Element) -> Result<usize>
where
    W: Write,
{
    if let X690Encoding::AlreadyEncoded(already_encoded_bytes) = node.value.borrow() {
        return output.write(already_encoded_bytes);
    }
    let tag_class = node.tag_class;
    let tag_number = node.tag_number;
    let mut constructed: bool = false;
    if let X690Encoding::Constructed(_) = *node.value {
        constructed = true;
    } else if let X690Encoding::EXPLICIT(_) = *node.value {
        constructed = true;
    }
    let len = node.value.len();
    let mut bytes_written: usize = 0;
    bytes_written += x690_write_tag(output, tag_class, constructed, tag_number)?;
    bytes_written += x690_write_length(output, len)?;
    bytes_written += write_x690_encoding(output, &node.value)?;
    Ok(bytes_written)
}

pub fn ber_encode<W>(output: &mut W, value: &ASN1Value) -> Result<usize>
where
    W: Write,
{
    let cst = create_x690_cst(value)?;
    write_x690_node(output, &cst.root)
}

pub fn ber_decode_tag(bytes: ByteSlice) -> ASN1Result<(usize, X690Tag)> {
    if bytes.len() == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
    }
    let mut bytes_read = 1;
    let tag_class = match (bytes[0] & 0b1100_0000) >> 6 {
        0 => TagClass::UNIVERSAL,
        1 => TagClass::APPLICATION,
        2 => TagClass::CONTEXT,
        3 => TagClass::PRIVATE,
        _ => panic!("Impossible tag class"),
    };
    let constructed = (bytes[0] & 0b0010_0000) > 0;
    let mut tag_number: TagNumber = 0;

    if (bytes[0] & 0b00011111) == 0b00011111 {
        // If it is a long tag...
        for byte in bytes[1..].into_iter() {
            let final_byte: bool = ((*byte) & 0b1000_0000) == 0;
            if (tag_number > 0) && !final_byte {
                // tag_number > 0 means we've already processed one byte.
                // Tag encoded on more than 14 bits / two bytes.
                return Err(ASN1Error::new(ASN1ErrorCode::tag_too_big));
            }
            let seven_bits = ((*byte) & 0b0111_1111) as u16;
            if !final_byte && (seven_bits == 0) {
                // You cannot encode a long tag with padding bytes.
                return Err(ASN1Error::new(ASN1ErrorCode::padding_in_tag_number));
            }
            tag_number <<= 7;
            tag_number += seven_bits;
            bytes_read += 1;
            if final_byte {
                break;
            }
        }
        if tag_number <= 31 {
            // TODO: Review this number / comparison.
            // This could have been encoded in short form.
            return Err(ASN1Error::new(
                ASN1ErrorCode::tag_number_could_have_used_short_form,
            ));
        }
    } else {
        tag_number = (bytes[0] & 0b00011111) as TagNumber;
    }

    let tag = X690Tag {
        tag_class,
        constructed,
        tag_number,
    };
    Ok((bytes_read, tag))
}

pub fn ber_decode_length(bytes: ByteSlice) -> ASN1Result<(usize, X690Length)> {
    if bytes.len() == 0 {
        // Truncated.
        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
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
        return Err(ASN1Error::new(ASN1ErrorCode::truncated));
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

// Get the CST of BER-encoded data.
pub fn ber_cst(bytes: ByteSlice) -> ASN1Result<(usize, X690Element)> {
    let (len, tag) = ber_decode_tag(bytes)?;
    let mut bytes_read: usize = len;
    let value_length;
    match ber_decode_length(&bytes[bytes_read..]) {
        Ok((len_len, len)) => {
            bytes_read += len_len;
            value_length = len;
        }
        Err(e) => return Err(e),
    };
    match value_length {
        X690Length::Definite(len) => {
            if (bytes.len() - bytes_read) < len {
                let mut err = ASN1Error::new(ASN1ErrorCode::truncated);
                err.tag = Some(Tag::new(tag.tag_class, tag.tag_number));
                err.length = Some(len);
                return Err(err);
            }
            if !tag.constructed {
                let el = X690Element::new(
                    tag.tag_class,
                    tag.tag_number,
                    Arc::new(X690Encoding::IMPLICIT(Vec::from(
                        &bytes[bytes_read..bytes_read + len],
                    ))),
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
                let (el_len, el) = ber_cst(&bytes[bytes_read..])?;
                if el_len == 0 {
                    break;
                }
                bytes_read += el_len;
                children.push(el);
            }
            let el = X690Element::new(
                tag.tag_class,
                tag.tag_number,
                Arc::new(X690Encoding::Constructed(children)),
            );
            Ok((bytes_read, el))
        }
        X690Length::Indefinite => {
            if !tag.constructed {
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
                match ber_cst(&bytes[bytes_read + value_bytes_read..]) {
                    Ok((el_len, el)) => {
                        if el_len == 0 {
                            break;
                        }
                        value_bytes_read += el_len;
                        if el.tag_class == TagClass::UNIVERSAL
                            && (el.tag_number == ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT)
                        {
                            // We do NOT append the EOC element. It is treated like it does not exist.
                            break;
                        }
                        children.push(el);
                    }
                    Err(e) => return Err(e),
                };
            }
            bytes_read += value_bytes_read;
            let el = X690Element::new(
                tag.tag_class,
                tag.tag_number,
                Arc::new(X690Encoding::Constructed(children)),
            );
            Ok((bytes_read, el))
        }
    }
}

// TODO: This needs testing.
pub fn deconstruct<'a>(el: &'a X690Element) -> ASN1Result<Cow<'a, [u8]>> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => Ok(Cow::Borrowed(bytes)),
        X690Encoding::EXPLICIT(inner) => return deconstruct(&inner),
        X690Encoding::AlreadyEncoded(bytes) => match ber_cst(&bytes) {
            Ok((_, cst)) => {
                return Ok(Cow::Owned(deconstruct(&cst)?.into_owned()));
            }
            Err(e) => return Err(e),
        },
        X690Encoding::Constructed(children) => {
            let mut deconstructed_value: Bytes = Vec::new();
            for child in children {
                /* Just to be clear, this is 100% intentional. In ITU X.690, it says that the substrings of a string
                type are to have OCTET STRING tags and it even has examples where it confirms this visually. */
                if child.tag_class != TagClass::UNIVERSAL
                    || child.tag_number != ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING
                {
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::string_constructed_with_invalid_tagging);
                    err.component_name = el.name.clone();
                    err.tag = Some(Tag::new(el.tag_class, el.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(true);
                    return Err(err);
                }
                match deconstruct(&child) {
                    Ok(deconstructed_child) => {
                        deconstructed_value.extend(deconstructed_child.as_ref());
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(Cow::Owned(deconstructed_value))
        }
    }
}

pub trait RelateTLV {
    fn relatve_tlv (&mut self, el: &X690Element);
}

impl RelateTLV for ASN1Error {
    fn relatve_tlv (&mut self, el: &X690Element) {
        self.tag = Some(Tag::new(el.tag_class, el.tag_number));
        self.component_name = el.name.clone();
        self.constructed = Some(el.is_constructed());
        self.length = Some(el.len());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_x690_write_boolean_value() {
        let mut output: Vec<u8> = Vec::new();
        crate::x690_write_boolean_value(&mut output, &true).unwrap();
        crate::x690_write_boolean_value(&mut output, &false).unwrap();
        assert_eq!(output.len(), 2);
        assert!(output.starts_with(&[0xFF, 0x00]));
    }

    #[test]
    fn test_x690_write_integer_value() {
        let mut output: Vec<u8> = Vec::new();
        let mut i = 0;
        for value in -128i8..127i8 {
            crate::x690_write_enum_value(&mut output, &i64::from(value)).unwrap();
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
    fn test_x690_write_octet_string_value() {
        let mut output: Vec<u8> = Vec::new();
        let bytes: Vec<u8> = vec![1, 3, 5, 7, 9];
        crate::x690_write_octet_string_value(&mut output, &bytes).unwrap();
        assert_eq!(output.len(), 5);
        assert!(output.starts_with(&[1, 3, 5, 7, 9]));
    }

    #[test]
    fn test_x690_write_object_identifier_value() {
        let mut output: Vec<u8> = Vec::new();
        let oid = asn1::types::OBJECT_IDENTIFIER(vec![2, 5, 4, 3]);
        crate::x690_write_object_identifier_value(&mut output, &oid).unwrap();
        assert_eq!(output.len(), 3);
        assert!(output.starts_with(&[0x55, 0x04, 0x03]));
    }

    #[test]
    fn test_x690_write_object_descriptor_value() {
        let mut output: Vec<u8> = Vec::new();
        let value = String::from("commonName");
        crate::x690_write_object_descriptor_value(&mut output, &value).unwrap();
        assert_eq!(output.len(), value.len());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            String::from("commonName")
        );
    }

    #[test]
    fn test_x690_write_real_value() {
        let mut output: Vec<u8> = Vec::new();
        let value = 1.2345;
        crate::x690_write_real_value(&mut output, &value).unwrap();
    }

    #[test]
    fn test_ber_encode() {
        let mut output: Vec<u8> = Vec::new();
        let val = TaggedASN1Value {
            tag: Tag::new(TagClass::APPLICATION, 5),
            explicit: true,
            value: Arc::new(ASN1Value::BooleanValue(true)),
        };
        let value: ASN1Value = ASN1Value::TaggedValue(val);
        let result = crate::ber_encode(&mut output, &value).unwrap();
        assert_eq!(result, output.len());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_ber_encode_deep_tagging_1() {
        let inner_val = TaggedASN1Value {
            tag: Tag::new(TagClass::CONTEXT, 7),
            explicit: true,
            value: Arc::from(ASN1Value::BooleanValue(false)),
        };
        let outer_val = TaggedASN1Value {
            tag: Tag::new(TagClass::APPLICATION, 5),
            explicit: true,
            value: Arc::from(ASN1Value::TaggedValue(inner_val)),
        };
        let mut output: Vec<u8> = Vec::new();
        let value: ASN1Value = ASN1Value::TaggedValue(outer_val);
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
    fn test_ber_encode_deep_tagging_2() {
        let mut output: Vec<u8> = Vec::new();
        let inner_val = TaggedASN1Value {
            tag: Tag::new(TagClass::CONTEXT, 7),
            explicit: false,
            value: Arc::new(ASN1Value::BooleanValue(false)),
        };
        let outer_val = TaggedASN1Value {
            tag: Tag::new(TagClass::APPLICATION, 5),
            explicit: false,
            value: Arc::new(ASN1Value::TaggedValue(inner_val)),
        };
        let value: ASN1Value = ASN1Value::TaggedValue(outer_val);
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
    fn test_constructed_encoding() {
        let asn1_data = X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(crate::X690Encoding::Constructed(vec![
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
                    Arc::new(crate::X690Encoding::IMPLICIT(vec![0xFF])),
                ),
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
                    Arc::new(crate::X690Encoding::IMPLICIT(vec![0x01, 0x03])),
                ),
            ])),
        );
        let mut output = Vec::new();
        match write_x690_node(&mut output, &asn1_data) {
            Ok(bytes_written) => {
                assert_eq!(bytes_written, 9);
            }
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
    fn test_ber_encode_2() {
        let asn1_data = ASN1Value::SequenceValue(vec![
            ASN1Value::BooleanValue(true),
            ASN1Value::IntegerValue(vec![127]),
        ]);
        let mut output = Vec::new();
        match ber_encode(&mut output, &asn1_data) {
            Ok(bytes_written) => {
                assert_eq!(bytes_written, 8);
            }
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
    fn test_ber_decode_definite_short() {
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
                if let X690Encoding::Constructed(children) = el.value.borrow() {
                    assert_eq!(children.len(), 2);
                    assert_eq!(children[0].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[1].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[0].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN);
                    assert_eq!(children[1].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER);
                } else {
                    panic!("Decoded non-constructed.");
                }
            }
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    fn test_ber_decode_indefinite() {
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
                if let X690Encoding::Constructed(children) = el.value.borrow() {
                    assert_eq!(children.len(), 2);
                    assert_eq!(children[0].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[1].tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[0].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN);
                    assert_eq!(children[1].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER);
                } else {
                    panic!("Decoded non-constructed.");
                }
            }
            Err(e) => panic!("{}", e),
        };
    }
}

// TODO: Tests to verify that my error handling from nested matches actually works.
