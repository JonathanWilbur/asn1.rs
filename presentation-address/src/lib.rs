#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "x690"), no_std)]
#![allow(non_snake_case)]

#[cfg(feature = "x690")]
pub mod asn1;
use core::fmt::{Display, Write};
use core::iter::{FusedIterator, Iterator};
use core::str::FromStr;
use core::write;
#[cfg(feature = "x690")]
use x690::X690Element;

extern crate alloc;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// An OSI network layer selector
///
/// These are unbounded in length in the abstract, but even in the OSI
/// protocols:
///
/// - The T-selector has a de facto limit of 254 octets (per ITU-T Rec. X.224)
/// - The S-selector has a de jure limit of 16 octets (per ITU-T Rec. X.225)
/// - The P-selector has no explicit limit in size
///
/// NSAPs are restricted by ITU-T Rec. X.213 to 20 octets, but ITU-T Rec. X.519
/// adds an exception for encoding URLs within NSAPs for usage within X.500
/// directories.
///
/// As such, the selectors in this crate are simply represented using `Vec<u8>`.
pub type Selector = Vec<u8>;

/// OSI Presentation Address
///
/// This ASN.1-based data structure comes from ITU-T Recommendation X.520.
///
/// ### ASN.1 Definition:
///
/// ```asn1
/// PresentationAddress ::= SEQUENCE {
///   pSelector   [0]  OCTET STRING OPTIONAL,
///   sSelector   [1]  OCTET STRING OPTIONAL,
///   tSelector   [2]  OCTET STRING OPTIONAL,
///   nAddresses  [3]  SET SIZE (1..MAX) OF OCTET STRING,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct PresentationAddress {
    /// The P-selector: subaddress for the OSI presentation layer
    pub pSelector: Option<Selector>,
    /// The S-selector: subaddress for the OSI session layer
    pub sSelector: Option<Selector>,
    /// The T-selector: subaddress for the OSI transport layer
    pub tSelector: Option<Selector>,
    /// N-addresses: network addresses
    pub nAddresses: Vec<Selector>,
    #[cfg(feature = "x690")]
    pub _unrecognized: Vec<X690Element>,
}

impl PresentationAddress {
    /// Create a new `PresentationAddress`
    #[inline]
    pub fn new(
        pSelector: Option<Selector>,
        sSelector: Option<Selector>,
        tSelector: Option<Selector>,
        nAddresses: Vec<Selector>,
        #[cfg(feature = "x690")] _unrecognized: Vec<X690Element>,
    ) -> Self {
        PresentationAddress {
            pSelector,
            sSelector,
            tSelector,
            nAddresses,
            #[cfg(feature = "x690")]
            _unrecognized,
        }
    }

    /// Returns `true` if `self` has the same selectors and a subset of N-addresses of `other`
    ///
    /// Note that the ordering of N-addresses does not matter.
    ///
    /// In the naming of this function, the term "naively" is used to mean that
    /// N-addresses are compared naively: byte-for-byte. This isn't totally
    /// accurate, since the same underlying network address could be represented
    /// in multiple ways in some cases. This is, in part, why `PartialEq` or
    /// `Eq` is not implemented for `PresentationAddress.`
    pub fn is_naively_subset_of(&self, other: &Self) -> bool {
        if self.pSelector.as_ref() != other.pSelector.as_ref() {
            return false;
        }
        if self.sSelector.as_ref() != other.sSelector.as_ref() {
            return false;
        }
        if self.tSelector.as_ref() != other.tSelector.as_ref() {
            return false;
        }
        if self.nAddresses.len() == 0 {
            return false;
        }
        // It cannot be a subset of the other if the other has fewer N-addresses
        if self.nAddresses.len() > other.nAddresses.len() {
            // Empty PresentationAddresses should match nothing.
            return false;
        }
        let othern = BTreeSet::from_iter(other.nAddresses.iter());
        for naddr in self.nAddresses.iter() {
            if !othern.contains(naddr) {
                return false;
            }
        }
        true
    }

    /// Returns `true` if `self` has the same selectors and the same N-addresses of `other`
    ///
    /// Note that the ordering of N-addresses does not matter.
    ///
    /// In the naming of this function, the term "naively" is used to mean that
    /// N-addresses are compared naively: byte-for-byte. This isn't totally
    /// accurate, since the same underlying network address could be represented
    /// in multiple ways in some cases. This is, in part, why `PartialEq` or
    /// `Eq` is not implemented for `PresentationAddress.`
    pub fn is_naively_exactly(&self, other: &Self) -> bool {
        if self.pSelector.as_ref() != other.pSelector.as_ref() {
            return false;
        }
        if self.sSelector.as_ref() != other.sSelector.as_ref() {
            return false;
        }
        if self.tSelector.as_ref() != other.tSelector.as_ref() {
            return false;
        }
        if self.nAddresses.len() == 0 {
            // Empty PresentationAddresses should match nothing.
            return false;
        }
        // It cannot be a subset of the other if the other has fewer N-addresses
        if self.nAddresses.len() != other.nAddresses.len() {
            return false;
        }
        let selfn = BTreeSet::from_iter(self.nAddresses.iter());
        let mut othern = BTreeSet::from_iter(other.nAddresses.iter());
        // This prevents duplicates from being a problem.
        if selfn.len() != othern.len() {
            return false;
        }
        for naddr in selfn.iter() {
            if !othern.remove(naddr) {
                return false;
            }
        }
        true
    }
}

fn sel_str_to_bytes(s: &str) -> Result<Vec<u8>, ()> {
    // A string with a length of one cannot be a valid selector string.
    if s.len() == 1 {
        return Err(());
    }
    match s.chars().next() {
        Some('"') => {
            // string of <other>
            if let Some(end_quote_idx) = s[1..].find('"') {
                let inner_str = &s[1..=end_quote_idx];
                Ok(inner_str.as_bytes().to_vec())
            } else {
                Err(())
            }
        }
        Some('\'') => {
            // hexstring
            if let Some(end_quote_idx) = s[1..].find('\'') {
                let inner_str = &s[1..=end_quote_idx];
                let bytelen = inner_str.len() >> 1;
                let mut bytes: Vec<u8> = Vec::with_capacity(bytelen);
                unsafe {
                    bytes.set_len(bytelen);
                }
                faster_hex::hex_decode(inner_str.as_bytes(), bytes.as_mut_slice())
                    .map_err(|_| ())?;
                Ok(bytes)
            } else {
                Err(())
            }
        }
        Some('#') => {
            // u16
            let sel = u16::from_str(&s[1..]).map_err(|_| ())?;
            Ok(sel.to_be_bytes().to_vec())
        }
        None => Ok(Vec::new()),
        _ => Err(()),
    }
}

fn naddr_str_to_bytes(s: &str) -> Result<Vec<u8>, ()> {
    #[cfg(feature = "nsap-address")]
    {
        let nsap = nsap_address::X213NetworkAddress::from_str(s).map_err(|_| ())?;
        Ok(nsap.get_octets().to_vec())
    }
    #[cfg(not(feature = "nsap-address"))]
    {
        if s.starts_with("NS+") {
            let bytelen = s[3..].len() >> 1;
            let mut bytes: Vec<u8> = Vec::with_capacity(bytelen);
            unsafe {
                bytes.set_len(bytelen);
            }
            faster_hex::hex_decode(s[3..].as_bytes(), bytes.as_mut_slice()).map_err(|_| ())?;
            Ok(bytes)
        } else {
            // No other syntaxes supported
            Err(())
        }
    }
}

impl FromStr for PresentationAddress {
    type Err = ();

    /// Parses the `PresentationAddress` according to
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut selectors = RFC1278SelectorIterator::new(s);
        let s1 = selectors.next();
        let s2 = selectors.next();
        let s3 = selectors.next();
        debug_assert!(selectors.next().is_none());
        let (psel, ssel, tsel) = if s3.is_some() {
            (s1, s2, s3)
        } else if s2.is_some() {
            (None, s1, s2)
        } else if s1.is_some() {
            (None, None, s1)
        } else {
            (None, None, None)
        };

        let psel = match psel {
            Some(sel) => Some(sel_str_to_bytes(sel)?),
            None => None,
        };
        let ssel = match ssel {
            Some(sel) => Some(sel_str_to_bytes(sel)?),
            None => None,
        };
        let tsel = match tsel {
            Some(sel) => Some(sel_str_to_bytes(sel)?),
            None => None,
        };

        let nsaps_len = selectors.remainder().split('_').count();
        let mut nsaps: Vec<Vec<u8>> = Vec::with_capacity(nsaps_len + 1);
        for nsap in selectors.remainder().split('_') {
            nsaps.push(naddr_str_to_bytes(nsap)?);
        }
        #[cfg(feature = "x690")]
        {
            Ok(PresentationAddress::new(psel, ssel, tsel, nsaps, vec![]))
        }
        #[cfg(not(feature = "x690"))]
        {
            Ok(PresentationAddress::new(psel, ssel, tsel, nsaps))
        }
    }
}

// <selector>  ::= '"' <otherstring> '"'        -- IA5
//                 | "#" <digitstring>          -- US GOSIP            40
//                 | "'" <hexstring> "'H"       -- Hex
//                 | ""                         -- Empty but present
fn print_selector(sel: &[u8], f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let len = sel.len();
    /*
    Rationale:

    We check that len is greater than two before deciding to print as IA5,
    because:
    1. If the len is 2 or less, it is unlikely that the bytes actually encode
       a real ASCII. On the other hand, there is less than a 1% chance that all
       three or more characters are <other> by random chance.
    2. Two bytes are used in US GOSIP to encode selectors as an unsigned
       integer, so this prevents these selectors from being occassionally
       misrepresented in strings as ASCII.

    We check that the string is less than or equal to 16 bytes, because it is
    unlikely that an ASCII string longer than 16 bytes won't contain some
    non-<other> character, which would invalidate our requirements for printing.
    Sixteen bytes is the size limit for selectors in the ITU-T Rec. X.225 OSI
    Session protocol.
    */
    if len > 2 && len <= 16 && sel.iter().all(|b| is_other_char(*b as char)) {
        let selstr = unsafe { str::from_utf8_unchecked(sel) };
        f.write_char('"')?;
        f.write_str(selstr)?;
        return f.write_char('"');
    }
    f.write_char('\'')?;
    for byte in sel {
        write!(f, "{:02X}", *byte)?;
    }
    f.write_char('\'')?;
    f.write_char('H')
}

fn print_ns_string(naddr: &[u8], f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("NS+")?;
    for byte in naddr {
        f.write_fmt(format_args!("{:02X}", *byte))?;
    }
    Ok(())
}

fn print_naddr(naddr: &[u8], f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    #[cfg(feature = "nsap-address")]
    {
        match nsap_address::X213NetworkAddress::try_from(naddr) {
            Ok(nsap) => nsap.fmt(f),
            Err(_) => print_ns_string(naddr, f),
        }
    }
    #[cfg(not(feature = "nsap-address"))]
    {
        print_ns_string(naddr, f)
    }
}

impl Display for PresentationAddress {
    /// Displays the `PresentationAddress` according to
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278)
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut print_selectors: bool = false;
        if let Some(sel) = self.pSelector.as_deref() {
            print_selector(sel, f)?;
            f.write_char('/')?;
            print_selectors = true;
        }
        if let Some(sel) = self.sSelector.as_deref() {
            print_selector(sel, f)?;
            f.write_char('/')?;
            print_selectors = true;
        } else if print_selectors {
            f.write_char('/')?;
        }
        if let Some(sel) = self.tSelector.as_deref() {
            print_selector(sel, f)?;
            f.write_char('/')?;
        } else if print_selectors {
            f.write_char('/')?;
        }
        for naddr in self.nAddresses.iter() {
            print_naddr(naddr.as_slice(), f)?;
        }
        Ok(())
    }
}

/// Whether the character `c` is an `<other>`, per RFC 1278.
#[inline]
const fn is_other_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.'
}

/// Iterator over the selectors of an IETF RFC 1278 string
///
/// This iterator exists because the NSAPs could have forward slashes in them,
/// so splitting the presentation address by forward slashes. Technically, this
/// isn't supposed to happen in the syntax given by IETF RFC 1278, but the
/// nsap-address library supports non-standard URL syntaxes, and with the
/// ability to print URLs, the ability to handle forward slashes in the DSP of
/// NSAPs becomes more important.
pub struct RFC1278SelectorIterator<'a> {
    s: &'a str,
    selectors_read: u8,
}

impl<'a> RFC1278SelectorIterator<'a> {
    /// Create a new `RFC1278SelectorIterator`
    pub fn new(s: &'a str) -> Self {
        RFC1278SelectorIterator {
            s,
            selectors_read: 0,
        }
    }

    /// Get the remainder of the string not yet parsed
    ///
    /// This can be used to obtain the network addresses part of the
    /// presentation address: just iterate over all selectors and the string
    /// that remains is the network addresses part.
    pub fn remainder(&self) -> &'a str {
        self.s
    }
}

// <selector>  ::= '"' <otherstring> '"'        -- IA5
//                                              -- For chars not in this
//                                              -- string use hex
//                 | "#" <digitstring>          -- US GOSIP            40
//                 | "'" <hexstring> "'H"       -- Hex
//                 | ""                         -- Empty but present
impl<'a> Iterator for RFC1278SelectorIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.selectors_read >= 3 {
            return None;
        }
        let first_char = self.s.chars().next()?;
        match first_char {
            '\'' | '"' | '#' => {
                // It is fine to split by forward slash, because
                let (sel, rest) = self.s.split_once('/')?;
                self.s = rest;
                self.selectors_read += 1;
                Some(sel)
            }
            // We are at the start of the Network addresses now. All done.
            _ => None,
        }
    }
}

impl<'a> FusedIterator for RFC1278SelectorIterator<'a> {}

#[cfg(test)]
mod tests {

    use core::str::FromStr;

    extern crate alloc;
    use alloc::string::ToString;
    use alloc::vec;

    use super::{PresentationAddress, RFC1278SelectorIterator};

    #[test]
    fn test_from_sel_iter() {
        let input = if cfg!(feature = "nsap-address") {
            "'01020304'H/\"HIMOM\"/#65535/TELEX+00728722+RFC-1006+03+10.0.0.6+9+2"
        } else {
            "'01020304'H/\"HIMOM\"/#65535/NS+5400728722030100000000060000900002"
        };
        let mut seliter = RFC1278SelectorIterator::new(input);
        let s1 = seliter.next();
        let s2 = seliter.next();
        let s3 = seliter.next();
        let s4 = seliter.next();
        let s5 = seliter.next();
        assert_eq!(s1, Some("'01020304'H"));
        assert_eq!(s2, Some("\"HIMOM\""));
        assert_eq!(s3, Some("#65535"));
        assert_eq!(s4, None);
        assert_eq!(s5, None);
    }

    #[test]
    fn test_from_str_01() {
        let input = if cfg!(feature = "nsap-address") {
            "'01020304'H/\"HIMOM\"/#65534/TELEX+00728722+RFC-1006+03+10.0.0.6+9+2"
        } else {
            "'01020304'H/\"HIMOM\"/#65534/NS+5400728722030100000000060000900002"
        };
        let paddr = PresentationAddress::from_str(input).unwrap();
        assert_eq!(
            paddr.pSelector.unwrap().as_slice(),
            [1u8, 2, 3, 4,].as_slice()
        );
        assert_eq!(paddr.sSelector.unwrap().as_slice(), b"HIMOM");
        assert_eq!(
            paddr.tSelector.unwrap().as_slice(),
            65534u16.to_be_bytes().as_slice()
        );
        assert_eq!(paddr.nAddresses.len(), 1);
        assert_eq!(
            paddr.nAddresses[0].as_slice(),
            &[
                // TELEX+00728722+RFC-1006+03+10.0.0.6+9+2
                0x54, 0x00, 0x72, 0x87, 0x22, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00,
                0x90, 0x00, 0x02,
            ]
        );
    }

    #[test]
    fn test_from_str_02() {
        let input = if cfg!(feature = "nsap-address") {
            "'01020304'H/\"HIMOM\"/#65534/TELEX+00728722+RFC-1006+03+10.0.0.6+9+2_NS+FF00013132333435"
        } else {
            "'01020304'H/\"HIMOM\"/#65534/NS+5400728722030100000000060000900002_NS+FF00013132333435"
        };
        let paddr = PresentationAddress::from_str(input).unwrap();
        assert_eq!(
            paddr.pSelector.unwrap().as_slice(),
            [1u8, 2, 3, 4,].as_slice()
        );
        assert_eq!(paddr.sSelector.unwrap().as_slice(), b"HIMOM");
        assert_eq!(
            paddr.tSelector.unwrap().as_slice(),
            65534u16.to_be_bytes().as_slice()
        );
        assert_eq!(paddr.nAddresses.len(), 2);
        assert_eq!(
            paddr.nAddresses[0].as_slice(),
            &[
                // TELEX+00728722+RFC-1006+03+10.0.0.6+9+2
                0x54, 0x00, 0x72, 0x87, 0x22, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00,
                0x90, 0x00, 0x02,
            ]
        );
        assert_eq!(
            paddr.nAddresses[1].as_slice(),
            &[0xFF, 0x00, 0x01, 0x31, 0x32, 0x33, 0x34, 0x35]
        );
    }

    #[test]
    fn test_display_01() {
        let paddr = PresentationAddress::new(
            Some(vec![1, 2, 3, 4]),
            Some(b"HIMOM".to_vec()),
            Some(vec![0xFF, 0xFE]),
            vec![
                // TELEX+00728722+RFC-1006+03+10.0.0.6+9+2
                vec![
                    0x54, 0, 0x72, 0x87, 0x22, 3, 1, 0, 0, 0, 0, 6, 0, 0, 0x90, 0, 2,
                ],
            ],
            #[cfg(feature = "x690")]
            vec![],
        );
        let expected = if cfg!(feature = "nsap-address") {
            "'01020304'H/\"HIMOM\"/'FFFE'H/TELEX+00728722+RFC-1006+03+10.0.0.6+9+2"
        } else {
            "'01020304'H/\"HIMOM\"/'FFFE'H/NS+5400728722030100000000060000900002"
        };
        assert_eq!(paddr.to_string().as_str(), expected);
    }
}
