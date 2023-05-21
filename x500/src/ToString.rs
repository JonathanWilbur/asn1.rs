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

/// Stolen from https://github.com/snylonue/multirep
/// Which is released under an MIT license as of May 17th, 2023.
///
/// Multiple version of `str::replace` which replaces multiple patterns at a time.
///
///
/// ```
/// use multirep::multi_replace;
///
/// let s = "Hana is cute";
/// let r = multi_replace(s, &[("Hana", "Minami"), ("cute", "kawaii")]);
/// assert_eq!(r, "Minami is kawaii");
/// ```
///
/// The replacement takes place in order of `pats`
///
/// ```
/// use multirep::multi_replace;
/// assert_eq!("Minami is kawaii", multi_replace("Hana is cute", &[("Hana", "Minami"), ("cute", "kawaii"), ("na", "no")]));
/// ```
///
/// Replacement will not be interfere with previosly replaced strings.
///
/// ```
/// use multirep::multi_replace;
/// assert_eq!("Minami is kawaii", multi_replace("Hana is cute", &[("Hana", "Minami"), ("cute", "kawaii"), ("kawaii", "hot")]));
/// ```
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
                // FIXME: This is close, but still not quite right. See: https://en.wikipedia.org/wiki/ITU_T.61
                let new_s = String::from_utf8_lossy(s);
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
                // FIXME: This is close, but still not quite right. See: https://en.wikipedia.org/wiki/ITU_T.61
                let new_s = String::from_utf8_lossy(s);
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

impl Display for GeneralName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralName::otherName(n) => {
                // TODO: SmtpUTF8Mailbox https://datatracker.ietf.org/doc/html/rfc8398
                // TODO: XmppAddr https://datatracker.ietf.org/doc/html/rfc3920#section-5.1.1
                // TODO: SRVName https://datatracker.ietf.org/doc/html/rfc4985
                // Subject Alternative Name = Other Name: Principal Name= (UPN). For example:
                // UPN = user1@name.com
                // The UPN OtherName OID is: "1.3.6.1.4.1.311.20.2.3"
                // The UPN OtherName value: Must be ASN1-encoded UTF8 string
                // TODO: UPN https://learn.microsoft.com/en-US/troubleshoot/windows-server/windows-security/enabling-smart-card-logon-third-party-certification-authorities
                // TODO: NAIRealm https://datatracker.ietf.org/doc/html/rfc7585#section-2.2
                // TODO: Try to print the value, even if it is unrecognized.
                f.write_str("other:?")
            },
            GeneralName::rfc822Name(n) => {
                f.write_str("rfc822:")?;
                f.write_str(n)
            },
            GeneralName::dNSName(n) => {
                f.write_str("dns:")?;
                f.write_str(n)
            },
            GeneralName::x400Address(n) => {
                f.write_str("x400:?")
            },
            GeneralName::directoryName(n) => {
                f.write_str("dir:")?;
                match n {
                    Name::rdnSequence(rdns) => {
                        f.write_fmt(format_args!("rdns:{}", &display_rdn_sequence(rdns)))
                    },
                }
            },
            GeneralName::ediPartyName(n) => {
                f.write_fmt(format_args!("edi:{}", &n))
            },
            GeneralName::uniformResourceIdentifier(n) => {
                f.write_str("uri:")?;
                f.write_str(n)
            },
            GeneralName::iPAddress(n) => {
                if n.len() == 4 {
                    f.write_str("ipv4:")?;
                    f.write_fmt(format_args!("{}.{}.{}.{}", n[0], n[1], n[2], n[3]))
                } else if n.len() == 16 {
                    f.write_str("ipv6:")?;
                    display_ipv6(n, f)
                } else {
                    Err(Error)
                }
            },
            GeneralName::registeredID(n) => {
                f.write_str("oid:")?;
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
