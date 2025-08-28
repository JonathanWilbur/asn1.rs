use core::error::Error;
use core::fmt::Display;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// An error parsing an NSAP address from bytes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NAddressParseError {
    TooShort,
    TooLong,
    MalformedDSP,
    NonDigitsInIDI,
}

impl Display for NAddressParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for NAddressParseError {}

/// Error representing an issue parsing an IETF RFC 1278 NSAP address string
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
    #[cfg(feature = "alloc")]
    ResolveDNSWithName(String),
    /// A DNS name needs to be resolved to an IP address, but it could not be
    /// returned to the user because this crate is configured to be heapless
    /// (no alloc).
    ResolveDNS,
    /// Shortcomings in the specification make it ambiguous as to how to parse
    /// or interpret the string
    SpecificationFailure,
    /// Used a prohibited character in the NSAP address string. One such
    /// character is the underscore `_`, which is used by RFC 1278 for
    /// delimiting NSAP addresses in a presentation address string.
    ProhibitedCharacter(char),
    /// The string is too large to parse into an NSAP address. This only happens
    /// when heap allocation (`alloc`) is not enabled.
    TooLarge,
}

impl Display for RFC1278ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RFC1278ParseError::Malformed => f.write_str("malformed"),
            RFC1278ParseError::UnrecognizedSyntax => f.write_str("unrecognized syntax"),
            RFC1278ParseError::UnrecognizedAFI => f.write_str("unrecognized afi"),
            #[cfg(feature = "alloc")]
            RFC1278ParseError::ResolveDNSWithName(dns_name) => {
                write!(f, "resolve dns name {}", dns_name)
            }
            RFC1278ParseError::SpecificationFailure => f.write_str("shortcoming in specifications"),
            RFC1278ParseError::ProhibitedCharacter(c) => write!(f, "prohibited character {}", c),
            RFC1278ParseError::TooLarge => write!(f, "too large"),
            RFC1278ParseError::ResolveDNS => write!(f, "resolve dns name"),
        }
    }
}

impl Error for RFC1278ParseError {}
