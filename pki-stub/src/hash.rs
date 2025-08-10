// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::PKI_Stub::{
    _decode_RDNSequence, _validate_RDNSequence, AlgorithmIdentifier, AlgorithmWithInvoke,
    AttCertIssuer, AttCertValidityPeriod, AttributeTypeAndValue, AttributeValue, EDIPartyName,
    FingerPrint, GeneralName, HASH, IssuerSerial, IssuerSerialNumber, Name, OTHER_NAME,
    ObjectDigestInfo, PKCertIdentifier, RDNSequence, RelativeDistinguishedName, SIGNED,
    TBSAttributeCertificate, TBSCertAVL, TBSCertificate, Time, Validity,
};
use crate::eq::{DNSLabelIter, get_string, get_time, trim_u16, trim_u32};
use crate::utils::{gt_to_chrono, utctime_to_chrono};
use crate::{DefaultX500ValueDisplayer, DisplayX500AttributeType, DisplayX500Value};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use email_address::EmailAddress;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{Display, Error, Write};
use std::hash::Hash;
use std::str::FromStr;
use teletex::teletex_to_utf8;
use wildboar_asn1::{
    ASN1Value, BMPString, ExternalEncoding, GeneralizedTime, INSTANCE_OF, TagClass, TagNumber,
    UNIV_TAG_BMP_STRING, UNIV_TAG_PRINTABLE_STRING, UNIV_TAG_T61_STRING, UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_UTF8_STRING, UniversalString, read_i64,
};
use x520_stringprep::{
    x520_stringprep_case_ignore_bmp, x520_stringprep_case_ignore_univ_str,
    x520_stringprep_to_case_ignore_string,
};
use x690::ber::BER;
use x690::{X690Codec, X690Element, deconstruct, primitive, x690_write_tlv};

const HASH_PREFIX_UNKNOWN: TagNumber = 0;
const HASH_PREFIX_STR: TagNumber = 7;
const HASH_PREFIX_DATETIME: TagNumber = 23;

const MAX_DEPTH: usize = 20;

fn hash_asn1_value<H: std::hash::Hasher>(value: &ASN1Value, state: &mut H, depth: usize) {
    if depth > MAX_DEPTH {
        return;
    }
    match value {
        ASN1Value::BooleanValue(b) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_BOOLEAN);
            state.write_u8(if *b { 0xFF } else { 0 })
        }
        ASN1Value::IntegerValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_INTEGER);
            v.hash(state)
        }
        ASN1Value::BitStringValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_BIT_STRING);
            v.hash(state)
        }
        ASN1Value::OctetStringValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_OCTET_STRING);
            v.hash(state)
        }
        ASN1Value::ObjectIdentifierValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_OBJECT_IDENTIFIER);
            v.hash(state)
        }
        ASN1Value::RelativeOIDValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_RELATIVE_OID);
            v.hash(state)
        }
        ASN1Value::ChoiceValue(v) => {
            state.write_u16(0xFFFF);
            hash_asn1_value(v, state, depth + 1)
        }
        ASN1Value::EmbeddedPDVValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_EMBEDDED_PDV);
            v.hash(state)
        }
        ASN1Value::EnumeratedValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_ENUMERATED);
            v.hash(state)
        }
        ASN1Value::InstanceOfValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_INSTANCE_OF);
            v.type_id.hash(state);
            state.write_u8(0xFF);
            hash_asn1_value(&v.value, state, depth + 1)
        }
        ASN1Value::IRIValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_OID_IRI);
            v.to_lowercase().hash(state)
        }
        ASN1Value::NullValue => {
            state.write_u16(wildboar_asn1::UNIV_TAG_NULL);
        }
        ASN1Value::RealValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_REAL);
            v.to_string().hash(state)
        }
        ASN1Value::RelativeIRIValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_RELATIVE_OID_IRI);
            v.to_lowercase().hash(state)
        }
        ASN1Value::SequenceValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_SEQUENCE);
            v.iter().for_each(|c| hash_asn1_value(c, state, depth + 1))
        }
        ASN1Value::SequenceOfValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_SEQUENCE_OF);
            v.iter().for_each(|c| hash_asn1_value(c, state, depth + 1))
        }
        ASN1Value::SetValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_SET);
            if v.len() <= 1 {
                // Do not sort if we don't have to.
                v.iter().for_each(|c| hash_asn1_value(c, state, depth + 1))
            }
            let mut sorted = v.to_owned();
            sorted.sort_by(|a, b| a.tag().cmp(&b.tag()));
            sorted
                .iter()
                .for_each(|c| hash_asn1_value(c, state, depth + 1))
        }
        ASN1Value::SetOfValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_SET_OF);
            v.iter().for_each(|c| hash_asn1_value(c, state, depth + 1))
        }
        ASN1Value::UnrestrictedCharacterStringValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_CHARACTER_STRING);
            v.hash(state)
        }
        ASN1Value::TaggedValue(v) => {
            state.write_u16(0);
            state.write_u16(v.tag.tag_number);
            hash_asn1_value(&v.value, state, depth + 1)
        }
        ASN1Value::TimeValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_TIME);
            v.hash(state)
        }
        ASN1Value::UTCTime(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_UTC_TIME);
            v.hash(state)
        }
        ASN1Value::GeneralizedTime(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_GENERALIZED_TIME);
            v.hash(state)
        }
        ASN1Value::DATE(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_DATE);
            v.hash(state)
        }
        ASN1Value::TIME_OF_DAY(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_TIME_OF_DAY);
            v.hash(state)
        }
        ASN1Value::DATE_TIME(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_DATE_TIME);
            v.hash(state)
        }
        ASN1Value::DURATION(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_DURATION);
            v.to_approximate_seconds().hash(state)
        }
        ASN1Value::UnknownBytes(v) => {
            state.write_u16(HASH_PREFIX_UNKNOWN);
            v.hash(state)
        }
        ASN1Value::ExternalValue(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_EXTERNAL);
            v.identification.hash(state);
            state.write_u8(0xFF);
            match &v.data_value {
                ExternalEncoding::octet_aligned(oct) => {
                    state.write_u8(0);
                    oct.hash(state)
                }
                ExternalEncoding::arbitrary(bin) => {
                    state.write_u8(1);
                    bin.hash(state)
                }
                ExternalEncoding::single_ASN1_type(asn_val) => {
                    state.write_u8(2);
                    hash_asn1_value(&asn_val, state, depth + 1);
                }
            }
        }

        // String types
        ASN1Value::ObjectDescriptor(v)
        | ASN1Value::GeneralString(v)
        | ASN1Value::GraphicString(v)
        | ASN1Value::IA5String(v)
        | ASN1Value::ISO646String(v)
        | ASN1Value::PrintableString(v)
        | ASN1Value::UTF8String(v)
        | ASN1Value::VisibleString(v) => {
            state.write_u16(HASH_PREFIX_STR);
            match x520_stringprep_to_case_ignore_string(v.trim()) {
                Ok(s) => s.hash(state),
                Err(_) => state.write(v.as_bytes()),
            }
        }
        ASN1Value::BMPString(v) => {
            state.write_u16(HASH_PREFIX_STR);
            let res: Result<String, char> =
                x520_stringprep_case_ignore_bmp(trim_u16(&v.0)).collect();
            match res {
                Ok(s) => s.hash(state),
                Err(_) => v.0.hash(state),
            }
        }
        ASN1Value::NumericString(v) => {
            state.write_u16(wildboar_asn1::UNIV_TAG_NUMERIC_STRING);
            v.bytes()
                .filter(|b| b.is_ascii_digit())
                .for_each(|b| state.write_u8(b));
        }
        ASN1Value::TeletexString(v) | ASN1Value::T61String(v) => {
            state.write_u16(HASH_PREFIX_STR);
            match x520_stringprep_to_case_ignore_string(
                teletex_to_utf8(v.as_slice()).as_ref().trim(),
            ) {
                Ok(s) => s.hash(state),
                Err(_) => state.write(v.as_slice()),
            }
        }
        ASN1Value::UniversalString(v) => {
            state.write_u16(HASH_PREFIX_STR);
            let res: Result<String, char> =
                x520_stringprep_case_ignore_univ_str(trim_u32(&v.0)).collect();
            match res {
                Ok(s) => s.hash(state),
                Err(_) => v.0.hash(state),
            }
        }
        ASN1Value::VideotexString(v) => {
            state.write_u16(HASH_PREFIX_STR);
            v.hash(state);
        }
    }
}

fn hash_instance_of<H: std::hash::Hasher>(value: &INSTANCE_OF, state: &mut H, depth: usize) {
    value.type_id.hash(state);
    state.write_u8(0xFF);
    hash_asn1_value(&value.value, state, depth + 1)
}

const HASH_PREFIX_TELEPHONE_NUMBER: TagNumber = 1800;
const HASH_PREFIX_EMAIL: TagNumber = 2525;
const TELEPHONE_NUMBER_TAG: TagNumber = UNIV_TAG_PRINTABLE_STRING;
const EMAIL_TAG: TagNumber = UNIV_TAG_UTF8_STRING;

fn hash_attr_value_ex<H: std::hash::Hasher>(
    self_: &AttributeValue,
    state: &mut H,
    depth: usize,
    allow_dn: bool,
) {
    if depth > MAX_DEPTH {
        return;
    }
    // If its not universal syntax, attempt an ignorant hash of the element.
    if self_.0.tag.tag_class != TagClass::UNIVERSAL {
        return self_.0.hash(state);
    }

    if self_.0.tag.tag_number == TELEPHONE_NUMBER_TAG {
        // Find the first primitively-constructed chunk of the value,
        // that has non-zero length content octets, or the first error.
        let a_start = self_
            .0
            .iter_deconstruction()
            .find(|x| x.is_err() || x.as_ref().is_ok_and(|x| x.as_ref().len() > 0));
        let a_bytes = match a_start {
            Some(Ok(x)) => x,
            _ => Cow::Borrowed([].as_slice()),
        };
        // If that first chunk starts with '+'...
        if a_bytes.as_ref().first() == Some(&b'+') {
            let a_bytes = match deconstruct(&self_.0) {
                Ok(x) => x,
                Err(_) => Cow::Borrowed([b'?'].as_slice()),
            };
            // ...and the rest looks like a telephone number...
            // (Allowed to contain numbers, spaces, and hyphens.)
            let looks_like_a_tel = a_bytes[1..]
                .iter()
                .cloned()
                .all(|b| matches!(b, b'0'..=b'9' | b' ' | b'-'));
            if looks_like_a_tel {
                // ...then hash it as a telephone number.
                state.write_u16(HASH_PREFIX_TELEPHONE_NUMBER);
                return a_bytes
                    .as_ref()
                    .iter()
                    .filter(|b| b.is_ascii_digit())
                    .for_each(|b| b.hash(state));
            }
        }
    }
    if self_.0.tag.tag_number == EMAIL_TAG {
        let looks_like_email = self_
            .0
            .iter_deconstruction()
            .any(|x| x.is_ok_and(|y| y.contains(&b'@')));
        if looks_like_email {
            let maybe_email = deconstruct(&self_.0).map_err(|_| ()).and_then(|decon| {
                let s = std::str::from_utf8(decon.as_ref()).map_err(|_| ())?;
                EmailAddress::from_str(s).map_err(|_| ())
            });
            if let Ok(email) = maybe_email {
                state.write_u16(HASH_PREFIX_EMAIL);
                return email.hash(state);
            }
        } else {
            // Could be a DNS name.
            let looks_like_punycoded_dns = self_.0.iter_deconstruction().any(|x| {
                x.is_ok_and(|y| {
                    y
                        // Yes, this is how you do a substring search in Rust. -_-
                        .windows(4)
                        .position(|window| window == b"xn--")
                        .is_some()
                })
            });
            // Only if it is punycoded do we need to do any transformation.
            // Otherwise, the normal string handling should be just fine.
            if looks_like_punycoded_dns {
                let maybe_dns_name = deconstruct(&self_.0).map_err(|_| ()).and_then(|decon| {
                    let s = std::str::from_utf8(decon.as_ref()).map_err(|_| ())?;
                    Ok(DNSLabelIter::new(s)
                        .map(|label| {
                            if label.starts_with("xn--") {
                                punycode::decode(label.as_ref())
                                    .unwrap_or(label.as_ref().to_owned())
                            } else {
                                label.as_ref().to_owned()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("."))
                });
                if let Ok(dns_name) = maybe_dns_name {
                    state.write_u16(HASH_PREFIX_STR);
                    if let Ok(s) = x520_stringprep_to_case_ignore_string(dns_name.as_ref()) {
                        return s.hash(state);
                    }
                }
            }
        }
    }

    let maybe_str = get_string(&self_.0);
    if let Some(str_result) = maybe_str {
        if let Ok(s) = str_result {
            if let Ok(ps) = x520_stringprep_to_case_ignore_string(s.as_ref().trim()) {
                state.write_u16(HASH_PREFIX_STR);
                return ps.hash(state);
            } else {
                return self_.0.hash(state);
            }
        } else {
            return self_.0.hash(state);
        }
    }

    let maybe_t1 = get_time(&self_.0);
    if let Some(t1r) = maybe_t1 {
        if let Ok(t1) = t1r {
            state.write_u16(HASH_PREFIX_DATETIME);
            return t1.hash(state);
        } else {
            return self_.0.hash(state);
        }
    }

    match self_.0.tag.tag_number {
        wildboar_asn1::UNIV_TAG_BOOLEAN => match BER.decode_boolean(&self_.0) {
            Ok(b) => {
                state.write_u16(self_.0.tag.tag_number);
                state.write_u8(if b { 0xFF } else { 0 })
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_INTEGER
        | wildboar_asn1::UNIV_TAG_ENUMERATED
        | wildboar_asn1::UNIV_TAG_OBJECT_IDENTIFIER
        | wildboar_asn1::UNIV_TAG_RELATIVE_OID
        | wildboar_asn1::UNIV_TAG_DATE
        | wildboar_asn1::UNIV_TAG_TIME_OF_DAY => match primitive(&self_.0) {
            Ok(b) => {
                state.write_u16(self_.0.tag.tag_number);
                b.as_ref().hash(state)
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_OCTET_STRING | wildboar_asn1::UNIV_TAG_VIDEOTEX_STRING => {
            state.write_u16(self_.0.tag.tag_number);
            for chunk in self_.0.iter_deconstruction() {
                match chunk {
                    Ok(x) => x.as_ref().hash(state),
                    Err(_) => return self_.0.hash(state),
                }
            }
        }
        wildboar_asn1::UNIV_TAG_OID_IRI | wildboar_asn1::UNIV_TAG_RELATIVE_OID_IRI => {
            match BER.decode_utf8_string(&self_.0) {
                Ok(s) => {
                    state.write_u16(self_.0.tag.tag_number);
                    s.trim().to_lowercase().hash(state)
                }
                Err(_) => self_.0.hash(state),
            }
        }
        wildboar_asn1::UNIV_TAG_BIT_STRING => match BER.decode_bit_string(&self_.0) {
            Ok(bs) => {
                state.write_u16(self_.0.tag.tag_number);
                bs.hash(state)
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_NULL => state.write_u16(wildboar_asn1::UNIV_TAG_NULL),
        wildboar_asn1::UNIV_TAG_EXTERNAL => match BER.decode_external(&self_.0) {
            Ok(x) => {
                state.write_u16(self_.0.tag.tag_number);
                x.identification.hash(state);
                match x.data_value {
                    ExternalEncoding::octet_aligned(oct) => {
                        state.write_u8(0);
                        oct.hash(state)
                    }
                    ExternalEncoding::arbitrary(bin) => {
                        state.write_u8(1);
                        bin.hash(state)
                    }
                    ExternalEncoding::single_ASN1_type(_) => {
                        state.write_u8(2);
                        self_.0.hash(state)
                    }
                }
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_REAL => match BER.decode_real(&self_.0) {
            Ok(r) => {
                state.write_u16(self_.0.tag.tag_number);
                r.to_string().hash(state)
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_EMBEDDED_PDV => match BER.decode_embedded_pdv(&self_.0) {
            Ok(x) => {
                state.write_u16(self_.0.tag.tag_number);
                x.identification.hash(state);
                x.data_value.hash(state)
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_SEQUENCE => {
            // Try to decode it as a distinguished name.
            if allow_dn && _validate_RDNSequence(&self_.0).is_ok() {
                let maybe_dn1 = _decode_RDNSequence(&self_.0);
                if maybe_dn1.is_ok() {
                    state.write_u8(0xFE);
                    return maybe_dn1.unwrap().hash(state);
                }
            }
            let comps1 = match self_.0.value.components() {
                Ok(x) => x.as_ref().to_owned(),
                Err(_) => return self_.0.hash(state),
            };
            state.write_u16(self_.0.tag.tag_number);
            comps1.iter().for_each(|c| {
                hash_attr_value_ex(&AttributeValue(c.clone()), state, depth + 1, allow_dn)
            })
        }
        wildboar_asn1::UNIV_TAG_SET => {
            let comps1 = match self_.0.value.components() {
                Ok(x) => x,
                Err(_) => return self_.0.hash(state),
            };
            if comps1.len() <= 1 {
                // Avoid cloning and sorting if we do not have to.
                state.write_u16(self_.0.tag.tag_number);
                return comps1.iter().for_each(|c| c.hash(state));
            }
            comps1.as_ref().to_owned().sort_by(|a, b| a.tag.cmp(&b.tag));
            comps1.iter().for_each(|c| {
                hash_attr_value_ex(&AttributeValue(c.clone()), state, depth + 1, allow_dn)
            })
        }
        wildboar_asn1::UNIV_TAG_NUMERIC_STRING => {
            state.write_u16(self_.0.tag.tag_number);
            for chunk in self_.0.iter_deconstruction() {
                match chunk {
                    Ok(x) => x
                        .as_ref()
                        .iter()
                        .filter(|b| b.is_ascii_digit())
                        .for_each(|b| b.hash(state)),
                    Err(_) => return self_.0.hash(state),
                }
            }
        }
        wildboar_asn1::UNIV_TAG_BMP_STRING => match BER.decode_bmp_string(&self_.0) {
            Ok(x) => {
                let s_result: Result<String, char> =
                    x520_stringprep_case_ignore_bmp(trim_u16(x.0.as_slice())).collect();
                if let Ok(s) = s_result {
                    state.write_u16(self_.0.tag.tag_number);
                    s.hash(state)
                } else {
                    self_.0.hash(state)
                }
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING => match BER.decode_universal_string(&self_.0) {
            Ok(x) => {
                let s_result: Result<String, char> =
                    x520_stringprep_case_ignore_univ_str(trim_u32(x.0.as_slice())).collect();
                if let Ok(s) = s_result {
                    state.write_u16(self_.0.tag.tag_number);
                    s.hash(state)
                } else {
                    self_.0.hash(state)
                }
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_CHARACTER_STRING => match BER.decode_character_string(&self_.0) {
            Ok(x) => {
                state.write_u16(self_.0.tag.tag_number);
                x.identification.hash(state);
                x.string_value.hash(state)
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_DURATION => match BER.decode_duration(&self_.0) {
            Ok(d) => {
                state.write_u16(self_.0.tag.tag_number);
                state.write_u64(d.to_approximate_seconds())
            }
            Err(_) => self_.0.hash(state),
        },
        wildboar_asn1::UNIV_TAG_TIME => match BER.decode_time(&self_.0) {
            Ok(t) => {
                state.write_u16(self_.0.tag.tag_number);
                t.hash(state)
            }
            Err(_) => self_.0.hash(state),
        },
        _ => self_.0.hash(state),
    }
}

#[inline]
fn hash_attr_value<H: std::hash::Hasher>(a: &AttributeValue, state: &mut H, allow_dn: bool) {
    hash_attr_value_ex(a, state, 0, allow_dn)
}

impl Hash for AttributeValue {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_attr_value(self, state, true)
    }
}

impl Hash for AttributeTypeAndValue {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_.hash(state);
        state.write_u8(0xFF);
        // DN-typed values are expressly forbidden from being
        // values in a distinguished name. We have to call this
        // function specifically to set allow_dn to false.
        hash_attr_value_ex(&AttributeValue(self.value.clone()), state, 0, false)
    }
}

fn hash<H: std::hash::Hasher>(rdn: &[AttributeTypeAndValue], state: &mut H) {
    if rdn.len() <= 1 {
        rdn.iter().for_each(|atav| {
            state.write_u8(0xFF);
            atav.hash(state)
        });
    } else {
        let mut sorted = rdn.to_owned();
        sorted.sort_by(|a, b| a.type_.cmp(&b.type_));
        sorted.iter().for_each(|atav| {
            state.write_u8(0xFF);
            atav.hash(state)
        });
    }
}

fn hash_rdn_sequence<H: std::hash::Hasher>(rdns: &[RelativeDistinguishedName], state: &mut H) {
    rdns.iter().for_each(|c| {
        state.write_u8(0xFE);
        c.hash(state)
    });
}

impl Hash for Name {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Name::rdnSequence(x) => {
                state.write_u8(0);
                hash_rdn_sequence(x.as_slice(), state)
            }
            Name::dnsName(x) => {
                state.write_u8(1);
                DNSLabelIter::new(x).for_each(|label| label.hash(state));
            }
            Name::oid(x) => {
                state.write_u8(2);
                x.hash(state)
            }
        }
    }
}

impl Hash for GeneralName {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            GeneralName::otherName(x) => {
                state.write_u8(0);
                hash_instance_of(x, state, 0)
            }
            GeneralName::rfc822Name(v) => {
                state.write_u8(1);
                if let Ok(email) = EmailAddress::from_str(v.trim()) {
                    email.hash(state)
                } else {
                    v.trim().to_lowercase().hash(state)
                }
            }
            GeneralName::dNSName(v) => {
                state.write_u8(2);
                DNSLabelIter::new(v).for_each(|label| label.hash(state))
            }
            GeneralName::x400Address(v) => {
                state.write_u8(3);
                v.hash(state)
            }
            GeneralName::directoryName(v) => {
                state.write_u8(4);
                v.hash(state)
            }
            GeneralName::ediPartyName(v) => {
                state.write_u8(5);
                v.hash(state)
            }
            GeneralName::uniformResourceIdentifier(v) => {
                state.write_u8(6);
                v.trim().to_lowercase().hash(state)
            }
            GeneralName::iPAddress(v) => {
                state.write_u8(7);
                v.hash(state)
            }
            GeneralName::registeredID(v) => {
                state.write_u8(8);
                v.hash(state)
            }
            GeneralName::_unrecognized(v) => {
                state.write_u8(0xFF);
                v.hash(state)
            }
        }
    }
}

impl Hash for EDIPartyName {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(na_name) = &self.nameAssigner {
            match get_string(&na_name) {
                Some(Ok(na_str)) => {
                    x520_stringprep_to_case_ignore_string(na_str.as_ref().trim()).hash(state)
                }
                _ => na_name.hash(state),
            };
        }
        state.write_u8(0xFA);
        match get_string(&self.partyName) {
            Some(Ok(pa_str)) => {
                x520_stringprep_to_case_ignore_string(pa_str.as_ref().trim()).hash(state)
            }
            _ => self.partyName.hash(state),
        }
    }
}

impl Hash for Time {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Time::utcTime(ut) => {
                state.write_u8(0xFC);
                match utctime_to_chrono(ut) {
                    Ok(t) => t.to_utc().hash(state),
                    Err(_) => ut.hash(state),
                }
            }
            Time::generalizedTime(gt) => {
                state.write_u8(0xFB);
                match gt_to_chrono(gt) {
                    Ok(t) => t.to_utc().hash(state),
                    Err(_) => gt.hash(state),
                }
            }
        }
    }
}

impl Hash for Validity {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(0);
        self.notBefore.hash(state);
        state.write_u8(1);
        self.notAfter.hash(state);
    }
}

impl Hash for AttCertValidityPeriod {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(0xFE);
        match gt_to_chrono(&self.notBeforeTime) {
            Ok(t) => t.to_utc().hash(state),
            Err(_) => self.notBeforeTime.hash(state),
        };
        state.write_u8(0xFF);
        match gt_to_chrono(&self.notAfterTime) {
            Ok(t) => t.to_utc().hash(state),
            Err(_) => self.notAfterTime.hash(state),
        };
    }
}

impl<T> Hash for SIGNED<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.algorithmIdentifier.hash(state);
        state.write_u8(0xFF);
        self.signature.hash(state);
    }
}

impl Hash for TBSCertificate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.issuer.hash(state);
        state.write_u8(0xFF);
        self.serialNumber.hash(state);
    }
}

impl Hash for IssuerSerial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.issuer.hash(state);
        state.write_u8(0xFF);
        self.serial.hash(state);
        if let Some(uid) = &self.issuerUID {
            state.write_u8(0xFE);
            uid.hash(state);
        }
    }
}

impl Hash for AlgorithmIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.algorithm.hash(state);
        if let Some(param) = &self.parameters {
            state.write_u8(0xFF);
            param.hash(state);
        }
    }
}

impl Hash for ObjectDigestInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.digestedObjectType.hash(state);
        state.write_u8(0xFF);
        if let Some(ootid) = &self.otherObjectTypeID {
            state.write_u8(0xFE);
            ootid.hash(state);
        }
        state.write_u8(0xFD);
        self.digestAlgorithm.hash(state);
        state.write_u8(0xFC);
        self.objectDigest.hash(state);
    }
}

impl Hash for AttCertIssuer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(issuer_name) = &self.issuerName {
            state.write_u8(0xFF);
            issuer_name.hash(state);
        }
        if let Some(base_cert_id) = &self.baseCertificateID {
            state.write_u8(0xFE);
            base_cert_id.hash(state);
        }
        if let Some(odi) = &self.objectDigestInfo {
            state.write_u8(0xFD);
            odi.hash(state);
        }
    }
}

impl Hash for TBSAttributeCertificate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.issuer.hash(state);
        state.write_u8(0xFF);
        self.serialNumber.hash(state);
    }
}

impl Hash for TBSCertAVL {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.issuer.hash(state);
        state.write_u8(0xFF);
        self.serialNumber.hash(state);
    }
}

impl Hash for IssuerSerialNumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.issuer.hash(state);
        state.write_u8(0xFF);
        self.serialNumber.hash(state);
    }
}

impl Hash for FingerPrint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.algorithmIdentifier.hash(state);
        state.write_u8(0xFF);
        self.fingerprint.hash(state);
    }
}

impl Hash for PKCertIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            PKCertIdentifier::issuerSerialNumber(x) => {
                state.write_u8(0xF0);
                x.hash(state);
            }
            PKCertIdentifier::fingerprintPKC(x) => {
                state.write_u8(0xF1);
                x.hash(state);
            }
            PKCertIdentifier::fingerprintPK(x) => {
                state.write_u8(0xF2);
                x.hash(state);
            }
            PKCertIdentifier::_unrecognized(x) => {
                state.write_u8(0xF3);
                x.hash(state);
            }
        }
    }
}

impl Hash for AlgorithmWithInvoke {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.algorithm.hash(state);
        if let Some(param) = &self.parameters {
            state.write_u8(0xFF);
            param.hash(state);
        }
        if let Some(param) = &self.dynamParms {
            state.write_u8(0xFE);
            param.hash(state);
        }
    }
}

impl Hash for HASH {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.algorithmIdentifier.hash(state);
        state.write_u8(0xFF);
        self.hashValue.hash(state);
    }
}
