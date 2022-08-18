use std::vec::Vec;

pub type Bytes = Vec<u8>;
pub type ByteSlice<'a> = &'a[u8];
pub type OPTIONAL<T> = Option<T>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TagClass {
    UNIVERSAL,
    APPLICATION,
    CONTEXT,
    PRIVATE,
}

// Based on an analysis of thousands of ASN.1 modules, no tag number ever
// exceeds this maximum. The largest tag number found in any ASN.1 specification
// is 12787. This fits within 14 bits, which means that, for X.690 encodings,
// it would be acceptable to only tolerate two bytes of long-length tag numbers.
pub type TagNumber = u16;

pub struct IdentificationSyntaxes {
    pub r#abstract: OBJECT_IDENTIFIER,
    pub transfer: OBJECT_IDENTIFIER,
}

pub struct ContextNegotiation {
    pub presentation_context_id: INTEGER,
    pub transfer_syntax: OBJECT_IDENTIFIER,
}

pub enum ExternalIdentification {
    // syntaxes                (IdentificationSyntaxes),
    syntax                  (OBJECT_IDENTIFIER),
    presentation_context_id (INTEGER),
    context_negotiation     (ContextNegotiation),
    // transfer_syntax         (OBJECT_IDENTIFIER),
    // fixed,
}

pub struct External {
    pub identification: ExternalIdentification,
    pub data_value_descriptor: OPTIONAL<ObjectDescriptor>,
    pub data_value: OCTET_STRING,
}

pub enum PresentationContextSwitchingTypeIdentification {
    syntaxes                (IdentificationSyntaxes),
    syntax                  (OBJECT_IDENTIFIER),
    presentation_context_id (INTEGER),
    context_negotiation     (ContextNegotiation),
    transfer_syntax         (OBJECT_IDENTIFIER),
    fixed,
}

pub struct EmbeddedPDV {
    pub identification: PresentationContextSwitchingTypeIdentification,
    // pub data_value_descriptor: ObjectDescriptor,
    pub data_value: OCTET_STRING,
}

pub struct CharacterString {
    pub identification: PresentationContextSwitchingTypeIdentification,
    // pub data_value_descriptor: ObjectDescriptor,
    pub string_value: OCTET_STRING,
}

pub struct InstanceOf <'a> {
    pub type_id: OBJECT_IDENTIFIER,
    pub value: &'a ASN1Value<'a>,
}

pub struct UTCOffset {
    pub hour: i8,
    pub minute: u8,
}

pub struct DurationFractionalPart {
    pub number_of_digits: u8,
    pub fractional_value: u32,
}

// Defined in ITU X.680, Section 38.4.4.2.
pub struct DURATION_EQUIVALENT {
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub fractional_part: Option<DurationFractionalPart>,
}

// type END_OF_CONTENT = None;
pub type BOOLEAN = bool;
pub type INTEGER = i64;
pub struct BIT_STRING {
    pub bytes: Vec<u8>,
    pub trailing_bits: u8,
}
pub type OCTET_STRING = Bytes;
// type NULL = None;
pub type OBJECT_IDENTIFIER = Vec<u32>;
pub type ObjectDescriptor = String;
pub type EXTERNAL = External;
pub type REAL = f64;
pub type ENUMERATED = i64;
pub type EMBEDDED_PDV = EmbeddedPDV;
pub type UTF8String = String;
pub type RELATIVE_OID = Vec<u32>;
pub type TIME = String;
// type Reserved15 = None;
pub type SEQUENCE <'a> = Vec<ASN1Value<'a>>;
pub type SEQUENCE_OF <'a> = Vec<ASN1Value<'a>>;
pub type SET <'a> = Vec<ASN1Value<'a>>;
pub type SET_OF <'a> = Vec<ASN1Value<'a>>;
pub type NumericString = String;
pub type PrintableString = String;
pub type T61String = Bytes;
pub type VideotexString = Bytes;
pub type IA5String = String;
// pub type UTCTime = DateTime<Utc>;
pub struct UTCTime {
    pub year: u8, // Yes, u8, not u16: it is left to the application to determine which century the two-digit year identifies.
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: Option<u8>,
    pub utc_offset: Option<UTCOffset>,
}
pub struct GeneralizedTime {
    pub date: DATE,
    pub hour: u8,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub fraction: Option<u16>,
    pub utc_offset: Option<UTCOffset>,
}
pub type GraphicString = String;
pub type VisibleString = String;
pub type GeneralString = String;
pub type UniversalString = String;
pub type CHARACTER_STRING = CharacterString;
pub type BMPString = String;

pub struct DATE {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

pub struct TIME_OF_DAY {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

pub struct DATE_TIME {
    pub date: DATE,
    pub time: TIME_OF_DAY,
}

pub type DURATION = DURATION_EQUIVALENT;
pub type OID_IRI = String;
pub type RELATIVE_OID_IRI = String;
pub type INSTANCE_OF <'a> = InstanceOf<'a>;

pub struct TaggedASN1Value <'a> {
    pub tag_class: TagClass,
    pub tag_number: TagNumber,
    pub explicit: bool,
    pub value: ASN1Value<'a>,
}

// Actually, I think this is unnecessary, because the tagged alternatives will
// be of type `TaggedASN1Value`.
// pub struct ChoiceValue <'a> {
//     pub name: String,
//     pub value: ASN1Value<'a>,
// }

pub enum ASN1Value <'a> {
    // BuiltInValue
    BitStringValue (BIT_STRING),
    BooleanValue (BOOLEAN),
    ChoiceValue (&'a ASN1Value<'a>),
    // ChoiceValue (&'a ChoiceValue<'a>),
    EmbeddedPDVValue (EMBEDDED_PDV),
    EnumeratedValue (ENUMERATED),
    ExternalValue (EXTERNAL),
    InstanceOfValue (INSTANCE_OF<'a>),
    IntegerValue (INTEGER),
    IRIValue (OID_IRI),
    NullValue,
    ObjectIdentifierValue (OBJECT_IDENTIFIER),
    OctetStringValue (OCTET_STRING),
    RealValue (REAL),
    RelativeIRIValue (RELATIVE_OID_IRI),
    RelativeOIDValue (RELATIVE_OID),
    SequenceValue (SEQUENCE<'a>),
    SequenceOfValue (SEQUENCE_OF<'a>),
    SetValue (SET<'a>),
    SetOfValue (SET_OF<'a>),
    // CharacterStringValue
    UnrestrictedCharacterStringValue (CHARACTER_STRING),
    // RestrictedCharacterStringType
    BMPString (BMPString),
	GeneralString (GeneralString),
	GraphicString (GraphicString),
	IA5String (IA5String),
	ISO646String (VisibleString), // Same as VisibleString.
	NumericString (NumericString),
	PrintableString (PrintableString),
	TeletexString (T61String), // Same as TeletexString.
	T61String (T61String),
	UniversalString (UniversalString),
	UTF8String (UTF8String),
	VideotexString (VideotexString),
	VisibleString (VisibleString),
    // PrefixedValue (&'a ASN1Value<'a>),
    TaggedValue (&'a TaggedASN1Value<'a>),
    TimeValue (TIME),
    UTCTime (UTCTime),
    GeneralizedTime (GeneralizedTime),
    DATE (DATE),
    TIME_OF_DAY (TIME_OF_DAY),
    DATE_TIME (DATE_TIME),
    DURATION (DURATION),
}

pub const ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT: TagNumber = 0;
pub const ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN: TagNumber = 1;
pub const ASN1_UNIVERSAL_TAG_NUMBER_INTEGER: TagNumber = 2;
pub const ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING: TagNumber = 3;
pub const ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING: TagNumber = 4;
pub const ASN1_UNIVERSAL_TAG_NUMBER_NULL: TagNumber = 5;
pub const ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER: TagNumber = 6;
pub const ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR: TagNumber = 7;
pub const ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL: TagNumber = 8;
pub const ASN1_UNIVERSAL_TAG_NUMBER_REAL: TagNumber = 9;
pub const ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED: TagNumber = 10;
pub const ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV: TagNumber = 11;
pub const ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING: TagNumber = 12;
pub const ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID: TagNumber = 13;
pub const ASN1_UNIVERSAL_TAG_NUMBER_TIME: TagNumber = 14;
pub const ASN1_UNIVERSAL_TAG_NUMBER_RESERVED_15: TagNumber = 15;
pub const ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE: TagNumber = 16;
pub const ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF: TagNumber = 16;
pub const ASN1_UNIVERSAL_TAG_NUMBER_SET: TagNumber = 17;
pub const ASN1_UNIVERSAL_TAG_NUMBER_SET_OF: TagNumber = 17;
pub const ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING: TagNumber = 18;
pub const ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING: TagNumber = 19;
pub const ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING: TagNumber = 20;
pub const ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING: TagNumber = 21;
pub const ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING: TagNumber = 22;
pub const ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME: TagNumber = 23;
pub const ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME: TagNumber = 24;
pub const ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING: TagNumber = 25;
pub const ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING: TagNumber = 26;
pub const ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING: TagNumber = 27;
pub const ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING: TagNumber = 28;
pub const ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING: TagNumber = 29;
pub const ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING: TagNumber = 30;
pub const ASN1_UNIVERSAL_TAG_NUMBER_DATE: TagNumber = 31;
pub const ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY: TagNumber = 32;
pub const ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME: TagNumber = 33;
pub const ASN1_UNIVERSAL_TAG_NUMBER_DURATION: TagNumber = 34;
pub const ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI: TagNumber = 35;
pub const ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI: TagNumber = 36;

pub const MAX_IA5_STRING_CHAR_CODE: char = 0x0000_00FF as char;