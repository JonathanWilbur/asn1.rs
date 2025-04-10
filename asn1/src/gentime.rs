use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCOffset, UTCTime, ISO8601Timestampable, DATE};
use crate::FractionalPart;
use std::cmp::min;
use std::fmt::{Display, Write};
use std::str::FromStr;

impl GeneralizedTime {
    pub fn new() -> Self {
        GeneralizedTime {
            date: DATE::default(),
            utc: true,
            hour: 0,
            minute: None,
            fraction: None,
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

}

impl ISO8601Timestampable for GeneralizedTime {

    /// Fractional seconds will only be displayed if the original
    /// GeneralizedTime used fractional seconds (not fractional hours or
    /// minutes).
    fn to_iso_8601_string (&self) -> String {
        let mut fraction_string: Option<String> = None;
        let (mut minute, mut second) = self.minute.unwrap_or((0, None));
        if let Some(frac) = &self.fraction {
            let num: f64 = frac.fractional_value.into();
            let denom: f64 = 10.0f64.powi(frac.number_of_digits as i32);
            if self.minute.is_none() {
                // Fractional hours
                let secondsf = (num / denom) * 3600.0;
                minute = (secondsf / 60.0).floor() as u8;
                second = Some((secondsf.round() % 60.0) as u8 );
            } else if second.is_none() {
                // Fractional minutes
                let secondsf = (num / denom) * 60.0;
                second = Some(secondsf.round() as u8);
            } else {
                // Fractional seconds
                fraction_string = Some(format!(".{:0>width$}",
                    frac.fractional_value,
                    width = frac.number_of_digits as usize
                ));
            }
        }
        if self.utc {
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
            hour: 0,
            minute: None,
            fraction: None,
            utc: true,
            utc_offset: None,
        }
    }
}

impl From<UTCTime> for GeneralizedTime {
    fn from(other: UTCTime) -> Self {
        let date = DATE::from(other);
        GeneralizedTime {
            date,
            utc: other.utc_offset.is_zero(),
            hour: other.hour,
            minute: Some((other.minute, Some(other.second))),
            fraction: None,
            utc_offset: None,
        }
    }
}

impl From<DATE> for GeneralizedTime {

    /// **WARNING**: This sets the GeneralizedTime to be in local time!
    fn from(other: DATE) -> Self {
        GeneralizedTime {
            date: other,
            utc: false,
            hour: 0,
            minute: None,
            fraction: None,
            utc_offset: None,
        }
    }
}

impl PartialEq<UTCTime> for GeneralizedTime {
    fn eq(&self, other: &UTCTime) -> bool {
        GeneralizedTime::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for GeneralizedTime {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        let len = value_bytes.len();
        if len < 10 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        // There is technically no limit on how big a GeneralizedTime can be, but
        // we have to set a reasonable limit here.
        if len > 32 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
        }
        if value_bytes[0..10].iter().any(|b| !b.is_ascii_digit()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let s = std::str::from_utf8(&value_bytes)
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let mut date = DATE::default();
        let mut ret = GeneralizedTime::new();
        date.year = u16::from_str(&s[0..4])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
        date.month = u8::from_str(&s[4..6])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
        if date.month == 0 || date.month > 12 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        date.day = u8::from_str(&s[6..8])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        if date.day == 0 || date.day > 31 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        ret.date = date;
        ret.hour = u8::from_str(&s[8..10])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
        if ret.hour > 23 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if (len >= 12) && value_bytes[10].is_ascii_digit() {
            if !value_bytes[11].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
            }
            let minute = u8::from_str(&s[10..12])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
            if minute > 59 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
            }
            ret.minute = Some((minute, None));
        }

        if let Some((m, _)) = ret.minute {
            // Normal "if"s cannot be combined with "if let"s.
            if (len >= 14) && value_bytes[12].is_ascii_digit() {
                // Seconds component is present.
                if !value_bytes[13].is_ascii_digit() { // TODO: Is this check necessary?
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                }
                let second = u8::from_str(&s[12..14])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?;
                if second > 59 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                }
                ret.minute = Some((m, Some(second)));
            }
        }

        let mut i: usize = match ret.minute {
            None => 10,
            Some((_, s)) => if s.is_some() { 14 } else { 12 },
        };
        if (len > (i + 1)) && ((value_bytes[i] == b'.') || (value_bytes[i] == b',')) {
            i += 1;
            let start = i;
            while i < len && value_bytes[i].is_ascii_digit() {
                i += 1;
            }
            let end = min(i, start + 9); // We can only tolerate 9 digits of precision.
            let fractional_value = u32::from_str(&s[start..end])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds))?;
            ret.fraction = Some(FractionalPart{
                fractional_value,
                number_of_digits: (end - start) as u8,
            });
        }

        let offset_sign = value_bytes.get(i);
        if offset_sign.is_some_and(|c| *c == b'Z') {
            // ret.utc = true; // This is the default.
            return Ok(ret); // UTCTime
        }
        ret.utc = false;
        if offset_sign.is_none() {
            return Ok(ret); // Local Time
        }

        // TODO: Use https://docs.rs/likely_stable/latest/likely_stable/
        if offset_sign.is_some_and(|c| *c != b'+' && *c != b'-') {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if (len != (i + 3)) && (len != (i + 5)) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        let offset_hour = i8::from_str(&s[i..i + 3])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?;
        if offset_hour.abs() > 12 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        i += 3;
        let offset_minute = if len == (i + 2) {
            u8::from_str(&s[i..i+2])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
        } else {
            0
        };
        if offset_minute > 59 {
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
        if let Some(frac) = &self.fraction {
            write!(f, ".{:0>width$}",
                frac.fractional_value,
                width = frac.number_of_digits as usize
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
        let input = b"19960415203000.0";
        GeneralizedTime::try_from(input.as_slice()).unwrap();
    }

    #[test]
    fn gen_time_from_str_accepts_fractional_seconds_and_timezone() {
        let input = b"19960415203000.0Z";
        GeneralizedTime::try_from(input.as_slice()).unwrap();
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
        ];
        for [valid_gentime, should_be] in subtests {
            let gt = GeneralizedTime::from_str(valid_gentime).expect(valid_gentime);
            assert_eq!(gt.to_iso_8601_string(), should_be);
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
