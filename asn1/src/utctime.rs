use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCOffset, UTCTime, DATE, DATE_TIME};
use std::fmt::Display;
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
}

impl From<GeneralizedTime> for UTCTime {
    fn from(other: GeneralizedTime) -> Self {
        UTCTime {
            year: (other.date.year % 100) as u8,
            month: other.date.month,
            day: other.date.day,
            hour: other.hour,
            minute: other.minute.unwrap_or(0),
            second: other.second,
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
        for byte in value_bytes[0..10].iter() {
            if !byte.is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
            }
        }
        let s = match String::from_utf8(value_bytes.to_vec()) {
            Ok(r) => r,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let mut ret = UTCTime::new();
        ret.year = match u8::from_str(&s[0..2]) {
            Ok(u) => u,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_year)),
        };
        ret.month = match u8::from_str(&s[2..4]) {
            Ok(u) => {
                if u == 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
                }
                if u > 12 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_month)),
        };
        ret.day = match u8::from_str(&s[4..6]) {
            Ok(u) => {
                if u == 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
                }
                if u > 31 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_day)),
        };
        ret.hour = match u8::from_str(&s[6..8]) {
            Ok(u) => {
                if u > 23 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour)),
        };
        ret.minute = match u8::from_str(&s[8..10]) {
            Ok(u) => {
                if u > 59 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute)),
        };
        if (len > 12) && value_bytes[10].is_ascii_digit() {
            // Seconds component is present.
            if !value_bytes[11].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            ret.second = match u8::from_str(&s[10..12]) {
                Ok(u) => {
                    if u > 59 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                    }
                    Some(u)
                }
                Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_second)),
            };
        }
        if value_bytes[len - 1] as char != 'Z' {
            if (value_bytes[len - 5] as char != '+') && (value_bytes[len - 5] as char != '-') {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            for byte in value_bytes[len - 4..len].iter() {
                if !byte.is_ascii_digit() {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
                }
            }
            let offset_hour = match i8::from_str(&s[len - 4..len - 2]) {
                Ok(u) => {
                    if u > 12 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
                    }
                    u
                }
                Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
            };
            let offset_minute = match u8::from_str(&s[len - 2..len]) {
                Ok(u) => {
                    if u > 59 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
                    }
                    u
                }
                Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
            };
            ret.utc_offset = Some(UTCOffset {
                hour: if value_bytes[len - 5] as char == '-' {
                    -1 * offset_hour
                } else {
                    offset_hour
                },
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
        let str_form = format!(
            "{:02}{:02}{:02}{:02}{:02}",
            self.year % 100,
            self.month,
            self.day,
            self.hour,
            self.minute,
        );
        f.write_str(&str_form)?;
        match self.second {
            Some(sec) => {
                f.write_str(&format!("{:02}", sec))?;
            }
            _ => (),
        };
        match &self.utc_offset {
            Some(offset) => {
                f.write_str(&format!("{:+03}{:02}", offset.hour, offset.minute))?;
            }
            _ => (),
        };
        Ok(())
    }
}
