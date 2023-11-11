pub mod pdu;
mod checksum;
mod classes;
mod decode;
mod encode;
mod service;
mod layer;
mod entity;
mod conn;
mod convert;
mod procedures;
pub use decode::*;
pub use encode::*;
pub use service::*;
pub use layer::*;
pub use entity::*;
pub use conn::*;
pub use pdu::*;
pub use convert::*;

pub type UserData = Vec<u8>;
pub type ParameterCode = u8;
pub type TransportRef = u16;

pub const DEFAULT_MAX_TSDU_SIZE_FOR_OSI: u8 = 128;
///
/// According to [IETF RFC 1006](https://datatracker.ietf.org/doc/html/rfc1006):
///
/// > In order to achieve good performance, the default TPDU size is 65531
/// > octets, instead of 128 octets.
///
pub const DEFAULT_MAX_TSDU_SIZE_FOR_ITOT: usize = 65531;
pub const DEFAULT_MAX_TSDU_SIZE: usize = DEFAULT_MAX_TSDU_SIZE_FOR_ITOT;
pub const DEFAULT_MAX_TPDU_SIZE: u8 = 128;
