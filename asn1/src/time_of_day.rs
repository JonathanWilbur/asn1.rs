use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCTime, DATE_TIME, TIME_OF_DAY};
use std::fmt::Display;
use std::str::FromStr;

impl TIME_OF_DAY {
    pub fn new() -> Self {
        TIME_OF_DAY {
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

impl From<DATE_TIME> for TIME_OF_DAY {
    fn from(other: DATE_TIME) -> Self {
        other.time
    }
}

impl From<GeneralizedTime> for TIME_OF_DAY {
    fn from(other: GeneralizedTime) -> Self {
        TIME_OF_DAY {
            hour: other.hour,
            minute: other.minute.unwrap_or(0),
            second: other.second.unwrap_or(0),
        }
    }
}

impl From<UTCTime> for TIME_OF_DAY {
    fn from(other: UTCTime) -> Self {
        TIME_OF_DAY {
            hour: other.hour,
            minute: other.minute,
            second: other.second.unwrap_or(0),
        }
    }
}

impl TryFrom<&[u8]> for TIME_OF_DAY {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if value_bytes.len() != 8 {
            // "HH:MM:SS".len()
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let str_ = match String::from_utf8(value_bytes.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let hour = match u8::from_str(&str_[0..2]) {
            Ok(x) => x,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let minute = match u8::from_str(&str_[3..5]) {
            Ok(x) => x,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let second = match u8::from_str(&str_[6..]) {
            Ok(x) => x,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        if hour > 23 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if minute > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if second > 59 {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TIME_OF_DAY::try_from(s.as_bytes())
    }
}

impl Display for TIME_OF_DAY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_form = format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second);
        f.write_str(&str_form)
    }
}
