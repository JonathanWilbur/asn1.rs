use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCOffset, UTCTime, DATE, DATE_TIME};
use std::cmp::min;
use std::fmt::Display;
use std::str::FromStr;

impl GeneralizedTime {
    pub fn new() -> Self {
        GeneralizedTime {
            date: DATE::default(),
            utc: true,
            hour: 0,
            minute: None,
            second: None,
            fraction: None,
            utc_offset: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.date.year == 0
            && self.date.month <= 1
            && self.date.day <= 1
            && self.hour == 0
            && self.minute.unwrap_or(0) == 0
            && self.second.unwrap_or(0) == 0
    }
}

impl Default for GeneralizedTime {
    fn default() -> Self {
        GeneralizedTime {
            date: DATE::default(),
            hour: 0,
            minute: None,
            second: None,
            fraction: None,
            utc: true,
            utc_offset: None,
        }
    }
}

impl From<DATE_TIME> for GeneralizedTime {
    fn from(other: DATE_TIME) -> Self {
        GeneralizedTime {
            date: other.date,
            utc: true,
            hour: other.time.hour,
            minute: Some(other.time.minute),
            second: Some(other.time.second),
            fraction: None,
            utc_offset: None,
        }
    }
}

impl From<UTCTime> for GeneralizedTime {
    fn from(other: UTCTime) -> Self {
        let date = DATE::from(other);
        GeneralizedTime {
            date,
            utc: true,
            hour: other.hour,
            minute: Some(other.minute),
            second: other.second,
            fraction: None,
            utc_offset: None,
        }
    }
}

impl From<DATE> for GeneralizedTime {
    fn from(other: DATE) -> Self {
        GeneralizedTime {
            date: other,
            utc: true,
            hour: 0,
            minute: None,
            second: None,
            fraction: None,
            utc_offset: None,
        }
    }
}

impl PartialEq<DATE> for GeneralizedTime {
    fn eq(&self, other: &DATE) -> bool {
        DATE::from(*self).eq(other)
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
        for byte in value_bytes[0..10].iter() {
            if !byte.is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
            }
        }
        let s = match std::str::from_utf8(&value_bytes) {
            Ok(r) => r,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
        let mut date = DATE::default();
        let mut ret = GeneralizedTime::new();
        date.year = match u16::from_str(&s[0..4]) {
            Ok(u) => u,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_year)),
        };
        date.month = match u8::from_str(&s[4..6]) {
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
        date.day = match u8::from_str(&s[6..8]) {
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
        ret.hour = match u8::from_str(&s[8..10]) {
            Ok(u) => {
                if u > 23 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour)),
        };
        if (len > 12) && value_bytes[10].is_ascii_digit() {
            if !value_bytes[11].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
            }
            ret.minute = match u8::from_str(&s[10..12]) {
                Ok(u) => {
                    if u > 59 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
                    }
                    Some(u)
                }
                Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute)),
            };
        }

        if let Some(_) = ret.minute {
            // Normal "if"s cannot be combined with "if let"s.
            if (len > 14) && value_bytes[12].is_ascii_digit() {
                // Seconds component is present.
                if !value_bytes[13].is_ascii_digit() {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                }
                ret.second = match u8::from_str(&s[12..14]) {
                    Ok(u) => {
                        if u > 59 {
                            return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                        }
                        Some(u)
                    }
                    Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_second)),
                };
            }
        }

        if let Some(_) = ret.second {
            if (len >= 16) && ((value_bytes[14] as char == '.') || (value_bytes[14] as char == ','))
            {
                // let frac_byte = value_bytes[15];
                let mut i = 15;
                while i < len && value_bytes[i].is_ascii_digit() {
                    i += 1;
                }
                let end = min(i, 19); // We can only tolerate four digits of precision.
                ret.fraction = match u16::from_str(&s[15..end]) {
                    Ok(u) => {
                        if u > 9999 {
                            return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                        }
                        Some(u)
                    }
                    Err(_) => {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds))
                    }
                };
            }
        }

        if value_bytes[len - 1] as char == 'Z' {
            // ret.utc = true; // This is the default.
            return Ok(ret); // UTCTime
        }

        if (value_bytes[len - 5] as char != '+') && (value_bytes[len - 5] as char != '-') {
            ret.utc = false;
            return Ok(ret); // Local Time
        }

        // For the rest of this function, we are parsing the UTC Offset.
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
        let str_form = format!(
            "{:04}{:02}{:02}{:02}",
            self.date.year % 10000,
            self.date.month,
            self.date.day,
            self.hour,
        );
        f.write_str(&str_form)?;
        match self.minute {
            Some(min) => {
                f.write_str(&format!("{:02}", min))?;
                match self.second {
                    Some(sec) => {
                        f.write_str(&format!("{:02}", sec))?;
                        match self.fraction {
                            Some(ms) => {
                                f.write_str(&format!(".{:03}", ms))?;
                            }
                            _ => (),
                        };
                    }
                    _ => (),
                };
            }
            _ => (),
        };
        match &self.utc_offset {
            Some(offset) => {
                f.write_str(&format!("{:+03}{:02}", offset.hour, offset.minute))?;
            }
            None => {
                f.write_str("Z")?;
            }
        };
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::GeneralizedTime;

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
}
