use crate::{types::OBJECT_IDENTIFIER, OID_ARC, RELATIVE_OID};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

impl OBJECT_IDENTIFIER {
    pub fn new(nodes: Vec<OID_ARC>) -> Self {
        OBJECT_IDENTIFIER(nodes)
    }

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
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<u32> = Vec::with_capacity(s.len());
        for str in s.split(".") {
            nodes.push(str.parse::<u32>()?);
        }
        Ok(OBJECT_IDENTIFIER(nodes))
    }
}

impl Display for OBJECT_IDENTIFIER {
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
            use super::OBJECT_IDENTIFIER;
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
