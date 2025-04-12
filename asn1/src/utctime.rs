use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCOffset, UTCTime, ISO8601Timestampable};
use crate::utils::get_days_in_month;
use crate::utils::macros::parse_uint;
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
            second: 0,
            utc_offset: UTCOffset::default()
        }
    }

    pub fn is_zero(&self) -> bool {
        self.year == 0
            && self.month <= 1
            && self.day <= 1
            && self.hour == 0
            && self.minute == 0
            && self.second == 0
    }

}

impl ISO8601Timestampable for UTCTime {

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
    fn from(other: GeneralizedTime) -> Self {
        let (minute, second) = other.minute.unwrap_or((0, None));
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
    fn eq(&self, other: &GeneralizedTime) -> bool {
        UTCTime::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for UTCTime {
    type Error = ASN1Error;

    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        let len = b.len();
        if len < 10 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if len > 17 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if !b[0..10].is_ascii() {
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
        if ret.month == 0 || ret.month > 12 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        let year = if ret.year >= 50 { ret.year as u16 + 1900 } else { ret.year as u16 + 2000 };
        let max_day: u8 = get_days_in_month(year, ret.month);
        if ret.day == 0 || ret.day > max_day {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        if ret.hour > 23 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if ret.minute > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if (len > 12) && b[10].is_ascii_digit() {
            // Seconds component is present.
            if !b[11].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            ret.second = parse_uint!(u8, &b[10..12], &s[10..12], ASN1ErrorCode::invalid_minute);
            if ret.second > 59 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
        }
        if b[len - 1] != b'Z' {
            if (b[len - 5] != b'+') && (b[len - 5] != b'-') {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            if b[len - 4..len].iter().any(|by| !by.is_ascii_digit()) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            let offset_hour = if cfg!(feature = "atoi_simd") {
                atoi_simd::parse_skipped(&b[len-5..len-2])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
            } else {
                i8::from_str(&s[len-5..len-2])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_time_offset))?
            };
            if offset_hour.abs() > 12 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            let offset_minute = parse_uint!(u8, &b[len-2..len], &s[len-2..len], ASN1ErrorCode::invalid_time_offset);
            if offset_minute > 59 {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UTCTime::try_from(s.as_bytes())
    }
}

impl Display for UTCTime {
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

    // TODO: Test format
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
