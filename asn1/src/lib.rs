#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]

pub mod bitstring;
pub mod constants;
pub mod construction;
pub mod date;
pub mod datetime;
pub mod display;
pub mod duration;
pub mod error;
pub mod external;
pub mod gentime;
pub mod oid;
pub mod roid;
pub mod strings;
pub mod tag;
pub mod time_of_day;
pub mod types;
pub mod utctime;
pub mod utils;

pub use crate::bitstring::*;
pub use crate::constants::*;
pub use crate::construction::*;
pub use crate::display::*;
pub use crate::error::*;
pub use crate::strings::*;
pub use crate::types::*;
pub use crate::utils::*;

pub const TRUE: bool = true;
pub const FALSE: bool = false;
