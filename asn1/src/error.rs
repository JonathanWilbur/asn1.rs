//! The `ASN1Error` error type
use crate::{ASN1Value, ComponentSpec};
use crate::tag::Tag;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::Utf8Error;

const VALUE_PREVIEW_SIZE: usize = 32;

/// The overall ASN.1 error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASN1ErrorCode {
    // I went with an enum rather than integer error codes, because of debug printing.

    /// Some kind of I/O error
    io,

    /// The encoding was truncated. This is NOT to be used when there are enough
    /// bytes of content octets for the length, but the content contained
    /// therein is invalid. This is for when the whole Tag-Length-Value (TLV) is
    /// truncated.
    tlv_truncated,

    /// The tag was too big
    tag_too_big,

    /// The length of an encoded value was too big
    length_too_big,

    /// The value encoded was too big, such as by overflowing an integer, or
    /// exceeding some other bounds imposed by the library.
    value_too_big,

    /// A field within an encoded value was too large
    field_too_big,

    /// An encoded value was too short to be valid
    value_too_short,

    /// An encoded value was malformed in some general way
    malformed_value,

    /// Unnecessary padding bytes (or bits) in an encoded tag number
    padding_in_tag_number,

    /// Tag number was encoded using the long form when it could have fit in
    /// short form. (I believe only applies to X.690 encodings, such as BER,
    /// CER, or DER.)
    tag_number_could_have_used_short_form,

    /// A constructed string was composed of substituent elements whose tag or
    /// tag class did not match.
    string_constructed_with_invalid_tagging,

    /// An element was expected to be encoded primitively, but it was
    /// constructed or vice versa.
    invalid_construction,

    /// An unrecognized component was found in an inextensible `SET` or
    /// `SEQUENCE`.
    unrecognized_components_in_inextensible_type,

    /// An unrecognized alternative was found in an inextensible `CHOICE`
    /// encoding.
    unrecognized_alternative_in_inextensible_choice,

    /// A prohibited character was found in a string. The first member is the
    /// character code point itself, and the second is its index.
    prohibited_character(u32, usize),

    /// An encoded constructed value was composed of way too many components and
    /// was not decoded as a protection against Denial of Service (DoS).
    construction_too_complex,

    /// Duplicate tags were found in a `SET` type.
    duplicate_tags_in_set,

    /// Duplicate components were found in a `SET` or `SEQUENCE`. This is a
    /// little different from [ASN1ErrorCode::duplicate_tags_in_set] in that
    /// this can happen when a component is identified by multiple tags, such as
    /// when it is a `CHOICE` type without an explicit tag.
    duplicate_components,

    /// Components that were required in a `SET` or `SEQUENCE` (not `OPTIONAL`
    /// or `DEFAULT`ing) were not found.
    missing_required_components,

    /// Padding bits or bytes were found in an integer encoding. In X.690
    /// encodings, the byte `0x00` may not appear at the start of a positive
    /// `INTEGER`, other than `0`, and `0xFF` may not appear at the start of a
    /// negative `INTEGER`.
    int_padding,

    /// A Distinguished Encoding Rules (DER)-encoded `BOOLEAN` was encoded using
    /// something other than a `0x00` or `0xFF`.
    der_boolean_not_0x00_or_0xFF,

    /// Unnecessary padding bytes or bits in an encoded `OBJECT IDENTIFIER`. In
    /// X.690 encodings, an `OBJECT IDENTIFIER` arc may not start with the byte
    /// `0x80`, which is meaningless padding.
    oid_padding,

    /// Unrecognized special `REAL` value not encountered.
    unrecognized_special_real,

    /// A binary real number used a base value (as in part of the mantissa,
    /// base, and exponent that comprise a real number) that was unrecognized.
    bin_real_unrecognized_base,

    /// A binary real number used an exponent format that was unrecognized.
    bin_real_unrecognized_exp_fmt,

    /// A base-10 `REAL` string was malformed. The variant value is the bytes
    /// of the string.
    base_10_real_string_malformed(Vec<u8>), // bytes of the string

    /// Unrecognized `REAL` string format. The variant value is the identifier
    /// for the format.
    base_10_real_unrecognized_format(u8),

    /// A base-10 `REAL` number used a base value (as in part of the mantissa,
    /// base, and exponent that comprise a real number) that was unrecognized.
    base_10_real_unrecognized_base(u8),

    /// Long-form length was used when it was not necessary.
    x690_long_form_unnecessary,

    /// Indefinite length was indicated, but constructed encoding was not used.
    x690_indefinite_length_but_not_constructed,

    /// X.690 `BOOLEAN` value not encoded on one byte
    x690_boolean_not_one_byte,

    /// X.690 `BIT STRING` had trailing bits greater than 7.
    x690_bit_string_remainder_gt_7,

    /// X.690 `BIT STRING` had trailing bits indicated, yet no bits
    x690_bit_string_remainder_but_no_bits,

    /// X.690 `BIT STRING` encoded on zero bytes
    x690_bit_string_zero_bytes,

    /// X.690 constructed `BIT STRING` had a non-terminal substituent
    /// tag-length-value that had trailing bits.
    x690_bit_string_non_terminal_segment_with_trailing_bits,

    /// Invalid year
    invalid_year,

    /// Invalid month
    invalid_month,

    /// Invalid day
    invalid_day,

    /// Invalid hour
    invalid_hour,

    /// Invalid minute
    invalid_minute,

    /// Invalid second
    invalid_second,

    /// Invalid fraction of seconds
    invalid_fraction_of_seconds,

    /// Invalid time offset
    invalid_time_offset,

    /// Invalid UTF-8 encoding
    invalid_utf8(Option<Utf8Error>),

    /// An impossible error that should never happen.
    nonsense,

    /// Constraint violation, such as an `INTEGER` value of `5` being used where
    /// the type `INTEGER (1..3)` is expected.
    constraint_violation,

    /// Trailing content octets
    trailing_content_octets,

    /// Invalid `DURATION` component
    invalid_duration_component(char),

    /// An OID arc was negative or the first OID arc was not 0, 1, or 2, or the
    /// second OID arc was greater than 39 whent he first was 0 or 1.
    invalid_oid_arc,

    /// Indefinite length was used where not allowed, such as in DER.
    indefinite_length_prohibited,

    /// A `BIT STRING` had trailing bits set, which is not allowed.
    bit_string_trailing_bits_set,

    /// A `REAL` was encoded using a format that is not allowed.
    real_format_prohibited,

    /// A `REAL` was encoded using a base that is not allowed.
    real_base_prohibited,
}

/// An ASN.1-related error
#[derive(Debug)]
pub struct ASN1Error {

    /// The "kind" of error
    pub error_code: ASN1ErrorCode,

    /// The name of the offending component in a `SET` or `SEQUENCE`
    pub component_name: Option<String>,

    /// The tag of the offending component
    pub tag: Option<Tag>,

    /// The length of the offending component
    pub length: Option<usize>,

    /// Whether the offending component was constructed
    pub constructed: Option<bool>,

    /// A human-readable preview of the value. Users MUST NOT rely on this
    /// having any particular format.
    pub value_preview: Option<String>,

    /// The number of bytes into the IO read stream where this error appeared.
    pub bytes_read: Option<usize>,

    /// The number of ASN.1 values into the IO read stream where this error appeared.
    pub values_read: Option<usize>,

    /// The underlying error
    pub err_source: Option<Box<dyn std::error::Error + 'static>>,
}

// TODO: fluent API: e.g. with_tag()

impl std::error::Error for ASN1Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.err_source.as_deref()
    }
}

impl fmt::Display for ASN1ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ASN1Error {

    /// Construct a new `ASN1Error` from just an error code
    #[inline]
    pub const fn new(error_code: ASN1ErrorCode) -> Self {
        ASN1Error {
            error_code,
            component_name: None,
            tag: None,
            length: None,
            constructed: None,
            value_preview: None,
            bytes_read: None,
            values_read: None,
            err_source: None,
        }
    }

    /// Relate an ASN.1 value to this error code.
    #[inline]
    pub fn relate_value (&mut self, value: &ASN1Value) {
        // We truncate the value string to prevent Denial-of-Service by flooding
        // logs with gigantic strings
        self.value_preview = Some(value
            .to_string() // TODO: to_preview() instead?
            .chars()
            .take(VALUE_PREVIEW_SIZE)
            .collect());
    }

    /// Relate a Tag to this error code
    #[inline]
    pub fn relate_tag (&mut self, tag: &Tag) {
        self.tag = Some(*tag);
    }

    /// Relate a component spec to this error code
    #[inline]
    pub fn relate_spec (&mut self, spec: &ComponentSpec<'_>) {
        self.component_name = Some(String::from(spec.name));
    }

}

impl From<Error> for ASN1Error {
    #[inline]
    fn from(other: Error) -> Self {
        ASN1Error {
            error_code: ASN1ErrorCode::io,
            component_name: None,
            tag: None,
            length: None,
            constructed: None,
            value_preview: None,
            bytes_read: None,
            values_read: None,
            err_source: Some(Box::new(other)),
        }
    }
}

impl From<ASN1Error> for std::io::Error {

    #[inline]
    fn from(mut value: ASN1Error) -> Self {
        if let Some(e) = value.err_source.take() {
            if let Ok(io_err) = e.downcast::<std::io::Error>() {
                return *io_err
            }
        }
        std::io::Error::from(ErrorKind::InvalidData)
    }

}

impl From<Utf8Error> for ASN1Error {

    #[inline]
    fn from(value: Utf8Error) -> Self {
        ASN1Error {
            error_code: ASN1ErrorCode::invalid_utf8(Some(value)),
            component_name: None,
            tag: None,
            length: None,
            constructed: None,
            value_preview: None,
            bytes_read: Some(value.valid_up_to()),
            values_read: None,
            err_source: None,
        }
    }

}

impl fmt::Display for ASN1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error_code.fmt(f)?;
        if let Some(component_name) = &self.component_name {
            write!(f, " component='{}'", component_name)?;
        }
        if let Some(tag) = self.tag {
            write!(f, " tag={}", tag)?;
        }
        if let Some(len) = self.length {
            write!(f, " len={}", len)?;
        }
        if let Some(c) = self.constructed {
            write!(f, " cons={}", c)?;
        }
        if let Some(index) = self.bytes_read {
            write!(f, " bytes_read={}", index)?;
        }
        if let Some(index) = self.values_read {
            write!(f, " values_read={}", index)?;
        }
        if let Some(preview) = &self.value_preview {
            write!(f, " peek='{}'", preview)?;
        }
        Ok(())
    }
}

/// An ASN.1-related result
pub type ASN1Result<T> = Result<T, ASN1Error>;
