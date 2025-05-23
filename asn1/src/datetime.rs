//! The `DATE-TIME` type
//!
//! You can parse, print, compare, and sort `DATE-TIME` values:
//!
//! ```rust
//! use asn1::datetime::DATE_TIME;
//! use std::str::FromStr;
//!
//! let d1 = DATE_TIME::from_str("2022-04-23T20:19:18").unwrap();
//! let d2 = DATE_TIME::new(2022, 04, 23, 20, 19, 18);
//! let d3 = DATE_TIME::new(2022, 04, 24, 20, 19, 18);
//! let d4 = DATE_TIME::new(2022, 04, 23, 20, 19, 19);
//! assert_eq!(d1.date.year, 2022);
//! assert_eq!(d1.date.month, 4);
//! assert_eq!(d1.date.day, 23);
//! assert_eq!(d1.time.hour, 20);
//! assert_eq!(d1.time.minute, 19);
//! assert_eq!(d1.time.second, 18);
//! assert_eq!(d1, d2);
//! assert!(d3 > d2);
//! assert!(d4 > d2);
//! ```
use crate::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use crate::{ISO8601Timestampable, X690KnownSize};
use crate::utils::unlikely;
use crate::X690Validate;
use std::fmt::Display;
use std::str::FromStr;
use crate::utctime::UTCTime;
use crate::gentime::GeneralizedTime;
use crate::date::DATE;
use crate::time_of_day::TIME_OF_DAY;

/// ASN.1 `DATE-TIME`
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct DATE_TIME {
    pub date: DATE,
    pub time: TIME_OF_DAY,
}

impl DATE_TIME {

    /// Create a new `DATE_TIME`
    #[inline]
    pub const fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        DATE_TIME {
            date: DATE { year, month, day },
            time: TIME_OF_DAY { hour, minute, second },
        }
    }

    /// Determine if the `DATE-TIME` is zero-valued, which is
    /// `0000-01-01T00:00:00` or the invalid value `0000-00-00T00:00:00`
    /// (invalid because there is no month 0 or day 0).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.date.is_zero() && self.time.is_zero()
    }

    /// Convert to a string of decimal digits only.
    ///
    /// This is intentionally designed to be suitable as an encoding of this
    /// abstract value as the content octets of a value according to the
    /// Basic Encoding Rules (BER), Distinguished Encoding Rules (DER), or
    /// Canonical Encoding Rules (CER) according to ITU-T Recommendation X.690.
    pub fn to_num_str(&self) -> String {
        if cfg!(feature = "itoa") {
            let mut buf1 = itoa::Buffer::new();
            let mut buf2 = itoa::Buffer::new();
            let mut buf3 = itoa::Buffer::new();
            let mut buf4 = itoa::Buffer::new();
            let mut buf5 = itoa::Buffer::new();
            let mut buf6 = itoa::Buffer::new();
            // TODO: Could you optimize encoding of single-digit values?
            format!("{:0>4}{:0>2}{:0>2}{:0>2}{:0>2}{:0>2}",
                buf1.format(self.date.year % 10000),
                buf2.format(self.date.month % 100),
                buf3.format(self.date.day % 100),
                buf4.format(self.time.hour % 100),
                buf5.format(self.time.minute % 100),
                buf6.format(self.time.second % 100),
            )
        } else {
            format!("{:04}{:02}{:02}{:02}{:02}{:02}",
                self.date.year % 10000,
                self.date.month % 100,
                self.date.day % 100,
                self.time.hour % 100,
                self.time.minute % 100,
                self.time.second % 100,
            )
        }
    }

    /// Convert from a string of decimal digits only.
    ///
    /// This is intentionally designed to be suitable as an decoding of this
    /// abstract value from the content octets of a value according to the
    /// Basic Encoding Rules (BER), Distinguished Encoding Rules (DER), or
    /// Canonical Encoding Rules (CER) according to ITU-T Recommendation X.690.
    pub fn try_from_num_str(s: &str) -> ASN1Result<Self> {
        if unlikely(s.len() != 14) {
            // "YYYYMMDDHHMMSS".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let date = DATE::try_from_num_str(&s[0..8])?;
        let time = crate::TIME_OF_DAY::try_from_num_str(&s[8..])?;
        return Ok(DATE_TIME { date, time });
    }

}

impl X690KnownSize for DATE_TIME {

    /// Returns 14. The X.690 encoding of a `DATE-TIME` is always 14 bytes long.
    fn x690_size (&self) -> usize {
        14
    }

}

impl X690Validate for DATE_TIME {

    /// Validate the X.690 encoding (BER, CER, or DER) for a `DATE-TIME` value.
    /// This takes the content octets ("value") of the X.690 Tag-Length-Value.
    fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()> {
        if content_octets.len() != 14 { // 19511014153000 (X.690 strips the hyphens, colon and "T")
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        DATE::validate_x690_encoding(&content_octets[0..8])?;
        TIME_OF_DAY::validate_x690_encoding(&content_octets[8..])
    }

}

impl ISO8601Timestampable for DATE_TIME {

    /// Print the `DATE-TIME` value as an ISO 8601 Timestamp.
    /// (This is actually just an alias to [DATE_TIME::to_string], because a
    /// `DATE-TIME` is already an ISO 8601 timestamp.)
    ///
    /// NOTE: There is not supposed to be a "Z" at the end of this, nor should
    /// there be a UTC offset. ITU-T Recommendation X.680 defines the
    /// `DATE-TIME` type as being in local time with no UTC offset indication.
    /// For this reason, I recommend against the use of `DATE-TIME`.
    #[inline]
    fn to_iso_8601_string (&self) -> String {
        self.to_string()
    }

}

impl From<GeneralizedTime> for DATE_TIME {
    #[inline]
    fn from(other: GeneralizedTime) -> Self {
        let (minute, second) = other.min_and_sec.unwrap_or((0, None));
        DATE_TIME {
            date: DATE {
                year: other.date.year,
                month: other.date.month,
                day: other.date.day,
            },
            time: TIME_OF_DAY {
                hour: other.hour,
                minute,
                second: second.unwrap_or(0),
            },
        }
    }
}

impl From<UTCTime> for DATE_TIME {
    #[inline]
    fn from(other: UTCTime) -> Self {
        DATE_TIME {
            date: DATE {
                year: other.year as u16,
                month: other.month,
                day: other.day,
            },
            time: TIME_OF_DAY {
                hour: other.hour,
                minute: other.minute,
                second: other.second,
            },
        }
    }
}

impl PartialEq<GeneralizedTime> for DATE_TIME {
    #[inline]
    fn eq(&self, other: &GeneralizedTime) -> bool {
        DATE_TIME::from(*other).eq(self)
    }
}

impl PartialEq<UTCTime> for DATE_TIME {
    #[inline]
    fn eq(&self, other: &UTCTime) -> bool {
        DATE_TIME::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for DATE_TIME {
    type Error = ASN1Error;

    /// Parse an abstract value string containing a `DATE-TIME` value. This
    /// expects dashes in the date, colons in the time, and a "T" separating the
    /// two, such as `2021-03-24T12:34:56`.
    ///
    /// X.690 encoding does _not_ use the dashes, colons, or "T". This is the
    /// wrong function for decoding BER, CER, or DER-encoded `DATE` values. Use
    /// [DATE_TIME::try_from_num_str] instead for X.690 decoding.
    #[inline]
    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if unlikely(value_bytes.len() != 19) {
            // "YYYY-MM-DDTHH:MM:SS".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let date = DATE::try_from(&value_bytes[0..10])?;
        let time = crate::TIME_OF_DAY::try_from(&value_bytes[11..19])?;
        return Ok(DATE_TIME { date, time });
    }
}

impl FromStr for DATE_TIME {
    type Err = ASN1Error;

    /// Parse an abstract value string containing a `DATE-TIME` value. This
    /// expects dashes in the date, colons in the time, and a "T" separating the
    /// two, such as `2021-03-24T12:34:56`.
    ///
    /// X.690 encoding does _not_ use the dashes, colons, or "T". This is the
    /// wrong function for decoding BER, CER, or DER-encoded `DATE` values. Use
    /// [DATE_TIME::try_from_num_str] instead for X.690 decoding.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DATE_TIME::try_from(s.as_bytes())
    }
}

impl Display for DATE_TIME {

    /// Prints an abstract value string containing a `DATE-TIME` value. This
    /// will include dashes in the date, colons in the time, and a "T"
    /// separating the two, such as `2021-03-24T12:34:56`.
    ///
    /// X.690 encoding does _not_ use the dashes, colons, or "T". This is the
    /// wrong function for encoding BER, CER, or DER-encoded `DATE-TIME` values.
    /// Use [DATE_TIME::to_num_str] instead for X.690 encoding.
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_display() {
        let x = DATE_TIME::new(2022, 04, 23, 20, 19, 18);
        assert_eq!(format!("{}", x), "2022-04-23T20:19:18");
    }

    #[test]
    fn test_datetime_parse() {
        let x = DATE_TIME::from_str("2022-04-23T20:19:18").unwrap();
        assert_eq!(x.date.year, 2022);
        assert_eq!(x.date.month, 4);
        assert_eq!(x.date.day, 23);
        assert_eq!(x.time.hour, 20);
        assert_eq!(x.time.minute, 19);
        assert_eq!(x.time.second, 18);
    }

    #[test]
    fn test_date_time_equality() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 22, 05, 33);
        let dt2 = DATE_TIME::new(2022, 04, 11, 22, 05, 33);
        assert!(dt2 == dt1);
    }

    #[test]
    fn test_date_time_inequality_1() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 22, 05, 33);
        let dt2 = DATE_TIME::new(2023, 04, 11, 22, 05, 33);
        assert!(dt2 != dt1);
    }

    #[test]
    fn test_date_time_inequality_2() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 22, 05, 33);
        let dt2 = DATE_TIME::new(2022, 04, 11, 23, 05, 33);
        assert!(dt2 != dt1);
    }

    #[test]
    fn test_date_time_ordering_1() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 22, 04, 22);
        let dt2 = DATE_TIME::new(2022, 04, 11, 22, 04, 23);
        assert!(dt2 > dt1);
    }

    #[test]
    fn test_date_time_ordering_2() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 22, 04, 23);
        let dt2 = DATE_TIME::new(2022, 04, 11, 22, 05, 22);
        assert!(dt2 > dt1);
    }

    #[test]
    fn test_date_time_ordering_3() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 22, 05, 22);
        let dt2 = DATE_TIME::new(2022, 04, 11, 23, 04, 22);
        assert!(dt2 > dt1);
    }

    #[test]
    fn test_date_time_ordering_4() {
        let dt1 = DATE_TIME::new(2022, 04, 11, 23, 05, 23);
        let dt2 = DATE_TIME::new(2022, 04, 12, 22, 04, 22);
        assert!(dt2 > dt1);
    }

    #[test]
    fn test_date_time_ordering_5() {
        let dt1 = DATE_TIME::new(2022, 04, 12, 23, 05, 23);
        let dt2 = DATE_TIME::new(2022, 05, 11, 22, 04, 22);
        assert!(dt2 > dt1);
    }

    #[test]
    fn test_date_time_ordering_6() {
        let dt1 = DATE_TIME::new(2022, 05, 12, 23, 05, 23);
        let dt2 = DATE_TIME::new(2023, 04, 11, 22, 04, 22);
        assert!(dt2 > dt1);
    }

    #[test]
    fn test_date_to_and_from_str_1() {
        let dt = DATE_TIME::try_from_num_str("20220304050607").unwrap();
        assert_eq!(dt.to_num_str(), "20220304050607");
    }

    #[test]
    fn test_date_to_and_from_str_2() {
        let dt = DATE_TIME::try_from_num_str("02220304050607").unwrap();
        assert_eq!(dt.to_num_str(), "02220304050607");
    }

}
