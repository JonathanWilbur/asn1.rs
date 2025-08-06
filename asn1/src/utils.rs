//! Various utilities
//!
//! Decode variable-length integers, which are used in multiple different encodings:
//!
//! ```rust
//! use wildboar_asn1::utils::{read_i64, read_i128};
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

    #[cfg(feature = "atoi_simd")]
    macro_rules! parse_uint {
        ( $inttype:ty, $bytes:expr, $string:expr, $errcode:expr ) => {
            atoi_simd::parse_pos::<$inttype>($bytes)
                .map_err(|_| ASN1Error::new($errcode))?
        };
    }

    #[cfg(not(feature = "atoi_simd"))]
    macro_rules! parse_uint {
        ( $inttype:ty, $bytes:expr, $string:expr, $errcode:expr ) => {
            <$inttype>::from_str($string)
                .map_err(|_| ASN1Error::new($errcode))?
        };
    }

    pub(crate) use parse_uint;

    /// This is implemented as a macro so that the <128 case is effectively
    /// "inlined" _and_ has access to `push()`. If we just called
    /// `write_big_oid_arc`
    macro_rules! write_oid_arc {
        ( $oid:expr, $arc:expr ) => {
            if $crate::likely($arc < 128) {
                $oid.push($arc as u8);
            } else {
                // A u128 can take up to 19 bytes. We do 20 just for safety.
                // let mut num = $arc;
                let mut encoded: [u8; 20] = [0; 20];
                let mut byte_count: usize = 0;
                let mut output_len = 0;
                let mut i = $arc;
                // Determine the output length.
                while i > 0 {
                    output_len += 1;
                    i >>= 7;
                }
                let mut j = output_len - 1;
                while j >= 0 {
                    let mut out_byte = ($arc >> (j * 7));
                    out_byte &= 0b0111_1111;
                    if j != 0 {
                        // Set the continuation bit.
                        out_byte |= 0b1000_0000;
                    }
                    encoded[byte_count] = out_byte as u8;
                    byte_count += 1;
                    j -= 1;
                }
                $oid.extend_from_slice(&encoded[0..byte_count])
            }
        };
    }

    pub(crate) use write_oid_arc;
}

#[cfg(feature = "likely_stable")]
#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    likely_stable::likely(expr)
}

#[cfg(feature = "likely_stable")]
#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    likely_stable::unlikely(expr)
}

#[cfg(not(feature = "likely_stable"))]
#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    expr
}

#[cfg(not(feature = "likely_stable"))]
#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    expr
}
