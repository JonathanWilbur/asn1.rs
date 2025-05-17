use crate::{types::OBJECT_IDENTIFIER, OID_ARC, RELATIVE_OID};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

impl RELATIVE_OID {
    #[inline]
    pub fn new(nodes: &[OID_ARC]) -> Self {
        RELATIVE_OID(Vec::from(nodes))
    }

    #[inline]
    pub fn to_asn1_string(&self) -> String {
        format!(
            "{{ {} }}",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    #[inline]
    pub fn to_iri_string(&self) -> String {
        format!(
            "/{}",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }

    #[inline]
    pub fn extend(&mut self, roid: &RELATIVE_OID) -> () {
        self.0.extend(roid.0.as_slice())
    }

    #[inline]
    pub fn starts_with(&mut self, roid: &RELATIVE_OID) -> bool {
        self.0.starts_with(roid.0.as_slice())
    }

    #[inline]
    pub fn ends_with(&mut self, roid: &RELATIVE_OID) -> bool {
        self.0.ends_with(roid.0.as_slice())
    }
}

impl FromStr for RELATIVE_OID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<u32> = Vec::with_capacity(s.len());
        for str in s.split(".") {
            nodes.push(str.parse::<u32>()?);
        }
        Ok(RELATIVE_OID(nodes))
        // TODO: Pending try_collect() stabilization (and it benchmarking favorably.)
        // Ok(OBJECT_IDENTIFIER(value
        //     .split(".")
        //     .map(|s| s.parse::<u32>())
        //     .try_collect::<Vec<u32>>()?))
    }
}

impl Display for RELATIVE_OID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(".")
                .as_str(),
        )
    }
}

impl IntoIterator for RELATIVE_OID {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for RELATIVE_OID {
    #[inline]
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

// impl From<OBJECT_IDENTIFIER> for RELATIVE_OID {
//     #[inline]
//     fn from(oid: OBJECT_IDENTIFIER) -> Self {
//         RELATIVE_OID(oid.0.clone())
//     }
// }

#[macro_export]
macro_rules! roid {
    ( $( $x:expr ),* ) => {
        {
            use super::RELATIVE_OID;
            RELATIVE_OID::new(&[ $($x,)* ])
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roid_parsing () {
        let roid1 = RELATIVE_OID::from_str("1.3.6.4.1").unwrap();
        let roid2 = roid!(1,3,6,4,1);
        assert_eq!(roid1, roid2);
    }

    #[test]
    fn test_roid_macro () {
        let roid1 = roid!(1,3,6,4,1);
        assert_eq!(roid1.to_string(), "1.3.6.4.1");
    }

}
