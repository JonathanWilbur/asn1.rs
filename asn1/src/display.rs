use crate::types::{ASN1Value, INTEGER};
use crate::utils::{read_i64, likely};
use crate::FractionalPart;
use std::fmt::{Display, Write};

pub(crate) fn write_hex(v: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if cfg!(feature = "faster-hex") {
        f.write_str(faster_hex::hex_string(v).as_str())
    } else {
        for b in v {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

/// Write an ASN.1 `INTEGER` for human-reading. The output may be in decimal
/// or hexadecimal, and this could vary by the length. Do not depend on this
/// function for a particular output.
pub fn write_int(int: &INTEGER, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match read_i64(int) {
        Some(v) => {
            if cfg!(feature = "itoa") {
                let mut buf = itoa::Buffer::new();
                f.write_str(buf.format(v))
            } else {
                write!(f, "{}", v)
            }
        },
        None => {
            f.write_str("0x")?;
            write_hex(int, f)
        }
    }
}

impl Display for ASN1Value {

    /// Print an ASN.1 value as closely as possible to its abstract syntax
    /// notation. For instance, a `BIT STRING` will be printed like
    /// `'11010110'B`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASN1Value::BitStringValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::BooleanValue(v) => {
                if *v {
                    f.write_str("TRUE")
                } else {
                    f.write_str("FALSE")
                }
            }
            ASN1Value::ChoiceValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::EmbeddedPDVValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::EnumeratedValue(v) => f.write_str(v.to_string().as_str()),
            ASN1Value::ExternalValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::InstanceOfValue(v) => write!(f,
                "{{ type-id {}, value {} }}",
                v.type_id.to_asn1_string(),
                v.value
            ),
            ASN1Value::IntegerValue(v) => write_int(v, f),
            ASN1Value::IRIValue(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::NullValue => f.write_str("NULL"),
            ASN1Value::ObjectIdentifierValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::ObjectDescriptor(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::OctetStringValue(v) => {
                f.write_char('\'')?;
                write_hex(v, f)?;
                f.write_str("'H")
            }
            ASN1Value::RealValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::RelativeIRIValue(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::RelativeOIDValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::SequenceValue(v) => {
                let mut i = 0;
                f.write_str("{ ")?;
                for component in v {
                    if likely(i > 0) {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(component, f)?;
                    i += 1;
                }
                f.write_str(" }")
            }
            ASN1Value::SequenceOfValue(v) => {
                let mut i = 0;
                f.write_str("{ ")?;
                for component in v {
                    if likely(i > 0) {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(component, f)?;
                    i += 1;
                }
                f.write_str(" }")
            }
            ASN1Value::SetValue(v) => {
                let mut i = 0;
                f.write_str("{ ")?;
                for component in v {
                    if likely(i > 0) {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(component, f)?;
                    i += 1;
                }
                f.write_str(" }")
            }
            ASN1Value::SetOfValue(v) => {
                let mut i = 0;
                f.write_str("{ ")?;
                for component in v {
                    if likely(i > 0) {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(component, f)?;
                    i += 1;
                }
                f.write_str(" }")
            }
            ASN1Value::UnrestrictedCharacterStringValue(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::BMPString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::GeneralString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::GraphicString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::IA5String(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::ISO646String(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::NumericString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::PrintableString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::TeletexString(v) => {
                f.write_char('\'')?;
                write_hex(v, f)?;
                f.write_str("'H")
            }
            ASN1Value::T61String(v) => {
                f.write_char('\'')?;
                write_hex(v, f)?;
                f.write_str("'H")
            }
            ASN1Value::UniversalString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::UTF8String(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::VideotexString(v) => {
                f.write_char('\'')?;
                write_hex(v, f)?;
                f.write_str("'H")
            }
            ASN1Value::VisibleString(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::TaggedValue(v) => {
                if v.explicit {
                    write!(f, "{} EXPLICIT {}", v.tag, v.value)
                } else {
                    write!(f, "{} IMPLICIT {}", v.tag, v.value)
                }
            }
            ASN1Value::TimeValue(v) => {
                f.write_char('"')?;
                f.write_str(v.replace("\"", "\"\"").as_str())?;
                f.write_char('"')
            }
            ASN1Value::UTCTime(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::GeneralizedTime(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::DATE(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::TIME_OF_DAY(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::DATE_TIME(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::DURATION(v) => std::fmt::Display::fmt(&v, f),
            ASN1Value::UnknownBytes(v) => {
                f.write_char('\'')?;
                write_hex(v, f)?;
                f.write_str("'H")
            }
        }
    }
}

impl Display for FractionalPart {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.number_of_digits == 0 {
            return Ok(());
        }
        if cfg!(feature = "itoa") {
            let mut buf = itoa::Buffer::new();
            write!(f, ".{:0>width$}",
                buf.format(self.fractional_value),
                width = self.number_of_digits as usize
            )
        } else {
            write!(f, ".{:0>width$}",
                self.fractional_value,
                width = self.number_of_digits as usize
            )
        }
    }

}
