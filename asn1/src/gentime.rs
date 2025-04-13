use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCOffset, UTCTime, ISO8601Timestampable, DATE};
use crate::utils::{get_days_in_month, unlikely};
use crate::utils::macros::parse_uint;
use std::cmp::min;
use std::fmt::{Display, Write};
use std::str::FromStr;

impl GeneralizedTime {
    pub fn new() -> Self {
        GeneralizedTime {
            date: DATE::default(),
            flags: 0,
            hour: 0,
            minute: None,
            fraction: 0,
            utc_offset: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        let (minute, second) = self.minute.unwrap_or((0, None));
        self.date.year == 0
            && self.date.month <= 1
            && self.date.day <= 1
            && self.hour == 0
            && minute == 0
            && second.unwrap_or(0) == 0
    }

    pub fn is_utc(&self) -> bool {
        self.utc_offset.is_some_and(|offset| offset.is_zero())
    }

    pub fn get_fraction_precision_digits(&self) -> u8 {
        // This implementation only handles up to nano-second precision, hence % 10.
        (self.flags & 0b0000_1111) % 10
    }

    pub fn has_fraction(&self) -> bool {
        self.get_fraction_precision_digits() > 0
    }

}

impl ISO8601Timestampable for GeneralizedTime {

    /// Fractional seconds will only be displayed if the original
    /// GeneralizedTime used fractional seconds (not fractional hours or
    /// minutes).
    #[cfg(feature = "itoa")]
    fn to_iso_8601_string (&self) -> String {
        let mut buf_year = itoa::Buffer::new();
        let mut buf_month = itoa::Buffer::new();
        let mut buf_day = itoa::Buffer::new();
        let mut buf_hour = itoa::Buffer::new();
        let mut buf_minute = itoa::Buffer::new();
        let mut buf_second = itoa::Buffer::new();
        let mut buf_frac = itoa::Buffer::new();
        let mut buf_offset_m = itoa::Buffer::new();

        let mut fraction_string: Option<String> = None;
        let (mut minute, mut second) = self.minute.unwrap_or((0, None));
        let frac_precision = self.get_fraction_precision_digits();
        if frac_precision > 0 {
            let num: f64 = self.fraction.into();
            let denom: f64 = 10.0f64.powi(frac_precision as i32);
            if unlikely(self.minute.is_none()) {
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
                    buf_frac.format(self.fraction),
                    width = frac_precision as usize
                ));
            }
        }

        if self.is_utc() {
            return format!(
                "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}{}Z",
                buf_year.format(self.date.year),
                buf_month.format(self.date.month),
                buf_day.format(self.date.day),
                buf_hour.format(self.hour),
                buf_minute.format(minute),
                buf_second.format(second.unwrap_or(0)),
                fraction_string.unwrap_or(String::new()),
            );
        }
        if let Some(offset) = &self.utc_offset {
            return format!(
                "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}{}{:+03}{:0>2}",
                buf_year.format(self.date.year),
                buf_month.format(self.date.month),
                buf_day.format(self.date.day),
                buf_hour.format(self.hour),
                buf_minute.format(minute),
                buf_second.format(second.unwrap_or(0)),
                fraction_string.unwrap_or(String::new()),
                offset.hour,
                buf_offset_m.format(offset.minute),
            );
        }
        return format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}{}",
            buf_year.format(self.date.year),
            buf_month.format(self.date.month),
            buf_day.format(self.date.day),
            buf_hour.format(self.hour),
            buf_minute.format(minute),
            buf_second.format(second.unwrap_or(0)),
            fraction_string.unwrap_or(String::new()),
        );
    }

    /// Fractional seconds will only be displayed if the original
    /// GeneralizedTime used fractional seconds (not fractional hours or
    /// minutes).
    #[cfg(not(feature = "itoa"))]
    fn to_iso_8601_string (&self) -> String {
        let mut fraction_string: Option<String> = None;
        let (mut minute, mut second) = self.minute.unwrap_or((0, None));
        let frac_precision = self.get_fraction_precision_digits();
        if frac_precision > 0 {
            let num: f64 = self.fraction.into();
            let denom: f64 = 10.0f64.powi(frac_precision as i32);
            if unlikely(self.minute.is_none()) {
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
    fn default() -> Self {
        GeneralizedTime {
            date: DATE::default(),
            flags: 0,
            hour: 0,
            minute: None,
            fraction: 0,
            utc_offset: None,
        }
    }
}

impl From<UTCTime> for GeneralizedTime {
    fn from(other: UTCTime) -> Self {
        let date = DATE::from(other);
        GeneralizedTime {
            date,
            flags: 0,
            hour: other.hour,
            minute: Some((other.minute, Some(other.second))),
            fraction: 0,
            utc_offset: None,
        }
    }
}

impl From<DATE> for GeneralizedTime {

    /// **WARNING**: This sets the GeneralizedTime to be in local time!
    fn from(other: DATE) -> Self {
        GeneralizedTime {
            date: other,
            flags: 0, // Local time, not UTC.
            hour: 0,
            minute: None,
            fraction: 0,
            utc_offset: None,
        }
    }
}

impl TryFrom<&[u8]> for GeneralizedTime {
    type Error = ASN1Error;

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
            ret.minute = Some((minute, None));
        }

        if let Some((m, _)) = ret.minute {
            // Normal "if"s cannot be combined with "if let"s.
            if (len >= 14) && b[12].is_ascii_digit() {
                // Seconds component is present.
                let second = parse_uint!(u8, &b[12..14], &s[12..14], ASN1ErrorCode::invalid_second);
                if unlikely(second > 59) {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                }
                ret.minute = Some((m, Some(second)));
            }
        }

        let mut i: usize = match ret.minute {
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

        // TODO: Use https://docs.rs/likely_stable/latest/likely_stable/
        if unlikely(offset_sign.is_some_and(|c| *c != b'+' && *c != b'-')) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely((len != (i + 3)) && (len != (i + 5))) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        let offset_hour = if cfg!(feature = "atoi_simd") {
            atoi_simd::parse_skipped(&b[i..i + 3])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
        } else {
            i8::from_str(&s[i..i + 3])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
        };
        if unlikely(offset_hour.abs() > 12) { // FIXME: 15 is valid
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GeneralizedTime::try_from(s.as_bytes())
    }
}

impl Display for GeneralizedTime {

    #[cfg(feature = "itoa")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf_year = itoa::Buffer::new();
        let mut buf_month = itoa::Buffer::new();
        let mut buf_day = itoa::Buffer::new();
        let mut buf_hour = itoa::Buffer::new();
        let mut buf_minute = itoa::Buffer::new();
        let mut buf_second = itoa::Buffer::new();
        let mut buf_frac = itoa::Buffer::new();
        let mut buf_offset_m = itoa::Buffer::new();

        write!(f, "{:0>4}{:0>2}{:0>2}{:0>2}",
            buf_year.format(self.date.year % 10000),
            buf_month.format(self.date.month),
            buf_day.format(self.date.day),
            buf_hour.format(self.hour),
        )?;
        if let Some((min, maybe_sec)) = &self.minute {
            write!(f, "{:0>2}", buf_minute.format(*min))?;
            if let Some(sec) = &maybe_sec {
                write!(f, "{:0>2}", buf_second.format(*sec))?;
            }
        }

        let frac_digits = self.get_fraction_precision_digits();
        if frac_digits > 0 {
            write!(f, ".{:0>width$}",
                buf_frac.format(self.fraction),
                width = frac_digits as usize
            )?;
        }
        match &self.utc_offset {
            Some(offset) => write!(f, "{:+03}{:0>2}",
                offset.hour,
                buf_offset_m.format(offset.minute),
            ),
            None => f.write_char('Z')
        }
    }

    #[cfg(not(feature = "itoa"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}{:02}{:02}{:02}",
            self.date.year % 10000,
            self.date.month,
            self.date.day,
            self.hour,
        )?;
        if let Some((min, maybe_sec)) = &self.minute {
            write!(f, "{:02}", min)?;
            if let Some(sec) = &maybe_sec {
                write!(f, "{:02}", sec)?;
            }
        }

        let frac_digits = self.get_fraction_precision_digits();
        if frac_digits > 0 {
            write!(f, ".{:0>width$}",
                self.fraction,
                width = frac_digits as usize
            )?;
        }
        match &self.utc_offset {
            Some(offset) => write!(f, "{:+03}{:02}", offset.hour, offset.minute),
            None => f.write_char('Z')
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
            [ "2021020304Z", "2021-02-03T04:00:00Z" ],
            // With fractional hours
            [ "2021020304.3334Z", "2021-02-03T04:20:00Z" ],
            [ "2021020304,3334Z", "2021-02-03T04:20:00Z" ],
            [ "2021020304.50Z", "2021-02-03T04:30:00Z" ],
            [ "2021020304.333333334Z", "2021-02-03T04:20:00Z" ],
            // With fractional minutes
            [ "202102030405.3334Z", "2021-02-03T04:05:20Z" ],
            [ "202102030405,3334Z", "2021-02-03T04:05:20Z" ],
            // With fractional seconds
            [ "20210203040506.3334Z", "2021-02-03T04:05:06.3334Z" ],
            [ "20210203040506,3334Z", "2021-02-03T04:05:06.3334Z" ],
            // Simple timezone offset
            [ "2021020304-05", "2021-02-03T04:00:00-0500" ],
            [ "2021020304+05", "2021-02-03T04:00:00+0500" ],
            [ "2021020304-0500", "2021-02-03T04:00:00-0500" ],
            // Carry over with offset minutes and fractional hours
            [ "2021020304.25+0815", "2021-02-03T04:15:00+0815" ],
            [ "2021020320.25-0815", "2021-02-03T20:15:00-0815" ],
            // Minutes with timezone offset
            [ "202102030406-0500", "2021-02-03T04:06:00-0500" ],
            // Seconds with timezone offset
            [ "20210203040607-0500", "2021-02-03T04:06:07-0500" ],
            // The most complicated examples
            [ "20210203040607.32895292-0503", "2021-02-03T04:06:07.32895292-0503" ],
            [ "20210203040607,32895292+0304", "2021-02-03T04:06:07.32895292+0304" ],
            // Nanosecond precision
            // [ "20210203040607.123456789-0503", "2021-02-03T04:06:07.123456789-0503" ],
        ];
        for [valid_gentime, should_be] in subtests {
            let gt = GeneralizedTime::from_str(valid_gentime).expect(valid_gentime);
            assert_eq!(gt.to_iso_8601_string(), should_be);
        }
    }

    // FIXME: Check that these do not panic and use assertions instead.
    // This is to check for out of bounds access and such.
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
