// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::PKI_Stub::{
    AttCertValidityPeriod, AttributeTypeAndValue, AttributeValue, EDIPartyName, GeneralName, Name,
    OTHER_NAME, RDNSequence, RelativeDistinguishedName, Time, Validity,
};
use crate::{DefaultX500ValueDisplayer, DisplayX500AttributeType, DisplayX500Value};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{Display, Error, Write};
use std::str::FromStr;
use teletex::teletex_to_utf8;
use wildboar_asn1::{
    ASN1Value, BMPString, INSTANCE_OF, TagClass, UNIV_TAG_BMP_STRING, UNIV_TAG_PRINTABLE_STRING,
    UNIV_TAG_T61_STRING, UNIV_TAG_UNIVERSAL_STRING, UNIV_TAG_UTF8_STRING, UniversalString,
    read_i64,
};
use x690::ber::BER;
use x690::{X690Codec, X690Element, x690_write_tlv};

impl Display for AttributeTypeAndValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayer = DefaultX500ValueDisplayer {};
        let maybe_name = displayer.attr_type_to_shortest_name(&self.type_);
        // IETF RFC states that the attribute type must be in numeric form if
        // the unknown attribute value syntax is to be used.
        if let Some(name) = maybe_name {
            f.write_str(&name)?;
            f.write_str("=")?;
            AttributeValue(self.value.clone()).fmt(f)
        } else {
            f.write_str(&self.type_.to_string())?;
            f.write_str("=")?;
            f.write_str(&displayer.unrecognized_value_to_string(&self.value))
        }
    }
}

fn replace_rfc_4514_specials(s: &str) -> Cow<str> {
    // let mut ret = Cow::Borrowed(s);
    let mut owned_s = String::new();
    // These are only special if at the start of the string.
    if s.starts_with("#") || s.starts_with(" ") {
        // Double the capacity of the original string, since every character
        // could be escaped, which would require one more character for each.
        owned_s = String::with_capacity(s.len() * 2);
        // s = format!("\\{}", s).to_string();
        owned_s.push('\\');
    }
    let mut start = 0;
    for (i, c) in s.char_indices() {
        let maybe_replacement = match c {
            '\\' => Some("\\\\"),
            '+' => Some("\\+"),
            ',' => Some("\\,"),
            ';' => Some("\\;"),
            '<' => Some("\\<"),
            '>' => Some("\\>"),
            '\0' => Some("\\00"),
            _ => None,
        };
        if let Some(rep) = maybe_replacement {
            owned_s.extend(s[start..i].chars());
            owned_s.extend(rep.chars());
            start = i + 1;
        }
    }

    if owned_s.len() > 0 {
        // This only happens if we replaced a character.
        let len = s.len();
        let mut remaining = &s[start..];
        if s.ends_with(" ") {
            remaining = &s[start..len - 1];
            owned_s.reserve(remaining.len() + 2);
            owned_s.extend(remaining.chars());
            owned_s.push('\\');
            owned_s.push(' ');
        } else {
            owned_s.extend(s[start..].chars());
        }
        return Cow::Owned(owned_s);
    }

    if s.ends_with(" ") {
        let len = s.len();
        let remaining = &s[..len - 1];
        owned_s.reserve(remaining.len() + 2);
        owned_s.extend(remaining.chars());
        owned_s.push('\\');
        owned_s.push(' ');
        return Cow::Owned(owned_s);
    }
    Cow::Borrowed(s)
}

/// This should ONLY be used to display a single RDN. It CANNOT be used to
/// display an RDN sequence.
pub fn rdn_to_string(rdn: &RelativeDistinguishedName) -> String {
    if rdn.len() == 1 {
        let atav = rdn[0].to_string();
        let s = replace_rfc_4514_specials(atav.as_str());
        if matches!(s, Cow::Borrowed(_)) {
            // If borrowed, it was unchanged, so we can just return the
            // original string.
            return atav;
        }
        return s.into_owned();
    }
    rdn.iter()
        .map(|atav| {
            let atav_str = atav.to_string();
            // We replace specials here--not in AttributeTypeAndValue--because
            // we don't know that they are going to be used in a DN or RDN until
            // they are stringified from this function.
            let s = replace_rfc_4514_specials(atav_str.as_str());
            if matches!(s, Cow::Borrowed(_)) {
                // If borrowed, it was unchanged, so we can just return the
                // original string.
                return atav_str;
            }
            s.into_owned()
        })
        .collect::<Vec<String>>()
        .join("+")
}

pub fn rdn_sequence_to_string(rdns: &RDNSequence) -> String {
    rdns.iter()
        .map(rdn_to_string)
        .collect::<Vec<String>>()
        .join(",")
}

pub fn display_rdn(
    f: &mut std::fmt::Formatter<'_>,
    rdn: &RelativeDistinguishedName,
) -> std::fmt::Result {
    let mut first = true;
    let displayer = DefaultX500ValueDisplayer {};
    for atav in rdn {
        if !first {
            f.write_char('+')?;
        }
        // Most of the code here was copied from the Display implementation
        // for `AttributeTypeAndValue`
        let maybe_name = displayer.attr_type_to_shortest_name(&atav.type_);
        // IETF RFC states that the attribute type must be in numeric form if
        // the unknown attribute value syntax is to be used.
        if let Some(name) = maybe_name {
            f.write_str(&name)?;
            f.write_str("=")?;
            let atav_str = AttributeValue(atav.value.clone()).to_string();
            let s = replace_rfc_4514_specials(atav_str.as_str());
            f.write_str(s.as_ref())?;
        } else {
            f.write_str(&atav.type_.to_string())?;
            f.write_str("=")?;
            f.write_str(&displayer.unrecognized_value_to_string(&atav.value))?;
        }
        first = false;
    }
    Ok(())
}

pub fn display_rdn_sequence(
    f: &mut std::fmt::Formatter<'_>,
    rdns: &RDNSequence,
) -> std::fmt::Result {
    let mut first = true;
    for rdn in rdns {
        if !first {
            f.write_char(',')?;
        }
        display_rdn(f, rdn)?;
        first = false;
    }
    Ok(())
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::rdnSequence(rdns) => {
                f.write_str("rdnSequence:")?;
                display_rdn_sequence(f, rdns)
            }
            Name::dnsName(dns) => {
                f.write_str("dnsName:")?;
                f.write_str(&dns)
            }
            Name::oid(oid) => {
                f.write_str("oid:")?;
                oid.fmt(f)
            }
        }
    }
}

fn display_ipv6(ip: &Vec<u8>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

fn display_directory_string(ds: &X690Element, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if ds.tag.tag_class != TagClass::UNIVERSAL {
        return ds.tag.fmt(f);
    }
    match ds.tag.tag_number {
        UNIV_TAG_UTF8_STRING => {
            let s = BER.decode_utf8_string(ds).unwrap_or("<?>".into());
            f.write_str(s.as_str())
        }
        UNIV_TAG_PRINTABLE_STRING => {
            let s = BER.decode_printable_string(ds).unwrap_or("<?>".into());
            f.write_str(s.as_str())
        }
        UNIV_TAG_BMP_STRING => {
            let s = BER
                .decode_bmp_string(ds)
                .unwrap_or(BMPString::from_str("<?>").unwrap());
            s.fmt(f)
        }
        UNIV_TAG_UNIVERSAL_STRING => {
            let s = BER
                .decode_universal_string(ds)
                .unwrap_or(UniversalString::from_str("<?>").unwrap());
            s.fmt(f)
        }
        UNIV_TAG_T61_STRING => {
            let s = BER.decode_t61_string(ds).unwrap_or("<?>".into());
            f.write_str(teletex_to_utf8(&s).as_ref())
        }
        _ => ds.tag.fmt(f),
    }
}

impl Display for EDIPartyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        if let Some(na) = &self.nameAssigner {
            f.write_str("nameAssigner:\"")?;
            display_directory_string(na, f)?;
            f.write_str("\" ")?;
            // f.write_fmt(format_args!("nameAssigner:\"{}\", ", na))?;
        }
        f.write_str("partyName:\"")?;
        display_directory_string(&self.partyName, f)?;
        f.write_str("\" }")
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
const HardwareModuleName: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 4];

/// XmppAddr is described here: https://datatracker.ietf.org/doc/html/rfc3920#section-5.1.1
///
/// ```asn1
/// id-on-xmppAddr  OBJECT IDENTIFIER ::= { id-on 5 }
/// ```
const XmppAddr: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 5];

/// SRVName is described here: https://datatracker.ietf.org/doc/html/rfc4985
///
/// ```asn1
/// id-on-dnsSRV OBJECT IDENTIFIER ::= { id-on 7 }
///
const SRVName: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 7];

/// The NAIRealm OtherName is described here: https://datatracker.ietf.org/doc/html/rfc7585#section-2.2
///
/// ```asn1
/// id-on-naiRealm OBJECT IDENTIFIER ::= { id-on 8 }
/// ```
const NAIRealm: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 8];

/// SmtpUTF8Mailbox is described here: https://datatracker.ietf.org/doc/html/rfc8398
///
/// ```asn1
/// id-on-SmtpUTF8Mailbox OBJECT IDENTIFIER ::= { id-on 9 }
/// ```
const SmtpUTF8Mailbox: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 9];

/// AcpNodeName is described here: https://www.rfc-editor.org/rfc/rfc8994.html
///
/// ```asn1
/// id-on-AcpNodeName OBJECT IDENTIFIER ::= { id-on 10 }
/// ```
const AcpNodeName: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 10];

/// BundleEID is described here: https://www.rfc-editor.org/rfc/rfc9174.html#name-asn1-module
///
/// ```asn1
/// id-on-bundleEID OBJECT IDENTIFIER ::= { id-on 11 }
/// ```
const BundleEID: [u32; 9] = [1, 3, 6, 1, 5, 5, 7, 8, 11];

/// The UPN OtherName is described here: https://learn.microsoft.com/en-US/troubleshoot/windows-server/windows-security/enabling-smart-card-logon-third-party-certification-authorities
///
const UPN: [u32; 10] = [1, 3, 6, 1, 4, 1, 311, 20, 2, 3];

pub fn display_other_name(n: &INSTANCE_OF, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let n_slice: Vec<u32> = n
        .type_id
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
                f.write_fmt(format_args!(
                    "{}:{}",
                    hwType.to_string(),
                    hex::encode(hwSerialNum)
                ))
            }
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == XmppAddr.as_slice() {
        f.write_str("xmppAddr:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == SRVName.as_slice() {
        f.write_str("srvName:")?;
        return match n.value.as_ref() {
            ASN1Value::IA5String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == NAIRealm.as_slice() {
        f.write_str("naiRealm:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == SmtpUTF8Mailbox.as_slice() {
        f.write_str("smtpUTF8Mailbox:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == AcpNodeName.as_slice() {
        f.write_str("acpNodeName:")?;
        return match n.value.as_ref() {
            ASN1Value::IA5String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == BundleEID.as_slice() {
        f.write_str("bundleEID:")?;
        return match n.value.as_ref() {
            ASN1Value::IA5String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else if &n_slice == UPN.as_slice() {
        f.write_str("upn:")?;
        return match n.value.as_ref() {
            ASN1Value::UTF8String(s) => f.write_str(s),
            _ => Err(std::fmt::Error),
        };
    } else {
        match BER.encode_any(&n.value) {
            Ok(any_el) => {
                let mut ber: Vec<u8> = vec![];
                let _ = x690_write_tlv(&mut ber, &any_el).map_err(|_| std::fmt::Error)?;
                f.write_fmt(format_args!(
                    "{}:{}",
                    n.type_id.to_string(),
                    hex::encode(ber)
                ))
            }
            Err(_) => f.write_fmt(format_args!(
                "{}:{}",
                n.type_id.to_string(),
                n.value.as_ref()
            )),
        }
    }
}

impl Display for GeneralName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralName::otherName(n) => {
                f.write_str("otherName:")?;
                display_other_name(n, f)
            }
            GeneralName::rfc822Name(n) => {
                f.write_str("rfc822Name:")?;
                f.write_str(n)
            }
            GeneralName::dNSName(n) => {
                f.write_str("dNSName:")?;
                f.write_str(n)
            }
            GeneralName::x400Address(n) => {
                // TODO: Make a feature to display this
                f.write_str("x400Address:?")
                // f.write_str(&n.to_rfc1685_string().unwrap_or("?".into()))
            }
            GeneralName::directoryName(n) => {
                f.write_str("directoryName:")?;
                n.fmt(f)
            }
            GeneralName::ediPartyName(n) => f.write_fmt(format_args!("ediPartyName:{}", &n)),
            GeneralName::uniformResourceIdentifier(n) => {
                f.write_str("uniformResourceIdentifier:")?;
                f.write_str(n)
            }
            GeneralName::iPAddress(n) => {
                f.write_str("iPAddress:")?;
                if n.len() == 4 {
                    write!(f, "{}.{}.{}.{}", n[0], n[1], n[2], n[3])
                } else if n.len() == 16 {
                    display_ipv6(n, f)
                } else {
                    write!(f, "{}", hex::encode(n.as_slice()).as_str())
                }
            }
            GeneralName::registeredID(n) => {
                f.write_str("registeredID:")?;
                n.fmt(f)
            }
            GeneralName::_unrecognized(_) => f.write_str("?"),
        }
    }
}

fn write_unrecognized_value(
    f: &mut std::fmt::Formatter<'_>,
    value: &X690Element,
) -> std::fmt::Result {
    let mut encoding: Vec<u8> = Vec::with_capacity(value.len() + 2);
    x690_write_tlv(&mut encoding, &value).unwrap_or_default();
    write!(f, "#{}", hex::encode(&encoding))
}

impl Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = &self.0;
        if value.tag.tag_class != TagClass::UNIVERSAL {
            return write_unrecognized_value(f, &self.0);
        }
        match value.tag.tag_number {
            // UNIV_TAG_END_OF_CONTENT => {},
            wildboar_asn1::UNIV_TAG_BOOLEAN => {
                let v = match BER.decode_boolean(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                if v {
                    f.write_str("TRUE")
                } else {
                    f.write_str("FALSE")
                }
            }
            wildboar_asn1::UNIV_TAG_INTEGER => {
                let integ = match BER.decode_integer(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                if let Some(i) = read_i64(&integ) {
                    write!(f, "{}", i)
                } else {
                    write_unrecognized_value(f, &self.0)
                }
            }
            wildboar_asn1::UNIV_TAG_BIT_STRING => {
                let v = match BER.decode_bit_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_OCTET_STRING => {
                let v = match BER.decode_octet_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                // NOTE: This is not the LDAP syntax. The LDAP syntax is just the raw octets.
                write_unrecognized_value(f, &self.0)
            }
            wildboar_asn1::UNIV_TAG_NULL => f.write_str("NULL"),
            wildboar_asn1::UNIV_TAG_OBJECT_IDENTIFIER => {
                let v = match BER.decode_object_identifier(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                // Ok(Some(v.to_string()))
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_OBJECT_DESCRIPTOR => {
                let v = match BER.decode_object_descriptor(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            // UNIV_TAG_EXTERNAL => {},
            wildboar_asn1::UNIV_TAG_REAL => {
                let v = match BER.decode_real(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                write!(f, "{}", v)
            }
            wildboar_asn1::UNIV_TAG_ENUMERATED => {
                let v = match BER.decode_enumerated(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                write!(f, "{}", v)
            }
            // UNIV_TAG_EMBEDDED_PDV => {},
            wildboar_asn1::UNIV_TAG_UTF8_STRING => {
                let v = match BER.decode_utf8_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_RELATIVE_OID => {
                let v = match BER.decode_relative_oid(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_TIME => {
                let v = match BER.decode_time(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            // UNIV_TAG_RESERVED_15 => {},
            // UNIV_TAG_SEQUENCE => {},
            // UNIV_TAG_SEQUENCE_OF => {},
            // UNIV_TAG_SET => {},
            // UNIV_TAG_SET_OF => {},
            wildboar_asn1::UNIV_TAG_NUMERIC_STRING => {
                let v = match BER.decode_numeric_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_PRINTABLE_STRING => {
                let v = match BER.decode_printable_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            // UNIV_TAG_T61_STRING => {},
            // UNIV_TAG_VIDEOTEX_STRING => {},
            wildboar_asn1::UNIV_TAG_IA5_STRING => {
                let v = match BER.decode_ia5_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_UTC_TIME => {
                let v = match BER.decode_utc_time(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_GENERALIZED_TIME => {
                let v = match BER.decode_generalized_time(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_GRAPHIC_STRING => {
                let v = match BER.decode_graphic_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_VISIBLE_STRING => {
                let v = match BER.decode_visible_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_GENERAL_STRING => {
                let v = match BER.decode_general_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING => {
                let v = match BER.decode_universal_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            // UNIV_TAG_CHARACTER_STRING => {},
            wildboar_asn1::UNIV_TAG_BMP_STRING => {
                let v = match BER.decode_bmp_string(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_DATE => {
                let v = match BER.decode_date(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                write!(f, "{}-{}-{}", v.year, v.month, v.day)
            }
            wildboar_asn1::UNIV_TAG_TIME_OF_DAY => {
                let v = match BER.decode_time_of_day(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                write!(f, "{}:{}:{}", v.hour, v.minute, v.second)
            }
            wildboar_asn1::UNIV_TAG_DATE_TIME => {
                let v = match BER.decode_date_time(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                write!(
                    f,
                    "{}-{}-{}T{}:{}:{}",
                    v.date.year,
                    v.date.month,
                    v.date.day,
                    v.time.hour,
                    v.time.minute,
                    v.time.second
                )
            }
            wildboar_asn1::UNIV_TAG_DURATION => {
                let v = match BER.decode_duration(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                v.fmt(f)
            }
            wildboar_asn1::UNIV_TAG_OID_IRI => {
                let v = match BER.decode_oid_iri(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            wildboar_asn1::UNIV_TAG_RELATIVE_OID_IRI => {
                let v = match BER.decode_relative_oid_iri(value) {
                    Ok(x) => x,
                    Err(_) => return write_unrecognized_value(f, &self.0),
                };
                f.write_str(v.as_str())
            }
            _ => write_unrecognized_value(f, &self.0),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::generalizedTime(gt) => gt.fmt(f),
            Time::utcTime(ut) => ut.fmt(f),
        }
    }
}

impl Display for Validity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.notBefore.fmt(f)?;
        f.write_str(" - ")?;
        self.notAfter.fmt(f)
    }
}

impl Display for AttCertValidityPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.notBeforeTime.fmt(f)?;
        f.write_str(" - ")?;
        self.notAfterTime.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AttributeTypeAndValue, GeneralName, Name};
    use wildboar_asn1::oid;
    use x690::X690Codec;
    use x690::ber::BER;

    use super::replace_rfc_4514_specials;
    use std::borrow::Cow;

    #[test]
    fn test_replace_rfc_4514_specials_1() {
        let outcome = replace_rfc_4514_specials("asdf\\, zxcv");
        if let Cow::Owned(s) = outcome {
            assert_eq!(s.as_str(), "asdf\\\\\\, zxcv");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_replace_rfc_4514_specials_2() {
        let outcome = replace_rfc_4514_specials("asdf zxcv");
        if let Cow::Borrowed(s) = outcome {
            assert_eq!(s, "asdf zxcv");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn display_general_name_1() {
        let gn = GeneralName::directoryName(Name::rdnSequence(vec![
            vec![AttributeTypeAndValue::new(
                oid!(0, 9, 2342, 19200300, 100, 1, 25), // dc
                BER.encode_ia5_string("com").unwrap(),
                vec![],
            )],
            vec![AttributeTypeAndValue::new(
                oid!(0, 9, 2342, 19200300, 100, 1, 25), // dc
                BER.encode_ia5_string("wildboarsoftware").unwrap(),
                vec![],
            )],
        ]));
        assert_eq!(
            gn.to_string().as_str(),
            "directoryName:rdnSequence:dc=com,dc=wildboarsoftware"
        );
    }

    #[test]
    fn display_general_name_2() {
        let gn = GeneralName::directoryName(Name::rdnSequence(vec![
            vec![AttributeTypeAndValue::new(
                oid!(0, 9, 2342, 19200300, 100, 1, 25), // dc
                BER.encode_ia5_string("com").unwrap(),
                vec![],
            )],
            vec![AttributeTypeAndValue::new(
                oid!(0, 9, 2342, 19200300, 100, 1, 25), // dc
                BER.encode_ia5_string("wildboar").unwrap(),
                vec![],
            )],
            vec![
                AttributeTypeAndValue::new(
                    oid!(2, 5, 4, 3), // cn
                    BER.encode_utf8_string("  Goobis\\ Boobis++ ").unwrap(),
                    vec![],
                ),
                AttributeTypeAndValue::new(
                    oid!(2, 5, 4, 4), // sn
                    BER.encode_utf8_string("#von Boobis ><>").unwrap(),
                    vec![],
                ),
            ],
        ]));
        let expected = "directoryName:rdnSequence:dc=com,dc=wildboar,cn=\\  Goobis\\\\ Boobis\\+\\+\\ +sn=\\#von Boobis \\>\\<\\>";
        assert_eq!(gn.to_string().as_str(), expected);
    }
}
