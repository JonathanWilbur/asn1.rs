use crate::types::{GeneralizedTime, DATE};

impl GeneralizedTime {
    pub fn new () -> Self {
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