use crate::types::{DATE, UTCTime, GeneralizedTime};

impl DATE {
    pub fn new() -> Self {
        DATE {
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl From<GeneralizedTime> for DATE {

    fn from(other: GeneralizedTime) -> Self {
        other.date
    }

}

impl From<UTCTime> for DATE {

    fn from(other: UTCTime) -> Self {
        DATE {
            /* The conversion below was taken from ITU Recommendation X.509
            (2019), Section 7.2.1. */
            year: if other.year >= 50 {
                1900 + other.year as u16
            } else {
                2000 + other.year as u16
            },
            month: other.month,
            day: other.day,
        }
    }

}

impl PartialEq<GeneralizedTime> for DATE {

    fn eq(&self, other: &GeneralizedTime) -> bool {
        DATE::from(*other).eq(self)
    }

}

impl PartialEq<UTCTime> for DATE {

    fn eq(&self, other: &UTCTime) -> bool {
        DATE::from(*other).eq(self)
    }

}
