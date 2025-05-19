use std::{collections::HashMap, str::FromStr, ops::Deref};

use cow_utils::CowUtils;
use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_until},
    combinator::opt,
    sequence::{preceded, delimited, tuple},
    character::complete::{digit1, space1, space0, u32 as parse_u32}, multi::{many1, many0},
    Err as NomErr,
    error::Error as NomError,
    error::ErrorKind as NomErrorKind,

};
use nom::branch::alt;
use std::borrow::Cow;
use asn1::OBJECT_IDENTIFIER;

fn is_ldh (c: char) -> bool {
    char::is_ascii_alphanumeric(&c) || c == '-'
}

fn is_ldhu (c: char) -> bool {
    is_ldh(c) || c == '_'
}

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
fn parse_qdescr (s: &str) -> IResult<&str, &str> {
    delimited(tag("'"), parse_descr, tag("'"))(s)
}

fn parse_qdescrlist (s: &str) -> IResult<&str, Vec<&str>> {
    let (s, desc1) = parse_qdescr(s)?;
    let subsequent_parser = preceded(space1, parse_qdescr);
    let (s, subsequent_descs) = many0(subsequent_parser)(s)?;
    let ret = [
        &[ desc1 ],
        subsequent_descs.as_slice(),
    ].concat();
    Ok((s, ret))
}

fn parse_qdescrs (s: &str) -> IResult<&str, Vec<&str>> {
    // If the lone descriptor syntax matches, return that, otherwise, try the descriptor list.
    match parse_qdescr(s) {
        Ok((s, r)) => Ok((s, vec![r])),
        Err(_) => {
            let (s, _) = tag("(")(s)?;
            let (s, _) = space1(s)?;
            let (s, descrs) = parse_qdescrlist(s)?;
            let (s, _) = space1(s)?;
            let (s, _) = tag(")")(s)?;
            Ok((s, descrs))
        },
    }
}

fn parse_qdstring <'a> (s: &'a str) -> IResult<&'a str, Cow<'a, str>> {
    let res: IResult<&'a str, &'a str> = delimited(tag("'"), take_until("'"), tag("'"))(s);
    match res {
        Ok((s, qds)) => {
            // For some reason, I just cannot get .cow_replace() daisy-chaining
            // here successfully, so this is my crappy secondary solution:
            // If the string has no backslash, there are no escapes, so we can
            // just return the string slice as borrowed; otherwise, we replace.
            // This should rarely get called.
            if qds.contains("\\") {
                let unescaped = qds
                    .cow_replace("\\27", "'")
                    .replace("\\5C", "\\")
                    .replace("\\5c", "\\");
                Ok((s, Cow::Owned(unescaped)))
            } else {
                Ok((s, Cow::Borrowed(qds)))
            }
        },
        Err(e) => Err(e),
    }
}

fn parse_qdstringslist (s: &str) -> IResult<&str, Vec<Cow<str>>> {
    let (s, qdstr1) = parse_qdstring(s)?;
    let subsequent_parser = preceded(space1, parse_qdstring);
    let (s, subsequent_descs) = many0(subsequent_parser)(s)?;
    let ret = [
        &[ qdstr1 ],
        subsequent_descs.as_slice(),
    ].concat();
    Ok((s, ret))
}

fn parse_qdstrings (s: &str) -> IResult<&str, Vec<Cow<str>>> {
    // If the lone descriptor syntax matches, return that, otherwise, try the descriptor list.
    match parse_qdstring(s) {
        Ok((s, r)) => Ok((s, vec![r])),
        Err(_) => {
            let (s, _) = tag("(")(s)?;
            let (s, _) = space1(s)?;
            let (s, qdstrs) = parse_qdstringslist(s)?;
            let (s, _) = space1(s)?;
            let (s, _) = tag(")")(s)?;
            Ok((s, qdstrs))
        },
    }
}

fn parse_oidlist (s: &str) -> IResult<&str, Vec<OidOrName>> {
    let (s, oid1) = parse_oid(s)?;
    let subsequent_parser = preceded(tuple((space0, tag("$"), space0)), parse_oid);
    let (s, subsequent_descs) = many0(subsequent_parser)(s)?;
    let ret = [
        &[ oid1 ],
        subsequent_descs.as_slice(),
    ].concat();
    Ok((s, ret))
}

fn parse_ruleid_list (s: &str) -> IResult<&str, Vec<u32>> {
    let (s, ruleid1) = parse_u32(s)?;
    let subsequent_parser = preceded(space1, parse_u32);
    let (s, subsequent_ruleids) = many0(subsequent_parser)(s)?;
    let ret = [
        &[ ruleid1 ],
        subsequent_ruleids.as_slice(),
    ].concat();
    Ok((s, ret))
}

fn parse_numeric_oid(s: &str) -> IResult<&str, OBJECT_IDENTIFIER> {
    let (s, sarc1) = digit1(s)?;
    let arc_parser = preceded(tag("."), digit1);
    let (s, sarcs) = many1(arc_parser)(s)?;
    let mut arcs: Vec<u32> = Vec::with_capacity(1 + sarcs.len());
    let arc1 = u32::from_str(sarc1).unwrap(); // TODO: Can this fail with multiple leading zeroes?
    arcs.push(arc1);
    for sarc in sarcs {
        arcs.push(u32::from_str(sarc).unwrap());
    }
    let oid = OBJECT_IDENTIFIER::try_from(arcs)
        .map_err(|_| NomErr::Failure(NomError::new(s, NomErrorKind::TooLarge)))?;
    Ok((s, oid))
}

#[derive(Clone)]
enum OidOrName <'a> {
    Oid(OBJECT_IDENTIFIER),
    Name(&'a str),
}

fn parse_oid (s: &str) -> IResult<&str, OidOrName> {
    match parse_numeric_oid(s) {
        Ok((s, oid)) => Ok((s, OidOrName::Oid(oid))),
        Err(_) => {
            let (s, descr) = parse_descr(s)?;
            Ok((s, OidOrName::Name(descr)))
        },
    }
}

fn parse_oids (s: &str) -> IResult<&str, Vec<OidOrName>> {
    // If the lone descriptor syntax matches, return that, otherwise, try the descriptor list.
    match parse_oid(s) {
        Ok((s, r)) => Ok((s, vec![r])),
        Err(_) => {
            let (s, _) = tag("(")(s)?;
            let (s, _) = space1(s)?;
            let (s, oids) = parse_oidlist(s)?;
            let (s, _) = space1(s)?;
            let (s, _) = tag(")")(s)?;
            Ok((s, oids))
        },
    }
}

fn parse_rule_ids (s: &str) -> IResult<&str, Vec<u32>> {
    // If the lone descriptor syntax matches, return that, otherwise, try the descriptor list.
    match parse_u32::<&str, ()>(s) {
        Ok((s, r)) => Ok((s, vec![r])),
        Err(_) => {
            let (s, _) = tag("(")(s)?;
            let (s, _) = space1(s)?;
            let (s, ruleids) = parse_ruleid_list(s)?;
            let (s, _) = space1(s)?;
            let (s, _) = tag(")")(s)?;
            Ok((s, ruleids))
        },
    }
}

fn parse_names(s: &str) -> IResult<&str, Vec<&str>> {
    let (s, _) = space1(s)?;
    let (s, _) = tag("NAME")(s)?;
    let (s, _) = space1(s)?;
    parse_qdescrs(s)
}

fn parse_desc(s: &str) -> IResult<&str, Cow<str>> {
    let (s, _) = space1(s)?;
    let (s, _) = tag("DESC")(s)?;
    let (s, _) = space1(s)?;
    parse_qdstring(s)
}

fn parse_superior_rule_ids(s: &str) -> IResult<&str, Vec<u32>> {
    let (s, _) = space1(s)?;
    let (s, _) = tag("SUP")(s)?;
    let (s, _) = space1(s)?;
    parse_rule_ids(s)
}

fn parse_obsolete(s: &str) -> IResult<&str, &str> {
    let (s, _) = space1(s)?;
    tag("OBSOLETE")(s)
}

fn parse_oids_field <'a> (field_name: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<OidOrName<'a>>> {
    Box::new(move |s: &'a str| {
        let (s, _) = space1(s)?;
        let (s, _) = tag(field_name)(s)?;
        let (s, _) = space1(s)?;
        parse_oids(s)
    })
}

fn parse_oid_field <'a> (field_name: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, OidOrName<'a>> {
    Box::new(move |s: &'a str| {
        let (s, _) = space1(s)?;
        let (s, _) = tag(field_name)(s)?;
        let (s, _) = space1(s)?;
        parse_oid(s)
    })
}

fn parse_attr_syntax (s: &str) -> IResult<&str, Noidlen> {
    let (s, _) = space1(s)?;
    let (s, _) = tag("SYNTAX")(s)?;
    let (s, _) = space1(s)?;
    let (s, oid) = parse_numeric_oid(s)?;
    let (s, len) = opt(delimited(tag("{"), parse_u32, tag("}")))(s)?;
    Ok((s, Noidlen{ oid, len }))
}

fn parse_kind(s: &str) -> IResult<&str, ObjectClassKind> {
    let (s, _) = space1(s)?;
    let (s, kind_str) = alt((
        tag("ABSTRACT"),
        tag("STRUCTURAL"),
        tag("AUXILIARY"),
    ))(s)?;
    Ok((s, ObjectClassKind::from_str(kind_str).unwrap()))
}

fn parse_usage(s: &str) -> IResult<&str, AttributeUsage> {
    let (s, _) = space1(s)?;
    let (s, kind_str) = alt((
        tag("userApplications"),
        tag("directoryOperation"),
        tag("distributedOperation"),
        tag("dSAOperation"),
    ))(s)?;
    Ok((s, AttributeUsage::from_str(kind_str).unwrap()))
}

// NOTE: There is technically nothing from prohibiting an extension from starting
// with X-_ or X--.
fn parse_xstring (s: &str) -> IResult<&str, &str> {
    preceded(tag("X-"), take_while(is_ldhu))(s)
}

fn parse_extension (s: &str) -> IResult<&str, (&str, Vec::<Cow<str>>)> {
    let (s, _) = space1(s)?;
    let (s, ext_name) = parse_xstring(s)?;
    let (s, _) = space1(s)?;
    let (s, ext_value) = parse_qdstrings(s)?;
    Ok((s, (ext_name, ext_value)))
}

fn parse_extensions (s: &str) -> IResult<&str, Extensions> {
    let (s, exts) = many0(parse_extension)(s)?;
    let mut exts_map = HashMap::with_capacity(exts.len());
    for ext in exts {
        let (ext_name, ext_value) = ext;
        let value: Vec<String> = ext_value
            .iter()
            .map(|v| v.as_ref().to_owned())
            .collect();
        exts_map.insert(ext_name.to_owned(), value);
    }
    Ok((s, exts_map))
}

fn parse_single_value(s: &str) -> IResult<&str, &str> {
    let (s, _) = space1(s)?;
    tag("SINGLE-VALUE")(s)
}

fn parse_collective(s: &str) -> IResult<&str, &str> {
    let (s, _) = space1(s)?;
    tag("COLLECTIVE")(s)
}

fn parse_no_user_mod(s: &str) -> IResult<&str, &str> {
    let (s, _) = space1(s)?;
    tag("NO-USER-MODIFICATION")(s)
}

fn parse_oc(s: &str) -> IResult<&str, LdapObjectClassDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, maybe_sups) = opt(parse_oids_field("SUP"))(s)?;
    let (s, maybe_kind) = opt(parse_kind)(s)?;
    let (s, maybe_must) = opt(parse_oids_field("MUST"))(s)?;
    let (s, maybe_may) = opt(parse_oids_field("MAY"))(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapObjectClassDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        sup: maybe_sups.map(|sups| sups.iter().map(|oid| oid.into()).collect()),
        must: maybe_must.map(|musts| musts.iter().map(|oid| oid.into()).collect()),
        may: maybe_may.map(|mays| mays.iter().map(|oid| oid.into()).collect()),
        kind: maybe_kind,
        exts,
    }))
}

fn parse_at(s: &str) -> IResult<&str, LdapAttributeTypeDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, sup) = opt(parse_oid_field("SUP"))(s)?;
    let (s, emr) = opt(parse_oid_field("EQUALITY"))(s)?;
    let (s, omr) = opt(parse_oid_field("ORDERING"))(s)?;
    let (s, smr) = opt(parse_oid_field("SUBSTR"))(s)?;
    let (s, syntax) = opt(parse_attr_syntax)(s)?;
    let (s, single_value) = opt(parse_single_value)(s)?;
    let (s, collective) = opt(parse_collective)(s)?;
    let (s, no_user_mod) = opt(parse_no_user_mod)(s)?;
    let (s, usage) = opt(parse_usage)(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapAttributeTypeDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        sup: sup.map(|x| x.into()),
        equality: emr.map(|x| x.into()),
        ordering: omr.map(|x| x.into()),
        substr: smr.map(|x| x.into()),
        syntax,
        single_value: single_value.is_some(),
        collective: collective.is_some(),
        no_user_mod: no_user_mod.is_some(),
        usage,
        exts,
    }))
}

fn parse_mr(s: &str) -> IResult<&str, LdapMatchingRuleDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("SYNTAX")(s)?;
    let (s, _) = space1(s)?;
    let (s, syntax) = parse_numeric_oid(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapMatchingRuleDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        syntax,
        exts,
    }))
}

fn parse_mru(s: &str) -> IResult<&str, LdapMatchingRuleUseDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, applies) = parse_oids_field("APPLIES")(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapMatchingRuleUseDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        applies: applies.iter().map(|x| x.into()).collect(),
        exts,
    }))
}

fn parse_lsx(s: &str) -> IResult<&str, LdapSyntaxDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapSyntaxDescription{
        id,
        desc: desc.map(|d| d.deref().to_owned()),
        exts,
    }))
}

fn parse_cr(s: &str) -> IResult<&str, LdapDITContentRuleDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, aux) = opt(parse_oids_field("AUX"))(s)?;
    let (s, must) = opt(parse_oids_field("MUST"))(s)?;
    let (s, may) = opt(parse_oids_field("MAY"))(s)?;
    let (s, not) = opt(parse_oids_field("NOT"))(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapDITContentRuleDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        aux: aux.map(|x| x.iter().map(|oid| oid.into()).collect()),
        must: must.map(|x| x.iter().map(|oid| oid.into()).collect()),
        may: may.map(|x| x.iter().map(|oid| oid.into()).collect()),
        not: not.map(|x| x.iter().map(|oid| oid.into()).collect()),
        exts,
    }))
}

fn parse_sr(s: &str) -> IResult<&str, LdapDITStructureRuleDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_u32(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, form) = parse_oid_field("FORM")(s)?;
    let (s, sup) = opt(parse_superior_rule_ids)(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapDITStructureRuleDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        form: form.into(),
        sup,
        exts,
    }))
}

fn parse_nf(s: &str) -> IResult<&str, LdapNameFormDescription> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = parse_numeric_oid(s)?;
    let (s, names) = opt(parse_names)(s)?;
    let (s, desc) = opt(parse_desc)(s)?;
    let (s, obs) = opt(parse_obsolete)(s)?;
    let (s, oc) = parse_oid_field("OC")(s)?;
    let (s, must) = parse_oids_field("MUST")(s)?;
    let (s, maybe_may) = opt(parse_oids_field("MAY"))(s)?;
    let (s, exts) = parse_extensions(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, LdapNameFormDescription{
        id,
        names: names.unwrap_or(vec![]).iter().map(|name| (*name).to_owned()).collect(),
        desc: desc.map(|d| d.deref().to_owned()),
        obsolete: obs.is_some(),
        oc: oc.into(),
        must: must.iter().map(|attr| attr.into()).collect(),
        may: maybe_may.map(|may| may.iter().map(|attr| attr.into()).collect()),
        exts,
    }))
}

#[derive(Clone, Debug, Eq)]
pub struct NamedOID {
    pub name: Option<String>,
    pub oid: Option<OBJECT_IDENTIFIER>,
}

impl PartialEq for NamedOID {

    fn eq(&self, other: &Self) -> bool {
        if self.name.is_some() && other.name.is_some() {
            self.name.as_ref().unwrap()
                .eq_ignore_ascii_case(other.name.as_ref().unwrap().as_str())
        }
        else if self.oid.is_some() && other.oid.is_some() {
            self.oid.as_ref().unwrap() == other.oid.as_ref().unwrap()
        }
        else {
            false
        }
    }

}

impl <'a> From<OidOrName<'a>> for NamedOID {

    fn from(value: OidOrName) -> Self {
        match value {
            OidOrName::Name(n) => NamedOID{ name: Some(n.to_owned()), oid: None },
            OidOrName::Oid(oid) => NamedOID{ name: None, oid: Some(oid) },
        }
    }

}

impl <'a> From<&OidOrName<'a>> for NamedOID {

    fn from(value: &OidOrName) -> Self {
        match value {
            OidOrName::Name(n) => NamedOID{ name: Some((*n).to_owned()), oid: None },
            OidOrName::Oid(oid) => NamedOID{ name: None, oid: Some(oid.to_owned()) },
        }
    }

}

impl <'a> From<&'a str> for NamedOID {

    fn from(value: &'a str) -> Self {
        NamedOID{
            name: Some(value.to_owned()),
            oid: None,
        }
    }

}

impl From<OBJECT_IDENTIFIER> for NamedOID {

    fn from(value: OBJECT_IDENTIFIER) -> Self {
        NamedOID{
            name: None,
            oid: Some(value),
        }
    }

}


#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum ObjectClassKind {
    ABSTRACT,
    STRUCTURAL,
    AUXILIARY,
}

impl FromStr for ObjectClassKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ABSTRACT" => Ok(ObjectClassKind::ABSTRACT),
            "STRUCTURAL" => Ok(ObjectClassKind::STRUCTURAL),
            "AUXILIARY" => Ok(ObjectClassKind::AUXILIARY),
            _ => Err(()),
        }
    }

}

pub type Extensions = HashMap<String, Vec<String>>;

#[derive(Debug, Clone)]
pub struct LdapObjectClassDescription {
    pub id: OBJECT_IDENTIFIER,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub sup: Option<Vec<NamedOID>>,
    pub kind: Option<ObjectClassKind>,
    pub must: Option<Vec<NamedOID>>,
    pub may: Option<Vec<NamedOID>>,
    pub exts: Extensions,
}

impl LdapObjectClassDescription {
    pub fn from_oid (oid: OBJECT_IDENTIFIER) -> Self {
        LdapObjectClassDescription {
            id: oid,
            names: vec![],
            desc: None,
            obsolete: false,
            sup: None,
            kind: None,
            must: None,
            may: None,
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapObjectClassDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, oc) = parse_oc(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(oc)
    }
}

#[derive(Debug, Clone)]
pub struct Noidlen {
    pub oid: OBJECT_IDENTIFIER, // Yes, it is a numeric-only OID.
    pub len: Option<u32>, // u32 is big enough. Anything above this is effectively unbounded.
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AttributeUsage {
    UserApplications,
    DirectoryOperation,
    DistributedOperation,
    DSAOperation,
}

impl FromStr for AttributeUsage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userApplications" => Ok(AttributeUsage::UserApplications),
            "directoryOperation" => Ok(AttributeUsage::DirectoryOperation),
            "distributedOperation" => Ok(AttributeUsage::DistributedOperation),
            "dSAOperation" => Ok(AttributeUsage::DSAOperation),
            _ => Err(()),
        }
    }

}

#[derive(Debug, Clone)]
pub struct LdapAttributeTypeDescription {
    pub id: OBJECT_IDENTIFIER,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub sup: Option<NamedOID>,
    pub equality: Option<NamedOID>,
    pub ordering: Option<NamedOID>,
    pub substr: Option<NamedOID>,
    pub syntax: Option<Noidlen>,
    pub single_value: bool,
    pub collective: bool,
    pub no_user_mod: bool,
    pub usage: Option<AttributeUsage>,
    pub exts: Extensions,
}

impl LdapAttributeTypeDescription {
    pub fn from_oid (oid: OBJECT_IDENTIFIER) -> Self {
        LdapAttributeTypeDescription {
            id: oid,
            names: vec![],
            desc: None,
            obsolete: false,
            sup: None,
            equality: None,
            ordering: None,
            substr: None,
            syntax: None,
            single_value: false,
            collective: false,
            no_user_mod: false,
            usage: None,
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapAttributeTypeDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, at) = parse_at(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(at)
    }
}

#[derive(Debug, Clone)]
pub struct LdapMatchingRuleDescription {
    pub id: OBJECT_IDENTIFIER,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub syntax: OBJECT_IDENTIFIER,
    pub exts: Extensions,
}

impl LdapMatchingRuleDescription {
    pub fn from_oids (id: OBJECT_IDENTIFIER, syntax: OBJECT_IDENTIFIER) -> Self {
        LdapMatchingRuleDescription {
            id,
            names: vec![],
            desc: None,
            obsolete: false,
            syntax,
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapMatchingRuleDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, mr) = parse_mr(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(mr)
    }
}

#[derive(Debug, Clone)]
pub struct LdapMatchingRuleUseDescription {
    pub id: OBJECT_IDENTIFIER,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub applies: Vec<NamedOID>,
    pub exts: Extensions,
}

impl LdapMatchingRuleUseDescription {
    pub fn from_oid (id: OBJECT_IDENTIFIER) -> Self {
        LdapMatchingRuleUseDescription {
            id,
            names: vec![],
            desc: None,
            obsolete: false,
            applies: vec![],
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapMatchingRuleUseDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, mru) = parse_mru(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(mru)
    }
}

#[derive(Debug, Clone)]
pub struct LdapSyntaxDescription {
    pub id: OBJECT_IDENTIFIER,
    pub desc: Option<String>,
    pub exts: Extensions,
}

impl LdapSyntaxDescription {
    pub fn from_oid (id: OBJECT_IDENTIFIER) -> Self {
        LdapSyntaxDescription {
            id,
            desc: None,
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapSyntaxDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, lsx) = parse_lsx(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(lsx)
    }
}

#[derive(Debug, Clone)]
pub struct LdapDITContentRuleDescription {
    pub id: OBJECT_IDENTIFIER,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub aux: Option<Vec<NamedOID>>,
    pub must: Option<Vec<NamedOID>>,
    pub may: Option<Vec<NamedOID>>,
    pub not: Option<Vec<NamedOID>>,
    pub exts: Extensions,
}

impl LdapDITContentRuleDescription {
    pub fn from_oid (id: OBJECT_IDENTIFIER) -> Self {
        LdapDITContentRuleDescription {
            id,
            names: vec![],
            desc: None,
            obsolete: false,
            aux: None,
            must: None,
            may: None,
            not: None,
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapDITContentRuleDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, cr) = parse_cr(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(cr)
    }
}

/// In X.500, this can be signed, but the LDAP syntax constrains this.
pub type RuleId = u32;

#[derive(Debug, Clone)]
pub struct LdapDITStructureRuleDescription {
    pub id: RuleId,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub form: NamedOID,
    pub sup: Option<Vec<RuleId>>,
    pub exts: Extensions,
}

impl LdapDITStructureRuleDescription {
    pub fn from_id_and_nf (id: RuleId, nf: OBJECT_IDENTIFIER) -> Self {
        LdapDITStructureRuleDescription {
            id,
            names: vec![],
            desc: None,
            obsolete: false,
            form: nf.into(),
            sup: None,
            exts: HashMap::new(),
        }
    }
}

impl FromStr for LdapDITStructureRuleDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, sr) = parse_sr(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(sr)
    }
}

#[derive(Debug, Clone)]
pub struct LdapNameFormDescription {
    pub id: OBJECT_IDENTIFIER,
    pub names: Vec<String>,
    pub desc: Option<String>,
    pub obsolete: bool,
    pub oc: NamedOID,
    pub must: Vec<NamedOID>,
    pub may: Option<Vec<NamedOID>>,
    pub exts: Extensions,
}

impl FromStr for LdapNameFormDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, nf) = parse_nf(s).map_err(|_| ())?;
        if s.trim().len() > 0 { // Fail if not all output was consumed.
            return Err(());
        }
        Ok(nf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asn1::oid;

    #[test]
    fn parsing_oc_works() {
        let s = "( 2.5.6.14 NAME 'device' SUP top STRUCTURAL MUST cn MAY ( serialNumber $ seeAlso ) )";
        let oc = LdapObjectClassDescription::from_str(s).unwrap();
        assert_eq!(oc.id, oid!(2, 5, 6, 14));
        assert_eq!(oc.names, vec![ String::from("device") ]);
        assert_eq!(oc.sup.unwrap(), vec![ "top".into() ]);
        assert_eq!(oc.kind.unwrap(), ObjectClassKind::STRUCTURAL);
        assert_eq!(oc.must.unwrap(), vec![ "cn".into() ]);
        assert_eq!(oc.may.unwrap(), vec![ "serialNumber".into(), "seeAlso".into() ]);
        assert_eq!(oc.exts.len(), 0);
        assert!(oc.desc.is_none());
        assert!(!oc.obsolete);
    }

    #[test]
    fn parsing_at_works() {
        let s = "( 0.9.2342.19200300.100.1.25 NAME 'dc' EQUALITY caseIgnoreIA5Match SUBSTR caseIgnoreIA5SubstringsMatch SYNTAX 1.3.6.1.4.1.1466.115.121.1.26 SINGLE-VALUE )";
        let at = LdapAttributeTypeDescription::from_str(s).unwrap();
        assert_eq!(at.id, oid!(0, 9, 2342, 19200300, 100, 1, 25));
        assert_eq!(at.names, vec![ String::from("dc") ]);
        assert_eq!(at.equality.unwrap().name, Some("caseIgnoreIA5Match".into()));
        assert_eq!(at.substr.unwrap().name, Some("caseIgnoreIA5SubstringsMatch".into()));
        assert_eq!(at.exts.len(), 0);
        assert!(at.ordering.is_none());
        assert_eq!(at.syntax.as_ref().unwrap().oid, oid!(1, 3, 6, 1, 4, 1, 1466, 115, 121, 1, 26));
        assert!(at.syntax.unwrap().len.is_none());
        assert!(at.sup.is_none());
        assert!(at.desc.is_none());
        assert!(!at.obsolete);
        assert!(at.single_value);
        assert!(!at.collective);
        assert!(!at.no_user_mod);
    }

    #[test]
    fn parsing_mr_works() {
        let s = "( 2.5.13.1 NAME 'distinguishedNameMatch' SYNTAX 1.3.6.1.4.1.1466.115.121.1.12 )";
        let mr = LdapMatchingRuleDescription::from_str(s).unwrap();
        assert_eq!(mr.id, oid!(2, 5, 13, 1));
        assert_eq!(mr.names, vec![ String::from("distinguishedNameMatch") ]);
        assert_eq!(mr.syntax, oid!(1, 3, 6, 1, 4, 1, 1466, 115, 121, 1, 12));
        assert_eq!(mr.exts.len(), 0);
        assert!(mr.desc.is_none());
        assert!(!mr.obsolete);
    }

    // TODO: matching rule use
    // TODO: DIT content rule

    #[test]
    fn parsing_sr_works() {
        let s = "( 2 NAME 'uddiContactStructureRule' FORM uddiContactNameForm SUP ( 1 ) )";
        let sr = LdapDITStructureRuleDescription::from_str(s).unwrap();
        assert_eq!(sr.id, 2);
        assert_eq!(sr.names, vec![ String::from("uddiContactStructureRule") ]);
        assert_eq!(sr.form.name, Some("uddiContactNameForm".into()));
        assert_eq!(sr.sup.unwrap(), Vec::from([ 1 ]));
        assert!(sr.form.oid.is_none());
        assert_eq!(sr.exts.len(), 0);
        assert!(sr.desc.is_none());
        assert!(!sr.obsolete);
    }

    #[test]
    fn parsing_nf_works() {
        let s = "( 1.3.6.1.1.10.15.1 NAME 'uddiBusinessEntityNameForm' OC uddiBusinessEntity MUST ( uddiBusinessKey ) )";
        let nf = LdapNameFormDescription::from_str(s).unwrap();
        assert_eq!(nf.id, oid!(1, 3, 6, 1, 1, 10, 15, 1));
        assert_eq!(nf.names, vec![ String::from("uddiBusinessEntityNameForm") ]);
        assert!(nf.oc.oid.is_none());
        assert_eq!(nf.oc.name.as_ref().unwrap(), "uddiBusinessEntity");
        assert_eq!(nf.must, Vec::from([ "uddiBusinessKey".into() ]));
        assert!(nf.may.is_none());
        assert_eq!(nf.exts.len(), 0);
        assert!(nf.desc.is_none());
        assert!(!nf.obsolete);
    }

}
