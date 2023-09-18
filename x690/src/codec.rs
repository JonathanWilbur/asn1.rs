use std::io::{Write, Result};
use crate::{
    X690Element,
    X690Value,
    x690_write_boolean_value,
    x690_write_object_identifier_value,
    x690_write_enum_value,
    x690_write_integer_value,
    x690_write_relative_oid_value,
    x690_write_date_value,
    x690_write_time_of_day_value,
    x690_write_string_value,
    x690_write_date_time_value,
    x690_write_time_value, x690_read_object_identifier_value,
    write_x690_node,
    _parse_sequence,
    x690_encode_external_components,
    x690_encode_character_string_components,
    x690_encode_embedded_pdv_components,
};
use asn1::{
    ASN1Result,
    ASN1Error,
    ASN1ErrorCode,
    read_i64,
    read_i128,
    Tag,
    MAX_IA5_STRING_CHAR_CODE,
    OPTIONAL,
};
use crate::{
    _RCTL1_FOR_EXTERNAL,
    _EAL_FOR_EXTERNAL,
    _RCTL2_FOR_EXTERNAL,
};
use asn1::types::{
    TagClass,
    ASN1Codec,
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
    CHARACTER_STRING,
    DATE,
    DATE_TIME,
    DURATION,
    EMBEDDED_PDV,
    ENUMERATED,
    InstanceOf,
    NULL,
    INSTANCE_OF,
    EXTERNAL,
    INTEGER,
    OBJECT_IDENTIFIER,
    OCTET_STRING,
    OID_IRI,
    REAL,
    RELATIVE_OID,
    SEQUENCE,
    SET,
    TIME,
    TIME_OF_DAY,
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
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
    ASN1_UNIVERSAL_TAG_NUMBER_SET,
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
    ContextNegotiation,
    ExternalEncoding,
    ExternalIdentification,
    PresentationContextSwitchingTypeIdentification,
    IdentificationSyntaxes,
};
use bytes::{Bytes, BytesMut, BufMut};
use std::sync::Arc;
use std::mem::size_of;

pub fn decode_presentation_context_switching_type_id(
    codec: impl X690Codec,
    el: &X690Element,
) -> ASN1Result<PresentationContextSwitchingTypeIdentification> {
    if el.tag.tag_class != TagClass::CONTEXT {
        let mut err =
            ASN1Error::new(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice);
        err.component_name = Some(String::from("identification"));
        err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
        err.length = Some(el.len());
        err.constructed = Some(el.is_constructed());
        return Err(err);
    }
    match el.tag.tag_number {
        0 => {
            // syntaxes
            if let X690Value::Constructed(children) = &el.value {
                if children.len() != 2 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                let r#abstract = codec.decode_object_identifier(&children[0])?;
                let transfer = codec.decode_object_identifier(&children[1])?;
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
            let v = codec.decode_object_identifier(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::syntax(v))
        }
        2 => {
            // presentation-context-id
            let v = codec.decode_integer(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::presentation_context_id(v))
        }
        3 => {
            // context-negotiation
            if let X690Value::Constructed(children) = &el.value {
                if children.len() != 2 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                let presentation_context_id = codec.decode_integer(&children[0])?;
                let transfer_syntax = codec.decode_object_identifier(&children[1])?;
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
            let v = codec.decode_object_identifier(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::transfer_syntax(v))
        }
        5 => {
            // fixed
            codec.decode_null(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::fixed)
        }
        _ => Err(ASN1Error::new(
            ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
        )),
    }
}

// Default implementations are defined where commonalities exist between BER, CER, and DER.
pub trait X690Codec {

    fn decode_presentation_context_switching_type_id(
        &self,
        el: &X690Element,
    ) -> ASN1Result<PresentationContextSwitchingTypeIdentification> {
        if el.tag.tag_class != TagClass::CONTEXT {
            let mut err =
                ASN1Error::new(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice);
            err.component_name = Some(String::from("identification"));
            err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
            err.length = Some(el.len());
            err.constructed = Some(el.is_constructed());
            return Err(err);
        }
        match el.tag.tag_number {
            0 => {
                // syntaxes
                if let X690Value::Constructed(children) = &el.value {
                    if children.len() != 2 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                    }
                    let r#abstract = self.decode_object_identifier(&children[0])?;
                    let transfer = self.decode_object_identifier(&children[1])?;
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
                let v = self.decode_object_identifier(el)?;
                Ok(PresentationContextSwitchingTypeIdentification::syntax(v))
            }
            2 => {
                // presentation-context-id
                let v = self.decode_integer(el)?;
                Ok(PresentationContextSwitchingTypeIdentification::presentation_context_id(v))
            }
            3 => {
                // context-negotiation
                if let X690Value::Constructed(children) = &el.value {
                    if children.len() != 2 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                    }
                    let presentation_context_id = self.decode_integer(&children[0])?;
                    let transfer_syntax = self.decode_object_identifier(&children[1])?;
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
                let v = self.decode_object_identifier(el)?;
                Ok(PresentationContextSwitchingTypeIdentification::transfer_syntax(v))
            }
            5 => {
                // fixed
                self.decode_null(el)?;
                Ok(PresentationContextSwitchingTypeIdentification::fixed)
            }
            _ => Err(ASN1Error::new(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
            )),
        }
    }

    fn decode_from_slice(&self, bytes: ByteSlice) -> ASN1Result<(usize, X690Element)>;
    fn decode_from_bytes(&self, bytes: Bytes) -> ASN1Result<(usize, X690Element)> {
        self.decode_from_slice(&bytes[..])
    }
    fn write<W>(&self, output: &mut W, el: &X690Element) -> Result<usize> where W: Write {
        write_x690_node(output, el)
    }
    fn decode_boolean_value(&self, value_bytes: ByteSlice) -> ASN1Result<BOOLEAN>;
    fn decode_integer_value(&self, value_bytes: ByteSlice) -> ASN1Result<INTEGER> {
        Ok(Vec::from(value_bytes))
    }
    fn decode_enum_value(&self, value_bytes: ByteSlice) -> ASN1Result<ENUMERATED> {
        match read_i64(value_bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
        }
    }
    fn decode_bit_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<BIT_STRING>;
    fn decode_octet_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<OCTET_STRING> {
        Ok(Vec::from(value_bytes))
    }
    fn decode_object_identifier_value(&self, value_bytes: ByteSlice) -> ASN1Result<OBJECT_IDENTIFIER> {
        x690_read_object_identifier_value(value_bytes)
    }
    fn decode_relative_oid_value(&self, value_bytes: ByteSlice) -> ASN1Result<RELATIVE_OID> {
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
    fn decode_real_value(&self, value_bytes: ByteSlice) -> ASN1Result<REAL>;
    fn decode_numeric_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<NumericString> {
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
    fn decode_printable_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<PrintableString> {
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
    fn decode_ia5_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<IA5String> {
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
    fn decode_utc_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<UTCTime>;
    fn decode_generalized_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<GeneralizedTime>;
    fn decode_graphic_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<GraphicString> {
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
    fn decode_visible_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<VisibleString> {
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
    fn decode_general_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<GeneralString> {
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
    fn decode_universal_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<UniversalString> {
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
        Ok(ret)
    }
    fn decode_bmp_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<BMPString> {
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
        Ok(ret)
    }
    fn decode_date_value(&self, value_bytes: ByteSlice) -> ASN1Result<DATE> {
        DATE::try_from(value_bytes)
    }
    fn decode_time_of_day_value(&self, value_bytes: ByteSlice) -> ASN1Result<TIME_OF_DAY> {
        TIME_OF_DAY::try_from(value_bytes)
    }
    fn decode_date_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<DATE_TIME> {
        DATE_TIME::try_from(value_bytes)
    }
    fn decode_duration_value(&self, value_bytes: ByteSlice) -> ASN1Result<DURATION>;
    fn decode_boolean(&self, el: &X690Element) -> ASN1Result<BOOLEAN> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_boolean_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_integer(&self, el: &X690Element) -> ASN1Result<INTEGER> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_integer_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_enumerated(&self, el: &X690Element) -> ASN1Result<ENUMERATED> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_enum_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_bit_string(&self, el: &X690Element) -> ASN1Result<BIT_STRING>;
    fn decode_octet_string(&self, el: &X690Element) -> ASN1Result<OCTET_STRING>;
    fn decode_null(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(bytes) => {
                if bytes.len() != 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                Ok(())
            }
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_object_identifier(&self, el: &X690Element) -> ASN1Result<OBJECT_IDENTIFIER> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_object_identifier_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_external(&self, el: &X690Element) -> ASN1Result<EXTERNAL> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
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
            Some(c) => match self.decode_object_identifier(c) {
                Ok(v) => Some(v),
                Err(e) => return Err(e),
            },
            None => None,
        };
        let indir_ref: OPTIONAL<INTEGER> = match components.get("indirect-reference") {
            Some(c) => match self.decode_integer(c) {
                Ok(v) => Some(v),
                Err(e) => return Err(e),
            },
            None => None,
        };
        let dvd: OPTIONAL<ObjectDescriptor> = match components.get("data-value-descriptor") {
            Some(c) => match self.decode_object_descriptor(c) {
                Ok(v) => Some(v),
                Err(e) => return Err(e),
            },
            None => None,
        };
        let encoding: ExternalEncoding = match components.get("encoding") {
            Some(c) => {
                if c.tag.tag_class != TagClass::CONTEXT {
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice);
                    err.component_name = Some(String::from("encoding"));
                    err.tag = Some(Tag::new(c.tag.tag_class, c.tag.tag_number));
                    err.length = Some(c.len());
                    err.constructed = Some(c.is_constructed());
                    return Err(err);
                }
                match c.tag.tag_number {
                    0 => {
                        if let X690Value::Constructed(components) = &c.value {
                            if components.len() != 1 {
                                return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                            }
                            let v = self.decode_any(&components[0])?;
                            ExternalEncoding::single_ASN1_type(Arc::new(v))
                        } else {
                            return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                        }
                    }
                    1 => {
                        let v = self.decode_octet_string(c)?;
                        ExternalEncoding::octet_aligned(v)
                    }
                    2 => {
                        let v = self.decode_bit_string(c)?;
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
    fn decode_instance_of(&self, el: &X690Element) -> ASN1Result<InstanceOf> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        if elements.len() != 2 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
        }
        if elements[0].tag.tag_class != TagClass::UNIVERSAL
            || elements[0].tag.tag_number != ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER
            || elements[1].tag.tag_class != TagClass::CONTEXT
            || elements[1].tag.tag_number != 0
        {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
        }
        let type_id: OBJECT_IDENTIFIER = self.decode_object_identifier(&elements[0])?;
        let value = self.decode_any(&elements[1].inner()?)?;

        Ok(InstanceOf {
            type_id,
            value: Arc::new(value),
        })
    }
    fn decode_embedded_pdv(&self, el: &X690Element) -> ASN1Result<EMBEDDED_PDV> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        if elements.len() != 2 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
        }
        let identification = self.decode_presentation_context_switching_type_id( &elements[0].inner()?)?;
        let data_value: OCTET_STRING = self.decode_octet_string(&elements[1])?;
        Ok(EMBEDDED_PDV {
            identification,
            data_value,
        })
    }
    fn decode_character_string(&self, el: &X690Element) -> ASN1Result<CHARACTER_STRING> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        if elements.len() != 2 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
        }
        let identification = self.decode_presentation_context_switching_type_id(&elements[0].inner()?)?;
        let string_value: OCTET_STRING = self.decode_octet_string(&elements[1])?;
        Ok(CHARACTER_STRING {
            identification,
            string_value,
        })
    }
    fn decode_relative_oid(&self, el: &X690Element) -> ASN1Result<RELATIVE_OID> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_relative_oid_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_sequence(&self, el: &X690Element) -> ASN1Result<SEQUENCE>;
    fn decode_set(&self, el: &X690Element) -> ASN1Result<SET>;
    fn decode_object_descriptor(&self, el: &X690Element) -> ASN1Result<ObjectDescriptor>;
    fn decode_utf8_string(&self, el: &X690Element) -> ASN1Result<UTF8String>;
    fn decode_real(&self, el: &X690Element) -> ASN1Result<REAL> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_real_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_numeric_string(&self, el: &X690Element) -> ASN1Result<NumericString>;
    fn decode_printable_string(&self, el: &X690Element) -> ASN1Result<PrintableString>;
    fn decode_t61_string(&self, el: &X690Element) -> ASN1Result<T61String>;
    fn decode_videotex_string(&self, el: &X690Element) -> ASN1Result<VideotexString>;
    fn decode_ia5_string(&self, el: &X690Element) -> ASN1Result<IA5String>;
    fn decode_utc_time(&self, el: &X690Element) -> ASN1Result<UTCTime>;
    fn decode_generalized_time(&self, el: &X690Element) -> ASN1Result<GeneralizedTime>;
    fn decode_graphic_string(&self, el: &X690Element) -> ASN1Result<GraphicString>;
    fn decode_visible_string(&self, el: &X690Element) -> ASN1Result<VisibleString>;
    fn decode_general_string(&self, el: &X690Element) -> ASN1Result<GeneralString>;
    fn decode_universal_string(&self, el: &X690Element) -> ASN1Result<UniversalString>;
    fn decode_bmp_string(&self, el: &X690Element) -> ASN1Result<BMPString>;
    fn decode_date(&self, el: &X690Element) -> ASN1Result<DATE> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_date_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_time_of_day(&self, el: &X690Element) -> ASN1Result<TIME_OF_DAY> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_time_of_day_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_date_time(&self, el: &X690Element) -> ASN1Result<DATE_TIME> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_date_time_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_duration(&self, el: &X690Element) -> ASN1Result<DURATION> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_duration_value(bytes),
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_oid_iri(&self, el: &X690Element) -> ASN1Result<OID_IRI> {
        match &el.value {
            X690Value::Primitive(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(x) => Ok(x),
                Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_relative_oid_iri(&self, el: &X690Element) -> ASN1Result<OID_IRI> {
        match &el.value {
            X690Value::Primitive(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(x) => Ok(x),
                Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_time(&self, el: &X690Element) -> ASN1Result<TIME> {
        match &el.value {
            X690Value::Primitive(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(x) => Ok(x),
                Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }
    fn decode_any(&self, el: &X690Element) -> ASN1Result<ASN1Value>;
    fn encode_boolean(&self, value: &BOOLEAN) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(1).writer();
        x690_write_boolean_value(&mut out, value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_integer(&self, value: &INTEGER) -> ASN1Result<X690Element> {
        let mut out = BytesMut::new().writer();
        x690_write_integer_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_enumerated(&self, value: &ENUMERATED) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(2).writer(); // Most enums are small.
        x690_write_enum_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_bit_string(&self, value: &BIT_STRING) -> ASN1Result<X690Element>;
    fn encode_octet_string(&self, value: &OCTET_STRING) -> ASN1Result<X690Element>;
    fn encode_null(&self, _value: &NULL) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_NULL),
            X690Value::Primitive(Bytes::new()),
        ))
    }
    fn encode_object_identifier(&self, value: &OBJECT_IDENTIFIER) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.0.len() << 2).writer(); // We assume, on average, each arc takes two bytes.
        x690_write_object_identifier_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_external(&self, value: &EXTERNAL) -> ASN1Result<X690Element> {
        let components = x690_encode_external_components(value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL),
            X690Value::Constructed(Arc::new(components)),
        ))
    }
    fn encode_instance_of(&self, value: &INSTANCE_OF) -> ASN1Result<X690Element> {
        let external = EXTERNAL {
            identification: ExternalIdentification::syntax(value.type_id.clone()),
            data_value_descriptor: None,
            data_value: ExternalEncoding::single_ASN1_type(value.value.clone()),
        };
        let components = x690_encode_external_components(&external)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL),
            X690Value::Constructed(Arc::new(components)),
        ))
    }
    fn encode_real(&self, value: &REAL) -> ASN1Result<X690Element>;
    fn encode_embedded_pdv(&self, value: &EMBEDDED_PDV) -> ASN1Result<X690Element> {
        let components = x690_encode_embedded_pdv_components(value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV),
            X690Value::Constructed(Arc::new(components)),
        ))
    }
    fn encode_character_string(&self, value: &CHARACTER_STRING) -> ASN1Result<X690Element> {
        let components = x690_encode_character_string_components(value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING),
            X690Value::Constructed(Arc::new(components)),
        ))
    }
    fn encode_relative_oid(&self, value: &RELATIVE_OID) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.0.len() << 2).writer(); // We assume, on average, each arc takes two bytes.
        x690_write_relative_oid_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_object_descriptor(&self, value: &ObjectDescriptor) -> ASN1Result<X690Element>;
    fn encode_utf8_string(&self, value: &str) -> ASN1Result<X690Element>;
    fn encode_numeric_string(&self, value: &str) -> ASN1Result<X690Element>;
    fn encode_printable_string(&self, value: &str) -> ASN1Result<X690Element>;
    fn encode_t61_string(&self, value: &T61String) -> ASN1Result<X690Element>;
    fn encode_videotex_string(&self, value: &VideotexString) -> ASN1Result<X690Element>;
    fn encode_ia5_string(&self, value: &IA5String) -> ASN1Result<X690Element>;
    fn encode_utc_time(&self, value: &UTCTime) -> ASN1Result<X690Element>;
    fn encode_generalized_time(&self, value: &GeneralizedTime) -> ASN1Result<X690Element>;
    fn encode_graphic_string(&self, value: &GraphicString) -> ASN1Result<X690Element>;
    fn encode_visible_string(&self, value: &VisibleString) -> ASN1Result<X690Element>;
    fn encode_general_string(&self, value: &GeneralString) -> ASN1Result<X690Element>;
    fn encode_universal_string(&self, value: &UniversalString) -> ASN1Result<X690Element>;
    fn encode_bmp_string(&self, value: &BMPString) -> ASN1Result<X690Element>;
    fn encode_date(&self, value: &DATE) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(10).writer(); // YYYY-MM-DD
        x690_write_date_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_DATE),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_time_of_day(&self, value: &TIME_OF_DAY) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(8).writer(); // HH:MM:SS
        x690_write_time_of_day_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_date_time(&self, value: &DATE_TIME) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(19).writer(); // 1951-10-14T15:30:00
        x690_write_date_time_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_duration(&self, value: &DURATION) -> ASN1Result<X690Element>;
    fn encode_oid_iri(&self, value: &OID_IRI) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_relative_oid_iri(&self, value: &OID_IRI) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn encode_time(&self, value: &TIME) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_time_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }
    fn validate_boolean_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_integer_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_bit_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_octet_string_value(&self, _content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_null_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_object_identifier_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_object_descriptor_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_real_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_enumerated_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_utf8_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_relative_object_identifier_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_time_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_numeric_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_printable_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_t61_string_value(&self, _content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_videotex_string_value(&self, _content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_ia5_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_utc_time_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_generalized_time_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_graphic_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_visible_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_general_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_universal_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_bmp_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_date_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_time_of_day_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_date_time_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_duration_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_oid_iri_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;
    fn validate_relative_oid_iri_value(&self, _content_octets: ByteSlice) -> ASN1Result<()>;

    fn validate_boolean(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_boolean_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_integer(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_integer_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_bit_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_octet_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_null(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_null_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_object_identifier(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_object_identifier_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_object_descriptor(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_real(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_real_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_enumerated(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_enumerated_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_utf8_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_relative_object_identifier(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_relative_object_identifier_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_time(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_time_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_numeric_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_printable_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_t61_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_videotex_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_ia5_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_utc_time(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_generalized_time(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_graphic_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_visible_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_general_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_universal_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_bmp_string(&self, el: &X690Element) -> ASN1Result<()>;
    fn validate_date(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_date_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_time_of_day(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_time_of_day_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_date_time(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_date_time_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_duration(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_duration_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_oid_iri(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_oid_iri_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }
    fn validate_relative_oid_iri(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_relative_oid_iri_value(&v),
            _ => {
                let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                err.relate_tag(&el.tag);
                err.constructed = Some(el.is_constructed());
                Err(err)
            },
        }
    }

    fn validate_external(&self, el: &X690Element) -> ASN1Result<()> {
        let invalid_cons = || {
            let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
            err.relate_tag(&el.tag);
            err.constructed = Some(el.is_constructed());
            err
        };
        let components = match &el.value {
            X690Value::Constructed(c) => c,
            _ => return Err(invalid_cons()),
        };
        let len = components.len();
        if len > 4 || len == 0 {
            return Err(invalid_cons());
        }
        let last_el = &components[len - 1];
        if last_el.tag.tag_class != TagClass::CONTEXT {
            return Err(invalid_cons());
        }
        match last_el.tag.tag_number {
            0 => self.validate_any(&last_el.inner()?)?,
            1 => self.validate_octet_string(&last_el)?,
            2 => self.validate_bit_string(&last_el)?,
            _ => return Err(invalid_cons()),
        };
        let mut s = 0; // component spec index.
        let mut int_seen: bool = false;
        let mut desc_seen: bool = false;
        for component in components[0..len - 1].iter() {
            if component.tag.tag_class != TagClass::UNIVERSAL {
                return Err(invalid_cons());
            }
            if
                s == 0
                && component.tag.tag_number == ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER {
                self.validate_object_identifier(component)?;
            }
            else if s <= 1
                && !int_seen
                && component.tag.tag_number == ASN1_UNIVERSAL_TAG_NUMBER_INTEGER {
                self.validate_integer(component)?;
                int_seen = true;
            }
            else if s <= 2
                && !desc_seen
                && component.tag.tag_number == ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR {
                self.validate_object_descriptor(component)?;
                desc_seen = true;
            }
            else {
                return Err(invalid_cons());
            }
            s += 1;
        }
        Ok(())
    }

    fn validate_embedded_pdv(&self, el: &X690Element) -> ASN1Result<()> {
        let invalid_cons = || {
            let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
            err.relate_tag(&el.tag);
            err.constructed = Some(el.is_constructed());
            err
        };
        let components = match &el.value {
            X690Value::Constructed(c) => c,
            _ => return Err(invalid_cons()),
        };
        let len = components.len();
        if len != 2 {
            return Err(invalid_cons());
        }
        let id_el = &components[0];
        let value_el = &components[1];
        if
            id_el.tag.tag_class != TagClass::CONTEXT
            || value_el.tag.tag_class != TagClass::CONTEXT
            || id_el.tag.tag_number != 0
            || value_el.tag.tag_number != 1
        {
            return Err(invalid_cons());
        }
        self.validate_octet_string(value_el)?;
        match id_el.tag.tag_number {
            0 => { // syntaxes
                let subs = match &id_el.value {
                    X690Value::Constructed(s) => s,
                    _ => return Err(invalid_cons()),
                };
                if
                    subs.len() != 2
                    || !subs.iter().all(|s| s.tag.tag_class != TagClass::CONTEXT)
                    || subs[0].tag.tag_number != 0
                    || subs[1].tag.tag_number != 1
                {
                    return Err(invalid_cons());
                }
                for sub in subs.iter() {
                    self.validate_object_identifier(sub)?;
                }
            },
            1 | 4 => self.validate_object_identifier(id_el)?, // syntax or transfer-syntax
            2 => self.validate_integer(id_el)?, // presentation-context-id
            3 => { // context-negotiation
                let subs = match &id_el.value {
                    X690Value::Constructed(s) => s,
                    _ => return Err(invalid_cons()),
                };
                if
                    subs.len() != 2
                    || !subs.iter().all(|s| s.tag.tag_class != TagClass::CONTEXT)
                    || subs[0].tag.tag_number != 0
                    || subs[1].tag.tag_number != 1
                {
                    return Err(invalid_cons());
                }
                self.validate_integer(&subs[0])?;
                self.validate_object_identifier(&subs[1])?;
            },
            5 => self.validate_null(id_el)?,
            _ => return Err(invalid_cons()),
        }
        Ok(())
    }

    fn validate_character_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_embedded_pdv(el)
    }

    fn validate_any (&self, el: &X690Element) -> ASN1Result<()> {
        let invalid_cons = || {
            let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
            err.relate_tag(&el.tag);
            err.constructed = Some(el.is_constructed());
            err
        };
        if el.tag.tag_class != TagClass::UNIVERSAL {
            match &el.value {
                X690Value::Primitive(_) => return Ok(()), // IMPLICIT tagged unknown type.
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        self.validate_any(sub)?;
                    }
                    return Ok(());
                }
            };
        }
        match el.tag.tag_number {
            ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT => self.validate_null(el),
            ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN => self.validate_boolean(el),
            ASN1_UNIVERSAL_TAG_NUMBER_INTEGER => self.validate_integer(el),
            ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING => self.validate_bit_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING => self.validate_octet_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_NULL => self.validate_null(el),
            ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER => self.validate_object_identifier(el),
            ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR => self.validate_object_descriptor(el),
            ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL => self.validate_external(el),
            ASN1_UNIVERSAL_TAG_NUMBER_REAL => self.validate_real(el),
            ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED => self.validate_enumerated(el),
            ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV => self.validate_embedded_pdv(el),
            ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING => self.validate_utf8_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID => self.validate_relative_object_identifier(el),
            ASN1_UNIVERSAL_TAG_NUMBER_TIME => self.validate_time(el),
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE | ASN1_UNIVERSAL_TAG_NUMBER_SET => {
                // NOTE: You can't check for duplicate tags in a SET because it could actually be a SET OF.
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            self.validate_any(sub)?;
                        }
                        Ok(())
                    },
                    _ => return Err(invalid_cons()),
                }
            },
            ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING => self.validate_numeric_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING => self.validate_printable_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING => self.validate_t61_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING => self.validate_videotex_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING => self.validate_ia5_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME => self.validate_utc_time(el),
            ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME => self.validate_generalized_time(el),
            ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING => self.validate_graphic_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING => self.validate_visible_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING => self.validate_general_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING => self.validate_universal_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING => self.validate_character_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING => self.validate_bmp_string(el),
            ASN1_UNIVERSAL_TAG_NUMBER_DATE => self.validate_date(el),
            ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY => self.validate_time_of_day(el),
            ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME => self.validate_date_time(el),
            ASN1_UNIVERSAL_TAG_NUMBER_DURATION => self.validate_duration(el),
            ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI => self.validate_oid_iri(el),
            ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI => self.validate_relative_oid_iri(el),
            _ => Ok(()), // It's hard to say what to do here. Accepting is future-proof, but the value could be invalid.
        }
    }

    fn write_boolean<W>(&self, output: &mut W, value: &BOOLEAN) -> Result<usize> where W: Write {
        let enc = self.encode_boolean(value)?;
        self.write(output, &enc)
    }
    fn write_integer<W>(&self, output: &mut W, value: &INTEGER) -> Result<usize> where W: Write {
        let enc = self.encode_integer(value)?;
        self.write(output, &enc)
    }
    fn write_enumerated<W>(&self, output: &mut W, value: &ENUMERATED) -> Result<usize> where W: Write {
        let enc = self.encode_enumerated(value)?;
        self.write(output, &enc)
    }
    fn write_bit_string<W>(&self, output: &mut W, value: &BIT_STRING) -> Result<usize> where W: Write {
        let enc = self.encode_bit_string(value)?;
        self.write(output, &enc)
    }
    fn write_octet_string<W>(&self, output: &mut W, value: &OCTET_STRING) -> Result<usize> where W: Write {
        let enc = self.encode_octet_string(value)?;
        self.write(output, &enc)
    }
    fn write_null<W>(&self, output: &mut W, value: &NULL) -> Result<usize> where W: Write {
        let enc = self.encode_null(value)?;
        self.write(output, &enc)
    }
    fn write_object_identifier<W>(&self, output: &mut W, value: &OBJECT_IDENTIFIER) -> Result<usize> where W: Write {
        let enc = self.encode_object_identifier(value)?;
        self.write(output, &enc)
    }
    fn write_external<W>(&self, output: &mut W, value: &EXTERNAL) -> Result<usize> where W: Write {
        let enc = self.encode_external(value)?;
        self.write(output, &enc)
    }
    fn write_instance_of<W>(&self, output: &mut W, value: &INSTANCE_OF) -> Result<usize> where W: Write {
        let enc = self.encode_instance_of(value)?;
        self.write(output, &enc)
    }
    fn write_real<W>(&self, output: &mut W, value: &REAL) -> Result<usize> where W: Write {
        let enc = self.encode_real(value)?;
        self.write(output, &enc)
    }
    fn write_embedded_pdv<W>(&self, output: &mut W, value: &EMBEDDED_PDV) -> Result<usize> where W: Write {
        let enc = self.encode_embedded_pdv(value)?;
        self.write(output, &enc)
    }
    fn write_character_string<W>(&self, output: &mut W, value: &CHARACTER_STRING) -> Result<usize> where W: Write {
        let enc = self.encode_character_string(value)?;
        self.write(output, &enc)
    }
    fn write_relative_oid<W>(&self, output: &mut W, value: &RELATIVE_OID) -> Result<usize> where W: Write {
        let enc = self.encode_relative_oid(value)?;
        self.write(output, &enc)
    }
    fn write_object_descriptor<W>(&self, output: &mut W, value: &ObjectDescriptor) -> Result<usize> where W: Write {
        let enc = self.encode_object_descriptor(value)?;
        self.write(output, &enc)
    }
    fn write_utf8_string<W>(&self, output: &mut W, value: &str) -> Result<usize> where W: Write {
        let enc = self.encode_utf8_string(value)?;
        self.write(output, &enc)
    }
    fn write_numeric_string<W>(&self, output: &mut W, value: &str) -> Result<usize> where W: Write {
        let enc = self.encode_numeric_string(value)?;
        self.write(output, &enc)
    }
    fn write_printable_string<W>(&self, output: &mut W, value: &str) -> Result<usize> where W: Write {
        let enc = self.encode_printable_string(value)?;
        self.write(output, &enc)
    }
    fn write_t61_string<W>(&self, output: &mut W, value: &T61String) -> Result<usize> where W: Write {
        let enc = self.encode_t61_string(value)?;
        self.write(output, &enc)
    }
    fn write_videotex_string<W>(&self, output: &mut W, value: &VideotexString) -> Result<usize> where W: Write {
        let enc = self.encode_videotex_string(value)?;
        self.write(output, &enc)
    }
    fn write_ia5_string<W>(&self, output: &mut W, value: &IA5String) -> Result<usize> where W: Write {
        let enc = self.encode_ia5_string(value)?;
        self.write(output, &enc)
    }
    fn write_utc_time<W>(&self, output: &mut W, value: &UTCTime) -> Result<usize> where W: Write {
        let enc = self.encode_utc_time(value)?;
        self.write(output, &enc)
    }
    fn write_generalized_time<W>(&self, output: &mut W, value: &GeneralizedTime) -> Result<usize> where W: Write {
        let enc = self.encode_generalized_time(value)?;
        self.write(output, &enc)
    }
    fn write_graphic_string<W>(&self, output: &mut W, value: &GraphicString) -> Result<usize> where W: Write {
        let enc = self.encode_graphic_string(value)?;
        self.write(output, &enc)
    }
    fn write_visible_string<W>(&self, output: &mut W, value: &VisibleString) -> Result<usize> where W: Write {
        let enc = self.encode_visible_string(value)?;
        self.write(output, &enc)
    }
    fn write_general_string<W>(&self, output: &mut W, value: &GeneralString) -> Result<usize> where W: Write {
        let enc = self.encode_general_string(value)?;
        self.write(output, &enc)
    }
    fn write_universal_string<W>(&self, output: &mut W, value: &UniversalString) -> Result<usize> where W: Write {
        let enc = self.encode_universal_string(value)?;
        self.write(output, &enc)
    }
    fn write_bmp_string<W>(&self, output: &mut W, value: &BMPString) -> Result<usize> where W: Write {
        let enc = self.encode_bmp_string(value)?;
        self.write(output, &enc)
    }
    fn write_date<W>(&self, output: &mut W, value: &DATE) -> Result<usize> where W: Write {
        let enc = self.encode_date(value)?;
        self.write(output, &enc)
    }
    fn write_time_of_day<W>(&self, output: &mut W, value: &TIME_OF_DAY) -> Result<usize> where W: Write {
        let enc = self.encode_time_of_day(value)?;
        self.write(output, &enc)
    }
    fn write_date_time<W>(&self, output: &mut W, value: &DATE_TIME) -> Result<usize> where W: Write {
        let enc = self.encode_date_time(value)?;
        self.write(output, &enc)
    }
    fn write_duration<W>(&self, output: &mut W, value: &DURATION) -> Result<usize> where W: Write {
        let enc = self.encode_duration(value)?;
        self.write(output, &enc)
    }
    fn write_oid_iri<W>(&self, output: &mut W, value: &OID_IRI) -> Result<usize> where W: Write {
        let enc = self.encode_oid_iri(value)?;
        self.write(output, &enc)
    }
    fn write_relative_oid_iri<W>(&self, output: &mut W, value: &OID_IRI) -> Result<usize> where W: Write {
        let enc = self.encode_relative_oid_iri(value)?;
        self.write(output, &enc)
    }
    fn write_time<W>(&self, output: &mut W, value: &TIME) -> Result<usize> where W: Write {
        let enc = self.encode_time(value)?;
        self.write(output, &enc)
    }

    fn encode_i8 (&self, value: i8) -> ASN1Result<X690Element> {
        let mut content = BytesMut::with_capacity(1);
        content.put_i8(value);
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
            X690Value::Primitive(content.into()),
        ))
    }

    fn decode_i8 (&self, el: &X690Element) -> ASN1Result<i8> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_i8 (&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = el.content_octets()?;
        if content_octets.len() == 1 {
            Ok(())
        } else {
            Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
        }
    }

    fn encode_u8 (&self, value: u8) -> ASN1Result<X690Element> {
        self.encode_i16(value.into())
    }

    fn decode_u8 (&self, el: &X690Element) -> ASN1Result<u8> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_u8 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u8 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    fn encode_i16 (&self, value: i16) -> ASN1Result<X690Element> {
        let possible_i8 = value.try_into();
        match possible_i8 {
            Ok(pi8) => self.encode_i8(pi8),
            Err(_) => {
                let mut content = BytesMut::with_capacity(2);
                content.put_i16(value);
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
                    X690Value::Primitive(content.into()),
                ))
            },
        }
    }

    fn decode_i16 (&self, el: &X690Element) -> ASN1Result<i16> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_i16 (&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = el.content_octets()?;
        if content_octets.len() > 2 {
            Ok(())
        } else {
            Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
        }
    }

    fn encode_u16 (&self, value: u16) -> ASN1Result<X690Element> {
        self.encode_i32(value.into())
    }

    fn decode_u16 (&self, el: &X690Element) -> ASN1Result<u16> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_u16 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u16 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    fn encode_i32 (&self, value: i32) -> ASN1Result<X690Element> {
        let possible_i16 = value.try_into();
        match possible_i16 {
            Ok(pi16) => self.encode_i16(pi16),
            Err(_) => {
                let octets = value.to_be_bytes();
                let padded: bool = ((octets[0] == 0x00) && ((octets[1] & 0b1000_0000) == 0))
                    || ((octets[0] == 0xFF) && ((octets[1] & 0b1000_0000) > 0));
                if padded {
                    let mut content = BytesMut::with_capacity(3);
                    content.put_slice(&octets[1..]);
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
                        X690Value::Primitive(content.into()),
                    ))
                } else {
                    let mut content = BytesMut::with_capacity(4);
                    content.put_i32(value);
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
                        X690Value::Primitive(content.into()),
                    ))
                }
            },
        }
    }

    fn decode_i32 (&self, el: &X690Element) -> ASN1Result<i32> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_i32 (&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = el.content_octets()?;
        if content_octets.len() > 4 {
            Ok(())
        } else {
            Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
        }
    }

    fn encode_u32 (&self, value: u32) -> ASN1Result<X690Element> {
        self.encode_i64(value.into())
    }

    fn decode_u32 (&self, el: &X690Element) -> ASN1Result<u32> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_u32 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u32 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    fn encode_i64 (&self, value: i64) -> ASN1Result<X690Element> {
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
        let mut content = BytesMut::with_capacity(8);
        /* If the bytes are 0xFFFFFFFF... or 0x00000000..., encode as 0xFF or 0x00 */
        if (number_of_padding_bytes == size_of::<i64>())
            // Or add single pad byte if needed.
            || (value >= 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) > 0))
            || (value < 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) == 0)) {
            content.put_u8(padding_byte);
        }
        content.put_slice(&(bytes[number_of_padding_bytes..size_of::<i64>()]));
        return Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    fn decode_i64 (&self, el: &X690Element) -> ASN1Result<i64> {
        let int_bytes = self.decode_integer(el)?;
        read_i64(&int_bytes).map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    // TODO: This is technically incorrect. It does not check padding.
    fn validate_i64 (&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = el.content_octets()?;
        if content_octets.len() > 8 {
            Ok(())
        } else {
            Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
        }
    }

    fn encode_u64 (&self, value: u64) -> ASN1Result<X690Element> {
        let bytes: [u8; 8] = value.to_be_bytes();
        let padding_byte: u8 = 0x00;
        let mut number_of_padding_bytes: usize = 0;
        for byte in bytes {
            if byte == padding_byte {
                number_of_padding_bytes += 1;
            } else {
                break;
            }
        }
        let mut content = BytesMut::with_capacity(8);
        /* If the bytes are 0xFFFFFFFF... or 0x00000000..., encode as 0xFF or 0x00 */
        if (number_of_padding_bytes == size_of::<i64>())
            // Or add single pad byte if needed.
            || ((bytes[number_of_padding_bytes] & 0b1000_0000) > 0) {
                content.put_u8(padding_byte);
        }
        content.put_slice(&(bytes[number_of_padding_bytes..size_of::<i64>()]));
        return Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    fn decode_u64 (&self, el: &X690Element) -> ASN1Result<u64> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_u64 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u64 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    fn encode_i128 (&self, value: i128) -> ASN1Result<X690Element> {
        let bytes: [u8; 16] = value.to_be_bytes();
        let padding_byte: u8 = if value >= 0 { 0x00 } else { 0xFF };
        let mut number_of_padding_bytes: usize = 0;
        for byte in bytes {
            if byte == padding_byte {
                number_of_padding_bytes += 1;
            } else {
                break;
            }
        }
        let mut content = BytesMut::with_capacity(16);
        /* If the bytes are 0xFFFFFFFF... or 0x00000000..., encode as 0xFF or 0x00 */
        if (number_of_padding_bytes == size_of::<i128>())
            // Or add single pad byte if needed.
            || (value >= 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) > 0))
            || (value < 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) == 0)) {
            content.put_u8(padding_byte);
        }
        content.put_slice(&(bytes[number_of_padding_bytes..size_of::<i128>()]));
        return Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    fn decode_i128 (&self, el: &X690Element) -> ASN1Result<i128> {
        let int_bytes = self.decode_integer(el)?; // TODO: Use content_octets instead to avoid a clone.
        read_i128(&int_bytes).map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    fn validate_i128 (&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = el.content_octets()?;
        if content_octets.len() > 16 {
            Ok(())
        } else {
            Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
        }
    }

    fn encode_u128 (&self, value: u128) -> ASN1Result<X690Element> {
        let bytes: [u8; 16] = value.to_be_bytes();
        let padding_byte: u8 = 0x00;
        let mut number_of_padding_bytes: usize = 0;
        for byte in bytes {
            if byte == padding_byte {
                number_of_padding_bytes += 1;
            } else {
                break;
            }
        }
        let mut content = BytesMut::with_capacity(16);
        /* If the bytes are 0xFFFFFFFF... or 0x00000000..., encode as 0xFF or 0x00 */
        if (number_of_padding_bytes == size_of::<i128>())
            // Or add single pad byte if needed.
            || ((bytes[number_of_padding_bytes] & 0b1000_0000) > 0) {
                content.put_u8(padding_byte);
        }
        content.put_slice(&(bytes[number_of_padding_bytes..size_of::<i128>()]));
        return Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    // TODO:
    // fn decode_u128 (&self, el: &X690Element) -> ASN1Result<u128> {

    // }

    // fn validate_u128 (&self, el: &X690Element) -> ASN1Result<()> {

    // }

    // TODO: Implement X690Element::from() all the integer types, string, [u8], etc.

}

// TODO: write_indefinite

pub struct BasicEncodingRules;
pub struct CanonicalEncodingRules;
pub struct DistinguishedEncodingRules;

impl BasicEncodingRules {
    pub const fn new() -> Self {
        BasicEncodingRules {}
    }
}

impl CanonicalEncodingRules {
    pub const fn new() -> Self {
        CanonicalEncodingRules {}
    }
}

impl DistinguishedEncodingRules {
    pub const fn new() -> Self {
        DistinguishedEncodingRules {}
    }
}

impl Default for BasicEncodingRules {
    fn default() -> Self {
        BasicEncodingRules::new()
    }
}

impl Default for CanonicalEncodingRules {
    fn default() -> Self {
        CanonicalEncodingRules::new()
    }
}

impl Default for DistinguishedEncodingRules {
    fn default() -> Self {
        DistinguishedEncodingRules::new()
    }
}

impl ASN1Codec for BasicEncodingRules {
    fn transfer_syntax_oid(&self) -> OBJECT_IDENTIFIER {
        OBJECT_IDENTIFIER(vec![2, 1, 1])
    }

    fn transfer_syntax_oid_iri(&self) -> Option<asn1::OID_IRI> {
        Some("/ASN.1/Basic-Encoding".into())
    }
}

impl ASN1Codec for CanonicalEncodingRules {
    fn transfer_syntax_oid(&self) -> OBJECT_IDENTIFIER {
        OBJECT_IDENTIFIER(vec![2, 1, 2, 0])
    }

    fn transfer_syntax_oid_iri(&self) -> Option<asn1::OID_IRI> {
        Some("/ASN.1/BER-Derived/Canonical-Encoding".into())
    }
}

impl ASN1Codec for DistinguishedEncodingRules {
    fn transfer_syntax_oid(&self) -> OBJECT_IDENTIFIER {
        OBJECT_IDENTIFIER(vec![2, 1, 2, 1])
    }

    fn transfer_syntax_oid_iri(&self) -> Option<asn1::OID_IRI> {
        Some("/ASN.1/BER-Derived/Distinguished-Encoding".into())
    }
}
