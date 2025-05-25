#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # TokenizationManifest
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `TokenizationManifest`.
//!
//! This compilation was produced by the
//! [Wildboar Software](https://wildboarsoftware.com/en)
//! [ASN.1 Compiler](https://wildboarsoftware.com/en/asn1-compilation).
//!
//! Types from the source ASN.1 module are generally available by their original
//! names, but with hyphens replaced by underscores. Encoders and decoders for
//! any given type are available as `_encode_TYPENAME()` and
//! `_decode_TYPENAME()`. Decoders are also available as implementations of
//! the `From<X690Element` and `From<&'a X690Element>` traits for some
//! types.
//!
use crate::CMSObjectIdentifiers::*;
use wildboar_asn1::*;
use std::sync::Arc;
use x500::InformationFramework::ATTRIBUTE;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// TokenizedParts  ::=  Tokenized {{ Manifest }}
/// ```
pub type TokenizedParts = Tokenized; // DefinedType

pub fn _decode_TokenizedParts(el: &X690Element) -> ASN1Result<TokenizedParts> {
    _decode_Tokenized(&el)
}

pub fn _encode_TokenizedParts(value_: &TokenizedParts) -> ASN1Result<X690Element> {
    _encode_Tokenized(&value_)
}

pub fn _validate_TokenizedParts(el: &X690Element) -> ASN1Result<()> {
    _validate_Tokenized(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Manifest TOKENIZED ::= {
/// xPathTokensManifest,
/// ...  -- Expect additional manifest objects --
/// }
/// ```
///
///
pub fn Manifest() -> Vec<TOKENIZED> {
    Vec::from([xPathTokensManifest()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// xPathTokensManifest TOKENIZED ::= {
/// OID id-XPathTokensSet PARMS XPathTokensSet
/// }
/// ```
///
///
pub fn xPathTokensManifest() -> TOKENIZED {
    TOKENIZED {
        id: id_XPathTokensSet(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod xPathTokensManifest {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = XPathTokensSet; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_XPathTokensSet(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_XPathTokensSet(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_XPathTokensSet(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// XPathTokensSet ::= SEQUENCE {
/// tSP       TokenServiceProvider OPTIONAL,
/// xPathSet  XPathSet
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct XPathTokensSet {
    pub tSP: OPTIONAL<TokenServiceProvider>,
    pub xPathSet: XPathSet,
}
impl XPathTokensSet {
    pub fn new(tSP: OPTIONAL<TokenServiceProvider>, xPathSet: XPathSet) -> Self {
        XPathTokensSet { tSP, xPathSet }
    }
}
impl TryFrom<&X690Element> for XPathTokensSet {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_XPathTokensSet(el)
    }
}

pub const _rctl1_components_for_XPathTokensSet: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "tSP",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "xPathSet",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_XPathTokensSet: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_XPathTokensSet: &[ComponentSpec; 0] = &[];

pub fn _decode_XPathTokensSet(el: &X690Element) -> ASN1Result<XPathTokensSet> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "XPathTokensSet"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_XPathTokensSet,
        _eal_components_for_XPathTokensSet,
        _rctl2_components_for_XPathTokensSet,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tSP_: OPTIONAL<TokenServiceProvider> = None;
    let mut xPathSet_: OPTIONAL<XPathSet> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tSP" => tSP_ = Some(_decode_TokenServiceProvider(_el)?),
            "xPathSet" => xPathSet_ = Some(_decode_XPathSet(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "XPathTokensSet")
                )
            }
        }
    }
    Ok(XPathTokensSet {
        tSP: tSP_,
        xPathSet: xPathSet_.unwrap(),
    })
}

pub fn _encode_XPathTokensSet(value_: &XPathTokensSet) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.tSP {
        components_.push(_encode_TokenServiceProvider(&v_)?);
    }
    components_.push(_encode_XPathSet(&value_.xPathSet)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_XPathTokensSet(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "XPathTokensSet"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_XPathTokensSet,
        _eal_components_for_XPathTokensSet,
        _rctl2_components_for_XPathTokensSet,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tSP" => _validate_TokenServiceProvider(_el)?,
            "xPathSet" => _validate_XPathSet(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "XPathTokensSet")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TokenServiceProvider  ::=  URI
/// ```
pub type TokenServiceProvider = URI; // DefinedType

pub fn _decode_TokenServiceProvider(el: &X690Element) -> ASN1Result<TokenServiceProvider> {
    _decode_URI(&el)
}

pub fn _encode_TokenServiceProvider(value_: &TokenServiceProvider) -> ASN1Result<X690Element> {
    _encode_URI(&value_)
}

pub fn _validate_TokenServiceProvider(el: &X690Element) -> ASN1Result<()> {
    _validate_URI(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// URI  ::=  UTF8String (SIZE(1..MAX))
/// ```
pub type URI = UTF8String; // UTF8String

pub fn _decode_URI(el: &X690Element) -> ASN1Result<URI> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_URI(value_: &URI) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_URI(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// XPathSet  ::=  SEQUENCE SIZE(1..MAX) OF xpath XPath
/// ```
pub type XPathSet = Vec<XPath>; // SequenceOfType

pub fn _decode_XPathSet(el: &X690Element) -> ASN1Result<XPathSet> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "XPathSet")),
    };
    let mut items: SEQUENCE_OF<XPath> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_XPath(el)?);
    }
    Ok(items)
}

pub fn _encode_XPathSet(value_: &XPathSet) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_XPath(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_XPathSet(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_XPath(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "XPathSet")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// XPath  ::=  UTF8String (CONSTRAINED BY { -- XML Path Language 2.0 -- })
/// ```
pub type XPath = UTF8String; // UTF8String

pub fn _decode_XPath(el: &X690Element) -> ASN1Result<XPath> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_XPath(value_: &XPath) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_XPath(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tokenizedParts ATTRIBUTE ::= { TYPE TokenizedParts IDENTIFIED BY id-tokenizedParts }
/// ```
///
pub fn tokenizedParts() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_tokenizedParts(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod tokenizedParts {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TokenizedParts; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TokenizedParts(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TokenizedParts(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TokenizedParts(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TOKENIZED ::= CLASS {
/// &id OBJECT IDENTIFIER UNIQUE,
/// &Type OPTIONAL
/// }
/// WITH SYNTAX { OID &id [ PARMS &Type ] }
/// ```
///
#[derive(Debug)]
pub struct TOKENIZED {
    pub id: OBJECT_IDENTIFIER,
}
impl TOKENIZED {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Tokenized { TOKENIZED:IOSet } ::= SEQUENCE {
/// name  TOKENIZED.&id({IOSet}),
/// parts TOKENIZED.&Type({IOSet}{@name}) OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Tokenized {
    pub name: OBJECT_IDENTIFIER,
    pub parts: OPTIONAL<X690Element>,
}
impl Tokenized {
    pub fn new(name: OBJECT_IDENTIFIER, parts: OPTIONAL<X690Element>) -> Self {
        Tokenized { name, parts }
    }
}
impl TryFrom<&X690Element> for Tokenized {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Tokenized(el)
    }
}

pub const _rctl1_components_for_Tokenized: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "parts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Tokenized: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Tokenized: &[ComponentSpec; 0] = &[];

pub fn _decode_Tokenized(el: &X690Element) -> ASN1Result<Tokenized> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Tokenized")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Tokenized,
        _eal_components_for_Tokenized,
        _rctl2_components_for_Tokenized,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut name_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parts_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "name" => name_ = Some(BER.decode_object_identifier(_el)?),
            "parts" => parts_ = Some(x690_identity(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Tokenized"))
            }
        }
    }
    Ok(Tokenized {
        name: name_.unwrap(),
        parts: parts_,
    })
}

pub fn _encode_Tokenized(value_: &Tokenized) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.name)?);
    if let Some(v_) = &value_.parts {
        components_.push(x690_identity(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Tokenized(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Tokenized")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Tokenized,
        _eal_components_for_Tokenized,
        _rctl2_components_for_Tokenized,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "name" => BER.validate_object_identifier(_el)?,
            "parts" => BER.validate_any(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Tokenized"))
            }
        }
    }
    Ok(())
}
