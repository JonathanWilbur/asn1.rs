use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{GeneralizedTime, UTCTime, DATE_TIME, TIME_OF_DAY};
use std::fmt::Display;
use std::str::FromStr;

impl TIME_OF_DAY {
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        TIME_OF_DAY {
            hour,
            minute,
            second,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.hour == 0 && self.minute == 0 && self.second == 0
    }
}

impl Default for TIME_OF_DAY {
    fn default() -> Self {
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
        let (minute, second) = other.minute.unwrap_or((0, None));
        TIME_OF_DAY {
            hour: other.hour,
            minute,
            second: second.unwrap_or(0),
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
        let str_ = std::str::from_utf8(&value_bytes)
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let hour = u8::from_str(&str_[0..2])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let minute = u8::from_str(&str_[3..5])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
        let second = u8::from_str(&str_[6..])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
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

}
