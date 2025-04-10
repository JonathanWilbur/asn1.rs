use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::ISO8601Timestampable;
use std::fmt::Display;
use std::str::FromStr;

use crate::types::{GeneralizedTime, UTCTime, DATE, DATE_TIME, TIME_OF_DAY};

impl DATE_TIME {
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

    pub fn is_zero(&self) -> bool {
        self.date.year == 0
            && self.date.month <= 1
            && self.date.day <= 1
            && self.time.hour == 0
            && self.time.minute == 0
            && self.time.second == 0
    }
}

impl ISO8601Timestampable for DATE_TIME {

    /// NOTE: There is not supposed to be a "Z" at the end of this, nor should
    /// there be a UTC offset. ITU-T Recommendation X.680 defines the
    /// `DATE-TIME` type as being in local time with no UTC offset indication.
    /// For this reason, I recommend against the use of `DATE-TIME`.
    fn to_iso_8601_string (&self) -> String {
        return format!("{}T{}", self.date, self.time);
    }

}

impl Default for DATE_TIME {
    fn default() -> Self {
        DATE_TIME {
            date: DATE::default(),
            time: TIME_OF_DAY::default(),
        }
    }
}

impl From<GeneralizedTime> for DATE_TIME {
    fn from(other: GeneralizedTime) -> Self {
        let (minute, second) = other.minute.unwrap_or((0, None));
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
    fn eq(&self, other: &GeneralizedTime) -> bool {
        DATE_TIME::from(*other).eq(self)
    }
}

impl PartialEq<UTCTime> for DATE_TIME {
    fn eq(&self, other: &UTCTime) -> bool {
        DATE_TIME::from(*other).eq(self)
    }
}

impl TryFrom<&[u8]> for DATE_TIME {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if value_bytes.len() != 19 {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DATE_TIME::try_from(s.as_bytes())
    }
}

// FIXME: Fewer allocations approach
impl Display for DATE_TIME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_form = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
        );
        f.write_str(&str_form)
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

}
