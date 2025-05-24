//! The `GeneralizedTime` type
//!
//! You can parse values of `UTCTime` and print them back as strings or convert
//! them to ISO 8601 timestamps:
//!
//! ```rust
//! use asn1::gentime::GeneralizedTime;
//! use asn1::ISO8601Timestampable;
//! use std::str::FromStr;
//!
//! let t1 = GeneralizedTime::from_str("20210203040607.32895292-0503").unwrap();
//! assert_eq!(t1.to_iso_8601_string(), "2021-02-03T04:06:07.32895292-0503");
//! assert_eq!(t1.to_string(), "20210203040607.32895292-0503");
//! ```
use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::{UTCOffset, ISO8601Timestampable};
use crate::utils::{get_days_in_month, unlikely};
use crate::utils::macros::parse_uint;
use crate::utctime::UTCTime;
use crate::date::DATE;
use std::cmp::min;
use std::fmt::{Display, Write};
use std::str::FromStr;

/// ASN.1 `GeneralizedTime`
///
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
    /// The date portion of the `GeneralizedTime`
    pub date: DATE,
    /// `None` = Local time
    /// `Some`, where the offset is zero: UTC time
    /// `Some`, where the offset is non-zero: UTC difference
    pub utc_offset: Option<UTCOffset>,

    /// The hour
    pub hour: u8,

    /// The minute and second
    /// Not only does this enforce correctness, but it also makes the struct
    /// 24 bytes instead of 28, meaning that it can be read or copied more
    /// efficiently on 64-bit systems.
    pub min_and_sec: Option<(u8, Option<u8>)>,

    /// The least significant four bits are the precision, in terms of decimal
    /// digits, of the fraction. This value will be 0 if there were no
    /// fractional digits.
    pub flags: u8,

    /// The fraction value. The digits of precision is stored in the `flags`
    /// field.
    pub fraction: u32,
}

impl GeneralizedTime {

    /// Create a new `GeneralizedTime`
    #[inline]
    pub const fn new() -> Self {
        GeneralizedTime {
            // This is the same as default(). I just copied the literal so this could be const.
            date: DATE { year: 0, month: 1, day: 1 },
            flags: 0,
            hour: 0,
            min_and_sec: None,
            fraction: 0,
            utc_offset: None,
        }
    }

    /// Return `true` if this `GeneralizedTime` is "zeroed" meaning that the
    /// date is 0000-01-01 (or the invalid date 0000-00-00) and the time is
    /// 00:00:00, with all time components defaulting to zero if absent.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        // Using unwrap_or() would have made this whole function much cleaner,
        // but it is not const for some reason.
        let first_part_is_zero = self.date.year == 0
            && self.date.month <= 1
            && self.date.day <= 1
            && self.hour == 0;
        if !first_part_is_zero {
            return false; // It is already non-zero. We don't need to check further.
        }
        if let Some((minute, second)) = self.min_and_sec {
            if minute != 0 {
                return false;
            }
            if let Some(sec) = second {
                if sec != 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Returns `true` if the `GeneralizedTime` is Coordinated Universal Time
    /// (UTC)
    #[inline]
    pub const fn is_utc(&self) -> bool {
        // This would have been more elegant, but not const:
        // self.utc_offset.is_some_and(|offset| offset.is_zero())
        if let Some(offset) = self.utc_offset {
            offset.is_zero()
        } else {
            false
        }
    }

    /// Get the number of digits of precision in the fractional component of the
    /// `GeneralizedTime`
    #[inline]
    pub const fn get_fraction_precision_digits(&self) -> u8 {
        // This implementation only handles up to nano-second precision, hence % 10.
        (self.flags & 0b0000_1111) % 10
    }

    /// Returns `true` if this `GeneralizedTime` has a fractional component.
    #[inline]
    pub const fn has_fraction(&self) -> bool {
        self.get_fraction_precision_digits() > 0
    }

}

impl ISO8601Timestampable for GeneralizedTime {

    /// Convert this `GeneralizedTime` string to an ISO 8601 String.
    ///
    /// Fractional seconds will only be displayed if the original
    /// GeneralizedTime used fractional seconds (not fractional hours or
    /// minutes).
    fn to_iso_8601_string (&self) -> String {
        let mut fraction_string: Option<String> = None;
        let (mut minute, mut second) = self.min_and_sec.unwrap_or((0, None));
        let frac_precision = self.get_fraction_precision_digits();
        if frac_precision > 0 {
            let num: f64 = self.fraction.into();
            let denom: f64 = 10.0f64.powi(frac_precision as i32);
            if unlikely(self.min_and_sec.is_none()) {
                // Fractional hours
                let secondsf = (num / denom) * 3600.0;
                minute = (secondsf / 60.0).floor() as u8;
                second = Some((secondsf.round() % 60.0) as u8 );
            } else if unlikely(second.is_none()) {
                // Fractional minutes
                let secondsf = (num / denom) * 60.0;
                second = Some(secondsf.round() as u8);
            } else {
                // Fractional seconds
                fraction_string = Some(format!(".{:0>width$}",
                    self.fraction,
                    width = frac_precision as usize
                ));
            }
        }

        if self.is_utc() {
            return format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}Z",
                self.date.year,
                self.date.month,
                self.date.day,
                self.hour,
                minute,
                second.unwrap_or(0),
                fraction_string.unwrap_or(String::new()),
            );
        }
        if let Some(offset) = &self.utc_offset {
            return format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}{:+03}{:02}",
                self.date.year,
                self.date.month,
                self.date.day,
                self.hour,
                minute,
                second.unwrap_or(0),
                fraction_string.unwrap_or(String::new()),
                offset.hour,
                offset.minute,
            );
        }
        return format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}",
            self.date.year,
            self.date.month,
            self.date.day,
            self.hour,
            minute,
            second.unwrap_or(0),
            fraction_string.unwrap_or(String::new()),
        );
    }

}

impl Default for GeneralizedTime {

    /// Create a zeroed `GeneralizedTime`
    #[inline]
    fn default() -> Self {
        GeneralizedTime {
            date: DATE::default(),
            flags: 0,
            hour: 0,
            min_and_sec: None,
            fraction: 0,
            utc_offset: None,
        }
    }
}

impl From<UTCTime> for GeneralizedTime {
    #[inline]
    fn from(other: UTCTime) -> Self {
        let date = DATE::from(other);
        GeneralizedTime {
            date,
            flags: 0,
            hour: other.hour,
            min_and_sec: Some((other.minute, Some(other.second))),
            fraction: 0,
            utc_offset: None,
        }
    }
}

impl From<DATE> for GeneralizedTime {

    /// **WARNING**: This sets the GeneralizedTime to be in local time!
    #[inline]
    fn from(other: DATE) -> Self {
        GeneralizedTime {
            date: other,
            flags: 0, // Local time, not UTC.
            hour: 0,
            min_and_sec: None,
            fraction: 0,
            utc_offset: None,
        }
    }
}

impl TryFrom<&[u8]> for GeneralizedTime {
    type Error = ASN1Error;

    /// Decode a `GeneralizedTime` from ASCII.
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        let len = b.len();
        if unlikely(len < 10) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        // There is technically no limit on how big a GeneralizedTime can be, but
        // we have to set a reasonable limit here. This accomodates nanoseconds.
        if unlikely(len > 32) {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
        }
        if unlikely(!b.is_ascii()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        // Note that we MUST check for ASCII before indexing into a string.
        let s = unsafe { // Safe because we check for ASCII above.
            std::str::from_utf8_unchecked(b)
        };
        let mut ret = GeneralizedTime::new();
        ret.date.year = parse_uint!(u16, &b[0..4], &s[0..4], ASN1ErrorCode::invalid_year);
        ret.date.month = parse_uint!(u8, &b[4..6], &s[4..6], ASN1ErrorCode::invalid_month);
        ret.date.day = parse_uint!(u8, &b[6..8], &s[6..8], ASN1ErrorCode::invalid_day);
        ret.hour = parse_uint!(u8, &b[8..10], &s[8..10], ASN1ErrorCode::invalid_hour);
        if unlikely(ret.date.month == 0 || ret.date.month > 12) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        let max_day: u8 = get_days_in_month(ret.date.year, ret.date.month);
        if unlikely(ret.date.day == 0 || ret.date.day > max_day) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        if unlikely(ret.hour > 23) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if (len >= 12) && b[10].is_ascii_digit() {
            let minute = parse_uint!(u8, &b[10..12], &s[10..12], ASN1ErrorCode::invalid_minute);
            if unlikely(minute > 59) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
            }
            ret.min_and_sec = Some((minute, None));
        }

        if let Some((m, _)) = ret.min_and_sec {
            // Normal "if"s cannot be combined with "if let"s.
            if (len >= 14) && b[12].is_ascii_digit() {
                // Seconds component is present.
                let second = parse_uint!(u8, &b[12..14], &s[12..14], ASN1ErrorCode::invalid_second);
                if unlikely(second > 59) {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                }
                ret.min_and_sec = Some((m, Some(second)));
            }
        }

        let mut i: usize = match ret.min_and_sec {
            None => 10,
            Some((_, s)) => if s.is_some() { 14 } else { 12 },
        };
        if (len > (i + 1)) && ((b[i] == b'.') || (b[i] == b',')) {
            i += 1;
            let start = i;
            while i < len && b[i].is_ascii_digit() {
                i += 1;
            }
            let end = min(i, start + 9); // We can only tolerate 9 digits of precision.
            let fractional_value = parse_uint!(
                u32,
                &b[start..end],
                &s[start..end],
                ASN1ErrorCode::invalid_fraction_of_seconds
            );
            ret.fraction = fractional_value;
            ret.flags &= 0b1111_0000; // Clear the bottom four bits
            ret.flags |= ((end - start) as u8) % 10;
        }

        let offset_sign = b.get(i);
        if offset_sign.is_some_and(|c| *c == b'Z') {
            // ret.utc = true; // This is the default.
            ret.utc_offset = Some(UTCOffset::utc());
            return Ok(ret); // UTCTime
        }
        if offset_sign.is_none() {
            return Ok(ret); // Local Time
        }

        if unlikely(offset_sign.is_some_and(|c| *c != b'+' && *c != b'-')) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely((len != (i + 3)) && (len != (i + 5))) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        let offset_hour: i8 = {
            #[cfg(feature = "atoi_simd")]
            {
                atoi_simd::parse_skipped(&b[i..i + 3])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
            }
            #[cfg(not(feature = "atoi_simd"))]
            {
                i8::from_str(&s[i..i + 3])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
            }
        };
        // I believe ISO 8601 allows hours up to 15.
        if unlikely(offset_hour.abs() > 15) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        i += 3;
        let offset_minute = if len == (i + 2) {
            parse_uint!(u8, &b[i..i+2], &s[i..i+2], ASN1ErrorCode::invalid_time_offset)
        } else {
            0
        };
        if unlikely(offset_minute > 59) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        ret.utc_offset = Some(UTCOffset {
            hour: offset_hour,
            minute: offset_minute,
        });
        Ok(ret)
    }
}

impl FromStr for GeneralizedTime {
    type Err = ASN1Error;

    /// Decode a `GeneralizedTime` from ASCII.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GeneralizedTime::try_from(s.as_bytes())
    }
}

// This trait MUST NOT be implemented for `GeneralizedTime`. In addition to the
// option for it to be encoded in constructed and indefinite length form in BER,
// it MUST be converted to UTC time ("Z") when CER or DER-encoded.
// impl X690KnownSize for GeneralizedTime {}

impl Display for GeneralizedTime {

    /// Prints a `GeneralizedTime`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}{:02}{:02}{:02}",
            self.date.year % 10000,
            self.date.month,
            self.date.day,
            self.hour,
        )?;
        if let Some((min, maybe_sec)) = &self.min_and_sec {
            write!(f, "{:02}", min)?;
            if let Some(sec) = &maybe_sec {
                write!(f, "{:02}", sec)?;
            }
        }

        let frac_digits = self.get_fraction_precision_digits();
        if frac_digits > 0 {
            write!(f, ".{:0>width$}", self.fraction, width = frac_digits as usize)?;
        }
        match &self.utc_offset {
            Some(offset) => if offset.is_zero() {
                f.write_char('Z')
            } else {
                write!(f, "{:+03}{:02}", offset.hour, offset.minute)
            },
            None => Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{GeneralizedTime, ISO8601Timestampable};

    #[test]
    fn gen_time_from_str_accepts_fractional_seconds() {
        let input = "19960415203000.0";
        GeneralizedTime::from_str(input).unwrap();
    }

    #[test]
    fn gen_time_from_str_accepts_fractional_seconds_and_timezone() {
        let input = "19960415203000.0Z";
        GeneralizedTime::from_str(input).unwrap();
    }

    #[test]
    fn gen_time_valid() {
        let subtests = [
            // LOCAL TIME: This only works for me in Florida. This should be commented out.
            // [ "2021020304", "2021-02-03T09:00:00.000Z" ],
            // The (second) smallest and simplest time.
            [ "2021020304Z", "2021-02-03T04:00:00Z", "2021020304Z" ],
            // With fractional hours
            [ "2021020304.3334Z", "2021-02-03T04:20:00Z", "2021020304.3334Z" ],
            [ "2021020304,3334Z", "2021-02-03T04:20:00Z", "2021020304.3334Z" ],
            [ "2021020304.50Z", "2021-02-03T04:30:00Z", "2021020304.50Z" ],
            [ "2021020304.333333334Z", "2021-02-03T04:20:00Z", "2021020304.333333334Z" ],
            // With fractional minutes
            [ "202102030405.3334Z", "2021-02-03T04:05:20Z", "202102030405.3334Z" ],
            [ "202102030405,3334Z", "2021-02-03T04:05:20Z", "202102030405.3334Z" ],
            // With fractional seconds
            [ "20210203040506.3334Z", "2021-02-03T04:05:06.3334Z", "20210203040506.3334Z" ],
            [ "20210203040506,3334Z", "2021-02-03T04:05:06.3334Z", "20210203040506.3334Z" ],
            // Simple timezone offset
            [ "2021020304-05", "2021-02-03T04:00:00-0500", "2021020304-0500" ],
            [ "2021020304+05", "2021-02-03T04:00:00+0500", "2021020304+0500" ],
            [ "2021020304-0500", "2021-02-03T04:00:00-0500", "2021020304-0500" ],
            // Carry over with offset minutes and fractional hours
            [ "2021020304.25+0815", "2021-02-03T04:15:00+0815", "2021020304.25+0815" ],
            [ "2021020320.25-0815", "2021-02-03T20:15:00-0815", "2021020320.25-0815" ],
            // Minutes with timezone offset
            [ "202102030406-0500", "2021-02-03T04:06:00-0500", "202102030406-0500" ],
            // Seconds with timezone offset
            [ "20210203040607-0500", "2021-02-03T04:06:07-0500", "20210203040607-0500" ],
            // The most complicated examples
            [ "20210203040607.32895292-0503", "2021-02-03T04:06:07.32895292-0503", "20210203040607.32895292-0503" ],
            [ "20210203040607,32895292+0304", "2021-02-03T04:06:07.32895292+0304", "20210203040607.32895292+0304" ],
            // Nanosecond precision
            // [ "20210203040607.123456789-0503", "2021-02-03T04:06:07.123456789-0503" ],
        ];
        for [valid_gentime, should_be, should_be_str] in subtests {
            let gt = GeneralizedTime::from_str(valid_gentime).expect(valid_gentime);
            assert_eq!(gt.to_iso_8601_string(), should_be);
            assert_eq!(gt.to_string(), should_be_str);
        }
    }

    #[test]
    fn gen_time_invalid() {
        let subtests = [
            [ "2021030303030303003030030030300303003003030303330033" ],
            [ "2021150204Z" ],
            [ "2021016204Z" ],
            [ "2021020329Z" ],
            [ "202102032067Z" ],
            [ "20210203205967Z" ],
            [ "2021" ],
            [ "202102" ],
            [ "20210203" ],
            [ "2021020304.05.06" ],
            [ "2021020304,05,06" ],
            [ "2021020304,05,06" ],
            [ "2021020320.25-0" ],
            [ "2021020320.25+0" ],
            [ "2021020320.25-081" ],
            [ "2021020320.25+081" ],
            [ "2021020320.25-08105" ],
            [ "2021020320.25+08105" ],
            [ "2021020320.25-0810Z" ],
            [ "2021020320.25+0810Z" ],
        ];
        for [ invalid_gentime ] in subtests {
            GeneralizedTime::from_str(invalid_gentime).expect_err(invalid_gentime);
        }
    }

}
