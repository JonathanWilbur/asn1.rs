//! Parsing X.213 NSAP addresses from strings
#[cfg(feature = "alloc")]
extern crate alloc;
use crate::bcd::BCDBuffer;
#[cfg(feature = "x25")]
use crate::data::ECMA_117_DECIMAL_STR;
#[cfg(feature = "x25")]
use crate::data::X25_PREFIX_STR;
use crate::data::{
    AFI_E163_BIN_LEADING_NON_ZERO, AFI_E163_BIN_LEADING_ZERO, AFI_E163_DEC_LEADING_NON_ZERO,
    AFI_E163_DEC_LEADING_ZERO, AFI_E164_BIN_LEADING_NON_ZERO, AFI_E164_BIN_LEADING_ZERO,
    AFI_E164_DEC_LEADING_NON_ZERO, AFI_E164_DEC_LEADING_ZERO, AFI_F69_BIN_LEADING_NON_ZERO,
    AFI_F69_BIN_LEADING_ZERO, AFI_F69_DEC_LEADING_NON_ZERO, AFI_F69_DEC_LEADING_ZERO,
    AFI_ISO_6523_ICD_BIN, AFI_ISO_6523_ICD_DEC, AFI_ISO_DCC_BIN, AFI_ISO_DCC_DEC, AFI_LOCAL_BIN,
    AFI_LOCAL_DEC, AFI_LOCAL_ISO_IEC_646, AFI_LOCAL_NATIONAL, AFI_URL,
    AFI_X121_BIN_LEADING_NON_ZERO, AFI_X121_BIN_LEADING_ZERO, AFI_X121_DEC_LEADING_NON_ZERO,
    AFI_X121_DEC_LEADING_ZERO, ECMA_117_BINARY_STR, IETF_RFC_1006_PREFIX_STR,
    get_nsap_address_schema,
};
#[cfg(feature = "nonstd")]
use crate::data::{
    AFI_IANA_ICP_BIN, AFI_IANA_ICP_DEC, AFI_ITU_T_IND_BIN, AFI_ITU_T_IND_DEC, AFI_STR_URL,
    IPV4_STR, IPV6_STR,
};
use crate::error::RFC1278ParseError;
use crate::isoiec646::char_to_local_iso_iec_646_byte;
use crate::utils::{u8_to_decimal_bytes, u16_to_decimal_bytes};
use crate::{AFI, DSPSyntax, X213NetworkAddress};
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::net::Ipv4Addr;
#[cfg(feature = "nonstd")]
use core::net::Ipv6Addr;
use core::str::FromStr;

pub(crate) type ParseResult<'a> = Result<X213NetworkAddress<'a>, RFC1278ParseError>;

/// Validate that string is a digitstring and shorter than `max_len`
#[cfg(feature = "nonstd")]
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

/// Decode an AFI from a `str`, such as "X121"
fn decode_afi_from_str(s: &str) -> Result<AFI, RFC1278ParseError> {
    debug_assert_eq!(s.len(), 2);
    let mut out: [u8; 1] = [0];
    faster_hex::hex_decode(s.as_bytes(), out.as_mut_slice())
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
    let schema = get_nsap_address_schema(afi).ok_or(RFC1278ParseError::UnrecognizedAFI)?;
    let idi_len_digits = schema.max_idi_len_digits as usize;
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
    let bcd_len = bcd_buf.len_in_bytes();
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
    Ok(X213NetworkAddress::Inline((bcd_len as u8, out)))
}

/// Whether the character `c` is an `<other>`, per RFC 1278.
#[inline]
const fn is_other_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.'
}

/// Translate an AFI string, such as "X121" to an AFI value
fn naddr_str_to_afi(s: &str, leading0: bool, dsp_syntax: DSPSyntax) -> Option<AFI> {
    /* I don't know the precise details of Rust's match implementation, but
    I implemented this as doubly-nested matching so that string comparison
    does not happen for every single case. */
    match s {
        crate::data::AFI_STR_X121 => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_X121_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_X121_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_X121_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_X121_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::data::AFI_STR_DCC => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_ISO_DCC_DEC),
            DSPSyntax::Binary => Some(AFI_ISO_DCC_BIN),
            _ => None,
        },
        crate::data::AFI_STR_TELEX => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_F69_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_F69_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_F69_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_F69_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::data::AFI_STR_PSTN => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_E163_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_E163_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_E163_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_E163_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::data::AFI_STR_ISDN => match (dsp_syntax, leading0) {
            (DSPSyntax::Decimal, false) => Some(AFI_E164_DEC_LEADING_NON_ZERO),
            (DSPSyntax::Decimal, true) => Some(AFI_E164_DEC_LEADING_ZERO),
            (DSPSyntax::Binary, false) => Some(AFI_E164_BIN_LEADING_NON_ZERO),
            (DSPSyntax::Binary, true) => Some(AFI_E164_BIN_LEADING_ZERO),
            _ => None,
        },
        crate::data::AFI_STR_ICD => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_ISO_6523_ICD_DEC),
            DSPSyntax::Binary => Some(AFI_ISO_6523_ICD_BIN),
            _ => None,
        },
        #[cfg(feature = "nonstd")]
        crate::data::AFI_STR_ICP => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_IANA_ICP_DEC),
            DSPSyntax::Binary => Some(AFI_IANA_ICP_BIN),
            _ => None,
        },
        #[cfg(feature = "nonstd")]
        crate::data::AFI_STR_IND => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_ITU_T_IND_DEC),
            DSPSyntax::Binary => Some(AFI_ITU_T_IND_BIN),
            _ => None,
        },
        crate::data::AFI_STR_LOCAL => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_LOCAL_DEC),
            DSPSyntax::Binary => Some(AFI_LOCAL_BIN),
            DSPSyntax::IsoIec646Chars => Some(AFI_LOCAL_ISO_IEC_646),
            DSPSyntax::NationalChars => Some(AFI_LOCAL_NATIONAL),
        },
        crate::data::AFI_STR_URL => Some(AFI_URL),
        _ => None,
    }
}

#[cfg(feature = "nonstd")]
fn parse_url(idi: &str, url: &str) -> ParseResult<'static> {
    /* The URL cannot contain underscores only because RFC 1278 uses
    underscores to separate NSAP addresses in a presentation address. */
    if url.contains('_') {
        return Err(RFC1278ParseError::ProhibitedCharacter('_'));
    }
    let mut bcd_buf = BCDBuffer::new();
    bcd_buf.push_byte(AFI_URL);
    let mut idi_deficit = 4usize.saturating_sub(idi.len());
    while idi_deficit > 0 {
        bcd_buf.push_nybble(0);
        idi_deficit -= 1;
    }
    bcd_buf.push_str(idi);
    if url.len() > 17 {
        #[cfg(feature = "alloc")]
        {
            let out = [bcd_buf.as_ref(), url.as_bytes()].concat();
            return Ok(X213NetworkAddress::Heap(out));
        }
        #[cfg(not(feature = "alloc"))]
        {
            return Err(RFC1278ParseError::TooLarge);
        }
    }
    let mut out: [u8; 22] = [0; 22];
    out[0..3].copy_from_slice(bcd_buf.as_ref());
    out[3..3 + url.len()].copy_from_slice(url.as_bytes());
    Ok(X213NetworkAddress::Inline((3 + url.len() as u8, out)))
}

/// Parse the part that comes after "NS+"
fn parse_ns_dsp(ns: &str) -> ParseResult<'static> {
    let len = ns.len() / 2;
    if len > 20 {
        #[cfg(feature = "alloc")]
        {
            let mut hexbytes: Vec<u8> = Vec::with_capacity(len);
            // We want to tolerate any size, because this could encode a URL NSAP.
            faster_hex::hex_decode(ns.as_bytes(), &mut hexbytes)
                .map_err(|_| RFC1278ParseError::Malformed)?;
            return Ok(X213NetworkAddress::Heap(hexbytes));
        }
        #[cfg(not(feature = "alloc"))]
        {
            return Err(RFC1278ParseError::TooLarge);
        }
    }
    let mut out: [u8; 22] = [0; 22];
    faster_hex::hex_decode(ns.as_bytes(), &mut out[0..len])
        .map_err(|_| RFC1278ParseError::Malformed)?;
    return Ok(X213NetworkAddress::Inline((len as u8, out)));
}

fn parse_decimal_dsp(mut bcd_buf: BCDBuffer, dsp: &str) -> ParseResult<'static> {
    if !dsp.as_bytes()[1..].iter().all(|b| b.is_ascii_digit()) {
        return Err(RFC1278ParseError::Malformed);
    }
    bcd_buf.push_ascii_bytes(&dsp.as_bytes()[1..]);
    if (bcd_buf.i % 2) > 0 {
        bcd_buf.push_nybble(0x0F);
    }
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_buf.len_in_bytes()].copy_from_slice(bcd_buf.as_ref());
    Ok(X213NetworkAddress::Inline((
        bcd_buf.len_in_bytes() as u8,
        out,
    )))
}

fn parse_hexadecimal_dsp(idp: BCDBuffer, dsp: &str) -> ParseResult<'static> {
    let dsp_len = dsp.as_bytes()[1..].len() >> 1;
    let bcd_len = idp.len_in_bytes();
    if bcd_len + dsp_len > 20 {
        #[cfg(feature = "alloc")]
        {
            let mut hexbytes: Vec<u8> = Vec::with_capacity(dsp.len() - 1);
            faster_hex::hex_decode(&dsp.as_bytes()[1..], &mut hexbytes)
                .map_err(|_| RFC1278ParseError::Malformed)?;
            let out = [idp.as_ref(), hexbytes.as_ref()].concat();
            return Ok(X213NetworkAddress::Heap(out));
        }
        #[cfg(not(feature = "alloc"))]
        {
            return Err(RFC1278ParseError::TooLarge);
        }
    }
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(idp.as_ref());
    faster_hex::hex_decode(&dsp.as_bytes()[1..], &mut out[bcd_len..bcd_len + dsp_len])
        .map_err(|_| RFC1278ParseError::Malformed)?;
    Ok(X213NetworkAddress::Inline(((bcd_len + dsp_len) as u8, out)))
}

fn parse_textual_dsp(idp: BCDBuffer, dsp: &str) -> ParseResult<'static> {
    // RFC 1278: <other> ::= [0-9a-zA-Z+-.]
    if !dsp.chars().all(is_other_char) {
        return Err(RFC1278ParseError::Malformed);
    }
    let bcd_len = idp.len_in_bytes();
    let outlen = bcd_len + dsp[1..].len();
    if outlen > 20 {
        #[cfg(feature = "alloc")]
        {
            let mut out = Vec::with_capacity(outlen);
            out.extend(idp.as_ref());
            out.extend(
                dsp[1..]
                    .chars()
                    // We check for permitted characters above, so the
                    // unwrap() below should never fail.
                    .map(|c| char_to_local_iso_iec_646_byte(c).unwrap()),
            );
            return Ok(X213NetworkAddress::Heap(out));
        }
        #[cfg(not(feature = "alloc"))]
        {
            return Err(RFC1278ParseError::TooLarge);
        }
    }
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(idp.as_ref());
    for (i, c) in dsp[1..].chars().enumerate() {
        out[bcd_len + i] =
            char_to_local_iso_iec_646_byte(c).map_err(|_| RFC1278ParseError::Malformed)?;
    }
    Ok(X213NetworkAddress::Inline((outlen as u8, out)))
}

fn parse_idp_and_dsp(idp: &str, dsp: &str, syntax: DSPSyntax) -> ParseResult<'static> {
    if !idp[2..].bytes().all(|b| b.is_ascii_digit()) || idp.len() < 2 {
        return Err(RFC1278ParseError::Malformed);
    }
    let afi = decode_afi_from_str(&idp[0..2])?;
    let schema = get_nsap_address_schema(afi).ok_or(RFC1278ParseError::UnrecognizedAFI)?;
    let idi_len_digits: usize = schema.max_idi_len_digits as usize;
    if (idi_len_digits % 2) > 0 && syntax == DSPSyntax::Decimal {
        /* In the encoding specified in ITU-T Rec. X.213, Section A.7, it
        is not clear how to encode decimal DSPs when the first digit
        occupies the last nybble of the IDP's last octet. It is not clear
        if an odd number of hex characters could be used, or if this
        representation is only suitable for binary DSPs. */
        return Err(RFC1278ParseError::SpecificationFailure);
    }
    let mut bcd_buf = BCDBuffer::new();
    bcd_buf.push_byte(afi);
    let idi_pad = if schema.leading_zeroes_in_idi { 1 } else { 0 };
    let mut idi_deficit = idi_len_digits.saturating_sub(idp.len() - 2);
    while idi_deficit > 0 {
        bcd_buf.push_nybble(idi_pad);
        idi_deficit -= 1;
    }
    bcd_buf.push_str(&idp[2..]);
    if (bcd_buf.i % 2) > 0 {
        bcd_buf.push_nybble(0xF);
    }
    let bcd_len = bcd_buf.len_in_bytes();
    let dsp_len = dsp.len() >> 1; // Half it, due to hex encoding
    let len = bcd_len + dsp_len;
    if len > 20 {
        #[cfg(feature = "alloc")]
        {
            let mut hexbytes: Vec<u8> = Vec::with_capacity(dsp_len);
            // We want to tolerate any size, because this could encode a URL NSAP.
            faster_hex::hex_decode(dsp.as_bytes(), &mut hexbytes)
                .map_err(|_| RFC1278ParseError::Malformed)?;
            return Ok(X213NetworkAddress::Heap(hexbytes));
        }
        #[cfg(not(feature = "alloc"))]
        {
            return Err(RFC1278ParseError::TooLarge);
        }
    }
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
    faster_hex::hex_decode(dsp.as_bytes(), &mut out[bcd_len..bcd_len + dsp_len])
        .map_err(|_| RFC1278ParseError::Malformed)?;
    return Ok(X213NetworkAddress::Inline((len as u8, out)));
}

// "RFC-1006" "+" <prefix> "+" <ip> [ "+" <port> [ "+" <tset> ]]
fn parse_rfc_1006_dsp(
    mut bcd_buf: BCDBuffer,
    prefix: &str,
    ip: &str,
    port: Option<&str>,
    tset: Option<&str>,
) -> ParseResult<'static> {
    if prefix.len() != 2 || !prefix.bytes().all(|b| b.is_ascii_digit()) {
        return Err(RFC1278ParseError::Malformed);
    }
    bcd_buf.push_digit_u8(prefix.as_bytes()[0]);
    bcd_buf.push_digit_u8(prefix.as_bytes()[1]);
    let ip = Ipv4Addr::from_str(ip).map_err(|_| {
        #[cfg(feature = "alloc")]
        {
            RFC1278ParseError::ResolveDNSWithName(ip.to_owned())
        }
        #[cfg(not(feature = "alloc"))]
        {
            RFC1278ParseError::ResolveDNS
        }
    })?;
    if port.is_some_and(|p| p.len() == 0) {
        return Err(RFC1278ParseError::Malformed);
    }
    if tset.is_some_and(|t| t.len() == 0) {
        return Err(RFC1278ParseError::Malformed);
    }
    let port = match port {
        Some(p) => Some(u16::from_str(p).map_err(|_| RFC1278ParseError::Malformed)?),
        None => None,
    };
    let tset = match tset {
        Some(t) => Some(u16::from_str(t).map_err(|_| RFC1278ParseError::Malformed)?),
        None => None,
    };

    ip.octets()
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
    let bcd_len = bcd_buf.len_in_bytes();
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
    Ok(X213NetworkAddress::Inline((bcd_len as u8, out)))
}

// "X.25(80)" "+" <prefix> "+" <dte> [ "+" <cudf-or-pid> "+" <hexstring> ]
#[cfg(feature = "x25")]
fn parse_x25_dsp(
    mut bcd_buf: BCDBuffer,
    prefix: &str,
    dte: &str,
    cudf_of_pid: Option<&str>,
    cudf_of_pid_hex: Option<&str>,
) -> ParseResult<'static> {
    if !prefix.bytes().all(|b| b.is_ascii_digit()) || !dte.bytes().all(|b| b.is_ascii_digit()) {
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
        faster_hex::hex_decode(hexstr.as_bytes(), &mut hexout[0..bytelen])
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
    let bcd_len = bcd_buf.len_in_bytes();
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
    Ok(X213NetworkAddress::Inline((bcd_len as u8, out)))
}

// "ECMA-117-Binary" "+" <hexstring> "+" <hexstring> "+" <hexstring>
#[cfg(feature = "ecma117")]
fn parse_ecma117_binary_dsp(
    bcd_buf: BCDBuffer,
    d1: &str,
    d2: &str,
    d3: &str,
) -> ParseResult<'static> {
    if d2.len() > 12 {
        return Err(RFC1278ParseError::Malformed);
    }
    let bcd_len = bcd_buf.len_in_bytes();
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
    faster_hex::hex_decode(d1.as_bytes(), &mut out[bcd_len..bcd_len + 2])
        .map_err(|_| RFC1278ParseError::Malformed)?;
    let d2len = d2.len() >> 1;
    faster_hex::hex_decode(d2.as_bytes(), &mut out[bcd_len + 2..bcd_len + 2 + d2len])
        .map_err(|_| RFC1278ParseError::Malformed)?;
    faster_hex::hex_decode(d3.as_bytes(), &mut out[bcd_len + 2 + d2len..bcd_len + 2 + d2len + 1])
        .map_err(|_| RFC1278ParseError::Malformed)?;
    Ok(X213NetworkAddress::Inline((
        (bcd_len + 2 + d2len + 1) as u8,
        out,
    )))
}

// "ECMA-117-Decimal" "+" <digitstring> "+" <digitstring> "+" <digitstring>
#[cfg(feature = "ecma117")]
fn parse_ecma117_decimal_dsp(
    mut bcd_buf: BCDBuffer,
    d1: &str,
    d2: &str,
    d3: &str,
) -> ParseResult<'static> {
    if d1.len() != 5
        || d2.len() > 15
        || d3.len() != 3
        || !d1.chars().all(|c| c.is_ascii_digit())
        || !d2.chars().all(|c| c.is_ascii_digit())
        || !d3.chars().all(|c| c.is_ascii_digit())
    {
        return Err(RFC1278ParseError::Malformed);
    }
    bcd_buf.push_str(d1);
    bcd_buf.push_str(d2);
    bcd_buf.push_str(d3);
    let bcd_len = bcd_buf.len_in_bytes();
    let mut out: [u8; 22] = [0; 22];
    out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
    Ok(X213NetworkAddress::Inline((bcd_len as u8, out)))
}

pub(crate) fn parse_nsap<'a>(s: &'a str) -> ParseResult<'static> {
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
        return parse_ns_dsp(second_part);
    }
    #[cfg(feature = "nonstd")]
    if first_part == AFI_STR_URL {
        validate_digitstring(second_part, 4)?;
        if parts.next().is_none() {
            return Err(RFC1278ParseError::Malformed);
        }
        // This indexing assumes "URL+" + second part + "+"
        let url = &s[5 + second_part.len()..];
        return parse_url(second_part, url);
    }
    let third_part = parts.next();
    #[cfg(feature = "nonstd")]
    if first_part == IPV6_STR {
        if third_part.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        let ip = Ipv6Addr::from_str(second_part).map_err(|_| RFC1278ParseError::Malformed)?;
        return Ok(X213NetworkAddress::from_ipv6(&ip));
    }
    #[cfg(feature = "nonstd")]
    if first_part == IPV4_STR {
        if third_part.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        let ip = Ipv4Addr::from_str(second_part).map_err(|_| RFC1278ParseError::Malformed)?;
        return Ok(X213NetworkAddress::from_ipv4(&ip));
    }
    let syntax: DSPSyntax = match third_part.and_then(|p3| p3.chars().next()) {
        Some('d') => DSPSyntax::Decimal,
        Some('x') => DSPSyntax::Binary,
        Some('l') => DSPSyntax::IsoIec646Chars,
        _ => {
            if third_part.is_some_and(|p3| p3.starts_with(ECMA_117_BINARY_STR)) {
                DSPSyntax::Binary
            } else {
                // All other encodings are assumed to be decimal.
                // This is true for all of:
                // * RFC-1006+
                // * X.25(80)+
                // * ECMA-117-Decimal+
                DSPSyntax::Decimal
            }
        }
    };
    let maybe_afi = naddr_str_to_afi(first_part, second_part.starts_with("0"), syntax);
    if maybe_afi.is_none() {
        // Otherwise, assume it is <idp> "+" <hexstring>
        if third_part.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        return parse_idp_and_dsp(first_part, second_part, syntax);
    }
    let afi = maybe_afi.unwrap();
    // This MUST be <afi> "+" <idi> [ "+" <dsp> ] syntax.
    let schema = get_nsap_address_schema(afi).ok_or(RFC1278ParseError::UnrecognizedAFI)?;
    let idi_len_digits: usize = schema.max_idi_len_digits as usize;
    if second_part.len() > idi_len_digits || !second_part.bytes().all(|b| b.is_ascii_digit()) {
        return Err(RFC1278ParseError::Malformed);
    }
    let mut bcd_buf = BCDBuffer::new();
    bcd_buf.push_byte(afi);
    let idi_pad = if schema.leading_zeroes_in_idi { 1 } else { 0 };
    let mut idi_deficit = idi_len_digits.saturating_sub(second_part.len());
    while idi_deficit > 0 {
        bcd_buf.push_nybble(idi_pad);
        idi_deficit -= 1;
    }
    bcd_buf.push_str(&second_part);
    if syntax != DSPSyntax::Decimal && (idi_len_digits % 2) > 0 {
        bcd_buf.push_nybble(0x0F);
    }
    if third_part.is_none() {
        let bcd_len = bcd_buf.len_in_bytes();
        let mut out: [u8; 22] = [0; 22];
        out[0..bcd_len].copy_from_slice(bcd_buf.as_ref());
        return Ok(X213NetworkAddress::Inline((bcd_len as u8, out)));
    }
    let third_part = third_part.unwrap();
    if third_part.len() < 2 {
        // Cannot be empty and must have a discriminator (e.g. 'd')
        return Err(RFC1278ParseError::Malformed);
    }
    if third_part == IETF_RFC_1006_PREFIX_STR {
        // "RFC-1006" "+" <prefix> "+" <ip> [ "+" <port> [ "+" <tset> ]]
        let prefix = parts.next();
        let ip = parts.next();
        if prefix.is_none() || ip.is_none() {
            return Err(RFC1278ParseError::Malformed);
        }
        let prefix = prefix.unwrap();
        let ip = ip.unwrap();
        let port = parts.next();
        let tset = parts.next();
        let invalid_part = parts.next(); // Should not exist
        if invalid_part.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        return parse_rfc_1006_dsp(bcd_buf, prefix, ip, port, tset);
    }
    #[cfg(feature = "x25")]
    if third_part == X25_PREFIX_STR {
        // "X.25(80)" "+" <prefix> "+" <dte> [ "+" <cudf-or-pid> "+" <hexstring> ]
        let prefix = parts.next();
        let dte = parts.next();
        let cudf_of_pid = parts.next();
        let cudf_of_pid_hex = parts.next();
        let invalid_part = parts.next(); // Should not exist
        if prefix.is_none()
            || dte.is_none()
            || invalid_part.is_some()
            || (cudf_of_pid.is_some() && cudf_of_pid_hex.is_none())
        {
            return Err(RFC1278ParseError::Malformed);
        }
        let prefix = prefix.unwrap();
        let dte = dte.unwrap();
        return parse_x25_dsp(bcd_buf, prefix, dte, cudf_of_pid, cudf_of_pid_hex);
    }
    #[cfg(feature = "ecma117")]
    if third_part == ECMA_117_BINARY_STR {
        // "ECMA-117-Binary" "+" <hexstring> "+" <hexstring> "+" <hexstring>
        let d1 = parts.next();
        let d2 = parts.next();
        let d3 = parts.next();
        let d4 = parts.next(); // Should not exist
        if d1.is_none() || d2.is_none() || d3.is_none() || d4.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        return parse_ecma117_binary_dsp(bcd_buf, d1.unwrap(), d2.unwrap(), d3.unwrap());
    }
    #[cfg(feature = "ecma117")]
    if third_part == ECMA_117_DECIMAL_STR {
        // "ECMA-117-Decimal" "+" <digitstring> "+" <digitstring> "+" <digitstring>
        let d1 = parts.next();
        let d2 = parts.next();
        let d3 = parts.next();
        let d4 = parts.next(); // Should not exist
        if d1.is_none() || d2.is_none() || d3.is_none() || d4.is_some() {
            return Err(RFC1278ParseError::Malformed);
        }
        return parse_ecma117_decimal_dsp(bcd_buf, d1.unwrap(), d2.unwrap(), d3.unwrap());
    }
    if parts.next().is_some() {
        return Err(RFC1278ParseError::Malformed);
    }
    match third_part.as_bytes()[0] as char {
        'd' => parse_decimal_dsp(bcd_buf, third_part),
        'x' => parse_hexadecimal_dsp(bcd_buf, third_part),
        'l' => parse_textual_dsp(bcd_buf, third_part),
        _ => Err(RFC1278ParseError::UnrecognizedSyntax),
    }
}

#[cfg(test)]
mod tests {

    use core::str::FromStr;

    #[cfg(feature = "nonstd")]
    use crate::AFI_IANA_ICP_BIN;
    use crate::{X213NetworkAddress, data::AFI_ISO_DCC_BIN};

    #[test]
    fn test_from_str() {
        let cases: [(&str, &[u8]); 3] = [
            // Example from RFC 1278
            ("NS+a433bb93c1", &[0xa4, 0x33, 0xbb, 0x93, 0xc1]),
            // Example from RFC 1278
            (
                "X121+234219200300",
                &[0x36, 0x00, 0x23, 0x42, 0x19, 0x20, 0x03, 0x00],
            ),
            // Example from RFC 1278
            (
                "TELEX+00728722+RFC-1006+03+10.0.0.6",
                &[
                    0x54, 0x00, 0x72, 0x87, 0x22, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00,
                    0x06, // 10.0.0.6
                ],
            ),
        ];
        for (case_str, expected) in cases {
            let actual = X213NetworkAddress::from_str(case_str);
            assert!(actual.is_ok(), "failed to parse: {}", case_str);
            let actual = actual.unwrap();
            assert_eq!(expected, actual.get_octets(), "{}", case_str);
        }
    }

    // This test only works if X.25 parsing is enabled
    #[cfg(feature = "x25")]
    #[test]
    fn test_from_str_x25() {
        let cases: [(&str, &[u8]); 1] = [
            // Example from RFC 1278
            // This one deviates from RFC 1278. It seems like it had quotes
            // around the CUDF in error. I am not totally sure.
            (
                "TELEX+00728722+X.25(80)+02+00002340555+CUDF+892796",
                &[
                    0x54, 0x00, 0x72, 0x87, 0x22, 0x02,
                    0x23, // CUDF, which is 3 octets encoded as 9 digits
                    0x13, 0x70, 0x39, 0x15, 0x00, // The last 0 here is for the DTE
                    0x00, 0x02, 0x34, 0x05, 0x55, // All but the first digit of the DTE
                ],
            ),
        ];
        for (case_str, expected) in cases {
            let actual = X213NetworkAddress::from_str(case_str);
            assert!(actual.is_ok(), "failed to parse: {}", case_str);
            let actual = actual.unwrap();
            assert_eq!(expected, actual.get_octets(), "{}", case_str);
        }
    }

    // This test only works if non-standard behavior is enabled
    #[cfg(feature = "nonstd")]
    #[test]
    fn test_from_str_nonstd() {
        let cases: [(&str, &[u8]); 2] = [
            // Non-standard syntax for X.519 URLs
            ("URL+001+https://asdf.com", b"\xFF\x00\x01https://asdf.com"),
            (
                "IP4+192.168.1.100",
                &[
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
                ],
            ),
        ];
        for (case_str, expected) in cases {
            let actual = X213NetworkAddress::from_str(case_str);
            assert!(actual.is_ok(), "failed to parse: {}", case_str);
            let actual = actual.unwrap();
            assert_eq!(expected, actual.get_octets(), "{}", case_str);
        }
    }

    // This test only works if alloc is enabled, usually because the encodings
    // are too long.
    #[cfg(all(feature = "alloc", feature = "nonstd"))]
    #[test]
    fn test_from_str_alloc() {
        let cases: [(&str, &[u8]); 1] = [
            // Non-standard syntax for X.519 URLs
            (
                "URL+001+https://wildboarsoftware.com/x500directory",
                b"\xFF\x00\x01https://wildboarsoftware.com/x500directory",
            ),
        ];
        for (case_str, expected) in cases {
            let actual = X213NetworkAddress::from_str(case_str);
            assert!(actual.is_ok(), "failed to parse: {}", case_str);
            let actual = actual.unwrap();
            assert_eq!(expected, actual.get_octets(), "{}", case_str);
        }
    }

    #[test]
    fn test_parse_too_long_idi() {
        let input = "ICD+23452+x0824";
        assert!(X213NetworkAddress::from_str(input).is_err());
    }

    #[test]
    fn test_parse_idi_with_non_digits() {
        let input = "ICD+23F3+x0824";
        assert!(X213NetworkAddress::from_str(input).is_err());
    }

    #[test]
    fn test_padding_nybble_1() {
        let input = "DCC+840+x0824";
        let addr = X213NetworkAddress::from_str(input).unwrap();
        assert_eq!(
            addr.get_octets(),
            &[AFI_ISO_DCC_BIN, 0x84, 0x0F, 0x08, 0x24,]
        );
    }
}
