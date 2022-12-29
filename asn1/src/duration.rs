use std::time::Duration;

use crate::types::DURATION_EQUIVALENT;

impl DURATION_EQUIVALENT {
    pub fn new() -> Self {
        DURATION_EQUIVALENT::default()
    }
}

impl Default for DURATION_EQUIVALENT {

    fn default() -> Self {
        DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: None,
        }
    }

}

impl TryFrom<Duration> for DURATION_EQUIVALENT {
    type Error = std::num::TryFromIntError;

    fn try_from(other: Duration) -> Result<Self, Self::Error> {
        Ok(DURATION_EQUIVALENT {
            seconds: other.as_secs().try_into()?,
            ..Default::default()
        })
    }

}
