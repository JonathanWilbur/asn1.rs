//! `X690Codec` trait, which is implemented by all X.690 codecs
use std::io::{Write, Result};
use crate::{
    X690Element,
    X690Value,
    x690_write_object_identifier_value,
    x690_write_enum_value,
    x690_write_integer_value,
    x690_write_relative_oid_value,
    x690_write_date_value,
    x690_write_time_of_day_value,
    x690_write_string_value,
    x690_write_date_time_value,
    x690_write_time_value, x690_read_object_identifier_value,
    x690_write_tlv,
    x690_encode_external_components,
    x690_encode_character_string_components,
    x690_encode_embedded_pdv_components,
    X690StructureIterator,
    BER,
};
use wildboar_asn1::{
    X690Validate,
    ASN1Result,
    ASN1Error,
    ASN1ErrorCode,
    read_i64,
    read_i128,
    Tag,
    OPTIONAL,
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
    UNIV_TAG_END_OF_CONTENT,
    UNIV_TAG_BOOLEAN,
    UNIV_TAG_INTEGER,
    UNIV_TAG_BIT_STRING,
    UNIV_TAG_OCTET_STRING,
    UNIV_TAG_NULL,
    UNIV_TAG_OBJECT_IDENTIFIER,
    UNIV_TAG_OBJECT_DESCRIPTOR,
    UNIV_TAG_EXTERNAL,
    UNIV_TAG_REAL,
    UNIV_TAG_ENUMERATED,
    UNIV_TAG_EMBEDDED_PDV,
    UNIV_TAG_UTF8_STRING,
    UNIV_TAG_RELATIVE_OID,
    UNIV_TAG_TIME,
    UNIV_TAG_SEQUENCE,
    UNIV_TAG_SET,
    UNIV_TAG_NUMERIC_STRING,
    UNIV_TAG_PRINTABLE_STRING,
    UNIV_TAG_T61_STRING,
    UNIV_TAG_VIDEOTEX_STRING,
    UNIV_TAG_IA5_STRING,
    UNIV_TAG_UTC_TIME,
    UNIV_TAG_GENERALIZED_TIME,
    UNIV_TAG_GRAPHIC_STRING,
    UNIV_TAG_VISIBLE_STRING,
    UNIV_TAG_GENERAL_STRING,
    UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_CHARACTER_STRING,
    UNIV_TAG_BMP_STRING,
    UNIV_TAG_DATE,
    UNIV_TAG_TIME_OF_DAY,
    UNIV_TAG_DATE_TIME,
    UNIV_TAG_DURATION,
    UNIV_TAG_OID_IRI,
    UNIV_TAG_RELATIVE_OID_IRI,
    ContextNegotiation,
    ExternalEncoding,
    ExternalIdentification,
    PresentationContextSwitchingTypeIdentification,
    IdentificationSyntaxes,
};
use crate::{
    _RCTL1_FOR_EXTERNAL,
    _EAL_FOR_EXTERNAL,
    _RCTL2_FOR_EXTERNAL,
};
use bytes::{Bytes, BytesMut, BufMut};
use std::sync::Arc;
use std::mem::size_of;
#[cfg(feature = "simdutf8")]
use simdutf8::basic::from_utf8;

/// Any codec defined in ITU-T Recommendation X.690.
///
/// Default implementations are defined where commonalities exist between BER, CER, and DER.
pub trait X690Codec {

    /// Decode the `identification` field of a context switching type, such as
    /// a `EMBEDDED PDV` or `CHARACTER STRING`.
    fn decode_presentation_context_switching_type_id(
        &self,
        el: &X690Element,
    ) -> ASN1Result<PresentationContextSwitchingTypeIdentification> {
        if el.tag.tag_class != TagClass::CONTEXT {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "identification"));
        }
        match el.tag.tag_number {
            0 => {
                // syntaxes
                let children = el.value.components()?;
                if children.len() != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "identification.syntaxes"));
                }
                let r#abstract = self.decode_object_identifier(&children[0])?;
                let transfer = self.decode_object_identifier(&children[1])?;
                Ok(PresentationContextSwitchingTypeIdentification::syntaxes(
                    IdentificationSyntaxes {
                        r#abstract,
                        transfer,
                    },
                ))
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
                let children = el.value.components()?;
                if children.len() != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "identification.context-negotiation"));
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
            _ => Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "identification",
            )),
        }
    }

    /// Validate the `identification` field of a context switching type, such as
    /// a `EMBEDDED PDV` or `CHARACTER STRING`.
    fn validate_presentation_context_switching_type_id(
        &self,
        id_el: &X690Element,
    ) -> ASN1Result<()> {
        let invalid_cons = || {
            id_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "identification")
        };
        match id_el.tag.tag_number {
            0 => { // syntaxes
                let subs = id_el.value.components()?;
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
                let subs = id_el.value.components()?;
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

    /// Decode X.690 elements from a slice of bytes
    fn decode_from_slice(&self, bytes: ByteSlice) -> ASN1Result<(usize, X690Element)>;

    /// Decode X.690 elements from a `bytes::Bytes`
    fn decode_from_bytes(&self, bytes: Bytes) -> ASN1Result<(usize, X690Element)> {
        self.decode_from_slice(&bytes[..])
    }

    /// Write an X.690 element
    fn write<W>(&self, output: &mut W, el: &X690Element) -> Result<usize> where W: Write {
        x690_write_tlv(output, el)
    }

    /// Decode a `BOOLEAN` from content octets
    fn decode_boolean_value(&self, value_bytes: ByteSlice) -> ASN1Result<BOOLEAN>;

    /// Decode an `INTEGER` from content octets
    fn decode_integer_value(&self, value_bytes: ByteSlice) -> ASN1Result<INTEGER> {
        if value_bytes.len() == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if value_bytes.len() == 1 {
            return Ok(Vec::from(value_bytes));
        }
        if value_bytes[0] == 0x00 && (value_bytes[1] & 0b1000_0000) == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::int_padding));
        }
        if value_bytes[0] == 0xFF && (value_bytes[1] & 0b1000_0000) > 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::int_padding));
        }
        Ok(Vec::from(value_bytes))
    }

    /// Decode an `ENUMERATED` from content octets
    fn decode_enum_value(&self, value_bytes: ByteSlice) -> ASN1Result<ENUMERATED> {
        match read_i64(value_bytes) {
            Some(v) => Ok(v),
            None => Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
        }
    }

    /// Decode a `BIT STRING` from content octets
    fn decode_bit_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<BIT_STRING>;

    /// Decode an `OCTET STRING` from content octets
    fn decode_octet_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<OCTET_STRING> {
        Ok(Vec::from(value_bytes))
    }

    /// Decode an `OBJECT IDENTIFIER` from content octets
    fn decode_object_identifier_value(&self, value_bytes: ByteSlice) -> ASN1Result<OBJECT_IDENTIFIER> {
        x690_read_object_identifier_value(value_bytes)
    }

    /// Decode a `RELATIVE-OID` from content octets
    fn decode_relative_oid_value(&self, value_bytes: ByteSlice) -> ASN1Result<RELATIVE_OID> {
        RELATIVE_OID::from_x690_encoding_slice(value_bytes)
    }

    /// Decode a `REAL` from content octets
    fn decode_real_value(&self, value_bytes: ByteSlice) -> ASN1Result<REAL>;

    /// Decode a `NumericString` from content octets
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

    /// Decode a `PrintableString` from content octets
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

    /// Decode an `IA5String` from content octets
    fn decode_ia5_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<IA5String> {
        for (i, byte) in value_bytes.iter().enumerate() {
            if *byte > 127 {
                return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                    *byte as u32,
                    i,
                )));
            }
        }
        unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
    }

    /// Decode a `UTCTime` from content octets
    fn decode_utc_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<UTCTime>;

    /// Decode a `GeneralizedTime` from content octets
    fn decode_generalized_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<GeneralizedTime>;

    /// Decode a `GraphicString` from content octets
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

    /// Decode a `VisibleString` / `ISO646String` from content octets
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

    /// Decode a `GeneralString` from content octets
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

    /// Decode a `UniversalString` from content octets
    fn decode_universal_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<UniversalString> {
        if (value_bytes.len() % 4) != 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let char_length = value_bytes.len() / 4;
        let mut ret = Vec::with_capacity(char_length);
        for i in 0..char_length {
            let code_point = u32::from_be_bytes([
                value_bytes[(i * 4) + 0],
                value_bytes[(i * 4) + 1],
                value_bytes[(i * 4) + 2],
                value_bytes[(i * 4) + 3],
            ]);
            ret.push(code_point);
        }
        Ok(UniversalString(ret))
    }

    /// Decode a `BMPString` from content octets
    fn decode_bmp_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<BMPString> {
        if (value_bytes.len() % 2) != 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let char_length = value_bytes.len() / 2;
        let mut ret = Vec::with_capacity(char_length);
        for i in 0..char_length {
            let code_point = u16::from_be_bytes([value_bytes[(i * 2) + 0], value_bytes[(i * 2) + 1]]);
            ret.push(code_point);
        }
        Ok(BMPString(ret))
    }

    /// Decode a `DATE` from content octets
    fn decode_date_value(&self, value_bytes: ByteSlice) -> ASN1Result<DATE> {
        DATE::try_from(value_bytes)
    }

    /// Decode a `TIME-OF-DAY` from content octets
    fn decode_time_of_day_value(&self, value_bytes: ByteSlice) -> ASN1Result<TIME_OF_DAY> {
        TIME_OF_DAY::try_from(value_bytes)
    }

    /// Decode a `DATE-TIME` from content octets
    fn decode_date_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<DATE_TIME> {
        DATE_TIME::try_from(value_bytes)
    }

    /// Decode a `DURATION` from content octets
    fn decode_duration_value(&self, value_bytes: ByteSlice) -> ASN1Result<DURATION>;

    /// Decode a `BOOLEAN` value from an X.690 encoding
    fn decode_boolean(&self, el: &X690Element) -> ASN1Result<BOOLEAN> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_boolean_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_boolean(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode an `INTEGER` value from an X.690 encoding
    fn decode_integer(&self, el: &X690Element) -> ASN1Result<INTEGER> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_integer_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_integer(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode an `ENUMERATED` value from an X.690 encoding
    fn decode_enumerated(&self, el: &X690Element) -> ASN1Result<ENUMERATED> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_enum_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_enumerated(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `BIT STRING` value from an X.690 encoding
    fn decode_bit_string(&self, el: &X690Element) -> ASN1Result<BIT_STRING>;

    /// Decode an `OCTET STRING` value from an X.690 encoding
    fn decode_octet_string(&self, el: &X690Element) -> ASN1Result<OCTET_STRING>;

    /// Decode a `NULL` value from an X.690 encoding
    fn decode_null(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(bytes) => {
                if bytes.len() != 0 {
                    return Err(el.to_asn1_error(ASN1ErrorCode::malformed_value));
                }
                Ok(())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_null(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode an `OBJECT IDENTIFIER` value from an X.690 encoding
    fn decode_object_identifier(&self, el: &X690Element) -> ASN1Result<OBJECT_IDENTIFIER> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_object_identifier_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_object_identifier(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode an `EXTERNAL` value from an X.690 encoding
    fn decode_external(&self, el: &X690Element) -> ASN1Result<EXTERNAL> {
        let elements = el.value.components()?;
        let it = X690StructureIterator::new(
            elements.as_slice(),
            _RCTL1_FOR_EXTERNAL.as_slice(),
            _EAL_FOR_EXTERNAL.as_slice(),
            _RCTL2_FOR_EXTERNAL.as_slice(),
        );
        let mut dir_ref: OPTIONAL<OBJECT_IDENTIFIER> = None;
        let mut indir_ref: OPTIONAL<INTEGER> = None;
        let mut dvd: OPTIONAL<ObjectDescriptor> = None;
        let mut encoding_el: OPTIONAL<&X690Element> = None;
        for (i, fallible_component_name) in it.enumerate() {
            let component_name = fallible_component_name?;
            match component_name {
                "direct-reference" => dir_ref = Some(self.decode_object_identifier(&elements[i])?),
                "indirect-reference" => indir_ref = Some(self.decode_integer(&elements[i])?),
                "data-value-descriptor" => dvd = Some(self.decode_object_descriptor(&elements[i])?),
                "encoding" => encoding_el = Some(&elements[i]),
                _ => { // This type is NOT extensible.
                    let mut err = elements[i].to_asn1_error(ASN1ErrorCode::invalid_construction);
                    err.relate_tag(&elements[i].tag);
                    err.constructed = Some(elements[i].is_constructed());
                    return Err(err);
                },
            }
        }
        let encoding_el = encoding_el
            .expect("X.690 sequence parsing is broken: encoding element of EXTERNAL missing");
        let encoding = match encoding_el.tag.tag_number {
            0 => {
                let components = encoding_el.value.components()?;
                if components.len() != 1 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                let v = self.decode_any(&components[0])?;
                ExternalEncoding::single_ASN1_type(Arc::new(v))
            }
            1 => {
                let v = self.decode_octet_string(encoding_el)?;
                ExternalEncoding::octet_aligned(v)
            }
            2 => {
                let v = self.decode_bit_string(encoding_el)?;
                ExternalEncoding::arbitrary(v)
            }
            _ => return Err(encoding_el.to_asn1_error(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice))
        };

        let identification: ExternalIdentification = match (dir_ref, indir_ref) {
            (Some(d), Some(i)) => ExternalIdentification::context_negotiation(ContextNegotiation::new(i, d)),
            (Some(d), None) => ExternalIdentification::syntax(d),
            (None, Some(i)) => ExternalIdentification::presentation_context_id(i),
            (None, None) => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EXTERNAL.identification")),
        };
        Ok(EXTERNAL {
            identification,
            data_value_descriptor: dvd,
            data_value: encoding,
        })
    }

    /// Decode an `INSTANCE OF` value from an X.690 encoding
    fn decode_instance_of(&self, el: &X690Element) -> ASN1Result<InstanceOf> {
        let elements = el.value.components()?;
        if elements.len() != 2 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "INSTANCE OF"));
        }
        if elements[0].tag.tag_class != TagClass::UNIVERSAL
            || elements[0].tag.tag_number != UNIV_TAG_OBJECT_IDENTIFIER
            || elements[1].tag.tag_class != TagClass::CONTEXT
            || elements[1].tag.tag_number != 0
        {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "INSTANCE OF"));
        }
        let type_id: OBJECT_IDENTIFIER = self.decode_object_identifier(&elements[0])?;
        let value = self.decode_any(&elements[1].inner()?)?;
        Ok(InstanceOf {
            type_id,
            value: Arc::new(value),
        })
    }

    /// Decode an `EMBEDDED PDV` value from an X.690 encoding
    fn decode_embedded_pdv(&self, el: &X690Element) -> ASN1Result<EMBEDDED_PDV> {
        let elements = el.value.components()?;
        if elements.len() != 2 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EMBEDDED PDV"));
        }
        if
            elements[0].tag.tag_class != TagClass::CONTEXT
            || elements[0].tag.tag_number != 0
            || !elements[0].is_constructed()
            || elements[1].tag.tag_class != TagClass::CONTEXT
            || elements[1].tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EMBEDDED PDV"));
        }
        let identification = self.decode_presentation_context_switching_type_id( &elements[0].inner()?)?;
        let data_value: OCTET_STRING = self.decode_octet_string(&elements[1])?;
        Ok(EMBEDDED_PDV {
            identification,
            data_value,
        })
    }

    /// Decode a `CHARACTER STRING` value from an X.690 encoding
    fn decode_character_string(&self, el: &X690Element) -> ASN1Result<CHARACTER_STRING> {
        let elements = el.value.components()?;
        if elements.len() != 2 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CHARACTER STRING"));
        }
        if
            elements[0].tag.tag_class != TagClass::CONTEXT
            || elements[0].tag.tag_number != 0
            || !elements[0].is_constructed()
            || elements[1].tag.tag_class != TagClass::CONTEXT
            || elements[1].tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CHARACTER STRING"));
        }
        let identification = self.decode_presentation_context_switching_type_id(&elements[0].inner()?)?;
        let string_value: OCTET_STRING = self.decode_octet_string(&elements[1])?;
        Ok(CHARACTER_STRING {
            identification,
            string_value,
        })
    }

    /// Decode a `RELATIVE-OID` value from an X.690 encoding
    fn decode_relative_oid(&self, el: &X690Element) -> ASN1Result<RELATIVE_OID> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_relative_oid_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_relative_oid(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `SEQUENCE` value from an X.690 encoding
    fn decode_sequence(&self, el: &X690Element) -> ASN1Result<SEQUENCE>;

    /// Decode a `SET` value from an X.690 encoding
    fn decode_set(&self, el: &X690Element) -> ASN1Result<SET>;

    /// Decode an `ObjectDescriptor` value from an X.690 encoding
    fn decode_object_descriptor(&self, el: &X690Element) -> ASN1Result<ObjectDescriptor>;

    /// Decode a `UTF8String` value from an X.690 encoding
    fn decode_utf8_string(&self, el: &X690Element) -> ASN1Result<UTF8String>;

    /// Decode a `REAL` value from an X.690 encoding
    fn decode_real(&self, el: &X690Element) -> ASN1Result<REAL> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_real_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_real(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `NumericString` value from an X.690 encoding
    fn decode_numeric_string(&self, el: &X690Element) -> ASN1Result<NumericString>;

    /// Decode a `PrintableString` value from an X.690 encoding
    fn decode_printable_string(&self, el: &X690Element) -> ASN1Result<PrintableString>;

    /// Decode a `T61String` / `TeletexString` value from an X.690 encoding
    fn decode_t61_string(&self, el: &X690Element) -> ASN1Result<T61String>;

    /// Decode a `VideotexString` value from an X.690 encoding
    fn decode_videotex_string(&self, el: &X690Element) -> ASN1Result<VideotexString>;

    /// Decode an `IA5String` value from an X.690 encoding
    fn decode_ia5_string(&self, el: &X690Element) -> ASN1Result<IA5String>;

    /// Decode a `UTCTime` value from an X.690 encoding
    fn decode_utc_time(&self, el: &X690Element) -> ASN1Result<UTCTime>;

    /// Decode a `GeneralizedTime` value from an X.690 encoding
    fn decode_generalized_time(&self, el: &X690Element) -> ASN1Result<GeneralizedTime>;

    /// Decode a `GraphicString` value from an X.690 encoding
    fn decode_graphic_string(&self, el: &X690Element) -> ASN1Result<GraphicString>;

    /// Decode a `VisibleString` / `ISO646String` value from an X.690 encoding
    fn decode_visible_string(&self, el: &X690Element) -> ASN1Result<VisibleString>;

    /// Decode a `GeneralString` value from an X.690 encoding
    fn decode_general_string(&self, el: &X690Element) -> ASN1Result<GeneralString>;

    /// Decode a `UniversalString` value from an X.690 encoding
    fn decode_universal_string(&self, el: &X690Element) -> ASN1Result<UniversalString>;

    /// Decode a `BMPString` value from an X.690 encoding
    fn decode_bmp_string(&self, el: &X690Element) -> ASN1Result<BMPString>;

    /// Decode a `DATE` value from an X.690 encoding
    fn decode_date(&self, el: &X690Element) -> ASN1Result<DATE> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_date_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_date(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `TIME-OF-DAY` value from an X.690 encoding
    fn decode_time_of_day(&self, el: &X690Element) -> ASN1Result<TIME_OF_DAY> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_time_of_day_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_time_of_day(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `DATE-TIME` value from an X.690 encoding
    fn decode_date_time(&self, el: &X690Element) -> ASN1Result<DATE_TIME> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_date_time_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_date_time(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `DURATION` value from an X.690 encoding
    fn decode_duration(&self, el: &X690Element) -> ASN1Result<DURATION> {
        match &el.value {
            X690Value::Primitive(bytes) => self.decode_duration_value(bytes)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_duration(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode an `OID-IRI` value from an X.690 encoding
    fn decode_oid_iri(&self, el: &X690Element) -> ASN1Result<OID_IRI> {
        match &el.value {
            X690Value::Primitive(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(x) => Ok(x),
                Err(e) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_utf8(Some(e.utf8_error())))),
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_oid_iri(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `RELATIVE-OID-IRI` value from an X.690 encoding
    fn decode_relative_oid_iri(&self, el: &X690Element) -> ASN1Result<OID_IRI> {
        match &el.value {
            X690Value::Primitive(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(x) => Ok(x),
                Err(e) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_utf8(Some(e.utf8_error())))),
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_relative_oid_iri(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode a `TIME` value from an X.690 encoding
    fn decode_time(&self, el: &X690Element) -> ASN1Result<TIME> {
        match &el.value {
            X690Value::Primitive(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(x) => Ok(x),
                Err(e) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_utf8(Some(e.utf8_error())))),
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.decode_time(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Decode any ASN.1 value from an X.690 encoding
    fn decode_any(&self, el: &X690Element) -> ASN1Result<ASN1Value>;

    /// Encode any ASN.1 value into an X.690 encoding
    fn encode_any(&self, value: &ASN1Value) -> ASN1Result<X690Element>;

    /// Encode a `BOOLEAN` value into an X.690 encoding
    fn encode_boolean(&self, value: &BOOLEAN) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BOOLEAN),
            X690Value::Primitive(Bytes::from(if *value { vec![0xFFu8] } else { vec![0x00u8] })),
        ))
    }

    /// Encode an `INTEGER` value into an X.690 encoding
    fn encode_integer(&self, value: &INTEGER) -> ASN1Result<X690Element> {
        let mut out = BytesMut::new().writer();
        x690_write_integer_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode an `ENUMERATED` value into an X.690 encoding
    fn encode_enumerated(&self, value: &ENUMERATED) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(2).writer(); // Most enums are small.
        x690_write_enum_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_ENUMERATED),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode a `BIT STRING` value into an X.690 encoding
    fn encode_bit_string(&self, value: &BIT_STRING) -> ASN1Result<X690Element>;

    /// Encode an `OCTET STRING` value into an X.690 encoding
    fn encode_octet_string(&self, value: &OCTET_STRING) -> ASN1Result<X690Element>;

    /// Encode a `NULL` value into an X.690 encoding
    fn encode_null(&self, _value: &NULL) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NULL),
            X690Value::Primitive(Bytes::new()),
        ))
    }

    /// Encode an `OBJECT IDENTIFIER` value into an X.690 encoding
    fn encode_object_identifier(&self, value: &OBJECT_IDENTIFIER) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.as_x690_slice().len()).writer();
        x690_write_object_identifier_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode an `EXTERNAL` value into an X.690 encoding
    fn encode_external(&self, value: &EXTERNAL) -> ASN1Result<X690Element> {
        let components = x690_encode_external_components(value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_EXTERNAL),
            X690Value::Constructed(Arc::new(components)),
        ))
    }

    /// Encode an `INSTANCE OF` value into an X.690 encoding
    fn encode_instance_of(&self, value: &INSTANCE_OF) -> ASN1Result<X690Element> {
        let external = EXTERNAL {
            identification: ExternalIdentification::syntax(value.type_id.clone()),
            data_value_descriptor: None,
            data_value: ExternalEncoding::single_ASN1_type(value.value.clone()),
        };
        let components = x690_encode_external_components(&external)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_EXTERNAL),
            X690Value::Constructed(Arc::new(components)),
        ))
    }

    /// Encode a `REAL` value into an X.690 encoding
    fn encode_real(&self, value: &REAL) -> ASN1Result<X690Element>;

    /// Encode an `EMBEDDED PDV` value into an X.690 encoding
    fn encode_embedded_pdv(&self, value: &EMBEDDED_PDV) -> ASN1Result<X690Element> {
        let components = x690_encode_embedded_pdv_components(value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_EMBEDDED_PDV),
            X690Value::Constructed(Arc::new(components)),
        ))
    }

    /// Encode a `CHARACTER STRING` value into an X.690 encoding
    fn encode_character_string(&self, value: &CHARACTER_STRING) -> ASN1Result<X690Element> {
        let components = x690_encode_character_string_components(value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_CHARACTER_STRING),
            X690Value::Constructed(Arc::new(components)),
        ))
    }

    /// Encode a `RELATIVE-OID` value into an X.690 encoding
    fn encode_relative_oid(&self, value: &RELATIVE_OID) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.as_x690_slice().len()).writer();
        x690_write_relative_oid_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_RELATIVE_OID),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode an `ObjectDescriptor` value into an X.690 encoding
    fn encode_object_descriptor(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `UTF8String` value into an X.690 encoding
    fn encode_utf8_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `NumericString` value into an X.690 encoding
    fn encode_numeric_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `PrintableString` value into an X.690 encoding
    fn encode_printable_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `T61String` / `TeletexString` value into an X.690 encoding
    fn encode_t61_string(&self, value: &T61String) -> ASN1Result<X690Element>;

    /// Encode a `VideotexString` value into an X.690 encoding
    fn encode_videotex_string(&self, value: &VideotexString) -> ASN1Result<X690Element>;

    /// Encode an `IA5String` value into an X.690 encoding
    fn encode_ia5_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `UTCTime` value into an X.690 encoding
    fn encode_utc_time(&self, value: &UTCTime) -> ASN1Result<X690Element>;

    /// Encode a `GeneralizedTime` value into an X.690 encoding
    fn encode_generalized_time(&self, value: &GeneralizedTime) -> ASN1Result<X690Element>;

    /// Encode a `GraphicString` value into an X.690 encoding
    fn encode_graphic_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `VisibleString` / `ISO646String` value into an X.690 encoding
    fn encode_visible_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode a `GeneralString` value into an X.690 encoding
    fn encode_general_string(&self, value: &str) -> ASN1Result<X690Element>;

    /// Encode an owned `UniversalString` value into an X.690 encoding
    fn encode_owned_universal_string(&self, value: UniversalString) -> ASN1Result<X690Element>;

    /// Encode an owned `BMPString` value into an X.690 encoding
    fn encode_owned_bmp_string(&self, value: BMPString) -> ASN1Result<X690Element>;

    /// Encode a `UniversalString` value into an X.690 encoding
    fn encode_universal_string<T: AsRef<[u32]>>(&self, value: &T) -> ASN1Result<X690Element>;

    /// Encode a `BMPString` value into an X.690 encoding
    fn encode_bmp_string<T: AsRef<[u16]>>(&self, value: &T) -> ASN1Result<X690Element>;

    /// Encode a `DATE` value into an X.690 encoding
    fn encode_date(&self, value: &DATE) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(8).writer(); // YYYYMMDD
        x690_write_date_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_DATE),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode a `TIME-OF-DAY` value into an X.690 encoding
    fn encode_time_of_day(&self, value: &TIME_OF_DAY) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(6).writer(); // HHMMSS
        x690_write_time_of_day_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_TIME_OF_DAY),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode a `DATE-TIME` value into an X.690 encoding
    fn encode_date_time(&self, value: &DATE_TIME) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(14).writer(); // 19511014153000
        x690_write_date_time_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_DATE_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode a `DURATION` value into an X.690 encoding
    fn encode_duration(&self, value: &DURATION) -> ASN1Result<X690Element>;

    /// Encode an `OID-IRI` value into an X.690 encoding
    fn encode_oid_iri(&self, value: &OID_IRI) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OID_IRI),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode a `RELATIVE-OID-IRI` value into an X.690 encoding
    fn encode_relative_oid_iri(&self, value: &OID_IRI) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_RELATIVE_OID_IRI),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Encode a `TIME` value into an X.690 encoding
    fn encode_time(&self, value: &TIME) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_time_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// Validate an encoded `BOOLEAN` value from content octets
    fn validate_boolean_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;

    /// Validate an encoded `INTEGER` value from content octets
    fn validate_integer_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
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

    /// Validate an encoded `TIME` value from content octets
    fn validate_time_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        let maybe_bad_char = content_octets.iter().position(|b| !wildboar_asn1::is_tstring_char(*b));
        if let Some(bad_char_index) = maybe_bad_char {
            let bad_char = content_octets[bad_char_index];
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                bad_char as u32,
                bad_char_index,
            )));
        }
        Ok(())
    }

    /// Validate an encoded `NumericString` value from content octets
    fn validate_numeric_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        let maybe_bad_char = content_octets.iter().position(|b| !wildboar_asn1::is_numeric_char(*b));
        if let Some(bad_char_index) = maybe_bad_char {
            let bad_char = content_octets[bad_char_index];
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                bad_char as u32,
                bad_char_index,
            )));
        }
        Ok(())
    }

    /// Validate an encoded `PrintableString` value from content octets
    fn validate_printable_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
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

    /// Validate an encoded `T61String` / `TeletexString` value from content octets
    #[inline]
    fn validate_t61_string_value (&self, _content_octets: ByteSlice) -> ASN1Result<()> {
        Ok(())
    }

    /// Validate an encoded `VideotexString` value from content octets
    #[inline]
    fn validate_videotex_string_value (&self, _content_octets: ByteSlice) -> ASN1Result<()> {
        Ok(())
    }

    /// Validate an encoded `IA5String` value from content octets
    fn validate_ia5_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
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

    /// Validate an encoded `UTF8String` value from content octets
    #[inline]
    fn validate_utf8_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        #[cfg(feature = "simdutf8")]
        return from_utf8(content_octets)
            .map(|_| ())
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8(None)));
        #[cfg(not(feature = "simdutf8"))]
        std::str::from_utf8(content_octets)
            .map(|_| ())
            .map_err(|e| ASN1Error::new(ASN1ErrorCode::invalid_utf8(Some(e))))
    }

    /// Validate an encoded `BIT STRING` value from content octets
    fn validate_bit_string_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;

    /// Validate an encoded `OCTET STRING` value from content octets
    #[inline]
    fn validate_octet_string_value (&self, _content_octets: ByteSlice) -> ASN1Result<()> {
        Ok(())
    }

    /// Validate an encoded `NULL` value from content octets
    #[inline]
    fn validate_null_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() == 0 {
            Ok(())
        } else {
            Err(ASN1Error::new(ASN1ErrorCode::malformed_value))
        }
    }

    /// Validate an encoded `OBJECT IDENTIFIER` value from content octets
    #[inline]
    fn validate_object_identifier_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        OBJECT_IDENTIFIER::validate_x690_encoding(content_octets)
    }

    /// Validate an encoded `ObjectDescriptor` value from content octets
    #[inline]
    fn validate_object_descriptor_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        self.validate_graphic_string_value(content_octets)
    }

    /// Validate an encoded `REAL` value from content octets
    fn validate_real_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;

    /// Validate an encoded `ENUMERATED` value from content octets
    #[inline]
    fn validate_enumerated_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        self.validate_integer_value(content_octets)
    }

    /// Validate an encoded `RELATIVE-OID` value from content octets
    #[inline]
    fn validate_relative_object_identifier_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        RELATIVE_OID::validate_x690_encoding(content_octets)
    }

    /// Validate an encoded `UTCTime` value from content octets
    fn validate_utc_time_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;

    /// Validate an encoded `GeneralizedTime` value from content octets
    fn validate_generalized_time_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;

    /// Validate an encoded `GraphicString` value from content octets
    #[inline]
    fn validate_graphic_string_value (&self, _: ByteSlice) -> ASN1Result<()> {
        Ok(())
    }

    /// Validate an encoded `VisibleString` value from content octets
    fn validate_visible_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        let maybe_bad_char = content_octets.iter().position(|b| !wildboar_asn1::is_visible_char(*b));
        if let Some(bad_char_index) = maybe_bad_char {
            let bad_char = content_octets[bad_char_index];
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                bad_char as u32,
                bad_char_index,
            )));
        }
        Ok(())
    }

    /// Validate an encoded `GeneralString` value from content octets
    ///
    /// There's no validation here, because a `GeneralString` can be basically
    /// anything.
    #[inline]
    fn validate_general_string_value (&self, _: ByteSlice) -> ASN1Result<()> {
        Ok(())
    }

    /// Validate an encoded `UniversalString` value from content octets
    fn validate_universal_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() % 4 > 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        // Theoretically, you could validate that every uint32 is a valid Unicode
        // code point as well, but that might be overkill.
        Ok(())
    }

    /// Validate an encoded `BMPString` value from content octets
    fn validate_bmp_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() % 2 > 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        Ok(())
    }

    /// Validate an encoded `DATE` value from content octets
    #[inline]
    fn validate_date_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        DATE::validate_x690_encoding(content_octets)
    }

    /// Validate an encoded `TIME-OF-DAY` value from content octets
    #[inline]
    fn validate_time_of_day_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        TIME_OF_DAY::validate_x690_encoding(content_octets)
    }

    /// Validate an encoded `DATE-TIME` value from content octets
    fn validate_date_time_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() != 14 { // 19511014153000 (X.690 strips the hyphens, colon and "T")
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        self.validate_date_value(&content_octets[0..8])?;
        self.validate_time_of_day_value(&content_octets[8..])
    }

    /// Validate an encoded `DURATION` value from content octets
    fn validate_duration_value(&self, content_octets: ByteSlice) -> ASN1Result<()>;

    /// Validate an encoded `OID-IRI` value from content octets
    fn validate_oid_iri_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() < 2 || content_octets[0] != b'/' {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        Ok(())
    }

    /// Validate an encoded `RELATIVE-OID-IRI` value from content octets
    #[inline]
    fn validate_relative_oid_iri_value (&self, _content_octets: ByteSlice) -> ASN1Result<()> {
        Ok(())
    }

    /// Validate an encoded `BOOLEAN`
    fn validate_boolean(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_boolean_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_boolean(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `INTEGER`
    fn validate_integer(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_integer_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_integer(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `BIT STRING`
    fn validate_bit_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `OCTET STRING`
    fn validate_octet_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `NULL`
    fn validate_null(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_null_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_null(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `OBJECT IDENTIFIER`
    fn validate_object_identifier(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_object_identifier_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_object_identifier(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `ObjectDescriptor`
    fn validate_object_descriptor(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `REAL`
    fn validate_real(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_real_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_real(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `ENUMERATED`
    fn validate_enumerated(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_enumerated_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_enumerated(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `UTF8String`
    fn validate_utf8_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `RELATIVE-OID`
    fn validate_relative_object_identifier(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_relative_object_identifier_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_relative_object_identifier(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `TIME`
    fn validate_time(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_time_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_time(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `NumericString`
    fn validate_numeric_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `PrintableString`
    fn validate_printable_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `T61String` / `TeletexString`
    fn validate_t61_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `VideotexString`
    fn validate_videotex_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `IA5String`
    fn validate_ia5_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `UTCTime`
    fn validate_utc_time(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `GeneralizedTime`
    fn validate_generalized_time(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `GraphicString`
    fn validate_graphic_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `VisibleString` / `ISO646String`
    fn validate_visible_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `GeneralString`
    fn validate_general_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `UniversalString`
    fn validate_universal_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `BMPString`
    fn validate_bmp_string(&self, el: &X690Element) -> ASN1Result<()>;

    /// Validate an encoded `DATE`
    fn validate_date(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_date_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_date(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `TIME-OF-DAY`
    fn validate_time_of_day(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_time_of_day_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_time_of_day(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `DATE-TIME`
    fn validate_date_time(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_date_time_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_date_time(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `DURATION`
    fn validate_duration(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_duration_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_duration(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `OID-IRI`
    fn validate_oid_iri(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_oid_iri_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_oid_iri(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `RELATIVE-OID-IRI`
    fn validate_relative_oid_iri(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_relative_oid_iri_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_relative_oid_iri(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Validate an encoded `EXTERNAL`
    fn validate_external(&self, el: &X690Element) -> ASN1Result<()> {
        let components = el.value.components()?;
        let len = components.len();
        if len > 4 || len == 0 {
            return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction));
        }
        let last_el = &components[len - 1];
        if last_el.tag.tag_class != TagClass::CONTEXT {
            return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction));
        }
        match last_el.tag.tag_number {
            0 => self.validate_any(&last_el.inner()?)?,
            1 => self.validate_octet_string(&last_el)?,
            2 => self.validate_bit_string(&last_el)?,
            _ => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        };
        let mut s = 0; // component spec index.
        let mut int_seen: bool = false;
        let mut desc_seen: bool = false;
        for component in components[0..len - 1].iter() {
            if component.tag.tag_class != TagClass::UNIVERSAL {
                return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction));
            }
            if
                s == 0
                && component.tag.tag_number == UNIV_TAG_OBJECT_IDENTIFIER {
                self.validate_object_identifier(component)?;
            }
            else if s <= 1
                && !int_seen
                && component.tag.tag_number == UNIV_TAG_INTEGER {
                self.validate_integer(component)?;
                int_seen = true;
            }
            else if s <= 2
                && !desc_seen
                && component.tag.tag_number == UNIV_TAG_OBJECT_DESCRIPTOR {
                self.validate_object_descriptor(component)?;
                desc_seen = true;
            }
            else {
                return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction));
            }
            s += 1;
        }
        Ok(())
    }

    /// Validate an encoded `EMBEDDED PDV`
    fn validate_embedded_pdv(&self, el: &X690Element) -> ASN1Result<()> {
        // you can use the same code for parsing the
        let children = el.value.components()?;
        let it = X690StructureIterator::new(
            children.as_slice(),
            wildboar_asn1::_rctl1_components_for_EMBEDDED_PDV,
            wildboar_asn1::_eal_components_for_EMBEDDED_PDV,
            wildboar_asn1::_rctl2_components_for_EMBEDDED_PDV,
        );
        for (i, fallible_component_name) in it.enumerate() {
            let component_name = fallible_component_name?;
            match component_name {
                "identification" => self.validate_presentation_context_switching_type_id(&children[i])?,
                "data-value-descriptor" => return Err(el.to_asn1_error(ASN1ErrorCode::constraint_violation)),
                "data-value" => self.validate_octet_string(&children[i])?,
                _ => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)), // This type is NOT extensible.
            }
        }
        Ok(())
    }

    /// Validate an encoded `CHARACTER STRING`
    fn validate_character_string(&self, el: &X690Element) -> ASN1Result<()> {
        // you can use the same code for parsing the
        let children: Arc<Vec<X690Element>> = el.components()?;
        let it = X690StructureIterator::new(
            children.as_slice(),
            wildboar_asn1::_rctl1_components_for_CharacterString,
            wildboar_asn1::_eal_components_for_CharacterString,
            wildboar_asn1::_rctl2_components_for_CharacterString
        );
        for (i, fallible_component_name) in it.enumerate() {
            let component_name = fallible_component_name?;
            match component_name {
                "identification" => self.validate_presentation_context_switching_type_id(&children[i])?,
                "string-value" => self.validate_octet_string(&children[i])?,
                _ => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)), // This type is NOT extensible.
            }
        }
        Ok(())
    }

    /// Validate any encoded ASN.1 value
    fn validate_any (&self, el: &X690Element) -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::UNIVERSAL {
            match &el.value {
                X690Value::Primitive(_) => return Ok(()), // IMPLICIT tagged unknown type.
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        self.validate_any(sub)?;
                    }
                },
                X690Value::Serialized(v) => {
                    let (_, el) = BER.decode_from_slice(&v)?;
                    self.validate_any(&el)?;
                }
            };
            return Ok(());
        }
        let result = match el.tag.tag_number {
            UNIV_TAG_END_OF_CONTENT => self.validate_null(el),
            UNIV_TAG_BOOLEAN => self.validate_boolean(el),
            UNIV_TAG_INTEGER => self.validate_integer(el),
            UNIV_TAG_BIT_STRING => self.validate_bit_string(el),
            UNIV_TAG_OCTET_STRING => self.validate_octet_string(el),
            UNIV_TAG_NULL => self.validate_null(el),
            UNIV_TAG_OBJECT_IDENTIFIER => self.validate_object_identifier(el),
            UNIV_TAG_OBJECT_DESCRIPTOR => self.validate_object_descriptor(el),
            UNIV_TAG_EXTERNAL => self.validate_external(el),
            UNIV_TAG_REAL => self.validate_real(el),
            UNIV_TAG_ENUMERATED => self.validate_enumerated(el),
            UNIV_TAG_EMBEDDED_PDV => self.validate_embedded_pdv(el),
            UNIV_TAG_UTF8_STRING => self.validate_utf8_string(el),
            UNIV_TAG_RELATIVE_OID => self.validate_relative_object_identifier(el),
            UNIV_TAG_TIME => self.validate_time(el),
            UNIV_TAG_SEQUENCE | UNIV_TAG_SET => {
                // NOTE: You can't check for duplicate tags in a SET because it could actually be a SET OF.
                let subs = el.value.components()?;
                for sub in subs.iter() {
                    self.validate_any(sub)?;
                }
                Ok(())
            },
            UNIV_TAG_NUMERIC_STRING => self.validate_numeric_string(el),
            UNIV_TAG_PRINTABLE_STRING => self.validate_printable_string(el),
            UNIV_TAG_T61_STRING => self.validate_t61_string(el),
            UNIV_TAG_VIDEOTEX_STRING => self.validate_videotex_string(el),
            UNIV_TAG_IA5_STRING => self.validate_ia5_string(el),
            UNIV_TAG_UTC_TIME => self.validate_utc_time(el),
            UNIV_TAG_GENERALIZED_TIME => self.validate_generalized_time(el),
            UNIV_TAG_GRAPHIC_STRING => self.validate_graphic_string(el),
            UNIV_TAG_VISIBLE_STRING => self.validate_visible_string(el),
            UNIV_TAG_GENERAL_STRING => self.validate_general_string(el),
            UNIV_TAG_UNIVERSAL_STRING => self.validate_universal_string(el),
            UNIV_TAG_CHARACTER_STRING => self.validate_character_string(el),
            UNIV_TAG_BMP_STRING => self.validate_bmp_string(el),
            UNIV_TAG_DATE => self.validate_date(el),
            UNIV_TAG_TIME_OF_DAY => self.validate_time_of_day(el),
            UNIV_TAG_DATE_TIME => self.validate_date_time(el),
            UNIV_TAG_DURATION => self.validate_duration(el),
            UNIV_TAG_OID_IRI => self.validate_oid_iri(el),
            UNIV_TAG_RELATIVE_OID_IRI => self.validate_relative_oid_iri(el),
            _ => Ok(()), // It's hard to say what to do here. Accepting is future-proof, but the value could be invalid.
        };
        result.map_err(|e| el.to_asn1_error(e.error_code))
    }

    /// Write a `BOOLEAN` value as an X.690 encoding to a writable stream
    fn write_boolean<W>(&self, output: &mut W, value: &BOOLEAN) -> Result<usize> where W: Write {
        let enc = self.encode_boolean(value)?;
        self.write(output, &enc)
    }

    /// Write an `INTEGER` value as an X.690 encoding to a writable stream
    fn write_integer<W>(&self, output: &mut W, value: &INTEGER) -> Result<usize> where W: Write {
        let enc = self.encode_integer(value)?;
        self.write(output, &enc)
    }

    /// Write an `ENUMERATED` value as an X.690 encoding to a writable stream
    fn write_enumerated<W>(&self, output: &mut W, value: &ENUMERATED) -> Result<usize> where W: Write {
        let enc = self.encode_enumerated(value)?;
        self.write(output, &enc)
    }

    /// Write a `BIT STRING` value as an X.690 encoding to a writable stream
    fn write_bit_string<W>(&self, output: &mut W, value: &BIT_STRING) -> Result<usize> where W: Write {
        let enc = self.encode_bit_string(value)?;
        self.write(output, &enc)
    }

    /// Write an `OCTET STRING` value as an X.690 encoding to a writable stream
    fn write_octet_string<W>(&self, output: &mut W, value: &OCTET_STRING) -> Result<usize> where W: Write {
        let enc = self.encode_octet_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `NULL` value as an X.690 encoding to a writable stream
    fn write_null<W>(&self, output: &mut W, value: &NULL) -> Result<usize> where W: Write {
        let enc = self.encode_null(value)?;
        self.write(output, &enc)
    }

    /// Write an `OBJECT IDENTIFIER` value as an X.690 encoding to a writable stream
    fn write_object_identifier<W>(&self, output: &mut W, value: &OBJECT_IDENTIFIER) -> Result<usize> where W: Write {
        let enc = self.encode_object_identifier(value)?;
        self.write(output, &enc)
    }

    /// Write an `EXTERNAL` value as an X.690 encoding to a writable stream
    fn write_external<W>(&self, output: &mut W, value: &EXTERNAL) -> Result<usize> where W: Write {
        let enc = self.encode_external(value)?;
        self.write(output, &enc)
    }

    /// Write an `INSTANCE OF` value as an X.690 encoding to a writable stream
    fn write_instance_of<W>(&self, output: &mut W, value: &INSTANCE_OF) -> Result<usize> where W: Write {
        let enc = self.encode_instance_of(value)?;
        self.write(output, &enc)
    }

    /// Write a `REAL` value as an X.690 encoding to a writable stream
    fn write_real<W>(&self, output: &mut W, value: &REAL) -> Result<usize> where W: Write {
        let enc = self.encode_real(value)?;
        self.write(output, &enc)
    }

    /// Write an `EMBEDDED PDV` value as an X.690 encoding to a writable stream
    fn write_embedded_pdv<W>(&self, output: &mut W, value: &EMBEDDED_PDV) -> Result<usize> where W: Write {
        let enc = self.encode_embedded_pdv(value)?;
        self.write(output, &enc)
    }

    /// Write a `CHARACTER STRING` value as an X.690 encoding to a writable stream
    fn write_character_string<W>(&self, output: &mut W, value: &CHARACTER_STRING) -> Result<usize> where W: Write {
        let enc = self.encode_character_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `RELATIVE-OID` value as an X.690 encoding to a writable stream
    fn write_relative_oid<W>(&self, output: &mut W, value: &RELATIVE_OID) -> Result<usize> where W: Write {
        let enc = self.encode_relative_oid(value)?;
        self.write(output, &enc)
    }

    /// Write an `ObjectDescriptor` value as an X.690 encoding to a writable stream
    fn write_object_descriptor<W>(&self, output: &mut W, value: &ObjectDescriptor) -> Result<usize> where W: Write {
        let enc = self.encode_object_descriptor(value)?;
        self.write(output, &enc)
    }

    /// Write a `UTF8String` value as an X.690 encoding to a writable stream
    fn write_utf8_string<W>(&self, output: &mut W, value: &str) -> Result<usize> where W: Write {
        let enc = self.encode_utf8_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `NumericString` value as an X.690 encoding to a writable stream
    fn write_numeric_string<W>(&self, output: &mut W, value: &str) -> Result<usize> where W: Write {
        let enc = self.encode_numeric_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `PrintableString` value as an X.690 encoding to a writable stream
    fn write_printable_string<W>(&self, output: &mut W, value: &str) -> Result<usize> where W: Write {
        let enc = self.encode_printable_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `T61String` / `TeletexString` value as an X.690 encoding to a writable stream
    fn write_t61_string<W>(&self, output: &mut W, value: &T61String) -> Result<usize> where W: Write {
        let enc = self.encode_t61_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `VideotexString` value as an X.690 encoding to a writable stream
    fn write_videotex_string<W>(&self, output: &mut W, value: &VideotexString) -> Result<usize> where W: Write {
        let enc = self.encode_videotex_string(value)?;
        self.write(output, &enc)
    }

    /// Write an `IA5String` value as an X.690 encoding to a writable stream
    fn write_ia5_string<W>(&self, output: &mut W, value: &IA5String) -> Result<usize> where W: Write {
        let enc = self.encode_ia5_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `UTCTime` value as an X.690 encoding to a writable stream
    fn write_utc_time<W>(&self, output: &mut W, value: &UTCTime) -> Result<usize> where W: Write {
        let enc = self.encode_utc_time(value)?;
        self.write(output, &enc)
    }

    /// Write a `GeneralizedTime` value as an X.690 encoding to a writable stream
    fn write_generalized_time<W>(&self, output: &mut W, value: &GeneralizedTime) -> Result<usize> where W: Write {
        let enc = self.encode_generalized_time(value)?;
        self.write(output, &enc)
    }

    /// Write a `GraphicString` value as an X.690 encoding to a writable stream
    fn write_graphic_string<W>(&self, output: &mut W, value: &GraphicString) -> Result<usize> where W: Write {
        let enc = self.encode_graphic_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `VisibleString` / `ISO646String` value as an X.690 encoding to a writable stream
    fn write_visible_string<W>(&self, output: &mut W, value: &VisibleString) -> Result<usize> where W: Write {
        let enc = self.encode_visible_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `GeneralString` value as an X.690 encoding to a writable stream
    fn write_general_string<W>(&self, output: &mut W, value: &GeneralString) -> Result<usize> where W: Write {
        let enc = self.encode_general_string(value)?;
        self.write(output, &enc)
    }

    /// Write a `UniversalString` value as an X.690 encoding to a writable stream
    fn write_universal_string<W>(&self, output: &mut W, value: &UniversalString) -> Result<usize> where W: Write {
        let enc = self.encode_universal_string(&value.0)?;
        self.write(output, &enc)
    }

    /// Write a `BMPString` value as an X.690 encoding to a writable stream
    fn write_bmp_string<W>(&self, output: &mut W, value: &BMPString) -> Result<usize> where W: Write {
        let enc = self.encode_bmp_string(&value.0)?;
        self.write(output, &enc)
    }

    /// Write a `DATE` value as an X.690 encoding to a writable stream
    fn write_date<W>(&self, output: &mut W, value: &DATE) -> Result<usize> where W: Write {
        let enc = self.encode_date(value)?;
        self.write(output, &enc)
    }

    /// Write a `TIME-OF-DAY` value as an X.690 encoding to a writable stream
    fn write_time_of_day<W>(&self, output: &mut W, value: &TIME_OF_DAY) -> Result<usize> where W: Write {
        let enc = self.encode_time_of_day(value)?;
        self.write(output, &enc)
    }

    /// Write a `DATE-TIME` value as an X.690 encoding to a writable stream
    fn write_date_time<W>(&self, output: &mut W, value: &DATE_TIME) -> Result<usize> where W: Write {
        let enc = self.encode_date_time(value)?;
        self.write(output, &enc)
    }

    /// Write a `DURATION` value as an X.690 encoding to a writable stream
    fn write_duration<W>(&self, output: &mut W, value: &DURATION) -> Result<usize> where W: Write {
        let enc = self.encode_duration(value)?;
        self.write(output, &enc)
    }

    /// Write an `OID-IRI` value as an X.690 encoding to a writable stream
    fn write_oid_iri<W>(&self, output: &mut W, value: &OID_IRI) -> Result<usize> where W: Write {
        let enc = self.encode_oid_iri(value)?;
        self.write(output, &enc)
    }

    /// Write a `RELATIVE-OID-IRI` value as an X.690 encoding to a writable stream
    fn write_relative_oid_iri<W>(&self, output: &mut W, value: &OID_IRI) -> Result<usize> where W: Write {
        let enc = self.encode_relative_oid_iri(value)?;
        self.write(output, &enc)
    }

    /// Write a `TIME` value as an X.690 encoding to a writable stream
    fn write_time<W>(&self, output: &mut W, value: &TIME) -> Result<usize> where W: Write {
        let enc = self.encode_time(value)?;
        self.write(output, &enc)
    }

    /// Encode an `i8` value as an X.690 encoding of an `INTEGER`
    fn encode_i8 (&self, value: i8) -> ASN1Result<X690Element> {
        let mut content = BytesMut::with_capacity(1);
        content.put_i8(value);
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
            X690Value::Primitive(content.into()),
        ))
    }

    /// Decode an `i8` from an X.690-encoded `INTEGER`
    fn decode_i8 (&self, el: &X690Element) -> ASN1Result<i8> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `i8`
    fn validate_i8 (&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(x) => if x.len() == 1 {
                Ok(())
            } else if x.len() == 0 {
                Err(el.to_asn1_error(ASN1ErrorCode::value_too_short))
            } else {
                Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
            },
            X690Value::Constructed(_) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_i8(&el)
            }
        }
    }

    /// Encode a `u8` value as an X.690 encoding of an `INTEGER`
    fn encode_u8 (&self, value: u8) -> ASN1Result<X690Element> {
        self.encode_i16(value.into())
    }

    /// Decode a `u8` from an X.690-encoded `INTEGER`
    fn decode_u8 (&self, el: &X690Element) -> ASN1Result<u8> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `u8`
    fn validate_u8 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u8 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    /// Encode an `i16` value as an X.690 encoding of an `INTEGER`
    fn encode_i16 (&self, value: i16) -> ASN1Result<X690Element> {
        let possible_i8 = value.try_into();
        match possible_i8 {
            Ok(pi8) => self.encode_i8(pi8),
            Err(_) => {
                let mut content = BytesMut::with_capacity(2);
                content.put_i16(value);
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
                    X690Value::Primitive(content.into()),
                ))
            },
        }
    }

    /// Decode an `i16` from an X.690-encoded `INTEGER`
    fn decode_i16 (&self, el: &X690Element) -> ASN1Result<i16> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `i16`
    fn validate_i16 (&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => {
                if v.len() > 2 {
                    return Err(el.to_asn1_error(ASN1ErrorCode::constraint_violation));
                }
                self.validate_integer_value(&v)
            },
            X690Value::Constructed(_) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_i16(&el)
            }
        }
    }

    /// Encode a `u16` value as an X.690 encoding of an `INTEGER`
    fn encode_u16 (&self, value: u16) -> ASN1Result<X690Element> {
        self.encode_i32(value.into())
    }

    /// Decode a `u16` from an X.690-encoded `INTEGER`
    fn decode_u16 (&self, el: &X690Element) -> ASN1Result<u16> {
        let int_bytes = match &el.value {
            X690Value::Primitive(v) => v,
            X690Value::Constructed(_) => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                return self.decode_u16(&el)
            }
        };
        if int_bytes.len() == size_of::<u16>() + 1 && int_bytes[0] == 0 {
            // It's still a u16, but it starts with a set bit.
            return Ok(u16::from_be_bytes([
                int_bytes[1],
                int_bytes[2],
            ]));
        }
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `u16`
    fn validate_u16 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u16 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    /// Encode an `i32` value as an X.690 encoding of an `INTEGER`
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
                        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
                        X690Value::Primitive(content.into()),
                    ))
                } else {
                    let mut content = BytesMut::with_capacity(4);
                    content.put_i32(value);
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
                        X690Value::Primitive(content.into()),
                    ))
                }
            },
        }
    }

    /// Decode an `i32` from an X.690-encoded `INTEGER`
    fn decode_i32 (&self, el: &X690Element) -> ASN1Result<i32> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `i32`
    fn validate_i32 (&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => {
                if v.len() > 4 {
                    return Err(el.to_asn1_error(ASN1ErrorCode::constraint_violation));
                }
                self.validate_integer_value(&v)
            },
            X690Value::Constructed(_) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_i32(&el)
            }
        }
    }

    /// Encode a `u32` value as an X.690 encoding of an `INTEGER`
    fn encode_u32 (&self, value: u32) -> ASN1Result<X690Element> {
        self.encode_i64(value.into())
    }

    /// Decode a `u32`
    fn decode_u32 (&self, el: &X690Element) -> ASN1Result<u32> {
        let int_bytes = match &el.value {
            X690Value::Primitive(v) => v,
            X690Value::Constructed(_) => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                return self.decode_u32(&el)
            }
        };
        if int_bytes.len() == size_of::<u32>() + 1 && int_bytes[0] == 0 {
            // It's still a u32, but it starts with a set bit.
            return Ok(u32::from_be_bytes([
                int_bytes[1],
                int_bytes[2],
                int_bytes[3],
                int_bytes[4],
            ]));
        }
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `u32`
    fn validate_u32 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u32 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    /// Encode an `i64` value as an X.690 encoding of an `INTEGER`
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
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    /// Decode an `i64` from an X.690-encoded `INTEGER`
    fn decode_i64 (&self, el: &X690Element) -> ASN1Result<i64> {
        let int_bytes = self.decode_integer(el)?;
        read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `i64`
    fn validate_i64 (&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => {
                if v.len() > 8 {
                    return Err(el.to_asn1_error(ASN1ErrorCode::constraint_violation));
                }
                self.validate_integer_value(&v)
            },
            X690Value::Constructed(_) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_i64(&el)
            }
        }
    }

    /// Encode a `u64` value as an X.690 encoding of an `INTEGER`
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
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    /// Decode a `u64` from an X.690-encoded `INTEGER`
    fn decode_u64 (&self, el: &X690Element) -> ASN1Result<u64> {
        let int_bytes = match &el.value {
            X690Value::Primitive(v) => v,
            X690Value::Constructed(_) => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                return self.decode_u64(&el);
            }
        };
        if int_bytes.len() == size_of::<u64>() + 1 && int_bytes[0] == 0 {
            // It's still a u64, but it starts with a set bit.
            return Ok(u64::from_be_bytes([
                int_bytes[1],
                int_bytes[2],
                int_bytes[3],
                int_bytes[4],
                int_bytes[5],
                int_bytes[6],
                int_bytes[7],
                int_bytes[8],
            ]));
        }
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        i.try_into().map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `u64`
    fn validate_u64 (&self, el: &X690Element) -> ASN1Result<()> {
        let int_bytes = self.decode_integer(el)?;
        let i = read_i64(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        let _: u64 = i.try_into()
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(())
    }

    /// Encode an `i128` value as an X.690 encoding of an `INTEGER`
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
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    /// Decode an `i128` from an X.690-encoded `INTEGER`
    fn decode_i128 (&self, el: &X690Element) -> ASN1Result<i128> {
        read_i128(&el.content_octets()?.as_ref())
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))
    }

    /// Validate an encoded `i128`
    fn validate_i128 (&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = el.content_octets()?;
        if content_octets.len() > 16 {
            Ok(())
        } else {
            Err(el.to_asn1_error(ASN1ErrorCode::value_too_big))
        }
    }

    /// Encode a `u128` value as an X.690 encoding of an `INTEGER`
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
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
            X690Value::Primitive(content.into()),
        ));
    }

    /// Decode a `u128` from an X.690-encoded `INTEGER`
    fn decode_u128 (&self, el: &X690Element) -> ASN1Result<u128> {
        let int_bytes = match &el.value {
            X690Value::Primitive(v) => v,
            X690Value::Constructed(_) => return Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                return self.decode_u128(&el);
            }
        };
        if int_bytes.len() == size_of::<u128>() + 1 && int_bytes[0] == 0 {
            // It's still a u128, but it starts with a set bit.
            return Ok(u128::from_be_bytes([
                int_bytes[1],
                int_bytes[2],
                int_bytes[3],
                int_bytes[4],
                int_bytes[5],
                int_bytes[6],
                int_bytes[7],
                int_bytes[8],
                int_bytes[9],
                int_bytes[10],
                int_bytes[11],
                int_bytes[12],
                int_bytes[13],
                int_bytes[14],
                int_bytes[15],
                int_bytes[16],
            ]));
        }
        let i = read_i128(&int_bytes)
            .ok_or(el.to_asn1_error(ASN1ErrorCode::value_too_big))?;
        Ok(i as u128)
    }

    /// Validate an encoded `u128`
    fn validate_u128 (&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => {
                if v.len() > 8 {
                    return Err(el.to_asn1_error(ASN1ErrorCode::constraint_violation));
                }
                self.validate_integer_value(&v)
            },
            X690Value::Constructed(_) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                self.validate_u128(&el)
            }
        }
    }

}

/// The Basic Encoding Rules (BER)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BasicEncodingRules;

/// The Distinguished Encoding Rules (DER)
#[cfg(feature = "der")]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DistinguishedEncodingRules;

impl BasicEncodingRules {
    /// Instantiate the Basic Encoding Rules (BER)
    pub const fn new() -> Self {
        BasicEncodingRules{}
    }
}

#[cfg(feature = "der")]
impl DistinguishedEncodingRules {
    /// Instantiate the Distinguished Encoding Rules (DER)
    pub const fn new() -> Self {
        DistinguishedEncodingRules{}
    }
}

impl ASN1Codec for BasicEncodingRules {
    fn transfer_syntax_oid(&self) -> OBJECT_IDENTIFIER {
        OBJECT_IDENTIFIER::try_from([2u32, 1, 1].as_slice()).unwrap()
    }

    fn transfer_syntax_oid_iri(&self) -> Option<wildboar_asn1::OID_IRI> {
        Some("/ASN.1/Basic-Encoding".into())
    }
}

#[cfg(feature = "der")]
impl ASN1Codec for DistinguishedEncodingRules {
    fn transfer_syntax_oid(&self) -> OBJECT_IDENTIFIER {
        OBJECT_IDENTIFIER::try_from([2u32, 1, 2, 1].as_slice()).unwrap()
    }

    fn transfer_syntax_oid_iri(&self) -> Option<wildboar_asn1::OID_IRI> {
        Some("/ASN.1/BER-Derived/Distinguished-Encoding".into())
    }
}
