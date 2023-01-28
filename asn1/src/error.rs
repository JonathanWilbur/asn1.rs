use crate::{ASN1Value, ComponentSpec};
use crate::types::Tag;
use std::fmt;
use std::io::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASN1ErrorCode {
    // I went with an enum rather than integer error codes, because of debug printing.
    io,
    truncated,
    tag_too_big,
    length_too_big,
    value_too_big,
    field_too_big,
    value_too_short,
    malformed_value,
    padding_in_tag_number,
    tag_number_could_have_used_short_form,
    string_constructed_with_invalid_tagging,
    failed_to_deconstruct,
    invalid_construction,
    unrecognized_components_in_inextensible_type,
    unrecognized_alternative_in_inextensible_choice,
    prohibited_character(u32, usize), // (charcode, index)
    construction_too_complex,
    duplicate_tags_in_set,
    duplicate_components,
    missing_required_components,
    int_padding,
    der_boolean_not_0x00_or_0xFF,
    oid_padding,
    urecognized_real_format,
    base_10_real_string_decoding_error,
    base_10_real_string_malformed(String),
    base_10_real_unrecognized_format(u8),
    base_10_real_unrecognized_base(u8),
    x690_long_form_unnecessary,
    x690_indefinite_length_but_not_constructed,
    x690_boolean_not_one_byte,
    x690_bit_string_remainder_gt_7,
    invalid_year,
    invalid_month,
    invalid_day,
    invalid_hour,
    invalid_minute,
    invalid_second,
    invalid_fraction_of_seconds,
    invalid_time_offset,
    invalid_utf8,
    nonsense, // An impossible error that should never happen.
    trailing_data,
}

#[derive(Debug)]
pub struct ASN1Error {
    pub error_code: ASN1ErrorCode,
    pub component_name: Option<String>,
    pub tag: Option<Tag>,
    pub length: Option<usize>,
    pub constructed: Option<bool>,
    pub value_preview: Option<String>,
    pub bytes_read: Option<usize>, // The number of bytes into the IO read stream where this error appeared.
    pub values_read: Option<usize>, // The number of ASN.1 values into the IO read stream where this error appeared.
    pub io_error: Option<Error>,
}

impl std::error::Error for ASN1Error {}

impl fmt::Display for ASN1ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::invalid_construction => write!(f, "invalid_construction"),
            Self::prohibited_character(c, i) => {
                write!(f, "prohibited_character (char={}, index={})", *c, *i)
            }
            Self::int_padding => write!(f, "int_padding"),
            Self::der_boolean_not_0x00_or_0xFF => write!(f, "der_boolean_not_0x00_or_0xFF"),
            Self::oid_padding => write!(f, "oid_padding"),
            Self::x690_long_form_unnecessary => write!(f, "x690_long_form_unnecessary"),
            _ => write!(f, "other"),
        }
    }
}

impl ASN1Error {
    pub fn new(error_code: ASN1ErrorCode) -> Self {
        ASN1Error {
            error_code,
            component_name: None,
            tag: None,
            length: None,
            constructed: None,
            value_preview: None,
            bytes_read: None,
            values_read: None,
            io_error: None,
        }
    }

    pub fn relate_value (&mut self, value: &ASN1Value) {
        self.value_preview = Some(value.to_string());
    }

    pub fn relate_tag (&mut self, tag: &Tag) {
        self.tag = Some(*tag);
    }

    pub fn relate_spec (&mut self, spec: &ComponentSpec<'_>) {
        self.component_name = Some(String::from(spec.name));
    }

}

impl From<Error> for ASN1Error {
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
            io_error: Some(other),
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

pub type ASN1Result<T> = Result<T, ASN1Error>;
