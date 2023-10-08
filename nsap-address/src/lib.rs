#![allow(non_camel_case_types)]
#![allow(unused_parens)]
use std::{fmt::{Display, Write}, error::Error, result::Result, net::Ipv4Addr, str::FromStr};

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

// These are denominated in digits. Subtract two for the AFI to get IDI length.
pub const MAX_IDI_LEN_X121: u8 = 14; // Up to
pub const MAX_IDI_LEN_ISO_DCC: u8 = 3; // Exactly
pub const MAX_IDI_LEN_F69: u8 = 8; // Up to
pub const MAX_IDI_LEN_E163: u8 = 12; // Up to
pub const MAX_IDI_LEN_E164: u8 = 15; // Up to
pub const MAX_IDI_LEN_ISO_6523_ICD: u8 = 4; // Exactly
pub const MAX_IDI_LEN_IANA_ICP: u8 = 4; // Exactly
pub const MAX_IDI_LEN_ITU_T_IND: u8 = 6; // Exactly
pub const MAX_IDI_LEN_LOCAL: u8 = 0; // Exactly
pub const MAX_IDI_LEN_URL: u8 = 2; // Exactly.

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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct X213NetworkAddress {
    pub network_type: X213NetworkAddressType,
    pub group: bool,
    pub idi: Vec<u8>, // FIXME: This is supposed to be ASCII digits, but some parts of the code below disagree!
    pub dsp: DomainSpecificPart,
}

impl X213NetworkAddress {

    pub fn new (
        network_type: X213NetworkAddressType,
        group: bool,
        idi: Vec<u8>,
        dsp: DomainSpecificPart,
    ) -> Self {
        X213NetworkAddress {
            network_type,
            group,
            idi,
            dsp,
        }
    }

    pub fn afi (&self) -> Option<u8> {
        let has_leading_zero = self.idi.get(0).is_some_and(|b| *b == 0);
        match (self.network_type, &self.dsp, has_leading_zero) {
            (X213NetworkAddressType::X121, DomainSpecificPart::Binary(_), true) => Some(AFI_X121_BIN_LEADING_ZERO),
            (X213NetworkAddressType::X121, DomainSpecificPart::Binary(_), false) => Some(AFI_X121_BIN_LEADING_NON_ZERO),
            (X213NetworkAddressType::X121, DomainSpecificPart::Decimal(_), true) => Some(AFI_X121_DEC_LEADING_ZERO),
            (X213NetworkAddressType::X121, DomainSpecificPart::Decimal(_), false) => Some(AFI_X121_DEC_LEADING_NON_ZERO),
            (X213NetworkAddressType::ISO_DCC, DomainSpecificPart::Binary(_), _) => Some(AFI_ISO_DCC_BIN),
            (X213NetworkAddressType::ISO_DCC, DomainSpecificPart::Decimal(_), _) => Some(AFI_ISO_DCC_DEC),
            (X213NetworkAddressType::F69, DomainSpecificPart::Binary(_), true) => Some(AFI_F69_BIN_LEADING_ZERO),
            (X213NetworkAddressType::F69, DomainSpecificPart::Binary(_), false) => Some(AFI_F69_BIN_LEADING_NON_ZERO),
            (X213NetworkAddressType::F69, DomainSpecificPart::Decimal(_), true) => Some(AFI_F69_DEC_LEADING_ZERO),
            (X213NetworkAddressType::F69, DomainSpecificPart::Decimal(_), false) => Some(AFI_F69_DEC_LEADING_NON_ZERO),
            (X213NetworkAddressType::E163, DomainSpecificPart::Binary(_), true) => Some(AFI_E163_BIN_LEADING_ZERO),
            (X213NetworkAddressType::E163, DomainSpecificPart::Binary(_), false) => Some(AFI_E163_BIN_LEADING_NON_ZERO),
            (X213NetworkAddressType::E163, DomainSpecificPart::Decimal(_), true) => Some(AFI_E163_DEC_LEADING_ZERO),
            (X213NetworkAddressType::E163, DomainSpecificPart::Decimal(_), false) => Some(AFI_E163_DEC_LEADING_NON_ZERO),
            (X213NetworkAddressType::E164, DomainSpecificPart::Binary(_), true) => Some(AFI_E164_BIN_LEADING_ZERO),
            (X213NetworkAddressType::E164, DomainSpecificPart::Binary(_), false) => Some(AFI_E164_BIN_LEADING_NON_ZERO),
            (X213NetworkAddressType::E164, DomainSpecificPart::Decimal(_), true) => Some(AFI_E164_DEC_LEADING_ZERO),
            (X213NetworkAddressType::E164, DomainSpecificPart::Decimal(_), false) => Some(AFI_E164_DEC_LEADING_NON_ZERO),
            (X213NetworkAddressType::ISO_6523_ICD, DomainSpecificPart::Binary(_), _) => Some(AFI_ISO_6523_ICD_BIN),
            (X213NetworkAddressType::ISO_6523_ICD, DomainSpecificPart::Decimal(_), _) => Some(AFI_ISO_6523_ICD_DEC),
            (X213NetworkAddressType::IANA_ICP, DomainSpecificPart::Binary(_), _) => Some(AFI_IANA_ICP_BIN),
            (X213NetworkAddressType::IANA_ICP, DomainSpecificPart::Decimal(_), _) => Some(AFI_IANA_ICP_DEC),
            (X213NetworkAddressType::ITU_T_IND, DomainSpecificPart::Binary(_), _) => Some(AFI_ITU_T_IND_BIN),
            (X213NetworkAddressType::ITU_T_IND, DomainSpecificPart::Decimal(_), _) => Some(AFI_ITU_T_IND_DEC),
            (X213NetworkAddressType::LOCAL, DomainSpecificPart::Binary(_), _) => Some(AFI_LOCAL_BIN),
            (X213NetworkAddressType::LOCAL, DomainSpecificPart::Decimal(_), _) => Some(AFI_LOCAL_DEC),
            (X213NetworkAddressType::LOCAL, DomainSpecificPart::IsoIec646(_), _) => Some(AFI_LOCAL_ISO_IEC_646),
            (X213NetworkAddressType::LOCAL, DomainSpecificPart::NationalCharacters(_), _) => Some(AFI_LOCAL_NATIONAL),
            _ => None,
        }
    }

    /// NOTE: URL is not treated as "internet" because it could be a TOR URL,
    /// an I2P URL, or anything else.
    pub fn is_internet(&self) -> bool {
        self.network_type == X213NetworkAddressType::F69
        && &self.idi == &INTERNET_PREFIX[1..]
    }

    pub fn to_socket_addr (&self) -> Option<(Ipv4Addr, Option<u16>)> {
        let dsp_bytes = match &self.dsp {
            DomainSpecificPart::Decimal(x) => x,
            _ => return None,
        };
        read_socket_addr_v4(&dsp_bytes)
    }

    pub fn to_url_str (&self) -> Option<String> {
        if self.network_type == X213NetworkAddressType::URL {
            let dsp_bytes = match &self.dsp {
                DomainSpecificPart::Binary(x) => x,
                _ => return None,
            };
            return Some(String::from_utf8(dsp_bytes.clone()).ok()?)
        }
        // If it is not explicitly URL, we can still convert an IP address into a URL.
        if self.network_type != X213NetworkAddressType::F69 {
            return None;
        }
        let dsp_bytes = match &self.dsp {
            DomainSpecificPart::Binary(x) => x,
            DomainSpecificPart::Decimal(x) => x,
            _ => return None,
        };
        if dsp_bytes.len() < 5 { // Ridiculously short URLs eliminated.
            return None;
        }
        if &self.idi != &INTERNET_PREFIX[1..] {
            return None;
        }
        let (ipv4, port) = read_socket_addr_v4(&dsp_bytes)?;
        // NOTE: Even though port 389 is default for LDAP, there is no default
        // defined in ITU-T Rec. X.519, and the specification requires an
        // explicit port for LDAP URLs.
        let default_port: Option<u16> = match dsp_bytes[0] {
            ITU_X519_DSP_PREFIX_ITOT_OVER_IPV4 => Some(ITOT_OVER_IPV4_DEFAULT_PORT),
            _ => None,
        };
        let port = port.or(default_port);
        match dsp_bytes[0] {
            ITU_X519_DSP_PREFIX_LDAP => {
                if port.is_none() {
                    return None;
                }
                Some(format!("ldap://{}:{}", ipv4, port.unwrap()))
            }
            ITU_X519_DSP_PREFIX_IDM_OVER_IPV4 => {
                if port.is_none() {
                    return None;
                }
                Some(format!("idm://{}:{}", ipv4, port.unwrap()))
            },
            ITU_X519_DSP_PREFIX_ITOT_OVER_IPV4 => {
                if port.is_none() {
                    return None;
                }
                Some(format!("itot://{}:{}", ipv4, port.unwrap_or(ITOT_OVER_IPV4_DEFAULT_PORT)))
            },
            _ => None, // Syntax not understood
        }
    }

}

fn dissect_nsap_str (s: &str, prefix_len: usize) -> Result<(&str, &str), NAddressError> {
    let idi_str = s[prefix_len+1..].split("+").next().ok_or(NAddressError::NoIDI)?;
    let dsp_str = &s[prefix_len+1+idi_str.len()+1..];
    let maybe_non_ascii_digit_in_idi = idi_str.as_bytes().iter().find(|b| !b.is_ascii_digit());
    if let Some(non_ascii_digit) = maybe_non_ascii_digit_in_idi {
        return Err(NAddressError::NonDigitInIDI(*non_ascii_digit));
    }
    Ok((idi_str, dsp_str))
}

/// Decodes a DSP according to the first three variants of the `<dsp>`
/// BNF production in [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).
/// (The ones starting with either `d`, `x` or `l`.)
fn decode_opaque_dsp (dsp: &[u8]) -> Result<DomainSpecificPart, NAddressError> {
    if dsp.len() == 0 {
        return Err(NAddressError::MalformedDSP);
    }
    match dsp[0] {
        b'd' => {
            let is_all_ascii_digits = dsp[1..].iter().all(|b| b.is_ascii_digit());
            if !is_all_ascii_digits {
                return Err(NAddressError::MalformedDSP);
            }
            Ok(DomainSpecificPart::Decimal(dsp[1..].iter().map(|b| b - 0x30).collect()))
        },
        b'x' => {
            let bytes = hex::decode(&dsp[1..]).map_err(|_| NAddressError::MalformedDSP)?;
            Ok(DomainSpecificPart::Binary(bytes))
        },
        b'l' => {
            let is_all_ascii = dsp[1..].iter().all(|b| b.is_ascii_graphic());
            if !is_all_ascii {
                return Err(NAddressError::MalformedDSP);
            }
            Ok(DomainSpecificPart::IsoIec646(dsp[1..].to_vec()))
        },
        _ => Err(NAddressError::UnsupportedDSPType),
    }
}

impl FromStr for X213NetworkAddress {
    type Err = NAddressError;

    // NS+a433bb93c1
    // NS+aa3106
    // X121+234219200300
    // TELEX+00728722+X.25(80)+02+00002340555+CUDF+"892796"
    // TELEX+00728722+RFC-1006+03+10.0.0.6
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("NS+") {
            let bytes = hex::decode(&s[3..]).map_err(|_| NAddressError::InvalidHexEncoding)?;
            return X213NetworkAddress::try_from(bytes.as_slice());
        }
        // TODO: Support the macros defined in section 6 of IETF RFC 1278. (Might be 1277)
        // ________________________________________________
        // |_Macro_____________|Value______________________ |
        // | Int-X25(80)       |TELEX+00728722+X25(80)+01+  |
        // | Janet-X25(80)     |TELEX+00728722+X25(80)+02+  |
        // | Internet-RFC-1006 |TELEX+00728722+RFC-1006+03+ |
        // |_IXI_______________|TELEX+00728722+RFC-1006+06+_|

        // This one is intentionally checked first, since most NSAPs nowadays
        // are likely to use the Telex-Internet IDP.
        if s.starts_with(AFI_STR_TELEX) {
            let nt = X213NetworkAddressType::F69;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_TELEX.len())?;
            let idi = idi_str.as_bytes().to_vec();
            if idi_str == IETF_RFC_1277_TELEX_NUMBER_STR {
                if dsp_str.starts_with(IETF_RFC_1006_PREFIX_STR) {
                    if dsp_str.len() < 19 {
                        return Err(NAddressError::TruncatedDSP);
                    }
                    let dsp_prefix_str = &dsp_str[IETF_RFC_1006_PREFIX_STR.len()..IETF_RFC_1006_PREFIX_STR.len()+2];
                    if [ "03", "10", "11" ].contains(&dsp_prefix_str) { // See ITU-T Rec. X.519 (2019), Sections 11.3.1 - 11.3.3
                        // The remainder of the DSP should be an IP address + port + transport
                        let mut ip_port_and_tset_iter = dsp_str[IETF_RFC_1006_PREFIX_STR.len()+3..].split("+");
                        let ip_str = ip_port_and_tset_iter.next();
                        if ip_str.is_none() {
                            return Err(NAddressError::MalformedDSP);
                        }
                        let maybe_port_str = ip_port_and_tset_iter.next();
                        let maybe_tset_str = ip_port_and_tset_iter.next();
                        let ipv4 = Ipv4Addr::from_str(ip_str.unwrap()).map_err(|_| NAddressError::MalformedDSP)?;
                        let parsed_port = maybe_port_str.map(|port_str| u16::from_str(port_str));
                        let parsed_tset = maybe_tset_str.map(|tset_str| u16::from_str(tset_str));
                        if parsed_tset.is_some() && parsed_port.is_none() {
                            // This should never happen.
                            return Err(NAddressError::InternalError);
                        }
                        let [ ipb1, ipb2, ipb3, ipb4 ] = ipv4.octets();
                        let dsp_dec: Vec<u8>;
                        if parsed_tset.is_some() {
                            let port = parsed_port.unwrap().map_err(|_| NAddressError::MalformedDSP)?;
                            let tset = parsed_tset.unwrap().map_err(|_| NAddressError::MalformedDSP)?;
                            dsp_dec = format!("{:03}{:03}{:03}{:03}{:05}{:05}", ipb1, ipb2, ipb3, ipb4, port, tset)
                                .as_bytes()
                                .iter()
                                .map(|b| b - 0x30)
                                .collect();
                        }
                        else if parsed_port.is_some() {
                            let port = parsed_port.unwrap().map_err(|_| NAddressError::MalformedDSP)?;
                            dsp_dec = format!("{:03}{:03}{:03}{:03}{:05}", ipb1, ipb2, ipb3, ipb4, port)
                                .as_bytes()
                                .iter()
                                .map(|b| b - 0x30)
                                .collect();
                        }
                        else {
                            dsp_dec = format!("{:03}{:03}{:03}{:03}", ipb1, ipb2, ipb3, ipb4)
                                .as_bytes()
                                .iter()
                                .map(|b| b - 0x30)
                                .collect();
                        }
                        return Ok(X213NetworkAddress::new(nt, false, idi, DomainSpecificPart::Decimal(dsp_dec)));
                    }
                    else if [ "00", "01", "02" ].contains(&dsp_prefix_str) {
                        return Err(NAddressError::UnsupportedDSPType);
                    }
                    else {
                        return Err(NAddressError::UnsupportedDSPType);
                    }
                }
                else if dsp_str.starts_with(X25_PREFIX_STR) {
                    let dsp_prefix_str = &dsp_str[X25_PREFIX_STR.len()..X25_PREFIX_STR.len()+2];
                    if [ "00", "01", "02" ].contains(&dsp_prefix_str) {
                        return Err(NAddressError::UnsupportedDSPType);
                    } else {
                        // Technically more variants could be defined, but
                        // realistically, they probably never will since X.25
                        // is being basically dead now.
                        return Err(NAddressError::MalformedDSP);
                    }
                }
                else {
                    return Err(NAddressError::UnsupportedDSPType);
                }
            } else {
                let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
                return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
            }
        }
        else if s.starts_with(AFI_STR_URL) {
            let nt = X213NetworkAddressType::URL;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_URL.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = DomainSpecificPart::Url(dsp_str.to_owned());
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_X121) {
            let nt = X213NetworkAddressType::X121;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_X121.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_DCC) {
            let nt = X213NetworkAddressType::ISO_DCC;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_DCC.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_PSTN) {
            let nt = X213NetworkAddressType::E164;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_PSTN.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_ICD) {
            let nt = X213NetworkAddressType::ISO_6523_ICD;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_ICD.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_ICP) {
            let nt = X213NetworkAddressType::IANA_ICP;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_ICP.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_IND) {
            let nt = X213NetworkAddressType::ITU_T_IND;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_IND.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else if s.starts_with(AFI_STR_LOCAL) {
            let nt = X213NetworkAddressType::LOCAL;
            let (idi_str, dsp_str) = dissect_nsap_str(s, AFI_STR_LOCAL.len())?;
            let idi = idi_str.as_bytes().to_vec();
            let dsp = decode_opaque_dsp(dsp_str.as_bytes())?;
            return Ok(X213NetworkAddress::new(nt, false, idi, dsp));
        }
        else {
            // If all numeric, it COULD be `<idp> + <hexstring>` format. (ISO 8348)
            let parts: Vec<&str> = s.split("+").collect();
            if parts.len() != 2 {
                return Err(NAddressError::UnrecognizedNetworkType);
            }
            let idp: &str = parts[0];
            if idp.len() < 2 {
                return Err(NAddressError::NoAFI);
            }
            let afi = u8::from_str(&idp[0..2]).map_err(|_| NAddressError::NoAFI)?; // Whatever, close enough.
            let nt = naddr_network_type(afi).ok_or(NAddressError::UnrecognizedNetworkType)?;
            let dsp_hex: &str = parts[1];
            let maybe_non_ascii_digit_in_idp = idp.as_bytes().iter().find(|b| !b.is_ascii_digit());
            if let Some(non_ascii_digit) = maybe_non_ascii_digit_in_idp {
                return Err(NAddressError::NonDigitInIDI(*non_ascii_digit));
            }
            let dsp_bytes = hex::decode(&dsp_hex).map_err(|_| NAddressError::MalformedDSP)?;
            let idi = idp[2..].as_bytes().to_vec();
            return Ok(X213NetworkAddress::new(nt, false, idi, DomainSpecificPart::Binary(dsp_bytes)));
        }
    }
}

fn read_socket_addr_v4 (dsp: &[u8]) -> Option<(Ipv4Addr, Option<u16>)> {
    let includes_tcp_port = match dsp.len() {
        12 => false,
        17 => true,
        _ => return None,
    };
    let ipv4 = Ipv4Addr::new(
        (
            (((dsp[1] & 0xF0) >> 4) * 100)
            + ((dsp[1] & 0x0F) * 10)
            + ((dsp[2] & 0xF0) >> 4)
        ),
        (
            ((dsp[2] & 0x0F) * 100)
            + (((dsp[3] & 0xF0) >> 4) * 10)
            + (dsp[3] & 0x0F)
        ),
        (
            (((dsp[4] & 0xF0) >> 4) * 100)
            + ((dsp[4] & 0x0F) * 10)
            + ((dsp[5] & 0xF0) >> 4)
        ),
        (
            ((dsp[5] & 0x0F) * 100)
            + (((dsp[6] & 0xF0) >> 4) * 10)
            + (dsp[6] & 0x0F)
        ),
    );
    if !includes_tcp_port {
        return Some((ipv4, None));
    }
    let port: u16 = (
        (((dsp[7] & 0xF0) >> 4) as u16 * 10000)
        + ((dsp[7] & 0x0F) as u16 * 1000)
        + (((dsp[8] & 0xF0) as u16 >> 4) * 100)
        + ((dsp[8] & 0x0F) as u16 * 10)
        + ((dsp[9] & 0xF0) as u16 >> 4)
    );
    if (dsp[9] & 0x0F) != 0x0F {
        return None; // Invalid padding.
    }
    Some((ipv4, Some(port)))
}

impl TryFrom<&[u8]> for X213NetworkAddress {
    type Error = NAddressError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let afi = get_afi_from_n_address(value)
            .ok_or(NAddressError::NoAFI)?;
        let nt = naddr_network_type(afi)
            .ok_or(NAddressError::UnrecognizedNetworkType)?;
        let max_idi_len = naddr_network_type_to_max_idi_length(nt)
            .ok_or(NAddressError::UnrecognizedNetworkType)?;
        let idi_len_in_bytes = if max_idi_len % 2 > 0 {
            (max_idi_len + 1) >> 1
        } else {
            max_idi_len >> 1
        };
        let expected_idp_len_in_bytes = 1 + idi_len_in_bytes as usize;
        if value.len() < expected_idp_len_in_bytes as usize {
            return Err(NAddressError::IDPTruncated(expected_idp_len_in_bytes, value.len()));
        }
        let padding_digit: u8 = if naddr_idi_has_leading_zero(afi) { 1 } else { 0 };
        let mut idi = Vec::with_capacity(idi_len_in_bytes as usize);
        let mut reading_left_padding = true;
        let mut read_right_pad_digit = false;
        for byte in &value[1..expected_idp_len_in_bytes] {
            let digit1 = *byte >> 4;
            let digit2 = *byte & 0b0000_1111;
            for digit in [ digit1, digit2 ] {
                if digit == padding_digit && reading_left_padding {
                    continue;
                }
                reading_left_padding = false;
                if digit == 0b0000_1111 {
                    if read_right_pad_digit {
                        return Err(NAddressError::InvalidRightPadding);
                    }
                    read_right_pad_digit = true;
                    continue;
                }
                if digit > 9 {
                    return Err(NAddressError::NonDigitInIDI(digit));
                }
                idi.push(digit + 0x30);
            }
        }

        let dsp;
        if naddr_dsp_is_decimal(afi) {
            let mut read_right_pad_digit = false;
            let mut dsp_bytes: Vec<u8> = Vec::with_capacity(value[expected_idp_len_in_bytes..].len() << 2);
            for byte in &value[expected_idp_len_in_bytes..] {
                let digit1 = *byte >> 4;
                let digit2 = *byte & 0b0000_1111;
                for digit in [ digit1, digit2 ] {
                    if digit == 0b0000_1111 {
                        if read_right_pad_digit {
                            return Err(NAddressError::InvalidRightPadding);
                        }
                        read_right_pad_digit = true;
                        continue;
                    }
                    if digit > 9 {
                        return Err(NAddressError::NonDecimalDigitInDSP(digit));
                    }
                    dsp_bytes.push(digit + 0x30);
                }
            }
            dsp = DomainSpecificPart::Decimal(dsp_bytes);
        } else if naddr_dsp_is_binary(afi) {
            dsp = DomainSpecificPart::Binary(Vec::from(&value[expected_idp_len_in_bytes..]));
        } else if naddr_dsp_is_national(afi) {
            dsp = DomainSpecificPart::NationalCharacters(Vec::from(&value[expected_idp_len_in_bytes..]));
        } else {
            for byte in &value[expected_idp_len_in_bytes..] {
                if !byte.is_ascii_graphic() && *byte != b' ' { // Technically, this is also supposed to have no national variant.
                    return Err(NAddressError::NonISO646Character(*byte));
                }
            }
            dsp = DomainSpecificPart::IsoIec646(Vec::from(&value[expected_idp_len_in_bytes..]));
        }

        Ok(X213NetworkAddress::new(nt, is_group_afi(afi), idi, dsp))
    }

}

impl TryInto<Vec<u8>> for X213NetworkAddress {
    type Error = NAddressError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut afi: u8 = self.afi().ok_or(NAddressError::InternalError)?;
        if self.group {
            afi = individual_afi_to_group_afi(afi).unwrap_or(afi);
        }
        let has_leading_zero = self.idi.get(0).is_some_and(|b| *b == 0);
        let leading_0_matters = idi_leading_0_significant(self.network_type).unwrap_or(false);
        let padding_digit = if has_leading_zero && leading_0_matters { 1 } else { 0 };
        let max_idi_len = naddr_network_type_to_max_idi_length(self.network_type)
            .ok_or(NAddressError::InternalError)?;
        if self.idi.len() > max_idi_len as usize {
            return Err(NAddressError::InternalError);
        }
        let mut padding_needed = max_idi_len as usize - self.idi.len();
        let idi_len_in_bytes = if max_idi_len % 2 > 0 {
            (max_idi_len + 1) >> 1
        } else {
            max_idi_len >> 1
        };
        let mut idp: Vec<u8> = Vec::with_capacity(1 + idi_len_in_bytes as usize);
        idp.push(afi);
        while padding_needed > 1 {
            let byte = (padding_digit << 4) + padding_digit;
            idp.push(byte);
            padding_needed -= 2;
        }
        let idi_digits_to_write;
        if padding_needed == 1 {
            let byte = (padding_digit << 4) + (self.idi[0] & 0b0000_1111);
            idp.push(byte);
            idi_digits_to_write = &self.idi[1..];
        } else {
            idi_digits_to_write = &self.idi;
        }
        let mut padding_digits_written = 0;
        while padding_digits_written < idi_digits_to_write.len() {
            let digit1 = idp.get(padding_digits_written);
            if digit1.is_none() {
                break;
            }
            let digit1 = digit1.unwrap();
            let digit2 = idp.get(padding_digits_written + 1).unwrap_or(&0b0000_1111);
            let byte = (*digit1 << 4) + (*digit2 & 0b0000_1111);
            idp.push(byte);
            padding_digits_written += 2;
        }
        let dsp_bytes = match self.dsp {
            DomainSpecificPart::Binary(x) => x,
            DomainSpecificPart::Decimal(x) => x,
            DomainSpecificPart::IsoIec646(x) => x,
            DomainSpecificPart::NationalCharacters(x) => x,
            DomainSpecificPart::Url(x) => x.as_bytes().to_vec(),
        };
        Ok([
            idp,
            dsp_bytes,
        ].concat())
    }

}

impl Display for X213NetworkAddress {

    /// Prints a human-readable string, per the procedures defined in
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.network_type.fmt(f)?;
        f.write_char('+')?;
        for digit in &self.idi {
            f.write_char((*digit + 0x30).into())?;
        }
        f.write_char('+')?;
        Ok(())
    }

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = naddr_network_type_to_str(*self).ok_or(std::fmt::Error)?;
        f.write_str(s)
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
}

impl Display for NAddressError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainSpecificPart {
    Decimal(Vec<u8>), // Each byte is a digit.
    Binary(Vec<u8>), // Raw bytes of the DSP.
    IsoIec646(Vec<u8>), // 7-digit characters, mostly overlapping ASCII
    NationalCharacters(Vec<u8>), // Raw bytes of the DSP.
    Url(String),
}

impl Display for DomainSpecificPart {

    /// Prints a human-readable string, per the procedures defined in
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainSpecificPart::Decimal(digits) => {
                f.write_char('d')?;
                for digit in digits {
                    f.write_char((*digit + 0x30).into())?;
                }
                Ok(())
            },
            DomainSpecificPart::Binary(bytes) => {
                f.write_char('x')?;
                for byte in bytes {
                    f.write_fmt(format_args!("{:02X}", *byte))?;
                }
                Ok(())
            },
            DomainSpecificPart::IsoIec646(chars) => {
                f.write_char('l')?;
                for c in chars {
                    f.write_char((*c).into())?;
                }
                Ok(())
            },
            DomainSpecificPart::NationalCharacters(chars) => {
                let printable = chars.iter().all(|c| c.is_ascii_graphic() || *c == b' ');
                // If the national characters happen to overlap with ASCII
                // enough, just try to print ASCII; otherwise, print hex.
                if printable {
                    f.write_char('l')?;
                    for c in chars {
                        f.write_char((*c).into())?;
                    }
                } else {
                    f.write_char('x')?;
                    for c in chars {
                        f.write_fmt(format_args!("{:02X}", *c))?;
                    }
                }
                Ok(())
            },
            DomainSpecificPart::Url(url) => f.write_str(&url)
        }
    }

}

impl Error for NAddressError {}

/// This function was kept around so you can still get the AFI without parsing the whole NSAP.
pub fn get_afi_from_n_address (naddr: &[u8]) -> Option<u8> {
    naddr.get(0).cloned()
}

pub fn idi_leading_0_significant (nt: X213NetworkAddressType) -> Option<bool> {
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

pub fn individual_afi_to_group_afi (afi: u8) -> Option<u8> {
    match afi {
        0x10..=0x99 => Some(afi + 0x90),
        _ => None,
    }
}

pub fn group_afi_to_individual_afi (afi: u8) -> Option<u8> {
    match afi {
        0xA0..=0xF9 => Some(afi - 0x90),
        _ => None,
    }
}

pub fn is_individual_afi (afi: u8) -> bool {
    afi >= 0x10 && afi <= 0x99
}

pub fn is_group_afi (afi: u8) -> bool {
    afi >= 0xA0 && afi <= 0xF9
}

pub fn is_invalid_afi (afi: u8) -> bool {
    !is_individual_afi(afi) && !is_group_afi(afi)
}

pub fn naddr_network_type (afi: u8) -> Option<X213NetworkAddressType> {
    let ind_afi = group_afi_to_individual_afi(afi).unwrap_or(afi);
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

pub fn naddr_network_type_to_str (nt: X213NetworkAddressType) -> Option<&'static str> {
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

pub fn naddr_network_type_to_max_dec_length (nt: X213NetworkAddressType) -> Option<u8> {
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

pub fn naddr_network_type_to_max_bin_length (nt: X213NetworkAddressType) -> Option<u8> {
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

pub fn naddr_network_type_to_max_idi_length (nt: X213NetworkAddressType) -> Option<u8> {
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

pub fn naddr_dsp_is_binary (afi: u8) -> bool {
    let ind_afi = group_afi_to_individual_afi(afi).unwrap_or(afi);
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

pub fn naddr_dsp_is_decimal (afi: u8) -> bool {
    let ind_afi = group_afi_to_individual_afi(afi).unwrap_or(afi);
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

pub fn naddr_dsp_is_iso_iec_646 (afi: u8) -> bool {
    let ind_afi = group_afi_to_individual_afi(afi).unwrap_or(afi);
    match ind_afi {
        AFI_LOCAL_ISO_IEC_646 => true,
        _ => false,
    }
}

pub fn naddr_dsp_is_national (afi: u8) -> bool {
    let ind_afi = group_afi_to_individual_afi(afi).unwrap_or(afi);
    match ind_afi {
        AFI_LOCAL_NATIONAL => true,
        _ => false,
    }
}


pub fn naddr_idi_has_leading_zero (afi: u8) -> bool {
    let ind_afi = group_afi_to_individual_afi(afi).unwrap_or(afi);
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

// TODO: Check Lengths of DSPs

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
