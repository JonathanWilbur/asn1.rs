#![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod bitstring;
pub mod constants;
pub mod construction;
pub mod date;
pub mod duration;
pub mod error;
pub mod gentime;
pub mod tag;
pub mod time_of_day;
pub mod types;
pub mod utctime;

pub use crate::bitstring::*;
pub use crate::constants::*;
pub use crate::construction::*;
pub use crate::date::*;
pub use crate::duration::*;
pub use crate::error::*;
pub use crate::gentime::*;
pub use crate::tag::*;
pub use crate::time_of_day::*;
pub use crate::types::*;
pub use crate::utctime::*;

pub const TRUE: bool = true;
pub const FALSE: bool = false;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
