#![allow(non_snake_case)]
use std::str::FromStr;

use nom::IResult;
use nom::branch::alt;
use nom::number::complete::double;
use nom::character::complete::{
    char as take_char, space0, space1,
    u32 as take_u32, i64 as take_i64,
};
use nom::bytes::complete::{tag, take_while, take_until};
use nom::character::complete::{hex_digit0, digit1};
use nom::multi::{separated_list0, separated_list1, many1};
use nom::Err as NomErr;
use nom::error::Error as NomError;
use nom::error::ErrorKind as NomErrorKind;
use asn1::{
    BIT_STRING,
    OCTET_STRING, BOOLEAN,
    GeneralizedTime,
    UTCTime,
    OBJECT_IDENTIFIER,
    RELATIVE_OID,
    REAL,
};
use nom::sequence::{delimited, tuple};
use std::borrow::Cow;

// #region General ASN.1 Identifier Parsers

fn is_ldh (c: char) -> bool {
    char::is_ascii_alphanumeric(&c) || c == '-'
}

fn is_bin_digit (c: char) -> bool {
    c == '1' || c == '0'
}

/// NOTE: ITU X.680 seems unclear as to whether non ASCII characters can appear
/// in identifiers. The character repetoire is only ASCII, but it seems to note
/// that "derivative standards" (of ASN.1) may include more characters.
/// However, IETF RFC 3641 says (in the BNF) that only ASCII is to be used.
pub fn parse_identifier (s: &str) -> IResult<&str, &str> {
    let (s, ret) = take_while(is_ldh)(s)?;
    if let Some(first_char) = ret.chars().nth(0) {
        if !first_char.is_ascii_lowercase() {
            return Err(NomErr::Error(NomError::new(s, NomErrorKind::Alpha)));
        }
    } else {
        return Err(NomErr::Error(NomError::new(s, NomErrorKind::Alpha)));
    }
    if ret.find("--").is_some() {
        return Err(NomErr::Error(NomError::new(s, NomErrorKind::AlphaNumeric)));
    }
    if let Some(last_char) = ret.chars().last() {
        if last_char == '-' {
            return Err(NomErr::Error(NomError::new(s, NomErrorKind::AlphaNumeric)));
        }
    }
    Ok((s, ret))
}

pub fn parse_bstring (s: &str) -> IResult<&str, &str> {
    let (s, _) = take_char('\'')(s)?;
    let (s, ret) = take_while(is_bin_digit)(s)?;
    let (s, _) = take_char('\'')(s)?;
    let (s, _) = take_char('B')(s)?;
    Ok((s, ret))
}

pub fn parse_hstring (s: &str) -> IResult<&str, &str> {
    let (s, _) = take_char('\'')(s)?;
    let (s, ret) = hex_digit0(s)?; // Technically, these should only be upper-cased.
    let (s, _) = take_char('\'')(s)?;
    let (s, _) = take_char('H')(s)?;
    Ok((s, ret))
}

pub fn parse_comma_space (s: &str) -> IResult<&str, &str> {
    let (s, _) = space0(s)?;
    let (s, _) = take_char(',')(s)?;
    space0(s)
}

pub fn parse_bit_list (s: &str) -> IResult<&str, Vec<&str>> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, ids) = separated_list0(parse_comma_space, parse_identifier)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, ids))
}

// #endregion General ASN.1 Identifier Parsers

fn parse_keystring (s: &str) -> IResult<&str, &str> {
    let (s, keystr) = take_while(&is_ldh)(s)?;
    if let Some(first_char) = keystr.chars().nth(0) {
        if !first_char.is_ascii_alphabetic() {
            return Err(NomErr::Error(NomError::new(s, NomErrorKind::Alpha)));
        }
    } else {
        return Err(NomErr::Error(NomError::new(s, NomErrorKind::Alpha)));
    }
    Ok((s, keystr))
}

#[inline]
fn parse_descr (s: &str) -> IResult<&str, &str> {
    parse_keystring(s)
}

#[inline]
fn parse_oid_component (s: &str) -> IResult<&str, u32> {
    take_u32(s)
}

#[derive(Debug)]
pub enum GserOidValue <'a> {
    Descriptor (&'a str),
    Literal (OBJECT_IDENTIFIER),
}

#[derive(Debug)]
pub enum GserBitStringValue <'a> {
    BitList (Vec<&'a str>),
    BitString (BIT_STRING),
}

#[derive(Debug)]
pub enum GserIntegerValue <'a> {
    Identifier (&'a str),

    ReasonableLiteral (i64),

    /// Represented as a string to outsource the parsing (and hence, size
    /// constraints) to the calling library.
    BigLiteral (&'a str),
}

#[derive(Debug)]
pub enum GserRealValue <'a> {
    Zero,
    PlusInfinity,
    MinusInfinity,
    FiniteValue(REAL),
    Destructured(SetOrSequenceValue<'a>),
}

#[derive(Debug)]
pub enum ChoiceValue <'a> {
    IdentifiedChoiceValue(GserNamedValue<'a>),
    ChoiceOfStringsValue(Cow<'a, str>),
}

#[derive(Debug)]
pub struct GserNamedValue <'a> {
    pub identifier: &'a str,
    pub value: GserValue<'a>,
}

pub type SetOrSequenceOfValue <'a> = Vec<GserValue<'a>>;
pub type SetOrSequenceValue <'a> = Vec<GserNamedValue<'a>>;

#[derive(Debug)]
pub enum GserValue <'a> {
    BooleanValue(BOOLEAN),
    IntegerValue(GserIntegerValue<'a>),
    BitStringValue(GserBitStringValue<'a>),
    NullValue,
    GeneralizedTimeValue(GeneralizedTime),
    UTCTimeValue(UTCTime),
    EnumeratedValue(&'a str),
    ObjectIdentifierValue(OBJECT_IDENTIFIER),
    RelativeOIDValue(RELATIVE_OID),
    // ObjectDescriptorValue(&'a str), // I think this is a duplicate of
    OctetStringValue(OCTET_STRING),
    StringValue(Cow<'a, str>),
    SetOfValue(SetOrSequenceOfValue<'a>),
    SequenceOfValue(SetOrSequenceOfValue<'a>),
    SetValue(SetOrSequenceValue<'a>),
    SequenceValue(SetOrSequenceValue<'a>),
    CharacterStringValue(SetOrSequenceValue<'a>),
    ChoiceValue(Box<ChoiceValue<'a>>),
    EmbeddedPDVValue(SetOrSequenceValue<'a>),
    ExternalValue(SetOrSequenceValue<'a>),
    InstanceOfValue(SetOrSequenceValue<'a>),
    RealValue(GserRealValue<'a>),
    RDNSequenceValue(String),
    RelativeDistinguishedNameValue(String),
    ORAddressValue(String),
}

pub fn parse_GserNamedValue (s: &str) -> IResult<&str, GserNamedValue> {
    let (s, id) = parse_identifier(s)?;
    let (s, _) = space1(s)?;
    let (s, val) = parse_GserValue(s)?;
    Ok((s, GserNamedValue{ identifier: id, value: val }))
}

pub fn parse_BitStringValue <'a> (s: &'a str) -> IResult<&'a str, GserBitStringValue<'a>> {
    if let Ok((s, bit_list)) = parse_bit_list(s) {
        return Ok((s, GserBitStringValue::BitList(bit_list)));
    }
    if let Ok((s, bin)) = parse_bstring(s) {
        let bs = BIT_STRING::from_bin(bin);
        return Ok((s, GserBitStringValue::BitString(bs)));
    }
    let (s, hexstr) = parse_hstring(s)?;
    let hbytes = hex::decode(hexstr).unwrap();
    let bs = BIT_STRING::from_bytes(hbytes);
    Ok((s, GserBitStringValue::BitString(bs)))
}

pub fn parse_BooleanValue (s: &str) -> IResult<&str, BOOLEAN> {
    let (s, bool_str) = alt((tag("TRUE"), tag("FALSE")))(s)?;
    Ok((s, bool_str == "TRUE"))
}

pub fn parse_CharacterStringValue (s: &str) -> IResult<&str, SetOrSequenceValue> {
    parse_SequenceValue(s)
}

pub fn parse_ChoiceValue (s: &str) -> IResult<&str, ChoiceValue> {
    if let Ok((s, strvalue)) = parse_StringValue(s) {
        return Ok((s, ChoiceValue::ChoiceOfStringsValue(strvalue)));
    }
    let (s, id) = parse_identifier(s)?;
    let (s, _) = take_char(':')(s)?;
    let (s, val) = parse_GserValue(s)?;
    Ok((s, ChoiceValue::IdentifiedChoiceValue(GserNamedValue{ identifier: id, value: val })))
}

pub fn parse_EmbeddedPDVValue (s: &str) -> IResult<&str, SetOrSequenceValue> {
    parse_SequenceValue(s)
}

pub fn parse_EnumeratedValue (s: &str) -> IResult<&str, &str> {
    parse_identifier(s)
}

pub fn parse_ExternalValue (s: &str) -> IResult<&str, SetOrSequenceValue> {
    parse_SequenceValue(s)
}

pub fn parse_GeneralizedTimeValue (s: &str) -> IResult<&str, GeneralizedTime> {
    let (s, gtstr) = delimited(take_char('"'), take_until("\""), take_char('"'))(s)?;
    match GeneralizedTime::from_str(gtstr) {
        Ok(gt) => Ok((s, gt)),
        Err(_) => Err(NomErr::Error(NomError::new(s, NomErrorKind::Verify))),
    }
}

pub fn parse_IntegerValue <'a> (s: &'a str) -> IResult<&'a str, GserIntegerValue<'a>> {
    if let Ok((s, id)) = parse_identifier(s) {
        return Ok((s, GserIntegerValue::Identifier(id)));
    }
    if let Ok((s, int)) = take_i64::<&str, ()>(s) {
        return Ok((s, GserIntegerValue::ReasonableLiteral(int)));
    }
    let (s, digits) = digit1(s)?;
    Ok((s, GserIntegerValue::BigLiteral(digits)))
}

pub fn parse_InstanceOfValue (s: &str) -> IResult<&str, SetOrSequenceValue> {
    parse_SequenceValue(s)
}

pub fn parse_NullValue (s: &str) -> IResult<&str, &str> {
    tag("NULL")(s)
}

pub fn parse_ObjectDescriptorValue (s: &str) -> IResult<&str, Cow<str>> {
    parse_StringValue(s)
}

pub fn parse_ObjectIdentifierValue (s: &str) -> IResult<&str, GserOidValue> {
    if let Ok((s, descr)) = parse_descr(s) {
        return Ok((s, GserOidValue::Descriptor(descr)));
    }
    let (s, arcs) = separated_list1(tag("."), parse_oid_component)(s)?;
    let oid = OBJECT_IDENTIFIER::try_from(arcs)
        .map_err(|_| NomErr::Failure(NomError::new(s, NomErrorKind::Fail)))?;
    Ok((s, GserOidValue::Literal(oid)))
}

pub fn parse_OctetStringValue (s: &str) -> IResult<&str, OCTET_STRING> {
    let (s, hexstr) = parse_hstring(s)?;
    let bytes = hex::decode(hexstr).unwrap();
    Ok((s, bytes))
}

pub fn parse_RealValue (s: &str) -> IResult<&str, GserRealValue> {
    if let Ok((s, _)) = tag::<&str, &str, ()>("PLUS-INFINITY")(s) {
        return Ok((s, GserRealValue::PlusInfinity));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("MINUS-INFINITY")(s) {
        return Ok((s, GserRealValue::MinusInfinity));
    }
    if let Ok((s, _)) = take_char::<&str, ()>('0')(s) {
        return Ok((s, GserRealValue::Zero));
    }
    if let Ok((s, seq)) = parse_SequenceValue(s) {
        return Ok((s, GserRealValue::Destructured(seq)));
    }
    let (s, d) = double(s)?;
    Ok((s, GserRealValue::FiniteValue(d)))
}

pub fn parse_RelativeOIDValue (s: &str) -> IResult<&str, RELATIVE_OID> {
    let (s, arcs) = separated_list1(tag("."), parse_oid_component)(s)?;
    let roid = RELATIVE_OID::try_from(arcs)
        .map_err(|_| NomErr::Failure(NomError::new(s, NomErrorKind::Fail)))?;
    Ok((s, roid))
}

pub fn parse_SequenceOfValue (s: &str) -> IResult<&str, SetOrSequenceOfValue> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, values) = separated_list0(tuple((take_char(','), space0)), parse_GserValue)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, values))
}

pub fn parse_SequenceValue (s: &str) -> IResult<&str, SetOrSequenceValue> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, values) = separated_list0(tuple((take_char(','), space0)), parse_GserNamedValue)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, values))
}

pub fn parse_SetOfValue (s: &str) -> IResult<&str, SetOrSequenceOfValue> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, values) = separated_list0(tuple((take_char(','), space0)), parse_GserValue)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, values))
}

pub fn parse_SetValue (s: &str) -> IResult<&str, SetOrSequenceValue> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, values) = separated_list0(tuple((take_char(','), space0)), parse_GserNamedValue)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, values))
}

pub fn parse_StringValue (s: &str) -> IResult<&str, Cow<str>> {
    let (s, substrs) = many1(delimited(take_char('"'), take_until("\""), take_char('"')))(s)?;
    if substrs.len() == 1 {
        return Ok((s, Cow::Borrowed(substrs[0])));
    }
    Ok((s, Cow::Owned(substrs.join("\""))))
}

pub fn parse_UTCTimeValue (s: &str) -> IResult<&str, UTCTime> {
    let (s, utcstr) = delimited(take_char('"'), take_until("\""), take_char('"'))(s)?;
    match UTCTime::from_str(utcstr) {
        Ok(ut) => Ok((s, ut)),
        Err(_) => Err(NomErr::Error(NomError::new(s, NomErrorKind::Verify))),
    }
}

pub fn parse_RDNSequenceValue (s: &str) -> IResult<&str, Cow<str>> {
    parse_StringValue(s)
}

pub fn parse_RelativeDistinguishedNameValue (s: &str) -> IResult<&str, Cow<str>> {
    parse_StringValue(s)
}

pub fn parse_ORAddressValue (s: &str) -> IResult<&str, Cow<str>> {
    parse_StringValue(s)
}

pub fn parse_GserValue (s: &str) -> IResult<&str, GserValue> {
    if let Ok((s, _)) = parse_NullValue(s) {
        return Ok((s, GserValue::NullValue));
    }
    if let Ok((s, v)) = parse_BooleanValue(s) {
        return Ok((s, GserValue::BooleanValue(v)));
    }
    if let Ok((s, v)) = separated_list1(tag("."), parse_oid_component)(s) {
        if v.len() > 2 {
            let oid = OBJECT_IDENTIFIER::try_from(v)
                .map_err(|_| NomErr::Error(NomError::new(s, NomErrorKind::Fail)))?;
            return Ok((s, GserValue::ObjectIdentifierValue(oid)));
        }
    }
    if let Ok((new_s, v)) = double::<&str, ()>(s) {
        let part_we_just_parsed = &s[0..s.len() - new_s.len()];
        /* We can only be confident that it was actually a real number if the
        parsed value is non-integral (the first condition), or if the part we
        just parsed contains a non-digit character. Integers may only contain
        digits. */
        if v.fract() != 0.0 || part_we_just_parsed.chars().any(|c| !c.is_ascii_digit()) {
            let finite_value = GserRealValue::FiniteValue(v);
            return Ok((new_s, GserValue::RealValue(finite_value)));
        }
    }
    if let Ok((s, v)) = take_i64::<&str, ()>(s) {
        let sizedint = GserIntegerValue::ReasonableLiteral(v);
        return Ok((s, GserValue::IntegerValue(sizedint)));
    }
    if let Ok((s, v)) = digit1::<&str, ()>(s) {
        let bigint = GserIntegerValue::BigLiteral(v);
        return Ok((s, GserValue::IntegerValue(bigint)));
    }
    // Removed because `identifier` is a valid alternative.
    // if let Ok((s, v)) = parse_IntegerValue(s) {
    //     return Ok((s, GserValue::IntegerValue(v)));
    // }
    if let Ok((s, v)) = parse_hstring(s) {
        let bytes = hex::decode(v).unwrap();
        return Ok((s, GserValue::OctetStringValue(bytes)));
    }
    if let Ok((s, v)) = parse_bstring(s) {
        let bs = BIT_STRING::from_bin(v);
        let bsv = GserBitStringValue::BitString(bs);
        return Ok((s, GserValue::BitStringValue(bsv)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("PLUS-INFINITY")(s) {
        return Ok((s, GserValue::RealValue(GserRealValue::PlusInfinity)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("MINUS-INFINITY")(s) {
        return Ok((s, GserValue::RealValue(GserRealValue::MinusInfinity)));
    }
    if let Ok((s, v)) = parse_GeneralizedTimeValue(s) {
        return Ok((s, GserValue::GeneralizedTimeValue(v)));
    }
    if let Ok((s, v)) = parse_UTCTimeValue(s) {
        return Ok((s, GserValue::UTCTimeValue(v)));
    }
    if let Ok((s, v)) = parse_StringValue(s) {
        return Ok((s, GserValue::StringValue(v)));
    }
    if let Ok((s, v)) = parse_SequenceValue(s) {
        return Ok((s, GserValue::SequenceValue(v)));
    }
    if let Ok((s, v)) = parse_SequenceOfValue(s) {
        return Ok((s, GserValue::SequenceOfValue(v)));
    }
    if let Ok((s, v)) = parse_EnumeratedValue(s) {
        return Ok((s, GserValue::EnumeratedValue(v)));
    }
    Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
}


#[cfg(test)]
mod tests {

    use asn1::OBJECT_IDENTIFIER;
    use crate::{parse_GserValue, GserValue, GserIntegerValue, GserRealValue};

    #[test]
    fn test_seq_parsing() {
        let input = "{ algorithm 2.5.43.19, parameters 65537 }"; // Not a real algorithm.
        let (s, v) = parse_GserValue(input).unwrap();
        assert_eq!(s.len(), 0);
        let seq = match v {
            GserValue::SequenceValue(s) => s,
            _ => panic!("Not a sequence value"),
        };
        assert_eq!(seq.len(), 2);
        let c1 = &seq[0];
        let c2 = &seq[1];
        assert_eq!(c1.identifier, "algorithm");
        assert_eq!(c2.identifier, "parameters");
        let alg = match &c1.value {
            GserValue::ObjectIdentifierValue(oid) => oid,
            _ => panic!("Not an OID value"),
        };
        assert!(alg == &OBJECT_IDENTIFIER::try_from(Vec::from([ 2u32, 5, 43, 19 ])).unwrap());
        dbg!(&c2.value);
        let params = match &c2.value {
            GserValue::IntegerValue(i) => i,
            _ => panic!("Not an integer value"),
        };
        let params_int = match params {
            GserIntegerValue::ReasonableLiteral(i) => i,
            _ => panic!("Not a reasonable integer value"),
        };
        assert_eq!(params_int, &65537i64);
    }

    #[test]
    fn test_string_parsing() {
        let input = "{ \"foobar\", \"I said \"\"FooBar\"\"\" }";
        let (s, v) = parse_GserValue(input).unwrap();
        assert_eq!(s.len(), 0);
        let seq = match v {
            GserValue::SequenceOfValue(s) => s,
            _ => panic!("Not a sequence value"),
        };
        assert_eq!(seq.len(), 2);
        let c1 = &seq[0];
        let c2 = &seq[1];
        let s1 = match &c1 {
            GserValue::StringValue(sv) => sv,
            _ => panic!("Not a string value"),
        };
        let s2 = match &c2 {
            GserValue::StringValue(sv) => sv,
            _ => panic!("Not a string value"),
        };
        assert_eq!(s1, "foobar");
        assert_eq!(s2, "I said \"FooBar\"");
    }

    #[test]
    fn test_everything_else_parsing() {
        let input = "{ justKen TRUE, barbie NULL, gt \"19960415203000Z\", utc \"9604152030Z\", mojo dojo, cope -3.14 }";
        let (s, v) = parse_GserValue(input).unwrap();
        assert_eq!(s.len(), 0);
        let seq = match v {
            GserValue::SequenceValue(s) => s,
            _ => panic!("Not a sequence value"),
        };
        assert_eq!(seq.len(), 6);
        let c1 = &seq[0];
        let c2 = &seq[1];
        let c3 = &seq[2];
        let c4 = &seq[3];
        let c5 = &seq[4];
        let c6 = &seq[5];
        assert_eq!(c1.identifier, "justKen");
        assert_eq!(c2.identifier, "barbie");
        assert_eq!(c3.identifier, "gt");
        assert_eq!(c4.identifier, "utc");
        assert_eq!(c5.identifier, "mojo");
        assert_eq!(c6.identifier, "cope");
        let justKen = match &c1.value {
            GserValue::BooleanValue(b) => b,
            _ => panic!("Not a boolean value"),
        };
        assert!(justKen);
        match &c2.value {
            GserValue::NullValue => (),
            _ => panic!("Not a null value"),
        };
        dbg!(&c3.value);
        match &c3.value {
            GserValue::GeneralizedTimeValue(_) => (),
            _ => panic!("Not a GeneralizedTime value"),
        };
        match &c4.value {
            GserValue::UTCTimeValue(_) => (),
            _ => panic!("Not a UTCTime value"),
        };
        match &c5.value {
            GserValue::EnumeratedValue(e) => assert_eq!(e, &"dojo"),
            _ => panic!("Not an enumerated value"),
        };
        let r = match &c6.value {
            GserValue::RealValue(re) => re,
            _ => panic!("Not a real value"),
        };
        let d = match r {
            GserRealValue::FiniteValue(i) => i,
            _ => panic!("Not a finite real value"),
        };
        assert!(*d < -3.0);
        assert!(*d > -4.0);
    }

}
