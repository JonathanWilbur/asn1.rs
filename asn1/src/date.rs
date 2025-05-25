//! The `DATE` type
//!
//! You can parse, print, compare, and sort `DATE` values:
//!
//! ```rust
//! use asn1::date::DATE;
//! use std::str::FromStr;
//!
//! let d1 = DATE::from_str("2022-04-23").unwrap();
//! let d2 = DATE::new(2022, 04, 23);
//! let d3 = DATE::new(2022, 04, 24);
//! assert_eq!(d1.year, 2022);
//! assert_eq!(d1.month, 4);
//! assert_eq!(d1.day, 23);
//! assert_eq!(d1.to_string(), "2022-04-23");
//! assert_eq!(d1, d2);
//! assert!(d3 > d2);
//! ```
use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::{X690KnownSize, X690Validate};
use crate::utils::{get_days_in_month, unlikely};
use crate::datetime::DATE_TIME;
use crate::gentime::GeneralizedTime;
use crate::utctime::UTCTime;
use crate::ASN1Result;
use std::fmt::Display;
use std::str::FromStr;

/// ASN.1 `DATE`
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct DATE {
    /// The Year
    pub year: u16,
    /// The Month
    pub month: u8,
    /// The Day
    pub day: u8,
}

impl DATE {

    /// Create a new `DATE`
    #[inline]
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        DATE { year, month, day }
    }

    /// Determine if the `DATE` is zero-valued, which is `0000-01-01` or the
    /// invalid value `0000-00-00` (invalid because there is no month 0 or day
    /// 0).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.year == 0 && self.month <= 1 && self.day <= 1
    }

    /// Convert to a string of decimal digits only.
    ///
    /// This is intentionally designed to be suitable as an encoding of this
    /// abstract value as the content octets of a value according to the
    /// Basic Encoding Rules (BER), Distinguished Encoding Rules (DER), or
    /// Canonical Encoding Rules (CER) according to ITU-T Recommendation X.690.
    pub fn to_num_str(&self) -> String {
        format!("{:04}{:02}{:02}", self.year % 10000, self.month, self.day)
    }

    /// Convert from a string of decimal digits only.
    ///
    /// This is intentionally designed to be suitable as an decoding of this
    /// abstract value from the content octets of a value according to the
    /// Basic Encoding Rules (BER), Distinguished Encoding Rules (DER), or
    /// Canonical Encoding Rules (CER) according to ITU-T Recommendation X.690.
    pub fn from_num_str(s: &str) -> ASN1Result<Self> {
        let b = s.as_bytes();
        if b.len() != 8 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely(!b.is_ascii()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let year: u16;
        let month: u8;
        let day: u8;
        #[cfg(feature = "atoi_simd")]
        {
            year = atoi_simd::parse_pos::<u16>(&b[0..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
            month = atoi_simd::parse_pos::<u8>(&b[4..6])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
            day = atoi_simd::parse_pos::<u8>(&b[6..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        }
        #[cfg(not(feature = "atoi_simd"))]
        {
            year = u16::from_str(&s[0..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
            month = u8::from_str(&s[4..6])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
            day = u8::from_str(&s[6..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        }
        Ok(DATE { year, month, day })
    }

}

impl Default for DATE {

    /// Create a new default `DATE` value, which is zeroed (`0000-01-01`).
    #[inline]
    fn default() -> Self {
        DATE {
            year: 0,
            month: 1,
            day: 1,
        }
    }
}

impl X690KnownSize for DATE {

    /// Returns 8. The X.690 encoding of a `DATE` is always 8 bytes long.
    fn x690_size (&self) -> usize {
        8
    }

}

impl From<GeneralizedTime> for DATE {

    /// Extracts the `date` from a `GeneralizedTime`
    #[inline]
    fn from(other: GeneralizedTime) -> Self {
        other.date
    }
}

impl From<DATE_TIME> for DATE {

    /// Extracts the `date` from a `DATE_TIME`
    #[inline]
    fn from(other: DATE_TIME) -> Self {
        other.date
    }
}

impl From<UTCTime> for DATE {

    /// Extracts the `date` from a `UTCTime`
    #[inline]
    fn from(other: UTCTime) -> Self {
        DATE {
            /* The conversion below was taken from ITU Recommendation X.509
            (2019), Section 7.2.1. */
            year: if other.year >= 50 {
                1900 + other.year as u16
            } else {
                2000 + other.year as u16
            },
            month: other.month,
            day: other.day,
        }
    }
}

impl PartialEq<GeneralizedTime> for DATE {
    #[inline]
    fn eq(&self, other: &GeneralizedTime) -> bool {
        DATE::from(*other).eq(self)
    }
}

impl PartialEq<UTCTime> for DATE {
    #[inline]
    fn eq(&self, other: &UTCTime) -> bool {
        DATE::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for DATE {
    type Error = ASN1Error;

    /// Parse an abstract value string containing a `DATE` value. This expects
    /// dashes in the date, such as `2001-01-24`.
    ///
    /// X.690 encoding does _not_ use the dashes. This is the wrong function for
    /// decoding BER, CER, or DER-encoded `DATE` values. Use
    /// [DATE::from_num_str] instead for X.690 decoding.
    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if unlikely(value_bytes.len() != 10) { // "YYYY-MM-DD".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely(value_bytes[4] != b'-' || value_bytes[7] != b'-') {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let year: u16;
        let month: u8;
        let day: u8;
        #[cfg(feature = "atoi_simd")]
        {
            year = atoi_simd::parse_pos::<u16>(&value_bytes[0..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            month = atoi_simd::parse_pos::<u8>(&value_bytes[5..7])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            day = atoi_simd::parse_pos::<u8>(&value_bytes[8..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        }
        #[cfg(not(feature = "atoi_simd"))]
        {
            if unlikely(!value_bytes.is_ascii()) {
                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
            }
            // We already checked for ASCII above.
            let str_ = unsafe { std::str::from_utf8_unchecked(&value_bytes) };
            year = u16::from_str(&str_[0..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            month = u8::from_str(&str_[5..7])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            day = u8::from_str(&str_[8..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        }
        if unlikely(month > 12 || month == 0) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        let max_day = get_days_in_month(year, month);
        if unlikely(day > max_day || day == 0) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        Ok(DATE { year, month, day })
    }
}

impl FromStr for DATE {
    type Err = ASN1Error;

    /// Parse an abstract value string containing a `DATE` value. This expects
    /// dashes in the date, such as `2001-01-24`.
    ///
    /// X.690 encoding does _not_ use the dashes. This is the wrong function for
    /// decoding BER, CER, or DER-encoded `DATE` values. Use
    /// [DATE::from_num_str] instead for X.690 decoding.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DATE::try_from(s.as_bytes())
    }
}

impl Display for DATE {

    /// Prints an abstract value string containing a `DATE` value. This will
    /// include dashes in the date, such as `2001-01-24`.
    ///
    /// X.690 encoding does _not_ use the dashes. This is the wrong function for
    /// encoding BER, CER, or DER-encoded `DATE` values. Use [DATE::to_num_str]
    /// instead for X.690 encoding.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year % 10000, self.month, self.day)
    }
}

impl X690Validate for DATE {

    /// Validate the X.690 encoding (BER, CER, or DER) for a `DATE` value.
    /// This takes the content octets ("value") of the X.690 Tag-Length-Value.
    fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()> {
        if content_octets.len() != 8 { // YYYYMMDD (X.690 strips the dashes)
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if !content_octets.iter().all(|b| b.is_ascii_digit()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let s = unsafe { std::str::from_utf8_unchecked(&content_octets) };
        let year: u16;
        let month: u8;
        let day: u8;
        #[cfg(feature = "atoi_simd")]
        {
            year = atoi_simd::parse_pos::<u16>(&content_octets[0..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            month = atoi_simd::parse_pos::<u8>(&content_octets[4..6])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            day = atoi_simd::parse_pos::<u8>(&content_octets[6..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        }
        #[cfg(not(feature = "atoi_simd"))]
        {
            year = u16::from_str(&s[0..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
            month = u8::from_str(&s[4..6])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
            day = u8::from_str(&s[6..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        }
        if month > 12 || month == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        let max_day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            // This isn't technically correct leap-year handling, but it should be good for the next 175 years or so.
            2 => if year % 4 > 0 { 28 } else { 29 },
            _ => 30,
        };
        if day == 0 || day > max_day {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_display() {
        let x = DATE::new(2022, 04, 23);
        assert_eq!(format!("{}", x), "2022-04-23");
    }

    #[test]
    fn test_date_parse() {
        let x = DATE::from_str("2022-04-23").unwrap();
        assert_eq!(x.year, 2022);
        assert_eq!(x.month, 4);
        assert_eq!(x.day, 23);
    }

    #[test]
    fn test_date_ordering_1() {
        let date1 = DATE::new(2022, 04, 22);
        let date2 = DATE::new(2022, 04, 23);
        assert!(date2 > date1);
    }

    #[test]
    fn test_date_ordering_2() {
        let date1 = DATE::new(2022, 04, 23);
        let date2 = DATE::new(2022, 05, 22);
        assert!(date2 > date1);
    }

    #[test]
    fn test_date_ordering_3() {
        let date1 = DATE::new(2022, 06, 23);
        let date2 = DATE::new(2023, 05, 22);
        assert!(date2 > date1);
    }

    #[test]
    fn test_date_to_and_from_str_1() {
        let date = DATE::from_num_str("20220304").unwrap();
        assert_eq!(date.to_num_str(), "20220304");
    }

    #[test]
    fn test_date_to_and_from_str_2() {
        let date = DATE::from_num_str("02220304").unwrap();
        assert_eq!(date.to_num_str(), "02220304");
    }

}
