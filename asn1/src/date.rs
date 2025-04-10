use crate::error::{ASN1Error, ASN1ErrorCode};
use std::fmt::Display;
use std::str::FromStr;

use crate::types::{GeneralizedTime, UTCTime, DATE, DATE_TIME};

impl DATE {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        DATE { year, month, day }
    }

    pub fn is_zero(&self) -> bool {
        self.year == 0 && self.month <= 1 && self.day <= 1
    }
}

impl Default for DATE {
    fn default() -> Self {
        DATE {
            year: 0,
            month: 1,
            day: 1,
        }
    }
}

impl From<GeneralizedTime> for DATE {
    fn from(other: GeneralizedTime) -> Self {
        other.date
    }
}

impl From<DATE_TIME> for DATE {
    fn from(other: DATE_TIME) -> Self {
        other.date
    }
}

impl From<UTCTime> for DATE {
    fn from(other: UTCTime) -> Self {
        DATE {
            /* The conversion below was taken from ITU Recommendation X.509
            (2019), Section 7.2.1. */
            year: if other.year >= 50 {
                1900 + other.year as u16
            } else {
                2000 + other.year as u16
            },
            month: other.month,
            day: other.day,
        }
    }
}

impl PartialEq<GeneralizedTime> for DATE {
    fn eq(&self, other: &GeneralizedTime) -> bool {
        DATE::from(*other).eq(self)
    }
}

impl PartialEq<UTCTime> for DATE {
    fn eq(&self, other: &UTCTime) -> bool {
        DATE::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for DATE {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if value_bytes.len() != 10 { // "YYYY-MM-DD".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let str_ = std::str::from_utf8(&value_bytes)
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        // TODO: Hack to accelerate decoding of 202x by checking for "202" prefix
        let year = u16::from_str(&str_[0..4])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let month = u8::from_str(&str_[5..7])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let day = u8::from_str(&str_[8..])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        if month > 12 || month == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        if day > 31 || day == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        Ok(DATE { year, month, day })
    }
}

impl FromStr for DATE {
    type Err = ASN1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DATE::try_from(s.as_bytes())
    }
}

impl Display for DATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:04}-{:02}-{:02}", self.year % 10000, self.month, self.day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_display() {
        let x = DATE::new(2022, 04, 23);
        assert_eq!(format!("{}", x), "2022-04-23");
    }

    #[test]
    fn test_date_parse() {
        let x = DATE::from_str("2022-04-23").unwrap();
        assert_eq!(x.year, 2022);
        assert_eq!(x.month, 4);
        assert_eq!(x.day, 23);
    }

    #[test]
    fn test_date_ordering_1() {
        let date1 = DATE::new(2022, 04, 22);
        let date2 = DATE::new(2022, 04, 23);
        assert!(date2 > date1);
    }

    #[test]
    fn test_date_ordering_2() {
        let date1 = DATE::new(2022, 04, 23);
        let date2 = DATE::new(2022, 05, 22);
        assert!(date2 > date1);
    }

    #[test]
    fn test_date_ordering_3() {
        let date1 = DATE::new(2022, 06, 23);
        let date2 = DATE::new(2023, 05, 22);
        assert!(date2 > date1);
    }

}
