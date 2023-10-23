#![allow(non_snake_case)]
use std::fmt::Display;
use std::str::FromStr;

use nom::IResult;
use nom::branch::alt;
use nom::number::complete::double;
use nom::combinator::opt;
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
    INTEGER,
};
use nom::sequence::{delimited, tuple, preceded};
use std::borrow::Cow;
use gser::{GserOidValue, GserValue, parse_ObjectIdentifierValue, parse_GserValue, parse_StringValue, parse_OctetStringValue, parse_IntegerValue, GserIntegerValue, parse_UTCTimeValue, parse_GeneralizedTimeValue, parse_RelativeDistinguishedNameValue, parse_BitStringValue, parse_identifier, GserBitStringValue};

// RFC 4523 only defines `rdnSequence` as a valid alternative, but this is used
// so that we can tolerate the newly introduced name alternatives.
#[derive(Debug)]
pub enum LdapName <'a> {
    RdnSequence(Cow<'a, str>),
    Oid(GserOidValue<'a>),
    DnsName(&'a str),
}

#[derive(Debug)]
pub struct LdapCertificateExactAssertion <'a> {
    pub serialNumber: GserIntegerValue<'a>,
    pub issuer: LdapName<'a>,
}

#[derive(Debug)]
pub enum LdapAltNameType <'a> {
    OtherName(GserOidValue<'a>),
    Rfc822Name,
    DnsName,
    X400Address,
    DirectoryName,
    EdiPartyName,
    URI,
    IPAddress,
    RegisteredId,
}

#[derive(Debug)]
pub struct LdapCertificateAssertion <'a> {
    pub serialNumber: Option<GserIntegerValue<'a>>,
    pub issuer: Option<LdapName<'a>>,
    pub subjectKeyIdentifier: Option<OCTET_STRING>,
    pub authorityKeyIdentifier: Option<LdapAuthorityKeyIdentifier<'a>>,
    pub certificateValid: Option<GeneralizedTime>, // Could be UTCTime also, but just convert it.
    pub privateKeyValid: Option<GeneralizedTime>,
    pub subjectPublicKeyAlgID: Option<GserOidValue<'a>>,
    pub keyUsage: Option<GserBitStringValue<'a>>,
    pub subjectAltName: Option<LdapAltNameType<'a>>,
    pub policy: Option<Vec<GserOidValue<'a>>>,
    pub pathToName: Option<LdapName<'a>>,
    pub subject: Option<LdapName<'a>>,
    pub nameConstraints: Option<LdapNameConstraintsSyntax<'a>>,
}

impl <'a> Default for LdapCertificateAssertion <'a> {

    fn default() -> Self {
        LdapCertificateAssertion {
            serialNumber: None,
            issuer: None,
            subjectKeyIdentifier: None,
            authorityKeyIdentifier: None,
            certificateValid: None,
            privateKeyValid: None,
            subjectPublicKeyAlgID: None,
            keyUsage: None,
            subjectAltName: None,
            policy: None,
            pathToName: None,
            subject: None,
            nameConstraints: None,
        }
    }

}

#[derive(Debug)]
pub struct LdapOtherName <'a> {
    pub type_id: GserOidValue<'a>,
    pub value: GserValue<'a>,
}

#[derive(Debug)]
pub struct LdapEdiPartyName <'a> {
    pub nameAssigner: Option<Cow<'a, str>>,
    pub partyName: Cow<'a, str>,
}

#[derive(Debug)]
pub enum LdapGeneralName <'a> {
    OtherName(LdapOtherName<'a>),
    Rfc822Name(Cow<'a, str>), // This might have quotes in it.
    DnsName(&'a str), // This WILL NOT have quotes in it.
    X400Address(Cow<'a, str>), // This might have quotes in it.
    DirectoryName(LdapName<'a>),
    EdiPartyName(LdapEdiPartyName<'a>),
    URI(Cow<'a, str>),
    IPAddress(OCTET_STRING),
    RegisteredId(GserOidValue<'a>),
}

#[derive(Debug)]
pub struct LdapAuthorityKeyIdentifier <'a> {
    pub keyIdentifier: Option<OCTET_STRING>,
    pub authorityCertIssuer: Option<Vec<LdapGeneralName<'a>>>,
    pub authorityCertSerialNumber: Option<GserIntegerValue<'a>>,
}

impl <'a> Default for LdapAuthorityKeyIdentifier<'a> {

    fn default() -> Self {
        LdapAuthorityKeyIdentifier{
            keyIdentifier: None,
            authorityCertIssuer: None,
            authorityCertSerialNumber: None,
        }
    }

}

#[derive(Debug)]
pub struct LdapGeneralSubtree <'a> {
    pub base: LdapGeneralName<'a>,
    pub minimum: Option<u32>,
    pub maximum: Option<u32>,
}

#[derive(Debug)]
pub struct LdapNameConstraintsSyntax <'a> {
    pub permittedSubtrees: Option<Vec<LdapGeneralSubtree<'a>>>,
    pub excludedSubtrees: Option<Vec<LdapGeneralSubtree<'a>>>,
}

impl <'a> Default for LdapNameConstraintsSyntax<'a> {

    fn default() -> Self {
        LdapNameConstraintsSyntax{
            permittedSubtrees: None,
            excludedSubtrees: None,
        }
    }

}

#[derive(Debug)]
pub struct LdapCertificatePairExactAssertion <'a> {
    pub issuedToThisCAAssertion: Option<LdapCertificateExactAssertion<'a>>,
    pub issuedByThisCAAssertion: Option<LdapCertificateExactAssertion<'a>>,
}

impl <'a> Default for LdapCertificatePairExactAssertion<'a> {

    fn default() -> Self {
        LdapCertificatePairExactAssertion{
            issuedToThisCAAssertion: None,
            issuedByThisCAAssertion: None,
        }
    }

}

#[derive(Debug)]
pub struct CertificatePairAssertion <'a> {
    pub issuedToThisCAAssertion: Option<LdapCertificateAssertion<'a>>,
    pub issuedByThisCAAssertion: Option<LdapCertificateAssertion<'a>>,
}

impl <'a> Default for CertificatePairAssertion<'a> {

    fn default() -> Self {
        CertificatePairAssertion{
            issuedToThisCAAssertion: None,
            issuedByThisCAAssertion: None,
        }
    }

}

#[derive(Debug)]
pub struct CertificateListExactAssertion <'a> {
    pub issuer: LdapName<'a>,
    pub thisUpdate: GeneralizedTime,
    pub distributionPoint: Option<DistributionPointName<'a>>,
}

#[derive(Debug)]
pub enum DistributionPointName <'a> {
    FullName(Vec<LdapGeneralName<'a>>),
    RelativeName(Cow<'a, str>),
}

#[derive(Debug)]
pub struct CertificateListAssertion <'a> {
    pub issuer: Option<LdapName<'a>>,
    pub minCRLNumber: Option<GserIntegerValue<'a>>,
    pub maxCRLNumber: Option<GserIntegerValue<'a>>,
    pub reasonFlags: Option<GserBitStringValue<'a>>,
    pub dateAndTime: Option<GeneralizedTime>,
    pub distributionPoint: Option<DistributionPointName<'a>>,
    pub authorityKeyIdentifier: Option<LdapAuthorityKeyIdentifier<'a>>,
}

impl <'a> Default for CertificateListAssertion<'a> {

    fn default() -> Self {
        CertificateListAssertion{
            issuer: None,
            minCRLNumber: None,
            maxCRLNumber: None,
            reasonFlags: None,
            dateAndTime: None,
            distributionPoint: None,
            authorityKeyIdentifier: None,
        }
    }

}

#[derive(Debug)]
pub struct LdapAlgorithmIdentifier <'a> {
    pub algorithm: GserOidValue<'a>,
    pub parameters: Option<GserValue<'a>>,
}

pub fn parse_OtherName (s: &str) -> IResult<&str, LdapOtherName> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("type-id")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_ObjectIdentifierValue(s)?;
    let (s, _) = take_char(',')(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("value")(s)?;
    let (s, _) = space1(s)?;
    let (s, val) = parse_GserValue(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, LdapOtherName{ type_id: id, value: val }))
}

pub fn parse_LdapName (s: &str) -> IResult<&str, LdapName> {
    if let Ok((s, _)) = tag::<&str, &str, ()>("rdnSequence:")(s) {
        let (s, v) = parse_StringValue(s)?;
        return Ok((s, LdapName::RdnSequence(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("oid:")(s) {
        let (s, v) = parse_ObjectIdentifierValue(s)?;
        return Ok((s, LdapName::Oid(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("dnsName:")(s) {
        let (s, v) = parse_StringValue(s)?;
        if let Cow::Borrowed(b) = v {
            return Ok((s, LdapName::DnsName(b)));
        } else {
            return Err(NomErr::Error(NomError::new(s, NomErrorKind::Verify)));
        }
    }
    Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
}

pub fn parse_EdiPartyName (s: &str) -> IResult<&str, LdapEdiPartyName> {
    let (s, _) = take_char('{')(s)?;
    let (s, maybe_na) = opt(tuple((
        space0,
        tag("nameAssigner"),
        space1,
        parse_StringValue,
        take_char(','),
    )))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("partyName")(s)?;
    let (s, _) = space1(s)?;
    let (s, party_name) = parse_StringValue(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    if let Some((_, _, _, na, _)) = maybe_na {
        Ok((s, LdapEdiPartyName{ nameAssigner: Some(na), partyName: party_name }))
    } else {
        Ok((s, LdapEdiPartyName{ nameAssigner: None, partyName: party_name }))
    }
}

pub fn parse_GeneralName (s: &str) -> IResult<&str, LdapGeneralName> {
    if let Ok((s, _)) = tag::<&str, &str, ()>("otherName:")(s) {
        let (s, v) = parse_OtherName(s)?;
        return Ok((s, LdapGeneralName::OtherName(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("rfc822Name:")(s) {
        let (s, v) = parse_StringValue(s)?;
        return Ok((s, LdapGeneralName::Rfc822Name(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("dNSName:")(s) {
        let (s, v) = parse_StringValue(s)?;
        if let Cow::Borrowed(b) = v {
            return Ok((s, LdapGeneralName::DnsName(b)));
        } else {
            return Err(NomErr::Error(NomError::new(s, NomErrorKind::Verify)));
        }
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("x400Address:")(s) {
        let (s, v) = parse_StringValue(s)?;
        return Ok((s, LdapGeneralName::X400Address(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("directoryName:")(s) {
        let (s, v) = parse_LdapName(s)?;
        return Ok((s, LdapGeneralName::DirectoryName(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("ediPartyName:")(s) {
        let (s, v) = parse_EdiPartyName(s)?;
        return Ok((s, LdapGeneralName::EdiPartyName(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("uniformResourceIdentifier:")(s) {
        let (s, v) = parse_StringValue(s)?;
        return Ok((s, LdapGeneralName::URI(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("iPAddress:")(s) {
        let (s, v) = parse_OctetStringValue(s)?;
        return Ok((s, LdapGeneralName::IPAddress(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("registeredID:")(s) {
        let (s, v) = parse_ObjectIdentifierValue(s)?;
        return Ok((s, LdapGeneralName::RegisteredId(v)));
    }
    Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
}

pub fn parse_GeneralNames (s: &str) -> IResult<&str, Vec<LdapGeneralName>> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, names) = separated_list1(tuple((take_char(','), space0)), parse_GeneralName)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, names))
}

pub fn parse_GeneralSubtree (s: &str) -> IResult<&str, LdapGeneralSubtree> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("base")(s)?;
    let (s, _) = space1(s)?;
    let (s, base) = parse_GeneralName(s)?;
    let (s, maybe_min) = opt(tuple((
        take_char(','),
        space1,
        tag("minimum"),
        space1,
        take_u32,
    )))(s)?;
    let (s, maybe_max) = opt(tuple((
        take_char(','),
        space1,
        tag("maximum"),
        space1,
        take_u32,
    )))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let minimum = maybe_min.map(|(_, _, _, _, m)| m);
    let maximum = maybe_max.map(|(_, _, _, _, m)| m);
    Ok((s, LdapGeneralSubtree{ base, minimum, maximum }))
}

pub fn parse_GeneralSubtrees (s: &str) -> IResult<&str, Vec<LdapGeneralSubtree>> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, subtrees) = separated_list1(tuple((take_char(','), space0)), parse_GeneralSubtree)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, subtrees))
}

pub fn parse_CertPolicySet (s: &str) -> IResult<&str, Vec<GserOidValue>> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, oids) = separated_list1(tuple((take_char(','), space0)), parse_ObjectIdentifierValue)(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, oids))
}

pub fn parse_NameConstraintsSyntax (s: &str) -> IResult<&str, LdapNameConstraintsSyntax> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, maybe_permitted) = opt(tuple((
        tag("permittedSubtrees"),
        space1,
        parse_GeneralSubtrees,
    )))(s)?;
    let (s, maybe_excluded) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("excludedSubtrees"),
        space1,
        parse_GeneralSubtrees,
    )))(s)?;
    let permittedSubtrees = maybe_permitted.map(|(_, _, s)| s);
    let excludedSubtrees = maybe_excluded.map(|(_, _, _, _, s)| s);
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, LdapNameConstraintsSyntax{ permittedSubtrees, excludedSubtrees }))
}

pub fn parse_AuthorityKeyIdentifier (s: &str) -> IResult<&str, LdapAuthorityKeyIdentifier> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, maybe_kid) = opt(tuple((
        tag("keyIdentifier"),
        space1,
        parse_OctetStringValue,
    )))(s)?;
    let (s, maybe_auth_cert_iss) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("authorityCertIssuer"),
        space1,
        parse_GeneralNames,
    )))(s)?;
    let (s, maybe_auth_cert_serial) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("authorityCertSerialNumber"),
        space1,
        parse_IntegerValue,
    )))(s)?;
    let keyIdentifier = maybe_kid.map(|(_, _, k)| k);
    let authorityCertIssuer = maybe_auth_cert_iss.map(|(_, _, _, _, i)| i);
    let authorityCertSerialNumber = maybe_auth_cert_serial.map(|(_, _, _, _, s)| s);
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, LdapAuthorityKeyIdentifier{ keyIdentifier, authorityCertIssuer, authorityCertSerialNumber }))
}

pub fn parse_Time (s: &str) -> IResult<&str, GeneralizedTime> {
    if let Ok((s, _)) = tag::<&str, &str, ()>("utcTime:")(s) {
        let (s, v) = parse_UTCTimeValue(s)?;
        return Ok((s, v.into()));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("generalizedTime:")(s) {
        let (s, v) = parse_GeneralizedTimeValue(s)?;
        return Ok((s, v));
    }
    Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
}

pub fn parse_DistributionPointName (s: &str) -> IResult<&str, DistributionPointName> {
    if let Ok((s, _)) = tag::<&str, &str, ()>("fullName:")(s) {
        let (s, v) = parse_GeneralNames(s)?;
        return Ok((s, DistributionPointName::FullName(v)));
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("nameRelativeToCRLIssuer:")(s) {
        let (s, v) = parse_RelativeDistinguishedNameValue(s)?;
        return Ok((s, DistributionPointName::RelativeName(v)));
    }
    Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
}

pub fn parse_AlgorithmIdentifier (s: &str) -> IResult<&str, LdapAlgorithmIdentifier> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("algorithm")(s)?;
    let (s, _) = space1(s)?;
    let (s, algorithm) = parse_ObjectIdentifierValue(s)?;
    let (s, maybe_parameters) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("parameters"),
        space1,
        parse_GserValue,
    )))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let parameters = maybe_parameters.map(|(_, _, _, _, v)| v);
    Ok((s, LdapAlgorithmIdentifier{ algorithm, parameters }))
}

pub fn parse_AltNameType (s: &str) -> IResult<&str, LdapAltNameType> {
    if let Ok((s, _)) = tag::<&str, &str, ()>("builtinNameForm:")(s) {
        if let Ok((s, built_in_type)) = parse_identifier(s) {
            return match built_in_type {
                "rfc822Name" => Ok((s, LdapAltNameType::Rfc822Name)),
                "dNSName" => Ok((s, LdapAltNameType::DnsName)),
                "x400Address" => Ok((s, LdapAltNameType::X400Address)),
                "directoryName" => Ok((s, LdapAltNameType::DirectoryName)),
                "ediPartyName" => Ok((s, LdapAltNameType::EdiPartyName)),
                "uniformResourceIdentifier" => Ok((s, LdapAltNameType::URI)),
                "iPAddress" => Ok((s, LdapAltNameType::IPAddress)),
                "registeredId" => Ok((s, LdapAltNameType::RegisteredId)),
                _ => Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
            };
        }
    }
    if let Ok((s, _)) = tag::<&str, &str, ()>("otherNameForm:")(s) {
        let (s, oid) = parse_ObjectIdentifierValue(s)?;
        return Ok((s, LdapAltNameType::OtherName(oid)));
    }
    Err(NomErr::Error(NomError::new(s, NomErrorKind::Alt)))
}

pub fn parse_CertificateExactAssertion (s: &str) -> IResult<&str, LdapCertificateExactAssertion> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("serialNumber")(s)?;
    let (s, _) = space1(s)?;
    let (s, serialNumber) = parse_IntegerValue(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char(',')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("issuer")(s)?;
    let (s, _) = space1(s)?;
    let (s, issuer) = parse_LdapName(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    Ok((s, LdapCertificateExactAssertion{ issuer, serialNumber }))
}

pub fn parse_CertificateAssertion (s: &str) -> IResult<&str, LdapCertificateAssertion> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, maybe_serialNumber) = opt(tuple((
        // opt(take_char(',')),
        // space0,
        tag("serialNumber"),
        space1,
        parse_IntegerValue,
    )))(s)?;
    let (s, maybe_issuer) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("issuer"),
        space1,
        parse_LdapName,
    )))(s)?;
    let (s, maybe_subjectKeyIdentifier) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("subjectKeyIdentifier"),
        space1,
        parse_OctetStringValue,
    )))(s)?;
    let (s, maybe_authorityKeyIdentifier) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("authorityKeyIdentifier"),
        space1,
        parse_AuthorityKeyIdentifier,
    )))(s)?;
    let (s, maybe_certificateValid) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("certificateValid"),
        space1,
        parse_Time,
    )))(s)?;
    let (s, maybe_privateKeyValid) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("privateKeyValid"),
        space1,
        parse_GeneralizedTimeValue,
    )))(s)?;
    let (s, maybe_subjectPublicKeyAlgID) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("subjectPublicKeyAlgID"),
        space1,
        parse_ObjectIdentifierValue,
    )))(s)?;
    let (s, maybe_keyUsage) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("keyUsage"),
        space1,
        parse_BitStringValue,
    )))(s)?;
    let (s, maybe_subjectAltName) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("subjectAltName"),
        space1,
        parse_AltNameType,
    )))(s)?;
    let (s, maybe_policy) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("policy"),
        space1,
        parse_CertPolicySet,
    )))(s)?;
    let (s, maybe_pathToName) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("pathToName"),
        space1,
        parse_LdapName,
    )))(s)?;
    let (s, maybe_subject) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("subject"),
        space1,
        parse_LdapName,
    )))(s)?;
    let (s, maybe_nameConstraints) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("nameConstraints"),
        space1,
        parse_NameConstraintsSyntax,
    )))(s)?;
    let serialNumber = maybe_serialNumber.map(|(_, _, v)| v);
    let issuer = maybe_issuer.map(|(_, _, _, _, v)| v);
    let subjectKeyIdentifier = maybe_subjectKeyIdentifier.map(|(_, _, _, _, v)| v);
    let authorityKeyIdentifier = maybe_authorityKeyIdentifier.map(|(_, _, _, _, v)| v);
    let certificateValid = maybe_certificateValid.map(|(_, _, _, _, v)| v);
    let privateKeyValid = maybe_privateKeyValid.map(|(_, _, _, _, v)| v);
    let subjectPublicKeyAlgID = maybe_subjectPublicKeyAlgID.map(|(_, _, _, _, v)| v);
    let keyUsage = maybe_keyUsage.map(|(_, _, _, _, v)| v);
    let subjectAltName = maybe_subjectAltName.map(|(_, _, _, _, v)| v);
    let policy = maybe_policy.map(|(_, _, _, _, v)| v);
    let pathToName = maybe_pathToName.map(|(_, _, _, _, v)| v);
    let subject = maybe_subject.map(|(_, _, _, _, v)| v);
    let nameConstraints = maybe_nameConstraints.map(|(_, _, _, _, v)| v);
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let ret = LdapCertificateAssertion{
        serialNumber,
        issuer,
        subjectKeyIdentifier,
        authorityKeyIdentifier,
        certificateValid,
        privateKeyValid,
        subjectPublicKeyAlgID,
        keyUsage,
        subjectAltName,
        policy,
        pathToName,
        subject,
        nameConstraints,
    };
    Ok((s, ret))
}

pub fn parse_CertificatePairExactAssertion (s: &str) -> IResult<&str, LdapCertificatePairExactAssertion> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, maybe_iss_to) = opt(tuple((
        tag("issuedToThisCAAssertion"),
        space1,
        parse_CertificateExactAssertion,
    )))(s)?;
    let (s, maybe_iss_by) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("issuedByThisCAAssertion"),
        space1,
        parse_CertificateExactAssertion,
    )))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let iss_to = maybe_iss_to.map(|(_, _, v)| v);
    let iss_by = maybe_iss_by.map(|(_, _, _, _, v)| v);
    let ret = LdapCertificatePairExactAssertion{
        issuedToThisCAAssertion: iss_to,
        issuedByThisCAAssertion: iss_by,
    };
    Ok((s, ret))
}

pub fn parse_CertificatePairAssertion (s: &str) -> IResult<&str, CertificatePairAssertion> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, maybe_iss_to) = opt(tuple((
        tag("issuedToThisCAAssertion"),
        space1,
        parse_CertificateAssertion,
    )))(s)?;
    let (s, maybe_iss_by) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("issuedByThisCAAssertion"),
        space1,
        parse_CertificateAssertion,
    )))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let iss_to = maybe_iss_to.map(|(_, _, v)| v);
    let iss_by = maybe_iss_by.map(|(_, _, _, _, v)| v);
    let ret = CertificatePairAssertion{
        issuedToThisCAAssertion: iss_to,
        issuedByThisCAAssertion: iss_by,
    };
    Ok((s, ret))
}

pub fn parse_CertificateListExactAssertion (s: &str) -> IResult<&str, CertificateListExactAssertion> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("issuer")(s)?;
    let (s, _) = space1(s)?;
    let (s, issuer) = parse_LdapName(s)?;
    let (s, _) = take_char(',')(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("thisUpdate")(s)?;
    let (s, _) = space1(s)?;
    let (s, thisUpdate) = parse_Time(s)?;
    let dp_prefix = tuple((take_char(','), space0, tag("distributionPoint"), space1));
    let (s, dp) = opt(preceded(dp_prefix, parse_DistributionPointName))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let ret = CertificateListExactAssertion { issuer, thisUpdate, distributionPoint: dp };
    Ok((s, ret))
}

pub fn parse_CertificateListAssertion (s: &str) -> IResult<&str, CertificateListAssertion> {
    let (s, _) = take_char('{')(s)?;
    let (s, _) = space0(s)?;
    let (s, maybe_issuer) = opt(tuple((
        // opt(take_char(',')),
        // space0,
        tag("issuer"),
        space1,
        parse_LdapName,
    )))(s)?;
    let (s, maybe_minCRLNumber) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("minCRLNumber"),
        space1,
        parse_IntegerValue,
    )))(s)?;
    let (s, maybe_maxCRLNumber) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("maxCRLNumber"),
        space1,
        parse_IntegerValue,
    )))(s)?;
    let (s, maybe_reasonFlags) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("reasonFlags"),
        space1,
        parse_BitStringValue,
    )))(s)?;
    let (s, maybe_dateAndTime) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("dateAndTime"),
        space1,
        parse_Time,
    )))(s)?;
    let (s, maybe_distributionPoint) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("distributionPoint"),
        space1,
        parse_DistributionPointName,
    )))(s)?;
    let (s, maybe_authorityKeyIdentifier) = opt(tuple((
        opt(take_char(',')),
        space0,
        tag("authorityKeyIdentifier"),
        space1,
        parse_AuthorityKeyIdentifier,
    )))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = take_char('}')(s)?;
    let issuer = maybe_issuer.map(|(_, _, x)| x);
    let minCRLNumber = maybe_minCRLNumber.map(|(_, _, _, _, x)| x);
    let maxCRLNumber = maybe_maxCRLNumber.map(|(_, _, _, _, x)| x);
    let reasonFlags = maybe_reasonFlags.map(|(_, _, _, _, x)| x);
    let dateAndTime = maybe_dateAndTime.map(|(_, _, _, _, x)| x);
    let distributionPoint = maybe_distributionPoint.map(|(_, _, _, _, x)| x);
    let authorityKeyIdentifier = maybe_authorityKeyIdentifier.map(|(_, _, _, _, x)| x);
    let ret = CertificateListAssertion{
        issuer,
        minCRLNumber,
        maxCRLNumber,
        reasonFlags,
        dateAndTime,
        distributionPoint,
        authorityKeyIdentifier,
    };
    Ok((s, ret))
}

#[cfg(test)]
mod tests {
    use super::*;
    use asn1::oid;

const TEST_CERT_ASSERTION_01: &str = "{ \
serialNumber 12345, \
issuer rdnSequence:\"c=US,st=FL,cn=Jon W\", \
subjectKeyIdentifier '456BC1F0'H \
}";

    #[test]
    fn parses_cert_assertion_01() {
        let input = TEST_CERT_ASSERTION_01;
        let (s, output) = parse_CertificateAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let serial = match output.serialNumber.unwrap() {
            GserIntegerValue::ReasonableLiteral(u) => u,
            _ => panic!("Not a reasonable integer value for serialNumber."),
        };
        assert_eq!(serial, 12345);
        let issuer = match output.issuer.unwrap() {
            LdapName::RdnSequence(rdns) => rdns,
            _ => panic!("Not an RDN sequence"),
        };
        assert_eq!(issuer.as_ref(), "c=US,st=FL,cn=Jon W");
        let skid = output.subjectKeyIdentifier.unwrap();
        assert_eq!(skid.as_slice(), &[ 0x45, 0x6B, 0xC1, 0xF0 ]);
    }

const TEST_CERT_ASSERTION_02: &str = "{ \
authorityKeyIdentifier { \
keyIdentifier '2034FFD1'H, \
authorityCertIssuer { dNSName:\"wildboar.software\", ediPartyName:{ partyName \"Foobar\" }, registeredID:wildboar }, \
authorityCertSerialNumber 56789 \
} \
}";

    #[test]
    fn parses_cert_assertion_02() {
        let input = TEST_CERT_ASSERTION_02;
        let (s, output) = parse_CertificateAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let akid =  output.authorityKeyIdentifier.unwrap();
        let kid = akid.keyIdentifier.unwrap();
        let ac_iss = akid.authorityCertIssuer.unwrap();
        let ac_ser = akid.authorityCertSerialNumber.unwrap();
        assert_eq!(kid.as_slice(), &[ 0x20, 0x34, 0xFF, 0xD1 ]);
        assert_eq!(ac_iss.len(), 3);
        let serial = match ac_ser {
            GserIntegerValue::ReasonableLiteral(i) => i,
            _ => panic!("Not a reasonable integer value for serialNumber."),
        };
        assert_eq!(serial, 56789);
        let iss_name_1 = &ac_iss[0];
        let iss_name_2 = &ac_iss[1];
        let iss_name_3 = &ac_iss[2];
        let dns_name = match iss_name_1 {
            LdapGeneralName::DnsName(n) => n,
            _ => panic!(),
        };
        assert_eq!(dns_name, &"wildboar.software");
        let edi_name = match iss_name_2 {
            LdapGeneralName::EdiPartyName(n) => n,
            _ => panic!(),
        };
        assert!(edi_name.nameAssigner.is_none());
        assert_eq!(edi_name.partyName.as_ref(), "Foobar");
        let oid_name = match iss_name_3 {
            LdapGeneralName::RegisteredId(n) => n,
            _ => panic!(),
        };
        let oid_name_desc = match oid_name {
            GserOidValue::Descriptor(d) => d,
            _ => panic!(),
        };
        assert_eq!(oid_name_desc, &"wildboar");
    }

const TEST_CERT_ASSERTION_03: &str = "{ \
certificateValid utcTime:\"990823052442Z\", \
privateKeyValid \"20081213065544+0004\", \
subjectPublicKeyAlgID rsaEncryption \
}";

    #[test]
    fn parses_cert_assertion_03() {
        let input = TEST_CERT_ASSERTION_03;
        let (s, output) = parse_CertificateAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let cert_valid = output.certificateValid.unwrap();
        let priv_key_valid = output.privateKeyValid.unwrap();
        let subj_pk_alg_id = output.subjectPublicKeyAlgID.unwrap();
        assert_eq!(cert_valid.date.year, 1999);
        assert_eq!(priv_key_valid.date.year, 2008);
        let oid_desc = match subj_pk_alg_id {
            GserOidValue::Descriptor(d) => d,
            _ => panic!(),
        };
        assert_eq!(oid_desc, "rsaEncryption");
    }

const TEST_CERT_ASSERTION_04: &str = "{ \
keyUsage { nonRepudiation, keyEncipherment }, \
subjectAltName builtinNameForm:x400Address, \
policy { wildboar-policy, 1.5.4.3 } \
}";

    #[test]
    fn parses_cert_assertion_04() {
        let input = TEST_CERT_ASSERTION_04;
        let (s, output) = parse_CertificateAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let key_usage = output.keyUsage.unwrap();
        let san = output.subjectAltName.unwrap();
        let policy = output.policy.unwrap();
        let key_usage_bits = match key_usage {
            GserBitStringValue::BitList(list) => list,
            _ => panic!(),
        };
        assert_eq!(key_usage_bits.len(), 2);
        assert_eq!(key_usage_bits[0], "nonRepudiation");
        assert_eq!(key_usage_bits[1], "keyEncipherment");
        if let LdapAltNameType::X400Address = &san {
            // Do nothing.
        } else {
            panic!();
        }
        assert_eq!(policy.len(), 2);
        let policy1 = &policy[0];
        let policy2 = &policy[1];
        let policy1_desc = match policy1 {
            GserOidValue::Descriptor(d) => d,
            _ => panic!(),
        };
        let policy2_oid = match policy2 {
            GserOidValue::Literal(oid) => oid,
            _ => panic!(),
        };
        assert_eq!(policy1_desc, &"wildboar-policy");
        assert_eq!(policy2_oid, &oid!(1, 5, 4, 3));
    }


const TEST_CERT_ASSERTION_05: &str = "{ \
keyUsage '101'B, \
subjectAltName otherNameForm:2.5.4.3, \
pathToName dnsName:\"wildboar.software\", \
subject oid:1.3.4.6.1.56940 \
}";

    #[test]
    fn parses_cert_assertion_05() {
        let input = TEST_CERT_ASSERTION_05;
        let (s, output) = parse_CertificateAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let key_usage = output.keyUsage.unwrap();
        let san = output.subjectAltName.unwrap();
        let path = output.pathToName.unwrap();
        let subject = output.subject.unwrap();
        let key_usage_bits = match key_usage {
            GserBitStringValue::BitString(bs) => bs,
            _ => panic!(),
        };
        assert_eq!(key_usage_bits.trailing_bits, 5);
        if let LdapAltNameType::OtherName(on_oid) = &san {
            let on_oid = match on_oid {
                GserOidValue::Literal(o) => o,
                _ => panic!(),
            };
            assert_eq!(on_oid, &oid!(2, 5, 4, 3));
        } else {
            panic!();
        }
        let path_dns_name = match path {
            LdapName::DnsName(n) => n,
            _ => panic!(),
        };
        assert_eq!(path_dns_name, "wildboar.software");
        let subject_oid = match subject {
            LdapName::Oid(o) => o,
            _ => panic!(),
        };
        let subject_oid = match subject_oid {
            GserOidValue::Literal(o) => o,
            _ => panic!(),
        };
        assert_eq!(subject_oid, oid!(1,3,4,6,1,56940));
    }

const TEST_CERT_ASSERTION_06: &str = "{ \
nameConstraints { excludedSubtrees { \
{ base dNSName:\"careers.mcdonalds.com\", minimum 1, maximum 5 }, \
{ base iPAddress:'08080808'H } \
} } }";

    #[test]
    fn parses_cert_assertion_06() {
        let input = TEST_CERT_ASSERTION_06;
        let (s, output) = parse_CertificateAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let nc = output.nameConstraints.unwrap();
        assert!(nc.permittedSubtrees.is_none());
        let xs = nc.excludedSubtrees.unwrap();
        assert_eq!(xs.len(), 2);
        let xs1 = &xs[0];
        let xs2 = &xs[1];
        assert_eq!(xs1.minimum, Some(1));
        assert_eq!(xs1.maximum, Some(5));
        assert_eq!(xs2.minimum, None);
        assert_eq!(xs2.maximum, None);
    }


const TEST_CERT_EXACT_ASSERTION_01: &str = "{ \
serialNumber 8675309, \
issuer rdnSequence:\"c=US,st=FL,o=Wildboar Software\" \
}";

    #[test]
    fn parses_cert_exact_assertion_01() {
        let input = TEST_CERT_EXACT_ASSERTION_01;
        let (s, output) = parse_CertificateExactAssertion(input).unwrap();
        assert_eq!(s.len(), 0);
        let serial = match output.serialNumber {
            GserIntegerValue::ReasonableLiteral(i) => i,
            _ => panic!(),
        };
        assert_eq!(serial, 8675309);
        let issuer = match output.issuer {
            LdapName::RdnSequence(rdns) => rdns,
            _ => panic!(),
        };
        assert_eq!(issuer, "c=US,st=FL,o=Wildboar Software");
    }

    // TODO: parse_CertificatePairExactAssertion
    // TODO: parse_CertificatePairAssertion
    // TODO: parse_CertificateListExactAssertion
    // TODO: parse_CertificateListAssertion

}
