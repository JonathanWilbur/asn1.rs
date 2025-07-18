//! Utility functions for branch prediction optimization.
//! 
//! This module provides wrapper functions around branch prediction hints that can help
//! the compiler optimize code paths. When the `likely_stable` feature is enabled,
//! these functions use the `likely_stable` crate for branch prediction hints.
//! When the feature is disabled, they simply return the input expression unchanged.
use wildboar_asn1::ByteSlice;

/// Indicates that a boolean expression is likely to be true.
/// 
/// This function provides a hint to the compiler that the given boolean expression
/// is expected to evaluate to `true` in most cases. This can help the compiler
/// optimize the code by placing the likely path in a more efficient location.
/// 
/// When the `likely_stable` feature is enabled, this function uses the `likely_stable::likely`
/// function to provide the hint. When the feature is disabled, it simply returns
/// the input expression without any optimization hints.
/// 
/// # Arguments
/// 
/// * `expr` - The boolean expression that is likely to be true
/// 
/// # Returns
/// 
/// The same boolean value as the input expression
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::utils::likely;
/// 
/// let value = 42;
/// if likely(value > 0) {
///     // This branch is optimized as the likely path
///     println!("Value is positive");
/// } else {
///     println!("Value is not positive");
/// }
/// ```
#[cfg(feature = "likely_stable")]
#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    likely_stable::likely(expr)
}

/// Indicates that a boolean expression is likely to be false.
/// 
/// This function provides a hint to the compiler that the given boolean expression
/// is expected to evaluate to `false` in most cases. This can help the compiler
/// optimize the code by placing the unlikely path in a less critical location.
/// 
/// When the `likely_stable` feature is enabled, this function uses the `likely_stable::unlikely`
/// function to provide the hint. When the feature is disabled, it simply returns
/// the input expression without any optimization hints.
/// 
/// # Arguments
/// 
/// * `expr` - The boolean expression that is likely to be false
/// 
/// # Returns
/// 
/// The same boolean value as the input expression
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::utils::unlikely;
/// 
/// let value = 42;
/// if unlikely(value < 0) {
///     // This branch is optimized as the unlikely path
///     println!("Value is negative");
/// } else {
///     println!("Value is not negative");
/// }
/// ```
#[cfg(feature = "likely_stable")]
#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    likely_stable::unlikely(expr)
}

/// Indicates that a boolean expression is likely to be true.
/// 
/// This function provides a hint to the compiler that the given boolean expression
/// is expected to evaluate to `true` in most cases. This can help the compiler
/// optimize the code by placing the likely path in a more efficient location.
/// 
/// When the `likely_stable` feature is enabled, this function uses the `likely_stable::likely`
/// function to provide the hint. When the feature is disabled, it simply returns
/// the input expression without any optimization hints.
/// 
/// # Arguments
/// 
/// * `expr` - The boolean expression that is likely to be true
/// 
/// # Returns
/// 
/// The same boolean value as the input expression
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::utils::likely;
/// 
/// let value = 42;
/// if likely(value > 0) {
///     // This branch is optimized as the likely path
///     println!("Value is positive");
/// } else {
///     println!("Value is not positive");
/// }
/// ```
#[cfg(not(feature = "likely_stable"))]
#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    expr
}

/// Indicates that a boolean expression is likely to be false.
/// 
/// This function provides a hint to the compiler that the given boolean expression
/// is expected to evaluate to `false` in most cases. This can help the compiler
/// optimize the code by placing the unlikely path in a less critical location.
/// 
/// When the `likely_stable` feature is enabled, this function uses the `likely_stable::unlikely`
/// function to provide the hint. When the feature is disabled, it simply returns
/// the input expression without any optimization hints.
/// 
/// # Arguments
/// 
/// * `expr` - The boolean expression that is likely to be false
/// 
/// # Returns
/// 
/// The same boolean value as the input expression
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::utils::unlikely;
/// 
/// let value = 42;
/// if unlikely(value < 0) {
///     // This branch is optimized as the unlikely path
///     println!("Value is negative");
/// } else {
///     println!("Value is not negative");
/// }
/// ```
#[cfg(not(feature = "likely_stable"))]
#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    expr
}

/// Convert a `Vec<u16>` to a `Vec<u8>` in place.
pub(crate) fn vec_u16_to_vec_u8(mut vec: Vec<u16>) -> Vec<u8> {
    let len = vec.len();
    let capacity = vec.capacity();
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent dropping the original Vec<u16>
    unsafe {
        Vec::from_raw_parts(ptr.cast::<u8>(), len * 2, capacity * 2)
    }
}

/// Convert a `Vec<u32>` to a `Vec<u8>` in place.
pub(crate) fn vec_u32_to_vec_u8(mut vec: Vec<u32>) -> Vec<u8> {
    let len = vec.len();
    let capacity = vec.capacity();
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent dropping the original Vec<u32>
    unsafe {
        Vec::from_raw_parts(ptr.cast::<u8>(), len * 4, capacity * 4)
    }
}

/// Get the number of days in a month.
///
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

/// Decode a variable-length `u64` from a byte slice
pub fn x690_read_var_length_u64(bytes: ByteSlice) -> Option<u64> {
    match bytes.len() {
        0 => Some(0),
        1 => Some(bytes[0] as u8 as u64),
        2 => Some(u16::from_be_bytes([bytes[0], bytes[1]]) as u64),
        3 => Some(u32::from_be_bytes([0x00, bytes[0], bytes[1], bytes[2]]) as u64),
        4 => Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64),
        5 => Some(u64::from_be_bytes([
            0x00, 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ])),
        6 => Some(u64::from_be_bytes([
            0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
        ])),
        7 => Some(u64::from_be_bytes([
            0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        ])),
        8 => Some(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])),
        _ => None,
    }
}
