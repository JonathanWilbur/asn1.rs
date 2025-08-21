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
// use core::slice::
// use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
// use core::str::FromStr;

// TODO: Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
// TODO: Is there a separate ATN addressing? It sounds like ATN uses ISO 6523 (ICD)

pub type AFI = u8;

/// The AFI is mandatory. The IDI may be zero bytes (in the case of Local IDI),
/// and the DSP (presumably) MUST be present.
pub const SMALLEST_VALID_NSAP_ADDR: usize = 2;

pub const AFI_URL: u8 = 0xFF;
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
pub const AFI_STR_PSTN: &str = "PSTN";
pub const AFI_STR_ICD: &str = "ICD";
pub const AFI_STR_ICP: &str = "ICP";
pub const AFI_STR_IND: &str = "IND";
pub const AFI_STR_LOCAL: &str = "LOCAL";
pub const AFI_STR_URL: &str = "URL";

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
        X213NetworkAddressType::E164 => Some(AFI_STR_PSTN),
        X213NetworkAddressType::ISO_6523_ICD => Some(AFI_STR_ICD),
        X213NetworkAddressType::IANA_ICP => Some(AFI_STR_ICP), // Not specified in IETF RFC 1278. See: https://www.iana.org/assignments/osi-nsapa-numbers/osi-nsapa-numbers.xhtml
        X213NetworkAddressType::ITU_T_IND => Some(AFI_STR_IND), // Not specified in IETF RFC 1278.
        X213NetworkAddressType::LOCAL => Some(AFI_STR_LOCAL),
        X213NetworkAddressType::URL => Some(AFI_STR_URL), // Not specified in IETF RFC 1278.
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
pub const fn naddr_network_type_to_max_idi_length (nt: X213NetworkAddressType) -> Option<usize> {
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
pub struct IDIDigitsIter<'a> {
    idi: &'a [u8],
    least_sig_nybble: bool,
    leading_0_sig: bool,
    processing_leading_digits: bool,
    ignore_last_nybble: bool,
}

impl <'a> IDIDigitsIter<'a> {

    pub fn new(idi: &'a [u8], leading_0_sig: bool, ignore_last_nybble: bool) -> IDIDigitsIter<'a> {
        IDIDigitsIter{
            idi,
            leading_0_sig,
            ignore_last_nybble,
            processing_leading_digits: true, // Start off handling leading digits
            least_sig_nybble: false, // Start off on the MSn
        }
    }

}

/// This SHOULD BE an ASCII digit, but might not be. It is on the caller to
/// check this and determine what to do if this has a non-digit value.
pub type ShouldBeASCIIDigit = u8;

impl <'a> Iterator for IDIDigitsIter<'a> {
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

impl <'a> FusedIterator for IDIDigitsIter<'a> {}

#[derive(Debug)]
pub struct X213NetworkAddress <'a> {
    pub octets: &'a [u8],
}

impl <'a> X213NetworkAddress <'a> {

    #[inline]
    pub const fn afi(&self) -> u8 {
        if self.octets.len() > 0 {
            self.octets[0]
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

    pub fn idi_digits(&'a self) -> IDIDigitsIter<'a> {
        let addr_type_info = get_address_type_info(self.afi())
            .unwrap_or(X213NetworkAddressInfo{
                network_type: X213NetworkAddressType::LOCAL, // This doesn't matter.
                leading_zeroes_in_idi: false,
                dsp_syntax: DSPSyntax::Binary,
            });
        let leading_0_sig = addr_type_info.leading_zeroes_in_idi;
        let is_dsp_decimal = matches!(addr_type_info.dsp_syntax, DSPSyntax::Decimal);
        // FIXME: You can handle unknown IDIs once you merge the slices into a single slice.
        let idi_len = naddr_network_type_to_max_idi_length(addr_type_info.network_type).unwrap();
        let odd_len_idi: bool = (idi_len % 2) > 0;
        let last_idi_digit_is_padding: bool = is_dsp_decimal && odd_len_idi;
        let idi = &self.octets[1..1+idi_len];
        IDIDigitsIter::new(idi, leading_0_sig, last_idi_digit_is_padding)
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
        Ok(X213NetworkAddress { octets })
    }

}

impl <'a> Display for X213NetworkAddress<'a> {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let info = match self.get_network_type_info() {
            Some(i) => i,
            None => { // If unrecognized, just print in NS+<hex> format
                let h = hex::encode(self.octets);
                f.write_str("NS+")?;
                return f.write_str(h.as_str());
            }
        };
        match naddr_network_type_to_str(info.network_type) {
            Some(s) => f.write_str(s)?,
            None => write!(f, "{:02X}", self.afi())?,
        };
        f.write_char('+')?;
        for digit in self.idi_digits() {
            f.write_char((digit + 0x30) as char)?;
        }
        f.write_char('+')?;

        // TODO:
        Ok(())

            // DomainSpecificPart::Decimal(digits) => {
            //     f.write_char('d')?;
            //     for digit in digits {
            //         f.write_char((*digit + 0x30).into())?;
            //     }
            //     Ok(())
            // },
            // DomainSpecificPart::Binary(bytes) => {
            //     f.write_char('x')?;
            //     for byte in bytes {
            //         f.write_fmt(format_args!("{:02X}", *byte))?;
            //     }
            //     Ok(())
            // },
            // DomainSpecificPart::IsoIec646(chars) => {
            //     f.write_char('l')?;
            //     for c in chars {
            //         f.write_char((*c).into())?;
            //     }
            //     Ok(())
            // },
            // DomainSpecificPart::NationalCharacters(chars) => {
            //     let printable = chars.iter().all(|c| c.is_ascii_graphic() || *c == b' ');
            //     // If the national characters happen to overlap with ASCII
            //     // enough, just try to print ASCII; otherwise, print hex.
            //     if printable {
            //         f.write_char('l')?;
            //         for c in chars {
            //             f.write_char((*c).into())?;
            //         }
            //     } else {
            //         f.write_char('x')?;
            //         for c in chars {
            //             f.write_fmt(format_args!("{:02X}", *c))?;
            //         }
            //     }
            //     Ok(())
            // },
            // DomainSpecificPart::Url(url) => f.write_str(&url),
            // DomainSpecificPart::IpAddress(ip) => f.write_str(&ip.to_string()),

    }

}

// TODO: Display
// TODO: FromStr
// TODO: PartialEq, Eq
// TODO: Hash
