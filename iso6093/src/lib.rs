#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;

/// An error that can occur while parsing ISO 6093 numbers
#[derive(PartialEq, Eq, Debug)]
pub struct ISO6093Error;

// An ISO 6093 Real Number
#[derive(Debug, PartialEq)]
pub enum ISO6093RealNumber {
    // NR1 Format
    // This is only able to represent integer values
    NR1(f64),

    // NR2 Format
    NR2(f64),

    // NR3 Format
    NR3(f64),
}

/// Parse an ISO 6093 NR1 format number (integer)
///
/// NR1 is a signed or unsigned integer:
/// - Optional leading spaces and zeroes
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits
///
/// Examples: "123", "-456", "+789", "  42", "0042"
pub fn parse_nr1(mut input: &str) -> Result<f64, ISO6093Error> {
    input = input.trim_start_matches(|c| c == ' ');
    input.parse::<i64>()
        .map(|v| v as f64)
        .map_err(|_| ISO6093Error)
}

/// Parse an ISO 6093 NR2 format number (decimal notation)
///
/// NR2 is a signed or unsigned fixed-point number:
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits with a decimal point
/// - The decimal point must be present
///
/// Examples: "123.45", "-0.789", "+12.3"
fn parse_nr2_ex(mut input: &str) -> Result<f64, ISO6093Error> {
    input = input.trim_start_matches(|c| c == ' ');
    // .5 Allowed by Rust, but not by NR2.
    if input.chars().next().is_some_and(|c| c == '.') {
        return Err(ISO6093Error);
    }
    let mut chars = input.chars();
    // +.5 and -.5 Allowed by Rust, but not by NR2.
    if chars.next().is_some_and(|c| c == '+' || c == '-') {
        if chars.next().is_some_and(|c| c == '.') {
            return Err(ISO6093Error);
        }
    }
    let mut has_decimal = false;
    let mut maybe_comma_index: Option<usize> = None;
    for (i, c) in input.char_indices() {
        if !c.is_ascii() {
            return Err(ISO6093Error);
        }
        if c == ',' {
            if has_decimal {
                return Err(ISO6093Error);
            }
            has_decimal = true;
            maybe_comma_index = Some(i);
        }
        else if c == '.' {
            if has_decimal {
                return Err(ISO6093Error);
            }
            has_decimal = true;
        }
        else if c.to_ascii_lowercase() == 'e' {
            return Err(ISO6093Error);
        }
    }

    if !has_decimal {
        return Err(ISO6093Error);
    }

    if let Some(comma_index) = maybe_comma_index {
        #[cfg(feature = "alloc")]
        {
            let mut normalized = input.to_owned();
            unsafe {
                normalized.as_bytes_mut()[comma_index] = b'.';
            }
            return normalized
                .parse::<f64>()
                .map_err(|_| ISO6093Error);
        }
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!();
        }
    } else {
        return input.parse::<f64>().map_err(|_| ISO6093Error);
    }
}

/// Parse an ISO 6093 NR3 format number (scientific notation)
///
/// NR3 is a signed or unsigned number in scientific notation:
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits with a decimal point
/// - The letter 'E' or 'e'
/// - Optional sign for the exponent (+ or -)
/// - Sequence of decimal digits for the exponent
///
/// Examples: "1.23E+45", "-6.78e-9", "+1.0E2"
fn parse_nr3_ex(mut input: &str) -> Result<f64, ISO6093Error> {
    input = input.trim_start_matches(|c| c == ' ');
    let mut has_exponent = false;
    let mut maybe_comma_index: Option<usize> = None;
    let mut has_decimal = false;
    for (i, c) in input.char_indices() {
        if !c.is_ascii() {
            return Err(ISO6093Error);
        }
        if c == '.' || c == ',' {
            if has_decimal { // duplicate
                return Err(ISO6093Error);
            }
            if has_exponent { // decimal after exponent
                return Err(ISO6093Error);
            }
            has_decimal = true;
            if c == ',' {
                maybe_comma_index = Some(i);
            }
        }
        else if c == 'E' || c == 'e' {
            has_exponent = true;
        }
    }

    if !has_exponent || !has_decimal {
        return Err(ISO6093Error);
    }

    if let Some(comma_index) = maybe_comma_index {
        #[cfg(feature = "alloc")]
        {
            let mut normalized = input.to_owned();
            unsafe {
                normalized.as_bytes_mut()[comma_index] = b'.';
            }
            return normalized
                .parse::<f64>()
                .map_err(|_| ISO6093Error);
        }
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!();
        }
    } else {
        return input.parse::<f64>().map_err(|_| ISO6093Error);
    }
}

/// Parse any ISO 6093 Numerical Value
///
/// This implementation does not call [parse_nr3] or [parse_nr2] directly, but
/// instead implements its own parsing logic that is mostly duplicate; the
/// rationale for this is to avoid duplicate iteration over all the characters
/// in the string. We need one pass over all characters to "categorize" the
/// string as NR1, NR2, or NR3. In that first pass, we can also do everything
/// that would have to do in [parse_nr3] or [parse_nr2] as well.
fn parse_iso6093_ex(mut input: &str) -> Result<ISO6093RealNumber, ISO6093Error> {
    input = input.trim_start_matches(|c| c == ' ');
    // .5 Allowed by Rust, but not by NR2.
    if input.chars().next().is_some_and(|c| c == '.') {
        return Err(ISO6093Error);
    }
    let mut chars = input.chars();
    // +.5 and -.5 Allowed by Rust, but not by NR2.
    if chars.next().is_some_and(|c| c == '+' || c == '-') {
        if chars.next().is_some_and(|c| c == '.') {
            return Err(ISO6093Error);
        }
    }

    let mut has_exponent = false;
    let mut has_decimal = false;
    let mut maybe_comma_index: Option<usize> = None;
    for (i, c) in input.char_indices() {
        if !c.is_ascii() {
            return Err(ISO6093Error);
        }
        if c == '.' || c == ',' {
            if has_decimal { // duplicate
                return Err(ISO6093Error);
            }
            if has_exponent { // decimal after exponent
                return Err(ISO6093Error);
            }
            has_decimal = true;
            if c == ',' {
                maybe_comma_index = Some(i);
            }
        }
        else if c == 'E' || c == 'e' {
            has_exponent = true;
        }
    }
    if !has_decimal {
        // This doesn't iterate over characters, so it has little overhead.
        return parse_nr1(input).map(|v| ISO6093RealNumber::NR1(v));
    }

    if let Some(comma_index) = maybe_comma_index {
        #[cfg(feature = "alloc")]
        {
            let mut normalized = input.to_owned();
            unsafe {
                normalized.as_bytes_mut()[comma_index] = b'.';
            }
            return normalized
                .parse::<f64>()
                .map(|v| if has_exponent {
                    ISO6093RealNumber::NR3(v)
                } else {
                    ISO6093RealNumber::NR2(v)
                })
                .map_err(|_| ISO6093Error);
        }
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!();
        }
    } else {
        return input
            .parse::<f64>()
            .map(|v| if has_exponent {
                ISO6093RealNumber::NR3(v)
            } else {
                ISO6093RealNumber::NR2(v)
            })
            .map_err(|_| ISO6093Error);
    }
}

/// Parse an ISO 6093 NR2 format number (decimal notation)
///
/// NR2 is a signed or unsigned fixed-point number:
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits with a decimal point
/// - The decimal point must be present
///
/// Examples: "123.45", "-0.789", "+12.3"
#[cfg(feature = "alloc")]
#[inline(always)]
pub fn parse_nr2(input: &str) -> Result<f64, ISO6093Error> {
    parse_nr2_ex(input)
}

/// Parse an ISO 6093 NR3 format number (scientific notation)
///
/// NR3 is a signed or unsigned number in scientific notation:
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits with a decimal point
/// - The letter 'E' or 'e'
/// - Optional sign for the exponent (+ or -)
/// - Sequence of decimal digits for the exponent
///
/// Examples: "1.23E+45", "-6.78e-9", "+1.0E2"
#[cfg(feature = "alloc")]
#[inline(always)]
pub fn parse_nr3(input: &str) -> Result<f64, ISO6093Error> {
    parse_nr3_ex(input)
}

/// Parse any ISO 6093 Numerical Value
///
/// This implementation does not call [parse_nr3] or [parse_nr2] directly, but
/// instead implements its own parsing logic that is mostly duplicate; the
/// rationale for this is to avoid duplicate iteration over all the characters
/// in the string. We need one pass over all characters to "categorize" the
/// string as NR1, NR2, or NR3. In that first pass, we can also do everything
/// that would have to do in [parse_nr3] or [parse_nr2] as well.
#[cfg(feature = "alloc")]
#[inline(always)]
pub fn parse_iso6093(input: &str) -> Result<ISO6093RealNumber, ISO6093Error> {
    parse_iso6093_ex(input)
}

/// Parse an ISO 6093 NR2 format number (decimal notation)
///
/// NR2 is a signed or unsigned fixed-point number:
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits with a decimal point
/// - The decimal point must be present
///
/// Examples: "123.45", "-0.789", "+12.3"
#[cfg(not(feature = "alloc"))]
pub fn parse_nr2(input: &mut str) -> Result<f64, ISO6093Error> {
    // This loop replaces just the first comma with a period.
    for (i, c) in input.char_indices() {
        if !c.is_ascii() {
            return Err(ISO6093Error);
        }
        if c == ',' {
            unsafe {
                input.as_bytes_mut()[i] = b'.';
            }
            // In the "deeper" functions, we check for duplicate decimals, so
            // we only need to replace one comma.
            break;
        }
    }
    parse_nr2_ex(input)
}

/// Parse an ISO 6093 NR3 format number (scientific notation)
///
/// NR3 is a signed or unsigned number in scientific notation:
/// - Optional leading sign (+ or -)
/// - Sequence of decimal digits with a decimal point
/// - The letter 'E' or 'e'
/// - Optional sign for the exponent (+ or -)
/// - Sequence of decimal digits for the exponent
///
/// Examples: "1.23E+45", "-6.78e-9", "+1.0E2"
#[cfg(not(feature = "alloc"))]
pub fn parse_nr3(input: &mut str) -> Result<f64, ISO6093Error> {
    // This loop replaces just the first comma with a period.
    for (i, c) in input.char_indices() {
        if !c.is_ascii() {
            return Err(ISO6093Error);
        }
        if c == ',' {
            unsafe {
                input.as_bytes_mut()[i] = b'.';
            }
            // In the "deeper" functions, we check for duplicate decimals, so
            // we only need to replace one comma.
            break;
        }
    }
    parse_nr3_ex(input)
}

/// Parse any ISO 6093 Numerical Value
///
/// This implementation does not call [parse_nr3] or [parse_nr2] directly, but
/// instead implements its own parsing logic that is mostly duplicate; the
/// rationale for this is to avoid duplicate iteration over all the characters
/// in the string. We need one pass over all characters to "categorize" the
/// string as NR1, NR2, or NR3. In that first pass, we can also do everything
/// that would have to do in [parse_nr3] or [parse_nr2] as well.
#[cfg(not(feature = "alloc"))]
pub fn parse_iso6093(input: &mut str) -> Result<ISO6093RealNumber, ISO6093Error> {
    for (i, c) in input.char_indices() {
        if !c.is_ascii() {
            return Err(ISO6093Error);
        }
        if c == ',' {
            unsafe {
                input.as_bytes_mut()[i] = b'.';
            }
            // In the "deeper" functions, we check for duplicate decimals, so
            // we only need to replace one comma.
            break;
        }
    }
    parse_iso6093_ex(input)
}

/// Format a number using ISO 6093 First Numerical Representation (NR1)
#[inline]
pub fn fmt_nr1(num: f64, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let inum: i64 = num as i64;
    write!(f, "{}", inum)
}

/// Format a number using ISO 6093 Second Numerical Representation (NR2)
///
/// This is marked as `unsafe` because floating point numbers can be printed
/// using scientific notation (which is invalid NR2 form) if they are "too big"
/// or "too small," and there is absolutely no way I can predict this or
/// circumvent it. There is no way to force fixed-point notation in Rust.
pub unsafe fn fmt_nr2(num: f64, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    /* Typically, you check if it is an integer by checking if .fract() == 0.0,
    but fract() is not available in no-std crates. Further, it is already
    accepted that this is an unsafe function that might not do what you expect. */
    if num.is_finite() && (num == (num as i64 as f64)) {
        let inum: i64 = num as i64;
        return write!(f, "{}.", inum); // Format it with a guaranteed terminal decimal.
    }
    write!(f, "{}", num)
}

/// Format a number using ISO 6093 Third Numerical Representation (NR3)
#[inline]
pub fn fmt_nr3(num: f64, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{:e}", num)
}

impl core::fmt::Display for ISO6093RealNumber {

    /// Note that NR2 may be printed like NR3. See the documentation on
    /// [fmt_nr2].
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ISO6093RealNumber::NR1(v) => fmt_nr1(*v, f),
            ISO6093RealNumber::NR2(v) => unsafe { fmt_nr2(*v, f) },
            ISO6093RealNumber::NR3(v) => fmt_nr3(*v, f),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;
    use alloc::string::ToString;

    #[cfg(feature = "alloc")]
    macro_rules! s {
        ($arg:expr) => {
            $arg
        }
    }

    #[cfg(not(feature = "alloc"))]
    macro_rules! s {
        ($arg:expr) => {
            alloc::string::ToString::to_string($arg).as_mut_str()
        };
    }

    #[test]
    fn test_parse_nr1() {
        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_nr1(s!("0004902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!("  4902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!("   4902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!("0001234")), Ok(1234.0));
        assert_eq!(parse_nr1(s!("   1234")), Ok(1234.0));
        assert_eq!(parse_nr1(s!("0000000")), Ok(0.0));
        assert_eq!(parse_nr1(s!("      0")), Ok(0.0));
        assert_eq!(parse_nr1(s!("1234567")), Ok(1234567.0));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_nr1(s!("+004902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!(" +4902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!("  +4902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!("   4902")), Ok(4902.0));
        assert_eq!(parse_nr1(s!("+001234")), Ok(1234.0));
        assert_eq!(parse_nr1(s!("  +1234")), Ok(1234.0));
        assert_eq!(parse_nr1(s!("   1234")), Ok(1234.0));
        assert_eq!(parse_nr1(s!("-56780")), Ok(-56780.0));
        assert_eq!(parse_nr1(s!(" -56780")), Ok(-56780.0));
        assert_eq!(parse_nr1(s!("+000000")), Ok(0.0));
        assert_eq!(parse_nr1(s!("     +0")), Ok(0.0));
        assert_eq!(parse_nr1(s!("      0")), Ok(0.0));

        // Other tests
        assert_eq!(parse_nr1(s!("123")), Ok(123.0));
        assert_eq!(parse_nr1(s!("-456")), Ok(-456.0));
        assert_eq!(parse_nr1(s!("+789")), Ok(789.0));
        assert_eq!(parse_nr1(s!("")), Err(ISO6093Error));
        assert_eq!(parse_nr1(s!("12.3")), Err(ISO6093Error));
        assert_eq!(parse_nr1(s!("12E3")), Err(ISO6093Error));
    }

    #[test]
    fn test_parse_nr2() {
        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_nr2(s!("1327.000")), Ok(1327.0));
        assert_eq!(parse_nr2(s!("0001327.")), Ok(1327.0));
        assert_eq!(parse_nr2(s!("   1327.")), Ok(1327.0));
        assert_eq!(parse_nr2(s!("00123.45")), Ok(123.45));
        assert_eq!(parse_nr2(s!("  123.45")), Ok(123.45));
        assert_eq!(parse_nr2(s!("  1237.0")), Ok(1237.0));
        assert_eq!(parse_nr2(s!("00.00001")), Ok(0.00001));
        assert_eq!(parse_nr2(s!("1234,567")), Ok(1234.567));
        assert_eq!(parse_nr2(s!("000,0000")), Ok(0.0));
        assert_eq!(parse_nr2(s!("     0,0")), Ok(0.0));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_nr2(s!("+1327.00")), Ok(1327.00));
        assert_eq!(parse_nr2(s!("  +1327.")), Ok(1327.0));
        assert_eq!(parse_nr2(s!("   1327.")), Ok(1327.0));
        assert_eq!(parse_nr2(s!(" +123.45")), Ok(123.45));
        assert_eq!(parse_nr2(s!("  123,45")), Ok(123.45));
        assert_eq!(parse_nr2(s!(" +1237.0")), Ok(1237.0));
        assert_eq!(parse_nr2(s!("  1237,0")), Ok(1237.0));
        assert_eq!(parse_nr2(s!("+0.00001")), Ok(0.00001));
        assert_eq!(parse_nr2(s!("-5,67800")), Ok(-5.67800));
        assert_eq!(parse_nr2(s!("-05,6780")), Ok(-5.6780));
        assert_eq!(parse_nr2(s!("+0.00000")), Ok(0.0));
        assert_eq!(parse_nr2(s!("    +0,0")), Ok(0.0));
        assert_eq!(parse_nr2(s!("     0,0")), Ok(0.0));
        assert_eq!(parse_nr2(s!("      0,")), Ok(0.0));

        // Other tests
        assert_eq!(parse_nr2(s!("123.45")), Ok(123.45));
        assert_eq!(parse_nr2(s!("-67.89")), Ok(-67.89));
        assert_eq!(parse_nr2(s!("+0.123")), Ok(0.123));
        assert_eq!(parse_nr2(s!("")), Err(ISO6093Error));
        assert_eq!(parse_nr2(s!("123")), Err(ISO6093Error));
        assert_eq!(parse_nr2(s!("12E3")), Err(ISO6093Error));
    }

    #[test]
    fn test_parse_nr3() {
        // Examples directly from ISO 6093
        assert_eq!(parse_nr3(s!("+0,56E+4")), Ok(5600.0));
        assert_eq!(parse_nr3(s!("+5.6e+03")), Ok(5600.0));
        assert_eq!(parse_nr3(s!("+0,3E-04")), Ok(0.00003));
        assert_eq!(parse_nr3(s!(" 0,3e-04")), Ok(0.00003));
        assert_eq!(parse_nr3(s!("-2,8E+00")), Ok(-2.8));
        assert_eq!(parse_nr3(s!("+0,0E+00")), Ok(0.0));
        assert_eq!(parse_nr3(s!("   0.e+0")), Ok(0.0));

        // Other tests
        assert_eq!(parse_nr3(s!("1.23E+45")), Ok(1.23E+45));
        assert_eq!(parse_nr3(s!("-6.78e-9")), Ok(-6.78e-9));
        assert_eq!(parse_nr3(s!("+1.0E2")), Ok(100.0));
        assert_eq!(parse_nr3(s!("")), Err(ISO6093Error));
        assert_eq!(parse_nr3(s!("123.45")), Err(ISO6093Error));
        assert_eq!(parse_nr3(s!("123")), Err(ISO6093Error));
    }

    #[test]
    fn test_parse_iso6093() {
        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_iso6093(s!("0004902")), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!("  4902")),  Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!("   4902")), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!("0001234")), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093(s!("   1234")), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093(s!("0000000")), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093(s!("      0")), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093(s!("1234567")), Ok(ISO6093RealNumber::NR1(1234567.0)));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_iso6093(s!("+004902")), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!(" +4902")),  Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!("  +4902")), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!("   4902")), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(s!("+001234")), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093(s!("  +1234")), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093(s!("   1234")), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093(s!("-56780")),  Ok(ISO6093RealNumber::NR1(-56780.0)));
        assert_eq!(parse_iso6093(s!(" -56780")), Ok(ISO6093RealNumber::NR1(-56780.0)));
        assert_eq!(parse_iso6093(s!("+000000")), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093(s!("     +0")), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093(s!("      0")), Ok(ISO6093RealNumber::NR1(0.0)));

        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_iso6093(s!("1327.000")), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093(s!("0001327.")), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093(s!("   1327.")), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093(s!("00123.45")), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093(s!("  123.45")), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093(s!("  1237.0")), Ok(ISO6093RealNumber::NR2(1237.0)));
        assert_eq!(parse_iso6093(s!("00.00001")), Ok(ISO6093RealNumber::NR2(0.00001)));
        assert_eq!(parse_iso6093(s!("1234,567")), Ok(ISO6093RealNumber::NR2(1234.567)));
        assert_eq!(parse_iso6093(s!("000,0000")), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093(s!("     0,0")), Ok(ISO6093RealNumber::NR2(0.0)));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_iso6093(s!("+1327.00")), Ok(ISO6093RealNumber::NR2(1327.00)));
        assert_eq!(parse_iso6093(s!("  +1327.")), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093(s!("   1327.")), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093(s!(" +123.45")), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093(s!("  123,45")), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093(s!(" +1237.0")), Ok(ISO6093RealNumber::NR2(1237.0)));
        assert_eq!(parse_iso6093(s!("  1237,0")), Ok(ISO6093RealNumber::NR2(1237.0)));
        assert_eq!(parse_iso6093(s!("+0.00001")), Ok(ISO6093RealNumber::NR2(0.00001)));
        assert_eq!(parse_iso6093(s!("-5,67800")), Ok(ISO6093RealNumber::NR2(-5.67800)));
        assert_eq!(parse_iso6093(s!("-05,6780")), Ok(ISO6093RealNumber::NR2(-5.6780)));
        assert_eq!(parse_iso6093(s!("+0.00000")), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093(s!("    +0,0")), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093(s!("     0,0")), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093(s!("      0,")), Ok(ISO6093RealNumber::NR2(0.0)));

        // Examples directly from ISO 6093
        assert_eq!(parse_iso6093(s!("+0,56E+4")), Ok(ISO6093RealNumber::NR3(5600.0)));
        assert_eq!(parse_iso6093(s!("+5.6e+03")), Ok(ISO6093RealNumber::NR3(5600.0)));
        assert_eq!(parse_iso6093(s!("+0,3E-04")), Ok(ISO6093RealNumber::NR3(0.00003)));
        assert_eq!(parse_iso6093(s!(" 0,3e-04")), Ok(ISO6093RealNumber::NR3(0.00003)));
        assert_eq!(parse_iso6093(s!("-2,8E+00")), Ok(ISO6093RealNumber::NR3(-2.8)));
        assert_eq!(parse_iso6093(s!("+0,0E+00")), Ok(ISO6093RealNumber::NR3(0.0)));
        assert_eq!(parse_iso6093(s!("   0.e+0")), Ok(ISO6093RealNumber::NR3(0.0)));
    }

    #[test]
    fn test_print_nr1() {
        assert_eq!(ISO6093RealNumber::NR1(123.0).to_string().as_str(), "123");
        assert_eq!(ISO6093RealNumber::NR1(-123.0).to_string().as_str(), "-123");
        assert_eq!(ISO6093RealNumber::NR1(123.5).to_string().as_str(), "123");
    }

    #[test]
    fn test_print_nr2() {
        assert_eq!(ISO6093RealNumber::NR2(123.0).to_string().as_str(), "123.");
        assert_eq!(ISO6093RealNumber::NR2(-123.0).to_string().as_str(), "-123.");
        assert_eq!(ISO6093RealNumber::NR2(123.5).to_string().as_str(), "123.5");
        assert_eq!(ISO6093RealNumber::NR2(0.00123).to_string().as_str(), "0.00123");
    }

    #[test]
    fn test_print_nr3() {
        assert_eq!(ISO6093RealNumber::NR3(12300.0).to_string().as_str(), "1.23e4");
        assert_eq!(ISO6093RealNumber::NR3(-12300.0).to_string().as_str(), "-1.23e4");
        assert_eq!(ISO6093RealNumber::NR3(12300.5).to_string().as_str(), "1.23005e4");
        assert_eq!(ISO6093RealNumber::NR3(0.00123).to_string().as_str(), "1.23e-3");
    }
}
