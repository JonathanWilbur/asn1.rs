use crate::SelectedAttributeTypes::PresentationAddress;
use nsap_address::{X213NetworkAddress, NAddressError};
use std::str::FromStr;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum PresentationAddressDecodeError {
    TooManySelectors,
    MalformedSelector,
    NAddressDecodeError(NAddressError),
}

impl Display for PresentationAddressDecodeError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }

}

impl Error for PresentationAddressDecodeError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PresentationAddressDecodeError::NAddressDecodeError(e) => Some(e),
            _ => None
        }
    }

}

impl From<NAddressError> for PresentationAddressDecodeError {

    fn from(value: NAddressError) -> Self {
        PresentationAddressDecodeError::NAddressDecodeError(value)
    }

}

impl PresentationAddress {

    /// Displays a presentation address according to
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).
    pub fn to_string (&self) -> String {
        if self.nAddresses.len() == 0 {
            return "".into();
        }
        let naddrs_len = self.nAddresses
            .iter()
            .map(|naddr| (naddr.len() << 1) + 50) // +50 for good measure.
            .reduce(|acc, curr| acc + curr)
            .unwrap();
        let psel_len = self.pSelector.as_ref().map(|s| s.len()).unwrap_or(0);
        let ssel_len = self.sSelector.as_ref().map(|s| s.len()).unwrap_or(0);
        let tsel_len = self.tSelector.as_ref().map(|s| s.len()).unwrap_or(0);
        let mut out: String = String::with_capacity(
            naddrs_len
            + (psel_len << 1) + 4
            + (ssel_len << 1) + 4
            + (tsel_len << 1) + 4
            + 10 // Plus ten extra bytes just for good measure.
            // We're trying to make this a single allocation.
        );
        if psel_len > 0 && ssel_len > 0 && tsel_len > 0 {
            let sel = self.pSelector.as_ref().unwrap();
            if sel.iter().all(|b| b.is_ascii_alphanumeric()) {
                out.push('"');
                out.push_str(unsafe { std::str::from_utf8_unchecked(&sel) });
                out.push_str("\"/");
            } else {
                out.push('\'');
                out.push_str(&hex::encode(&sel));
                out.push_str("'H/");
            }
        }
        if ssel_len > 0 && tsel_len > 0 {
            let sel = self.sSelector.as_ref().unwrap();
            if sel.iter().all(|b| b.is_ascii_alphanumeric()) {
                out.push('"');
                out.push_str(unsafe { std::str::from_utf8_unchecked(&sel) });
                out.push_str("\"/");
            } else {
                out.push('\'');
                out.push_str(&hex::encode(&sel));
                out.push_str("'H/");
            }
        }
        if tsel_len > 0 {
            let sel = self.tSelector.as_ref().unwrap();
            if sel.iter().all(|b| b.is_ascii_alphanumeric()) {
                out.push('"');
                out.push_str(unsafe { std::str::from_utf8_unchecked(&sel) });
                out.push_str("\"/");
            } else {
                out.push('\'');
                out.push_str(&hex::encode(&sel));
                out.push_str("'H/");
            }
        }
        let naddr_strs = self.nAddresses.iter()
            .map(|naddr_bytes| {
                match X213NetworkAddress::try_from(naddr_bytes.as_slice()) {
                    Ok(naddr) => naddr.to_string().replace("_", ""),
                    Err(_) => format!("NS+{}", &hex::encode(&naddr_bytes.as_slice())),
                }
            })
            .collect::<Vec<String>>()
            .join("_");
        out.push_str(&naddr_strs);
        out
    }

}

// ::= '"' <otherstring> '"'        -- IA5
// | "#" <digitstring>          -- US GOSIP
// | "'" <hexstring> "'H"       -- Hex
fn parse_selector (s: &str) -> Result<Vec<u8>, ()> {
    debug_assert!(s.len() > 0);
    let first_char = s.chars().nth(0).unwrap();
    match first_char {
        // The US GOSIP Format is documented on pages 23-24 in this document:
        // https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub146-1.pdf
        // It is just a decimal between 1-65535.
        '#' => u16::from_str(&s[1..])
            .map(|i| i.to_be_bytes().to_vec())
            .map_err(|_| ()),
        '"' => {
            if s.len() < 2 {
                return Err(());
            }
            if !s.ends_with("\"") {
                return Err(());
            }
            if !s[1..s.len()-1].as_bytes().iter().all(|b| b.is_ascii()) {
                return Err(());
            }
            Ok(s[1..s.len()-1].as_bytes().to_vec())
        },
        '\'' => {
            if s.len() < 3 {
                return Err(());
            }
            if !s.ends_with("\'H") {
                return Err(());
            }
            hex::decode(&s[1..s.len()-2]).map_err(|_| ())
        },
        _ => Err(()),
    }
}

impl FromStr for PresentationAddress {

    type Err = PresentationAddressDecodeError;

    /// Parses a human-readable string, per the procedures defined in
    /// [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sels: Vec<Vec<u8>> = vec![];
        let mut naddrs: Vec<Vec<u8>> = vec![];
        for substr in s.split("/") {
            let maybe_first_char = substr.chars().nth(0);
            if maybe_first_char.is_none() {
                if sels.len() >= 3 {
                    return Err(PresentationAddressDecodeError::TooManySelectors);
                }
                sels.push(vec![]);
                continue;
            }
            let first_char = maybe_first_char.unwrap();
            let is_selector = substr.len() == 0
                || first_char == '#'
                || first_char == '"'
                || first_char == '\''
                ;
            if is_selector {
                if sels.len() >= 3 {
                    return Err(PresentationAddressDecodeError::TooManySelectors);
                }
                let sel = parse_selector(s)
                    .map_err(|_| PresentationAddressDecodeError::MalformedSelector)?;
                sels.push(sel);
                continue;
            }
            for naddr_str in substr.split("_") {
                let naddr = X213NetworkAddress::from_str(naddr_str)?;
                naddrs.push(naddr.try_into()
                    .map_err(|e| PresentationAddressDecodeError::NAddressDecodeError(e))?);
            }
        }
        let tsel: Option<Vec<u8>> = sels.pop();
        let ssel: Option<Vec<u8>> = sels.pop();
        let psel: Option<Vec<u8>> = sels.pop();
        Ok(PresentationAddress::new(psel, ssel, tsel, naddrs, vec![]))
    }

}

impl PartialEq for PresentationAddress {

    fn eq(&self, other: &Self) -> bool {
        if
            self.pSelector != other.pSelector
            || self.sSelector != other.sSelector
            || self.tSelector != other.tSelector
            || self.nAddresses.len() != other.nAddresses.len()
        {
            return false;
        }
        if self.nAddresses.len() == 1 {
            // No clone or sort required: a performance shortcut.
            return self.nAddresses[0] == other.nAddresses[0];
        }
        let a_sorted = self.nAddresses.clone().sort();
        let b_sorted = other.nAddresses.clone().sort();
        for i in 0..self.nAddresses.len() {
            let naddr_a = &self.nAddresses[i];
            let naddr_b = &other.nAddresses[i];
            if naddr_a != naddr_b {
                return false;
            }
        }
        true
    }

}
