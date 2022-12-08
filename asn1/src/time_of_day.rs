use crate::types::TIME_OF_DAY;

impl TIME_OF_DAY {
    pub fn new() -> Self {
        TIME_OF_DAY {
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}
