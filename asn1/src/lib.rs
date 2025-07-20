#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod bitstring;
pub mod constants;
pub mod construction;
pub mod date;
pub mod datetime;
pub mod display;
pub mod duration;
pub mod error;
pub mod external;
pub mod gentime;
pub mod oid;
pub mod roid;
pub mod strings;
pub mod tag;
pub mod time_of_day;
pub mod utctime;
pub mod utils;

pub use bitstring::*;
pub use constants::*;
pub use construction::*;
pub use date::*;
pub use datetime::*;
pub use display::*;
pub use duration::*;
pub use error::*;
pub use external::*;
pub use gentime::*;
pub use oid::*;
pub use roid::*;
pub use strings::*;
pub use tag::*;
pub use time_of_day::*;
pub use utctime::*;
pub use utils::*;

/// Alias to make `true` look like ASN.1
pub const TRUE: bool = true;

/// Alias to make `false` look like ASN.1
pub const FALSE: bool = false;

/// How this library represents borrowed "bytes"
pub type ByteSlice<'a> = &'a [u8];

/// An alias to make `Option<>` look more like ASN.1.
pub type OPTIONAL<T> = Option<T>;

/// Coordinated Universal Time (UTC) Offset
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct UTCOffset {
    /// The hour offset from Coordinated Universal Time (UTC)
    /// This may be between -15 to +15 inclusively.
    pub hour: i8,
    /// The minute offset from Coordinated Universal Time (UTC)
    pub minute: u8,
}

impl UTCOffset {

    /// Construct a new Coordinated Universal Time (UTC) Offset
    #[inline]
    pub const fn new(hour: i8, minute: u8) -> Self {
        UTCOffset { hour, minute }
    }

    /// Returns `true` if the Construct a new Coordinated Universal Time (UTC)
    /// Offset is 0 hours and 0 minutes.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.hour == 0 && self.minute == 0
    }

    /// Construct a new zeroed Coordinated Universal Time (UTC) Offset
    #[inline]
    pub const fn utc() -> Self {
        UTCOffset{ hour: 0, minute: 0 }
    }
}

impl Default for UTCOffset {

    /// Construct a new zeroed Coordinated Universal Time (UTC) Offset
    #[inline]
    fn default() -> Self {
        UTCOffset::utc()
    }
}

/// Decimal digits fractional part
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct FractionalPart {
    /// Number of digits of precision
    pub number_of_digits: u8,

    /// The fractional value that is to be converted to a decimal string and
    /// left padded with zeroes until it is `number_of_digits` digits long to
    /// produce the fractional part.
    pub fractional_value: u32,
}

impl FractionalPart {

    /// Construct a new [FractionalPart]
    #[inline]
    pub const fn new(number_of_digits: u8, fractional_value: u32) -> Self {
        FractionalPart {
            number_of_digits,
            fractional_value,
        }
    }
}

// type END_OF_CONTENT = None;

/// ASN.1 `BOOLEAN`
pub type BOOLEAN = bool;

/// ASN.1 `INTEGER`
pub type INTEGER = Vec<u8>;

/// Index into an ASN.1 `BIT STRING`
pub type BIT_INDEX = usize;

/// An ASN.1 `OCTET STRING`
pub type OCTET_STRING = Vec<u8>;

/// An ASN.1 `NULL` value
pub type NULL = ();

/// An arc within an ASN.1 `OBJECT IDENTIFIER` or `RELATIVE-OID`
pub type OID_ARC = u32;

/// ASN.1 `ObjectDescriptor`, which is defined as
///
/// ```asn1
/// ObjectDescriptor ::= [UNIVERSAL 7] IMPLICIT GraphicString
/// ```
///
pub type ObjectDescriptor = GraphicString;

/// ASN.1 `EXTERNAL`
pub type EXTERNAL = crate::external::External;

/// ASN.1 `REAL`
pub type REAL = f64;

/// ASN.1 `ENUMERATED`
pub type ENUMERATED = i64;

/// ASN.1 `EMBEDDED PDV`
pub type EMBEDDED_PDV = crate::external::EmbeddedPDV;

/// ASN.1 `UTF8String`
pub type UTF8String = String;

/// ASN.1 `TIME`
pub type TIME = String;
// type Reserved15 = None;

/// ASN.1 `SEQUENCE`
pub type SEQUENCE = Vec<ASN1Value>;

/// ASN.1 `SEQUENCE OF`
pub type SEQUENCE_OF<T> = Vec<T>;

/// ASN.1 `SET`
pub type SET = Vec<ASN1Value>;

/// ASN.1 `SET OF`
pub type SET_OF<T> = Vec<T>;

/// ASN.1 `NumericString`
pub type NumericString = String;

/// ASN.1 `PrintableString`
pub type PrintableString = String;

/// ASN.1 `T61String` / `TeletexString`
pub type T61String = Vec<u8>;

/// ASN.1 `T61String` / `TeletexString`
pub type TeletexString = T61String;

/// ASN.1 `VideotexString`
pub type VideotexString = Vec<u8>;

/// ASN.1 `IA5String`
pub type IA5String = String;

/// ASN.1 `GraphicString`
pub type GraphicString = String;

/// ASN.1 `VisibleString`
pub type VisibleString = String;

/// ASN.1 `GeneralString`
pub type GeneralString = String;

/// ASN.1 `UniversalString`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct UniversalString(pub Vec<u32>);

/// ASN.1 `CharacterString`
pub type CHARACTER_STRING = crate::external::CharacterString;

/// ASN.1 `BMPString`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct BMPString(pub Vec<u16>);

/// ASN.1 `DURATION`
pub type DURATION = crate::duration::DURATION_EQUIVALENT;

/// ASN.1 `OBJECT IDENTIFIER` Internationalized Resource Identifier (OID-IRI)
pub type OID_IRI = String;
/// ASN.1 `RELATIVE-OID` Internationalized Resource Identifier (Relative-OID-IRI)
pub type RELATIVE_OID_IRI = String;

/// ASN.1 `INSTANCE OF`
pub type INSTANCE_OF = crate::external::InstanceOf;

/// ASN.1 `TYPE-IDENTIFIER`
pub struct TYPE_IDENTIFIER {
    /// The `type-id` field.
    pub id: OBJECT_IDENTIFIER,
}

/// ASN.1 value
#[derive(Debug, Clone, PartialEq)]
pub enum ASN1Value {
    // BuiltInValue
    BitStringValue(crate::bitstring::BIT_STRING),
    BooleanValue(BOOLEAN),
    ChoiceValue(Box<ASN1Value>),
    // ChoiceValue (&'a ChoiceValue<'a>),
    EmbeddedPDVValue(EMBEDDED_PDV),
    EnumeratedValue(ENUMERATED),
    ExternalValue(EXTERNAL),
    InstanceOfValue(INSTANCE_OF),
    IntegerValue(INTEGER),
    IRIValue(OID_IRI),
    NullValue,
    ObjectIdentifierValue(OBJECT_IDENTIFIER),
    ObjectDescriptor(ObjectDescriptor),
    OctetStringValue(OCTET_STRING),
    RealValue(REAL),
    RelativeIRIValue(RELATIVE_OID_IRI),
    RelativeOIDValue(crate::roid::RELATIVE_OID),
    SequenceValue(SEQUENCE),
    SequenceOfValue(SEQUENCE_OF<ASN1Value>),
    SetValue(SET),
    SetOfValue(SET_OF<ASN1Value>),
    // CharacterStringValue
    UnrestrictedCharacterStringValue(CHARACTER_STRING),
    // RestrictedCharacterStringType
    BMPString(BMPString),
    GeneralString(GeneralString),
    GraphicString(GraphicString),
    IA5String(IA5String),
    ISO646String(VisibleString), // Same as VisibleString.
    NumericString(NumericString),
    PrintableString(PrintableString),
    TeletexString(T61String), // Same as TeletexString.
    T61String(T61String),
    UniversalString(UniversalString),
    UTF8String(UTF8String),
    VideotexString(VideotexString),
    VisibleString(VisibleString),
    // PrefixedValue (&'a ASN1Value<'a>),
    TaggedValue(TaggedASN1Value),
    TimeValue(TIME),
    UTCTime(crate::utctime::UTCTime),
    GeneralizedTime(GeneralizedTime),
    DATE(DATE),
    TIME_OF_DAY(TIME_OF_DAY),
    DATE_TIME(crate::datetime::DATE_TIME),
    DURATION(DURATION),

    /* This is a type that stores the value bytes of values that were encoded
    with an implicit tag and decoded as ANY. Since we cannot know what the
    actual encoded ASN.1 value was, we just have to store raw bytes. */
    UnknownBytes(std::sync::Arc<Vec<u8>>),
}

/// The `UNIVERSAL` tag number for `END-OF-CONTENT`
pub const UNIV_TAG_END_OF_CONTENT: TagNumber = 0;
/// The `UNIVERSAL` tag number for `BOOLEAN`
pub const UNIV_TAG_BOOLEAN: TagNumber = 1;
/// The `UNIVERSAL` tag number for `INTEGER`
pub const UNIV_TAG_INTEGER: TagNumber = 2;
/// The `UNIVERSAL` tag number for `BIT STRING`
pub const UNIV_TAG_BIT_STRING: TagNumber = 3;
/// The `UNIVERSAL` tag number for `OCTET STRING`
pub const UNIV_TAG_OCTET_STRING: TagNumber = 4;
/// The `UNIVERSAL` tag number for `NULL`
pub const UNIV_TAG_NULL: TagNumber = 5;
/// The `UNIVERSAL` tag number for `OBJECT IDENTIFIER`
pub const UNIV_TAG_OBJECT_IDENTIFIER: TagNumber = 6;
/// The `UNIVERSAL` tag number for `ObjectDescriptor`
pub const UNIV_TAG_OBJECT_DESCRIPTOR: TagNumber = 7;
/// The `UNIVERSAL` tag number for `EXTERNAL`
pub const UNIV_TAG_EXTERNAL: TagNumber = 8;
/// The `UNIVERSAL` tag number for `INSTANCE OF`
pub const UNIV_TAG_INSTANCE_OF: TagNumber = UNIV_TAG_EXTERNAL;
/// The `UNIVERSAL` tag number for `REAL`
pub const UNIV_TAG_REAL: TagNumber = 9;
/// The `UNIVERSAL` tag number for `ENUMERATED`
pub const UNIV_TAG_ENUMERATED: TagNumber = 10;
/// The `UNIVERSAL` tag number for `EMBEDDED PDV`
pub const UNIV_TAG_EMBEDDED_PDV: TagNumber = 11;
/// The `UNIVERSAL` tag number for `UTF8String`
pub const UNIV_TAG_UTF8_STRING: TagNumber = 12;
/// The `UNIVERSAL` tag number for `RELATIVE-OID`
pub const UNIV_TAG_RELATIVE_OID: TagNumber = 13;
/// The `UNIVERSAL` tag number for `TIME`
pub const UNIV_TAG_TIME: TagNumber = 14;
/// The reserved `UNIVERSAL` tag number 15
pub const UNIV_TAG_RESERVED_15: TagNumber = 15;
/// The `UNIVERSAL` tag number for `SEQUENCE`
pub const UNIV_TAG_SEQUENCE: TagNumber = 16;
/// The `UNIVERSAL` tag number for `SEQUENCE OF`
pub const UNIV_TAG_SEQUENCE_OF: TagNumber = UNIV_TAG_SEQUENCE;
/// The `UNIVERSAL` tag number for `SET`
pub const UNIV_TAG_SET: TagNumber = 17;
/// The `UNIVERSAL` tag number for `SET OF`
pub const UNIV_TAG_SET_OF: TagNumber = UNIV_TAG_SET;
/// The `UNIVERSAL` tag number for `NumericString`
pub const UNIV_TAG_NUMERIC_STRING: TagNumber = 18;
/// The `UNIVERSAL` tag number for `PrintableString`
pub const UNIV_TAG_PRINTABLE_STRING: TagNumber = 19;
/// The `UNIVERSAL` tag number for `T61String` / `TeletexString`
pub const UNIV_TAG_T61_STRING: TagNumber = 20;
/// The `UNIVERSAL` tag number for `VideotexString`
pub const UNIV_TAG_VIDEOTEX_STRING: TagNumber = 21;
/// The `UNIVERSAL` tag number for `IA5String`
pub const UNIV_TAG_IA5_STRING: TagNumber = 22;
/// The `UNIVERSAL` tag number for `UTCTime`
pub const UNIV_TAG_UTC_TIME: TagNumber = 23;
/// The `UNIVERSAL` tag number for `GeneralizedTime`
pub const UNIV_TAG_GENERALIZED_TIME: TagNumber = 24;
/// The `UNIVERSAL` tag number for `GraphicString`
pub const UNIV_TAG_GRAPHIC_STRING: TagNumber = 25;
/// The `UNIVERSAL` tag number for `VisibleString`
pub const UNIV_TAG_VISIBLE_STRING: TagNumber = 26;
/// The `UNIVERSAL` tag number for `GeneralString`
pub const UNIV_TAG_GENERAL_STRING: TagNumber = 27;
/// The `UNIVERSAL` tag number for `UniversalString`
pub const UNIV_TAG_UNIVERSAL_STRING: TagNumber = 28;
/// The `UNIVERSAL` tag number for `CharacterString`
pub const UNIV_TAG_CHARACTER_STRING: TagNumber = 29;
/// The `UNIVERSAL` tag number for `BMPString`
pub const UNIV_TAG_BMP_STRING: TagNumber = 30;
/// The `UNIVERSAL` tag number for `DATE`
pub const UNIV_TAG_DATE: TagNumber = 31;
/// The `UNIVERSAL` tag number for `TIME-OF-DAY`
pub const UNIV_TAG_TIME_OF_DAY: TagNumber = 32;
/// The `UNIVERSAL` tag number for `DATE-TIME`
pub const UNIV_TAG_DATE_TIME: TagNumber = 33;
/// The `UNIVERSAL` tag number for `DURATION`
pub const UNIV_TAG_DURATION: TagNumber = 34;
/// The `UNIVERSAL` tag number for `OID-IRI`
pub const UNIV_TAG_OID_IRI: TagNumber = 35;
/// The `UNIVERSAL` tag number for `RELATIVE-OID-IRI`
pub const UNIV_TAG_RELATIVE_OID_IRI: TagNumber = 36;

/// An ASN.1 Codec
pub trait ASN1Codec {

    /// Get an `OBJECT IDENTIFIER` representing this codec as a transfer syntax
    fn transfer_syntax_oid (&self) -> OBJECT_IDENTIFIER;

    /// Get an OID-IRI representing this codec as a transfer syntax
    fn transfer_syntax_oid_iri (&self) -> Option<OID_IRI> {
        None
    }

}

/// Something that can be converted into an ISO 8601 Timestamp
pub trait ISO8601Timestampable {

    /// Convert this into an ISO 8601 Timestamp
    fn to_iso_8601_string (&self) -> String;

}

/// Trait for a type whose X.690 content octets can be validated in such a way
/// that holds true for all X.690 codecs.
///
/// These are:
///
/// - The Basic Encoding Rules (BER)
/// - The Distinguished Encoding Rules (DER)
/// - The Canonical Encoding Rules (CER)
///
/// These functions are often useful for other codecs, such as the Packed
/// Encoding Rules (PER) or the Octet Encoding Rules (OER).
pub trait X690Validate {

    /// Validate that the `content_octets` are a valid X.690 encoding of this
    /// data type.
    fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()>;

}

/// A Named Type, such as would appear in the component type lists in a
/// `SET` or `SEQUENCE`
#[derive(Debug, Clone)]
pub struct NamedType <'a, Type = ASN1Value> {

    /// The identifier, such as `subjectPublicKeyInfo`
    pub identifier: &'a str,

    /// The value
    pub value: Type,
}

/// Anything that, when encoded as the content octets ("value") of an X.690
/// Tag-Length-Value (TLV), will be encoded on a number of octets that can be
/// trivially calculated, and does not vary with the choice of concrete syntax
/// (BER, CER, or DER). This is so a codec can know in advance how many bytes
/// a value will take up and pre-allocate them.
///
/// `OBJECT IDENTIFIER` values are an example: they are encoded the same way
/// for BER, CER, and DER, and it isn't too hard to calculate how many bytes
/// they will take up in advance (in this library's implementation, there is
/// _no_ calculation that needs to be done: just reading a `.len()`). An example
/// of a type that MUST NOT implement this trait would be a `BIT STRING` because
/// it would have a different length if encoded using DER or CER. All of the
/// context-switching types (e.g. `EXTERNAL`, `EMBEDDED PDV`, `CharacterString`)
/// are also transitively disqualified for this reason and more.
///
pub trait X690KnownSize {

    /// Get the size of the content octets ("value") of an X.690
    /// Tag-Length-Value (TLV) encoding when this value is encoded.
    fn x690_size (&self) -> usize;

}

/// Create an `OCTET STRING`
///
/// This is really just an alias for vec![], but it is defined for future-proofing.
#[macro_export]
macro_rules! octs {
    () => {
        std::vec![]
    };
    ( $( $x:expr ),+ ) => {
        std::vec![$($x,)*]
    };
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_octs_macro () {
        let octets = octs!(1,3,6,4,1);
        assert_eq!(octets.len(), 5);
    }

}
