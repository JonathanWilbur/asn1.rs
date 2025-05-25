// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use wildboar_asn1::{ASN1Value, INSTANCE_OF};
use x690::ber_encode;
use crate::types::{
    DefaultX500ValueDisplayer,
    DisplayX500AttributeType,
    DisplayX500Value,
};
use crate::SelectedAttributeTypes::{
    UnboundedDirectoryString,
    DirectoryString,
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
use std::fmt::{Display, Error};
use std::collections::BTreeMap;
use teletex::teletex_to_utf8;

/// Stolen from https://github.com/snylonue/multirep
/// Which is released under an MIT license as of May 17th, 2023.
///
/// Multiple version of `str::replace` which replaces multiple patterns at a time.
///
/// MIT License
///
/// Copyright (c) 2022 snylonue
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.
fn multi_replace(s: &str, pats: &[(&str, &str)]) -> String {
    let mut indices = BTreeMap::new();

    for (pat, new) in pats {
        for (i, p) in s.match_indices(pat) {
            if indices
                .range(..=i)
                .next_back()
                .map(|(pos, (len, _))| pos + len <= i)
                .unwrap_or(true)
            {
                indices.insert(i, (p.len(), *new));
            }
        }
    }

    let mut result = String::new();
    let mut end = 0usize;

    for (pos, (len, new)) in indices {
        // SAFETY: pos is returned by `str::match_indices`, which is valid
        // end >= 0 since it starts at 0 and only increases
        // end < pos since `str::match_indices` doesn't overlap
        // len is the length of one pattern string, so `pos + len`(`end`) should be on unicode boundaries.
        result.push_str(unsafe { s.get_unchecked(end..pos) });
        result.push_str(new);
        end = pos + len;
    }

    if end < s.len() {
        // SAFETY: end >= 0 and is on unicode boundaries as above
        // end < s.len()
        result.push_str(unsafe { s.get_unchecked(end..) });
    }

    result
}

impl Display for UnboundedDirectoryString {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnboundedDirectoryString::teletexString(s) => {
                let new_s = teletex_to_utf8(s.as_slice());
                f.write_str(new_s.as_ref())
            },
            UnboundedDirectoryString::printableString(s) => {
                f.write_str(s)
            },
            UnboundedDirectoryString::bmpString(s) => {
                f.write_str(s)
            },
            UnboundedDirectoryString::universalString(s) => {
                f.write_str(s)
            },
            UnboundedDirectoryString::uTF8String(s) => {
                f.write_str(s)
            },
        }
    }

}

impl Display for DirectoryString {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectoryString::teletexString(s) => {
                let new_s = teletex_to_utf8(s.as_slice());
                f.write_str(new_s.as_ref())
            },
            DirectoryString::printableString(s) => {
                f.write_str(s)
            },
            DirectoryString::bmpString(s) => {
                f.write_str(s)
            },
            DirectoryString::universalString(s) => {
                f.write_str(s)
            },
            DirectoryString::uTF8String(s) => {
                f.write_str(s)
            },
        }
    }

}

impl Display for AttributeTypeAndValue {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayer = DefaultX500ValueDisplayer{};
        let maybe_name = displayer.attr_type_to_name(&self.type_);
        // IETF RFC states that the attribute type must be in numeric form if
        // the unknown attribute value syntax is to be used.
        if let Some(name) = maybe_name {
            let maybe_value = displayer.value_to_string(&self.type_, &self.value)
                .map_err(|_| Error)?;
            if let Some(value) = maybe_value {
                f.write_str(&name)?;
                f.write_str("=")?;
                f.write_str(&value)?;
            } else {
                f.write_str(&self.type_.to_string())?;
                f.write_str("=")?;
                f.write_str(&displayer.unrecognized_value_to_string(&self.value))?;
            }
        } else {
            f.write_str(&self.type_.to_string())?;
            f.write_str("=")?;
            f.write_str(&displayer.unrecognized_value_to_string(&self.value))?;
        }
        Ok(())
    }

}

const IETF_RFC_4514_MANDATORY_ESCAPES: [(&str, &str); 8] = [
    ("\\", "\\\\"),
    ("\"", "\\\""),
    ("+", "\\+"),
    (",", "\\,"),
    (";", "\\;"),
    ("<", "\\<"),
    (">", "\\>"),
    ("\x00", "\\00"),
];

/// This should ONLY be used to display a single RDN. It CANNOT be used to
/// display an RDN sequence.
pub fn display_rdn (rdn: &RelativeDistinguishedName) -> String {
    rdn
        .iter()
        .map(|atav| {
            let mut s = multi_replace(
                &format!("{}", &atav).to_string(),
                &IETF_RFC_4514_MANDATORY_ESCAPES
            );
            if s.starts_with("#") || s.starts_with(" ") {
                s = format!("\\{}", s).to_string();
            }
            let len = s.len();
            if s.ends_with(" ") {
                s = format!("{}\\ ", &s[0..len-1]).to_string();
            }
            s
        })
        .collect::<Vec<String>>()
        .join("+")
}

pub fn display_rdn_sequence (rdns: &RDNSequence) -> String {
    rdns
        .iter()
        .map(|rdn| rdn
            .iter()
            .map(|atav| {
                let mut s = multi_replace(
                    &format!("{}", &atav).to_string(),
                    &IETF_RFC_4514_MANDATORY_ESCAPES
                );
                if s.starts_with("#") || s.starts_with(" ") {
                    s = format!("\\{}", s).to_string();
                }
                let len = s.len();
                if s.ends_with(" ") {
                    s = format!("{}\\ ", &s[0..len-1]).to_string();
                }
                s
            })
            .collect::<Vec<String>>()
            .join("+")
        )
        .collect::<Vec<String>>()
        .join(",")
}

impl Display for Name {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::rdnSequence(rdns) => {
                f.write_str("rdnSequence:")?;
                f.write_str(&display_rdn_sequence(&rdns))
            },
            Name::dnsName(dns) => {
                f.write_str("dnsName:")?;
                f.write_str(&dns)
            },
            Name::oid(oid) => {
                f.write_str("oid:")?;
                f.write_str(&oid.to_string())
            }
        }
    }

}

fn display_ipv6 (ip: &Vec<u8>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        &hex::encode(&ip[0..=1]),
        &hex::encode(&ip[2..=3]),
        &hex::encode(&ip[4..=5]),
        &hex::encode(&ip[6..=7]),
        &hex::encode(&ip[8..=9]),
        &hex::encode(&ip[10..=11]),
        &hex::encode(&ip[12..=13]),
        &hex::encode(&ip[14..=15]),
    ))
}

impl Display for EDIPartyName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        if let Some(na) = &self.nameAssigner {
            f.write_fmt(format_args!("nameAssigner:\"{}\", ", na))?;
        }
        f.write_fmt(format_args!("partyName:\"{}\"", &self.partyName))?;
        f.write_str(" }")
    }

}

// id-pkix  OBJECT IDENTIFIER  ::=
// {iso(1) identified-organization(3) dod(6) internet(1) security(5)
// mechanisms(5) pkix(7)}
// id-on OBJECT IDENTIFIER ::= { id-pkix 8 }

/// HardwareModuleName is described here: https://www.rfc-editor.org/rfc/rfc4108.html#page-56
///
/// ```asn1
/// id-on-hardwareModuleName  OBJECT IDENTIFIER ::= { id-on 4 }
/// ```
const HardwareModuleName: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 4 ];

/// XmppAddr is described here: https://datatracker.ietf.org/doc/html/rfc3920#section-5.1.1
///
/// ```asn1
/// id-on-xmppAddr  OBJECT IDENTIFIER ::= { id-on 5 }
/// ```
const XmppAddr: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 5 ];

/// SRVName is described here: https://datatracker.ietf.org/doc/html/rfc4985
///
/// ```asn1
/// id-on-dnsSRV OBJECT IDENTIFIER ::= { id-on 7 }
///
const SRVName: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 7 ];

/// The NAIRealm OtherName is described here: https://datatracker.ietf.org/doc/html/rfc7585#section-2.2
///
/// ```asn1
/// id-on-naiRealm OBJECT IDENTIFIER ::= { id-on 8 }
/// ```
const NAIRealm: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 8 ];

/// SmtpUTF8Mailbox is described here: https://datatracker.ietf.org/doc/html/rfc8398
///
/// ```asn1
/// id-on-SmtpUTF8Mailbox OBJECT IDENTIFIER ::= { id-on 9 }
/// ```
const SmtpUTF8Mailbox: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 9 ];

/// AcpNodeName is described here: https://www.rfc-editor.org/rfc/rfc8994.html
///
/// ```asn1
/// id-on-AcpNodeName OBJECT IDENTIFIER ::= { id-on 10 }
/// ```
const AcpNodeName: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 10 ];

/// BundleEID is described here: https://www.rfc-editor.org/rfc/rfc9174.html#name-asn1-module
///
/// ```asn1
/// id-on-bundleEID OBJECT IDENTIFIER ::= { id-on 11 }
/// ```
const BundleEID: [u32; 9] = [ 1, 3, 6, 1, 5, 5, 7, 8, 11 ];


/// The UPN OtherName is described here: https://learn.microsoft.com/en-US/troubleshoot/windows-server/windows-security/enabling-smart-card-logon-third-party-certification-authorities
///
const UPN: [u32; 10] = [ 1, 3, 6, 1, 4, 1, 311, 20, 2, 3 ];

pub fn display_other_name (n: &INSTANCE_OF, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let n_slice: Vec<u32> = n.type_id
        .arcs()
        .map(|arc| std::cmp::min(arc, u32::MAX as u128) as u32)
        .collect();
    if &n_slice == HardwareModuleName.as_slice() {
        f.write_str("hardwareModuleName:")?;
        // HardwareModuleName ::= SEQUENCE {
        //     hwType OBJECT IDENTIFIER,
        //     hwSerialNum OCTET STRING }
        return match n.value.as_ref() {
            ASN1Value::SequenceValue(components) => {
                if components.len() != 2 {
                    return Err(std::fmt::Error);
                }
                let hwType = match components[0] {
                    ASN1Value::ObjectIdentifierValue(ref o) => o,
                    _ => return Err(std::fmt::Error),
                };
                let hwSerialNum = match components[1] {
                    ASN1Value::OctetStringValue(ref o) => o,
                    _ => return Err(std::fmt::Error),
                };
                f.write_fmt(format_args!("{}:{}", hwType.to_string(), hex::encode(hwSerialNum)))
            },
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == XmppAddr.as_slice() {
        f.write_str("xmppAddr:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == SRVName.as_slice() {
        f.write_str("srvName:")?;
        return match n.value.as_ref() {
            ASN1Value::IA5String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == NAIRealm.as_slice() {
        f.write_str("naiRealm:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == SmtpUTF8Mailbox.as_slice() {
        f.write_str("smtpUTF8Mailbox:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == AcpNodeName.as_slice() {
        f.write_str("acpNodeName:")?;
        return match n.value.as_ref() {
            ASN1Value::IA5String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == BundleEID.as_slice() {
        f.write_str("bundleEID:")?;
        return match n.value.as_ref() {
            ASN1Value::IA5String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else if &n_slice == UPN.as_slice() {
        f.write_str("upn:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error)
        };
    } else {
        let mut ber: Vec<u8> = vec![];
        match ber_encode(&mut ber, &n.value) {
            Ok(_) => f.write_fmt(format_args!("{}:{}", n.type_id.to_string(), hex::encode(ber))),
            Err(_) => f.write_fmt(format_args!("{}:{}", n.type_id.to_string(), n.value.as_ref())),
        }
    }
}

impl Display for GeneralName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralName::otherName(n) => {
                f.write_str("otherName:")?;
                display_other_name(n, f)
            },
            GeneralName::rfc822Name(n) => {
                f.write_str("rfc822Name:")?;
                f.write_str(n)
            },
            GeneralName::dNSName(n) => {
                f.write_str("dNSName:")?;
                f.write_str(n)
            },
            GeneralName::x400Address(n) => {
                f.write_str("x400Address:")?;
                f.write_str(&n.to_rfc1685_string().unwrap_or("?".into()))
            },
            GeneralName::directoryName(n) => {
                f.write_str("directoryName:")?;
                match n {
                    Name::rdnSequence(rdns) => {
                        f.write_str("rdnSequence:")?;
                        f.write_str(&display_rdn_sequence(rdns))
                    },
                    Name::dnsName(dns) => {
                        f.write_str("dnsName:")?;
                        f.write_str(&dns)
                    },
                    Name::oid(oid) => {
                        f.write_str("oid:")?;
                        f.write_str(&oid.to_string())
                    }
                }
            },
            GeneralName::ediPartyName(n) => {
                f.write_fmt(format_args!("ediPartyName:{}", &n))
            },
            GeneralName::uniformResourceIdentifier(n) => {
                f.write_str("uniformResourceIdentifier:")?;
                f.write_str(n)
            },
            GeneralName::iPAddress(n) => {
                f.write_str("iPAddress:")?;
                if n.len() == 4 {
                    f.write_fmt(format_args!("{}.{}.{}.{}", n[0], n[1], n[2], n[3]))
                } else if n.len() == 16 {
                    display_ipv6(n, f)
                } else {
                    Err(Error)
                }
            },
            GeneralName::registeredID(n) => {
                f.write_str("registeredID:")?;
                f.write_str(&n.to_string())
            },
            GeneralName::_unrecognized(_) => f.write_str("?")
        }
    }

}

impl Display for NameAndOptionalUID {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ dn:\"")?;
        f.write_str(&display_rdn_sequence(&self.dn))?;
        if let Some(uid) = &self.uid {
            f.write_fmt(format_args!("\", uid:{} }}", uid))
        } else {
            f.write_str("\" }")
        }
    }

}
