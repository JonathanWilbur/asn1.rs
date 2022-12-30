use crate::error::{ASN1Error, ASN1ErrorCode};
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
        DATE_TIME {
            date: DATE {
                year: other.date.year,
                month: other.date.month,
                day: other.date.day,
            },
            time: TIME_OF_DAY {
                hour: other.hour,
                minute: other.minute.unwrap_or(0),
                second: other.second.unwrap_or(0),
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
                second: other.second.unwrap_or(0),
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
