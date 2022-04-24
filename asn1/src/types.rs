use bit_vec::BitVec;
use std::vec::Vec;
use chrono::prelude::*;
use chrono::{Duration};

pub type Bytes = Vec<u8>;

// type END_OF_CONTENT = None;
pub type BOOLEAN = bool;
pub type INTEGER = i64;
pub type BIT_STRING = BitVec;
pub type OCTET_STRING = Bytes;
// type NULL = None;
pub type OBJECT_IDENTIFIER = Vec<u32>;
pub type ObjectDescriptor = String; // See: https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html/
// type EXTERNAL = // TODO:
pub type REAL = f64;
pub type ENUMERATED = i64;
// type EMBEDDED_PDV = // TODO:
pub type UTF8String = String;
pub type RELATIVE_OID = Vec<u32>;
pub type TIME = String;
// type Reserved15 = None;
pub type SEQUENCE = Vec<ASN1Value>;
pub type SEQUENCE_OF = Vec<ASN1Value>;
pub type SET = Vec<ASN1Value>;
pub type SET_OF = Vec<ASN1Value>;
pub type NumericString = String;
pub type PrintableString = String;
pub type T61String = Bytes;
pub type VideotexString = Bytes;
pub type IA5String = String;
pub type UTCTime = DateTime<Utc>;
pub type GeneralizedTime = DateTime<Utc>;
pub type GraphicString = String;
pub type VisibleString = String;
pub type GeneralString = String;
pub type UniversalString = String;
// pub type CharacterString = // TODO:
pub type BMPString = String;
pub type DATE = Date<Utc>;
pub type TIME_OF_DAY = DateTime<Utc>; // The "Date" part is ignored.
pub type DATE_TIME = DateTime<Utc>;
pub type DURATION = Duration;
pub type OID_IRI = String;
pub type RELATIVE_OID_IRI = String;

// BitStringValue
// BooleanValue
// CharacterStringValue
// ChoiceValue
// EmbeddedPDVValue
// EnumeratedValue
// ExternalValue
// InstanceOfValue
// IntegerValue
// IRIValue
// NullValue
// ObjectIdentifierValue
// OctetStringValue
// RealValue
// RelativeIRIValue
// RelativeOIDValue
// SequenceValue
// SequenceOfValue
// SetValue
// SetOfValue
// PrefixedValue
// TimeValue
pub enum ASN1Value {
    // BuiltInValue
    BitStringValue (BIT_STRING),
    BooleanValue (BOOLEAN),
    // CharacterStringValue  // TODO:
    // ChoiceValue // TODO
    // EmbeddedPDVValue // TODO:
    EnumeratedValue (ENUMERATED),
    // ExternalValue // TODO
    // InstanceOfValue // TODO:
    IntegerValue (INTEGER),
    IRIValue (OID_IRI),
    NullValue,
    ObjectIdentifierValue (OBJECT_IDENTIFIER),
    OctetStringValue (OCTET_STRING),
    RealValue (REAL),
    RelativeIRIValue (RELATIVE_OID_IRI),
    RelativeOIDValue (RELATIVE_OID),
    SequenceValue (SEQUENCE),
    SequenceOfValue (SEQUENCE_OF),
    SetValue (SET),
    SetOfValue (SET_OF),
    // PrefixedValue
    TimeValue (TIME),
}
