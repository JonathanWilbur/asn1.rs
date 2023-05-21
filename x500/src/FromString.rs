use std::str::FromStr;
use asn1::ASN1Error;

// use crate::types::{
//     DefaultX500ValueDisplayer,
//     DisplayX500AttributeType,
//     DisplayX500Value,
// };
use crate::SelectedAttributeTypes::{
    UnboundedDirectoryString,
    NameAndOptionalUID,
};
use crate::InformationFramework::{
    AttributeTypeAndValue,
    RelativeDistinguishedName,
    RDNSequence,
    Name,
};
use crate::CertificateExtensions::{
    GeneralName,
    EDIPartyName,
};

impl FromStr for UnboundedDirectoryString {
    type Err = ASN1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let has_colon: bool = s.contains(':');
        if !has_colon {
            return Ok(UnboundedDirectoryString::uTF8String(String::from(s)));
        }
        let lowercased = s.to_lowercase();
        if lowercased.starts_with("utf8string:") {
            return Ok(UnboundedDirectoryString::uTF8String(String::from(&s[10..])));
        }
        else if lowercased.starts_with("printablestring:") {
            return Ok(UnboundedDirectoryString::printableString(String::from(&s[15..])));
        }
        else if lowercased.starts_with("universalstring:") {
            return Ok(UnboundedDirectoryString::universalString(String::from(&s[15..])));
        }
        else if lowercased.starts_with("bmpstring:") {
            return Ok(UnboundedDirectoryString::bmpString(String::from(&s[9..])));
        }
        else if lowercased.starts_with("teletexstring:") {
            return Ok(UnboundedDirectoryString::teletexString(Vec::from(&s[13..])));
        }
        else {
            return Ok(UnboundedDirectoryString::uTF8String(String::from(s)));
        }
    }

}
