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
mod data;
use core::fmt::{Display, Write};
use core::error::Error;
use core::convert::TryFrom;
use core::iter::{Iterator, FusedIterator};
use crate::data::{get_address_type_info, X213NetworkAddressInfo};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use core::str::FromStr;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// TODO: Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
// TODO: Is there a separate ATN addressing? It sounds like ATN uses ISO 6523 (ICD)

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
pub const IETF_RFC_1006_PREFIX_STR: &str = "RFC-1006+";
pub const X25_PREFIX_STR: &str = "X.25(80)+";
pub const ECMA_117_BINARY_STR: &str = "ECMA-117-Binary+";
pub const ECMA_117_DECIMAL_STR: &str = "ECMA-117-Decimal+";

/// This is exported for convenience, since the Internet is most likely to be
/// used in NSAPs now. If an application only wants / can use Internet NSAPs,
/// the NSAPs could be checked to see if they begin with this sequence.
pub const INTERNET_PREFIX: [u8; 5] = [
    AFI_F69_DEC_LEADING_ZERO, // AFI
    0x00, 0x72, 0x87, 0x22, // IDI
];

pub const INTERNET_PREFIX_IDI_DIGITS: [u8; 8] = *b"00728722";

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
                // Although the ISO/IEC 646 syntax can take any graphic
                // characters and SPACE, the IETF RFC 1278 string formatting
                // only accepts alphanumerics, +, -, and period. I think part of
                // this is because the underscore is used to separate NSAP
                // addresses in the same specification.
                if dsp.iter().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'+' | b'-' | b'.')) {
                    f.write_char('l')?;
                    for c in dsp {
                        f.write_char((*c).into())?;
                    }
                } else {
                    // If the DSP is not ASCII (which it should be), just write hex
                    f.write_char('x')?;
                    for byte in dsp {
                        f.write_fmt(format_args!("{:02X}", *byte))?;
                    }
                }
            },
        };
        Ok(())
        // DomainSpecificPart::Url(url) => f.write_str(&url),
        // DomainSpecificPart::IpAddress(ip) => f.write_str(&ip.to_string()),
    }

}

fn decode_afi_from_str(s: &str) -> Result<AFI, ()> {
    if s.len() != 2 {
        return Err(());
    }
    let mut out: [u8; 1] = [0];
    hex::decode_to_slice(s, out.as_mut_slice()).map_err(|_| ())?;
    Ok(out[0])
}

fn write_bcd(out: &mut Vec<u8>, digits: &[u8], idi_len_digits: usize) {
    let mut out_byte: u8 = 0;
    for (i, digit) in digits.iter().enumerate() {
        if (i % 2) > 0 { // On least significant nybble
            out_byte |= *digit - 0x30;
            out.push(out_byte);
            out_byte = 0;
        } else { // On most significant nybble
            out_byte |= (*digit - 0x30) << 4;
        }
    }
    if (idi_len_digits % 2) > 0 {
        out_byte |= 0x0F;
        out.push(out_byte);
    }
}

#[cfg(feature = "alloc")]
impl <'a> FromStr for X213NetworkAddress<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // I think this is the shortest possible: NS+0011 or DCC+1+2
        if s.len() < 7 {
            return Err(());
        }
        if !s.is_ascii() {
            return Err(());
        }
        let mut parts = s.split('+');
        let first_part = parts.next().ok_or(())?;
        if first_part.len() < 2 {
            return Err(());
        }
        let second_part = match parts.next() {
            Some(sp) => sp,
            None => {
                /*
                Every valid NSAP string has a second part... except per ITU-T Rec.
                X.213, section A.7, which handles a zero-length DSP.

                Note: there seems to be an error in RFC 1278 in making the
                second <hexstring> non-optional, since this is optional in X.213.
                X.213 also says that the first byte of the <idp> may be hex,
                which RFC 1278 does not permit.
                */
                if !first_part[2..].as_bytes().iter().all(|b| b.is_ascii_digit()) {
                    return Err(());
                }
                let afi = decode_afi_from_str(&first_part[0..2])?;
                // If the schema is not known, we cannot construct an NSAP,
                // because we don't know how long the IDI is.
                let schema = get_address_type_info(afi).ok_or(())?;
                let idi_len_digits = get_idi_len_in_digits(schema.network_type).ok_or(())?;
                // FIXME: IDI length in bytes is calculated incorrectly throughout this library.
                let idi_len_bytes = (idi_len_digits >> 1) + 1; // +1 for odd len
                let mut out: Vec<u8> = Vec::with_capacity(1 + idi_len_bytes);
                out.push(afi);
                write_bcd(&mut out, first_part[2..].as_bytes(), idi_len_digits);
                // let mut out_byte: u8 = 0;
                // for (i, digit) in first_part[2..].as_bytes().iter().enumerate() {
                //     if (i % 2) > 0 { // On least significant nybble
                //         out_byte |= *digit - 0x30;
                //         out.push(out_byte);
                //         out_byte = 0;
                //     } else { // On most significant nybble
                //         out_byte |= (*digit - 0x30) << 4;
                //     }
                // }
                // if (idi_len_digits % 2) > 0 {
                //     out_byte |= 0x0F;
                //     out.push(out_byte);
                // }
                debug_assert_eq!(out.len(), 1 + idi_len_bytes);
                return Ok(X213NetworkAddress { octets: Cow::Owned(out) });
            },
        };
        if first_part == "NS" {
            let hexstr = parts.next().ok_or(())?;
            let hexbytes = hex::decode(second_part).map_err(|_| ())?;
            return Ok(X213NetworkAddress { octets: Cow::Owned(hexbytes) });
        }
        let third_part = parts.next();
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
            // This is valid <afi> "+" <idi> [ "+" <dsp> ] syntax.
        }
        // Otherwise, assume it is <idp> "+" <hexstring>
        // FIXME: The AFI can contain hex digits too
        if !first_part[2..].as_bytes().iter().all(|b| b.is_ascii_digit()) || first_part.len() < 2 {
            return Err(());
        }
        let afi = decode_afi_from_str(&s[0..2])?;
        let addr_type_info = get_address_type_info(afi);
        let idp = &first_part.as_bytes()[2..];
        let mut idp_bytes: Vec<u8> = Vec::with_capacity(idp.len() >> 1);
        for digit in idp {

        }
        Err(())
    }

}

// TODO: FromStr
// TODO: PartialEq, Eq
// TODO: Hash

#[cfg(test)]
mod tests {

    extern crate alloc;
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

}
