#![doc = include_str!("../README.md")]
#![no_std]
#![allow(non_camel_case_types)]
pub mod bcd;
pub mod data;
mod display;
pub mod error;
pub mod isoiec646;
mod parse;
mod utils;
use crate::bcd::{BCDBuffer, BCDDigitsIter};
use crate::data::{
    AFI_IANA_ICP_BIN, AFI_URL, INTERNET_PREFIX, RFC_1277_PREFIX, X213NetworkAddressInfo,
    afi_to_network_type, get_address_type_info,
};
use crate::display::{fmt_naddr, fmt_naddr_type};
use crate::error::{NAddressParseError, RFC1278ParseError};
use crate::parse::parse_nsap;
use crate::utils::{u8_to_decimal_bytes, u16_to_decimal_bytes};
use core::convert::TryFrom;
use core::fmt::Display;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4};
use core::str::FromStr;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Authority and Format Identifier (AFI): part of an NSAP address
pub type AFI = u8;

const DEFAULT_ITOT_PORT: u16 = 102;

/// X.213 NSAP Domain-Specific Part Syntax
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DSPSyntax {
    Decimal,
    Binary,
    IsoIec646Chars,
    NationalChars,
}

/// X.213 NSAP network address type
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
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt_naddr_type(self, f)
    }
}

impl TryFrom<AFI> for X213NetworkAddressType {
    type Error = ();

    #[inline]
    fn try_from(value: AFI) -> Result<Self, Self::Error> {
        afi_to_network_type(value).ok_or(())
    }
}

/// ITU-T Recommendation X.213 NSAP Address
///
/// This is composed of three parts, encoded in this order:
///
/// - Authority and Format Identifier (AFI): always a single byte, which
///   identifies the network type and the syntax of the encoding that follows
/// - Initial Domain Identifier (IDI): the authority for allocating further
///   address identifiers, which is always encoded as binary-coded decimal,
///   and padded with either 0x0 or 0x1 depending on whether leading zeroes are
///   significant or not until the maximum length is reached in digits.
/// - Domain-Specific Part (DSP): the remainder of the information that
///   completes the address and is allocated by the authority identified by the
///   IDI. It can have four abstract syntaxes: BCD, binary, ISO/IEC 646 string,
///   and a national character encoding.
///
/// Together, the AFI and IDI are referred to as the Initial Domain Part (IDP).
///
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
pub enum X213NetworkAddress<'a> {
    #[cfg(feature = "alloc")]
    Heap(Vec<u8>),
    /// Even though NSAPs are capped at 20 bytes, this inline buffer accepts up
    /// to 22 just so that programming errors are less likely to result in
    /// reading out-of-bounds.
    Inline((u8, [u8; 22])),
    Borrowed(&'a [u8]),
}

impl<'a> X213NetworkAddress<'a> {

    /// Get the bytes of the encoded NSAP address
    #[inline]
    pub fn get_octets(&'a self) -> &'a [u8] {
        match &self {
            #[cfg(feature = "alloc")]
            X213NetworkAddress::Heap(o) => o.as_ref(),
            X213NetworkAddress::Inline((sz, buf)) => &buf[0..(*sz).clamp(0u8, 20u8) as usize],
            X213NetworkAddress::Borrowed(o) => *o,
        }
    }

    /// Get the Authority and Format Identifier (AFI): part of an NSAP address
    #[inline]
    pub fn afi(&self) -> u8 {
        if self.get_octets().len() > 0 {
            self.get_octets()[0]
        } else {
            panic!("Zero-length IDP in an X.213 network address")
        }
    }

    /// Create an NSAP address from its binary encoding as a `Vec<u8>` without checking validity
    #[cfg(feature = "alloc")]
    pub fn from_vec_unchecked(octets: Vec<u8>) -> X213NetworkAddress<'static> {
        X213NetworkAddress::Heap(octets)
    }

    /// Create an NSAP address from its binary encoding as a `Vec<u8>` after checking validity
    #[cfg(feature = "alloc")]
    pub fn from_vec(octets: Vec<u8>) -> Result<X213NetworkAddress<'static>, NAddressParseError> {
        validate_raw_nsap(octets.as_ref())?;
        Ok(X213NetworkAddress::Heap(octets))
    }

    /// Create an NSAP address from its binary encoding as a `&[u8]` without checking validity
    pub fn from_slice_unchecked(octets: &'a [u8]) -> X213NetworkAddress<'a> {
        X213NetworkAddress::Borrowed(octets)
    }

    /// Create an NSAP address from its binary encoding as a `&[u8]` after checking validity
    pub fn from_slice(octets: &'a [u8]) -> Result<X213NetworkAddress<'a>, NAddressParseError> {
        validate_raw_nsap(octets.as_ref())?;
        Ok(X213NetworkAddress::Borrowed(octets))
    }

    /// Get network type info for this NSAP address
    #[inline]
    pub fn get_network_type_info(&self) -> Option<X213NetworkAddressInfo> {
        get_address_type_info(self.afi())
    }

    /// Get the network type for this NSAP address
    #[inline]
    pub fn get_network_type(&self) -> Option<X213NetworkAddressType> {
        afi_to_network_type(self.afi())
    }

    /// Iterate over the IDI digits for this NSAP address
    ///
    /// Returns `None` if the AFI is unrecognized, and therefore, that the
    /// NSAP address cannot be parsed, since the end of the IDI cannot be
    /// determined.
    pub fn idi_digits(&'a self) -> Option<BCDDigitsIter<'a>> {
        let addr_type_info = get_address_type_info(self.afi())?;
        let leading_0_sig = addr_type_info.leading_zeroes_in_idi;
        let is_dsp_decimal = matches!(addr_type_info.dsp_syntax, DSPSyntax::Decimal);
        let idi_len = addr_type_info.max_idi_len_digits as usize;
        let idi_len_in_bytes = (idi_len >> 1) + (idi_len % 2);
        let odd_len_idi: bool = (idi_len % 2) > 0;
        let octets = self.get_octets();
        let idi = &octets[1..1 + idi_len_in_bytes];
        Some(BCDDigitsIter::new(
            idi,
            leading_0_sig,
            is_dsp_decimal && odd_len_idi,
            false,
            true,
        ))
    }

    /// Iterate over the IDI digits for this NSAP address, if the DSP is in decimal
    ///
    /// Returns `None` if the AFI is unrecognized, and therefore, that the
    /// NSAP address cannot be parsed, since the end of the IDI cannot be
    /// determined. Also returns `None` if the DSP syntax is not decimal.
    pub fn dsp_digits(&'a self) -> Option<BCDDigitsIter<'a>> {
        let addr_type_info = get_address_type_info(self.afi())?;
        let is_dsp_decimal = matches!(addr_type_info.dsp_syntax, DSPSyntax::Decimal);
        if !is_dsp_decimal {
            return None;
        }
        let idi_len = addr_type_info.max_idi_len_digits as usize;
        let idi_len_in_bytes = (idi_len >> 1) + (idi_len % 2);
        let odd_len_idi: bool = (idi_len % 2) > 0;
        let octets = self.get_octets();
        // This needs to take the byte before if odd number of IDI digits
        let (dsp, start_on_lsn) = if is_dsp_decimal && odd_len_idi {
            (&octets[idi_len_in_bytes..], true)
        } else {
            (&octets[1 + idi_len_in_bytes..], false)
        };
        Some(BCDDigitsIter::new(
            dsp,
            false, // No leading zeroes supported in DSPs
            false, // Only ignore the last nybble if it is 0x0F
            start_on_lsn,
            false,
        ))
    }

    /// Get the encoded URL
    ///
    /// This returns `None` if this NSAP does not encode a URL
    pub fn get_url(&'a self) -> Option<&'a str> {
        let octets = self.get_octets();
        // It couldn't be a valid URL in two characters, AFAIK.
        if octets.len() <= 5 || octets[0] != AFI_URL {
            return None;
        }
        str::from_utf8(&octets[3..]).ok()
    }

    /// Get the encoded IP address
    ///
    /// This only returns an IP address for IANA ICP-based NSAP addresses
    ///
    /// This returns `None` if this NSAP does not encode an IP address
    /// See: <https://www.rfc-editor.org/rfc/rfc4548.html>
    pub fn get_ip(&self) -> Option<IpAddr> {
        let octets = self.get_octets();
        if octets.len() < 7 || octets[0] != AFI_IANA_ICP_BIN {
            return None;
        }
        // See doc comments on AFI_IANA_ICP_DEC for why it is not supported.
        match (octets[1], octets[2]) {
            (0, 0) => {
                // IPv6
                if octets.len() < 19 {
                    return None;
                }
                let ip = Ipv6Addr::from([
                    octets[3], octets[4], octets[5], octets[6], octets[7], octets[8], octets[9],
                    octets[10], octets[11], octets[12], octets[13], octets[14], octets[15],
                    octets[16], octets[17], octets[18],
                ]);
                Some(IpAddr::V6(ip))
            }
            (0, 1) => {
                // IPv4
                let ip = Ipv4Addr::from([octets[3], octets[4], octets[5], octets[6]]);
                Some(IpAddr::V4(ip))
            }
            _ => None,
        }
    }

    /// Get the ISO Transport over TCP (ITOT) socket address
    ///
    /// This returns `None` if this NSAP does not encode an ITOT socket address
    pub fn get_itot_socket_addr(&self) -> Option<SocketAddrV4> {
        let octets = self.get_octets();
        if !octets.starts_with(INTERNET_PREFIX.as_slice()) {
            return None;
        }
        let dsp = &octets[INTERNET_PREFIX.len()..];
        if dsp.len() < 6 {
            return None;
        }
        let mut bcd = BCDDigitsIter::new(dsp, false, false, false, false);
        let oct1digs = [bcd.next()? + 0x30, bcd.next()? + 0x30, bcd.next()? + 0x30];
        let oct2digs = [bcd.next()? + 0x30, bcd.next()? + 0x30, bcd.next()? + 0x30];
        let oct3digs = [bcd.next()? + 0x30, bcd.next()? + 0x30, bcd.next()? + 0x30];
        let oct4digs = [bcd.next()? + 0x30, bcd.next()? + 0x30, bcd.next()? + 0x30];
        let oct1str = unsafe { str::from_utf8_unchecked(oct1digs.as_slice()) };
        let oct2str = unsafe { str::from_utf8_unchecked(oct2digs.as_slice()) };
        let oct3str = unsafe { str::from_utf8_unchecked(oct3digs.as_slice()) };
        let oct4str = unsafe { str::from_utf8_unchecked(oct4digs.as_slice()) };
        let oct1: u8 = oct1str.parse().ok()?;
        let oct2: u8 = oct2str.parse().ok()?;
        let oct3: u8 = oct3str.parse().ok()?;
        let oct4: u8 = oct4str.parse().ok()?;
        let ip = Ipv4Addr::new(oct1, oct2, oct3, oct4);
        if dsp.len() < 9 {
            return Some(SocketAddrV4::new(ip, DEFAULT_ITOT_PORT));
        }
        let portstr = [
            bcd.next()? + 0x30,
            bcd.next()? + 0x30,
            bcd.next()? + 0x30,
            bcd.next()? + 0x30,
            bcd.next()? + 0x30,
        ];
        let portstr = unsafe { str::from_utf8_unchecked(portstr.as_slice()) };
        let port: u16 = portstr.parse().ok()?;
        Some(SocketAddrV4::new(ip, port))
    }

    /// Create a new IANA ICP NSAP address from an IP address
    pub fn from_ip(ip: &IpAddr) -> Self {
        match ip {
            IpAddr::V4(v4) => X213NetworkAddress::from_ipv4(v4),
            IpAddr::V6(v6) => X213NetworkAddress::from_ipv6(v6),
        }
    }

    /// Create a new IANA ICP NSAP address from an IPv4 address
    pub fn from_ipv4(ip: &Ipv4Addr) -> Self {
        let mut out: [u8; 22] = [0; 22];
        out[0..3].copy_from_slice(&[AFI_IANA_ICP_BIN, 0, 1]);
        out[3..7].copy_from_slice(ip.octets().as_slice());
        // IANA ICP NSAP addresses are always 20 bytes
        return X213NetworkAddress::Inline((20, out));
    }

    /// Create a new IANA ICP NSAP address from an IPv6 address
    pub fn from_ipv6(ip: &Ipv6Addr) -> Self {
        let mut out: [u8; 22] = [0; 22];
        out[0..3].copy_from_slice(&[AFI_IANA_ICP_BIN, 0, 0]);
        out[3..19].copy_from_slice(ip.octets().as_slice());
        // IANA ICP NSAP addresses are always 20 bytes
        return X213NetworkAddress::Inline((20, out));
    }

    /// Create a new X.519 ITOT URL NSAP address from a URL
    #[cfg(feature = "alloc")]
    pub fn from_itot_url(url: &str) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(3 + url.len());
        out.extend(&[AFI_URL, 0, 0]);
        out.extend(url.as_bytes());
        return X213NetworkAddress::Heap(out);
    }

    /// Create a new X.519 Non-OSI (LDAP, IDM, etc.) URL NSAP address from a URL
    #[cfg(feature = "alloc")]
    pub fn from_non_osi_url(url: &str) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(3 + url.len());
        out.extend(&[AFI_URL, 0, 1]);
        out.extend(url.as_bytes());
        return X213NetworkAddress::Heap(out);
    }

    /// Create an ITOT NSAP address from a socket address and optional transport set
    ///
    /// Note that this only supports IPv4 due to the encoding.
    pub fn from_itot_socket_addr(addr: &SocketAddrV4, tset: Option<u16>) -> Self {
        let mut out: [u8; 22] = [0; 22];
        out[0..6].copy_from_slice(INTERNET_PREFIX.as_slice());
        let mut bcd_buf = BCDBuffer::new();
        addr.ip()
            .octets()
            .map(|o| u8_to_decimal_bytes(o))
            .iter()
            .for_each(|dec_oct| bcd_buf.push_ascii_bytes(dec_oct.as_slice()));
        let port = addr.port();
        if port != DEFAULT_ITOT_PORT || tset.is_some_and(|t| t != DEFAULT_ITOT_TRANSPORT_SET) {
            let port_str = u16_to_decimal_bytes(port);
            bcd_buf.push_ascii_bytes(port_str.as_slice());
            if let Some(tset) = tset {
                let tset_str = u16_to_decimal_bytes(tset);
                bcd_buf.push_ascii_bytes(tset_str.as_slice());
            } else {
                bcd_buf.push_nybble(0xF);
            }
        }
        let bcd_len = bcd_buf.len_in_bytes();
        debug_assert_eq!(bcd_len, bcd_buf.as_ref().len());
        debug_assert!(bcd_len < 19);
        out[6..6 + bcd_len].copy_from_slice(bcd_buf.as_ref());
        X213NetworkAddress::Inline((6 + bcd_len as u8, out))
    }

    /// Convert to a `String` using the `NS+<hex>` syntax
    ///
    /// This is desirable for portability / interoperability: the `NS+<hex>`
    /// syntax is the easiest display syntax to parse and leaves no ambiguity of
    /// encoding. This is a great choice if you are exporting an NSAP address in
    /// string format for use in other systems.
    ///
    /// The output looks like `NS+A433BB93C1`.
    #[cfg(feature = "alloc")]
    pub fn to_ns_string(&self) -> String {
        ["NS+", hex::encode(self.get_octets()).as_str()].join("")
    }

    /// Display using the `NS+<hex>` syntax
    ///
    /// This is desirable for portability / interoperability: the `NS+<hex>`
    /// syntax is the easiest display syntax to parse and leaves no ambiguity of
    /// encoding. This is a great choice if you are exporting an NSAP address in
    /// string format for use in other systems.
    ///
    /// The output looks like `NS+A433BB93C1`.
    pub fn fmt_as_ns_string(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("NS+")?;
        for byte in self.get_octets() {
            f.write_fmt(format_args!("{:02X}", *byte))?;
        }
        Ok(())
    }
}

fn validate_decimal(bytes: &[u8]) -> bool {
    for byte in bytes {
        if (byte & 0b0000_1111) > 9 {
            return false;
        }
        if (byte & 0b1111_0000) > 0b1001_0000 {
            return false;
        }
    }
    true
}

fn validate_raw_nsap<'a>(octets: &'a [u8]) -> Result<(), NAddressParseError> {
    let len = octets.len();
    if len < 2 {
        // I don't think one byte can be a valid address.
        return Err(NAddressParseError::TooShort);
    }
    /* ITU-T Rec. X.213, Section A.5.4 states that the maximum length MUST
    be 20 octets, but ITU-T Rec. X.519 section 11.4 basically overrules
    that. As such, we are just setting a limit of 248 bytes just to close up
    any attack vectors related to large NSAP addresses. */
    if octets[0] != AFI_URL && len > 20 {
        return Err(NAddressParseError::TooLong);
    }

    match octets[0] {
        crate::AFI_URL => {
            if len > 248 {
                return Err(NAddressParseError::TooLong);
            }
            if len <= 5 {
                // I think you can't have a valid URL under two characters.
                return Err(NAddressParseError::TooShort);
            }
            if !validate_decimal(&octets[1..3]) {
                return Err(NAddressParseError::NonDigitsInIDI);
            }
        }
        crate::AFI_IANA_ICP_BIN => {
            if len > 20 {
                return Err(NAddressParseError::TooLong);
            }
            if len < 20 {
                return Err(NAddressParseError::TooShort);
            }
            if !validate_decimal(&octets[1..3]) {
                return Err(NAddressParseError::NonDigitsInIDI);
            }
        }
        _ => (),
    };

    if len >= RFC_1277_PREFIX.len() + 7 && octets.starts_with(RFC_1277_PREFIX.as_slice()) {
        match octets[RFC_1277_PREFIX.len()] {
            crate::data::RFC_1277_WELL_KNOWN_NETWORK_DARPA_NSF_INTERNET
            | crate::data::ITU_X519_DSP_PREFIX_LDAP
            | crate::data::ITU_X519_DSP_PREFIX_IDM_OVER_IPV4
            // | crate::data::ITU_X519_DSP_PREFIX_ITOT_OVER_IPV4 (duplicate)
            => {
                let end_of_digits = match len {
                    12 => 12,
                    15 => 14,
                    17 => 17,
                    _ => return Err(NAddressParseError::MalformedDSP),
                };
                if !validate_decimal(&octets[6..end_of_digits]) {
                    return Err(NAddressParseError::MalformedDSP);
                }
            },
            _ => (),
        };
    }
    Ok(())
}

impl<'a> TryFrom<&'a [u8]> for X213NetworkAddress<'a> {
    type Error = NAddressParseError;

    fn try_from(octets: &'a [u8]) -> Result<Self, Self::Error> {
        validate_raw_nsap(octets)?;
        Ok(X213NetworkAddress::Borrowed(octets))
    }
}

#[cfg(feature = "alloc")]
impl<'a> TryFrom<Vec<u8>> for X213NetworkAddress<'a> {
    type Error = NAddressParseError;

    fn try_from(octets: Vec<u8>) -> Result<Self, Self::Error> {
        validate_raw_nsap(octets.as_ref())?;
        Ok(X213NetworkAddress::Heap(octets))
    }
}

impl<'a> From<&IpAddr> for X213NetworkAddress<'a> {
    #[inline]
    fn from(value: &IpAddr) -> Self {
        X213NetworkAddress::from_ip(value)
    }
}

impl<'a> From<&Ipv4Addr> for X213NetworkAddress<'a> {
    #[inline]
    fn from(value: &Ipv4Addr) -> Self {
        X213NetworkAddress::from_ipv4(value)
    }
}

impl<'a> From<&Ipv6Addr> for X213NetworkAddress<'a> {
    #[inline]
    fn from(value: &Ipv6Addr) -> Self {
        X213NetworkAddress::from_ipv6(value)
    }
}

const DEFAULT_ITOT_TRANSPORT_SET: u16 = 1;

impl<'a> Display for X213NetworkAddress<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt_naddr(self, f)
    }
}

impl<'a> FromStr for X213NetworkAddress<'a> {
    type Err = RFC1278ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, RFC1278ParseError> {
        parse_nsap(s)
    }
}

#[cfg(test)]
mod tests {

    extern crate alloc;
    use crate::data::{AFI_IANA_ICP_BIN, AFI_ISO_DCC_DEC, AFI_X121_DEC_LEADING_ZERO};
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use core::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4};
    use core::str::FromStr;

    use super::X213NetworkAddress;

    use super::data::AFI_F69_DEC_LEADING_ZERO;
    #[cfg(feature = "nonstddisplay")]
    use super::data::AFI_IANA_ICP_BIN;

    #[test]
    fn test_display_01() {
        let input = [
            0x36u8, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, // IDI = 102030405
            0x12, 0x34, 0x56, 0x78, 0x90,
        ];
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "X121+102030405+d1234567890");
    }

    #[cfg(feature = "nonstddisplay")]
    #[test]
    fn test_display_02_url() {
        let input = b"\xFF\x00\x01https://wildboarsoftware.com/x500directory";
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(
            addr_str,
            "URL+0001+https://wildboarsoftware.com/x500directory"
        );
    }

    #[test]
    fn test_display_02_itot() {
        let input = &[
            0x54, 0, 0x72, 0x87, 0x22, 3, 1, 0, 0, 0, 0, 6, 0, 0, 0x90, 0, 2,
        ];
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "TELEX+00728722+RFC-1006+03+10.0.0.6+9+2");
    }

    #[cfg(feature = "nonstddisplay")]
    #[test]
    fn test_display_03_ip() {
        let input = &[
            AFI_IANA_ICP_BIN,
            0,
            1,
            192,
            168,
            1,
            100,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "IP4+192.168.1.100");
    }

    #[test]
    fn test_get_url() {
        let input = b"\xFF\x00\x01https://wildboarsoftware.com/x500directory";
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        assert_eq!(
            addr.get_url().unwrap(),
            "https://wildboarsoftware.com/x500directory"
        );
    }

    #[test]
    fn test_from_itot_socket_addr() {
        let sock = SocketAddrV4::from_str("192.168.1.100:8000").unwrap();
        let addr = X213NetworkAddress::from_itot_socket_addr(&sock, None);
        // assert_eq!(addr, "https://wildboarsoftware.com/x500directory");
        assert_eq!(
            addr.get_octets(),
            &[
                AFI_F69_DEC_LEADING_ZERO, // AFI
                0x00,
                0x72,
                0x87,
                0x22, // IDI
                0x03, // The DSP prefix "03"
                0x19,
                0x21,
                0x68,
                0x00,
                0x11,
                0x00,
                0x08,
                0x00,
                0x0F,
            ]
        );
    }

    #[cfg(feature = "nonstd")]
    #[test]
    fn test_ip_overflow_1() {
        let input = "IP4+999.999.2.100";
        let maybe_addr = X213NetworkAddress::from_str(input);
        assert!(maybe_addr.is_err());
    }

    #[test]
    fn test_ip_overflow_2() {
        let input = "TELEX+00728722+RFC-1006+03+256.0.0.2+9+2";
        let maybe_addr = X213NetworkAddress::from_str(input);
        assert!(maybe_addr.is_err());
    }

    #[test]
    fn test_ip_overflow_3() {
        let input = "TELEX+00728722+RFC-1006+03+0.255.255.255+99999+88888";
        let maybe_addr = X213NetworkAddress::from_str(input);
        assert!(maybe_addr.is_err());
    }

    #[test]
    #[ignore]
    fn test_ip_overflow_4() {
        let input: &[u8] = &[
            0x54, 0x00, 0x72, 0x87, 0x22, 0x03, 0x99, 0x90, 0x00, 0x00, 0x00,
            0x06, // 999.0.0.6
        ];
        let maybe_addr = X213NetworkAddress::try_from(input);
        assert!(maybe_addr.is_err());
    }

    #[test]
    fn test_get_itot_socket_adder() {
        let input = "TELEX+00728722+RFC-1006+03+255.0.0.2+65535+2";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let sock = addr.get_itot_socket_addr().unwrap();
        assert_eq!(sock.ip(), &Ipv4Addr::new(255, 0, 0, 2));
        assert_eq!(sock.port(), 65535);
    }

    // Up to 14 digits, leading zeroes significant
    #[test]
    fn test_idi_digits_x121() {
        let input = "X121+00123456789+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_idi_digits_dcc() {
        let input = "X121+023+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[0, 2, 3]);
    }

    // Up to 8 digits, leading zero significant
    #[test]
    fn test_idi_digits_telex() {
        let input = "TELEX+01234+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[0, 1, 2, 3, 4]);
    }

    // Up to 12 digits, leading zero significant
    #[test]
    fn test_idi_digits_pstn() {
        let input = "PSTN+8883334022+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[8, 8, 8, 3, 3, 3, 4, 0, 2, 2]);
    }

    // Up to 15 digits, leading zero significant
    #[test]
    fn test_idi_digits_idsn() {
        let input = "ISDN+0018883334022+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[0, 0, 1, 8, 8, 8, 3, 3, 3, 4, 0, 2, 2]);
    }

    #[test]
    fn test_idi_digits_icd() {
        let input = "ICD+0023+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[2, 3]);
    }

    #[cfg(feature = "nonstd")]
    #[test]
    fn test_idi_digits_icp() {
        let input = "ICP+0001+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[1]);
    }

    #[cfg(feature = "nonstd")]
    #[test]
    fn test_idi_digits_ind() {
        let input = "IND+0123+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_idi_digits_local() {
        let input = "LOCAL++x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[]);
    }

    #[cfg(feature = "nonstd")]
    #[test]
    fn test_idi_digits_url() {
        let input = "URL+0001+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.idi_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[1]);
    }

    #[test]
    fn test_dsp_digits_x121() {
        let input = "X121+00123456789+d2929";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.dsp_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[2, 9, 2, 9]);
        assert_eq!(
            addr.get_octets(),
            &[
                AFI_X121_DEC_LEADING_ZERO,
                0x11,
                0x10,
                0x01,
                0x23,
                0x45,
                0x67,
                0x89,
                0x29,
                0x29,
            ]
        );
    }

    #[test]
    fn test_dsp_digits_dcc() {
        let input = "DCC+840+d1298";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        let digits: Vec<u8> = addr.dsp_digits().unwrap().collect();
        assert_eq!(digits.as_slice(), &[1, 2, 9, 8]);
        assert_eq!(
            addr.get_octets(),
            &[AFI_ISO_DCC_DEC, 0x84, 0x01, 0x29, 0x8F,]
        );
    }

    #[test]
    fn test_from_ipv4() {
        let input = Ipv4Addr::new(192, 168, 1, 255);
        let addr = X213NetworkAddress::from_ipv4(&input);
        assert_eq!(
            addr.get_octets(),
            &[
                AFI_IANA_ICP_BIN,
                0,
                1, // Ipv4
                192,
                168,
                1,
                255, // The IP address
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0, // Required padding
            ]
        );
    }

    #[test]
    fn test_from_ipv6() {
        let input = Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8);
        let addr = X213NetworkAddress::from_ipv6(&input);
        assert_eq!(
            addr.get_octets(),
            &[
                AFI_IANA_ICP_BIN,
                0,
                0, // Ipv4
                0,
                1,
                0,
                2,
                0,
                3,
                0,
                4,
                0,
                5,
                0,
                6,
                0,
                7,
                0,
                8, // IP address
                0, // Required padding
            ]
        );
    }

    #[test]
    fn test_from_itot_url() {
        let addr = X213NetworkAddress::from_itot_url("https://himom.org");
        assert_eq!(addr.get_octets(), b"\xFF\x00\x00https://himom.org");
    }

    #[test]
    fn test_to_ns_string() {
        let input = "DCC+840+d1298";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        assert_eq!(addr.to_ns_string().as_str(), "NS+388401298f");
    }
}
