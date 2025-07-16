//! Utility functions for branch prediction optimization.
//! 
//! This module provides wrapper functions around branch prediction hints that can help
//! the compiler optimize code paths. When the `likely_stable` feature is enabled,
//! these functions use the `likely_stable` crate for branch prediction hints.
//! When the feature is disabled, they simply return the input expression unchanged.

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
