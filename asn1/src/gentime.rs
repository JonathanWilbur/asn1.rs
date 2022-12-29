use crate::types::{GeneralizedTime, DATE, UTCTime};

impl GeneralizedTime {
    pub fn new() -> Self {
        GeneralizedTime {
            date: DATE::new(),
            utc: true,
            hour: 0,
            minute: None,
            second: None,
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
