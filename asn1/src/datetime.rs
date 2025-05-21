use crate::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use crate::types::{ISO8601Timestampable, X690KnownSize};
use crate::utils::unlikely;
use std::fmt::Display;
use std::str::FromStr;
use crate::types::{GeneralizedTime, UTCTime, DATE, DATE_TIME, TIME_OF_DAY};

impl DATE_TIME {
    #[inline]
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        DATE_TIME {
            date: DATE { year, month, day },
            time: crate::TIME_OF_DAY {
                hour,
                minute,
                second,
            },
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.date.is_zero() && self.time.is_zero()
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

    fn x690_size (&self) -> usize {
        14
    }

}

impl ISO8601Timestampable for DATE_TIME {

    /// NOTE: There is not supposed to be a "Z" at the end of this, nor should
    /// there be a UTC offset. ITU-T Recommendation X.680 defines the
    /// `DATE-TIME` type as being in local time with no UTC offset indication.
    /// For this reason, I recommend against the use of `DATE-TIME`.
    #[inline]
    fn to_iso_8601_string (&self) -> String {
        self.to_string()
    }

}

impl Default for DATE_TIME {
    #[inline]
    fn default() -> Self {
        DATE_TIME {
            date: DATE::default(),
            time: TIME_OF_DAY::default(),
        }
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

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DATE_TIME::try_from(s.as_bytes())
    }
}

impl Display for DATE_TIME {
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
