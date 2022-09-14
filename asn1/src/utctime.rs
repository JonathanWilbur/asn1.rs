use crate::types::{UTCTime};

impl UTCTime {
    pub fn new () -> Self {
        UTCTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: None, utc_offset: None }
    }
}