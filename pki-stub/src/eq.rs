use crate::{GeneralNames, RDNSequence, RelativeDistinguishedName, _encode_EDIPartyName};
use crate::PKI_Stub::{
    AttributeValue,
    AttributeTypeAndValue,
    Name,
    GeneralName,
    Certificate,
    AttributeCertificate,
    SIGNED,
    TBSCertAVL,
    TBSCertificate,
    TBSAttributeCertificate,
    IssuerSerialNumber,
    PKCertIdentifier,
    AlgorithmIdentifier,
    AlgorithmWithInvoke,
    FingerPrint,
    HASH,
    EDIPartyName,
    AttCertIssuer,
    IssuerSerial,
    ObjectDigestInfo,
    _encode_GeneralName,
};
use teletex::teletex_to_utf8;
use wildboar_asn1::{
    compare_numeric_string,
    ExternalEncoding,
    PresentationContextSwitchingTypeIdentification,
    Tag,
    TagClass,
    TagNumber,
    OBJECT_IDENTIFIER,
    UNIV_TAG_BMP_STRING,
    UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_PRINTABLE_STRING,
    UNIV_TAG_UTF8_STRING,
};
use x690::x690_write_tlv;
use x690::{ber::BER, X690Codec, X690Element, primitive, deconstruct};
use chrono::{DateTime, Utc, TimeZone, FixedOffset, Local};
use x520_stringprep::{
    x520_stringprep_case_ignore_compare,
    x520_stringprep_case_ignore_str,
    x520_stringprep_case_ignore_bmp,
    x520_stringprep_case_ignore_univ_str,
    X520CaseIgnoreStringPrepChars,
    x520_stringprep_to_case_ignore_string,
};
use std::borrow::Cow;
use std::str::{Chars, Split};
use std::collections::{HashMap, HashSet};
use std::iter::{Iterator, FusedIterator};
use email_address::EmailAddress;
use std::str::FromStr;

/// Returns a subslice with leading and trailing whitespace removed, for a slice of u16 code units (BMPString).
pub(crate) fn trim_u16(slice: &[u16]) -> &[u16] {
    let is_ws = |c: u16| char::from_u32(c as u32).map_or(false, |ch| ch.is_whitespace());
    let mut start = 0;
    let mut end = slice.len();

    // Find first non-whitespace from the start
    while start < end && is_ws(slice[start]) {
        start += 1;
    }
    // Find first non-whitespace from the end
    while end > start && is_ws(slice[end - 1]) {
        end -= 1;
    }
    &slice[start..end]
}

/// Returns a subslice with leading and trailing whitespace removed, for a slice of u32 code points (UniversalString).
pub(crate) fn trim_u32(slice: &[u32]) -> &[u32] {
    let is_ws = |c: u32| char::from_u32(c).map_or(false, |ch| ch.is_whitespace());
    let mut start = 0;
    let mut end = slice.len();

    // Find first non-whitespace from the start
    while start < end && is_ws(slice[start]) {
        start += 1;
    }
    // Find first non-whitespace from the end
    while end > start && is_ws(slice[end - 1]) {
        end -= 1;
    }
    &slice[start..end]
}

pub(crate)struct DNSLabelIter<'a> {
    inner: std::str::Split<'a, char>,
}

impl <'a> DNSLabelIter<'a> {
    pub fn new(s: &'a str) -> Self {
        DNSLabelIter{
            inner: s.split('.'),
        }
    }
}

impl <'a> Iterator for DNSLabelIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        let label = self.inner.next()?;
        if label.starts_with("xn--") {
            if let Ok(punycoded) = punycode::encode(label.split_at(4).1) {
                Some(Cow::Owned(punycoded))
            } else {
                Some(Cow::Borrowed(label))
            }
        } else {
            Some(Cow::Borrowed(label))
        }
    }

}

fn dns_compare(a: &str, b: &str) -> bool {
    let mut a = DNSLabelIter::new(a);
    let mut b = DNSLabelIter::new(b);
    while let Some(alabel) = a.next() {
        if let Some(blabel) = b.next() {
            if !alabel.as_ref().eq_ignore_ascii_case(blabel.as_ref())
                && alabel != "*"
                && blabel != "*" {
                return false;
            }
        } else {
            return false;
        }
    }
    b.next().is_none()
}

impl <'a> FusedIterator for DNSLabelIter<'a> {}

const FEW_ATAVS: usize = 4;
fn rdn_compare(a: &RelativeDistinguishedName, b: &RelativeDistinguishedName) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let few = a.len() < FEW_ATAVS;
    if few {
        // If there are few ATAVs, we just do a naive O(n^2) approach.
        for a_atav in a {
            if !b.iter().any(|b_atav| a_atav == b_atav) {
                return false;
            }
        }
        return true;
    }

    let mut a_atavs: HashMap<&OBJECT_IDENTIFIER, &AttributeTypeAndValue> = HashMap::with_capacity(a.len());
    for a_atav in a {
        if a_atavs.insert(&a_atav.type_, a_atav).is_some() {
            return false; // Invalid: duplicate attribute types
        }
    }
    for b_atav in b {
        match a_atavs.remove(&b_atav.type_) {
            Some(a_atav) if a_atav == b_atav => continue,
            _ => return false, // Invalid: attribute type not found
        }
    }
    a_atavs.is_empty() // If not empty, there are unmatched attributes
}

fn rdn_seq_compare(a: &RDNSequence, b: &RDNSequence) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (a_rdn, b_rdn) in a.iter().zip(b.iter()) {
        if !rdn_compare(a_rdn, b_rdn) {
            return false;
        }
    }
    true
}

impl PartialEq for AlgorithmIdentifier {

    fn eq(&self, other: &Self) -> bool {
        self.algorithm == other.algorithm
        && self.parameters == other.parameters
    }

}

impl PartialEq for HASH {

    fn eq(&self, other: &Self) -> bool {
        self.algorithmIdentifier == other.algorithmIdentifier
        && self.hashValue == other.hashValue
    }

}

impl PartialEq for AlgorithmWithInvoke {

    fn eq(&self, other: &Self) -> bool {
        self.algorithm == other.algorithm
        && self.parameters == other.parameters
        && self.dynamParms == other.dynamParms
    }

}

impl PartialEq for FingerPrint {

    fn eq(&self, other: &Self) -> bool {
        self.algorithmIdentifier == other.algorithmIdentifier
        && self.fingerprint == other.fingerprint
    }

}

pub(crate) fn get_time(el: &X690Element) -> Option<Result<DateTime<Utc>, ()>> {
    match el.tag.tag_number {
        wildboar_asn1::UNIV_TAG_UTC_TIME => {
            let t = match BER.decode_utc_time(el) {
                Ok(x) => x,
                Err(_) => return Some(Err(())),
            };
            let maybe_tz = FixedOffset::east_opt(
                (t.utc_offset.hour as i32 * 3600)
                + t.utc_offset.minute as i32
            );
            let tz = match maybe_tz {
                Some(x) => x,
                None => return Some(Err(())),
            };
            let maybe_t = tz.with_ymd_and_hms(
                t.year as i32,
                t.month as u32,
                t.day as u32,
                t.hour as u32,
                t.minute as u32,
                t.second as u32,
            ).earliest();
            let t = match maybe_t {
                Some(x) => x,
                None => return Some(Err(())),
            };
            Some(Ok(t.to_utc()))
        },
        wildboar_asn1::UNIV_TAG_GENERALIZED_TIME => {
            let t = match BER.decode_generalized_time(el) {
                Ok(x) => x,
                Err(_) => return Some(Err(())),
            };
            let (min, maybe_sec) = t.min_and_sec.unwrap_or((0, Some(0)));
            let sec = maybe_sec.unwrap_or(0);
            if let Some(offset) = t.utc_offset.as_ref() {
                let maybe_tz = FixedOffset::east_opt(
                    (offset.hour as i32 * 3600)
                    + offset.minute as i32
                );
                let tz = match maybe_tz {
                    Some(x) => x,
                    None => return Some(Err(())),
                };
                let maybe_t = tz.with_ymd_and_hms(
                    t.date.year as i32,
                    t.date.month as u32,
                    t.date.day as u32,
                    t.hour as u32,
                    min as u32,
                    sec as u32,
                ).earliest();
                let t = match maybe_t {
                    Some(x) => x,
                    None => return Some(Err(())),
                };
                Some(Ok(t.to_utc()))
            } else {
                let maybe_t = Local.with_ymd_and_hms(
                    t.date.year as i32,
                    t.date.month as u32,
                    t.date.day as u32,
                    t.hour as u32,
                    min as u32,
                    sec as u32,
                ).earliest();
                let t = match maybe_t {
                    Some(x) => x,
                    None => return Some(Err(())),
                };
                Some(Ok(t.to_utc()))
            }
        },
        wildboar_asn1::UNIV_TAG_DATE_TIME => {
            let t = match BER.decode_date_time(el) {
                Ok(x) => x,
                Err(_) => return Some(Err(())),
            };
            let maybe_t = Local.with_ymd_and_hms(
                t.date.year as i32,
                t.date.month as u32,
                t.date.day as u32,
                t.time.hour as u32,
                t.time.minute as u32,
                t.time.second as u32,
            ).earliest();
            let t = match maybe_t {
                Some(x) => x,
                None => return Some(Err(())),
            };
            Some(Ok(t.to_utc()))
        },
        _ => None,
    }
}

pub(crate) fn get_string(el: &X690Element) -> Option<Result<Cow<str>, ()>> {
    debug_assert!(el.tag.tag_class == TagClass::UNIVERSAL);
    match el.tag.tag_number {
        wildboar_asn1::UNIV_TAG_UTF8_STRING
        | wildboar_asn1::UNIV_TAG_PRINTABLE_STRING
        | wildboar_asn1::UNIV_TAG_IA5_STRING
        | wildboar_asn1::UNIV_TAG_GRAPHIC_STRING
        | wildboar_asn1::UNIV_TAG_VISIBLE_STRING
        | wildboar_asn1::UNIV_TAG_GENERAL_STRING
        | wildboar_asn1::UNIV_TAG_OBJECT_DESCRIPTOR
        => {
            let ds = match deconstruct(el) {
                Ok(x) => x,
                Err(_) => return Some(Err(())),
            };
            match ds {
                Cow::Borrowed(bs) => {
                    let s = match std::str::from_utf8(bs) {
                        Ok(x) => x,
                        Err(_) => return Some(Err(())),
                    };
                    Some(Ok(Cow::Borrowed(s)))
                },
                Cow::Owned(bs) => {
                    let s = match String::from_utf8(bs) {
                        Ok(x) => x,
                        Err(_) => return Some(Err(())),
                    };
                    Some(Ok(Cow::Owned(s)))
                }
            }
        },
        wildboar_asn1::UNIV_TAG_T61_STRING => {
            let ds: std::borrow::Cow<'_, [u8]> = match deconstruct(el) {
                Ok(x) => x,
                Err(_) => return Some(Err(())),
            };
            match ds {
                Cow::Borrowed(bs) => {
                    if bs.is_ascii() {
                        Some(Ok(Cow::Borrowed(std::str::from_utf8(bs).unwrap())))
                    } else {
                        Some(Ok(teletex_to_utf8(bs)))
                    }
                },
                Cow::Owned(bs) => {
                    if bs.as_slice().is_ascii() {
                        Some(Ok(Cow::Owned(String::from_utf8(bs).unwrap())))
                    } else {
                        Some(Ok(teletex_to_utf8(bs.as_slice()).into_owned().into()))
                    }
                }
            }
        },
        _ => None,
    }
}

const MAX_DEPTH: usize = 20;
const BIG_STRING: usize = 128;
fn compare_attr_value_ex(self_: &AttributeValue, other: &AttributeValue, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        return false;
    }
    // If its not universal syntax, attempt a naive byte-for-byte compare.
    if self_.0.tag.tag_class != TagClass::UNIVERSAL {
        return self_.0 == other.0;
    }

    let maybe_str1 = get_string(&self_.0);
    let maybe_str2 = get_string(&other.0);
    match (maybe_str1.as_ref(), maybe_str2.as_ref()) {
        (Some(s1r), Some(s2r)) => match (s1r, s2r) {
            (Ok(s1), Ok(s2)) => {
                // If the strings are big enough, we just allocate them so we can amortize
                // the benefits of cache locality, SIMD, memcmp implementation, etc.
                if s1.len() > BIG_STRING || s2.len() > BIG_STRING {
                    let s1 = match x520_stringprep_to_case_ignore_string(s1) {
                        Ok(x) => x,
                        Err(_) => return false,
                    };
                    let s2 = match x520_stringprep_to_case_ignore_string(s2) {
                        Ok(x) => x,
                        Err(_) => return false,
                    };
                    return s1 == s2;
                }
                // ...otherwise (small strings), we just compare character-by-character
                return x520_stringprep_case_ignore_compare(s1.as_ref().trim(), s2.as_ref().trim());
            },
            _ => return false,
        },
        _ => (),
    };
    let mut prep_str1: Option<X520CaseIgnoreStringPrepChars<Chars<'_>>> = maybe_str1
        .as_ref()
        .map(|s| s.as_ref().ok().map(|s| x520_stringprep_case_ignore_str(s.as_ref().trim()))).flatten();
    let mut prep_str2: Option<X520CaseIgnoreStringPrepChars<Chars<'_>>> = maybe_str2
        .as_ref()
        .map(|s| s.as_ref().ok().map(|s| x520_stringprep_case_ignore_str(s.as_ref().trim()))).flatten();

    let maybe_t1 = get_time(&self_.0);
    let maybe_t2 = get_time(&other.0);
    match (maybe_t1, maybe_t2) {
        (Some(t1r), Some(t2r)) => match (t1r, t2r) {
            (Ok(t1), Ok(t2)) => return t1 == t2,
            _ => return false,
        },
        (None, None) => (),
        _ => return false,
    };

    match (self_.0.tag.tag_number, other.0.tag.tag_number) {
        // These can be compared byte-for-byte.
        (wildboar_asn1::UNIV_TAG_INTEGER, wildboar_asn1::UNIV_TAG_INTEGER)
        | (wildboar_asn1::UNIV_TAG_ENUMERATED, wildboar_asn1::UNIV_TAG_ENUMERATED)
        | (wildboar_asn1::UNIV_TAG_OBJECT_IDENTIFIER, wildboar_asn1::UNIV_TAG_OBJECT_IDENTIFIER)
        | (wildboar_asn1::UNIV_TAG_RELATIVE_OID, wildboar_asn1::UNIV_TAG_RELATIVE_OID)
        | (wildboar_asn1::UNIV_TAG_DATE, wildboar_asn1::UNIV_TAG_DATE)
        | (wildboar_asn1::UNIV_TAG_TIME_OF_DAY, wildboar_asn1::UNIV_TAG_TIME_OF_DAY)
        | (wildboar_asn1::UNIV_TAG_TIME, wildboar_asn1::UNIV_TAG_TIME) // This is a wild guess.
        => {
            let prim1 = match primitive(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let prim2 = match primitive(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            return prim1.as_ref() == prim2.as_ref();
        },
        (wildboar_asn1::UNIV_TAG_OCTET_STRING, wildboar_asn1::UNIV_TAG_OCTET_STRING) => {
            let prim1 = match deconstruct(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let prim2 = match deconstruct(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            return prim1.as_ref() == prim2.as_ref();
        },
        (wildboar_asn1::UNIV_TAG_BMP_STRING, wildboar_asn1::UNIV_TAG_BMP_STRING) => {
            let (decon1, decon2) = match (deconstruct(&self_.0), deconstruct(&other.0)) {
                (Ok(x), Ok(y)) => (x, y),
                _ => return false,
            };
            let u16s1: &[u16] = match bytemuck::try_cast_slice(decon1.as_ref()) {
                Ok(x) => trim_u16(x),
                Err(_) => return false,
            };
            let u16s2: &[u16] = match bytemuck::try_cast_slice(decon2.as_ref()) {
                Ok(x) => trim_u16(x),
                Err(_) => return false,
            };
            let mut s1 = x520_stringprep_case_ignore_bmp(u16s1);
            let mut s2 = x520_stringprep_case_ignore_bmp(u16s2);
            loop {
                match (s1.next(), s2.next()) {
                    (None, None) => return true,
                    (Some(Ok(x)), Some(Ok(y))) if x == y => continue,
                    (Some(Ok(_)), Some(Ok(_))) => return false,
                    (Some(Err(e)), _) | (_, Some(Err(e))) => return false,
                    _ => return false,
                }
            }
        },
        (wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING, wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING) => {
            let (decon1, decon2) = match (deconstruct(&self_.0), deconstruct(&other.0)) {
                (Ok(x), Ok(y)) => (x, y),
                _ => return false,
            };
            let u32s1: &[u32] = match bytemuck::try_cast_slice(decon1.as_ref()) {
                Ok(x) => trim_u32(x),
                Err(_) => return false,
            };
            let u32s2: &[u32] = match bytemuck::try_cast_slice(decon2.as_ref()) {
                Ok(x) => trim_u32(x),
                Err(_) => return false,
            };
            let mut s1 = x520_stringprep_case_ignore_univ_str(u32s1);
            let mut s2 = x520_stringprep_case_ignore_univ_str(u32s2);
            loop {
                match (s1.next(), s2.next()) {
                    (None, None) => return true,
                    (Some(Ok(x)), Some(Ok(y))) if x == y => continue,
                    (Some(Ok(_)), Some(Ok(_))) => return false,
                    (Some(Err(e)), _) | (_, Some(Err(e))) => return false,
                    _ => return false,
                }
            }
        },
        (wildboar_asn1::UNIV_TAG_BMP_STRING, _) | (_, wildboar_asn1::UNIV_TAG_BMP_STRING) => {
            debug_assert!(prep_str1.as_ref().and(prep_str2.as_ref()).is_none());
            let mut prep_str;
            let decon_result: Result<Cow<'_, [u8]>, wildboar_asn1::ASN1Error> = if self_.0.tag.tag_number == UNIV_TAG_BMP_STRING {
                prep_str = prep_str2.as_mut().expect("prep_str2 is not None");
                deconstruct(&self_.0)
            } else {
                prep_str = prep_str1.as_mut().expect("prep_str1 is not None");
                deconstruct(&other.0)
            };
            let decon = match decon_result {
                Ok(x) => x,
                Err(_) => return false,
            };
            let u16s: &[u16] = match bytemuck::try_cast_slice(decon.as_ref()) {
                Ok(x) => trim_u16(x),
                Err(_) => return false,
            };
            let mut bmp = x520_stringprep_case_ignore_bmp(u16s);
            loop {
                match (bmp.next(), prep_str.next()) {
                    (None, None) => return true,
                    (Some(Ok(x)), Some(Ok(y))) if x == y => continue,
                    (Some(Ok(_)), Some(Ok(_))) => return false,
                    (Some(Err(e)), _) | (_, Some(Err(e))) => return false,
                    _ => return false,
                }
            }
        },
        (wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING, _) | (_, wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING) => {
            debug_assert!(prep_str1.as_ref().and(prep_str2.as_ref()).is_none());
            let mut prep_str;
            let decon_result = if self_.0.tag.tag_number == UNIV_TAG_UNIVERSAL_STRING {
                prep_str = prep_str2.as_mut().expect("prep_str2 is not None");
                deconstruct(&self_.0)
            } else {
                prep_str = prep_str1.as_mut().expect("prep_str1 is not None");
                deconstruct(&other.0)
            };
            let decon = match decon_result {
                Ok(x) => x,
                Err(_) => return false,
            };
            let u32s: &[u32] = match bytemuck::try_cast_slice(decon.as_ref()) {
                Ok(x) => trim_u32(x),
                Err(_) => return false,
            };
            let mut ustr= x520_stringprep_case_ignore_univ_str(u32s);
            loop {
                match (ustr.next(), prep_str.next()) {
                    (None, None) => return true,
                    (Some(Ok(x)), Some(Ok(y))) if x == y => continue,
                    (Some(Ok(_)), Some(Ok(_))) => return false,
                    (Some(Err(e)), _) | (_, Some(Err(e))) => return false,
                    _ => return false,
                }
            }
        },
        (wildboar_asn1::UNIV_TAG_NUMERIC_STRING, wildboar_asn1::UNIV_TAG_NUMERIC_STRING) => {
            let prim1 = match deconstruct(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let prim2 = match deconstruct(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let s1 = match std::str::from_utf8(prim1.as_ref()) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let s2 = match std::str::from_utf8(prim2.as_ref()) {
                Ok(x) => x,
                Err(_) => return false,
            };
            return compare_numeric_string(s1, s2);
        },
        (wildboar_asn1::UNIV_TAG_BOOLEAN, wildboar_asn1::UNIV_TAG_BOOLEAN) =>
            return match (BER.decode_boolean(&self_.0), BER.decode_boolean(&other.0)) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            },
        (wildboar_asn1::UNIV_TAG_NULL, wildboar_asn1::UNIV_TAG_NULL) =>
            return match (BER.decode_null(&self_.0), BER.decode_null(&other.0)) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            },
        (wildboar_asn1::UNIV_TAG_BIT_STRING, wildboar_asn1::UNIV_TAG_BIT_STRING) =>
            return match (BER.decode_bit_string(&self_.0), BER.decode_bit_string(&other.0)) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            },
        (wildboar_asn1::UNIV_TAG_BIT_STRING, wildboar_asn1::UNIV_TAG_BIT_STRING) =>
            return match (BER.decode_bit_string(&self_.0), BER.decode_bit_string(&other.0)) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            },
        (wildboar_asn1::UNIV_TAG_OID_IRI, wildboar_asn1::UNIV_TAG_OID_IRI) => {
            let oid1 = match BER.decode_oid_iri(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let oid2 = match BER.decode_oid_iri(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            oid1.to_lowercase() == oid2.to_lowercase()
        },
        (wildboar_asn1::UNIV_TAG_RELATIVE_OID_IRI, wildboar_asn1::UNIV_TAG_RELATIVE_OID_IRI) => {
            let oid1 = match BER.decode_relative_oid_iri(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let oid2 = match BER.decode_relative_oid_iri(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            oid1.to_lowercase() == oid2.to_lowercase()
        },
        (wildboar_asn1::UNIV_TAG_DURATION, wildboar_asn1::UNIV_TAG_DURATION) => {
            let dur1 = match BER.decode_duration(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let dur2 = match BER.decode_duration(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            dur1.to_approximate_seconds() == dur2.to_approximate_seconds()
        },
        (wildboar_asn1::UNIV_TAG_SEQUENCE, wildboar_asn1::UNIV_TAG_SEQUENCE) => {
            let comps1 = match self_.0.value.components() {
                Ok(x) => x,
                Err(_) => return false,
            };
            let comps2 = match other.0.value.components() {
                Ok(x) => x,
                Err(_) => return false,
            };
            if comps1.len() != comps2.len() {
                return false;
            }
            /* Even for a SET, we do not attempt to intep */
            for (c1, c2) in comps1.iter().zip(comps2.iter()) {
                if AttributeValue(c1.clone()) != AttributeValue(c2.clone()) {
                    return false;
                }
            }
            return true;
        },
        (wildboar_asn1::UNIV_TAG_SET, wildboar_asn1::UNIV_TAG_SET) => {
            let comps1 = match self_.0.value.components() {
                Ok(x) => x,
                Err(_) => return false,
            };
            let comps2 = match other.0.value.components() {
                Ok(x) => x,
                Err(_) => return false,
            };
            if comps1.len() != comps2.len() {
                return false;
            }
            let mut comps1_set: HashMap<Tag, &X690Element> = HashMap::with_capacity(comps1.len());
            for comp1 in comps1.iter() {
                if comps1_set.insert(comp1.tag, comp1).is_some() {
                    return false;
                }
            }
            for comp2 in comps2.iter() {
                let comp1 = match comps1_set.remove(&comp2.tag) {
                    Some(x) => x,
                    None => return false,
                };
                if AttributeValue(comp1.clone()) != AttributeValue(comp2.clone()) {
                    return false;
                }
            }
            return true;
        },
        (wildboar_asn1::UNIV_TAG_REAL, wildboar_asn1::UNIV_TAG_REAL) => {
            let real1 = match BER.decode_real(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let real2 = match BER.decode_real(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            real1 == real2
        },
        (wildboar_asn1::UNIV_TAG_EXTERNAL, wildboar_asn1::UNIV_TAG_EXTERNAL) => {
            let ext1 = match BER.decode_external(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let ext2 = match BER.decode_external(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            ext1.identification == ext2.identification
            && ext1.data_value == ext2.data_value
        },
        (wildboar_asn1::UNIV_TAG_EMBEDDED_PDV, wildboar_asn1::UNIV_TAG_EMBEDDED_PDV) => {
            let pdv1 = match BER.decode_embedded_pdv(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let pdv2 = match BER.decode_embedded_pdv(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            pdv1.identification == pdv2.identification
            && pdv1.data_value == pdv2.data_value
        },
        (wildboar_asn1::UNIV_TAG_CHARACTER_STRING, wildboar_asn1::UNIV_TAG_CHARACTER_STRING) => {
            let cs1 = match BER.decode_character_string(&self_.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            let cs2 = match BER.decode_character_string(&other.0) {
                Ok(x) => x,
                Err(_) => return false,
            };
            cs1.identification == cs2.identification
            && cs1.string_value == cs2.string_value
        },
        _ => false,
    }
}

#[inline]
fn compare_attr_value(a: &AttributeValue, b: &AttributeValue) -> bool {
    compare_attr_value_ex(a, b, 0)
}

impl PartialEq for AttributeValue {

    /// This implements matching for the following matching rules:
    /// - `caseIgnoreMatch` for almost all string types
    /// - `caseIgnoreIA5StringMatch` for IA5String
    /// - `numericStringMatch` for `NumericString`
    /// - `objectIdentifierMatch` for `OBJECT IDENTIFIER`
    /// - `integerMatch` for `INTEGER`
    /// - `bitStringMatch` for `BIT STRING`
    /// - `booleanMatch` for `BOOLEAN`
    /// - `utcTimeMatch` for `UTCTime`
    /// - `generalizedTimeMatch` for `GeneralizedTime`
    ///
    /// It also recurses for `SEQUENCE` and `SET` types, and tries to compare
    /// context-switching types.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        compare_attr_value(self, other)
    }

}

fn telephone_number_match_bytes(a: &[u8], b: &[u8]) -> bool {
    let mut a_iter = a.iter().filter(|c| **c != b' ' && **c != b'-');
    let mut b_iter = b.iter().filter(|c| **c != b' ' && **c != b'-');
    for a_char in a_iter {
        if !a_char.is_ascii_digit() {
            return false;
        }
        if let Some(b_char) = b_iter.next() {
            if !b_char.is_ascii_digit() || b_char != a_char {
                return false;
            }
        } else {
            return false;
        }
    }
    b_iter.next().is_none() // If false, trailing digits were unmatched.
}

fn telephone_number_match(a: &X690Element, b: &X690Element) -> bool {
    let a_bytes = match deconstruct(a) {
        Ok(x) => x,
        Err(_) => return false,
    };
    let b_bytes = match deconstruct(b) {
        Ok(x) => x,
        Err(_) => return false,
    };
    if a_bytes.as_ref().first() != Some(&b'+') {
        return false;
    }
    if b_bytes.as_ref().first() != Some(&b'+') {
        return false;
    }
    telephone_number_match_bytes(&a_bytes[1..], &b_bytes[1..])
}

fn dns_or_email_match(a: &X690Element, b: &X690Element) -> bool {
    let a_bytes = match deconstruct(a) {
        Ok(x) => x,
        Err(_) => return false,
    };
    let b_bytes = match deconstruct(b) {
        Ok(x) => x,
        Err(_) => return false,
    };
    let a_str = match std::str::from_utf8(a_bytes.as_ref()) {
        Ok(x) => x,
        Err(_) => return false,
    };
    let b_str = match std::str::from_utf8(b_bytes.as_ref()) {
        Ok(x) => x,
        Err(_) => return false,
    };
    if a_str.contains('@') {
        let a_email = match EmailAddress::from_str(a_str) {
            Ok(x) => x,
            Err(_) => return false,
        };
        let b_email = match EmailAddress::from_str(b_str) {
            Ok(x) => x,
            Err(_) => return false,
        };
        return a_email == b_email;
    }
    dns_compare(a_str, b_str)
}

const TELEPHONE_NUMBER_TAG: Tag = Tag::new(TagClass::UNIVERSAL, UNIV_TAG_PRINTABLE_STRING);
const EMAIL_TAG: Tag = Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING);

impl PartialEq for AttributeTypeAndValue {

    // NOTE: uUIDPairMatch is already covered.
    fn eq(&self, other: &Self) -> bool {
        if self.type_ != other.type_ {
            return false;
        }
        // TODO: Move these to AttributeValue
        if self.value.tag == TELEPHONE_NUMBER_TAG && other.value.tag == TELEPHONE_NUMBER_TAG {
            if telephone_number_match(&self.value, &other.value) {
                return true;
            }
        }
        if self.value.tag == EMAIL_TAG && other.value.tag == EMAIL_TAG {
            if dns_or_email_match(&self.value, &other.value) {
                return true;
            }
        }
        AttributeValue(self.value.clone()) == AttributeValue(other.value.clone())
    }

}

impl PartialEq for Name {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Name::rdnSequence(a), Name::rdnSequence(b)) => rdn_seq_compare(a, b),
            (Name::dnsName(a), Name::dnsName(b)) => dns_compare(a, b),
            (Name::oid(a), Name::oid(b)) => a == b,
            _ => false,
        }
    }

}

impl PartialEq for GeneralName {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GeneralName::otherName(a), GeneralName::otherName(b)) => todo!(),
            (GeneralName::rfc822Name(a), GeneralName::rfc822Name(b)) =>
                match (EmailAddress::from_str(a), EmailAddress::from_str(b)) {
                    (Ok(a), Ok(b)) => a == b,
                    _ => false,
                },
            (GeneralName::dNSName(a), GeneralName::dNSName(b)) => dns_compare(a, b),
            // This is a naive implementation, since we don't actually fully
            // parse the X.400 address.
            (GeneralName::x400Address(a), GeneralName::x400Address(b)) => a == b,
            (GeneralName::directoryName(a), GeneralName::directoryName(b)) => a == b,
            (GeneralName::ediPartyName(a), GeneralName::ediPartyName(b)) => a == b,
            (GeneralName::uniformResourceIdentifier(a), GeneralName::uniformResourceIdentifier(b)) => a.eq_ignore_ascii_case(b),
            (GeneralName::iPAddress(a), GeneralName::iPAddress(b)) => a == b,
            (GeneralName::registeredID(a), GeneralName::registeredID(b)) => a == b,

            (GeneralName::dNSName(a), GeneralName::directoryName(Name::dnsName(b)))
            | (GeneralName::directoryName(Name::dnsName(a)), GeneralName::dNSName(b)) => dns_compare(a.as_str(), b.as_str()),

            (GeneralName::registeredID(a), GeneralName::directoryName(Name::oid(b)))
            | (GeneralName::directoryName(Name::oid(a)), GeneralName::registeredID(b)) => a == b,
            _ => false,
        }
    }

}

impl PartialEq for TBSCertificate {

    fn eq(&self, other: &Self) -> bool {
        self.serialNumber == other.serialNumber
        && self.issuer == other.issuer
    }

}

impl PartialEq for TBSAttributeCertificate {

    fn eq(&self, other: &Self) -> bool {
        self.serialNumber == other.serialNumber
        && self.issuer == other.issuer
    }

}

impl PartialEq for TBSCertAVL {

    fn eq(&self, other: &Self) -> bool {
        self.serialNumber == other.serialNumber
        && self.issuer == other.issuer
    }

}

impl <T> PartialEq for SIGNED<T> {

    fn eq(&self, other: &Self) -> bool {
        self.algorithmIdentifier == other.algorithmIdentifier
        && self.signature == other.signature
    }

}

impl PartialEq for IssuerSerialNumber {

    fn eq(&self, other: &Self) -> bool {
        self.serialNumber == other.serialNumber
        && self.issuer == other.issuer
    }

}

impl PartialEq for PKCertIdentifier {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PKCertIdentifier::issuerSerialNumber(a), PKCertIdentifier::issuerSerialNumber(b)) => a == b,
            (PKCertIdentifier::fingerprintPKC(a), PKCertIdentifier::fingerprintPKC(b)) => a == b,
            (PKCertIdentifier::fingerprintPK(a), PKCertIdentifier::fingerprintPK(b)) => a == b,
            _ => false,
        }
    }

}

impl PartialEq for EDIPartyName {

    fn eq(&self, other: &Self) -> bool {
        if self.partyName.tag.tag_class != TagClass::UNIVERSAL
            || other.partyName.tag.tag_class != TagClass::UNIVERSAL
            || !matches!(self.partyName.tag.tag_number,
                wildboar_asn1::UNIV_TAG_PRINTABLE_STRING
                | wildboar_asn1::UNIV_TAG_UTF8_STRING
                | wildboar_asn1::UNIV_TAG_T61_STRING
                | wildboar_asn1::UNIV_TAG_BMP_STRING
                | wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING
            )
            || !matches!(other.partyName.tag.tag_number,
                wildboar_asn1::UNIV_TAG_PRINTABLE_STRING
                | wildboar_asn1::UNIV_TAG_UTF8_STRING
                | wildboar_asn1::UNIV_TAG_T61_STRING
                | wildboar_asn1::UNIV_TAG_BMP_STRING
                | wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING
            )
        {
            return false;
        }
        if AttributeValue(self.partyName.clone()) != AttributeValue(other.partyName.clone()) {
            return false;
        }
        match (self.nameAssigner.as_ref(), other.nameAssigner.as_ref()) {
            (Some(a), Some(b)) => (
                a.tag.tag_class == TagClass::UNIVERSAL
                && b.tag.tag_class == TagClass::UNIVERSAL
                && matches!(a.tag.tag_number,
                    wildboar_asn1::UNIV_TAG_PRINTABLE_STRING
                    | wildboar_asn1::UNIV_TAG_UTF8_STRING
                    | wildboar_asn1::UNIV_TAG_T61_STRING
                    | wildboar_asn1::UNIV_TAG_BMP_STRING
                    | wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING
                )
                && matches!(b.tag.tag_number,
                    wildboar_asn1::UNIV_TAG_PRINTABLE_STRING
                    | wildboar_asn1::UNIV_TAG_UTF8_STRING
                    | wildboar_asn1::UNIV_TAG_T61_STRING
                    | wildboar_asn1::UNIV_TAG_BMP_STRING
                    | wildboar_asn1::UNIV_TAG_UNIVERSAL_STRING
                )
                && AttributeValue(a.clone()) == AttributeValue(b.clone())
            ),
            _ => true,
        }
    }

}

impl PartialEq for AttCertIssuer {

    fn eq(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return false; // Malformed
        }

        let base_cert_id_match = match (self.baseCertificateID.as_ref(), other.baseCertificateID.as_ref()) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        };
        if !base_cert_id_match {
            return false;
        }

        let issuer_name_match = match (self.issuerName.as_ref(), other.issuerName.as_ref()) {
            (Some(a), Some(b)) => compare_general_names(a, b),
            _ => true,
        };
        if !issuer_name_match {
            return false;
        }

        match (self.objectDigestInfo.as_ref(), other.objectDigestInfo.as_ref()) {
            (Some(a), Some(b)) => {
                if a.is_comparable_with(b) && a.objectDigest != b.objectDigest {
                    return false;
                }
            },
            _ => (),
        };

        true
    }

}

impl PartialEq for IssuerSerial {

    fn eq(&self, other: &Self) -> bool {
        self.serial == other.serial
        && compare_general_names(&self.issuer, &other.issuer)
        && match (self.issuerUID.as_ref(), other.issuerUID.as_ref()) {
            (Some(auid), Some(buid)) => auid == buid,
            _ => true,
        }
    }

}

impl PartialEq for ObjectDigestInfo {

    fn eq(&self, other: &Self) -> bool {
        if self.digestedObjectType != other.digestedObjectType {
            return false;
        }
        if self.otherObjectTypeID.is_some()
            && other.otherObjectTypeID.is_some()
            && self.otherObjectTypeID != other.otherObjectTypeID {
            return false;
        }
        self.digestAlgorithm == other.digestAlgorithm
        && self.objectDigest == other.objectDigest
    }

}

const FEW_GENERAL_NAMES: usize = 10;
fn compare_general_names(a: &GeneralNames, b: &GeneralNames) -> bool {
    let len = a.len() + b.len();
    let few = len < FEW_GENERAL_NAMES;
    if few || a.len() == 1 || b.len() == 1 {
        // If there are few GNs, we just do a naive O(n^2) approach.
        return a
            .iter()
            .any(|agn| b
                .iter()
                .any(|bgn| agn == bgn));
    }
    let mut agens: HashSet<Vec<u8>> = HashSet::from_iter(a
        .iter()
        .filter_map(|agn| {
            let el = _encode_GeneralName(agn).ok()?;
            let mut buf = Vec::new();
            let bytes_written = x690_write_tlv(&mut buf, &el).ok()?;
            Some(buf[0..bytes_written].to_vec())
        }));
    b
        .iter()
        .filter_map(|agn| {
            let el = _encode_GeneralName(agn).ok()?;
            let mut buf = Vec::new();
            let bytes_written = x690_write_tlv(&mut buf, &el).ok()?;
            Some(buf[0..bytes_written].to_vec())
        })
        .any(|b: Vec<u8>| agens.contains(&b))
}

impl Eq for AttributeValue {}
impl Eq for AttributeTypeAndValue {}
impl Eq for Name {}
impl Eq for GeneralName {}
impl Eq for Certificate {}
impl Eq for AttributeCertificate {}
impl Eq for TBSCertAVL {}
impl Eq for TBSCertificate {}
impl Eq for TBSAttributeCertificate {}
impl Eq for IssuerSerialNumber {}
impl Eq for PKCertIdentifier {}
impl Eq for AlgorithmIdentifier {}
impl Eq for AlgorithmWithInvoke {}
impl Eq for FingerPrint {}
impl Eq for HASH {}
impl Eq for EDIPartyName {}
impl Eq for AttCertIssuer {}
impl Eq for IssuerSerial {}
impl Eq for ObjectDigestInfo {}
