use core::fmt::{Display, Write};
use core::net::{Ipv4Addr, Ipv6Addr};
use crate::{X213NetworkAddress, X213NetworkAddressType, DSPSyntax};
use crate::data::{
    INTERNET_PREFIX,
    AFI_URL,
    AFI_IANA_ICP_BIN,
    AFI_STR_X121,
    AFI_STR_DCC,
    AFI_STR_TELEX,
    AFI_STR_PSTN,
    AFI_STR_ISDN,
    AFI_STR_ICD,
    AFI_STR_ICP,
    AFI_STR_IND,
    AFI_STR_LOCAL,
    AFI_STR_URL,
    IANA_ICP_IDI_IPV4,
    IANA_ICP_IDI_IPV6,
};
use crate::isoiec646::local_iso_iec_646_byte_to_char;

const DEFAULT_ITOT_PORT: u16 = 102;

/// Convert the network type to a string
#[inline]
pub const fn naddr_network_type_to_str (nt: X213NetworkAddressType) -> &'static str {
    match nt {
        X213NetworkAddressType::X121 => AFI_STR_X121,
        X213NetworkAddressType::ISO_DCC => AFI_STR_DCC,
        X213NetworkAddressType::F69 => AFI_STR_TELEX,
        X213NetworkAddressType::E163 => AFI_STR_PSTN,
        X213NetworkAddressType::E164 => AFI_STR_ISDN,
        X213NetworkAddressType::ISO_6523_ICD => AFI_STR_ICD,
        X213NetworkAddressType::IANA_ICP => AFI_STR_ICP, // Not specified in IETF RFC 1278. See: https://www.iana.org/assignments/osi-nsapa-numbers/osi-nsapa-numbers.xhtml
        X213NetworkAddressType::ITU_T_IND => AFI_STR_IND, // Not specified in IETF RFC 1278.
        X213NetworkAddressType::LOCAL => AFI_STR_LOCAL,
        X213NetworkAddressType::URL => AFI_STR_URL, // Not specified in IETF RFC 1278.
    }
}

#[inline]
pub(crate) fn fmt_naddr_type(
    t: &X213NetworkAddressType,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    f.write_str(naddr_network_type_to_str(*t))
}

/// Decode a BCD-encoded IPv4 address. Returns `None` upon integer overflow.
fn ipv4_from_slice(bytes: &[u8]) -> Option<Ipv4Addr> {
    debug_assert_eq!(bytes.len(), 6);
    let oct1: u32 =
          (((bytes[0] & 0xF0) >> 4) as u32 * 100)
        + (((bytes[0] & 0x0F) >> 0) as u32 * 10)
        + (((bytes[1] & 0xF0) >> 4) as u32 * 1)
        ;
    let oct2: u32 =
          (((bytes[1] & 0x0F) >> 0) as u32 * 100)
        + (((bytes[2] & 0xF0) >> 4) as u32 * 10)
        + (((bytes[2] & 0x0F) >> 0) as u32 * 1)
        ;
    let oct3: u32 =
          (((bytes[3] & 0xF0) >> 4) as u32 * 100)
        + (((bytes[3] & 0x0F) >> 0) as u32 * 10)
        + (((bytes[4] & 0xF0) >> 4) as u32 * 1)
        ;
    let oct4: u32 =
          (((bytes[4] & 0x0F) >> 0) as u32 * 100)
        + (((bytes[5] & 0xF0) >> 4) as u32 * 10)
        + (((bytes[5] & 0x0F) >> 0) as u32 * 1)
        ;
    let oct1: u8 = oct1.try_into().ok()?;
    let oct2: u8 = oct2.try_into().ok()?;
    let oct3: u8 = oct3.try_into().ok()?;
    let oct4: u8 = oct4.try_into().ok()?;
    Some(Ipv4Addr::new(oct1, oct2, oct3, oct4))
}

const DEFAULT_ITOT_TRANSPORT_SET: u16 = 1;

pub(crate) fn fmt_naddr(
    naddr: &X213NetworkAddress<'_>,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    let octets = naddr.get_octets();
    match octets.get(0..3) {
        Some(octs) if octs[0] == AFI_URL => {
            if let Ok(url) = str::from_utf8(&octets[3..]) {
                if !url.contains('_') {
                    return write!(f, "URL+{:02X}{:02X}+{}", octs[1], octs[2], url);
                }
            }
        },
        _ => (),
    };
    if octets[0] == AFI_IANA_ICP_BIN && octets.len() == 20 {
        let icp = &octets[1..3];
        if icp == IANA_ICP_IDI_IPV6.as_slice() {
            let ip = Ipv6Addr::from([
                octets[3],  octets[4],  octets[5],  octets[6],
                octets[7],  octets[8],  octets[9],  octets[10],
                octets[11], octets[12], octets[13], octets[14],
                octets[15], octets[16], octets[17], octets[18],
            ]);
            return write!(f, "IP6+{}", ip);
        }
        if icp == IANA_ICP_IDI_IPV4.as_slice() {
            let ip = Ipv4Addr::from([
                octets[3],
                octets[4],
                octets[5],
                octets[6],
            ]);
            return write!(f, "IP4+{}", ip);
        }
    }
    if octets.starts_with(INTERNET_PREFIX.as_slice())
        && octets.len() >= INTERNET_PREFIX.len() + 6 {
        let ip_and_stuff = &octets[INTERNET_PREFIX.len()..];
        let ip = ipv4_from_slice(&ip_and_stuff[0..6]);
        let port: u32 = if octets.len() >= INTERNET_PREFIX.len() + 6 + 3 {
                (((ip_and_stuff[6] & 0xF0) >> 4) as u32 * 10000)
            + (((ip_and_stuff[6] & 0x0F) >> 0) as u32 * 1000)
            + (((ip_and_stuff[7] & 0xF0) >> 4) as u32 * 100)
            + (((ip_and_stuff[7] & 0x0F) >> 0) as u32 * 10)
            + (((ip_and_stuff[8] & 0xF0) >> 4) as u32 * 1)
        } else {
            DEFAULT_ITOT_PORT as u32
        };
        let tset: u32 = if octets.len() >= INTERNET_PREFIX.len() + 6 + 5 {
                (((ip_and_stuff[8]  & 0x0F) >> 0) as u32 * 10000)
            + (((ip_and_stuff[9]  & 0xF0) >> 4) as u32 * 1000)
            + (((ip_and_stuff[9]  & 0x0F) >> 0) as u32 * 100)
            + (((ip_and_stuff[10] & 0xF0) >> 4) as u32 * 10)
            + (((ip_and_stuff[10] & 0x0F) >> 0) as u32 * 1)
        } else {
            DEFAULT_ITOT_TRANSPORT_SET as u32
        };
        let port: u16 = port.try_into().unwrap_or(0);
        let tset: u16 = tset.try_into().unwrap_or(DEFAULT_ITOT_TRANSPORT_SET);
        if let Some(ip) = ip {
            write!(f, "TELEX+00728722+RFC-1006+03+{}", ip)?;
            if port != DEFAULT_ITOT_PORT {
                write!(f, "+{}", port)?;
            }
            if tset != DEFAULT_ITOT_TRANSPORT_SET {
                write!(f, "+{}", tset)?;
            }
            return Ok(());
        }
    }
    let (info, idi_digits) = match (naddr.get_network_type_info(), naddr.idi_digits()) {
        (Some(i), Some(d)) => (i, d),
        _ => { // If unrecognized, just print in NS+<hex> format
            let h = hex::encode(octets);
            f.write_str("NS+")?;
            return f.write_str(h.as_str());
        }
    };
    info.network_type.fmt(f)?;
    f.write_char('+')?;
    for digit in idi_digits {
        f.write_char((digit + 0x30) as char)?;
    }
    f.write_char('+')?;
    let idi_len = info.max_idi_len_digits as usize;
    let idi_len_in_bytes: usize = idi_len >> 1;
    match info.dsp_syntax {
        DSPSyntax::Decimal => {
            match naddr.dsp_digits() {
                Some(dsp_digits) => {
                    f.write_char('d')?;
                    for digit in dsp_digits {
                        f.write_char((digit + 0x30).into())?;
                    }
                },
                None => { // This shouldn't happen
                    f.write_char('x')?;
                    match naddr.get_octets().get(1+idi_len_in_bytes..) {
                        Some(dsp) => {
                            for byte in dsp {
                                f.write_fmt(format_args!("{:02X}", *byte))?;
                            }
                        },
                        None => (),
                    };

                },
            };
        },
        DSPSyntax::Binary | DSPSyntax::NationalChars => {
            // FIXME: Make this part of get_schema
            let dsp = &octets[1+idi_len_in_bytes..];
            f.write_char('x')?;
            for byte in dsp {
                f.write_fmt(format_args!("{:02X}", *byte))?;
            }
        },
        DSPSyntax::IsoIec646Chars => {
            let dsp = &octets[1+idi_len_in_bytes..];
            let decode = dsp
                .iter()
                .map(|b| local_iso_iec_646_byte_to_char(*b).unwrap_or('?'));
            for c in decode {
                f.write_char(c)?;
            }
        },
    };
    Ok(())
}

#[cfg(test)]
mod tests {

    extern crate alloc;
    use alloc::string::ToString;
    use super::{X213NetworkAddress, AFI_IANA_ICP_BIN};

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
    fn test_display_02_itot() {
        let input = &[ 0x54, 0, 0x72, 0x87, 0x22, 3, 1, 0, 0, 0, 0, 6, 0, 0, 0x90, 0, 2 ];
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "TELEX+00728722+RFC-1006+03+10.0.0.6+9+2");
    }

    #[test]
    fn test_display_03_ip() {
        let input = &[
            AFI_IANA_ICP_BIN, 0, 1, 192, 168, 1, 100,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let addr = X213NetworkAddress::try_from(input.as_slice()).unwrap();
        let addr_str = addr.to_string();
        assert_eq!(addr_str, "IP4+192.168.1.100");
    }

}
