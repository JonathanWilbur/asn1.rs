use atoi_simd::AtoiSimdError;

use crate::{types::OBJECT_IDENTIFIER, unlikely, OID_ARC, RELATIVE_OID};
use std::{fmt::{Display, Write}, num::{IntErrorKind, ParseIntError}, str::FromStr};

impl OBJECT_IDENTIFIER {
    pub fn new(nodes: Vec<OID_ARC>) -> Self {
        OBJECT_IDENTIFIER(nodes)
    }

    // TODO: Dedupe from ROID
    pub fn to_asn1_string(&self) -> String {
        // I don't think there's really a much more performant way to implement
        // this. itoa is not very helpful here, because we have to clone the
        // stack buffer into an owned string anyway.
        format!(
            "{{ {} }}",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    pub fn to_iri_string(&self) -> String {
        // I don't think there's really a much more performant way to implement
        // this. itoa is not very helpful here, because we have to clone the
        // stack buffer into an owned string anyway.
        format!(
            "/{}",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }

    pub fn extend(&mut self, roid: RELATIVE_OID) -> () {
        self.0.extend(roid.0)
    }

    pub fn starts_with(&mut self, roid: &RELATIVE_OID) -> bool {
        self.0.starts_with(roid.0.as_slice())
    }

    pub fn ends_with(&mut self, roid: &RELATIVE_OID) -> bool {
        self.0.ends_with(roid.0.as_slice())
    }
}

// TODO: Ordering matching rules?
// TODO: Iterator

impl FromStr for OBJECT_IDENTIFIER {
    type Err = (); // TODO: More detailed error type? Overflow, empty, etc.

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<u32> = Vec::with_capacity(s.len());
        for arc_string in s.split(".") {
            if cfg!(feature = "atoi_simd") {
                let arc = atoi_simd::parse::<u32>(arc_string.as_bytes()).map_err(|_| ())?;
                nodes.push(arc);
            } else {
                nodes.push(arc_string.parse::<u32>().map_err(|_| ())?);
            }
        }
        Ok(OBJECT_IDENTIFIER(nodes))
    }
}

// TODO: Dedupe by converting into ROID or vice versa
impl Display for OBJECT_IDENTIFIER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if unlikely(self.0.len() == 0) {
            return Ok(());
        }
        if cfg!(feature = "itoa") {
            let mut buf1 = itoa::Buffer::new();
            f.write_str(buf1.format(self.0[0]))?;
        } else {
            f.write_str(self.0[0].to_string().as_str())?;
        }
        for arc in self.0[1..].iter() {
            f.write_char('.')?;
            if cfg!(feature = "itoa") {
                let mut buf1 = itoa::Buffer::new();
                f.write_str(buf1.format(*arc))?;
            } else {
                f.write_str(arc.to_string().as_str())?;
            }
        }
        Ok(())
    }
}

impl IntoIterator for OBJECT_IDENTIFIER {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for OBJECT_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        // We use .ends_with() as a performance hack.
        // The first arc can only have 3 values. The
        // last arc can take on any value. That means
        // going in reverse is more likely to find a
        // non-match and short-circuit.
        self.0.ends_with(other.0.as_slice())
    }
}


#[macro_export]
macro_rules! oid {
    ( $( $x:expr ),* ) => {
        {
            use $crate::OBJECT_IDENTIFIER;
            OBJECT_IDENTIFIER::new(Vec::from([ $($x,)* ]))
        }
    };
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::OBJECT_IDENTIFIER;

    #[test]
    fn test_oid_parsing () {
        let oid1 = OBJECT_IDENTIFIER::from_str("1.3.6.4.1").unwrap();
        let oid2 = oid!(1,3,6,4,1);
        assert_eq!(oid1, oid2);
    }

    #[test]
    fn test_oid_macro () {
        let oid1 = oid!(1,3,6,4,1);
        assert_eq!(oid1.to_string(), "1.3.6.4.1");
    }

}
