use crate::error::{ASN1Error, ASN1ErrorCode};
use crate::types::DURATION_EQUIVALENT;
use std::{fmt::Display, str::FromStr, time::Duration};

impl DURATION_EQUIVALENT {
    pub fn new() -> Self {
        DURATION_EQUIVALENT::default()
    }
}

impl Default for DURATION_EQUIVALENT {
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
        if value_bytes.len() < 3 {
            // The smallest duration string, e.g. P1Y
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if value_bytes[0] as char != 'P' {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        let mut ret = DURATION_EQUIVALENT::new();
        let mut start_of_last_digit = 0;
        let mut processing_time_components: bool = false;
        let mut index_of_period = 0; // 0 means NULL in this case.
        let mut encountered: u8 = 0;
        for i in 1..value_bytes.len() {
            let char_ = value_bytes[i];
            if !char_.is_ascii_digit() {
                if start_of_last_digit == i {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                match char_ as char {
                    '.' => {
                        index_of_period = i;
                    }
                    'Y' | 'W' | 'M' | 'D' | 'H' | 'S' => {
                        if index_of_period > 0 {
                            if i != (value_bytes.len() - 1) {
                                // Extra data after the last permitted unit.
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                        }
                        let end_index = if index_of_period > 0 {
                            index_of_period
                        } else {
                            i
                        };
                        let component_str = match String::from_utf8(
                            value_bytes[start_of_last_digit..end_index].to_vec(),
                        ) {
                            Ok(s) => s,
                            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
                        };
                        let component_value = match u32::from_str(&component_str) {
                            Ok(v) => v,
                            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
                        };
                        start_of_last_digit = i + 1;
                        match char_ as char {
                            'Y' => {
                                if processing_time_components {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                if encountered > 0 {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_YEARS;
                                ret.years = component_value;
                            }
                            'M' => {
                                if processing_time_components {
                                    if encountered > DURATION_COMPONENT_HOURS {
                                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                    }
                                    encountered |= DURATION_COMPONENT_MINUTES;
                                    ret.minutes = component_value;
                                } else {
                                    if encountered > DURATION_COMPONENT_YEARS {
                                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                    }
                                    encountered |= DURATION_COMPONENT_MONTHS;
                                    ret.months = component_value;
                                }
                            }
                            'W' => {
                                if processing_time_components {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                if encountered > DURATION_COMPONENT_MONTHS {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_WEEKS;
                                ret.weeks = component_value;
                            }
                            'D' => {
                                if processing_time_components {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                if encountered > DURATION_COMPONENT_WEEKS {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_DAYS;
                                ret.days = component_value;
                            }
                            'H' => {
                                if !processing_time_components {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                if encountered > DURATION_COMPONENT_DAYS {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_HOURS;
                                ret.hours = component_value;
                            }
                            'S' => {
                                if !processing_time_components {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                if encountered > DURATION_COMPONENT_MINUTES {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_SECONDS;
                                ret.seconds = component_value;
                            }
                            _ => panic!("Impossible code reached."),
                        };
                    }
                    'T' => {
                        processing_time_components = true;
                    }
                    _ => (),
                }
            }
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

impl Display for DURATION_EQUIVALENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = vec![String::from("P")];
        if self.years > 0 {
            parts.push(format!("{}Y", self.years));
        }
        if self.months > 0 {
            parts.push(format!("{}M", self.months));
        }
        if self.weeks > 0 {
            parts.push(format!("{}W", self.weeks));
        }
        if self.days > 0 {
            parts.push(format!("{}D", self.days));
        }
        if self.hours > 0 {
            parts.push(format!("{}H", self.hours));
        }
        if self.minutes > 0 {
            parts.push(format!("{}M", self.minutes));
        }
        if self.seconds > 0 {
            parts.push(format!("{}S", self.seconds));
        }
        // TODO: This definitely needs some testing.
        if let Some(frac) = &self.fractional_part {
            let last_part = parts.last_mut();
            match last_part {
                Some(part) => {
                    let last_char = part.pop();
                    match last_char {
                        Some(c) => {
                            parts.push(format!(
                                ".{:>width$}{}",
                                frac.fractional_value,
                                c,
                                width = frac.number_of_digits as usize
                            ));
                        }
                        None => {
                            let str_form = parts.join("");
                            return f.write_str(str_form.as_str());
                        }
                    }
                }
                None => {
                    parts.push(format!(
                        "0.{:>width$}S",
                        frac.fractional_value,
                        width = frac.number_of_digits as usize
                    ));
                }
            };
        }
        let str_form = parts.join("");
        f.write_str(str_form.as_str())
    }
}
