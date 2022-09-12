use crate::types::{DURATION_EQUIVALENT};

impl DURATION_EQUIVALENT {

    pub fn new () -> Self {
        return DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: None
        };
    }

}