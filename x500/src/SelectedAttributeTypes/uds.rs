#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
use asn1::*;
use x690::*;
use std::str::FromStr;

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnboundedDirectoryString  ::=  CHOICE {
///   teletexString    TeletexString(SIZE (1..MAX)),
///   printableString  PrintableString(SIZE (1..MAX)),
///   bmpString        BMPString(SIZE (1..MAX)),
///   universalString  UniversalString(SIZE (1..MAX)),
///   uTF8String       UTF8String(SIZE (1..MAX)) }
/// ```
#[derive(Debug, Clone)]
pub enum UnboundedDirectoryString {
    teletexString(TeletexString),
    printableString(PrintableString),
    bmpString(BMPString),
    universalString(UniversalString),
    uTF8String(UTF8String),
}

impl TryFrom<&X690Element> for UnboundedDirectoryString {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UnboundedDirectoryString(el)
    }
}

pub fn _decode_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<UnboundedDirectoryString> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => Ok(UnboundedDirectoryString::teletexString(
            BER.decode_t61_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 19) => Ok(UnboundedDirectoryString::printableString(
            BER.decode_printable_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 30) => Ok(UnboundedDirectoryString::bmpString(
            BER.decode_bmp_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 28) => Ok(UnboundedDirectoryString::universalString(
            BER.decode_universal_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 12) => Ok(UnboundedDirectoryString::uTF8String(
            BER.decode_utf8_string(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UnboundedDirectoryString",
            ))
        }
    }
}

pub fn _encode_UnboundedDirectoryString(
    value_: &UnboundedDirectoryString,
) -> ASN1Result<X690Element> {
    match value_ {
        UnboundedDirectoryString::teletexString(v) => BER.encode_t61_string(&v),
        UnboundedDirectoryString::printableString(v) => BER.encode_printable_string(&v),
        UnboundedDirectoryString::bmpString(v) => BER.encode_bmp_string(&v),
        UnboundedDirectoryString::universalString(v) => BER.encode_universal_string(&v),
        UnboundedDirectoryString::uTF8String(v) => BER.encode_utf8_string(&v),
    }
}

/// This was added to increase the efficiency of encoding UDS. Instead of
/// _copying_ the bytes of the string into the encoding, this function takes the
/// underlying bytes of the string to be used as the encoding output.
pub fn _encode_owned_UnboundedDirectoryString(
    value_: UnboundedDirectoryString,
) -> ASN1Result<X690Element> {
    match value_ {
        UnboundedDirectoryString::teletexString(v) => BER.encode_owned_t61_string(v),
        UnboundedDirectoryString::printableString(v) => BER.encode_owned_printable_string(v),
        UnboundedDirectoryString::bmpString(ref v) => BER.encode_bmp_string(&v),
        UnboundedDirectoryString::universalString(ref v) => BER.encode_universal_string(&v),
        UnboundedDirectoryString::uTF8String(v) => BER.encode_owned_utf8_string(v),
    }
}

pub fn _validate_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => BER.validate_t61_string(&el),
        (TagClass::UNIVERSAL, 19) => BER.validate_printable_string(&el),
        (TagClass::UNIVERSAL, 30) => BER.validate_bmp_string(&el),
        (TagClass::UNIVERSAL, 28) => BER.validate_universal_string(&el),
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UnboundedDirectoryString",
            ))
        }
    }
}

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

impl From<String> for UnboundedDirectoryString {

    /// For efficiency and compatibility, this function stores the string as
    /// a PrintableString if it fits within the allowed characters, then falls
    /// back on TeletexString if the string is all ASCII characters (since
    /// ASCII overlaps with TeletexString), and UTF8String otherwise.
    fn from(value: String) -> Self {
        let is_printable = is_printable_str(value.as_str());
        if is_printable {
            return UnboundedDirectoryString::printableString(value);
        }
        if value.is_ascii() {
            return UnboundedDirectoryString::teletexString(value.into_bytes());
        }
        UnboundedDirectoryString::uTF8String(value)
    }

}

// TODO: Add these functions for DirectoryString too.
