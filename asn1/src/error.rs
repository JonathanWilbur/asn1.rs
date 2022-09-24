use crate::types::Tag;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASN1ErrorCode { // I went with an enum rather than integer error codes, because of debug printing.
    invalid_construction,
    prohibited_character (u32, usize), // (charcode, index)
    int_padding,
    der_boolean_not_0x00_or_0xFF,
    oid_padding,
    x690_long_form_unnecessary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASN1Error {
    pub error_code: ASN1ErrorCode,
    pub component_name: Option<String>,
    pub tag: Option<Tag>,
    pub length: Option<usize>,
    pub constructed: Option<bool>,
    pub value_preview: Option<String>,
    pub bytes_read: Option<usize>, // The number of bytes into the IO read stream where this error appeared.
    pub values_read: Option<usize>, // The number of ASN.1 values into the IO read stream where this error appeared.
}

impl fmt::Display for ASN1ErrorCode {

    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::invalid_construction => write!(f, "invalid_construction"),
            Self::prohibited_character(c, i) =>
                write!(f, "prohibited_character (char={}, index={})", *c, *i),
            Self::int_padding => write!(f, "int_padding"),
            Self::der_boolean_not_0x00_or_0xFF => write!(f, "der_boolean_not_0x00_or_0xFF"),
            Self::oid_padding => write!(f, "oid_padding"),
            Self::x690_long_form_unnecessary => write!(f, "x690_long_form_unnecessary"),
            // _ => write!(f, "other"),
        }
    }

}

impl fmt::Display for ASN1Error {

    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
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

pub type ASN1Result <T> = Result<T, ASN1Error>;
