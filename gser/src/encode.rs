// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![allow(non_upper_case_globals)]
use cow_utils::CowUtils;
use asn1::{
    ASN1Value,
    BOOLEAN,
    BIT_STRING,
    INTEGER,
    INSTANCE_OF,
    EXTERNAL,
    EMBEDDED_PDV,
    CHARACTER_STRING,
    GeneralizedTime,
    UTCTime,
    SEQUENCE_OF,
    SET_OF,
    OBJECT_IDENTIFIER,
    RELATIVE_OID,
    REAL,
    ObjectDescriptor,
    OCTET_STRING,
    read_i128,
    NamedType,
};

// BitStringValue /
// BooleanValue /
// CharacterStringValue /
// ChoiceValue /
// EmbeddedPDVValue /
// EnumeratedValue /
// ExternalValue /
// GeneralizedTimeValue /
// IntegerValue /
// InstanceOfValue /
// NullValue /
// ObjectDescriptorValue /
// ObjectIdentifierValue /
// OctetStringValue /
// RealValue /
// RelativeOIDValue /
// SequenceOfValue /
// SequenceValue /
// SetOfValue /
// SetValue /
// StringValue /
// UTCTimeValue /
// VariantEncoding

pub fn gser_encode_any (value: &ASN1Value) -> String {
    value.to_string()
}

pub fn gser_encode_BitStringValue (value: &BIT_STRING) -> String {
    value.to_string()
}

pub fn gser_encode_BooleanValue (value: &BOOLEAN) -> String {
    if *value { "TRUE".into() } else { "FALSE".into() }
}

pub fn gser_encode_CharacterStringValue (value: &CHARACTER_STRING) -> String {
    value.to_string()
}

pub fn gser_encode_ChoiceValue (identifier: &str, value: &ASN1Value) -> String {
    [
        identifier,
        ":",
        value.to_string().as_str(),
    ].concat()
}

pub fn gser_encode_EmbeddedPDVValue (value: &EMBEDDED_PDV) -> String {
    value.to_string()
}

// You need to supply the identifier.
pub fn gser_encode_EnumeratedValue (value: &str) -> String {
    value.to_string()
}

pub fn gser_encode_ExternalValue (value: &EXTERNAL) -> String {
    value.to_string()
}

pub fn gser_encode_GeneralizedTimeValue (value: &GeneralizedTime) -> String {
    format!("\"{}\"", &value)
}

pub fn gser_encode_IntegerValue (value: &INTEGER) -> String {
    match read_i128(&value) {
        Ok(i) => i.to_string(),
        Err(_) => hex::encode(&value),
    }
}

pub fn gser_encode_InstanceOfValue (value: &INSTANCE_OF) -> String {
    format!("{{ type-id {}, value {} }}", value.type_id.to_asn1_string(), &value.value)
}

pub fn gser_encode_NullValue () -> String {
    "NULL".into()
}

pub fn gser_encode_ObjectDescriptorValue (value: &ObjectDescriptor) -> String {
    format!("\"{}\"", &value.cow_replace("\"", "\"\"").as_ref())
}

pub fn gser_encode_ObjectIdentifierValue (value: &OBJECT_IDENTIFIER) -> String {
    value.to_string()
}

pub fn gser_encode_OctetStringValue (value: &OCTET_STRING) -> String {
    format!("\'{}\'H", &hex::encode(&value))
}

pub fn gser_encode_RealValue (value: &REAL) -> String {
    if value.is_infinite() {
        if value.is_sign_positive() {
            return "PLUS-INFINITY".into();
        } else {
            return "MINUS-INFINITY".into();
        }
    }
    value.to_string()
}

pub fn gser_encode_RelativeOIDValue (value: &RELATIVE_OID) -> String {
    value.to_string()
}

pub fn gser_encode_SequenceOfValue (value: &SEQUENCE_OF<ASN1Value>) -> String {
    let components = value
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("{{ {} }}", components)
}

pub fn gser_encode_SequenceValue <'a> (value: &[NamedType<'a, ASN1Value>]) -> String {
    let components = value
        .iter()
        .map(|c| format!("{} {}", c.identifier, c.value))
        .collect::<Vec<String>>()
        .join(", ");
    format!("{{ {} }}", components)
}

pub fn gser_encode_SetOfValue (value: &SET_OF<ASN1Value>) -> String {
    let components = value
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("{{ {} }}", components)
}

pub fn gser_encode_SetValue <'a> (value: &[NamedType<'a, ASN1Value>]) -> String {
    let components = value
        .iter()
        .map(|c| format!("{} {}", c.identifier, c.value))
        .collect::<Vec<String>>()
        .join(", ");
    format!("{{ {} }}", components)
}

pub fn gser_encode_StringValue (value: &str) -> String {
    format!("\"{}\"", &value.cow_replace("\"", "\"\"").as_ref())
}

pub fn gser_encode_UTCTimeValue (value: &UTCTime) -> String {
    format!("\"{}\"", &value)
}
