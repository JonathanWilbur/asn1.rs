//! Various utilities
//!
//! Decode variable-length integers, which are used in multiple different encodings:
//!
//! ```rust
//! use asn1::utils::{read_i64, read_i128};
//! assert_eq!{read_i64(&[ 0x01, 0x05 ]), Some(256 + 5)}
//! assert_eq!{read_i128(&[ 0x01, 0x05 ]), Some(256 + 5)}
//! ```
use crate::ByteSlice;

/// Attempt to read a variable-length big-endian signed integer (encoded as
/// two's complement) from a byte slice (`&[u8]`) into an `i64`. This is
/// primarily used for reading an ASN.1 `INTEGER` or `ENUMERATED` value into a
/// more useful form.
///
/// No validation is performed on the byte slice `bytes`, not that there is
/// really to be done, other than checking for padding bytes.
///
/// If the encoded value is too large to fit into an `i64`, `None` is returned.
pub fn read_i64(bytes: ByteSlice) -> Option<i64> {
    let len = bytes.len();
    match len {
        1 => Some(bytes[0] as i64),
        2 => Some(i16::from_be_bytes([bytes[0], bytes[1]]) as i64),
        3 => Some(i32::from_be_bytes([
            if bytes[0] & 0b1000_0000 > 0 {
                0xFF
            } else {
                0x00
            },
            bytes[0],
            bytes[1],
            bytes[2],
        ]) as i64),
        4 => Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64),
        5..=8 => {
            let mut buf: [u8; 8] = [0; 8];
            buf[8 - len..].copy_from_slice(bytes);
            Some(i64::from_be_bytes(buf))
        }
        _ => None,
    }
}

/// Attempt to read a variable-length big-endian signed integer (encoded as
/// two's complement) from a byte slice (`&[u8]`) into an `i128`. This is
/// primarily used for reading an ASN.1 `INTEGER` or `ENUMERATED` value into a
/// more useful form.
///
/// No validation is performed on the byte slice `bytes`, not that there is
/// really to be done, other than checking for padding bytes.
///
/// If the encoded value is too large to fit into an `i128`, `None` is returned.
pub fn read_i128(bytes: ByteSlice) -> Option<i128> {
    let len = bytes.len();
    match len {
        1 => Some(bytes[0] as i128),
        2 => Some(i16::from_be_bytes([bytes[0], bytes[1]]) as i128),
        3 => Some(i32::from_be_bytes([
            if bytes[0] & 0b1000_0000 > 0 {
                0xFF
            } else {
                0x00
            },
            bytes[0],
            bytes[1],
            bytes[2],
        ]) as i128),
        4 => Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i128),
        5..=16 => {
            let mut buf: [u8; 16] = [0; 16];
            buf[16 - len..].copy_from_slice(bytes);
            Some(i128::from_be_bytes(buf))
        }
        _ => None,
    }
}

/// Write a single `OBJECT IDENTIFIER` arc, using the Variable-Length Quantity
/// (VLQ) serialization that is used to encode arcs in X.690 encodings and
/// others, to a writeable output, returning the number of bytes written within
/// an [std::io::Result].
pub fn write_oid_arc<W>(output: &mut W, mut num: u128) -> std::io::Result<usize>
where
    W: std::io::Write
{
    if likely(num < 128) {
        return output.write(&[num as u8]);
    }

    // A u128 can take up to 19 bytes. We do 20 just for safety.
    let mut encoded: [u8; 20] = [0; 20];
    let mut byte_count: usize = 0;
    while num > 0b0111_1111 {
        encoded[byte_count] = (num & 0b0111_1111) as u8 | 0b1000_0000;
        byte_count += 1;
        num >>= 7;
    }
    encoded[byte_count] = num as u8;
    output.write(&encoded[0..byte_count+1])
}

/// This is not a time library.
#[inline]
pub(crate) const fn get_days_in_month (year: u16, month: u8) -> u8 {
    let is_leap_year = ((year % 4) == 0) && (((year % 100) > 0) || ((year % 400) == 0));
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => if is_leap_year { 29 } else { 28 },
        _ => 30,
    }
}

pub(crate) mod macros {
    macro_rules! parse_uint {
        ( $inttype:ty, $bytes:expr, $string:expr, $errcode:expr ) => {
            if cfg!(feature = "atoi_simd") {
                atoi_simd::parse_pos::<$inttype>($bytes)
                    .map_err(|_| ASN1Error::new($errcode))?
            } else {
                <$inttype>::from_str($string)
                    .map_err(|_| ASN1Error::new($errcode))?
            }
        };
    }

    pub(crate) use parse_uint;
}

#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    if cfg!(feature = "likely_stable") {
        likely_stable::likely(expr)
    } else {
        expr
    }
}

#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    if cfg!(feature = "likely_stable") {
        likely_stable::unlikely(expr)
    } else {
        expr
    }
}
