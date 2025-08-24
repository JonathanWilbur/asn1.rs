//! This module decodes and encodes a Network Service Access Point (NSAP) to and
//! from the "preferred binary encoding" described in Annex A, Section A.5.3 of
//! [ITU-T Recommendation X.213 (2001)](https://www.itu.int/rec/T-REC-X.213-200110-I/en).
//!
//! In addition to this, it displays and decodes NSAPs to and from
//! human-readable strings according to the procedures defined in
//! [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/), drawing on
//! additional information found in
//! [IETF RFC 1277](https://datatracker.ietf.org/doc/html/rfc1277).
//!
//! There are some deviations to the above, however. Since the human-friendly
//! string syntax was defined, new AFIs were added, including one for directly
//! representing IP addresses and another for representing URLs. As such this
//! library extends the specification, but should be completely backwards
//! compatible with it.
//!
//! You should **not** expect an NSAP decoded from a string to encode back into
//! the same exact string. You should **not** expect an NSAP decoded from bytes
//! to encode back into the same exact bytes. You should **not** expect all
//! NSAP syntaxes to be recognized everywhere; your application and dependencies
//! should handle unrecognized NSAP syntaxes gracefully.
#![no_std]
#![allow(non_camel_case_types)]
mod bcd;
mod data;
mod isoiec646;
use core::fmt::{Display, Write};
use core::error::Error;
use core::convert::TryFrom;
use core::iter::{Iterator, FusedIterator};
use crate::data::{get_address_type_info, X213NetworkAddressInfo};
use bcd::BCDBuffer;
use isoiec646::{
    char_to_local_iso_iec_646_byte,
    local_iso_iec_646_byte_to_char,
};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use core::str::FromStr;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4};
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;

pub type AFI = u8;

/// The AFI is mandatory. The IDI may be zero bytes (in the case of Local IDI),
/// and the DSP (presumably) MUST be present.
pub const SMALLEST_VALID_NSAP_ADDR: usize = 2;

pub const AFI_URL: u8 = 0xFF; // Specified in ITU-T Rec. X.519 (2019).
pub const AFI_X121_DEC_LEADING_NON_ZERO: u8 = 0x36;
pub const AFI_X121_DEC_LEADING_ZERO: u8 = 0x52;
pub const AFI_X121_BIN_LEADING_NON_ZERO: u8 = 0x37;
pub const AFI_X121_BIN_LEADING_ZERO: u8 = 0x53;
pub const AFI_ISO_DCC_DEC: u8 = 0x38;
pub const AFI_ISO_DCC_BIN: u8 = 0x39;
pub const AFI_F69_DEC_LEADING_NON_ZERO: u8 = 0x40;
pub const AFI_F69_DEC_LEADING_ZERO: u8 = 0x54;
pub const AFI_F69_BIN_LEADING_NON_ZERO: u8 = 0x41;
pub const AFI_F69_BIN_LEADING_ZERO: u8 = 0x55;
pub const AFI_E163_DEC_LEADING_NON_ZERO: u8 = 0x42;
pub const AFI_E163_DEC_LEADING_ZERO: u8 = 0x56;
pub const AFI_E163_BIN_LEADING_NON_ZERO: u8 = 0x43;
pub const AFI_E163_BIN_LEADING_ZERO: u8 = 0x57;
pub const AFI_E164_DEC_LEADING_NON_ZERO: u8 = 0x44;
pub const AFI_E164_DEC_LEADING_ZERO: u8 = 0x58;
pub const AFI_E164_BIN_LEADING_NON_ZERO: u8 = 0x45;
pub const AFI_E164_BIN_LEADING_ZERO: u8 = 0x59;
pub const AFI_ISO_6523_ICD_DEC: u8 = 0x46;
pub const AFI_ISO_6523_ICD_BIN: u8 = 0x47;

/// Quoting IETF RFC 4548:
///
/// > One of these two AFIs ('34') is
/// > allocated for assignment of NSAPA in Decimal Numeric Format.  This
/// > document does not address allocation for this AFI as it is not clear
/// > what use (if any) can be made of this encoding format at this time.
pub const AFI_IANA_ICP_DEC: u8 = 0x34;
pub const AFI_IANA_ICP_BIN: u8 = 0x35;
pub const AFI_ITU_T_IND_DEC: u8 = 0x76;
pub const AFI_ITU_T_IND_BIN: u8 = 0x77;
pub const AFI_LOCAL_DEC: u8 = 0x48;
pub const AFI_LOCAL_BIN: u8 = 0x49;
pub const AFI_LOCAL_ISO_IEC_646: u8 = 0x50;
pub const AFI_LOCAL_NATIONAL: u8 = 0x51;
pub const GROUP_AFI_X121_DEC_LEADING_NON_ZERO: u8 = 0xBA;
pub const GROUP_AFI_X121_DEC_LEADING_ZERO: u8 = 0xCA;
pub const GROUP_AFI_X121_BIN_LEADING_NON_ZERO: u8 = 0xBB;
pub const GROUP_AFI_X121_BIN_LEADING_ZERO: u8 = 0xCB;
pub const GROUP_AFI_ISO_DCC_DEC: u8 = 0xBC;
pub const GROUP_AFI_ISO_DCC_BIN: u8 = 0xBD;
pub const GROUP_AFI_F69_DEC_LEADING_NON_ZERO: u8 = 0xBE;
pub const GROUP_AFI_F69_DEC_LEADING_ZERO: u8 = 0xCC;
pub const GROUP_AFI_F69_BIN_LEADING_NON_ZERO: u8 = 0xBF;
pub const GROUP_AFI_F69_BIN_LEADING_ZERO: u8 = 0xCD;
pub const GROUP_AFI_E163_DEC_LEADING_NON_ZERO: u8 = 0xC0;
pub const GROUP_AFI_E163_DEC_LEADING_ZERO: u8 = 0xCE;
pub const GROUP_AFI_E163_BIN_LEADING_NON_ZERO: u8 = 0xC1;
pub const GROUP_AFI_E163_BIN_LEADING_ZERO: u8 = 0xCF;
pub const GROUP_AFI_E164_DEC_LEADING_NON_ZERO: u8 = 0xC2;
pub const GROUP_AFI_E164_DEC_LEADING_ZERO: u8 = 0xD0;
pub const GROUP_AFI_E164_BIN_LEADING_NON_ZERO: u8 = 0xC3;
pub const GROUP_AFI_E164_BIN_LEADING_ZERO: u8 = 0xD1;
pub const GROUP_AFI_ISO_6523_ICD_DEC: u8 = 0xC4;
pub const GROUP_AFI_ISO_6523_ICD_BIN: u8 = 0xC5;
pub const GROUP_AFI_IANA_ICP_DEC: u8 = 0xB8;
pub const GROUP_AFI_IANA_ICP_BIN: u8 = 0xB9;
pub const GROUP_AFI_ITU_T_IND_DEC: u8 = 0xE2;
pub const GROUP_AFI_ITU_T_IND_BIN: u8 = 0xE3;
pub const GROUP_AFI_LOCAL_DEC: u8 = 0xC6;
pub const GROUP_AFI_LOCAL_BIN: u8 = 0xC7;
pub const GROUP_AFI_LOCAL_ISO_IEC_646: u8 = 0xC8;
pub const GROUP_AFI_LOCAL_NATIONAL: u8 = 0xC9;
pub const MAX_DEC_DSP_LEN_X121: u8 = 24;
pub const MAX_DEC_DSP_LEN_ISO_DCC: u8 = 35;
pub const MAX_DEC_DSP_LEN_F69: u8 = 30;
pub const MAX_DEC_DSP_LEN_E163: u8 = 26;
pub const MAX_DEC_DSP_LEN_E164: u8 = 23;
pub const MAX_DEC_DSP_LEN_ISO_6523_ICD: u8 = 34;
pub const MAX_DEC_DSP_LEN_IANA_ICP: u8 = 34;
pub const MAX_DEC_DSP_LEN_ITU_T_IND: u8 = 32;
pub const MAX_DEC_DSP_LEN_LOCAL: u8 = 38;
pub const MAX_BIN_DSP_LEN_X121: u8 = 12;
pub const MAX_BIN_DSP_LEN_ISO_DCC: u8 = 17;
pub const MAX_BIN_DSP_LEN_F69: u8 = 15;
pub const MAX_BIN_DSP_LEN_E163: u8 = 13;
pub const MAX_BIN_DSP_LEN_E164: u8 = 11;
pub const MAX_BIN_DSP_LEN_ISO_6523_ICD: u8 = 17;
pub const MAX_BIN_DSP_LEN_IANA_ICP: u8 = 17;
pub const MAX_BIN_DSP_LEN_ITU_T_IND: u8 = 16;
pub const MAX_BIN_DSP_LEN_LOCAL: u8 = 19;
pub const MAX_ISO_IEC_646_LEN_LOCAL: u8 = 19;
pub const MAX_NATIONAL_CHAR_LEN_LOCAL: u8 = 9;

// TODO: Rename to make this clear? These are denominated in digits.
pub const MAX_IDI_LEN_X121: usize = 14; // Up to
pub const MAX_IDI_LEN_ISO_DCC: usize = 3; // Exactly
pub const MAX_IDI_LEN_F69: usize = 8; // Up to
pub const MAX_IDI_LEN_E163: usize = 12; // Up to
pub const MAX_IDI_LEN_E164: usize = 15; // Up to
pub const MAX_IDI_LEN_ISO_6523_ICD: usize = 4; // Exactly
pub const MAX_IDI_LEN_IANA_ICP: usize = 4; // Exactly
pub const MAX_IDI_LEN_ITU_T_IND: usize = 6; // Exactly
pub const MAX_IDI_LEN_LOCAL: usize = 0; // Exactly
pub const MAX_IDI_LEN_URL: usize = 4; // Exactly.

// DSP Prefixes that start with 0x54, 0x00, 0x72, 0x87, 0x22,
pub const RFC_1277_WELL_KNOWN_NETWORK_INTL_X25: u8 = 0x01;
pub const RFC_1277_WELL_KNOWN_NETWORK_JANET: u8 = 0x02;
pub const RFC_1277_WELL_KNOWN_NETWORK_DARPA_NSF_INTERNET: u8 = 0x03;
pub const RFC_1277_WELL_KNOWN_NETWORK_IXI: u8 = 0x06;
pub const ITU_X519_DSP_PREFIX_LDAP: u8 = 0x11;
pub const ITU_X519_DSP_PREFIX_IDM_OVER_IPV4: u8 = 0x10;
pub const ITU_X519_DSP_PREFIX_ITOT_OVER_IPV4: u8 = RFC_1277_WELL_KNOWN_NETWORK_DARPA_NSF_INTERNET;

pub const ITOT_OVER_IPV4_DEFAULT_PORT: u16 = 102;

pub const AFI_STR_X121: &str = "X121";
pub const AFI_STR_DCC: &str = "DCC";
pub const AFI_STR_TELEX: &str = "TELEX";
pub const AFI_STR_PSTN: &str = "PSTN"; // I think this is E.163
pub const AFI_STR_ISDN: &str = "ISDN"; // I think this is E.164 FIXME: Unused
pub const AFI_STR_ICD: &str = "ICD";
pub const AFI_STR_ICP: &str = "ICP"; // TODO: Document that this is not standard
pub const AFI_STR_IND: &str = "IND"; // TODO: Document that this is not standard
pub const AFI_STR_LOCAL: &str = "LOCAL";
pub const AFI_STR_URL: &str = "URL"; // TODO: Document that this is not standard

pub const IETF_RFC_1277_TELEX_NUMBER_STR: &str = "00728722";
pub const IETF_RFC_1006_PREFIX_STR: &str = "RFC-1006";
pub const X25_PREFIX_STR: &str = "X.25(80)";
pub const ECMA_117_BINARY_STR: &str = "ECMA-117-Binary";
pub const ECMA_117_DECIMAL_STR: &str = "ECMA-117-Decimal";

/// This is exported for convenience, since the Internet is most likely to be
/// used in NSAPs now. If an application only wants / can use Internet NSAPs,
/// the NSAPs could be checked to see if they begin with this sequence.
pub const INTERNET_PREFIX: [u8; 6] = [
    AFI_F69_DEC_LEADING_ZERO, // AFI
    0x00, 0x72, 0x87, 0x22, // IDI
    0x03, // The DSP prefix "03"
];

pub const INTERNET_PREFIX_IDI_DIGITS: [u8; 8] = *b"00728722";

const DEFAULT_ITOT_PORT: u16 = 102;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DSPSyntax {
    Decimal,
    Binary,
    IsoIec646Chars,
    NationalChars,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum X213NetworkAddressType {
    X121,
    ISO_DCC,
    F69,
    E163,
    E164,
    ISO_6523_ICD,
    IANA_ICP,
    ITU_T_IND,
    LOCAL,
    URL, // Defined (without a name) in ITU-T Rec. X.519.
}

impl Display for X213NetworkAddressType {

    /// Prints a human-readable string, per the procedures defined in
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = naddr_network_type_to_str(*self).ok_or(core::fmt::Error)?;
        f.write_str(s)
    }

}

impl TryFrom<AFI> for X213NetworkAddressType {
    type Error = ();

    #[inline]
    fn try_from(value: AFI) -> Result<Self, Self::Error> {
        afi_to_network_type(value).ok_or(())
    }

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NAddressError {
    NoAFI,
    NoIDI,
    NoDSPPrefix,
    TruncatedDSP,
    MalformedDSP,
    UnrecognizedNetworkType,
    IDPTruncated(usize, usize), // expected, actual
    InvalidRightPadding,
    NonDigitInIDI(u8),
    NonDecimalDigitInDSP(u8),
    NonISO646Character(u8),
    InternalError, // Can happen if fields are modified to produce internal incoherence.
    InvalidHexEncoding,
    UnsupportedDSPType,
    IDITooLong,
}

impl Display for NAddressError {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }

}

impl Error for NAddressError {}

/// This function was kept around so you can still get the AFI without parsing the whole NSAP.
pub const fn get_afi_from_n_address (naddr: &[u8]) -> Option<u8> {
    if naddr.len() == 0 {
        return None;
    }
    Some(naddr[0])
}

// TODO: Rename to clarify that this only means that the leading zeroes COULD be
// significant, but not necessarily that they are, since the IDI is padded with
// zeroes if the encoded abstract value does not have leading zeroes.

/// Returns `Some(true)` if leading zeroes in the IDI are significant,
/// `Some(false)` if not, and `None` if it is not known.
pub const fn idi_leading_0_significant (nt: X213NetworkAddressType) -> Option<bool> {
    match nt {
        X213NetworkAddressType::X121
        | X213NetworkAddressType::F69
        | X213NetworkAddressType::E163
        | X213NetworkAddressType::E164 => Some(true),
        X213NetworkAddressType::IANA_ICP
        | X213NetworkAddressType::ISO_6523_ICD
        | X213NetworkAddressType::ISO_DCC
        | X213NetworkAddressType::ITU_T_IND
        | X213NetworkAddressType::LOCAL => Some(false),
        _ => None,
    }
}

// FIXME: I think you should combine a lot of these functions into a single
// AFI -> (network_type, dsp syntax, leading_zeroes) lookup

/// Returns `Some(true)` if leading zeroes in the IDI are significant,
/// `Some(false)` if not, and `None` if it is not known.
///
/// Quoting X.213:
/// "The numerically greater AFI value is used when the first significant digit in the IDI is zero."
pub const fn idi_leading_0_significant_by_afi (afi: AFI) -> Option<bool> {
    match afi {
        0x52 // X.121 IDI with leading zeroes
        | 0x54 // F.69 IDI with leading zeroes
        | 0x56 // E.163 IDI with leading zeroes
        | 0x58 // E.164 IDI with leading zeroes
        => Some(true),
        0x36 // X.121 IDI with no leading zeroes
        | 0x38 // ISO DCC IDI
        | 0x40 // F.69 IDI with no leading zeroes
        | 0x42 // E.163 IDI with no leading zeroes
        | 0x44 // E.164 IDI with no leading zeroes
        | 0x46 // ISO 6523-ICD IDI
        | 0x34 // IANA ICP IDI
        | 0x76 // ITU-T IND IDI
        | 0x48 // Local IDI
        => Some(false),
        _ => None, // Otherwise, unknown
    }
}

/// Translate an individual AFI to its group equivalent
#[inline]
pub const fn individual_afi_to_group_afi (afi: u8) -> Option<u8> {
    match afi {
        0x10..=0x99 => Some(afi + 0x90),
        _ => None,
    }
}

/// Translate a group AFI to its individual equivalent
#[inline]
pub const fn group_afi_to_individual_afi (afi: u8) -> Option<u8> {
    match afi {
        0xA0..=0xF9 => Some(afi - 0x90),
        _ => None,
    }
}

/// Return `true` if this is an individual AFI
#[inline]
pub const fn is_individual_afi (afi: u8) -> bool {
    afi >= 0x10 && afi <= 0x99
}

/// Return `true` if this is a group AFI
#[inline]
pub const fn is_group_afi (afi: u8) -> bool {
    afi >= 0xA0 && afi <= 0xF9
}

/// Return `true` if this is an invalid AFI
#[inline]
pub const fn is_invalid_afi (afi: u8) -> bool {
    !is_individual_afi(afi) && !is_group_afi(afi)
}

/// Return get the N-address network type from the AFI
pub const fn afi_to_network_type (afi: AFI) -> Option<X213NetworkAddressType> {
    let ind_afi = match group_afi_to_individual_afi(afi) {
        Some(x) => x,
        None => afi,
    };
    match ind_afi {
        AFI_X121_DEC_LEADING_NON_ZERO
        | AFI_X121_DEC_LEADING_ZERO
        | AFI_X121_BIN_LEADING_NON_ZERO
        | AFI_X121_BIN_LEADING_ZERO => Some(X213NetworkAddressType::X121),
        AFI_ISO_DCC_DEC
        | AFI_ISO_DCC_BIN => Some(X213NetworkAddressType::ISO_DCC),
        AFI_F69_DEC_LEADING_NON_ZERO
        | AFI_F69_DEC_LEADING_ZERO
        | AFI_F69_BIN_LEADING_NON_ZERO
        | AFI_F69_BIN_LEADING_ZERO => Some(X213NetworkAddressType::F69),
        AFI_E163_DEC_LEADING_NON_ZERO
        | AFI_E163_DEC_LEADING_ZERO
        | AFI_E163_BIN_LEADING_NON_ZERO
        | AFI_E163_BIN_LEADING_ZERO => Some(X213NetworkAddressType::E163),
        AFI_E164_DEC_LEADING_NON_ZERO
        | AFI_E164_DEC_LEADING_ZERO
        | AFI_E164_BIN_LEADING_NON_ZERO
        | AFI_E164_BIN_LEADING_ZERO => Some(X213NetworkAddressType::E164),
        AFI_ISO_6523_ICD_DEC
        | AFI_ISO_6523_ICD_BIN => Some(X213NetworkAddressType::ISO_6523_ICD),
        AFI_IANA_ICP_DEC
        | AFI_IANA_ICP_BIN => Some(X213NetworkAddressType::IANA_ICP),
        AFI_ITU_T_IND_DEC
        | AFI_ITU_T_IND_BIN => Some(X213NetworkAddressType::ITU_T_IND),
        AFI_LOCAL_DEC
        | AFI_LOCAL_BIN
        | AFI_LOCAL_ISO_IEC_646
        | AFI_LOCAL_NATIONAL => Some(X213NetworkAddressType::LOCAL),
        _ => None,
    }
}

/// Convert the network type to a string
#[inline]
pub const fn naddr_network_type_to_str (nt: X213NetworkAddressType) -> Option<&'static str> {
    match nt {
        X213NetworkAddressType::X121 => Some(AFI_STR_X121),
        X213NetworkAddressType::ISO_DCC => Some(AFI_STR_DCC),
        X213NetworkAddressType::F69 => Some(AFI_STR_TELEX),
        X213NetworkAddressType::E163 => Some(AFI_STR_PSTN),
        X213NetworkAddressType::E164 => Some(AFI_STR_ISDN),
        X213NetworkAddressType::ISO_6523_ICD => Some(AFI_STR_ICD),
        X213NetworkAddressType::IANA_ICP => Some(AFI_STR_ICP), // Not specified in IETF RFC 1278. See: https://www.iana.org/assignments/osi-nsapa-numbers/osi-nsapa-numbers.xhtml
        X213NetworkAddressType::ITU_T_IND => Some(AFI_STR_IND), // Not specified in IETF RFC 1278.
        X213NetworkAddressType::LOCAL => Some(AFI_STR_LOCAL),
        X213NetworkAddressType::URL => Some(AFI_STR_URL), // Not specified in IETF RFC 1278.
    }
}

#[cfg(feature = "alloc")]
fn naddr_str_to_afi (
    s: &str,
    leading0: bool,
    dsp_syntax: DSPSyntax,
) -> Option<AFI> {
    /* I don't know the precise details of Rust's match implementation, but
    I implemented this as doubly-nested matching so that string comparison
    does not happen for every single case. */
    match s {
        crate::AFI_STR_X121 => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_X121_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_X121_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_X121_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_X121_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::AFI_STR_DCC => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_ISO_DCC_DEC),
            DSPSyntax::Binary => Some(AFI_ISO_DCC_BIN),
            _ => None,
        },
        crate::AFI_STR_TELEX => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_F69_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_F69_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_F69_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_F69_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::AFI_STR_PSTN => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_E163_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_E163_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_E163_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_E163_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::AFI_STR_ISDN => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_E164_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_E164_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_E164_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_E164_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::AFI_STR_ICD => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_ISO_6523_ICD_DEC),
            DSPSyntax::Binary => Some(AFI_ISO_6523_ICD_BIN),
            _ => None,
        },
        crate::AFI_STR_ICP => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_IANA_ICP_DEC),
            DSPSyntax::Binary => Some(AFI_IANA_ICP_BIN),
            _ => None,
        },
        crate::AFI_STR_IND => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_ITU_T_IND_DEC),
            DSPSyntax::Binary => Some(AFI_ITU_T_IND_BIN),
            _ => None,
        },
        crate::AFI_STR_LOCAL => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_LOCAL_DEC),
            DSPSyntax::Binary => Some(AFI_LOCAL_BIN),
            DSPSyntax::IsoIec646Chars => Some(AFI_LOCAL_ISO_IEC_646),
            DSPSyntax::NationalChars => Some(AFI_LOCAL_NATIONAL),
        },
        crate::AFI_STR_URL => Some(AFI_URL),
        _ => None,
    }
}

/// Get the max decimal length given a network address type
#[inline]
pub const fn naddr_network_type_to_max_dec_length (nt: X213NetworkAddressType) -> Option<u8> {
    match nt {
        X213NetworkAddressType::X121 => Some(MAX_DEC_DSP_LEN_X121),
        X213NetworkAddressType::ISO_DCC => Some(MAX_DEC_DSP_LEN_ISO_DCC),
        X213NetworkAddressType::F69 => Some(MAX_DEC_DSP_LEN_F69),
        X213NetworkAddressType::E163 => Some(MAX_DEC_DSP_LEN_E163),
        X213NetworkAddressType::E164 => Some(MAX_DEC_DSP_LEN_E164),
        X213NetworkAddressType::ISO_6523_ICD => Some(MAX_DEC_DSP_LEN_ISO_6523_ICD),
        X213NetworkAddressType::IANA_ICP => Some(MAX_DEC_DSP_LEN_IANA_ICP), // Not specified in IETF RFC 1278. See: https://www.iana.org/assignments/osi-nsapa-numbers/osi-nsapa-numbers.xhtml
        X213NetworkAddressType::ITU_T_IND => Some(MAX_DEC_DSP_LEN_ITU_T_IND), // Not specified in IETF RFC 1278.
        X213NetworkAddressType::LOCAL => Some(MAX_DEC_DSP_LEN_LOCAL),
        X213NetworkAddressType::URL => None,
    }
}

/// Get the max binary length given a network address type
#[inline]
pub const fn naddr_network_type_to_max_bin_length (nt: X213NetworkAddressType) -> Option<u8> {
    match nt {
        X213NetworkAddressType::X121 => Some(MAX_BIN_DSP_LEN_X121),
        X213NetworkAddressType::ISO_DCC => Some(MAX_BIN_DSP_LEN_ISO_DCC),
        X213NetworkAddressType::F69 => Some(MAX_BIN_DSP_LEN_F69),
        X213NetworkAddressType::E163 => Some(MAX_BIN_DSP_LEN_E163),
        X213NetworkAddressType::E164 => Some(MAX_BIN_DSP_LEN_E164),
        X213NetworkAddressType::ISO_6523_ICD => Some(MAX_BIN_DSP_LEN_ISO_6523_ICD),
        X213NetworkAddressType::IANA_ICP => Some(MAX_DEC_DSP_LEN_IANA_ICP), // Not specified in IETF RFC 1278. See: https://www.iana.org/assignments/osi-nsapa-numbers/osi-nsapa-numbBIN.xhtml
        X213NetworkAddressType::ITU_T_IND => Some(MAX_DEC_DSP_LEN_ITU_T_IND), // Not specified in IETF RFC BIN8.
        X213NetworkAddressType::LOCAL => Some(MAX_BIN_DSP_LEN_LOCAL),
        X213NetworkAddressType::URL => Some(u8::MAX), // There really is no limit, but 255 characters is good enough.
    }
}

/// Get the max IDI length (in digits) given a network address type
#[inline]
pub const fn get_idi_len_in_digits (nt: X213NetworkAddressType) -> Option<usize> {
    match nt {
        X213NetworkAddressType::X121 => Some(MAX_IDI_LEN_X121),
        X213NetworkAddressType::ISO_DCC => Some(MAX_IDI_LEN_ISO_DCC),
        X213NetworkAddressType::F69 => Some(MAX_IDI_LEN_F69),
        X213NetworkAddressType::E163 => Some(MAX_IDI_LEN_E163),
        X213NetworkAddressType::E164 => Some(MAX_IDI_LEN_E164),
        X213NetworkAddressType::ISO_6523_ICD => Some(MAX_IDI_LEN_ISO_6523_ICD),
        X213NetworkAddressType::IANA_ICP => Some(MAX_IDI_LEN_IANA_ICP), // Not specified in IETF RFC 1278. See: https://www.iana.org/assignments/osi-nsapa-numbers/osi-nsapa-numbBIN.xhtml
        X213NetworkAddressType::ITU_T_IND => Some(MAX_IDI_LEN_ITU_T_IND), // Not specified in IETF RFC BIN8.
        X213NetworkAddressType::LOCAL => Some(MAX_IDI_LEN_LOCAL),
        X213NetworkAddressType::URL => Some(MAX_IDI_LEN_URL),
    }
}

/// Return `true` if the DSP is in binary format
pub const fn naddr_dsp_is_binary (afi: u8) -> bool {
    let ind_afi = match group_afi_to_individual_afi(afi) {
        Some(x) => x,
        None => afi,
    };
    match ind_afi {
        AFI_X121_BIN_LEADING_NON_ZERO
        | AFI_X121_BIN_LEADING_ZERO
        | AFI_ISO_DCC_BIN
        | AFI_F69_BIN_LEADING_NON_ZERO
        | AFI_F69_BIN_LEADING_ZERO
        | AFI_E163_BIN_LEADING_NON_ZERO
        | AFI_E163_BIN_LEADING_ZERO
        | AFI_E164_BIN_LEADING_NON_ZERO
        | AFI_E164_BIN_LEADING_ZERO
        | AFI_ISO_6523_ICD_BIN
        | AFI_IANA_ICP_BIN
        | AFI_ITU_T_IND_BIN
        | AFI_LOCAL_BIN
        | AFI_URL => true,
        _ => false,
    }
}

/// Return `true` if the DSP is in decimal format
pub const fn naddr_dsp_is_decimal (afi: u8) -> bool {
    let ind_afi = match group_afi_to_individual_afi(afi) {
        Some(x) => x,
        None => afi,
    };
    match ind_afi {
        AFI_X121_DEC_LEADING_NON_ZERO
        | AFI_X121_DEC_LEADING_ZERO
        | AFI_ISO_DCC_DEC
        | AFI_F69_DEC_LEADING_NON_ZERO
        | AFI_F69_DEC_LEADING_ZERO
        | AFI_E163_DEC_LEADING_NON_ZERO
        | AFI_E163_DEC_LEADING_ZERO
        | AFI_E164_DEC_LEADING_NON_ZERO
        | AFI_E164_DEC_LEADING_ZERO
        | AFI_ISO_6523_ICD_DEC
        | AFI_IANA_ICP_DEC
        | AFI_ITU_T_IND_DEC
        | AFI_LOCAL_DEC => true,
        _ => false,
    }
}

/// Return `true` if the DSP is in ISO/IEC 646 format
pub const fn naddr_dsp_is_iso_iec_646 (afi: u8) -> bool {
    let ind_afi = match group_afi_to_individual_afi(afi) {
        Some(x) => x,
        None => afi,
    };
    match ind_afi {
        AFI_LOCAL_ISO_IEC_646 => true,
        _ => false,
    }
}

/// Return `true` if the DSP is in nationally-defined format
pub const fn naddr_dsp_is_national (afi: u8) -> bool {
    let ind_afi = match group_afi_to_individual_afi(afi) {
        Some(x) => x,
        None => afi,
    };
    match ind_afi {
        AFI_LOCAL_NATIONAL => true,
        _ => false,
    }
}

/// Return `true` if the IDI has leading zeroes
pub const fn naddr_idi_has_leading_zero (afi: u8) -> bool {
    let ind_afi = match group_afi_to_individual_afi(afi) {
        Some(x) => x,
        None => afi,
    };
    match ind_afi {
        AFI_X121_DEC_LEADING_ZERO
        | AFI_X121_BIN_LEADING_ZERO
        | AFI_F69_DEC_LEADING_ZERO
        | AFI_F69_BIN_LEADING_ZERO
        | AFI_E163_DEC_LEADING_ZERO
        | AFI_E163_BIN_LEADING_ZERO
        | AFI_E164_DEC_LEADING_ZERO
        | AFI_E164_BIN_LEADING_ZERO => true,
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct BCDDigitsIter<'a> {
    idi: &'a [u8],
    least_sig_nybble: bool,
    leading_0_sig: bool,
    processing_leading_digits: bool,
    ignore_last_nybble: bool,
}

impl <'a> BCDDigitsIter<'a> {

    pub fn new(
        idi: &'a [u8],
        leading_0_sig: bool,
        ignore_last_nybble: bool,
        least_sig_nybble: bool,
    ) -> BCDDigitsIter<'a> {
        BCDDigitsIter{
            idi,
            leading_0_sig,
            ignore_last_nybble,
            // FIXME: This should be false when processing the DSP
            processing_leading_digits: true, // Start off handling leading digits
            least_sig_nybble, // Start off on the MSn
        }
    }

}

/// This SHOULD BE an ASCII digit, but might not be. It is on the caller to
/// check this and determine what to do if this has a non-digit value.
pub type ShouldBeASCIIDigit = u8;

impl <'a> Iterator for BCDDigitsIter<'a> {
    type Item = ShouldBeASCIIDigit;

    /// This implementation does NOT handle malformed digits. The caller MUST
    /// check for non-ASCII digits being returned
    fn next(&mut self) -> Option<Self::Item> {
        while self.idi.len() > 0 {
            let nybble: u8 = if self.least_sig_nybble {
                self.idi[0] & 0b0000_1111
            } else {
                (self.idi[0] & 0b1111_0000) >> 4
            };
            if self.least_sig_nybble {
                self.least_sig_nybble = false;
                self.idi = &self.idi[1..];
            } else {
                self.least_sig_nybble = true;
            }
            if self.processing_leading_digits {
                let leading_digit: u8 = if self.leading_0_sig { 1 } else { 0 };
                if nybble == leading_digit {
                    continue;
                } else {
                    self.processing_leading_digits = false;
                }
            }
            // If the last nybble is 0b1111, it is padding.
            // If the DSP is in decimal digits, the last nybble of the
            if self.idi.len() == 0 && (nybble == 0b1111 || self.ignore_last_nybble) {
                return None;
            }
            return Some(nybble);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut max_digits = self.idi.len() << 1; // Double it
        if self.least_sig_nybble {
            max_digits = max_digits.saturating_sub(1);
        }
        if self.ignore_last_nybble {
            max_digits = max_digits.saturating_sub(1);
        }
        // Every digit could be a leading digit
        (0, Some(max_digits))
    }
}

impl <'a> FusedIterator for BCDDigitsIter<'a> {}

/// This type does not implement `PartialEq`, `Eq`, or `Hash`, because:
///
/// 1. Unrecognized encodings could mean that two values cannot be compared for
///    equality because their semantics are unknown.
/// 2. Even among recognized encodings, it is not clear whether or not the
///    decimal encoding should always be considered equal to the binary
///    encoding.
/// 3. The semantics of the DSP encodings seems to be undefined for most AFIs.
///
/// A simple `Eq` or `Hash` implementation could just use the raw octets, but
/// this could contradict cases where two different encoding should be treated
/// as equal. Letting the caller explicitly hash or compare the octets is more
/// clear as to what the underlying behavior is.
///
#[derive(Debug)]
pub struct X213NetworkAddress <'a> {
    #[cfg(feature = "alloc")]
    pub(crate) octets: Cow<'a, [u8]>,
    #[cfg(not(feature = "alloc"))]
    pub(crate) octets: &'a [u8],
}

impl <'a> X213NetworkAddress <'a> {

    #[inline]
    pub const fn get_octets(&'a self) -> &'a [u8] {
        #[cfg(feature = "alloc")]
        {
            match &self.octets {
                Cow::Borrowed(o) => o,
                Cow::Owned(o) => o.as_slice(),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            &self.octets
        }
    }

    #[inline]
    pub const fn afi(&self) -> u8 {
        if self.get_octets().len() > 0 {
            self.get_octets()[0]
        } else {
            panic!("Zero-length IDP in an X.213 network address")
        }
    }

    // #[inline]
    // pub const fn idi(&self) -> &'a [u8] {
    //     if self.octets.len() > 0 {
    //         // Sorry for the unsafe code. I was just trying to do
    //         // &self.idp[1..self.idp.len()] in a const fn.
    //         let slice = &self.octets;
    //         unsafe { core::slice::from_raw_parts(slice.as_ptr().add(1), slice.len() - 1) }
    //     } else {
    //         panic!("Zero-length IDP in an X.213 network address")
    //     }
    // }

    #[inline]
    pub const fn get_network_type_info(&self) -> Option<X213NetworkAddressInfo> {
        get_address_type_info(self.afi())
    }

    #[inline]
    pub const fn get_network_type(&self) -> Option<X213NetworkAddressType> {
        afi_to_network_type(self.afi())
    }

    pub fn idi_digits(&'a self) -> Option<BCDDigitsIter<'a>> {
        // TODO: Skip padding digits
        let addr_type_info = get_address_type_info(self.afi())?;
        let leading_0_sig = addr_type_info.leading_zeroes_in_idi;
        let is_dsp_decimal = matches!(addr_type_info.dsp_syntax, DSPSyntax::Decimal);
        // TODO: Return IDI len from get_address_type_info()?
        let idi_len = get_idi_len_in_digits(addr_type_info.network_type)?;
        let idi_len_in_bytes = idi_len >> 1;
        let odd_len_idi: bool = (idi_len % 2) > 0;
        let idi = &self.octets[1..1+idi_len_in_bytes];
        Some(BCDDigitsIter::new(
            idi,
            leading_0_sig,
            is_dsp_decimal && odd_len_idi,
            false,
        ))
    }

    pub fn dsp_digits(&'a self) -> Option<BCDDigitsIter<'a>> {
        let addr_type_info = get_address_type_info(self.afi())?;
        let is_dsp_decimal = matches!(addr_type_info.dsp_syntax, DSPSyntax::Decimal);
        if !is_dsp_decimal {
            return None;
        }
        let idi_len = get_idi_len_in_digits(addr_type_info.network_type)?;
        let idi_len_in_bytes: usize = idi_len >> 1;
        let odd_len_idi: bool = (idi_len % 2) > 0;
        // This needs to take the byte before if odd number of IDI digits
        let (dsp, start_on_lsn) = if is_dsp_decimal && odd_len_idi {
            (&self.octets[idi_len_in_bytes..], true)
        } else {
            (&self.octets[1+idi_len_in_bytes..], false)
        };
        Some(BCDDigitsIter::new(
            dsp,
            false, // No leading zeroes supported in DSPs
            false, // Only ignore the last nybble if it is 0x0F
            start_on_lsn,
        ))
    }

    pub fn get_url(&'a self) -> Option<&'a str> {
        let octets = self.octets.as_ref();
        // It couldn't be a valid URL in two characters, AFAIK.
        if octets.len() < 5 || octets[0] != AFI_URL {
            return None;
        }
        str::from_utf8(&octets[3..]).ok()
    }

    /// This **does not** extract an IP address from a URL.
    ///
    /// See: <https://www.rfc-editor.org/rfc/rfc4548.html>
    pub fn get_ip(&self) -> Option<IpAddr> {
        let octets = self.octets.as_ref();
        if octets.len() < 7 || octets[0] != AFI_IANA_ICP_BIN {
            return None;
        }
        // See doc comments on AFI_IANA_ICP_DEC for why it is not supported.
        match (octets[1], octets[2]) {
            (0, 0) => { // IPv6
                if octets.len() < 19 {
                    return None;
                }
                let ip = Ipv6Addr::from([
                    octets[3],  octets[4],  octets[5],  octets[6],
                    octets[7],  octets[8],  octets[9],  octets[10],
                    octets[11], octets[12], octets[13], octets[14],
                    octets[15], octets[16], octets[17], octets[18],
                ]);
                Some(IpAddr::V6(ip))
            },
            (0, 1) => { // IPv4
                let ip = Ipv4Addr::from([
                    octets[3],
                    octets[4],
                    octets[5],
                    octets[6],
                ]);
                Some(IpAddr::V4(ip))
            },
            _ => None,
        }
    }

    pub fn get_itot_socket_addr(&self) -> Option<SocketAddrV4> {
        let octets = self.octets.as_ref();
        if !octets.starts_with(INTERNET_PREFIX.as_slice()) {
            return None;
        }
        let dsp = &octets[INTERNET_PREFIX.len()..];
        if dsp.len() < 12 {
            return None;
        }
        if !dsp.iter().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let dsp = unsafe { str::from_utf8_unchecked(dsp) };
        let oct1: u8 = dsp[0.. 3].parse().ok()?;
        let oct2: u8 = dsp[3.. 6].parse().ok()?;
        let oct3: u8 = dsp[6.. 9].parse().ok()?;
        let oct4: u8 = dsp[9..12].parse().ok()?;
        let ip = Ipv4Addr::new(oct1, oct2, oct3, oct4);
        if dsp.len() < 17 {
            return Some(SocketAddrV4::new(ip, DEFAULT_ITOT_PORT));
        }
        let port: u16 = dsp[12..17].parse().ok()?;
        Some(SocketAddrV4::new(ip, port))
    }

}

// TODO: Also support the special directory URL AFI 0xFF

impl <'a> TryFrom<&'a [u8]> for X213NetworkAddress <'a> {
    type Error = NAddressError;

    fn try_from(octets: &'a [u8]) -> Result<Self, Self::Error> {
        if octets.len() < 2 { // I don't think one byte can be a valid address.
            // FIXME: This should just be TooShort
            return Err(NAddressError::NoAFI);
        }
        /* ITU-T Rec. X.213, Section A.5.4 states that the maximum length MUST
        be 20 octets, but ITU-T Rec. X.519 section 11.4 basically overrules
        that. As such, we are just setting a limit of 248 bytes just to close up
        any attack vectors related to large NSAP addresses. */
        if octets.len() > 248 {
            // FIXME: This should just be TooLong
            return Err(NAddressError::IDITooLong);
        }
        // TODO: Based on AFI, check that it is not too short
        // Ok(X213NetworkAddress { octets })
        #[cfg(feature = "alloc")]
        {
            Ok(X213NetworkAddress { octets: Cow::Borrowed(octets) })
        }
        #[cfg(not(feature = "alloc"))]
        {
            Ok(X213NetworkAddress { octets })
        }
    }

}

impl <'a> Display for X213NetworkAddress<'a> {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.octets.as_ref().get(0..3) {
            Some(octs) if octs[0] == AFI_URL => {
                if let Ok(url) = str::from_utf8(&self.octets[3..]) {
                    if !url.contains('_') {
                        return write!(f, "URL+{:02X}{:02X}+{}", octs[1], octs[2], url);
                    }
                }
            },
            _ => (),
        };
        let (info, idi_digits) = match (self.get_network_type_info(), self.idi_digits()) {
            (Some(i), Some(d)) => (i, d),
            _ => { // If unrecognized, just print in NS+<hex> format
                let h = hex::encode(self.get_octets());
                f.write_str("NS+")?;
                return f.write_str(h.as_str());
            }
        };
        match naddr_network_type_to_str(info.network_type) {
            Some(s) => f.write_str(s)?,
            None => write!(f, "{:02X}", self.afi())?,
        };
        f.write_char('+')?;
        for digit in idi_digits {
            f.write_char((digit + 0x30) as char)?;
        }
        f.write_char('+')?;
        let idi_len = get_idi_len_in_digits(info.network_type).unwrap();
        let idi_len_in_bytes: usize = idi_len >> 1;
        // TODO: Display RFC-1006 / X.25(80), etc.
        match info.dsp_syntax {
            DSPSyntax::Decimal => {
                match self.dsp_digits() {
                    Some(dsp_digits) => {
                        f.write_char('d')?;
                        for digit in dsp_digits {
                            f.write_char((digit + 0x30).into())?;
                        }
                    },
                    None => {
                        // This shouldn't happen
                        f.write_char('x')?;
                        // FIXME: could read out of bounds
                        let dsp = &self.octets[1+idi_len_in_bytes..];
                        for byte in dsp {
                            f.write_fmt(format_args!("{:02X}", *byte))?;
                        }
                    },
                };
            },
            DSPSyntax::Binary | DSPSyntax::NationalChars => {
                // FIXME: Make this part of get_schema
                let dsp = &self.octets[1+idi_len_in_bytes..];
                f.write_char('x')?;
                for byte in dsp {
                    f.write_fmt(format_args!("{:02X}", *byte))?;
                }
            },
            DSPSyntax::IsoIec646Chars => {
                let dsp = &self.octets[1+idi_len_in_bytes..];
                let decode = dsp
                    .iter()
                    .map(|b| local_iso_iec_646_byte_to_char(*b).unwrap_or('?'));
                for c in decode {
                    f.write_char(c)?;
                }
            },
        };
        Ok(())
        // DomainSpecificPart::Url(url) => f.write_str(&url),
        // DomainSpecificPart::IpAddress(ip) => f.write_str(&ip.to_string()),
    }

}

fn decode_afi_from_str(s: &str) -> Result<AFI, RFC1278ParseError> {
    debug_assert_eq!(s.len(), 2);
    let mut out: [u8; 1] = [0];
    hex::decode_to_slice(s, out.as_mut_slice())
        .map_err(|_| RFC1278ParseError::Malformed)?;
    Ok(out[0])
}

/*
Every valid NSAP string has a second part... except per ITU-T Rec.
X.213, section A.7, which handles a zero-length DSP.

Note: there seems to be an error in RFC 1278 in making the
second <hexstring> non-optional, since this is optional in X.213.
X.213 also says that the first byte of the <idp> may be hex,
which RFC 1278 does not permit.
*/
fn decode_idp_only<'a>(s: &'a str) -> Result<X213NetworkAddress<'static>, RFC1278ParseError> {
    if !s[2..].as_bytes().iter().all(|b| b.is_ascii_digit()) {
        return Err(RFC1278ParseError::Malformed);
    }
    let afi = decode_afi_from_str(&s[0..2])?;
    // If the schema is not known, we cannot construct an NSAP,
    // because we don't know how long the IDI is.
    let schema = get_address_type_info(afi)
        .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
    let idi_len_digits = get_idi_len_in_digits(schema.network_type)
        .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
    let mut bcd_buf = BCDBuffer::new();
    bcd_buf.push_byte(afi);
    let idi_pad = if schema.leading_zeroes_in_idi { 1 } else { 0 };
    let mut idi_deficit = idi_len_digits.saturating_sub(s.len() - 2);
    while idi_deficit > 0 {
        bcd_buf.push_nybble(idi_pad);
        idi_deficit -= 1;
    }
    bcd_buf.push_str(&s[2..]);
    if (bcd_buf.i % 2) > 0 {
        bcd_buf.push_nybble(0xF);
    }
    return Ok(X213NetworkAddress {
        octets: Cow::Owned(bcd_buf.as_ref().to_vec()),
    });
}

pub const fn leading_0_in_idi_significant(nt: X213NetworkAddressType) -> bool {
    nt as usize == X213NetworkAddressType::F69 as usize
    || nt as usize == X213NetworkAddressType::E163 as usize
    || nt as usize == X213NetworkAddressType::E164 as usize
    || nt as usize == X213NetworkAddressType::X121 as usize
}

fn u8_to_decimal_bytes(mut n: u8) -> [u8; 3] {
    let hundreds = n / 100;
    n %= 100;
    let tens = n / 10;
    let ones = n % 10;
    [
        b'0' + hundreds,
        b'0' + tens,
        b'0' + ones,
    ]
}

fn u16_to_decimal_bytes(mut n: u16) -> [u8; 5] {
    let ten_thousands = (n / 10000) as u8;
    n %= 10000;
    let thousands = (n / 1000) as u8;
    n %= 1000;
    let hundreds = (n / 100) as u8;
    n %= 100;
    let tens = (n / 10) as u8;
    let ones = (n % 10) as u8;
    [
        b'0' + ten_thousands,
        b'0' + thousands,
        b'0' + hundreds,
        b'0' + tens,
        b'0' + ones,
    ]
}

#[inline]
fn validate_digitstring(s: &str, max_len: usize) -> Result<(), RFC1278ParseError> {
    if s.len() > max_len {
        return Err(RFC1278ParseError::Malformed);
    }
    if !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(RFC1278ParseError::Malformed);
    }
    Ok(())
}

#[inline]
const fn is_other_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.'
}

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
impl <'a> FromStr for X213NetworkAddress<'a> {
    type Err = RFC1278ParseError;

    fn from_str(s: &str) -> Result<Self, RFC1278ParseError> {
        // I think this is the shortest possible: NS+0011 or DCC+1+2
        if s.len() < 7 || !s.is_ascii() {
            return Err(RFC1278ParseError::Malformed);
        }
        let mut parts = s.split('+');
        let first_part = parts.next().ok_or(RFC1278ParseError::Malformed)?;
        if first_part.len() < 2 {
            return Err(RFC1278ParseError::Malformed);
        }
        let second_part = match parts.next() {
            Some(sp) => sp,
            None => return decode_idp_only(first_part),
        };
        if first_part == "NS" {
            let hexbytes = hex::decode(second_part)
                .map_err(|_| RFC1278ParseError::Malformed)?;
            return Ok(X213NetworkAddress { octets: Cow::Owned(hexbytes) });
        }
        let mut bcd_buf = BCDBuffer::new();
        let third_part = parts.next();
        if first_part == "URL" {
            validate_digitstring(second_part, 4)?;
            let url = match third_part {
                Some(u) => u,
                None => return Err(RFC1278ParseError::Malformed),
            };
            /* The URL cannot contain underscores only because RFC 1278 uses
            underscores to separate NSAP addresses in a presentation address. */
            if url.contains('_') {
                return Err(RFC1278ParseError::ProhibitedCharacter('_'));
            }
            let mut idi_deficit = 4usize.saturating_sub(second_part.len());
            while idi_deficit > 0 {
                bcd_buf.push_nybble(0);
                idi_deficit -= 1;
            }
            bcd_buf.push_str(second_part);
            let out = [
                [AFI_URL].as_ref(),
                bcd_buf.as_ref(),
                url.as_bytes(),
            ].concat();
            return Ok(X213NetworkAddress { octets: Cow::Owned(out) });
        }
        let syntax: DSPSyntax = match third_part.and_then(|p3| p3.chars().next()) {
            Some('d') => DSPSyntax::Decimal,
            Some('x') => DSPSyntax::Binary,
            Some('l') => DSPSyntax::IsoIec646Chars,
            _ => if third_part.is_some_and(|p3| p3.starts_with("ECMA-117-Binary+")) {
                DSPSyntax::Binary
            } else {
                // All other encodings are assumed to be decimal.
                // This is true for all of:
                // * RFC-1006+
                // * X.25(80)+
                // * ECMA-117-Decimal+
                DSPSyntax::Decimal
            }
        };
        let maybe_afi = naddr_str_to_afi(first_part, second_part.starts_with("0"), syntax);
        if let Some(afi) = maybe_afi {
            // This MUST be <afi> "+" <idi> [ "+" <dsp> ] syntax.
            let schema = get_address_type_info(afi)
                .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
            let idi_len_digits: usize = get_idi_len_in_digits(schema.network_type)
                .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
            if second_part.len() > idi_len_digits
                || !second_part.bytes().all(|b| b.is_ascii_digit()) {
                return Err(RFC1278ParseError::Malformed);
            }
            bcd_buf.push_byte(afi);
            let idi_pad = if schema.leading_zeroes_in_idi { 1 } else { 0 };
            let mut idi_deficit = idi_len_digits.saturating_sub(second_part.len());
            while idi_deficit > 0 {
                bcd_buf.push_nybble(idi_pad);
                idi_deficit -= 1;
            }
            bcd_buf.push_str(&second_part);
            if (idi_len_digits % 2) > 0 {
                bcd_buf.push_nybble(0x0F);
            }
            if third_part.is_none() {
                return Ok(X213NetworkAddress {
                    octets: Cow::Owned(bcd_buf.as_ref().to_vec()),
                });
            }
            let third_part = third_part.unwrap();
            if third_part.len() < 2 {
                // Cannot be empty and must have a discriminator (e.g. 'd')
                return Err(RFC1278ParseError::Malformed);
            }
            return match third_part.as_bytes()[0] as char {
                'd' => { // decimal syntax
                    if !third_part.as_bytes()[1..].iter().all(|b| b.is_ascii_digit()) {
                        return Err(RFC1278ParseError::Malformed);
                    }
                    bcd_buf.push_ascii_bytes(&third_part.as_bytes()[1..]);
                    Ok(X213NetworkAddress {
                        octets: Cow::Owned(bcd_buf.as_ref().to_vec()),
                    })
                },
                'x' => {
                    let hexbytes = hex::decode(&third_part.as_bytes()[1..])
                        .map_err(|_| RFC1278ParseError::Malformed)?;
                    let out = [
                        bcd_buf.as_ref(),
                        hexbytes.as_ref(),
                    ].concat();
                    Ok(X213NetworkAddress { octets: Cow::Owned(out) })
                },
                'l' => {
                    // RFC 1278: <other> ::= [0-9a-zA-Z+-.]
                    if !third_part.chars().all(is_other_char) {
                        return Err(RFC1278ParseError::Malformed);
                    }
                    let outlen = bcd_buf.len_in_bytes() + third_part.len();
                    let mut out = Vec::with_capacity(outlen);
                    out.extend(bcd_buf.as_ref());
                    out.extend(third_part[1..]
                        .chars()
                        // We check for permitted characters above, so the
                        // unwrap() below should never fail.
                        .map(|c| char_to_local_iso_iec_646_byte(c).unwrap()));
                    Ok(X213NetworkAddress { octets: Cow::Owned(out) })
                },
                _ => {
                    if third_part == IETF_RFC_1006_PREFIX_STR {
                        // "RFC-1006" "+" <prefix> "+" <ip> [ "+" <port> [ "+" <tset> ]]
                        let prefix = parts.next();
                        let ip = parts.next();
                        let port = parts.next();
                        let tset = parts.next();
                        if prefix.is_none() || ip.is_none() {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        let prefix = prefix.unwrap();
                        if prefix.len() != 2 || !prefix.bytes().all(|b| b.is_ascii_digit()) {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        bcd_buf.push_digit_u8(prefix.as_bytes()[0]);
                        bcd_buf.push_digit_u8(prefix.as_bytes()[1]);
                        let ip = ip.unwrap();
                        let ip = Ipv4Addr::from_str(ip)
                            .map_err(|_| RFC1278ParseError::ResolveDNS(ip.to_owned()))?;
                        if port.is_some_and(|p| p.len() != 5) {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        if tset.is_some_and(|t| t.len() != 5) {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        let port = match port {
                            Some(p) => Some(u16::from_str(p)
                                .map_err(|_| RFC1278ParseError::Malformed)?),
                            None => None,
                        };
                        let tset = match tset {
                            Some(t) => Some(u16::from_str(t)
                                .map_err(|_| RFC1278ParseError::Malformed)?),
                            None => None,
                        };

                        ip
                            .octets()
                            .map(|o| u8_to_decimal_bytes(o))
                            .iter()
                            .for_each(|dec_oct| bcd_buf.push_ascii_bytes(dec_oct.as_slice()));
                        if let Some(port) = port {
                            let port_str = u16_to_decimal_bytes(port);
                            bcd_buf.push_ascii_bytes(port_str.as_slice());
                        }
                        if let Some(tset) = tset {
                            let tset_str = u16_to_decimal_bytes(tset);
                            bcd_buf.push_ascii_bytes(tset_str.as_slice());
                        }
                        return Ok(X213NetworkAddress {
                            octets: Cow::Owned(bcd_buf.as_ref().to_vec()),
                        });
                    }
                    if third_part == X25_PREFIX_STR {
                        // "X.25(80)" "+" <prefix> "+" <dte> [ "+" <cudf-or-pid> "+" <hexstring> ]
                        let prefix = parts.next();
                        let dte = parts.next();
                        let cudf_of_pid = parts.next();
                        let cudf_of_pid_hex = parts.next();
                        if prefix.is_none()
                            || dte.is_none()
                            || (cudf_of_pid.is_some() && cudf_of_pid_hex.is_none()) {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        let prefix = prefix.unwrap();
                        let dte = dte.unwrap();
                        if !prefix.bytes().all(|b| b.is_ascii_digit())
                            || !dte.bytes().all(|b| b.is_ascii_digit()) {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        bcd_buf.push_str(prefix);
                        match cudf_of_pid {
                            Some("PID") => bcd_buf.push_digit_u8(0x31),
                            Some("CUDF") => bcd_buf.push_digit_u8(0x32),
                            Some(_) => return Err(RFC1278ParseError::Malformed),
                            None => bcd_buf.push_digit_u8(0x30), // DTE-only
                        };
                        if let Some(hexstr) = cudf_of_pid_hex {
                            let mut hexout: [u8; 20] = [0; 20];
                            if (hexstr.len() % 2) > 0 || hexstr.len() > 14 {
                                // If my calculations are correct, only a
                                // 7-byte long CUDF/PID fits in an NSAP addr.
                                return Err(RFC1278ParseError::Malformed);
                            }
                            let bytelen = hexstr.len() >> 1;
                            hex::decode_to_slice(hexstr, &mut hexout[0..bytelen])
                                .map_err(|_| RFC1278ParseError::Malformed)?;
                            // This is the CUDF/PID length field
                            bcd_buf.push_digit_u8(bytelen as u8 + 0x30);
                            // Then the CUDF/PID itself
                            for b in hexout[0..bytelen].iter() {
                                let b_dec = u8_to_decimal_bytes(*b);
                                bcd_buf.push_ascii_bytes(b_dec.as_slice());
                            }
                        }
                        let dte_len_bytes = (dte.len() >> 1) + (dte.len() % 2);
                        if bcd_buf.len_in_bytes() + dte_len_bytes > 20 {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        bcd_buf.push_str(dte);
                        return Ok(X213NetworkAddress {
                            octets: Cow::Owned(bcd_buf.as_ref().to_vec()),
                        });
                    }
                    if third_part == ECMA_117_BINARY_STR {
                        // "ECMA-117-Binary" "+" <hexstring> "+" <hexstring> "+" <hexstring>
                        let d1 = parts.next();
                        let d2 = parts.next();
                        let d3 = parts.next();
                        let d4 = parts.next();
                        if d1.is_none() || d2.is_none() || d3.is_none() || d4.is_some() {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        let mut subnet_id: [u8; 2] = [0, 0];
                        let mut selector: [u8; 1] = [0];
                        // decode_to_slice handles our length-checking.
                        hex::decode_to_slice(d1.unwrap(), subnet_id.as_mut_slice())
                            .map_err(|_| RFC1278ParseError::Malformed)?;
                        hex::decode_to_slice(d3.unwrap(), selector.as_mut_slice())
                            .map_err(|_| RFC1278ParseError::Malformed)?;
                        let subnet_addr = hex::decode(d2.unwrap())
                            .map_err(|_| RFC1278ParseError::Malformed)?;
                        if subnet_addr.len() > 6 {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        let mut out: Vec<u8> = Vec::with_capacity(bcd_buf.len_in_bytes() + 9);
                        out.extend(bcd_buf.as_ref());
                        out.extend(subnet_id.as_slice());
                        out.extend(subnet_addr.as_slice());
                        out.extend(selector.as_slice());
                        return Ok(X213NetworkAddress { octets: Cow::Owned(out) });
                    }
                    if third_part == ECMA_117_DECIMAL_STR {
                        // "ECMA-117-Decimal" "+" <digitstring> "+" <digitstring> "+" <digitstring>
                        let d1 = parts.next();
                        let d2 = parts.next();
                        let d3 = parts.next();
                        let d4 = parts.next();
                        if d1.is_none() || d2.is_none() || d3.is_none() || d4.is_some() {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        let d1 = d1.unwrap();
                        let d2 = d2.unwrap();
                        let d3 = d3.unwrap();
                        if d1.len() != 5 || d2.len() > 15 || d3.len() != 3
                            || !d1.chars().all(|c| c.is_ascii_digit())
                            || !d2.chars().all(|c| c.is_ascii_digit())
                            || !d3.chars().all(|c| c.is_ascii_digit()) {
                            return Err(RFC1278ParseError::Malformed);
                        }
                        bcd_buf.push_str(d1);
                        bcd_buf.push_str(d2);
                        bcd_buf.push_str(d3);
                        return Ok(X213NetworkAddress {
                            octets: Cow::Owned(bcd_buf.as_ref().to_vec()),
                        });
                    }
                    return Err(RFC1278ParseError::UnrecognizedSyntax);
                }
            }
        }
        // Otherwise, assume it is <idp> "+" <hexstring>
        if !first_part[2..].as_bytes().iter().all(|b| b.is_ascii_digit())
            || first_part.len() < 2
            || third_part.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        let afi = decode_afi_from_str(&s[0..2])?;
        let schema = get_address_type_info(afi)
            .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
        let idi_len_digits: usize = get_idi_len_in_digits(schema.network_type)
            .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
        if (idi_len_digits % 2) > 0 && syntax == DSPSyntax::Decimal {
            /* In the encoding specified in ITU-T Rec. X.213, Section A.7, it
            is not clear how to encode decimal DSPs when the first digit
            occupies the last nybble of the IDP's last octet. It is not clear
            if an odd number of hex characters could be used, or if this
            representation is only suitable for binary DSPs. */
            return Err(RFC1278ParseError::SpecificationFailure);
        }
        bcd_buf.push_byte(afi);
        let idi_pad = if schema.leading_zeroes_in_idi { 1 } else { 0 };
        let mut idi_deficit = idi_len_digits.saturating_sub(first_part.len() - 2);
        while idi_deficit > 0 {
            bcd_buf.push_nybble(idi_pad);
            idi_deficit -= 1;
        }
        bcd_buf.push_str(&first_part[2..]);
        if (bcd_buf.i % 2) > 0 {
            bcd_buf.push_nybble(0xF);
        }
        let dsp = hex::decode(second_part).map_err(|_| RFC1278ParseError::Malformed)?;
        let out = [
            bcd_buf.as_ref(),
            dsp.as_ref(),
        ].concat();
        return Ok(X213NetworkAddress { octets: Cow::Owned(out) });
    }

}

// TODO: PartialEq, Eq
// TODO: Hash

#[cfg(test)]
mod tests {

    extern crate alloc;
    use core::str::FromStr;

    use alloc::string::ToString;
    use super::X213NetworkAddress;

    #[test]
    fn test_display_01() {
        let input = [
            0x36u8,
            0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, // IDI = 102030405
            0x12, 0x34, 0x56, 0x78, 0x90,
        ];
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "X121+102030405+d1234567890");
    }

    #[test]
    fn test_display_02_url() {
        let input = b"\xFF\x00\x01https://wildboarsoftware.com/x500directory";
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "URL+0001+https://wildboarsoftware.com/x500directory");
    }

    #[test]
    fn test_get_url() {
        let input = b"\xFF\x00\x01https://wildboarsoftware.com/x500directory";
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        assert_eq!(addr.get_url().unwrap(), "https://wildboarsoftware.com/x500directory");
    }

    #[test]
    fn test_from_str() {
        let cases: [(&str, &[u8]); 5] = [
            // Example from RFC 1278
            ("NS+a433bb93c1", &[0xa4, 0x33, 0xbb, 0x93, 0xc1]),
            // Example from RFC 1278
            ("X121+234219200300", &[0x36, 0x00, 0x23, 0x42, 0x19, 0x20, 0x03, 0x00 ]),
            // Example from RFC 1278
            ("TELEX+00728722+RFC-1006+03+10.0.0.6", &[
                0x54,
                0x00, 0x72, 0x87, 0x22,
                0x03,
                0x01, 0x00, 0x00, 0x00, 0x00, 0x06, // 10.0.0.6
            ]),
            // Example from RFC 1278
            // This one deviates from RFC 1278. It seems like it had quotes
            // around the CUDF in error. I am not totally sure.
            ("TELEX+00728722+X.25(80)+02+00002340555+CUDF+892796", &[
                0x54,
                0x00, 0x72, 0x87, 0x22,
                0x02,
                0x23, // CUDF, which is 3 octets encoded as 9 digits
                0x13, 0x70, 0x39, 0x15, 0x00, // The last 0 here is for the DTE
                0x00, 0x02, 0x34, 0x05, 0x55, // All but the first digit of the DTE
            ]),
            // Non-standard syntax for X.519 URLs
            ("URL+001+https://wildboarsoftware.com/x500directory",
            b"\xFF\x00\x01https://wildboarsoftware.com/x500directory"),
        ];
        for (case_str, expected) in cases {
            let actual = X213NetworkAddress::from_str(case_str);
            assert!(actual.is_ok(), "failed to parse: {}", case_str);
            let actual = actual.unwrap();
            assert_eq!(expected, actual.octets.as_ref(), "{}", case_str);
        }
    }

}
