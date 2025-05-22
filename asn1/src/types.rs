use std::{fmt::Debug, sync::Arc, vec::Vec};
use smallvec::SmallVec;

use crate::error::ASN1Result;

pub type Bytes = Vec<u8>;
pub type ByteSlice<'a> = &'a [u8];
pub type OPTIONAL<T> = Option<T>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord)]
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct IdentificationSyntaxes {
    pub r#abstract: OBJECT_IDENTIFIER,
    pub transfer: OBJECT_IDENTIFIER,
}

impl IdentificationSyntaxes {
    #[inline]
    pub const fn new(r#abstract: OBJECT_IDENTIFIER, transfer: OBJECT_IDENTIFIER) -> Self {
        IdentificationSyntaxes {
            r#abstract,
            transfer,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ContextNegotiation {
    pub presentation_context_id: INTEGER,
    pub transfer_syntax: OBJECT_IDENTIFIER,
}

impl ContextNegotiation {
    #[inline]
    pub const fn new(presentation_context_id: INTEGER, transfer_syntax: OBJECT_IDENTIFIER) -> Self {
        ContextNegotiation {
            presentation_context_id,
            transfer_syntax,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExternalIdentification {
    // syntaxes                (IdentificationSyntaxes),
    syntax(OBJECT_IDENTIFIER),
    presentation_context_id(INTEGER),
    context_negotiation(ContextNegotiation),
    // transfer_syntax         (OBJECT_IDENTIFIER),
    // fixed,
}

#[derive(Debug, Clone, PartialEq)]
// See ITU Recommendation X.690, Section 8.18.
pub enum ExternalEncoding<Asn1Type: Sized = Arc<ASN1Value>> {
    single_ASN1_type(Asn1Type),
    octet_aligned(OCTET_STRING),
    arbitrary(BIT_STRING),
}

#[derive(Debug, Clone, PartialEq)]
pub struct External<Asn1Type: Sized = Arc<ASN1Value>> {
    pub identification: ExternalIdentification,
    pub data_value_descriptor: OPTIONAL<ObjectDescriptor>,
    pub data_value: ExternalEncoding<Asn1Type>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum PresentationContextSwitchingTypeIdentification {
    syntaxes(IdentificationSyntaxes),
    syntax(OBJECT_IDENTIFIER),
    presentation_context_id(INTEGER),
    context_negotiation(ContextNegotiation),
    transfer_syntax(OBJECT_IDENTIFIER),
    fixed,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct EmbeddedPDV {
    pub identification: PresentationContextSwitchingTypeIdentification,
    // pub data_value_descriptor: ObjectDescriptor,
    pub data_value: OCTET_STRING,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CharacterString {
    pub identification: PresentationContextSwitchingTypeIdentification,
    // pub data_value_descriptor: ObjectDescriptor,
    pub string_value: OCTET_STRING,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstanceOf {
    pub type_id: OBJECT_IDENTIFIER,
    pub value: Arc<ASN1Value>,
}

impl InstanceOf {
    #[inline]
    pub const fn new(type_id: OBJECT_IDENTIFIER, value: Arc<ASN1Value>) -> Self {
        InstanceOf { type_id, value }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct UTCOffset {
    pub hour: i8,
    pub minute: u8,
}

impl UTCOffset {
    #[inline]
    pub const fn new(hour: i8, minute: u8) -> Self {
        UTCOffset { hour, minute }
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.hour == 0 && self.minute == 0
    }

    #[inline]
    pub const fn utc() -> Self {
        UTCOffset{ hour: 0, minute: 0 }
    }
}

impl Default for UTCOffset {
    #[inline]
    fn default() -> Self {
        UTCOffset::utc()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct FractionalPart {
    pub number_of_digits: u8,
    pub fractional_value: u32,
}

impl FractionalPart {
    #[inline]
    pub const fn new(number_of_digits: u8, fractional_value: u32) -> Self {
        FractionalPart {
            number_of_digits,
            fractional_value,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum DurationPart {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds
}

impl Into<char> for DurationPart {

    fn into(self) -> char {
        match self {
            DurationPart::Years => 'Y',
            DurationPart::Months => 'M',
            DurationPart::Weeks => 'W',
            DurationPart::Days => 'D',
            DurationPart::Hours => 'H',
            DurationPart::Minutes => 'M',
            DurationPart::Seconds => 'S',
        }
    }

}

/// Defined in ITU-T Recommendation X.680, Section 38.4.4.2.
#[derive(Debug, Eq, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct DURATION_EQUIVALENT {
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub fractional_part: Option<(DurationPart, FractionalPart)>,
}

// type END_OF_CONTENT = None;
pub type BOOLEAN = bool;
pub type INTEGER = Bytes;
pub type BIT = usize;


#[cfg(not(feature = "smallvec"))]
#[derive(Debug, Eq, Clone)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct BIT_STRING {
    pub(crate) bytes: Vec<u8>,
    pub(crate) trailing_bits: u8,
}

#[cfg(feature = "smallvec")]
#[derive(Debug, Eq, Clone)]
pub struct BIT_STRING {
    pub(crate) bytes: SmallVec<[u8; 16]>,
    pub(crate) trailing_bits: u8,
}

pub type OCTET_STRING = Bytes;
// type NULL = None;
pub type OID_ARC = u32;

#[cfg(not(feature = "smallvec"))]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct OBJECT_IDENTIFIER (
    /// This contains the DER-encoding of the `OBJECT IDENTIFIER``, per ITU-T
    /// Recommendation X.690. This implementation favors faster comparison and
    /// hashing and lower memory footprint at the expense of slower parsing and
    /// printing.
    ///
    /// Intentionally not exported to library users so as to avoid dependency
    /// on the underlying storage of arcs.
    pub(crate) Vec<u8>
);

#[cfg(feature = "smallvec")]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct OBJECT_IDENTIFIER (
    /// This contains the DER-encoding of the `OBJECT IDENTIFIER``, per ITU-T
    /// Recommendation X.690. This implementation favors faster comparison and
    /// hashing and lower memory footprint at the expense of slower parsing and
    /// printing.
    ///
    /// The 16-byte inline vector was chosen because it is more than enough to
    /// accommodate the 12 bytes needed for an object identifier like
    /// 1.3.6.1.4.1.56490.5.4.13000. The vast majority of all object identifiers
    /// will fit without needing _any_ allocation on the heap.
    ///
    /// Intentionally not exported to library users so as to avoid dependency
    /// on the underlying storage of arcs.
    pub(crate) smallvec::SmallVec<[u8; 16]>
);

#[derive(Debug, Clone, Copy)]
pub struct OidArcs<'a> {
    /// The full DER-encoding, but optionally with a hack where a single root
    /// arc is stored as a single byte with the most significant bit set.
    pub(crate) encoded: &'a [u8],
    /// Index into the encoded OID. u32 instead of usize so this struct would
    /// still be 24 bytes instead of 32.
    pub(crate) i: u32,
    /// Whether the iterator already handled the first arc. We need this because
    /// both the first and second arcs could be encoded in the first byte, and
    /// i alone would be insufficient to tell us if we iterated over the first
    /// arc already.
    pub(crate) first_arc_read: bool,
    /// This is just used for reverse iteration.
    pub(crate) second_arc_read: bool,
}

#[cfg(not(feature = "smallvec"))]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Default)]
pub struct RELATIVE_OID (pub(crate) Vec<u8>);

#[cfg(feature = "smallvec")]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Default)]
pub struct RELATIVE_OID (pub(crate) smallvec::SmallVec<[u8; 16]>);

#[derive(Debug, Clone, Copy)]
pub struct RelOidArcs<'a> {
    /// The full DER-encoding
    pub(crate) encoded: &'a [u8],
    /// Index into the encoded OID.
    pub(crate) i: usize,
}

pub type ObjectDescriptor = GraphicString; // ObjectDescriptor ::= [UNIVERSAL 7] IMPLICIT GraphicString
pub type EXTERNAL = External;
pub type REAL = f64;
pub type ENUMERATED = i64;
pub type EMBEDDED_PDV = EmbeddedPDV;
pub type UTF8String = String;
pub type TIME = String;
// type Reserved15 = None;
pub type SEQUENCE = Vec<ASN1Value>;
pub type SEQUENCE_OF<T> = Vec<T>;
pub type SET = Vec<ASN1Value>;
pub type SET_OF<T> = Vec<T>;
pub type NumericString = String;
pub type PrintableString = String;
pub type T61String = Bytes;
pub type TeletexString = T61String;
pub type VideotexString = Bytes;
pub type IA5String = String;

/// ## Omitted Seconds Handling
///
/// Technically, seconds is optional, and it therefore may have been desirable
/// in the abstract to define this with `Option<u8>` being the value of the
/// seconds field, however, by making this field a simple `u8` instead, the
/// `UTCTime` type now fits in 8 bytes, meaning that it can be copied very
/// quickly in a single CPU instruction. _Since UTCTime should not be used in
/// general_, and since, where it is used, the seconds-level precision probably
/// does not matter too much anyways, this is a trade-off we accept.
///
/// When parsed, and the seconds component is omitted from the UTC timestamp,
/// the seconds time is set to 0. This aligns with the expected behavior
/// uTCTimeMatch X.500 directory equality matching rule defined in ITU-T
/// Recommendation X.520.
///
/// If this is a problem for your use case, let me know and I will figure out
/// something clever or just undo this and make `second` an `Option<u8>`, but I
/// think the much better performance is worth this trade-off.
///
/// ## Conversion to and from `DATE`, `TIME`, and `DATE-TIME`
///
/// `DATE`, `TIME`, and `DATE-TIME` are all defined in ITU-T Recommendation
/// X.680 as using local time, which means that it is unclear how to translate
/// a `UTCTime` value to a correct value of those other types and vice-versa.
/// Hence, there is intentionally no implementation of `From` for those types in
/// either direction; it is to prevent you from doing something wrong and
/// potentially insecure. This is true for `GeneralizedTime` as well.
///
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct UTCTime {
    /// Yes, `u8`, not `u16`: it is left to the application to determine which century the two-digit year identifies.
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub utc_offset: UTCOffset,
}

/// ## Conversion to and from `DATE`, `TIME`, and `DATE-TIME`
///
/// `DATE`, `TIME`, and `DATE-TIME` are all defined in ITU-T Recommendation
/// X.680 as using local time, which means that it is unclear how to translate
/// a `GeneralizedTime` value to a correct value of those other types and vice-versa.
/// Hence, there is intentionally no implementation of `From` for those types in
/// either direction; it is to prevent you from doing something wrong and
/// potentially insecure. This is true for `UTCTime` as well.
///
/// There is, however, an exception for converting `DATE` into
/// `GeneralizedTime`: when `From<DATE>` is used, the `GeneralizedTime` is made
/// into a local-item `GeneralizedTime` with the hours set to 0 and minutes and
/// seconds set to `None`. This is not incorrect: we are setting one local time
/// to another.
///
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct GeneralizedTime {
    pub date: DATE,
    /// `None` = Local time
    /// `Some`, where the offset is zero: UTC time
    /// `Some`, where the offset is non-zero: UTC difference
    pub utc_offset: Option<UTCOffset>,
    pub hour: u8,

    // Not only does this enforce correctness, but it also makes the struct
    // 24 bytes instead of 28, meaning that it can be read or copied more
    // efficiently on 64-bit systems.
    pub min_and_sec: Option<(u8, Option<u8>)>,

    /// The least significant four bits are the precision, in terms of decimal
    /// digits, of the fraction. This value will be 0 if there were no
    /// fractional digits.
    pub flags: u8,
    pub fraction: u32,
}
pub type GraphicString = String;
pub type VisibleString = String;
pub type GeneralString = String;
pub type UniversalString = String;
pub type CHARACTER_STRING = CharacterString;
pub type BMPString = String;
pub type NULL = ();

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct DATE {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct TIME_OF_DAY {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct DATE_TIME {
    pub date: DATE,
    pub time: TIME_OF_DAY,
}

pub type DURATION = DURATION_EQUIVALENT;
pub type OID_IRI = String;
pub type RELATIVE_OID_IRI = String;
pub type INSTANCE_OF = InstanceOf;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Tag {
    pub tag_class: TagClass,
    pub tag_number: TagNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaggedASN1Value {
    pub tag: Tag,
    pub explicit: bool,
    pub value: Arc<ASN1Value>,
}

impl TaggedASN1Value {
    #[inline]
    pub const fn new(
        tag_class: TagClass,
        tag_number: TagNumber,
        explicit: bool,
        value: Arc<ASN1Value>,
    ) -> Self {
        TaggedASN1Value {
            tag: Tag::new(tag_class, tag_number),
            explicit,
            value,
        }
    }
}

pub struct TYPE_IDENTIFIER {
    pub id: OBJECT_IDENTIFIER,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASN1Value {
    // BuiltInValue
    BitStringValue(BIT_STRING),
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
    RelativeOIDValue(RELATIVE_OID),
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
    UTCTime(UTCTime),
    GeneralizedTime(GeneralizedTime),
    DATE(DATE),
    TIME_OF_DAY(TIME_OF_DAY),
    DATE_TIME(DATE_TIME),
    DURATION(DURATION),

    /* This is a type that stores the value bytes of values that were encoded
    with an implicit tag and decoded as ANY. Since we cannot know what the
    actual encoded ASN.1 value was, we just have to store raw bytes. */
    UnknownBytes(Arc<Bytes>),
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

pub const MAX_IA5_STRING_CHAR_CODE: u8 = 0x0000_007F;

pub trait ObjectIdentifierIntoDescriptor {

    fn get_oid_descriptor (&self, oid: &OBJECT_IDENTIFIER) -> Option<String>;

}

pub trait ObjectIdentifierFromDescriptor {

    fn get_oid (&self, desc: &str) -> Option<OBJECT_IDENTIFIER>;

}

pub trait ASN1Codec {

    fn transfer_syntax_oid (&self) -> OBJECT_IDENTIFIER;

    fn transfer_syntax_oid_iri (&self) -> Option<OID_IRI> {
        None
    }

}

pub trait ISO8601Timestampable {

    fn to_iso_8601_string (&self) -> String;

}

/// Trait for a type whose X.690 content octets can be validated in such a way
/// that holds true for all of:
///
/// - The Basic Encoding Rules (BER)
/// - The Distinguished Encoding Rules (DER)
/// - The Canonical Encoding Rules (CER)
///
/// These functions are often useful for other codecs, such as the Packed
/// Encoding Rules (PER) or the Octet Encoding Rules (OER).
pub trait X690Validate {

    fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()>;

}

#[derive(Debug, Clone)]
pub struct NamedType <'a, Type = ASN1Value> {
    pub identifier: &'a str,
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

// This is really just an alias for vec![], but it is defined for future-proofing.
#[macro_export]
macro_rules! octs {
    () => {
        std::vec![]
    };
    ( $( $x:expr ),* ) => {
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
