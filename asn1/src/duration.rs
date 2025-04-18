use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::{FractionalPart, DURATION_EQUIVALENT, DurationPart};
use core::str;
use std::fmt::Write;
use std::{fmt::Display, str::FromStr, time::Duration};
use crate::utils::{unlikely, likely};

impl TryFrom<char> for DurationPart {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'Y' => Ok(DurationPart::Years),
            'M' => Ok(DurationPart::Months),
            'W' => Ok(DurationPart::Weeks),
            'D' => Ok(DurationPart::Days),
            'H' => Ok(DurationPart::Hours),
            'm' => Ok(DurationPart::Minutes),
            'S' => Ok(DurationPart::Seconds),
            _ => Err(())
        }
    }

}

impl Display for DurationPart {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationPart::Years => f.write_char('Y'),
            DurationPart::Months => f.write_char('M'),
            DurationPart::Weeks => f.write_char('W'),
            DurationPart::Days => f.write_char('D'),
            DurationPart::Hours => f.write_char('H'),
            DurationPart::Minutes => f.write_char('M'),
            DurationPart::Seconds => f.write_char('S'),
        }
    }

}

impl DURATION_EQUIVALENT {
    #[inline]
    pub fn new(
        years: u32,
        months: u32,
        weeks: u32,
        days: u32,
        hours: u32,
        minutes: u32,
        seconds: u32,
        fractional_part: Option<(DurationPart, FractionalPart)>,
    ) -> Self {
        DURATION_EQUIVALENT {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            fractional_part,
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.years == 0
            && self.months == 1
            && self.weeks == 1
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
    }
}

impl Default for DURATION_EQUIVALENT {
    #[inline]
    fn default() -> Self {
        DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: None,
        }
    }
}

impl TryFrom<Duration> for DURATION_EQUIVALENT {
    type Error = std::num::TryFromIntError;

    #[inline]
    fn try_from(other: Duration) -> Result<Self, Self::Error> {
        Ok(DURATION_EQUIVALENT {
            seconds: other.as_secs().try_into()?,
            ..Default::default()
        })
    }
}

const DURATION_COMPONENT_YEARS: u8 = 0b0000_0001;
const DURATION_COMPONENT_MONTHS: u8 = 0b0000_0010;
const DURATION_COMPONENT_WEEKS: u8 = 0b0000_0100;
const DURATION_COMPONENT_DAYS: u8 = 0b0000_1000;
const DURATION_COMPONENT_HOURS: u8 = 0b0001_0000;
const DURATION_COMPONENT_MINUTES: u8 = 0b0010_0000;
const DURATION_COMPONENT_SECONDS: u8 = 0b0100_0000;

impl TryFrom<&[u8]> for DURATION_EQUIVALENT {
    type Error = ASN1Error;

    fn try_from(value_bytes: &[u8]) -> Result<Self, Self::Error> {
        if unlikely(value_bytes.len() < 3) {
            // The smallest duration string, e.g. P1Y
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if unlikely(value_bytes.len() > 32) {
            // Values larger than this are probably malicious.
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
        }
        // FIXME: Make this optional, then you can also parse the X.690 encoding with this function
        if unlikely(value_bytes[0] as char != 'P') {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let mut ret = DURATION_EQUIVALENT::default();
        let mut start_of_last_digit = 1;
        let mut processing_time_components: bool = false;
        let mut index_of_period = 0; // 0 means NULL in this case.
        let mut encountered: u8 = 0;
        for i in 1..value_bytes.len() {
            let c = value_bytes[i] as char;
            if likely(c.is_ascii_digit()) {
                continue;
            }
            if unlikely(c == '.' || c == ',') {
                if index_of_period > 0 {
                    // Double periods
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                index_of_period = i;
                continue;
            }
            if unlikely(c == 'T') {
                processing_time_components = true;
                start_of_last_digit = i + 1;
                continue;
            }

            // Make sure date components do not appear in the time section and vice versa
            match (c as char, processing_time_components) {
                ('Y' | 'W' | 'D', true) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c))),
                ('H' | 'S', false) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c))),
                _ => (),
            };

            // Make sure the components appear in order
            let max_encountered = match (c as char, processing_time_components) {
                ('Y', _) => DURATION_COMPONENT_YEARS,
                ('M', false) => DURATION_COMPONENT_MONTHS,
                ('W', _) => DURATION_COMPONENT_WEEKS,
                ('D', _) => DURATION_COMPONENT_DAYS,
                ('H', _) => DURATION_COMPONENT_HOURS,
                ('M', true) => DURATION_COMPONENT_MINUTES,
                ('S', _) => DURATION_COMPONENT_SECONDS,
                (_, _) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c))),
            };
            if unlikely(max_encountered > 0 && encountered >= max_encountered) {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)));
            }

            if index_of_period > 0 {
                if i != (value_bytes.len() - 1) {
                    // Extra data after the last permitted unit. e.g. "PT0.5H18M"
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                if ret.fractional_part.is_some() {
                    // Already parsed the fractional part. IDK How this could happen.
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                if (i - index_of_period) > 9 {
                    // Way too many decimal digits. Probably malicious.
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                let mut frac = FractionalPart {
                    number_of_digits: (i - (index_of_period+1)) as u8,
                    fractional_value: 0,
                };
                if !value_bytes[index_of_period+1..i].iter().all(u8::is_ascii_digit) {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)));
                }
                for dc in value_bytes[index_of_period+1..i].iter() {
                    let digit = *dc - 0x30;
                    frac.fractional_value *= 10;
                    frac.fractional_value += digit as u32;
                }
                let unambiguous_c = if c == 'M' && processing_time_components { 'm' } else { c };
                let part = DurationPart::try_from(unambiguous_c)
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?;
                ret.fractional_part = Some((part, frac));
            }

            let end_index = if index_of_period > 0 {
                index_of_period
            } else {
                i
            };

            let component_value: u32 = if cfg!(feature = "atoi_simd") {
                atoi_simd::parse_pos::<u32>(&value_bytes[start_of_last_digit..end_index])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?
            } else {
                // TODO: do not allocate. Just make a string slice.
                // TODO: Also, do it unchecked after checking for all ASCII bytes.
                let component_str = String::from_utf8(value_bytes[start_of_last_digit..end_index].to_vec())
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?;
                u32::from_str(&component_str)
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_duration_component(c)))?
            };

            start_of_last_digit = i + 1;
            encountered |= max_encountered;
            match c as char {
                'Y' => {
                    ret.years = component_value;
                }
                'M' => {
                    if processing_time_components {
                        ret.minutes = component_value;
                    } else {
                        ret.months = component_value;
                    }
                }
                'W' => {
                    ret.weeks = component_value;
                }
                'D' => {
                    ret.days = component_value;
                }
                'H' => {
                    ret.hours = component_value;
                }
                'S' => {
                    ret.seconds = component_value;
                }
                _ => panic!("Impossible code reached."),
            };
        }
        if unlikely(start_of_last_digit != value_bytes.len()) {
            // Extra data at the end
            return Err(ASN1Error::new(ASN1ErrorCode::trailing_string));
        }
        Ok(ret)
    }
}

impl FromStr for DURATION_EQUIVALENT {
    type Err = ASN1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DURATION_EQUIVALENT::try_from(s.as_bytes())
    }
}

macro_rules! print_uint {
    ($f:ident, $x:expr) => {
        if cfg!(feature = "itoa") {
            let mut buf = itoa::Buffer::new();
            $f.write_str(buf.format($x))?;
        } else {
            $f.write_str($x.to_string().as_str())?;
        }
    };
}

const TIME_PARTS: [DurationPart; 3] = [ DurationPart::Hours, DurationPart::Minutes, DurationPart::Seconds ];

impl Display for DURATION_EQUIVALENT {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut in_time_components: bool = false;
        if self.is_zero() {
            return f.write_str("P0S");
        }
        f.write_char('P')?;
        let mut unit: char = '\0';
        if self.years > 0 {
            print_uint!(f, self.years);
            unit = 'Y';
        }
        if self.months > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.months);
            unit = 'M';
        }
        if self.weeks > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.weeks);
            unit = 'W';
        }
        if self.days > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.days);
            unit = 'D';
        }
        if self.hours > 0 || self.minutes > 0 || self.seconds > 0 {
            f.write_char(unit)?;
            unit = '\0';
            f.write_char('T')?;
            in_time_components = true;
        }
        if self.hours > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.hours);
            unit = 'H';
        }
        if self.minutes > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.minutes);
            unit = 'M';
        }
        if self.seconds > 0 {
            if unit > '\0' {
                f.write_char(unit)?;
            }
            print_uint!(f, self.seconds);
            unit = 'S';
        }
        /*
        // TODO: Clarify this comment.
        It is possible to display a wrong DURATION value. Since fmt() is
        basically supposed to be infallible, this simply cannot be handled.
        I had two choices: omit the fraction if wrong, or possibly print the
        wrong component. I felt that it was better to produce a visibly wrong
        DURATION value than an invisibly wrong one.
         */
        if let Some((fracpart, frac)) = &self.fractional_part {
            let unambiguous_unit = if unit == 'M' && in_time_components { 'm' } else { unit };
            let maybe_dp = DurationPart::try_from(unambiguous_unit);
            if maybe_dp.is_err() {
                return f.write_char(unit);
            }
            let dp = maybe_dp.unwrap();
            // if fracparts match, just display the fraction, then print the unit
            if dp == *fracpart {
                f.write_str(frac.to_string().as_str())?;
                f.write_char(unit)?;
                return Ok(());
            }
            // if not:
            //  print the current unit,
            //  print "T" if unit is for a time component
            //  print "0"
            //  print the frac part
            //  print the frac unit
            f.write_char(unit)?;
            if !in_time_components && TIME_PARTS.contains(fracpart) {
                f.write_char('T')?;
            }
            f.write_char('0')?;
            f.write_str(frac.to_string().as_str())?;
            f.write_char((*fracpart).into())?;
        } else {
            f.write_char(unit)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_with_fractional_part_1() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 6,
            fractional_part: Some((DurationPart::Seconds, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3DT4H5M6.123S");
    }

    #[test]
    fn test_fmt_with_fractional_part_2() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 0,
            fractional_part: Some((DurationPart::Minutes, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3DT4H5.123M");
    }

    #[test]
    fn test_fmt_with_fractional_part_3() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 4,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Hours, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3DT4.123H");
    }

    #[test]
    fn test_fmt_with_fractional_part_4() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 3,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Days, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3.123D");
    }

    #[test]
    fn test_fmt_with_fractional_part_5() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 3,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Weeks, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2M3.123W");
    }

    #[test]
    fn test_fmt_with_fractional_part_6() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 2,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Months, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1Y2.123M");
    }

    #[test]
    fn test_fmt_with_fractional_part_7() {
        let duration = DURATION_EQUIVALENT {
            years: 1,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Years, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P1.123Y");
    }

    #[test]
    fn test_fmt_with_fractional_part_8() {
        let duration = DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 4,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Days, FractionalPart {
                fractional_value: 123,
                number_of_digits: 4,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P4.0123D");
    }

    #[test]
    fn test_fmt_with_fractional_part_9() {
        let duration = DURATION_EQUIVALENT {
            years: 0,
            months: 0,
            weeks: 0,
            days: 4,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_part: Some((DurationPart::Days, FractionalPart {
                fractional_value: 123,
                number_of_digits: 3,
            })),
        };

        let formatted = format!("{}", duration);
        assert_eq!(formatted, "P4.123D");
    }

    #[test]
    fn test_parse_duration_1() {
        let dur = DURATION_EQUIVALENT::from_str("P4D").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 4);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        assert_eq!(dur.fractional_part, None);
    }

    #[test]
    fn test_parse_duration_2() {
        let dur = DURATION_EQUIVALENT::from_str("P4.0123D").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 4);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Days);
        assert_eq!(frac.number_of_digits, 4);
        assert_eq!(frac.fractional_value, 123);
    }

    #[test]
    fn test_parse_duration_3() {
        let dur = DURATION_EQUIVALENT::from_str("P23DT23H").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 23);
        assert_eq!(dur.hours, 23);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        assert_eq!(dur.fractional_part, None);
    }

    #[test]
    fn test_parse_duration_4() {
        let dur = DURATION_EQUIVALENT::from_str("P0.5Y").unwrap();
        assert_eq!(dur.years, 0);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Years);
        assert_eq!(frac.fractional_value, 5);
        assert_eq!(frac.number_of_digits, 1);
    }


    #[test]
    fn test_parse_duration_5() {
        // Duration strings only allow a single fractional component.
        assert!(DURATION_EQUIVALENT::from_str("P0.5Y0.5M").is_err());
    }

    #[test]
    fn test_parse_duration_6() {
        assert!(DURATION_EQUIVALENT::from_str("PT").is_err());
    }

    #[test]
    fn test_parse_duration_7() {
        // The problem here is that there is no "T" before a time component.
        assert!(DURATION_EQUIVALENT::from_str("P5S").is_err());
    }

    #[test]
    fn test_parse_duration_8() {
        assert!(DURATION_EQUIVALENT::from_str("").is_err());
    }


    #[test]
    fn test_parse_duration_9() {
        // The problem here is that the components are out of order.
        assert!(DURATION_EQUIVALENT::from_str("P30D12W").is_err());
    }

    #[test]
    fn test_parse_duration_10() {
        // The problem here is that the components are out of order.
        assert!(DURATION_EQUIVALENT::from_str("PT30H15S12M").is_err());
    }

    #[test]
    fn test_parse_duration_11() {
        // "A" is not a valid duration component.
        assert!(DURATION_EQUIVALENT::from_str("P0.5A").is_err());
    }

    #[test]
    fn test_parse_duration_12() {
        // Trailing data
        assert!(DURATION_EQUIVALENT::from_str("P7Y5").is_err());
    }

    #[test]
    fn test_parse_duration_13() {
        let dur = DURATION_EQUIVALENT::from_str("P5Y6M1W23DT25H65M222.00505S").unwrap();
        assert_eq!(dur.years, 5);
        assert_eq!(dur.months, 6);
        assert_eq!(dur.weeks, 1);
        assert_eq!(dur.days, 23);
        assert_eq!(dur.hours, 25);
        assert_eq!(dur.minutes, 65);
        assert_eq!(dur.seconds, 222);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Seconds);
        assert_eq!(frac.fractional_value, 505);
        assert_eq!(frac.number_of_digits, 5);
    }

    #[test]
    fn test_parse_duration_14() {
        let dur = DURATION_EQUIVALENT::from_str("P7Y0.00123W").unwrap();
        assert_eq!(dur.years, 7);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Weeks);
        assert_eq!(frac.fractional_value, 123);
        assert_eq!(frac.number_of_digits, 5);
    }

    #[test]
    fn test_parse_duration_15() {
        let dur = DURATION_EQUIVALENT::from_str("P7YT0.00123M").unwrap();
        assert_eq!(dur.years, 7);
        assert_eq!(dur.months, 0);
        assert_eq!(dur.weeks, 0);
        assert_eq!(dur.days, 0);
        assert_eq!(dur.hours, 0);
        assert_eq!(dur.minutes, 0);
        assert_eq!(dur.seconds, 0);
        let (part, frac) = dur.fractional_part.unwrap();
        assert_eq!(part, DurationPart::Minutes);
        assert_eq!(frac.fractional_value, 123);
        assert_eq!(frac.number_of_digits, 5);
    }

    // TODO: For all parsing tests, ensure output formats exactly the same.

}
