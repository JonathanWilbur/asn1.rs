//! The `UTCTime` type
//!
//! You can parse values of `UTCTime` and print them back as strings or convert them
//! to ISO 8601 timestamps:
//!
//! ```rust
//! use asn1::utctime::UTCTime;
//! use std::str::FromStr;
//! use asn1::ISO8601Timestampable;
//!
//! let t1 = UTCTime::from_str("010203040506+0415").unwrap();
//! assert_eq!(t1.to_iso_8601_string(), "2001-02-03T04:05:06+0415");
//! assert_eq!(t1.to_string(), "010203040506+0415");
//! ```
//!
//! There are no functions for comparison or ordering defined in this library,
//! because this is not a time library, and I intentionally want to keep this very
//! core library very slim in terms of dependencies. You can easily pass the fields
//! of the `UTCTime` struct into a time library-specific data structure for
//! equation and ordering.
use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::{UTCOffset, ISO8601Timestampable};
use crate::gentime::GeneralizedTime;
use crate::utils::{get_days_in_month, unlikely};
use crate::utils::macros::parse_uint;
use std::fmt::{Display, Write};
use std::str::FromStr;

/// ASN.1 `UTCTime`
///
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
    /// The year
    /// Yes, `u8`, not `u16`: it is left to the application to determine which century the two-digit year identifies.
    pub year: u8,
    /// The month, where 1 = January
    pub month: u8,
    /// The day, where 1 = the first day of the month
    pub day: u8,
    /// The hour
    pub hour: u8,
    /// The minute
    pub minute: u8,
    /// The second
    pub second: u8,
    /// The Coordinated Universal Time (UTC) Offset
    pub utc_offset: UTCOffset,
}

impl UTCTime {

    /// Create a new `UTCTime`
    #[inline]
    pub const fn new() -> Self {
        UTCTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            utc_offset: UTCOffset::utc()
        }
    }

    /// Return `true` if the `UTCTime` is zeroed (e.g. `00000101000000`) or the
    /// invalid value, `00000000000000`.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.year == 0
            && self.month <= 1
            && self.day <= 1
            && self.hour == 0
            && self.minute == 0
            && self.second == 0
    }

}

impl ISO8601Timestampable for UTCTime {

    /// Convert the `UTCTime` to an ISO 8601 Timestamp string
    #[cfg(feature = "itoa")]
    fn to_iso_8601_string(&self) -> String {
        let mut buf_year = itoa::Buffer::new();
        let mut buf_month = itoa::Buffer::new();
        let mut buf_day = itoa::Buffer::new();
        let mut buf_hour = itoa::Buffer::new();
        let mut buf_minute = itoa::Buffer::new();
        let mut buf_second = itoa::Buffer::new();
        let mut buf_offset_h = itoa::Buffer::new();
        let mut buf_offset_m = itoa::Buffer::new();

        let year = if self.year >= 50 {
            self.year as u16 + 1900
        } else {
            self.year as u16 + 2000
        };

        if !self.utc_offset.is_zero() {
            let sign = if self.utc_offset.hour >= 0 { '+' } else { '-' };
            return format!(
                "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}{}{:0>2}{:0>2}",
                buf_year.format(year),
                buf_month.format(self.month),
                buf_day.format(self.day),
                buf_hour.format(self.hour),
                buf_minute.format(self.minute),
                buf_second.format(self.second),
                sign,
                buf_offset_h.format(self.utc_offset.hour.abs()),
                buf_offset_m.format(self.utc_offset.minute),
            );
        }
        return format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
            buf_year.format(year),
            buf_month.format(self.month),
            buf_day.format(self.day),
            buf_hour.format(self.hour),
            buf_minute.format(self.minute),
            buf_second.format(self.second),
        );
    }

    /// Convert the `UTCTime` to an ISO 8601 Timestamp string
    #[cfg(not(feature = "itoa"))]
    fn to_iso_8601_string(&self) -> String {
        if !self.utc_offset.is_zero() {
            let sign = if self.utc_offset.hour >= 0 { '+' } else { '-' };
            return format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}{:02}{:02}",
                if self.year >= 50 { self.year as u16 + 1900 } else { self.year as u16 + 2000 },
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
                sign,
                self.utc_offset.hour.abs(),
                self.utc_offset.minute,
            );
        }
        return format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            if self.year >= 50 { self.year as u16 + 1900 } else { self.year as u16 + 2000 },
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        );
    }

}

impl Default for UTCTime {

    /// Create a zeroed `UTCTime` of `00000101000000Z`.
    #[inline]
    fn default() -> Self {
        UTCTime {
            year: 0,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            utc_offset: UTCOffset::default(),
        }
    }
}

impl From<GeneralizedTime> for UTCTime {
    #[inline]
    fn from(other: GeneralizedTime) -> Self {
        let (minute, second) = other.min_and_sec.unwrap_or((0, None));
        UTCTime {
            year: (other.date.year % 100) as u8,
            month: other.date.month,
            day: other.date.day,
            hour: other.hour,
            minute,
            second: second.unwrap_or(0),
            utc_offset: UTCOffset::default(),
        }
    }
}

impl PartialEq<GeneralizedTime> for UTCTime {
    #[inline]
    fn eq(&self, other: &GeneralizedTime) -> bool {
        UTCTime::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for UTCTime {
    type Error = ASN1Error;

    /// Convert a `UTCTime` value from ASCII
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        let len = b.len();
        if unlikely(len < 10) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely(len > 17) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely(!b[0..10].is_ascii()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        // Note that we MUST check for ASCII before indexing into a string.
        let s = unsafe { // Safe because we check for ASCII above.
            std::str::from_utf8_unchecked(&b)
        };
        let mut ret = UTCTime::new();
        ret.year = parse_uint!(u8, &b[0..2], &s[0..2], ASN1ErrorCode::invalid_year);
        ret.month = parse_uint!(u8, &b[2..4], &s[2..4], ASN1ErrorCode::invalid_month);
        ret.day = parse_uint!(u8, &b[4..6], &s[4..6], ASN1ErrorCode::invalid_month);
        ret.hour = parse_uint!(u8, &b[6..8], &s[6..8], ASN1ErrorCode::invalid_hour);
        ret.minute = parse_uint!(u8, &b[8..10], &s[8..10], ASN1ErrorCode::invalid_minute);
        if unlikely(ret.month == 0 || ret.month > 12) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        let year = if unlikely(ret.year >= 50) { ret.year as u16 + 1900 } else { ret.year as u16 + 2000 };
        let max_day: u8 = get_days_in_month(year, ret.month);
        if unlikely(ret.day == 0 || ret.day > max_day) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        if unlikely(ret.hour > 23) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if unlikely(ret.minute > 59) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if (len > 12) && b[10].is_ascii_digit() {
            // Seconds component is present.
            if unlikely(!b[11].is_ascii_digit()) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            ret.second = parse_uint!(u8, &b[10..12], &s[10..12], ASN1ErrorCode::invalid_minute);
            if unlikely(ret.second > 59) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
        }
        if b[len - 1] != b'Z' {
            if unlikely((b[len - 5] != b'+') && (b[len - 5] != b'-')) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            if unlikely(b[len - 4..len].iter().any(|by| !by.is_ascii_digit())) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            let offset_hour = if cfg!(feature = "atoi_simd") {
                atoi_simd::parse_skipped(&b[len-5..len-2])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
            } else {
                i8::from_str(&s[len-5..len-2])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
            };
            // I believe ISO 8601 allows hours up to 15.
            if unlikely(offset_hour.abs() > 15) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            let offset_minute = parse_uint!(u8, &b[len-2..len], &s[len-2..len], ASN1ErrorCode::invalid_time_offset);
            if unlikely(offset_minute > 59) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            ret.utc_offset = UTCOffset {
                hour: offset_hour,
                minute: offset_minute,
            };
        }
        Ok(ret)
    }
}

impl FromStr for UTCTime {
    type Err = ASN1Error;

    /// Convert a `UTCTime` value from ASCII
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UTCTime::try_from(s.as_bytes())
    }
}

// This trait MUST NOT be implemented for `UTCTime`. In addition to the
// option for it to be encoded in constructed and indefinite length form in BER,
// it MUST be converted to UTC time ("Z") when CER or DER-encoded.
// impl X690KnownSize for GeneralizedTime {}

impl Display for UTCTime {

    /// Print a `UTCTime` according to the abstract syntax, such as
    /// `20210304123456Z`
    #[cfg(feature = "itoa")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf_year = itoa::Buffer::new();
        let mut buf_month = itoa::Buffer::new();
        let mut buf_day = itoa::Buffer::new();
        let mut buf_hour = itoa::Buffer::new();
        let mut buf_minute = itoa::Buffer::new();
        let mut buf_second = itoa::Buffer::new();
        let mut buf_offset_m = itoa::Buffer::new();

        write!(f, "{:0>2}{:0>2}{:0>2}{:0>2}{:0>2}{:0>2}",
            buf_year.format(self.year),
            buf_month.format(self.month),
            buf_day.format(self.day),
            buf_hour.format(self.hour),
            buf_minute.format(self.minute),
            buf_second.format(self.second),
        )?;
        if self.utc_offset.is_zero() {
            f.write_char('Z')
        } else {
            write!(f, "{:+03}{:0>2}",
                self.utc_offset.hour,
                buf_offset_m.format(self.utc_offset.minute),
            )
        }
    }

    /// Print a `UTCTime` according to the abstract syntax, such as
    /// `20210304123456Z`
    #[cfg(not(feature = "itoa"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}{:02}{:02}{:02}{:02}{:02}",
            self.year % 100,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )?;
        if self.utc_offset.is_zero() {
            f.write_char('Z')
        } else {
            write!(f, "{:+03}{:02}", self.utc_offset.hour, self.utc_offset.minute)
        }
    }

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{UTCTime, ISO8601Timestampable};

    #[test]
    fn utc_time_display_1() {
        let t = UTCTime{
            year: 22,
            month: 11,
            day: 10,
            hour: 9,
            minute: 8,
            second: 7,
            ..Default::default()
        };
        assert_eq!(format!("{}", t), "221110090807Z");
    }

    #[test]
    fn utc_time_valid() {
        let subtests = [
            [ "0102030405Z", "2001-02-03T04:05:00Z", "010203040500Z" ],
            [ "010203040506Z", "2001-02-03T04:05:06Z", "010203040506Z" ],
            [ "0102030405-0400", "2001-02-03T04:05:00-0400", "010203040500-0400" ],
            [ "010203040506-0400", "2001-02-03T04:05:06-0400", "010203040506-0400" ],
            [ "0102030405+0400", "2001-02-03T04:05:00+0400", "010203040500+0400" ],
            [ "010203040506+0400", "2001-02-03T04:05:06+0400", "010203040506+0400" ],
            // Minute-specific timezone offsets
            [ "0102030405-0415", "2001-02-03T04:05:00-0415", "010203040500-0415" ],
            [ "010203040506-0415", "2001-02-03T04:05:06-0415", "010203040506-0415" ],
            [ "0102030405+0415", "2001-02-03T04:05:00+0415", "010203040500+0415" ],
            [ "010203040506+0415", "2001-02-03T04:05:06+0415", "010203040506+0415" ],
        ];
        for [valid_utctime, should_be, should_be_str] in subtests {
            let ut = UTCTime::from_str(valid_utctime).expect(valid_utctime);
            assert_eq!(ut.to_iso_8601_string(), should_be);
            assert_eq!(ut.to_string(), should_be_str);
        }
    }

    #[test]
    fn utc_time_invalid() {
        let subtests = [
            [ "21030303030303003030030030300303003003030303330033" ],
            [ "21150204Z" ],
            [ "21016204Z" ],
            [ "21020329Z" ],
            [ "2102032067Z" ],
            [ "210203205967Z" ],
            [ "21" ],
            [ "2102" ],
            [ "210203" ],
            [ "21020304.05.06" ],
            [ "21020304,05,06" ],
            [ "21020304,05,06" ],
            [ "21020320.25-0" ],
            [ "21020320.25+0" ],
            [ "21020320.25-081" ],
            [ "21020320.25+081" ],
            [ "21020320.25-08105" ],
            [ "21020320.25+08105" ],
            [ "21020320.25-0810Z" ],
            [ "21020320.25+0810Z" ],
        ];
        for [invalid_utctime] in subtests {
            UTCTime::from_str(invalid_utctime).expect_err(invalid_utctime);
        }
    }

}
