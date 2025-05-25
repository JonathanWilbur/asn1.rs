//! The `DURATION` type
//!
//! You can parse and print `DURATION` values (via the `DURATION_EQUIVALENT` struct):
//!
//! ```rust
//! use wildboar_asn1::duration::DURATION_EQUIVALENT;
//! use wildboar_asn1::FractionalPart;
//! use std::str::FromStr;
//!
//! let dur = DURATION_EQUIVALENT::from_str("P5Y6M1W23DT25H65M222.00505S").unwrap();
//! assert_eq!(dur.years, 5);
//! assert_eq!(dur.months, 6);
//! assert_eq!(dur.weeks, 1);
//! assert_eq!(dur.days, 23);
//! assert_eq!(dur.hours, 25);
//! assert_eq!(dur.minutes, 65);
//! assert_eq!(dur.seconds, 222);
//! assert_eq!(dur.to_string(), "P5Y6M1W23DT25H65M222.00505S");
//! ```
use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::{FractionalPart, X690KnownSize};
use core::str;
use std::fmt::Write;
use std::{fmt::Display, str::FromStr, time::Duration};
use crate::utils::{unlikely, likely};

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;
const SECONDS_PER_WEEK: u64 = 7 * SECONDS_PER_DAY;
// Average month length (30.44 days)
const SECONDS_PER_MONTH: u64 = 2_629_746;
// Average year length including leap years (365.24 days)
const SECONDS_PER_YEAR: u64 = 31_556_952;

/// A unit of time found in an ASN.1 `DURATION` value (which itself is an
/// ISO 8601 duration)
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum DurationPart {
    /// Years
    Years,
    /// Months
    Months,
    /// Weeks
    Weeks,
    /// Days
    Days,
    /// Hours
    Hours,
    /// Minutes
    Minutes,
    /// Seconds
    Seconds
}

impl Into<char> for DurationPart {

    /// Convert the duration unit into its ISO 8601 character. For example,
    /// `Years` would be converted to `Y`.
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

/// ASN.1 `DURATION` value (which itself is an ISO 8601 duration)
/// Defined in ITU-T Recommendation X.680, Section 38.4.4.2.
#[derive(Debug, Eq, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct DURATION_EQUIVALENT {
    /// Number of years
    pub years: u32,
    /// Number of months
    pub months: u32,
    /// Number of weeks
    pub weeks: u32,
    /// Number of days
    pub days: u32,
    /// Number of hours
    pub hours: u32,
    /// Number of minutes
    pub minutes: u32,
    /// Number of seconds
    pub seconds: u32,
    /// Fractional part, and the part of the duration to which it applies
    pub fractional_part: Option<(DurationPart, FractionalPart)>,
}

impl TryFrom<char> for DurationPart {
    type Error = ();

    /// Convert from a duration part's unit character (such as 'Y' for years)
    /// Since `M` is used for both months and minutes, `m` (lowercased) is used
    /// for minutes.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'Y' => Ok(DurationPart::Years),
            'M' => Ok(DurationPart::Months),
            'W' => Ok(DurationPart::Weeks),
            'D' => Ok(DurationPart::Days),
            'H' => Ok(DurationPart::Hours),
            'm' => Ok(DurationPart::Minutes),
            'S' => Ok(DurationPart::Seconds),
            _ => Err(())
        }
    }

}

impl Display for DurationPart {

    /// Display a duration part's unit character, such as 'Y' for years
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationPart::Years => f.write_char('Y'),
            DurationPart::Months => f.write_char('M'),
            DurationPart::Weeks => f.write_char('W'),
            DurationPart::Days => f.write_char('D'),
            DurationPart::Hours => f.write_char('H'),
            DurationPart::Minutes => f.write_char('M'),
            DurationPart::Seconds => f.write_char('S'),
        }
    }

}

impl DURATION_EQUIVALENT {

    /// Create a new `DURATION` value.
    #[inline]
    pub const fn new(
        years: u32,
        months: u32,
        weeks: u32,
        days: u32,
        hours: u32,
        minutes: u32,
        seconds: u32,
        fractional_part: Option<(DurationPart, FractionalPart)>,
    ) -> Self {
        DURATION_EQUIVALENT {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            fractional_part,
        }
    }

    /// Return `true` if the `DURATION` value is "zeroed."
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.years == 0
            && self.months == 0
            && self.weeks == 0
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
            && self.fractional_part.is_none()
    }

    /// This method normalizes `DURATION` values by "folding" smaller units of
    /// time into larger ones where there is an unconditional conversion factor.
    /// For example, this method would normalize `PT60S` by "folding" it into
    /// `PT1M` (60 seconds "folded" into one minute).
    ///
    /// One exception is for the folding of hours to days: this method regards
    /// 24 hours as being always equal to one day, ignoring leap seconds. The
    /// rationale for this is that:
    ///
    /// 1. Most people think of 24 hours as being synonymous with one day.
    /// 2. Most years do not have a leap second, and a second is very small in
    ///    comparison to 24-hours, so the difference is almost negligible.
    /// 3. Normalization becomes much more effective at making two duration
    ///    values comparable if we can fold seconds into minutes, minutes,
    ///    to hours, etc. The more "folding" that we can "daisy chain" between
    ///    the units, the more likely that two duration values that look the
    ///    same to a human will also look the same to a computer.
    /// 4. The concept of the leap second itself is deprecated.
    ///
    /// If the attempt to normalize the `DURATION` value _would_ cause an integer
    /// overflow, the original `DURATION` value is return unchanged. It would
    /// therefore fail to be normalized, but this is preferable to the `DURATION`
    /// value being returned being _wrong_.
    pub fn normalize(&self) -> DURATION_EQUIVALENT {
        let mut seconds = self.seconds;
        let mut minutes = match self.minutes.checked_add(self.seconds / 60) {
            Some(m) => m,
            None => return self.clone(),
        };
        seconds %= 60;

        let mut hours = match self.hours.checked_add(minutes / 60) {
            Some(h) => h,
            None => return self.clone(),
        };
        minutes %= 60;

        let mut days = match self.days.checked_add(hours / 24) {
            Some(d) => d,
            None => return self.clone(),
        };
        hours %= 24;

        let weeks = match self.weeks.checked_add(days / 7) {
            Some(w) => w,
            None => return self.clone(),
        };
        days %= 7;

        let mut months = self.months;
        let years = match self.years.checked_add(self.months / 12) {
            Some(y) => y,
            None => return self.clone(),
        };
        months %= 12;

        DURATION_EQUIVALENT {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            fractional_part: self.fractional_part,
        }
    }

    /// Converts the duration to an approximate number of seconds.
    ///
    /// This uses the following approximations:
    /// - 1 year ≈ 365.24 days ≈ 31,556,952 seconds
    /// - 1 month ≈ 30.44 days ≈ 2,629,746 seconds
    /// - 1 week = 7 days = 604,800 seconds
    /// - 1 day = 24 hours = 86,400 seconds
    /// - 1 hour = 60 minutes = 3,600 seconds
    /// - 1 minute = 60 seconds
    pub const fn to_approximate_seconds(&self) -> u64 {
        let mut total: u64 = 0;
        // Overflow of a u64 is not possible.
        total += SECONDS_PER_YEAR * self.years as u64;
        total += SECONDS_PER_MONTH * self.months as u64;
        total += SECONDS_PER_WEEK * self.weeks as u64;
        total += SECONDS_PER_DAY * self.days as u64;
        total += SECONDS_PER_HOUR * self.hours as u64;
        total += SECONDS_PER_MINUTE * self.minutes as u64;
        total += self.seconds as u64;
        if let Some((durpart, frac)) = self.fractional_part {
            let multiplier = match durpart {
                DurationPart::Seconds => 1,
                DurationPart::Minutes => SECONDS_PER_MINUTE,
                DurationPart::Hours => SECONDS_PER_HOUR,
                DurationPart::Days => SECONDS_PER_DAY,
                DurationPart::Weeks => SECONDS_PER_WEEK,
                DurationPart::Months => SECONDS_PER_MONTH,
                DurationPart::Years => SECONDS_PER_YEAR,
            };
            let numerator: u64 = multiplier * frac.fractional_value as u64;
            // The None case is ignored.
            if let Some(denominator) = 10u64.checked_pow(frac.number_of_digits as u32) {
                total += numerator / denominator; // flooring division
            }
        }
        total
    }
}

impl Default for DURATION_EQUIVALENT {

    /// Creates a zeroed duration
    #[inline]
    fn default() -> Self {
        DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: None,
        }
    }
}

impl TryFrom<Duration> for DURATION_EQUIVALENT {
    type Error = std::num::TryFromIntError;

    #[inline]
    fn try_from(other: Duration) -> Result<Self, Self::Error> {
        Ok(DURATION_EQUIVALENT {
            seconds: other.as_secs().try_into()?,
            ..Default::default()
        })
    }
}

const DURATION_COMPONENT_YEARS: u8 = 0b0000_0001;
const DURATION_COMPONENT_MONTHS: u8 = 0b0000_0010;
const DURATION_COMPONENT_WEEKS: u8 = 0b0000_0100;
const DURATION_COMPONENT_DAYS: u8 = 0b0000_1000;
const DURATION_COMPONENT_HOURS: u8 = 0b0001_0000;
const DURATION_COMPONENT_MINUTES: u8 = 0b0010_0000;
const DURATION_COMPONENT_SECONDS: u8 = 0b0100_0000;

impl TryFrom<&[u8]> for DURATION_EQUIVALENT {
    type Error = ASN1Error;

    /// Parse a `DURATION`.
    ///
    /// This implementation makes the leading 'P' optional so that this can be
    /// used to parse duration values as they are encoded by X.690--which is to
    /// say: without the leading 'P'.
    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if unlikely(value_bytes.len() < 2) {
            // The smallest duration string, e.g. P1Y
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if unlikely(value_bytes.len() > 32) {
            // Values larger than this are probably malicious.
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
        }
        let mut start = 0;
        if value_bytes[0] == b'P' {
            start += 1;
        } else if !value_bytes[0].is_ascii_digit() && value_bytes[0] != b'T' {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let mut ret = DURATION_EQUIVALENT::default();
        let mut start_of_last_digit = start;
        let mut processing_time_components: bool = false;
        let mut index_of_period = 0; // 0 means NULL in this case.
        let mut encountered: u8 = 0;
        for i in start..value_bytes.len() {
            let c = value_bytes[i] as char;
            if likely(c.is_ascii_digit()) {
                continue;
            }
            if unlikely(c == '.' || c == ',') {
                if index_of_period > 0 {
                    // Double periods
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                index_of_period = i;
                continue;
            }
            if unlikely(c == 'T') {
                processing_time_components = true;
                start_of_last_digit = i + 1;
                continue;
            }

            // Make sure date components do not appear in the time section and vice versa
            match (c as char, processing_time_components) {
                ('Y' | 'W' | 'D', true) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c))),
                ('H' | 'S', false) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c))),
                _ => (),
            };

            // Make sure the components appear in order
            let max_encountered = match (c as char, processing_time_components) {
                ('Y', _) => DURATION_COMPONENT_YEARS,
                ('M', false) => DURATION_COMPONENT_MONTHS,
                ('W', _) => DURATION_COMPONENT_WEEKS,
                ('D', _) => DURATION_COMPONENT_DAYS,
                ('H', _) => DURATION_COMPONENT_HOURS,
                ('M', true) => DURATION_COMPONENT_MINUTES,
                ('S', _) => DURATION_COMPONENT_SECONDS,
                (_, _) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c))),
            };
            if unlikely(max_encountered > 0 && encountered >= max_encountered) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)));
            }

            if index_of_period > 0 {
                if i != (value_bytes.len() - 1) {
                    // Extra data after the last permitted unit. e.g. "PT0.5H18M"
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                if ret.fractional_part.is_some() {
                    // Already parsed the fractional part. IDK How this could happen.
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                if (i - index_of_period) > 9 {
                    // Way too many decimal digits. Probably malicious.
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                let mut frac = FractionalPart {
                    number_of_digits: (i - (index_of_period+1)) as u8,
                    fractional_value: 0,
                };
                if !value_bytes[index_of_period+1..i].iter().all(u8::is_ascii_digit) {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)));
                }
                for dc in value_bytes[index_of_period+1..i].iter() {
                    let digit = *dc - 0x30;
                    frac.fractional_value *= 10;
                    frac.fractional_value += digit as u32;
                }
                let unambiguous_c = if c == 'M' && processing_time_components { 'm' } else { c };
                let part = DurationPart::try_from(unambiguous_c)
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?;
                ret.fractional_part = Some((part, frac));
            }

            let end_index = if index_of_period > 0 {
                index_of_period
            } else {
                i
            };

            let component_value: u32 = {
                #[cfg(feature = "atoi_simd")]
                {
                    atoi_simd::parse_pos::<u32>(&value_bytes[start_of_last_digit..end_index])
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?
                }
                #[cfg(not(feature = "atoi_simd"))]
                {
                    let component_str = std::str::from_utf8(&value_bytes[start_of_last_digit..end_index])?;
                    u32::from_str(&component_str)
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?
                }
            };

            start_of_last_digit = i + 1;
            encountered |= max_encountered;
            match c as char {
                'Y' => {
                    ret.years = component_value;
                }
                'M' => {
                    if processing_time_components {
                        ret.minutes = component_value;
                    } else {
                        ret.months = component_value;
                    }
                }
                'W' => {
                    ret.weeks = component_value;
                }
                'D' => {
                    ret.days = component_value;
                }
                'H' => {
                    ret.hours = component_value;
                }
                'S' => {
                    ret.seconds = component_value;
                }
                _ => panic!("Impossible code reached."),
            };
        }
        if unlikely(start_of_last_digit != value_bytes.len()) {
            // Extra data at the end
            return Err(ASN1Error::new(ASN1ErrorCode::trailing_content_octets));
        }
        if unlikely(encountered == 0) {
            // No components
            return Err(ASN1Error::new(ASN1ErrorCode::trailing_content_octets));
        }
        Ok(ret)
    }
}

impl FromStr for DURATION_EQUIVALENT {
    type Err = ASN1Error;

    /// Parse a `DURATION`.
    ///
    /// This implementation makes the leading 'P' optional so that this can be
    /// used to parse duration values as they are encoded by X.690--which is to
    /// say: without the leading 'P'.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DURATION_EQUIVALENT::try_from(s.as_bytes())
    }
}

macro_rules! print_uint {
    ($f:ident, $x:expr) => {
        #[cfg(feature = "itoa")]
        {
            let mut buf = itoa::Buffer::new();
            $f.write_str(buf.format($x))?;
        }
        #[cfg(not(feature = "itoa"))]
        {
            $f.write_str($x.to_string().as_str())?;
        }
    };
}

const TIME_PARTS: [DurationPart; 3] = [ DurationPart::Hours, DurationPart::Minutes, DurationPart::Seconds ];

impl Display for DURATION_EQUIVALENT {

    /// Print a `DURATION` value in the ISO 8601 format, which includes the
    /// leading "P", such as "P5M".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut in_time_components: bool = false;
        if self.is_zero() {
            return f.write_str("PT0S");
        }
        f.write_char('P')?;
        let mut unit: char = '\0';
        if self.years > 0 {
            print_uint!(f, self.years);
            unit = 'Y';
        }
        if self.months > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.months);
            unit = 'M';
        }
        if self.weeks > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.weeks);
            unit = 'W';
        }
        if self.days > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.days);
            unit = 'D';
        }
        if self.hours > 0 || self.minutes > 0 || self.seconds > 0 {
            f.write_char(unit)?;
            unit = '\0';
            f.write_char('T')?;
            in_time_components = true;
        }
        if self.hours > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.hours);
            unit = 'H';
        }
        if self.minutes > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.minutes);
            unit = 'M';
        }
        if self.seconds > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.seconds);
            unit = 'S';
        }
        /* It is possible to display a wrong DURATION value where the fractional
        part applies to a unit larger than the smallest unit. Since fmt() is
        basically supposed to be infallible, this simply cannot be handled.
        I had two choices: omit the fraction if wrong, or possibly print the
        wrong component. I felt that it was better to produce a visibly wrong
        DURATION value than an invisibly wrong one. */
        if let Some((fracpart, frac)) = &self.fractional_part {
            let unambiguous_unit = match unit {
                'M' => if in_time_components { 'm' } else { unit },
                '\0' => (*fracpart).into(),
                _ => unit,
            };
            let maybe_dp = DurationPart::try_from(unambiguous_unit);
            if maybe_dp.is_err() {
                return f.write_char(unit);
            }
            let dp = maybe_dp.unwrap();
            // if fracparts match, just display the fraction, then print the unit
            if dp == *fracpart {
                if unit == '\0' {
                    f.write_char('0')?;
                    unit = unambiguous_unit;
                }
                f.write_str(frac.to_string().as_str())?;
                return f.write_char(unit);
            }
            // if not:
            //  print the current unit,
            //  print "T" if unit is for a time component
            //  print "0"
            //  print the frac part
            //  print the frac unit
            f.write_char(unit)?;
            if !in_time_components && TIME_PARTS.contains(fracpart) {
                f.write_char('T')?;
            }
            f.write_char('0')?;
            f.write_str(frac.to_string().as_str())?;
            f.write_char((*fracpart).into())?;
        } else {
            f.write_char(unit)?;
        }
        Ok(())
    }
}

impl PartialEq for DURATION_EQUIVALENT {

    fn eq(&self, other: &Self) -> bool {
        if self.fractional_part.is_some() && self.fractional_part != other.fractional_part {
            return false;
        }
        let selfn = self.normalize();
        let othern = other.normalize();

        selfn.weeks == othern.weeks
            && selfn.days == othern.days
            && selfn.hours == othern.hours
            && selfn.minutes == othern.minutes
            && selfn.seconds == othern.seconds
    }

}

// Calculates the number of decimal digits in a u32 without string conversion
const fn decimal_digits(n: u32) -> usize {
    if n == 0 {
        return 1;
    }
    // Are you not entertained?
    if n < 10 { return 1; }
    if n < 100 { return 2; }
    if n < 1000 { return 3; }
    if n < 10000 { return 4; }
    if n < 100000 { return 5; }
    if n < 1000000 { return 6; }
    if n < 10000000 { return 7; }
    if n < 100000000 { return 8; }
    if n < 1000000000 { return 9; }
    10
}

impl X690KnownSize for DURATION_EQUIVALENT {

    fn x690_size (&self) -> usize {
        let mut size = 0;
        // For each component, add the number of digits plus the designator
        if self.years > 0 { size += decimal_digits(self.years) + 1; }
        if self.months > 0 { size += decimal_digits(self.months) + 1; }
        if self.weeks > 0 { size += decimal_digits(self.weeks) + 1; }
        if self.days > 0 { size += decimal_digits(self.days) + 1; }

        // Add T if there are time components
        if self.hours > 0 || self.minutes > 0 || self.seconds > 0 {
            size += 1;
        }

        if self.hours > 0 { size += decimal_digits(self.hours) + 1; }
        if self.minutes > 0 { size += decimal_digits(self.minutes) + 1; }
        if self.seconds > 0 { size += decimal_digits(self.seconds) + 1; }

        // Add fractional part if present
        if let Some((_, frac)) = &self.fractional_part {
            // Add size for decimal point and digits
            size += 1 + frac.number_of_digits as usize;
        }
        size
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_with_fractional_part_1() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 6,
            fractional_part: Some((DurationPart::Seconds, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3DT4H5M6.123S");
    }

    #[test]
    fn test_fmt_with_fractional_part_2() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 0,
            fractional_part: Some((DurationPart::Minutes, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3DT4H5.123M");
    }

    #[test]
    fn test_fmt_with_fractional_part_3() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Hours, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3DT4.123H");
    }

    #[test]
    fn test_fmt_with_fractional_part_4() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Days, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3.123D");
    }

    #[test]
    fn test_fmt_with_fractional_part_5() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 3,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Weeks, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3.123W");
    }

    #[test]
    fn test_fmt_with_fractional_part_6() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Months, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2.123M");
    }

    #[test]
    fn test_fmt_with_fractional_part_7() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Years, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1.123Y");
    }

    #[test]
    fn test_fmt_with_fractional_part_8() {
        let duration = DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 4,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Days, FractionalPart {
                fractional_value: 123,
                number_of_digits: 4,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P4.0123D");
    }

    #[test]
    fn test_fmt_with_fractional_part_9() {
        let duration = DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 4,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Days, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P4.123D");
    }

    #[test]
    fn test_parse_duration_1() {
        let dur = DURATION_EQUIVALENT::from_str("P4D").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 4);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        assert_eq!(dur.fractional_part, None);
        assert_eq!(dur.to_string().as_str(), "P4D");
    }

    #[test]
    fn test_parse_duration_2() {
        let dur = DURATION_EQUIVALENT::from_str("P4.0123D").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 4);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Days);
        assert_eq!(frac.number_of_digits, 4);
        assert_eq!(frac.fractional_value, 123);
        assert_eq!(dur.to_string().as_str(), "P4.0123D");
    }

    #[test]
    fn test_parse_duration_3() {
        let dur = DURATION_EQUIVALENT::from_str("P23DT23H").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 23);
        assert_eq!(dur.hours, 23);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        assert_eq!(dur.fractional_part, None);
        assert_eq!(dur.to_string().as_str(), "P23DT23H");
    }

    #[test]
    fn test_parse_duration_4() {
        let dur = DURATION_EQUIVALENT::from_str("P0.5Y").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Years);
        assert_eq!(frac.fractional_value, 5);
        assert_eq!(frac.number_of_digits, 1);
        assert_eq!(dur.to_string().as_str(), "P0.5Y");
    }


    #[test]
    fn test_parse_duration_5() {
        // Duration strings only allow a single fractional component.
        assert!(DURATION_EQUIVALENT::from_str("P0.5Y0.5M").is_err());
    }

    #[test]
    fn test_parse_duration_6() {
        assert!(DURATION_EQUIVALENT::from_str("PT").is_err());
    }

    #[test]
    fn test_parse_duration_7() {
        // The problem here is that there is no "T" before a time component.
        assert!(DURATION_EQUIVALENT::from_str("P5S").is_err());
    }

    #[test]
    fn test_parse_duration_8() {
        assert!(DURATION_EQUIVALENT::from_str("").is_err());
    }


    #[test]
    fn test_parse_duration_9() {
        // The problem here is that the components are out of order.
        assert!(DURATION_EQUIVALENT::from_str("P30D12W").is_err());
    }

    #[test]
    fn test_parse_duration_10() {
        // The problem here is that the components are out of order.
        assert!(DURATION_EQUIVALENT::from_str("PT30H15S12M").is_err());
    }

    #[test]
    fn test_parse_duration_11() {
        // "A" is not a valid duration component.
        assert!(DURATION_EQUIVALENT::from_str("P0.5A").is_err());
    }

    #[test]
    fn test_parse_duration_12() {
        // Trailing data
        assert!(DURATION_EQUIVALENT::from_str("P7Y5").is_err());
    }

    #[test]
    fn test_parse_duration_13() {
        let dur = DURATION_EQUIVALENT::from_str("P5Y6M1W23DT25H65M222.00505S").unwrap();
        assert_eq!(dur.years, 5);
        assert_eq!(dur.months, 6);
        assert_eq!(dur.weeks, 1);
        assert_eq!(dur.days, 23);
        assert_eq!(dur.hours, 25);
        assert_eq!(dur.minutes, 65);
        assert_eq!(dur.seconds, 222);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Seconds);
        assert_eq!(frac.fractional_value, 505);
        assert_eq!(frac.number_of_digits, 5);
        assert_eq!(dur.to_string().as_str(), "P5Y6M1W23DT25H65M222.00505S");
    }

    #[test]
    fn test_parse_duration_14() {
        let dur = DURATION_EQUIVALENT::from_str("P7Y0.00123W").unwrap();
        assert_eq!(dur.years, 7);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Weeks);
        assert_eq!(frac.fractional_value, 123);
        assert_eq!(frac.number_of_digits, 5);
        assert_eq!(dur.to_string().as_str(), "P7Y0.00123W");
    }

    #[test]
    fn test_parse_duration_15() {
        let dur = DURATION_EQUIVALENT::from_str("P7YT0.00123M").unwrap();
        assert_eq!(dur.years, 7);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Minutes);
        assert_eq!(frac.fractional_value, 123);
        assert_eq!(frac.number_of_digits, 5);
        assert_eq!(dur.to_string().as_str(), "P7YT0.00123M");
    }

    #[test]
    fn test_parse_duration_no_p() {
        let dur = DURATION_EQUIVALENT::from_str("7YT0.00123M").unwrap();
        assert_eq!(dur.years, 7);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Minutes);
        assert_eq!(frac.fractional_value, 123);
        assert_eq!(frac.number_of_digits, 5);
        assert_eq!(dur.to_string().as_str(), "P7YT0.00123M");
    }

    #[test]
    fn test_compare_duration_1() {
        let dur1 = DURATION_EQUIVALENT::from_str("P5Y").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P5Y").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_2() {
        let dur1 = DURATION_EQUIVALENT::from_str("P5M").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P5M").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_3() {
        let dur1 = DURATION_EQUIVALENT::from_str("P5W").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P5W").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_4() {
        let dur1 = DURATION_EQUIVALENT::from_str("P5D").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P5D").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_5() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT5H").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT5H").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_6() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT5M").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT5M").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_7() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT5S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT5S").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_8() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT0.00005S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT0.00005S").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_9() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT1M5S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT65S").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_10() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT65M").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT1H5M").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_11() {
        let dur1 = DURATION_EQUIVALENT::from_str("P19D").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P2W5D").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_12() {
        let dur1 = DURATION_EQUIVALENT::from_str("PT3605S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("PT1H5S").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_13() {
        let dur1 = DURATION_EQUIVALENT::from_str("P49DT3605S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P7WT1H5S").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_14() {
        let dur1 = DURATION_EQUIVALENT::from_str("P49DT3605S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P7WT1H5S").unwrap();
        assert_eq!(dur1, dur2);
        assert_eq!(dur2, dur1);
    }

    #[test]
    fn test_compare_duration_15() {
        let dur1 = DURATION_EQUIVALENT::from_str("P49DT3605.0013S").unwrap();
        let dur2 = DURATION_EQUIVALENT::from_str("P7WT1H5.0014S").unwrap();
        assert!(dur1 != dur2);
        assert!(dur2 != dur1);
    }

    #[test]
    fn duration_to_seconds_1() {
        let dur = DURATION_EQUIVALENT::from_str("PT5S").unwrap();
        assert_eq!(dur.to_approximate_seconds(), 5);
    }

    #[test]
    fn duration_to_seconds_2() {
        let dur = DURATION_EQUIVALENT::from_str("PT5.4S").unwrap();
        assert_eq!(dur.to_approximate_seconds(), 5);
    }

    #[test]
    fn duration_to_seconds_3() {
        let dur = DURATION_EQUIVALENT::from_str("PT2.5M").unwrap();
        assert_eq!(dur.to_approximate_seconds(), 150);
    }

    #[test]
    fn duration_to_seconds_4() {
        let dur = DURATION_EQUIVALENT::from_str("PT2.00085M").unwrap();
        assert_eq!(dur.to_approximate_seconds(), 120);
    }

}
