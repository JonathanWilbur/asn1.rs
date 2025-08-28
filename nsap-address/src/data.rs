//! Constants, identifiers, codes, data and functions for mapping or lookups
use crate::{AFI, DSPSyntax, X213NetworkAddressType};

/// The AFI is mandatory. The IDI may be zero bytes (in the case of Local IDI),
/// and the DSP (presumably) MUST be present.
pub const SMALLEST_VALID_NSAP_ADDR: usize = 2;

/// The URL AFI defined in ITU-T Recommendation X.519 (2019).
///
/// The IDI has a fixed length of four digits. The DSP encodes a URL in an
/// unspecified encoding (presumably UTF-8).
pub const AFI_URL: u8 = 0xFF;
/// AFI for an X.121 Address (used for X.25), decimal, leading non-zero digit
pub const AFI_X121_DEC_LEADING_NON_ZERO: u8 = 0x36;
/// AFI for an X.121 Address (used for X.25), decimal, leading zero digit
pub const AFI_X121_DEC_LEADING_ZERO: u8 = 0x52;
/// AFI for an X.121 Address (used for X.25), binary, leading non-zero digit
pub const AFI_X121_BIN_LEADING_NON_ZERO: u8 = 0x37;
/// AFI for an X.121 Address (used for X.25), binary, leading zero digit
pub const AFI_X121_BIN_LEADING_ZERO: u8 = 0x53;
/// AFI for ISO Data Country Code (DCC) decimal
pub const AFI_ISO_DCC_DEC: u8 = 0x38;
/// AFI for ISO Data Country Code (DCC) binary
pub const AFI_ISO_DCC_BIN: u8 = 0x39;
/// AFI for F.69 / Telex, decimal, leading non-zero digit
pub const AFI_F69_DEC_LEADING_NON_ZERO: u8 = 0x40;
/// AFI for F.69 / Telex, decimal, leading zero digit
pub const AFI_F69_DEC_LEADING_ZERO: u8 = 0x54;
/// AFI for F.69 / Telex, binary, leading non-zero digit
pub const AFI_F69_BIN_LEADING_NON_ZERO: u8 = 0x41;
/// AFI for F.69 / Telex, binary, leading zero digit
pub const AFI_F69_BIN_LEADING_ZERO: u8 = 0x55;
/// AFI for E.163 number (used in PSTN), decimal, leading non-zero digit
pub const AFI_E163_DEC_LEADING_NON_ZERO: u8 = 0x42;
/// AFI for E.163 number (used in PSTN), decimal, leading zero digit
pub const AFI_E163_DEC_LEADING_ZERO: u8 = 0x56;
/// AFI for E.163 number (used in PSTN), binary, leading non-zero digit
pub const AFI_E163_BIN_LEADING_NON_ZERO: u8 = 0x43;
/// AFI for E.163 number (used in PSTN), binary, leading zero digit
pub const AFI_E163_BIN_LEADING_ZERO: u8 = 0x57;
/// AFI for E.164 number (used in ISDN), decimal, leading non-zero digit
pub const AFI_E164_DEC_LEADING_NON_ZERO: u8 = 0x44;
/// AFI for E.164 number (used in ISDN), decimal, leading zero digit
pub const AFI_E164_DEC_LEADING_ZERO: u8 = 0x58;
/// AFI for E.164 number (used in ISDN), binary, leading non-zero digit
pub const AFI_E164_BIN_LEADING_NON_ZERO: u8 = 0x45;
/// AFI for E.164 number (used in ISDN), binary, leading zero digit
pub const AFI_E164_BIN_LEADING_ZERO: u8 = 0x59;
/// AFI for ISO/IEC 6523 International Code Designator (ICD), decimal
pub const AFI_ISO_6523_ICD_DEC: u8 = 0x46;
/// AFI for ISO/IEC 6523 International Code Designator (ICD), binary
pub const AFI_ISO_6523_ICD_BIN: u8 = 0x47;

/// AFI for IANA Internet Code Point (ICP), decimal, per IETF RFFC 4548
///
/// Quoting IETF RFC 4548:
///
/// > One of these two AFIs ('34') is
/// > allocated for assignment of NSAPA in Decimal Numeric Format.  This
/// > document does not address allocation for this AFI as it is not clear
/// > what use (if any) can be made of this encoding format at this time.
pub const AFI_IANA_ICP_DEC: u8 = 0x34;

/// AFI for IANA Internet Code Point (ICP), binary, per IETF RFFC 4548
pub const AFI_IANA_ICP_BIN: u8 = 0x35;

/// AFI for ITU Rec. E.191.1 International Network Designator (IDN), decimal
pub const AFI_ITU_T_IND_DEC: u8 = 0x76;
/// AFI for ITU Rec. E.191.1 International Network Designator (IDN), decimal
pub const AFI_ITU_T_IND_BIN: u8 = 0x77;
/// Local AFI, decimal
pub const AFI_LOCAL_DEC: u8 = 0x48;
/// Local AFI, binary
pub const AFI_LOCAL_BIN: u8 = 0x49;
/// Local AFI, ISO/IEC 646 (ASCII or ASCII-like)
pub const AFI_LOCAL_ISO_IEC_646: u8 = 0x50;
/// Local AFI, characters from a national character set
pub const AFI_LOCAL_NATIONAL: u8 = 0x51;

/// Group AFI for [AFI_X121_DEC_LEADING_NON_ZERO]
pub const GROUP_AFI_X121_DEC_LEADING_NON_ZERO: u8 = 0xBA;
/// Group AFI for [AFI_X121_DEC_LEADING_ZERO]
pub const GROUP_AFI_X121_DEC_LEADING_ZERO: u8 = 0xCA;
/// Group AFI for [AFI_X121_BIN_LEADING_NON_ZERO]
pub const GROUP_AFI_X121_BIN_LEADING_NON_ZERO: u8 = 0xBB;
/// Group AFI for [AFI_X121_BIN_LEADING_ZERO]
pub const GROUP_AFI_X121_BIN_LEADING_ZERO: u8 = 0xCB;
/// Group AFI for [AFI_ISO_DCC_DEC]
pub const GROUP_AFI_ISO_DCC_DEC: u8 = 0xBC;
/// Group AFI for [AFI_ISO_DCC_BIN]
pub const GROUP_AFI_ISO_DCC_BIN: u8 = 0xBD;
/// Group AFI for [AFI_F69_DEC_LEADING_NON_ZERO]
pub const GROUP_AFI_F69_DEC_LEADING_NON_ZERO: u8 = 0xBE;
/// Group AFI for [AFI_F69_DEC_LEADING_ZERO]
pub const GROUP_AFI_F69_DEC_LEADING_ZERO: u8 = 0xCC;
/// Group AFI for [AFI_F69_BIN_LEADING_NON_ZERO]
pub const GROUP_AFI_F69_BIN_LEADING_NON_ZERO: u8 = 0xBF;
/// Group AFI for [AFI_F69_BIN_LEADING_ZERO]
pub const GROUP_AFI_F69_BIN_LEADING_ZERO: u8 = 0xCD;
/// Group AFI for [AFI_E163_DEC_LEADING_NON_ZERO]
pub const GROUP_AFI_E163_DEC_LEADING_NON_ZERO: u8 = 0xC0;
/// Group AFI for [AFI_E163_DEC_LEADING_ZERO]
pub const GROUP_AFI_E163_DEC_LEADING_ZERO: u8 = 0xCE;
/// Group AFI for [AFI_E163_BIN_LEADING_NON_ZERO]
pub const GROUP_AFI_E163_BIN_LEADING_NON_ZERO: u8 = 0xC1;
/// Group AFI for [AFI_E163_BIN_LEADING_ZERO]
pub const GROUP_AFI_E163_BIN_LEADING_ZERO: u8 = 0xCF;
/// Group AFI for [AFI_E164_DEC_LEADING_NON_ZERO]
pub const GROUP_AFI_E164_DEC_LEADING_NON_ZERO: u8 = 0xC2;
/// Group AFI for [AFI_E164_DEC_LEADING_ZERO]
pub const GROUP_AFI_E164_DEC_LEADING_ZERO: u8 = 0xD0;
/// Group AFI for [AFI_E164_BIN_LEADING_NON_ZERO]
pub const GROUP_AFI_E164_BIN_LEADING_NON_ZERO: u8 = 0xC3;
/// Group AFI for [AFI_E164_BIN_LEADING_ZERO]
pub const GROUP_AFI_E164_BIN_LEADING_ZERO: u8 = 0xD1;
/// Group AFI for [AFI_ISO_6523_ICD_DEC]
pub const GROUP_AFI_ISO_6523_ICD_DEC: u8 = 0xC4;
/// Group AFI for [AFI_ISO_6523_ICD_BIN]
pub const GROUP_AFI_ISO_6523_ICD_BIN: u8 = 0xC5;
/// Group AFI for [AFI_IANA_ICP_DEC]
pub const GROUP_AFI_IANA_ICP_DEC: u8 = 0xB8;
/// Group AFI for [AFI_IANA_ICP_BIN]
pub const GROUP_AFI_IANA_ICP_BIN: u8 = 0xB9;
/// Group AFI for [AFI_ITU_T_IND_DEC]
pub const GROUP_AFI_ITU_T_IND_DEC: u8 = 0xE2;
/// Group AFI for [AFI_ITU_T_IND_BIN]
pub const GROUP_AFI_ITU_T_IND_BIN: u8 = 0xE3;
/// Group AFI for [AFI_LOCAL_DEC]
pub const GROUP_AFI_LOCAL_DEC: u8 = 0xC6;
/// Group AFI for [AFI_LOCAL_BIN]
pub const GROUP_AFI_LOCAL_BIN: u8 = 0xC7;
/// Group AFI for [AFI_LOCAL_ISO_IEC_646]
pub const GROUP_AFI_LOCAL_ISO_IEC_646: u8 = 0xC8;
/// Group AFI for [AFI_LOCAL_NATIONAL]
pub const GROUP_AFI_LOCAL_NATIONAL: u8 = 0xC9;

/// Maximum decimal DSP length in digits for X.121 / X.25 addressing
pub const MAX_DEC_DSP_LEN_DIGITS_X121: u8 = 24;
/// Maximum decimal DSP length in digits for ISO DCC
pub const MAX_DEC_DSP_LEN_DIGITS_ISO_DCC: u8 = 35;
/// Maximum decimal DSP length in digits for F.69 addressing / Telex
pub const MAX_DEC_DSP_LEN_DIGITS_F69: u8 = 30;
/// Maximum decimal DSP length in digits for E.163 / PSTN addressing
pub const MAX_DEC_DSP_LEN_DIGITS_E163: u8 = 26;
/// Maximum decimal DSP length in digits for E.164 / ISDN addressing
pub const MAX_DEC_DSP_LEN_DIGITS_E164: u8 = 23;
/// Maximum decimal DSP length in digits for ISO/IEC 6523 ICD addressing
pub const MAX_DEC_DSP_LEN_DIGITS_ISO_6523_ICD: u8 = 34;
/// Maximum decimal DSP length in digits for IANA ICP (IPv4 or IPv6) addressing
pub const MAX_DEC_DSP_LEN_DIGITS_IANA_ICP: u8 = 34;
/// Maximum decimal DSP length in digits for ITU-T IND addressing
pub const MAX_DEC_DSP_LEN_DIGITS_ITU_T_IND: u8 = 32;
/// Maximum decimal DSP length in digits for local addressing
pub const MAX_DEC_DSP_LEN_DIGITS_LOCAL: u8 = 38;

/// Maximum binary DSP length in bytes for X.121 / X.25 addressing
pub const MAX_BIN_DSP_LEN_X121: u8 = 12;
/// Maximum binary DSP length in bytes for ISO DCC
pub const MAX_BIN_DSP_LEN_ISO_DCC: u8 = 17;
/// Maximum binary DSP length in bytes for F.69 addressing / Telex
pub const MAX_BIN_DSP_LEN_F69: u8 = 15;
/// Maximum binary DSP length in bytes for E.163 / PSTN addressing
pub const MAX_BIN_DSP_LEN_E163: u8 = 13;
/// Maximum binary DSP length in bytes for E.164 / ISDN addressing
pub const MAX_BIN_DSP_LEN_E164: u8 = 11;
/// Maximum binary DSP length in bytes for ISO/IEC 6523 ICD addressing
pub const MAX_BIN_DSP_LEN_ISO_6523_ICD: u8 = 17;
/// Maximum binary DSP length in bytes for IANA ICP (IPv4 or IPv6) addressing
pub const MAX_BIN_DSP_LEN_IANA_ICP: u8 = 17;
/// Maximum binary DSP length in bytes for ITU-T IND addressing
pub const MAX_BIN_DSP_LEN_ITU_T_IND: u8 = 16;
/// Maximum binary DSP length in bytes for local addressing
pub const MAX_BIN_DSP_LEN_LOCAL: u8 = 19;

/// Maximum ISO/IEC 646-encoded DSP length in bytes for local addressing
pub const MAX_ISO_IEC_646_LEN_LOCAL: u8 = 19;
/// Maximum national character-encoded DSP length in bytes for local addressing
pub const MAX_NATIONAL_CHAR_LEN_LOCAL: u8 = 9;

/// Maximum IDI length in digits for X.121 / X.25 addressing
pub const MAX_IDI_LEN_DIGITS_X121: u8 = 14; // Up to
/// Maximum IDI length in digits for ISO DCC
pub const MAX_IDI_LEN_DIGITS_ISO_DCC: u8 = 3; // Exactly
/// Maximum IDI length in digits for F.69 addressing / Telex
pub const MAX_IDI_LEN_DIGITS_F69: u8 = 8; // Up to
/// Maximum IDI length in digits for E.163 / PSTN addressing
pub const MAX_IDI_LEN_DIGITS_E163: u8 = 12; // Up to
/// Maximum IDI length in digits for E.164 / ISDN addressing
pub const MAX_IDI_LEN_DIGITS_E164: u8 = 15; // Up to
/// Maximum IDI length in digits for ISO/IEC 6523 ICD addressing
pub const MAX_IDI_LEN_DIGITS_ISO_6523_ICD: u8 = 4; // Exactly
/// Maximum IDI length in digits for IANA ICP (IPv4 or IPv6) addressing
pub const MAX_IDI_LEN_DIGITS_IANA_ICP: u8 = 4; // Exactly
/// Maximum IDI length in digits for ITU-T IND addressing
pub const MAX_IDI_LEN_DIGITS_ITU_T_IND: u8 = 6; // Exactly
/// Maximum IDI length in digits for local addressing
pub const MAX_IDI_LEN_DIGITS_LOCAL: u8 = 0; // Exactly
/// Maximum IDI length in digits for ITU-T Rec. X.519 URL NSAPs
pub const MAX_IDI_LEN_DIGITS_URL: u8 = 4; // Exactly.

// DSP Prefixes that start with 0x54, 0x00, 0x72, 0x87, 0x22,

/// IETF RFC 1277 well-known network: International X.25
pub const RFC_1277_WELL_KNOWN_NETWORK_INTL_X25: u8 = 0x01;
/// IETF RFC 1277 well-known network: JANET
///
/// See: <https://en.wikipedia.org/wiki/JANET>
pub const RFC_1277_WELL_KNOWN_NETWORK_JANET: u8 = 0x02;
/// IETF RFC 1277 well-known network: DARPA/NSF Internet (The internet)
pub const RFC_1277_WELL_KNOWN_NETWORK_DARPA_NSF_INTERNET: u8 = 0x03;
/// IETF RFC 1277 well-known network: IXI
///
/// See: <https://cordis.europa.eu/project/id/2718>
pub const RFC_1277_WELL_KNOWN_NETWORK_IXI: u8 = 0x06;
/// ITU-T Rec. X.519 DSP prefix for LDAP
pub const ITU_X519_DSP_PREFIX_LDAP: u8 = 0x11;
/// ITU-T Rec. X.519 DSP prefix for IDM over IPv4
///
/// See: <https://www.itu.int/rec/T-REC-X.519/en>
pub const ITU_X519_DSP_PREFIX_IDM_OVER_IPV4: u8 = 0x10;
/// ITU-T Rec. X.519 DSP prefix for ISO Transport over TCP (ITOT) over IPv4
///
/// See: <https://datatracker.ietf.org/doc/rfc2126/>
pub const ITU_X519_DSP_PREFIX_ITOT_OVER_IPV4: u8 = RFC_1277_WELL_KNOWN_NETWORK_DARPA_NSF_INTERNET;

/// Default TCP port for ISO Transport over TCP (ITOT) per IETF RFC 2126
///
/// See: <https://datatracker.ietf.org/doc/rfc2126/>
pub const ITOT_OVER_IPV4_DEFAULT_PORT: u16 = 102;

/// AFI string for X.121 addressing, per IETF RFC 1278
pub const AFI_STR_X121: &str = "X121";
/// AFI string for ISO DCC addressing, per IETF RFC 1278
pub const AFI_STR_DCC: &str = "DCC";
/// AFI string for Telex / F.69 addressing, per IETF RFC 1278
pub const AFI_STR_TELEX: &str = "TELEX";
/// AFI string for PSTN / E.163 addressing, per IETF RFC 1278
pub const AFI_STR_PSTN: &str = "PSTN";
/// AFI string for ISDN / E.164 addressing, per IETF RFC 1278
pub const AFI_STR_ISDN: &str = "ISDN";
/// AFI string for ISO/IEC 6523 ICD addressing, per IETF RFC 1278
pub const AFI_STR_ICD: &str = "ICD";
/// Not-standard AFI string for IANA ICP addressing
pub const AFI_STR_ICP: &str = "ICP";
/// Not-standard AFI string for ITU-T IND addressing
pub const AFI_STR_IND: &str = "IND";
/// AFI string for local addressing, per IETF RFC 1278
pub const AFI_STR_LOCAL: &str = "LOCAL";
/// Not-standard AFI string for ITU-T Rec. X.519 URL-based addressing
pub const AFI_STR_URL: &str = "URL";

// TODO: Review if these are used.
/// IETF RFC 1277 Telex / F.69 number for non-OSI networks
pub const IETF_RFC_1277_TELEX_NUMBER_STR: &str = "00728722";
/// IETF RFC 1278 DSP string for ISO Transport over TCP (ITOT)
pub const IETF_RFC_1006_PREFIX_STR: &str = "RFC-1006";
/// IETF RFC 1278 DSP string for X.25
pub const X25_PREFIX_STR: &str = "X.25(80)";
/// IETF RFC 1278 DSP string for ECMA 117 binary syntax
pub const ECMA_117_BINARY_STR: &str = "ECMA-117-Binary";
/// IETF RFC 1278 DSP string for ECMA 117 decimal syntax
pub const ECMA_117_DECIMAL_STR: &str = "ECMA-117-Decimal";

/// Non-standard string for NSAP-encoded IPv4 addresses
pub const IPV4_STR: &str = "IP4";
/// Non-standard string for NSAP-encoded IPv6 addresses
pub const IPV6_STR: &str = "IP6";

/// IANA-allocated Internet Code Point for IPv4 per IETF RFC 4548
///
/// See: <https://www.rfc-editor.org/rfc/rfc4548.html>
pub const IANA_ICP_IDI_IPV4: [u8; 2] = [0, 1];
/// IANA-allocated Internet Code Point for IPv6 per IETF RFC 4548
///
/// See: <https://www.rfc-editor.org/rfc/rfc4548.html>
pub const IANA_ICP_IDI_IPV6: [u8; 2] = [0, 0];

/// IETF RFC 1277 NSAP prefix for non-OSI addressing
pub const RFC_1277_PREFIX: [u8; 5] = [
    AFI_F69_DEC_LEADING_ZERO, // AFI
    0x00,
    0x72,
    0x87,
    0x22, // IDI
];

// FIXME: Get rid of this
/// This is exported for convenience, since the Internet is most likely to be
/// used in NSAPs now. If an application only wants / can use Internet NSAPs,
/// the NSAPs could be checked to see if they begin with this sequence.
pub const INTERNET_PREFIX: [u8; 6] = [
    AFI_F69_DEC_LEADING_ZERO, // AFI
    0x00,
    0x72,
    0x87,
    0x22, // IDI
    0x03, // The DSP prefix "03"
];

/// Maps group AFIs to individual ones per Table A.2 in ITU-T Rec. X.213
pub(crate) const fn group_afi_to_individual_afi(afi: AFI) -> AFI {
    match afi {
        0xA1 => 0x11,
        0xA2 => 0x12,
        0xA3 => 0x13,
        0xA4 => 0x14,
        0xA5 => 0x15,
        0xA6 => 0x16,
        0xA7 => 0x17,
        0xA8 => 0x18,
        0xA9 => 0x19,
        0xAB => 0x21,
        0xAC => 0x22,
        0xAD => 0x23,
        0xAE => 0x24,
        0xAF => 0x25,
        0xB0 => 0x26,
        0xB1 => 0x27,
        0xB2 => 0x28,
        0xB3 => 0x29,
        0xB4 => 0x30,
        0xB5 => 0x31,
        0xB6 => 0x32,
        0xB7 => 0x33,
        0xB8 => 0x34,
        0xB9 => 0x35,
        0xBA => 0x36,
        0xBB => 0x37,
        0xBC => 0x38,
        0xBD => 0x39,
        0xBE => 0x40,
        0xBF => 0x41,
        0xC0 => 0x42,
        0xC1 => 0x43,
        0xC2 => 0x44,
        0xC3 => 0x45,
        0xC4 => 0x46,
        0xC5 => 0x47,
        0xC6 => 0x48,
        0xC7 => 0x49,
        0xC8 => 0x50,
        0xC9 => 0x51,
        0xCA => 0x52,
        0xCB => 0x53,
        0xCC => 0x54,
        0xCD => 0x55,
        0xCE => 0x56,
        0xCF => 0x57,
        0xD0 => 0x58,
        0xD1 => 0x59,
        0xD2 => 0x60,
        0xD3 => 0x61,
        0xD4 => 0x62,
        0xD5 => 0x63,
        0xD6 => 0x64,
        0xD7 => 0x65,
        0xD8 => 0x66,
        0xD9 => 0x67,
        0xDA => 0x68,
        0xDB => 0x69,
        0xDC => 0x70,
        0xDD => 0x71,
        0xDE => 0x72,
        0xDF => 0x73,
        0xE0 => 0x74,
        0xE1 => 0x75,
        0xE2 => 0x76,
        0xE3 => 0x77,
        0xE4 => 0x78,
        0xE5 => 0x79,
        0xE6 => 0x80,
        0xE7 => 0x81,
        0xE8 => 0x82,
        0xE9 => 0x83,
        0xEA => 0x84,
        0xEB => 0x85,
        0xEC => 0x86,
        0xED => 0x87,
        0xEE => 0x88,
        0xEF => 0x89,
        0xF0 => 0x90,
        0xF1 => 0x91,
        0xF2 => 0x92,
        0xF3 => 0x93,
        0xF4 => 0x94,
        0xF5 => 0x95,
        0xF6 => 0x96,
        0xF7 => 0x97,
        0xF8 => 0x98,
        0xF9 => 0x99,
        _ => afi,
    }
}

/// information about a particular NSAP syntax and what network it addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X213NetworkAddressInfo {
    /// The network type for this AFI
    pub network_type: X213NetworkAddressType,
    /// Whether there are leading zeroes in the IDI for this AFI
    pub leading_zeroes_in_idi: bool,
    /// The Domain Specific Part (DSP) syntax
    pub dsp_syntax: DSPSyntax,
    /// The maximum length of the IDI in decimal digits
    pub max_idi_len_digits: u8,
    /// Whether the IDI can ever be shorter than `max_idi_len_digits`
    pub idi_len_exact: bool,
}

/// Table of address info about NSAP syntaxes by AFI.
///
/// This table is biased by -34, because the first 34 AFIs are not defined.
///
/// Quoting X.213:
/// "The numerically greater AFI value is used when the first significant digit
/// in the IDI is zero."
const AFI_INFO: [Option<X213NetworkAddressInfo>; 45] = [
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::IANA_ICP,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_IANA_ICP,
        idi_len_exact: true,
    }), // 34
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::IANA_ICP,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_IANA_ICP,
        idi_len_exact: true,
    }), // 35
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_X121,
        idi_len_exact: false,
    }), // 36
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_X121,
        idi_len_exact: false,
    }), // 37
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::ISO_DCC,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_ISO_DCC,
        idi_len_exact: true,
    }), // 38
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::ISO_DCC,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_ISO_DCC,
        idi_len_exact: true,
    }), // 39
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_F69,
        idi_len_exact: false,
    }), // 40
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_F69,
        idi_len_exact: false,
    }), // 41
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E163,
        idi_len_exact: false,
    }), // 42
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E163,
        idi_len_exact: false,
    }), // 43
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E164,
        idi_len_exact: false,
    }), // 44
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E164,
        idi_len_exact: false,
    }), // 45
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::ISO_6523_ICD,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_ISO_6523_ICD,
        idi_len_exact: true,
    }), // 46
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::ISO_6523_ICD,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_ISO_6523_ICD,
        idi_len_exact: true,
    }), // 47
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::LOCAL,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_LOCAL,
        idi_len_exact: true,
    }), // 48
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::LOCAL,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_LOCAL,
        idi_len_exact: true,
    }), // 49
    None, // 50
    None, // 51
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_X121,
        idi_len_exact: false,
    }), // 52
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_X121,
        idi_len_exact: false,
    }), // 53
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_F69,
        idi_len_exact: false,
    }), // 54
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_F69,
        idi_len_exact: false,
    }), // 55
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E163,
        idi_len_exact: false,
    }), // 56
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E163,
        idi_len_exact: false,
    }), // 57
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E164,
        idi_len_exact: false,
    }), // 58
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_E164,
        idi_len_exact: false,
    }), // 59
    None, // 60
    None, // 61
    None, // 62
    None, // 63
    None, // 64
    None, // 65
    None, // 66
    None, // 67
    None, // 68
    None, // 69
    None, // 70
    None, // 71
    None, // 72
    None, // 73
    None, // 74
    None, // 75
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::ITU_T_IND,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_ITU_T_IND,
        idi_len_exact: true,
    }), // 76
    Some(X213NetworkAddressInfo {
        network_type: X213NetworkAddressType::ITU_T_IND,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
        max_idi_len_digits: MAX_IDI_LEN_DIGITS_ITU_T_IND,
        idi_len_exact: true,
    }), // 77
    None, // 78
];

// TODO: Rename to get_nsap_schema()
/// Get information about the NSAP syntax and network type by AFI
///
/// Returns `None` if the AFI is unrecognized.
pub const fn get_address_type_info(afi: AFI) -> Option<X213NetworkAddressInfo> {
    if afi == 0xFF {
        return Some(X213NetworkAddressInfo {
            network_type: X213NetworkAddressType::URL,
            leading_zeroes_in_idi: false,
            dsp_syntax: DSPSyntax::Binary,
            max_idi_len_digits: MAX_IDI_LEN_DIGITS_URL,
            idi_len_exact: true,
        });
    }
    let normalized = group_afi_to_individual_afi(afi);
    // Conversion of the BCD to a true binary value.
    let afi_bin: u8 = (((normalized & 0xF0) >> 4) * 10) + (normalized & 0x0F);
    if afi_bin < 34 || afi_bin > 77 {
        return None;
    }
    // The first 34 are undefined
    let afi_bin = afi_bin - 34;
    AFI_INFO[afi_bin as usize]
}

/// Return get the N-address network type from the AFI
#[inline]
pub const fn afi_to_network_type(afi: AFI) -> Option<X213NetworkAddressType> {
    match get_address_type_info(afi) {
        Some(info) => Some(info.network_type),
        None => None,
    }
}

/// Returns `true` if an AFI is an individual AFI
pub const fn is_individual_afi(afi: AFI) -> bool {
    let individual = group_afi_to_individual_afi(afi);
    afi == individual
}

/// Returns `true` if an AFI is a group AFI
pub const fn is_group_afi(afi: AFI) -> bool {
    let individual = group_afi_to_individual_afi(afi);
    afi != individual
}

#[cfg(test)]
mod tests {

    use super::get_address_type_info;

    // This test is really just to make sure we don't panic.
    #[test]
    fn test_get_address_type_info() {
        for i in 0..0xFFu8 {
            let _ = get_address_type_info(i);
        }
    }
}
