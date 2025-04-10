use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCOffset, UTCTime, DATE, DATE_TIME};
use std::fmt::{Display, Write};
use std::str::FromStr;

impl UTCTime {
    pub fn new() -> Self {
        UTCTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: None,
            utc_offset: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.year == 0
            && self.month <= 1
            && self.day <= 1
            && self.hour == 0
            && self.minute == 0
            && self.second.unwrap_or(0) == 0
    }

    // TODO: Maybe make a trait for this?
    pub fn to_iso_8601_string(&self) -> String {
        if let Some(offset) = &self.utc_offset {
            let sign = if offset.hour >= 0 { '+' } else { '-' };
            return format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}{:02}{:02}",
                if self.year >= 50 { self.year as u16 + 1900 } else { self.year as u16 + 2000 },
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second.unwrap_or(0),
                sign,
                offset.hour.abs(),
                offset.minute,
            );
        }
        return format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            if self.year >= 50 { self.year as u16 + 1900 } else { self.year as u16 + 2000 },
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second.unwrap_or(0),
        );
    }

}

impl Default for UTCTime {
    fn default() -> Self {
        UTCTime {
            year: 0,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: None,
            utc_offset: None,
        }
    }
}

impl From<GeneralizedTime> for UTCTime {
    fn from(other: GeneralizedTime) -> Self {
        let (minute, second) = other.minute.unwrap_or((0, None));
        UTCTime {
            year: (other.date.year % 100) as u8,
            month: other.date.month,
            day: other.date.day,
            hour: other.hour,
            minute,
            second,
            utc_offset: None,
        }
    }
}

impl From<DATE_TIME> for UTCTime {
    fn from(other: DATE_TIME) -> Self {
        UTCTime {
            year: (other.date.year % 100) as u8,
            month: other.date.month,
            day: other.date.day,
            hour: other.time.hour,
            minute: other.time.minute,
            second: Some(other.time.second),
            utc_offset: None,
        }
    }
}

impl From<DATE> for UTCTime {
    fn from(other: DATE) -> Self {
        UTCTime {
            year: (other.year % 100) as u8,
            month: other.month,
            day: other.day,
            hour: 0,
            minute: 0,
            second: None,
            utc_offset: None,
        }
    }
}

impl PartialEq<DATE> for UTCTime {
    fn eq(&self, other: &DATE) -> bool {
        DATE::from(*self).eq(other)
    }
}

impl PartialEq<GeneralizedTime> for UTCTime {
    fn eq(&self, other: &GeneralizedTime) -> bool {
        UTCTime::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for UTCTime {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        let len = value_bytes.len();
        if len < 10 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if len > 17 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if value_bytes[0..10].iter().any(|b| !b.is_ascii_digit()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let s = std::str::from_utf8(&value_bytes)
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let mut ret = UTCTime::new();
        ret.year = u8::from_str(&s[0..2])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
        ret.month = u8::from_str(&s[2..4])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
        if ret.month == 0 || ret.month > 12 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        ret.day = u8::from_str(&s[4..6])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        let year = if ret.year >= 50 { ret.year as u16 + 1900 } else { ret.year as u16 + 2000 };
        let is_leap_year = ((year % 4) == 0) && (((year % 100) > 0) || ((year % 400) == 0));
        let max_day = match ret.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => if is_leap_year { 29 } else { 28 },
            _ => 30,
        };
        if ret.day == 0 || ret.day > max_day {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        ret.hour = u8::from_str(&s[6..8])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
        if ret.hour > 23 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        ret.minute = u8::from_str(&s[8..10])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
        if ret.minute > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if (len > 12) && value_bytes[10].is_ascii_digit() {
            // Seconds component is present.
            if !value_bytes[11].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            ret.second = u8::from_str(&s[10..12])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?
                .into();
            if ret.second.is_some_and(|s| s > 59) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
        }
        if value_bytes[len - 1] != b'Z' {
            if (value_bytes[len - 5] != b'+') && (value_bytes[len - 5] != b'-') {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            if value_bytes[len - 4..len].iter().any(|b| !b.is_ascii_digit()) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            let offset_hour = i8::from_str(&s[len - 5..len - 2])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?;
            if offset_hour.abs() > 12 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            let offset_minute = u8::from_str(&s[len - 2..len])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?;
            if offset_minute > 59 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            ret.utc_offset = Some(UTCOffset {
                hour: offset_hour,
                minute: offset_minute,
            });
        }
        Ok(ret)
    }
}

impl FromStr for UTCTime {
    type Err = ASN1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UTCTime::try_from(s.as_bytes())
    }
}

impl Display for UTCTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}{:02}{:02}{:02}{:02}",
            self.year % 100,
            self.month,
            self.day,
            self.hour,
            self.minute,
        )?;
        if let Some(sec) = &self.second {
            write!(f, "{:02}", sec)?;
        }
        match &self.utc_offset {
            // TODO: Use this sign-formatting technique
            Some(offset) => write!(f, "{:+03}{:02}", offset.hour, offset.minute),
            _ => f.write_char('Z'),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::UTCTime;

    #[test]
    fn utc_time_display_1() {
        let t = UTCTime{
            year: 22,
            month: 11,
            day: 10,
            hour: 9,
            minute: 8,
            second: Some(7),
            ..Default::default()
        };
        assert_eq!(format!("{}", t), "221110090807Z");
    }

    #[test]
    fn utc_time_valid() {
        let subtests = [
            [ "0102030405Z", "2001-02-03T04:05:00Z" ],
            [ "010203040506Z", "2001-02-03T04:05:06Z" ],
            [ "0102030405-0400", "2001-02-03T04:05:00-0400" ],
            [ "010203040506-0400", "2001-02-03T04:05:06-0400" ],
            [ "0102030405+0400", "2001-02-03T04:05:00+0400" ],
            [ "010203040506+0400", "2001-02-03T04:05:06+0400" ],
            // Minute-specific timezone offsets
            [ "0102030405-0415", "2001-02-03T04:05:00-0415" ],
            [ "010203040506-0415", "2001-02-03T04:05:06-0415" ],
            [ "0102030405+0415", "2001-02-03T04:05:00+0415" ],
            [ "010203040506+0415", "2001-02-03T04:05:06+0415" ],
        ];
        for [valid_utctime, should_be] in subtests {
            let ut = UTCTime::from_str(valid_utctime).expect(valid_utctime);
            assert_eq!(ut.to_iso_8601_string(), should_be);
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
