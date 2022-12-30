use crate::error::{ASN1Error, ASN1ErrorCode};
use std::fmt::Display;
use std::str::FromStr;

use crate::types::{GeneralizedTime, UTCTime, DATE, DATE_TIME};

impl DATE {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        DATE { year, month, day }
    }

    pub fn is_zero(&self) -> bool {
        self.year == 0 && self.month <= 1 && self.day == 0
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
        if value_bytes.len() != 10 {
            // "YYYY-MM-DD".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let str_ = match String::from_utf8(value_bytes.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let year = match u16::from_str(&str_[0..4]) {
            Ok(x) => x,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let month = match u8::from_str(&str_[5..7]) {
            Ok(x) => x,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let day = match u8::from_str(&str_[8..]) {
            Ok(x) => x,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
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
        let str_form = format!("{:04}-{:02}-{:02}", self.year % 10000, self.month, self.day,);
        f.write_str(&str_form)
    }
}
