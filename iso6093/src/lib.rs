// #![no_std]

/// Error types that can occur while parsing ISO 6093 numbers
#[derive(Debug, PartialEq)]
pub enum ISO6093Error {
    /// Input string was empty
    EmptyString,
    /// Input contains invalid characters for the format
    InvalidFormat,
    /// Failed to parse the number
    ParseError,
}

#[derive(Debug, PartialEq)]
pub enum ISO6093RealNumber {
    NR1(f64),
    NR2(f64),
    NR3(f64),
}

// TODO: Print

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
        .map_err(|_| ISO6093Error::ParseError)
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
        return Err(ISO6093Error::InvalidFormat);
    }
    let mut chars = input.chars();
    // +.5 and -.5 Allowed by Rust, but not by NR2.
    if chars.next().is_some_and(|c| c == '+' || c == '-') {
        if chars.next().is_some_and(|c| c == '.') {
            return Err(ISO6093Error::InvalidFormat);
        }
    }
    let mut needs_replace = false;
    let mut has_decimal = false;
    for c in input.chars() {
        if c == ',' {
            if has_decimal {
                return Err(ISO6093Error::InvalidFormat);
            }
            has_decimal = true;
            needs_replace = true;
        }
        if c == '.' {
            if has_decimal {
                return Err(ISO6093Error::InvalidFormat);
            }
            has_decimal = true;
        }
        if c.to_ascii_lowercase() == 'e' {
            return Err(ISO6093Error::InvalidFormat);
        }
    }

    if !has_decimal {
        return Err(ISO6093Error::InvalidFormat);
    }

    if !needs_replace {
        return input.parse::<f64>().map_err(|_| ISO6093Error::ParseError);
    }

    input
        .replacen(',', ".", 1) // We already checked that there's only one.
        .parse::<f64>()
        .map_err(|_| ISO6093Error::ParseError)
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
    let mut needs_replace = false;
    let mut has_decimal = false;
    for c in input.chars() {
        if c == '.' || c == ',' {
            if has_decimal { // duplicate
                return Err(ISO6093Error::InvalidFormat);
            }
            if has_exponent { // decimal after exponent
                return Err(ISO6093Error::InvalidFormat);
            }
            has_decimal = true;
            needs_replace = c == ',';
        }
        if c == 'E' || c == 'e' {
            has_exponent = true;
        }
    }

    if !has_exponent || !has_decimal {
        return Err(ISO6093Error::InvalidFormat);
    }

    if !needs_replace {
        return input.parse::<f64>().map_err(|_| ISO6093Error::ParseError);
    }

    input
        .replacen(',', ".", 1) // We already checked that there's only one.
        .parse::<f64>()
        .map_err(|_| ISO6093Error::ParseError)
}

/// This implementation does not call [parse_nr3] or [parse_nr2] directly, but
/// instead implements its own parsing logic that is mostly duplicate; the
/// rationale for this is
fn parse_iso6093_ex(mut input: &str) -> Result<ISO6093RealNumber, ISO6093Error> {
    input = input.trim_start_matches(|c| c == ' ');
    // .5 Allowed by Rust, but not by NR2.
    if input.chars().next().is_some_and(|c| c == '.') {
        return Err(ISO6093Error::InvalidFormat);
    }
    let mut chars = input.chars();
    // +.5 and -.5 Allowed by Rust, but not by NR2.
    if chars.next().is_some_and(|c| c == '+' || c == '-') {
        if chars.next().is_some_and(|c| c == '.') {
            return Err(ISO6093Error::InvalidFormat);
        }
    }

    let mut has_exponent = false;
    let mut needs_replace = false;
    let mut has_decimal = false;
    for c in input.chars() {
        if c == '.' || c == ',' {
            if has_decimal { // duplicate
                return Err(ISO6093Error::InvalidFormat);
            }
            if has_exponent { // decimal after exponent
                return Err(ISO6093Error::InvalidFormat);
            }
            has_decimal = true;
            needs_replace = c == ',';
        }
        if c == 'E' || c == 'e' {
            has_exponent = true;
        }
    }
    if !has_decimal {
        // This doesn't iterate over characters, so it has little overhead.
        return parse_nr1(input).map(|v| ISO6093RealNumber::NR1(v));
    }

    #[cfg(not(feature = "alloc"))]
    {

    }

    #[cfg(feature = "alloc")]
    {
        // Replace the ',' with '.', but only if we need to.
        let mut normstr = String::new();
        let normalized = if needs_replace {
            normstr = input.replacen(',', ".", 1);
            normstr.as_str()
        } else {
            input
        };

        normalized
            .parse::<f64>()
            .map(|v| if has_exponent {
                ISO6093RealNumber::NR3(v)
            } else {
                ISO6093RealNumber::NR2(v)
            })
            .map_err(|_| ISO6093Error::ParseError)
    }

}

#[cfg(feature = "alloc")]
#[inline(always)]
pub fn parse_nr2(input: &str) -> Result<f64, ISO6093Error> {
    parse_nr2_ex(input)
}

#[cfg(feature = "alloc")]
#[inline(always)]
pub fn parse_nr3(input: &str) -> Result<f64, ISO6093Error> {
    parse_nr3_ex(input)
}

#[cfg(feature = "alloc")]
#[inline(always)]
pub fn parse_iso6093(input: &str) -> Result<ISO6093RealNumber, ISO6093Error> {
    parse_iso6093_ex(input)
}

// #[cfg(not(feature = "alloc"))]
// pub fn parse_iso6093(input: &mut str) -> Result<ISO6093RealNumber, ISO6093Error> {
//     for (i, c) in input.char_indices() {
//         if c == ',' {
//             input[i] = '.';
//         }
//     }
//     parse_iso6093_ex(input)
// }

// TODO: print_nr1
// TODO: print_nr2
// TODO: print_nr3
// TODO: print_iso6093

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nr1() {
        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_nr1("0004902"), Ok(4902.0));
        assert_eq!(parse_nr1("  4902"), Ok(4902.0));
        assert_eq!(parse_nr1("   4902"), Ok(4902.0));
        assert_eq!(parse_nr1("0001234"), Ok(1234.0));
        assert_eq!(parse_nr1("   1234"), Ok(1234.0));
        assert_eq!(parse_nr1("0000000"), Ok(0.0));
        assert_eq!(parse_nr1("      0"), Ok(0.0));
        assert_eq!(parse_nr1("1234567"), Ok(1234567.0));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_nr1("+004902"), Ok(4902.0));
        assert_eq!(parse_nr1(" +4902"), Ok(4902.0));
        assert_eq!(parse_nr1("  +4902"), Ok(4902.0));
        assert_eq!(parse_nr1("   4902"), Ok(4902.0));
        assert_eq!(parse_nr1("+001234"), Ok(1234.0));
        assert_eq!(parse_nr1("  +1234"), Ok(1234.0));
        assert_eq!(parse_nr1("   1234"), Ok(1234.0));
        assert_eq!(parse_nr1("-56780"), Ok(-56780.0));
        assert_eq!(parse_nr1(" -56780"), Ok(-56780.0));
        assert_eq!(parse_nr1("+000000"), Ok(0.0));
        assert_eq!(parse_nr1("     +0"), Ok(0.0));
        assert_eq!(parse_nr1("      0"), Ok(0.0));

        // Other tests
        assert_eq!(parse_nr1("123"), Ok(123.0));
        assert_eq!(parse_nr1("-456"), Ok(-456.0));
        assert_eq!(parse_nr1("+789"), Ok(789.0));
        assert_eq!(parse_nr1(""), Err(ISO6093Error::ParseError));
        assert_eq!(parse_nr1("12.3"), Err(ISO6093Error::ParseError));
        assert_eq!(parse_nr1("12E3"), Err(ISO6093Error::ParseError));
    }

    #[test]
    fn test_parse_nr2() {
        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_nr2("1327.000"), Ok(1327.0));
        assert_eq!(parse_nr2("0001327."), Ok(1327.0));
        assert_eq!(parse_nr2("   1327."), Ok(1327.0));
        assert_eq!(parse_nr2("00123.45"), Ok(123.45));
        assert_eq!(parse_nr2("  123.45"), Ok(123.45));
        assert_eq!(parse_nr2("  1237.0"), Ok(1237.0));
        assert_eq!(parse_nr2("00.00001"), Ok(0.00001));
        assert_eq!(parse_nr2("1234,567"), Ok(1234.567));
        assert_eq!(parse_nr2("000,0000"), Ok(0.0));
        assert_eq!(parse_nr2("     0,0"), Ok(0.0));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_nr2("+1327.00"), Ok(1327.00));
        assert_eq!(parse_nr2("  +1327."), Ok(1327.0));
        assert_eq!(parse_nr2("   1327."), Ok(1327.0));
        assert_eq!(parse_nr2(" +123.45"), Ok(123.45));
        assert_eq!(parse_nr2("  123,45"), Ok(123.45));
        assert_eq!(parse_nr2(" +1237.0"), Ok(1237.0));
        assert_eq!(parse_nr2("  1237,0"), Ok(1237.0));
        assert_eq!(parse_nr2("+0.00001"), Ok(0.00001));
        assert_eq!(parse_nr2("-5,67800"), Ok(-5.67800));
        assert_eq!(parse_nr2("-05,6780"), Ok(-5.6780));
        assert_eq!(parse_nr2("+0.00000"), Ok(0.0));
        assert_eq!(parse_nr2("    +0,0"), Ok(0.0));
        assert_eq!(parse_nr2("     0,0"), Ok(0.0));
        assert_eq!(parse_nr2("      0,"), Ok(0.0));

        // Other tests
        assert_eq!(parse_nr2("123.45"), Ok(123.45));
        assert_eq!(parse_nr2("-67.89"), Ok(-67.89));
        assert_eq!(parse_nr2("+0.123"), Ok(0.123));
        assert_eq!(parse_nr2(""), Err(ISO6093Error::InvalidFormat));
        assert_eq!(parse_nr2("123"), Err(ISO6093Error::InvalidFormat));
        assert_eq!(parse_nr2("12E3"), Err(ISO6093Error::InvalidFormat));
    }

    #[test]
    fn test_parse_nr3() {
        // Examples directly from ISO 6093
        assert_eq!(parse_nr3("+0,56E+4"), Ok(5600.0));
        assert_eq!(parse_nr3("+5.6e+03"), Ok(5600.0));
        assert_eq!(parse_nr3("+0,3E-04"), Ok(0.00003));
        assert_eq!(parse_nr3(" 0,3e-04"), Ok(0.00003));
        assert_eq!(parse_nr3("-2,8E+00"), Ok(-2.8));
        assert_eq!(parse_nr3("+0,0E+00"), Ok(0.0));
        assert_eq!(parse_nr3("   0.e+0"), Ok(0.0));

        // Other tests
        assert_eq!(parse_nr3("1.23E+45"), Ok(1.23E+45));
        assert_eq!(parse_nr3("-6.78e-9"), Ok(-6.78e-9));
        assert_eq!(parse_nr3("+1.0E2"), Ok(100.0));
        assert_eq!(parse_nr3(""), Err(ISO6093Error::InvalidFormat));
        assert_eq!(parse_nr3("123.45"), Err(ISO6093Error::InvalidFormat));
        assert_eq!(parse_nr3("123"), Err(ISO6093Error::InvalidFormat));
    }

    #[test]
    fn test_parse_iso6093() {
        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_iso6093("0004902"), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093("  4902"),  Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093("   4902"), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093("0001234"), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093("   1234"), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093("0000000"), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093("      0"), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093("1234567"), Ok(ISO6093RealNumber::NR1(1234567.0)));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_iso6093("+004902"), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093(" +4902"),  Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093("  +4902"), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093("   4902"), Ok(ISO6093RealNumber::NR1(4902.0)));
        assert_eq!(parse_iso6093("+001234"), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093("  +1234"), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093("   1234"), Ok(ISO6093RealNumber::NR1(1234.0)));
        assert_eq!(parse_iso6093("-56780"),  Ok(ISO6093RealNumber::NR1(-56780.0)));
        assert_eq!(parse_iso6093(" -56780"), Ok(ISO6093RealNumber::NR1(-56780.0)));
        assert_eq!(parse_iso6093("+000000"), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093("     +0"), Ok(ISO6093RealNumber::NR1(0.0)));
        assert_eq!(parse_iso6093("      0"), Ok(ISO6093RealNumber::NR1(0.0)));

        // Unsigned examples directly from ISO 6093
        assert_eq!(parse_iso6093("1327.000"), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093("0001327."), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093("   1327."), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093("00123.45"), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093("  123.45"), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093("  1237.0"), Ok(ISO6093RealNumber::NR2(1237.0)));
        assert_eq!(parse_iso6093("00.00001"), Ok(ISO6093RealNumber::NR2(0.00001)));
        assert_eq!(parse_iso6093("1234,567"), Ok(ISO6093RealNumber::NR2(1234.567)));
        assert_eq!(parse_iso6093("000,0000"), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093("     0,0"), Ok(ISO6093RealNumber::NR2(0.0)));

        // Signed examples directly from ISO 6093
        assert_eq!(parse_iso6093("+1327.00"), Ok(ISO6093RealNumber::NR2(1327.00)));
        assert_eq!(parse_iso6093("  +1327."), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093("   1327."), Ok(ISO6093RealNumber::NR2(1327.0)));
        assert_eq!(parse_iso6093(" +123.45"), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093("  123,45"), Ok(ISO6093RealNumber::NR2(123.45)));
        assert_eq!(parse_iso6093(" +1237.0"), Ok(ISO6093RealNumber::NR2(1237.0)));
        assert_eq!(parse_iso6093("  1237,0"), Ok(ISO6093RealNumber::NR2(1237.0)));
        assert_eq!(parse_iso6093("+0.00001"), Ok(ISO6093RealNumber::NR2(0.00001)));
        assert_eq!(parse_iso6093("-5,67800"), Ok(ISO6093RealNumber::NR2(-5.67800)));
        assert_eq!(parse_iso6093("-05,6780"), Ok(ISO6093RealNumber::NR2(-5.6780)));
        assert_eq!(parse_iso6093("+0.00000"), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093("    +0,0"), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093("     0,0"), Ok(ISO6093RealNumber::NR2(0.0)));
        assert_eq!(parse_iso6093("      0,"), Ok(ISO6093RealNumber::NR2(0.0)));

        // Examples directly from ISO 6093
        assert_eq!(parse_iso6093("+0,56E+4"), Ok(ISO6093RealNumber::NR3(5600.0)));
        assert_eq!(parse_iso6093("+5.6e+03"), Ok(ISO6093RealNumber::NR3(5600.0)));
        assert_eq!(parse_iso6093("+0,3E-04"), Ok(ISO6093RealNumber::NR3(0.00003)));
        assert_eq!(parse_iso6093(" 0,3e-04"), Ok(ISO6093RealNumber::NR3(0.00003)));
        assert_eq!(parse_iso6093("-2,8E+00"), Ok(ISO6093RealNumber::NR3(-2.8)));
        assert_eq!(parse_iso6093("+0,0E+00"), Ok(ISO6093RealNumber::NR3(0.0)));
        assert_eq!(parse_iso6093("   0.e+0"), Ok(ISO6093RealNumber::NR3(0.0)));
    }
}
