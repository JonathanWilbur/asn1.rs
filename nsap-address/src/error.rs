use core::fmt::Display;
use core::error::Error;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// An error parsing an NSAP address from bytes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NAddressParseError {
    TooShort,
    TooLong,
}

impl Display for NAddressParseError {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }

}

impl Error for NAddressParseError {}

/// Error representing an issue parsing an IETF RFC 1278 NSAP address string
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub enum RFC1278ParseError {
    /// A malformed IETF RFC 1278 string
    Malformed,
    /// An unrecognized--but possibly valid--syntax
    UnrecognizedSyntax,
    /// Parsing cannot proceed, because the AFI is not recognized, so the number
    /// of IDI digits and the syntax of the DSP cannot be determined.
    UnrecognizedAFI,
    /// A DNS name needs to be resolved to an IP address. Replace the DNS name
    /// in the string with the resolved IP address to obtain the correct
    /// string encoding.
    ResolveDNS(String),
    /// Shortcomings in the specification make it ambiguous as to how to parse
    /// or interpret the string
    SpecificationFailure,
    /// Used a prohibited character in the NSAP address string. One such
    /// character is the underscore `_`, which is used by RFC 1278 for
    /// delimiting NSAP addresses in a presentation address string.
    ProhibitedCharacter(char),
}

#[cfg(feature = "alloc")]
impl Display for RFC1278ParseError {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RFC1278ParseError::Malformed => f.write_str("malformed"),
            RFC1278ParseError::UnrecognizedSyntax => f.write_str("unrecognized syntax"),
            RFC1278ParseError::UnrecognizedAFI => f.write_str("unrecognized afi"),
            RFC1278ParseError::ResolveDNS(dns_name) => write!(f, "resolve dns name {}", dns_name),
            RFC1278ParseError::SpecificationFailure => f.write_str("shortcoming in specifications"),
            RFC1278ParseError::ProhibitedCharacter(c) => write!(f, "prohibited character {}", c),
        }
    }

}

#[cfg(feature = "alloc")]
impl Error for RFC1278ParseError {}
