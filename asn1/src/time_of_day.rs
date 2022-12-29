use crate::types::{TIME_OF_DAY, GeneralizedTime, UTCTime};

impl TIME_OF_DAY {
    pub fn new() -> Self {
        TIME_OF_DAY {
            hour: 0,
            minute: 0,
            second: 0,
        }
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
