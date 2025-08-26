#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use core::str::FromStr;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use core::net::{Ipv4Addr, Ipv6Addr};
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
#[cfg(feature = "alloc")]
use crate::{X213NetworkAddress, DSPSyntax, AFI};
#[cfg(feature = "alloc")]
use crate::bcd::BCDBuffer;
#[cfg(feature = "alloc")]
use crate::data::{
    get_address_type_info,
    AFI_URL,
    AFI_X121_DEC_LEADING_NON_ZERO,
    AFI_X121_DEC_LEADING_ZERO,
    AFI_X121_BIN_LEADING_NON_ZERO,
    AFI_X121_BIN_LEADING_ZERO,
    AFI_ISO_DCC_DEC,
    AFI_ISO_DCC_BIN,
    AFI_F69_DEC_LEADING_NON_ZERO,
    AFI_F69_DEC_LEADING_ZERO,
    AFI_F69_BIN_LEADING_NON_ZERO,
    AFI_F69_BIN_LEADING_ZERO,
    AFI_E163_DEC_LEADING_NON_ZERO,
    AFI_E163_DEC_LEADING_ZERO,
    AFI_E163_BIN_LEADING_NON_ZERO,
    AFI_E163_BIN_LEADING_ZERO,
    AFI_E164_DEC_LEADING_NON_ZERO,
    AFI_E164_DEC_LEADING_ZERO,
    AFI_E164_BIN_LEADING_NON_ZERO,
    AFI_E164_BIN_LEADING_ZERO,
    AFI_ISO_6523_ICD_DEC,
    AFI_ISO_6523_ICD_BIN,
    AFI_IANA_ICP_DEC,
    AFI_IANA_ICP_BIN,
    AFI_ITU_T_IND_DEC,
    AFI_ITU_T_IND_BIN,
    AFI_LOCAL_DEC,
    AFI_LOCAL_BIN,
    AFI_LOCAL_ISO_IEC_646,
    AFI_LOCAL_NATIONAL,
    ECMA_117_BINARY_STR,
    ECMA_117_DECIMAL_STR,
    IETF_RFC_1006_PREFIX_STR,
    X25_PREFIX_STR,
};
#[cfg(feature = "alloc")]
use crate::isoiec646::char_to_local_iso_iec_646_byte;
#[cfg(feature = "alloc")]
use crate::error::RFC1278ParseError;
#[cfg(feature = "alloc")]
use crate::utils::{u16_to_decimal_bytes, u8_to_decimal_bytes};

#[cfg(feature = "alloc")]
pub(crate) type ParseResult<'a> = Result<X213NetworkAddress<'a>, RFC1278ParseError>;

/// Validate that string is a digitstring and shorter than `max_len`
#[cfg(feature = "alloc")]
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
#[cfg(feature = "alloc")]
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
#[cfg(feature = "alloc")]
fn decode_idp_only<'a>(s: &'a str) -> Result<X213NetworkAddress<'static>, RFC1278ParseError> {
    if !s[2..].as_bytes().iter().all(|b| b.is_ascii_digit()) {
        return Err(RFC1278ParseError::Malformed);
    }
    let afi = decode_afi_from_str(&s[0..2])?;
    // If the schema is not known, we cannot construct an NSAP,
    // because we don't know how long the IDI is.
    let schema = get_address_type_info(afi)
        .ok_or(RFC1278ParseError::UnrecognizedAFI)?;
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
    Ok(X213NetworkAddress::Heap(bcd_buf.as_ref().to_vec()))
}

/// Whether the character `c` is an `<other>`, per RFC 1278.
#[cfg(feature = "alloc")]
#[inline]
const fn is_other_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.'
}

/// Translate an AFI string, such as "X121" to an AFI value
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
        crate::data::AFI_STR_ICP => match dsp_syntax {
            DSPSyntax::Decimal => Some(AFI_IANA_ICP_DEC),
            DSPSyntax::Binary => Some(AFI_IANA_ICP_BIN),
            _ => None,
        },
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

#[cfg(feature = "alloc")]
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
        let hexbytes = hex::decode(second_part)
            .map_err(|_| RFC1278ParseError::Malformed)?;
        return Ok(X213NetworkAddress::Heap(hexbytes));
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
        return Ok(X213NetworkAddress::Heap(out));
    }
    if first_part == "IP6" {
        let ip = Ipv6Addr::from_str(second_part)
            .map_err(|_| RFC1278ParseError::Malformed)?;
        let mut out: Vec<u8> = Vec::with_capacity(20);
        out.extend(&[AFI_IANA_ICP_BIN, 0, 0]);
        out.extend(ip.octets().as_slice());
        out.push(0);
        return Ok(X213NetworkAddress::Heap(out));
    }
    if first_part == "IP4" {
        let ip = Ipv4Addr::from_str(second_part)
            .map_err(|_| RFC1278ParseError::Malformed)?;
        let mut out: Vec<u8> = Vec::with_capacity(20);
        out.extend(&[AFI_IANA_ICP_BIN, 0, 1]);
        out.extend(ip.octets().as_slice());
        out.extend([0; 13].as_slice());
        return Ok(X213NetworkAddress::Heap(out));
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
        let idi_len_digits: usize = schema.max_idi_len_digits as usize;
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
            return Ok(X213NetworkAddress::Heap(bcd_buf.as_ref().to_vec()));
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
                Ok(X213NetworkAddress::Heap(bcd_buf.as_ref().to_vec()))
            },
            'x' => {
                let hexbytes = hex::decode(&third_part.as_bytes()[1..])
                    .map_err(|_| RFC1278ParseError::Malformed)?;
                let out = [
                    bcd_buf.as_ref(),
                    hexbytes.as_ref(),
                ].concat();
                Ok(X213NetworkAddress::Heap(out))
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
                Ok(X213NetworkAddress::Heap(out))
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
                    if port.is_some_and(|p| p.len() == 0) {
                        return Err(RFC1278ParseError::Malformed);
                    }
                    if tset.is_some_and(|t| t.len() == 0) {
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
                    return Ok(X213NetworkAddress::Heap(bcd_buf.as_ref().to_vec()));
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
                    return Ok(X213NetworkAddress::Heap(bcd_buf.as_ref().to_vec()));
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
                    return Ok(X213NetworkAddress::Heap(out));
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
                    return Ok(X213NetworkAddress::Heap(bcd_buf.as_ref().to_vec()));
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
    let idi_len_digits: usize = schema.max_idi_len_digits as usize;
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
    return Ok(X213NetworkAddress::Heap(out));
}

#[cfg(all(test, feature = "alloc"))]
mod tests {

    use core::str::FromStr;

    use crate::X213NetworkAddress;
    use crate::AFI_IANA_ICP_BIN;

    #[test]
    fn test_from_str() {
        let cases: [(&str, &[u8]); 6] = [
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
            ("IP4+192.168.1.100", &[
                AFI_IANA_ICP_BIN, 0, 1, 192, 168, 1, 100,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        ];
        for (case_str, expected) in cases {
            let actual = X213NetworkAddress::from_str(case_str);
            assert!(actual.is_ok(), "failed to parse: {}", case_str);
            let actual = actual.unwrap();
            assert_eq!(expected, actual.get_octets(), "{}", case_str);
        }
    }

}
