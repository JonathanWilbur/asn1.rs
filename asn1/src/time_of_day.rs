use crate::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use crate::types::{GeneralizedTime, UTCTime, DATE_TIME, TIME_OF_DAY, X690KnownSize};
use crate::utils::unlikely;
use crate::X690Validate;
use std::fmt::Display;
use std::str::FromStr;

impl TIME_OF_DAY {
    #[inline]
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        TIME_OF_DAY {
            hour,
            minute,
            second,
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.hour == 0 && self.minute == 0 && self.second == 0
    }

    /// This is intentionally designed to be suitable as an encoding of this
    /// abstract value as the content octets of a value according to the
    /// Basic Encoding Rules (BER), Distinguished Encoding Rules (DER), or
    /// Canonical Encoding Rules (CER) according to ITU-T Recommendation X.690.
    pub fn to_num_str(&self) -> String {
        if cfg!(feature = "itoa") {
            let mut buf1 = itoa::Buffer::new();
            let mut buf2 = itoa::Buffer::new();
            let mut buf3 = itoa::Buffer::new();
            format!("{:0>2}{:0>2}{:0>2}",
                buf1.format(self.hour % 24),
                buf2.format(self.minute % 60),
                buf3.format(self.second % 60)
            )
        } else {
            format!("{:02}{:02}{:02}", self.hour % 24, self.minute % 60, self.second % 60)
        }
    }

    /// This is intentionally designed to be suitable as an decoding of this
    /// abstract value from the content octets of a value according to the
    /// Basic Encoding Rules (BER), Distinguished Encoding Rules (DER), or
    /// Canonical Encoding Rules (CER) according to ITU-T Recommendation X.690.
    pub fn try_from_num_str(s: &str) -> ASN1Result<Self> {
        let b = s.as_bytes();
        if b.len() != 6 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely(!b.is_ascii()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let hour: u8;
        let minute: u8;
        let second: u8;
        if cfg!(feature = "atoi_simd") {
            hour = atoi_simd::parse_pos::<u8>(&b[0..2])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
            minute = atoi_simd::parse_pos::<u8>(&b[2..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
            second = atoi_simd::parse_pos::<u8>(&b[4..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?;
        } else {
            hour = u8::from_str(&s[0..2])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
            minute = u8::from_str(&s[2..4])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
            second = u8::from_str(&s[4..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        }
        Ok(TIME_OF_DAY { hour, minute, second })
    }

}

impl Default for TIME_OF_DAY {
    #[inline]
    fn default() -> Self {
        TIME_OF_DAY {
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

impl From<DATE_TIME> for TIME_OF_DAY {
    #[inline]
    fn from(other: DATE_TIME) -> Self {
        other.time
    }
}

impl From<GeneralizedTime> for TIME_OF_DAY {
    #[inline]
    fn from(other: GeneralizedTime) -> Self {
        let (minute, second) = other.min_and_sec.unwrap_or((0, None));
        TIME_OF_DAY {
            hour: other.hour,
            minute,
            second: second.unwrap_or(0),
        }
    }
}

impl From<UTCTime> for TIME_OF_DAY {
    #[inline]
    fn from(other: UTCTime) -> Self {
        TIME_OF_DAY {
            hour: other.hour,
            minute: other.minute,
            second: other.second,
        }
    }
}

impl TryFrom<&[u8]> for TIME_OF_DAY {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if unlikely(value_bytes.len() != 8) {
            // "HH:MM:SS".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if unlikely(value_bytes[2] != b':' || value_bytes[5] != b':') {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let hour: u8;
        let minute: u8;
        let second: u8;
        if cfg!(feature = "atoi_simd") {
            hour = atoi_simd::parse_pos::<u8>(&value_bytes[0..2])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            minute = atoi_simd::parse_pos::<u8>(&value_bytes[3..5])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            second = atoi_simd::parse_pos::<u8>(&value_bytes[6..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        } else {
            let str_ = std::str::from_utf8(&value_bytes)
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            hour = u8::from_str(&str_[0..2])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            minute = u8::from_str(&str_[3..5])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            second = u8::from_str(&str_[6..])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        }
        if unlikely(hour > 23) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if unlikely(minute > 59) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if unlikely(second > 59) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
        }
        return Ok(TIME_OF_DAY {
            hour,
            minute,
            second,
        });
    }
}

impl FromStr for TIME_OF_DAY {
    type Err = ASN1Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TIME_OF_DAY::try_from(s.as_bytes())
    }
}

impl X690KnownSize for TIME_OF_DAY {

    fn x690_size (&self) -> usize {
        6
    }

}

impl X690Validate for TIME_OF_DAY {

    fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()> {
        if content_octets.len() != 6 { // HHMMSS (X.690 strips the colons)
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if !content_octets.iter().all(|b| b.is_ascii_digit()) {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let s = unsafe { std::str::from_utf8_unchecked(&content_octets) };
        let hour = u8::from_str(&s[0..2])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
        let minute = u8::from_str(&s[2..4])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
        let second = u8::from_str(&s[4..])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?;
        if hour > 23 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if minute > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if second > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
        }
        Ok(())
    }

}

impl Display for TIME_OF_DAY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if cfg!(feature = "itoa") {
            let mut buf1 = itoa::Buffer::new();
            let mut buf2 = itoa::Buffer::new();
            let mut buf3 = itoa::Buffer::new();
            write!(f, "{:0>2}:{:0>2}:{:0>2}",
                buf1.format(self.hour),
                buf2.format(self.minute),
                buf3.format(self.second)
            )
        } else {
            write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_display() {
        let x = TIME_OF_DAY::new(20, 19, 18);
        assert_eq!(format!("{}", x), "20:19:18");
    }

    #[test]
    fn test_time_parse() {
        let x = TIME_OF_DAY::from_str("20:19:18").unwrap();
        assert_eq!(x.hour, 20);
        assert_eq!(x.minute, 19);
        assert_eq!(x.second, 18);
    }

    #[test]
    fn test_time_of_day_ordering_1() {
        let tod1 = TIME_OF_DAY::new(22, 04, 22);
        let tod2 = TIME_OF_DAY::new(22, 04, 23);
        assert!(tod2 > tod1);
    }

    #[test]
    fn test_time_of_day_ordering_2() {
        let tod1 = TIME_OF_DAY::new(22, 04, 23);
        let tod2 = TIME_OF_DAY::new(22, 05, 22);
        assert!(tod2 > tod1);
    }

    #[test]
    fn test_time_of_day_ordering_3() {
        let tod1 = TIME_OF_DAY::new(22, 06, 23);
        let tod2 = TIME_OF_DAY::new(23, 05, 22);
        assert!(tod2 > tod1);
    }

    #[test]
    fn test_time_of_day_to_and_from_str_1() {
        let tod = TIME_OF_DAY::try_from_num_str("151317").unwrap();
        assert_eq!(tod.to_num_str(), "151317");
    }

    #[test]
    fn test_time_of_day_to_and_from_str_2() {
        let tod = TIME_OF_DAY::try_from_num_str("050307").unwrap();
        assert_eq!(tod.to_num_str(), "050307");
    }

}
