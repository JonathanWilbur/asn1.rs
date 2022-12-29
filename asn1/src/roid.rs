use std::{num::ParseIntError, str::FromStr, fmt::Display};
use crate::{types::OBJECT_IDENTIFIER, OID_ARC, RELATIVE_OID};

impl RELATIVE_OID {

    pub fn new (nodes: &[OID_ARC]) -> Self {
        RELATIVE_OID(Vec::from(nodes))
    }

    pub fn to_asn1_string (&self) -> String {
        format!("{{ {} }}", self.0
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" "))
    }

    pub fn to_iri_string (&self) -> String {
        format!("/{}", self.0
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("/"))
    }

    pub fn extend (&mut self, roid: &RELATIVE_OID) -> () {
        self.0.extend(roid.0.as_slice())
    }

    pub fn starts_with (&mut self, roid: &RELATIVE_OID) -> bool {
        self.0.starts_with(roid.0.as_slice())
    }

    pub fn ends_with (&mut self, roid: &RELATIVE_OID) -> bool {
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
        // Pending try_collect() stabilization.
        // Ok(OBJECT_IDENTIFIER(value
        //     .split(".")
        //     .map(|s| s.parse::<u32>())
        //     .try_collect::<Vec<u32>>()?))
    }

}

impl Display for RELATIVE_OID {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(".").as_str())
    }

}

impl IntoIterator for RELATIVE_OID {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }

}

impl PartialEq for RELATIVE_OID {

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

impl From<OBJECT_IDENTIFIER> for RELATIVE_OID {

    fn from(oid: OBJECT_IDENTIFIER) -> Self {
        RELATIVE_OID(oid.0.clone())
    }

}
