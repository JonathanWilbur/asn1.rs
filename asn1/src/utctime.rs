use crate::types::{GeneralizedTime, UTCTime, DATE};

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
