#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CMSProfileAttributes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CMSProfileAttributes`.
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
use crate::CryptographicMessageSyntax::*;
use crate::TrustedTimeStamp::*;
use asn1::*;
use std::sync::Arc;
use x500::InformationFramework::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-signerInfo ATTRIBUTE ::= {TYPE SignerInfo IDENTIFIED BY id-signerInfo}
/// ```
///
///
pub fn aa_signerInfo() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_signerInfo(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_signerInfo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SignerInfo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SignerInfo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SignerInfo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SignerInfo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signerInfo OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) signerInfo(0)}
/// ```
///
///
pub fn id_signerInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* signerInfo */ 0,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-signerInfos ATTRIBUTE ::= {TYPE SignerInfos IDENTIFIED BY id-signerInfos}
/// ```
///
///
pub fn aa_signerInfos() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_signerInfos(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_signerInfos {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SignerInfos; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SignerInfos(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SignerInfos(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SignerInfos(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signerInfos  OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) signerInfos(1)}
/// ```
///
///
pub fn id_signerInfos() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* signerInfos */ 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-contentLocation ATTRIBUTE ::= {TYPE URI IDENTIFIED BY id-contentLocation}
/// ```
///
///
pub fn aa_contentLocation() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_contentLocation(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_contentLocation {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = URI; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_URI(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_URI(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_URI(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// URI  ::=  UTF8String(SIZE(1..MAX))
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
/// id-contentLocation OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) contentLocation(2)}
/// ```
///
///
pub fn id_contentLocation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* contentLocation */ 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-contentLocations ATTRIBUTE ::= {TYPE URIs IDENTIFIED BY id-contentLocations}
/// ```
///
///
pub fn aa_contentLocations() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_contentLocations(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_contentLocations {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = URIs; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_URIs(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_URIs(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_URIs(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// URIs  ::=  SEQUENCE SIZE(1..MAX) OF uri URI
/// ```
pub type URIs = Vec<URI>; // SequenceOfType

pub fn _decode_URIs(el: &X690Element) -> ASN1Result<URIs> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "URIs")),
    };
    let mut items: SEQUENCE_OF<URI> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_URI(el)?);
    }
    Ok(items)
}

pub fn _encode_URIs(value_: &URIs) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_URI(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_URIs(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_URI(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "URIs")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contentLocations OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) contentLocations(3)}
/// ```
///
///
pub fn id_contentLocations() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* contentLocations */ 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-precedingBlock ATTRIBUTE ::= {TYPE HashPointer IDENTIFIED BY id-precedingBlock}
/// ```
///
///
pub fn aa_precedingBlock() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_precedingBlock(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_precedingBlock {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = HashPointer; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_HashPointer(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_HashPointer(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_HashPointer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashPointer ::= SEQUENCE {
///     hash            DigestedData OPTIONAL,
///     pointers        Pointers OPTIONAL
///     }
///     (
///         (WITH COMPONENTS {..., hash PRESENT}) |
///         (WITH COMPONENTS {..., pointers PRESENT})
///     )
/// ```
///
#[derive(Debug, Clone)]
pub struct HashPointer {
    pub hash: OPTIONAL<DigestedData>,
    pub pointers: OPTIONAL<Pointers>,
}
impl HashPointer {
    pub fn new(hash: OPTIONAL<DigestedData>, pointers: OPTIONAL<Pointers>) -> Self {
        HashPointer { hash, pointers }
    }
}
impl Default for HashPointer {
    fn default() -> Self {
        HashPointer {
            hash: None,
            pointers: None,
        }
    }
}
impl TryFrom<&X690Element> for HashPointer {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_HashPointer(el)
    }
}

pub const _rctl1_components_for_HashPointer: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "hash",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pointers",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_HashPointer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_HashPointer: &[ComponentSpec; 0] = &[];

pub fn _decode_HashPointer(el: &X690Element) -> ASN1Result<HashPointer> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HashPointer")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HashPointer,
        _eal_components_for_HashPointer,
        _rctl2_components_for_HashPointer,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut hash_: OPTIONAL<DigestedData> = None;
    let mut pointers_: OPTIONAL<Pointers> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hash" => hash_ = Some(_decode_DigestedData(_el)?),
            "pointers" => pointers_ = Some(_decode_Pointers(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HashPointer")
                )
            }
        }
    }
    Ok(HashPointer {
        hash: hash_,
        pointers: pointers_,
    })
}

pub fn _encode_HashPointer(value_: &HashPointer) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.hash {
        components_.push(_encode_DigestedData(&v_)?);
    }
    if let Some(v_) = &value_.pointers {
        components_.push(_encode_Pointers(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_HashPointer(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HashPointer")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HashPointer,
        _eal_components_for_HashPointer,
        _rctl2_components_for_HashPointer,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hash" => _validate_DigestedData(_el)?,
            "pointers" => _validate_Pointers(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HashPointer")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Pointers  ::=  SEQUENCE SIZE(1..MAX) OF pointer Pointer
/// ```
pub type Pointers = Vec<Pointer>; // SequenceOfType

pub fn _decode_Pointers(el: &X690Element) -> ASN1Result<Pointers> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Pointers")),
    };
    let mut items: SEQUENCE_OF<Pointer> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Pointer(el)?);
    }
    Ok(items)
}

pub fn _encode_Pointers(value_: &Pointers) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Pointer(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Pointers(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Pointer(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Pointers")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Pointer  ::=  CHOICE {
///     uri         URI,  -- Uniform Resource Identifier
///     rfid        RFID,  -- Radio Frequency Identification
///     gps         GPS,  -- Global Positioning System
///     address     Address, -- Free format object location
///     dbRecord    DBRecord, -- Number of fully qualified name
///     ...         -- Expect other pointer types
///     }
/// ```
#[derive(Debug, Clone)]
pub enum Pointer {
    uri(URI),
    rfid(RFID),
    gps(GPS),
    address(Address),
    dbRecord(DBRecord),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Pointer {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Pointer(el)
    }
}

pub fn _decode_Pointer(el: &X690Element) -> ASN1Result<Pointer> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(Pointer::uri(_decode_URI(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(Pointer::rfid(_decode_RFID(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(Pointer::gps(_decode_GPS(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(Pointer::address(_decode_Address(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(Pointer::dbRecord(_decode_DBRecord(&el)?)),
        _ => Ok(Pointer::_unrecognized(el.clone())),
    }
}

pub fn _encode_Pointer(value_: &Pointer) -> ASN1Result<X690Element> {
    match value_ {
        Pointer::uri(v) => _encode_URI(&v),
        Pointer::rfid(v) => _encode_RFID(&v),
        Pointer::gps(v) => _encode_GPS(&v),
        Pointer::address(v) => _encode_Address(&v),
        Pointer::dbRecord(v) => _encode_DBRecord(&v),
        Pointer::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Pointer(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => _validate_URI(&el),
        (TagClass::CONTEXT, 1) => _validate_RFID(&el),
        (TagClass::CONTEXT, 2) => _validate_GPS(&el),
        (TagClass::CONTEXT, 3) => _validate_Address(&el),
        (TagClass::CONTEXT, 4) => _validate_DBRecord(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RFID  ::=  OCTET STRING
/// ```
pub type RFID = OCTET_STRING; // OctetStringType

pub fn _decode_RFID(el: &X690Element) -> ASN1Result<RFID> {
    BER.decode_octet_string(&el)
}

pub fn _encode_RFID(value_: &RFID) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_RFID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GPS  ::=  OCTET STRING
/// ```
pub type GPS = OCTET_STRING; // OctetStringType

pub fn _decode_GPS(el: &X690Element) -> ASN1Result<GPS> {
    BER.decode_octet_string(&el)
}

pub fn _encode_GPS(value_: &GPS) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_GPS(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Address  ::=  UTF8String
/// ```
pub type Address = UTF8String; // UTF8String

pub fn _decode_Address(el: &X690Element) -> ASN1Result<Address> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_Address(value_: &Address) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_Address(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DBRecord  ::=  UTF8String
/// ```
pub type DBRecord = UTF8String; // UTF8String

pub fn _decode_DBRecord(el: &X690Element) -> ASN1Result<DBRecord> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_DBRecord(value_: &DBRecord) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_DBRecord(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-precedingBlock OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) precedingBlock(4)}
/// ```
///
///
pub fn id_precedingBlock() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* precedingBlock */ 4,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-timeStamped ATTRIBUTE ::= {TYPE TimeStamped IDENTIFIED BY id-timeStamped}
/// ```
///
///
pub fn aa_timeStamped() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_timeStamped(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_timeStamped {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TimeStamped; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TimeStamped(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TimeStamped(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TimeStamped(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStamped ::= SEQUENCE {
///     timeStampValue      TimeStamp,
///     timeStampService    URI OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeStamped {
    pub timeStampValue: TimeStamp,
    pub timeStampService: OPTIONAL<URI>,
}
impl TimeStamped {
    pub fn new(timeStampValue: TimeStamp, timeStampService: OPTIONAL<URI>) -> Self {
        TimeStamped {
            timeStampValue,
            timeStampService,
        }
    }
}
impl TryFrom<&X690Element> for TimeStamped {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStamped(el)
    }
}

pub const _rctl1_components_for_TimeStamped: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "timeStampValue",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStampService",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeStamped: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeStamped: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeStamped(el: &X690Element) -> ASN1Result<TimeStamped> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStamped")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStamped,
        _eal_components_for_TimeStamped,
        _rctl2_components_for_TimeStamped,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut timeStampValue_: OPTIONAL<TimeStamp> = None;
    let mut timeStampService_: OPTIONAL<URI> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "timeStampValue" => timeStampValue_ = Some(_decode_TimeStamp(_el)?),
            "timeStampService" => timeStampService_ = Some(_decode_URI(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStamped")
                )
            }
        }
    }
    Ok(TimeStamped {
        timeStampValue: timeStampValue_.unwrap(),
        timeStampService: timeStampService_,
    })
}

pub fn _encode_TimeStamped(value_: &TimeStamped) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_TimeStamp(&value_.timeStampValue)?);
    if let Some(v_) = &value_.timeStampService {
        components_.push(_encode_URI(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TimeStamped(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStamped")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStamped,
        _eal_components_for_TimeStamped,
        _rctl2_components_for_TimeStamped,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "timeStampValue" => _validate_TimeStamp(_el)?,
            "timeStampService" => _validate_URI(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStamped")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStamp  ::=  CHOICE {
///     timeStampToken      TimeStampToken,
///     localTimeStamp      GeneralizedTime,
///     ... -- Expect additional time types --
/// }
/// ```
#[derive(Debug, Clone)]
pub enum TimeStamp {
    timeStampToken(TimeStampToken),
    localTimeStamp(GeneralizedTime),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for TimeStamp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStamp(el)
    }
}

pub fn _decode_TimeStamp(el: &X690Element) -> ASN1Result<TimeStamp> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(TimeStamp::timeStampToken(_decode_TimeStampToken(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(TimeStamp::localTimeStamp(BER.decode_generalized_time(&el)?)),
        _ => Ok(TimeStamp::_unrecognized(el.clone())),
    }
}

pub fn _encode_TimeStamp(value_: &TimeStamp) -> ASN1Result<X690Element> {
    match value_ {
        TimeStamp::timeStampToken(v) => _encode_TimeStampToken(&v),
        TimeStamp::localTimeStamp(v) => BER.encode_generalized_time(&v),
        TimeStamp::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TimeStamp(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => _validate_TimeStampToken(&el),
        (TagClass::CONTEXT, 1) => BER.validate_generalized_time(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-timeStamped OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) timeStamped(5)}
/// ```
///
///
pub fn id_timeStamped() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* timeStamped */ 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-sidechains ATTRIBUTE ::= {TYPE Sidechains IDENTIFIED BY id-sidechains}
/// ```
///
///
pub fn aa_sidechains() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_sidechains(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_sidechains {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Sidechains; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Sidechains(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Sidechains(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Sidechains(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Sidechains  ::=  SEQUENCE (SIZE(0..MAX)) OF linked Sidechain
/// ```
pub type Sidechains = Vec<Sidechain>; // SequenceOfType

pub fn _decode_Sidechains(el: &X690Element) -> ASN1Result<Sidechains> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Sidechains")),
    };
    let mut items: SEQUENCE_OF<Sidechain> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Sidechain(el)?);
    }
    Ok(items)
}

pub fn _encode_Sidechains(value_: &Sidechains) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Sidechain(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Sidechains(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Sidechain(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Sidechains")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Sidechain  ::=  HashPointer
/// ```
pub type Sidechain = HashPointer; // DefinedType

pub fn _decode_Sidechain(el: &X690Element) -> ASN1Result<Sidechain> {
    _decode_HashPointer(&el)
}

pub fn _encode_Sidechain(value_: &Sidechain) -> ASN1Result<X690Element> {
    _encode_HashPointer(&value_)
}

pub fn _validate_Sidechain(el: &X690Element) -> ASN1Result<()> {
    _validate_HashPointer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sidechains OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) sidechains(6)}
/// ```
///
///
pub fn id_sidechains() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* sidechains */ 6,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-parentBlock ATTRIBUTE ::= {TYPE ParentBlock IDENTIFIED BY id-parentBlock}
/// ```
///
///
pub fn aa_parentBlock() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_parentBlock(), /* OBJECT_FIELD_SETTING */
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

pub mod aa_parentBlock {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ParentBlock; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ParentBlock(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ParentBlock(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ParentBlock(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ParentBlock  ::=  HashPointer
/// ```
pub type ParentBlock = HashPointer; // DefinedType

pub fn _decode_ParentBlock(el: &X690Element) -> ASN1Result<ParentBlock> {
    _decode_HashPointer(&el)
}

pub fn _encode_ParentBlock(value_: &ParentBlock) -> ASN1Result<X690Element> {
    _encode_HashPointer(&value_)
}

pub fn _validate_ParentBlock(el: &X690Element) -> ASN1Result<()> {
    _validate_HashPointer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-parentBlock OBJECT IDENTIFIER ::= {itu-t recommendation(0) x(24)
///     cms-profile(894) attribute(2) parentBlock(7)}
/// ```
///
///
pub fn id_parentBlock() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        itu_t, /* recommendation */ 0, /* x */ 24, /* cms-profile */ 894,
        /* attribute */ 2, /* parentBlock */ 7,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CMSProfileAttributes ATTRIBUTE ::= {
///     aa-signerInfo
///     | aa-signerInfos
///     | aa-contentLocation
///     | aa-contentLocations
///     | aa-precedingBlock
///     | aa-timeStamped
///     | aa-sidechains
///     | aa-parentBlock,
///     ...
///     }
/// ```
///
///
pub fn CMSProfileAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([
        aa_signerInfo(),
        aa_signerInfos(),
        aa_contentLocation(),
        aa_contentLocations(),
        aa_precedingBlock(),
        aa_timeStamped(),
        aa_sidechains(),
        aa_parentBlock(),
    ])
}
