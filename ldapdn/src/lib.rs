//! LDAP Distinguished Name (DN) parsing per IETF RFC 4514
//!
//! See: <https://datatracker.ietf.org/doc/html/rfc4514>
#![doc = include_str!("../README.md")]
#![no_std]
pub mod escape;
pub mod parse;
