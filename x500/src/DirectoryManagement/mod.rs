#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DirectoryManagement
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DirectoryManagement`.
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
use crate::DSAOperationalAttributeTypes::*;
use crate::DistributedOperations::*;
use crate::InformationFramework::*;
use crate::SchemaAdministration::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use wildboar_asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// Accessors  ::=  SET OF Name
/// ```
pub type Accessors = Vec<Name>; // SetOfType

pub fn _decode_Accessors(el: &X690Element) -> ASN1Result<Accessors> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Accessors")),
    };
    let mut items: SET_OF<Name> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Name(el)?);
    }
    Ok(items)
}

pub fn _encode_Accessors(value_: &Accessors) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Name(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Accessors(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Name(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Accessors")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AdministrativeRole  ::=  OBJECT-CLASS.&id
/// ```
pub type AdministrativeRole = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_AdministrativeRole(el: &X690Element) -> ASN1Result<AdministrativeRole> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_AdministrativeRole(value_: &AdministrativeRole) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_AdministrativeRole(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ApplicationContext  ::=  OBJECT IDENTIFIER
/// ```
pub type ApplicationContext = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_ApplicationContext(el: &X690Element) -> ASN1Result<ApplicationContext> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_ApplicationContext(value_: &ApplicationContext) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_ApplicationContext(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AssociationEstablishment  ::=  BIT STRING {inward(0), outward(1)}
/// ```
pub type AssociationEstablishment = BIT_STRING;

pub const AssociationEstablishment_inward: BIT_INDEX = 0; /* LONG_NAMED_BIT */

pub const AssociationEstablishment_outward: BIT_INDEX = 1; /* LONG_NAMED_BIT */

pub fn _decode_AssociationEstablishment(el: &X690Element) -> ASN1Result<AssociationEstablishment> {
    BER.decode_bit_string(&el)
}

pub fn _encode_AssociationEstablishment(
    value_: &AssociationEstablishment,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_AssociationEstablishment(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AssociationId  ::=  INTEGER
/// ```
pub type AssociationId = INTEGER;

pub fn _decode_AssociationId(el: &X690Element) -> ASN1Result<AssociationId> {
    BER.decode_integer(&el)
}

pub fn _encode_AssociationId(value_: &AssociationId) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_AssociationId(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenReasonSyntax  ::=  INTEGER {
///   unknownUser(0), incorrectPassword(1), inaccessiblePassword(2),
///   passwordVerificationLoop(3), unrecognizedUser(4)}
/// ```
pub type AuthenReasonSyntax = i8;

pub const AuthenReasonSyntax_unknownUser: AuthenReasonSyntax = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_incorrectPassword: AuthenReasonSyntax = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_inaccessiblePassword: AuthenReasonSyntax = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_passwordVerificationLoop: AuthenReasonSyntax = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_unrecognizedUser: AuthenReasonSyntax = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AuthenReasonSyntax(el: &X690Element) -> ASN1Result<AuthenReasonSyntax> {
    BER.decode_i8(&el)
}

pub fn _encode_AuthenReasonSyntax(value_: &AuthenReasonSyntax) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_AuthenReasonSyntax(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryInformationServiceElement ::= SEQUENCE {
///   operationType
///     BIT STRING {read(0), compare(1), abandon(2), list(3), search(4),
///                 addEntry(5), removeEntry(6), modifyEntry(7), modifyDN(8)}
///       OPTIONAL,
///   attributeType   AttributeType OPTIONAL,
///   attributeValue  [0]  AttributeValue OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct DirectoryInformationServiceElement {
    pub operationType: OPTIONAL<DirectoryInformationServiceElement_operationType>,
    pub attributeType: OPTIONAL<AttributeType>,
    pub attributeValue: OPTIONAL<AttributeValue>,
}
impl DirectoryInformationServiceElement {
    pub fn new(
        operationType: OPTIONAL<DirectoryInformationServiceElement_operationType>,
        attributeType: OPTIONAL<AttributeType>,
        attributeValue: OPTIONAL<AttributeValue>,
    ) -> Self {
        DirectoryInformationServiceElement {
            operationType,
            attributeType,
            attributeValue,
        }
    }
}
impl Default for DirectoryInformationServiceElement {
    fn default() -> Self {
        DirectoryInformationServiceElement {
            operationType: None,
            attributeType: None,
            attributeValue: None,
        }
    }
}
impl TryFrom<&X690Element> for DirectoryInformationServiceElement {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryInformationServiceElement(el)
    }
}

pub const _rctl1_components_for_DirectoryInformationServiceElement: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "operationType",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeType",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeValue",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DirectoryInformationServiceElement: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DirectoryInformationServiceElement: &[ComponentSpec; 0] = &[];

pub fn _decode_DirectoryInformationServiceElement(
    el: &X690Element,
) -> ASN1Result<DirectoryInformationServiceElement> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "DirectoryInformationServiceElement",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DirectoryInformationServiceElement,
        _eal_components_for_DirectoryInformationServiceElement,
        _rctl2_components_for_DirectoryInformationServiceElement,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut operationType_: OPTIONAL<DirectoryInformationServiceElement_operationType> = None;
    let mut attributeType_: OPTIONAL<AttributeType> = None;
    let mut attributeValue_: OPTIONAL<AttributeValue> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "operationType" => {
                operationType_ = Some(_decode_DirectoryInformationServiceElement_operationType(
                    _el,
                )?)
            }
            "attributeType" => attributeType_ = Some(_decode_AttributeType(_el)?),
            "attributeValue" => {
                attributeValue_ = Some(|el: &X690Element| -> ASN1Result<AttributeValue> {
                    Ok(_decode_AttributeValue(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "DirectoryInformationServiceElement",
                ))
            }
        }
    }
    Ok(DirectoryInformationServiceElement {
        operationType: operationType_,
        attributeType: attributeType_,
        attributeValue: attributeValue_,
    })
}

pub fn _encode_DirectoryInformationServiceElement(
    value_: &DirectoryInformationServiceElement,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.operationType {
        components_.push(_encode_DirectoryInformationServiceElement_operationType(
            &v_,
        )?);
    }
    if let Some(v_) = &value_.attributeType {
        components_.push(_encode_AttributeType(&v_)?);
    }
    if let Some(v_) = &value_.attributeValue {
        components_.push(|v_1: &AttributeValue| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(_encode_AttributeValue(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_DirectoryInformationServiceElement(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "DirectoryInformationServiceElement",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DirectoryInformationServiceElement,
        _eal_components_for_DirectoryInformationServiceElement,
        _rctl2_components_for_DirectoryInformationServiceElement,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "operationType" => _validate_DirectoryInformationServiceElement_operationType(_el)?,
            "attributeType" => _validate_AttributeType(_el)?,
            "attributeValue" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeValue")
                    );
                }
                Ok(_validate_AttributeValue(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "DirectoryInformationServiceElement",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSAScopeOfChainingValue  ::=  INTEGER {dmd(0), country(1), global(2)}
/// ```
pub type DSAScopeOfChainingValue = i8;

pub const DSAScopeOfChainingValue_dmd: DSAScopeOfChainingValue = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfChainingValue_country: DSAScopeOfChainingValue = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfChainingValue_global: DSAScopeOfChainingValue = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_DSAScopeOfChainingValue(el: &X690Element) -> ASN1Result<DSAScopeOfChainingValue> {
    BER.decode_i8(&el)
}

pub fn _encode_DSAScopeOfChainingValue(
    value_: &DSAScopeOfChainingValue,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_DSAScopeOfChainingValue(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSAScopeOfReferralValue  ::=  INTEGER {dmd(0), country(1), global(2)}
/// ```
pub type DSAScopeOfReferralValue = i8;

pub const DSAScopeOfReferralValue_dmd: DSAScopeOfReferralValue = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfReferralValue_country: DSAScopeOfReferralValue = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfReferralValue_global: DSAScopeOfReferralValue = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_DSAScopeOfReferralValue(el: &X690Element) -> ASN1Result<DSAScopeOfReferralValue> {
    BER.decode_i8(&el)
}

pub fn _encode_DSAScopeOfReferralValue(
    value_: &DSAScopeOfReferralValue,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_DSAScopeOfReferralValue(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HOBRole  ::=  INTEGER {superior(0), subordinate(1)}
/// ```
pub type HOBRole = i8;

pub const HOBRole_superior: HOBRole = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const HOBRole_subordinate: HOBRole = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_HOBRole(el: &X690Element) -> ASN1Result<HOBRole> {
    BER.decode_i8(&el)
}

pub fn _encode_HOBRole(value_: &HOBRole) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_HOBRole(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtBitString  ::=  BIT STRING
/// ```
pub type MgtBitString = BIT_STRING;

pub fn _decode_MgtBitString(el: &X690Element) -> ASN1Result<MgtBitString> {
    BER.decode_bit_string(&el)
}

pub fn _encode_MgtBitString(value_: &MgtBitString) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_MgtBitString(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtBoolean  ::=  BOOLEAN
/// ```
pub type MgtBoolean = BOOLEAN; // BooleanType

pub fn _decode_MgtBoolean(el: &X690Element) -> ASN1Result<MgtBoolean> {
    BER.decode_boolean(&el)
}

pub fn _encode_MgtBoolean(value_: &MgtBoolean) -> ASN1Result<X690Element> {
    BER.encode_boolean(&value_)
}

pub fn _validate_MgtBoolean(el: &X690Element) -> ASN1Result<()> {
    BER.validate_boolean(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtCommonName  ::=  UnboundedDirectoryString
/// ```
pub type MgtCommonName = UnboundedDirectoryString; // DefinedType

pub fn _decode_MgtCommonName(el: &X690Element) -> ASN1Result<MgtCommonName> {
    _decode_UnboundedDirectoryString(&el)
}

pub fn _encode_MgtCommonName(value_: &MgtCommonName) -> ASN1Result<X690Element> {
    _encode_UnboundedDirectoryString(&value_)
}

pub fn _validate_MgtCommonName(el: &X690Element) -> ASN1Result<()> {
    _validate_UnboundedDirectoryString(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtGeneralizedTime  ::=  GeneralizedTime
/// ```
pub type MgtGeneralizedTime = GeneralizedTime; // GeneralizedTime

pub fn _decode_MgtGeneralizedTime(el: &X690Element) -> ASN1Result<MgtGeneralizedTime> {
    BER.decode_generalized_time(&el)
}

pub fn _encode_MgtGeneralizedTime(value_: &MgtGeneralizedTime) -> ASN1Result<X690Element> {
    BER.encode_generalized_time(&value_)
}

pub fn _validate_MgtGeneralizedTime(el: &X690Element) -> ASN1Result<()> {
    BER.validate_generalized_time(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtInteger  ::=  INTEGER
/// ```
pub type MgtInteger = INTEGER;

pub fn _decode_MgtInteger(el: &X690Element) -> ASN1Result<MgtInteger> {
    BER.decode_integer(&el)
}

pub fn _encode_MgtInteger(value_: &MgtInteger) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_MgtInteger(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtName  ::=  Name
/// ```
pub type MgtName = Name; // DefinedType

pub fn _decode_MgtName(el: &X690Element) -> ASN1Result<MgtName> {
    _decode_Name(&el)
}

pub fn _encode_MgtName(value_: &MgtName) -> ASN1Result<X690Element> {
    _encode_Name(&value_)
}

pub fn _validate_MgtName(el: &X690Element) -> ASN1Result<()> {
    _validate_Name(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtOctetString  ::=  OCTET STRING
/// ```
pub type MgtOctetString = OCTET_STRING; // OctetStringType

pub fn _decode_MgtOctetString(el: &X690Element) -> ASN1Result<MgtOctetString> {
    BER.decode_octet_string(&el)
}

pub fn _encode_MgtOctetString(value_: &MgtOctetString) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_MgtOctetString(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtOID  ::=  OBJECT IDENTIFIER
/// ```
pub type MgtOID = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_MgtOID(el: &X690Element) -> ASN1Result<MgtOID> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_MgtOID(value_: &MgtOID) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_MgtOID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtPrintableString  ::=  PrintableString
/// ```
pub type MgtPrintableString = PrintableString; // PrintableString

pub fn _decode_MgtPrintableString(el: &X690Element) -> ASN1Result<MgtPrintableString> {
    BER.decode_printable_string(&el)
}

pub fn _encode_MgtPrintableString(value_: &MgtPrintableString) -> ASN1Result<X690Element> {
    BER.encode_printable_string(&value_)
}

pub fn _validate_MgtPrintableString(el: &X690Element) -> ASN1Result<()> {
    BER.validate_printable_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PeerEntityAuthenticationPolicy  ::=  BIT STRING {
///   none(0), nameOnly(1), simpleUnprotected(2), simpleProtected(3), strong(4),
///   external(5)}
/// ```
pub type PeerEntityAuthenticationPolicy = BIT_STRING;

pub const PeerEntityAuthenticationPolicy_none: BIT_INDEX = 0; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_nameOnly: BIT_INDEX = 1; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_simpleUnprotected: BIT_INDEX = 2; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_simpleProtected: BIT_INDEX = 3; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_strong: BIT_INDEX = 4; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_external: BIT_INDEX = 5; /* LONG_NAMED_BIT */

pub fn _decode_PeerEntityAuthenticationPolicy(
    el: &X690Element,
) -> ASN1Result<PeerEntityAuthenticationPolicy> {
    BER.decode_bit_string(&el)
}

pub fn _encode_PeerEntityAuthenticationPolicy(
    value_: &PeerEntityAuthenticationPolicy,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_PeerEntityAuthenticationPolicy(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RemoteDSAList  ::=  SET OF AccessPoint
/// ```
pub type RemoteDSAList = Vec<AccessPoint>; // SetOfType

pub fn _decode_RemoteDSAList(el: &X690Element) -> ASN1Result<RemoteDSAList> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RemoteDSAList")),
    };
    let mut items: SET_OF<AccessPoint> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AccessPoint(el)?);
    }
    Ok(items)
}

pub fn _encode_RemoteDSAList(value_: &RemoteDSAList) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AccessPoint(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RemoteDSAList(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AccessPoint(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RemoteDSAList")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestAuthenticationPolicy  ::=  BIT STRING {none(0), simpleName(1), strong(2)}
/// ```
pub type RequestAuthenticationPolicy = BIT_STRING;

pub const RequestAuthenticationPolicy_none: BIT_INDEX = 0; /* LONG_NAMED_BIT */

pub const RequestAuthenticationPolicy_simpleName: BIT_INDEX = 1; /* LONG_NAMED_BIT */

pub const RequestAuthenticationPolicy_strong: BIT_INDEX = 2; /* LONG_NAMED_BIT */

pub fn _decode_RequestAuthenticationPolicy(
    el: &X690Element,
) -> ASN1Result<RequestAuthenticationPolicy> {
    BER.decode_bit_string(&el)
}

pub fn _encode_RequestAuthenticationPolicy(
    value_: &RequestAuthenticationPolicy,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_RequestAuthenticationPolicy(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ResourceSyntax  ::=  INTEGER {
///   insufficientMemory(0), insufficientAssociations(1), insufficientDiskSpace(2),
///   miscellaneousResourceExhausted(4)}
/// ```
pub type ResourceSyntax = i8;

pub const ResourceSyntax_insufficientMemory: ResourceSyntax = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ResourceSyntax_insufficientAssociations: ResourceSyntax = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ResourceSyntax_insufficientDiskSpace: ResourceSyntax = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const ResourceSyntax_miscellaneousResourceExhausted: ResourceSyntax = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ResourceSyntax(el: &X690Element) -> ASN1Result<ResourceSyntax> {
    BER.decode_i8(&el)
}

pub fn _encode_ResourceSyntax(value_: &ResourceSyntax) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_ResourceSyntax(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ResultAuthenticationPolicy  ::=  RequestAuthenticationPolicy
/// ```
pub type ResultAuthenticationPolicy = RequestAuthenticationPolicy; // DefinedType

pub fn _decode_ResultAuthenticationPolicy(
    el: &X690Element,
) -> ASN1Result<ResultAuthenticationPolicy> {
    _decode_RequestAuthenticationPolicy(&el)
}

pub fn _encode_ResultAuthenticationPolicy(
    value_: &ResultAuthenticationPolicy,
) -> ASN1Result<X690Element> {
    _encode_RequestAuthenticationPolicy(&value_)
}

pub fn _validate_ResultAuthenticationPolicy(el: &X690Element) -> ASN1Result<()> {
    _validate_RequestAuthenticationPolicy(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecondaryShadows  ::=  SET OF SupplierAndConsumers
/// ```
pub type SecondaryShadows = Vec<SupplierAndConsumers>; // SetOfType

pub fn _decode_SecondaryShadows(el: &X690Element) -> ASN1Result<SecondaryShadows> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SecondaryShadows")
            )
        }
    };
    let mut items: SET_OF<SupplierAndConsumers> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_SupplierAndConsumers(el)?);
    }
    Ok(items)
}

pub fn _encode_SecondaryShadows(value_: &SecondaryShadows) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_SupplierAndConsumers(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SecondaryShadows(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_SupplierAndConsumers(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SecondaryShadows")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowingRole  ::=  INTEGER {supplier(0), consumer(1)}
/// ```
pub type ShadowingRole = i8;

pub const ShadowingRole_supplier: ShadowingRole = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowingRole_consumer: ShadowingRole = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ShadowingRole(el: &X690Element) -> ASN1Result<ShadowingRole> {
    BER.decode_i8(&el)
}

pub fn _encode_ShadowingRole(value_: &ShadowingRole) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_ShadowingRole(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubSchemaSyntax  ::=
///   SEQUENCE OF
///     SEQUENCE {name       [1]  Name, --  Name of the subschema subentry for the subschema
///               subSchema
///                 [2]  SEQUENCE {structureRules
///                                  [1]  SEQUENCE OF DITStructureRuleDescription
///                                    OPTIONAL,
///                                contentRules
///                                  [2]  SEQUENCE OF DITContentRuleDescription
///                                    OPTIONAL,
///                                matchingRules
///                                  [3]  SEQUENCE OF MatchingRuleDescription
///                                    OPTIONAL,
///                                attributeTypes
///                                  [4]  SEQUENCE OF AttributeTypeDescription
///                                    OPTIONAL,
///                                objectClasses
///                                  [5]  SEQUENCE OF ObjectClassDescription
///                                    OPTIONAL,
///                                nameForms
///                                  [6]  SEQUENCE OF NameFormDescription OPTIONAL,
///                                matchRuleUses
///                                  [7]  SEQUENCE OF MatchingRuleUseDescription
///                                    OPTIONAL}}
/// ```
pub type SubSchemaSyntax = Vec<SubSchemaSyntax_Item>; // SequenceOfType

pub fn _decode_SubSchemaSyntax(el: &X690Element) -> ASN1Result<SubSchemaSyntax> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubSchemaSyntax"))
        }
    };
    let mut items: SEQUENCE_OF<SubSchemaSyntax_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_SubSchemaSyntax_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_SubSchemaSyntax(value_: &SubSchemaSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_SubSchemaSyntax_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SubSchemaSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_SubSchemaSyntax_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubSchemaSyntax")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedApplicationContexts  ::=  SET OF OBJECT IDENTIFIER
/// ```
pub type SupportedApplicationContexts = Vec<OBJECT_IDENTIFIER>; // SetOfType

pub fn _decode_SupportedApplicationContexts(
    el: &X690Element,
) -> ASN1Result<SupportedApplicationContexts> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SupportedApplicationContexts",
            ))
        }
    };
    let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_object_identifier(el)?);
    }
    Ok(items)
}

pub fn _encode_SupportedApplicationContexts(
    value_: &SupportedApplicationContexts,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_object_identifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SupportedApplicationContexts(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_object_identifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "SupportedApplicationContexts",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// zero INTEGER ::= 0
/// ```
///
///
pub const zero: i64 = 0;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac OBJECT IDENTIFIER ::= {id-mgt 0}
/// ```
///
///
#[inline]
pub fn id_mac () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mgt(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat OBJECT IDENTIFIER ::= {id-mgt 1}
/// ```
///
///
#[inline]
pub fn id_mat () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mgt(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc OBJECT IDENTIFIER ::= {id-mgt 2}
/// ```
///
///
#[inline]
pub fn id_moc () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mgt(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb OBJECT IDENTIFIER ::= {id-mgt 3}
/// ```
///
///
#[inline]
pub fn id_mnb () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mgt(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp OBJECT IDENTIFIER ::= {id-mgt 4}
/// ```
///
///
#[inline]
pub fn id_mp () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mgt(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa OBJECT IDENTIFIER ::= {id-mgt 5}
/// ```
///
///
#[inline]
pub fn id_mpa () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mgt(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac-useRemoteDSA OBJECT IDENTIFIER ::= {id-mac 0}
/// ```
///
///
#[inline]
pub fn id_mac_useRemoteDSA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 0, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac-useHomeDSA OBJECT IDENTIFIER ::= {id-mac 1}
/// ```
///
///
#[inline]
pub fn id_mac_useHomeDSA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 0, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac-update OBJECT IDENTIFIER ::= {id-mac 2}
/// ```
///
///
#[inline]
pub fn id_mac_update () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 0, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accessPoint OBJECT IDENTIFIER ::= {id-mat 0}
/// ```
///
///
#[inline]
pub fn id_mat_accessPoint () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-masterEntries OBJECT IDENTIFIER ::= {id-mat 1}
/// ```
///
///
#[inline]
pub fn id_mat_masterEntries () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-copyEntries OBJECT IDENTIFIER ::= {id-mat 2}
/// ```
///
///
#[inline]
pub fn id_mat_copyEntries () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-loopsDetected OBJECT IDENTIFIER ::= {id-mat 3}
/// ```
///
///
#[inline]
pub fn id_mat_loopsDetected () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-securityErrors OBJECT IDENTIFIER ::= {id-mat 4}
/// ```
///
///
#[inline]
pub fn id_mat_securityErrors () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nameErrors OBJECT IDENTIFIER ::= {id-mat 5}
/// ```
///
///
#[inline]
pub fn id_mat_nameErrors () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-foundLocalEntries OBJECT IDENTIFIER ::= {id-mat 6}
/// ```
///
///
#[inline]
pub fn id_mat_foundLocalEntries () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-referrals OBJECT IDENTIFIER ::= {id-mat 7}
/// ```
///
///
#[inline]
pub fn id_mat_referrals () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-serviceErrors OBJECT IDENTIFIER ::= {id-mat 8}
/// ```
///
///
#[inline]
pub fn id_mat_serviceErrors () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasDereferences OBJECT IDENTIFIER ::= {id-mat 9}
/// ```
///
///
#[inline]
pub fn id_mat_aliasDereferences () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chainings OBJECT IDENTIFIER ::= {id-mat 10}
/// ```
///
///
#[inline]
pub fn id_mat_chainings () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-invalidReferences OBJECT IDENTIFIER ::= {id-mat 11}
/// ```
///
///
#[inline]
pub fn id_mat_invalidReferences () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-unableToProceed OBJECT IDENTIFIER ::= {id-mat 12}
/// ```
///
///
#[inline]
pub fn id_mat_unableToProceed () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-outOfScope OBJECT IDENTIFIER ::= {id-mat 13}
/// ```
///
///
#[inline]
pub fn id_mat_outOfScope () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-noSuchObject OBJECT IDENTIFIER ::= {id-mat 14}
/// ```
///
///
#[inline]
pub fn id_mat_noSuchObject () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasProblem OBJECT IDENTIFIER ::= {id-mat 15}
/// ```
///
///
#[inline]
pub fn id_mat_aliasProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasDereferencingProblem OBJECT IDENTIFIER ::= {id-mat 16}
/// ```
///
///
#[inline]
pub fn id_mat_aliasDereferencingProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-affectsMultipleDSAs OBJECT IDENTIFIER ::= {id-mat 17}
/// ```
///
///
#[inline]
pub fn id_mat_affectsMultipleDSAs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-unavailableCriticalExtension OBJECT IDENTIFIER ::= {id-mat 18}
/// ```
///
///
#[inline]
pub fn id_mat_unavailableCriticalExtension () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 18 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeLimitExceeded OBJECT IDENTIFIER ::= {id-mat 19}
/// ```
///
///
#[inline]
pub fn id_mat_timeLimitExceeded () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 19 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-sizeLimitExceeded OBJECT IDENTIFIER ::= {id-mat 20}
/// ```
///
///
#[inline]
pub fn id_mat_sizeLimitExceeded () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 20 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-adminLimitExceeded OBJECT IDENTIFIER ::= {id-mat 21}
/// ```
///
///
#[inline]
pub fn id_mat_adminLimitExceeded () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 21 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-prohibitChaining OBJECT IDENTIFIER ::= {id-mat 24}
/// ```
///
///
#[inline]
pub fn id_mat_prohibitChaining () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 24 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-readOpsProc OBJECT IDENTIFIER ::= {id-mat 25}
/// ```
///
///
#[inline]
pub fn id_mat_readOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 25 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-compareOpsProc OBJECT IDENTIFIER ::= {id-mat 26}
/// ```
///
///
#[inline]
pub fn id_mat_compareOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 26 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-abandonOpsProc OBJECT IDENTIFIER ::= {id-mat 27}
/// ```
///
///
#[inline]
pub fn id_mat_abandonOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 27 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-listOpsProc OBJECT IDENTIFIER ::= {id-mat 28}
/// ```
///
///
#[inline]
pub fn id_mat_listOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 28 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-searchBaseOpsProc OBJECT IDENTIFIER ::= {id-mat 29}
/// ```
///
///
#[inline]
pub fn id_mat_searchBaseOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 29 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-search1LevelOpsProc OBJECT IDENTIFIER ::= {id-mat 30}
/// ```
///
///
#[inline]
pub fn id_mat_search1LevelOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 30 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-searchSubtreeOpsProc OBJECT IDENTIFIER ::= {id-mat 31}
/// ```
///
///
#[inline]
pub fn id_mat_searchSubtreeOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 31 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-addEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 32}
/// ```
///
///
#[inline]
pub fn id_mat_addEntryOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 32 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-removeEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 33}
/// ```
///
///
#[inline]
pub fn id_mat_removeEntryOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 33 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 34}
/// ```
///
///
#[inline]
pub fn id_mat_modifyEntryOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 34 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyDNOpsProc OBJECT IDENTIFIER ::= {id-mat 35}
/// ```
///
///
#[inline]
pub fn id_mat_modifyDNOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 35 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chReadOpsProc OBJECT IDENTIFIER ::= {id-mat 36}
/// ```
///
///
#[inline]
pub fn id_mat_chReadOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 36 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chCompareOpsProc OBJECT IDENTIFIER ::= {id-mat 37}
/// ```
///
///
#[inline]
pub fn id_mat_chCompareOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 37 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chAbandonOpsProc OBJECT IDENTIFIER ::= {id-mat 38}
/// ```
///
///
#[inline]
pub fn id_mat_chAbandonOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 38 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chListOpsProc OBJECT IDENTIFIER ::= {id-mat 39}
/// ```
///
///
#[inline]
pub fn id_mat_chListOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 39 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chSearchBaseOpsProc OBJECT IDENTIFIER ::= {id-mat 40}
/// ```
///
///
#[inline]
pub fn id_mat_chSearchBaseOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 40 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chSearch1LevelOpsProc OBJECT IDENTIFIER ::= {id-mat 41}
/// ```
///
///
#[inline]
pub fn id_mat_chSearch1LevelOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 41 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chSearchSubtreeOpsProc OBJECT IDENTIFIER ::= {id-mat 42}
/// ```
///
///
#[inline]
pub fn id_mat_chSearchSubtreeOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 42 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chAddEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 43}
/// ```
///
///
#[inline]
pub fn id_mat_chAddEntryOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 43 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chRemoveEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 44}
/// ```
///
///
#[inline]
pub fn id_mat_chRemoveEntryOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 44 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chModifyEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 45}
/// ```
///
///
#[inline]
pub fn id_mat_chModifyEntryOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 45 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chModifyDNOpsProc OBJECT IDENTIFIER ::= {id-mat 46}
/// ```
///
///
#[inline]
pub fn id_mat_chModifyDNOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 46 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSAScopeOfReferral OBJECT IDENTIFIER ::= {id-mat 47}
/// ```
///
///
#[inline]
pub fn id_mat_dSAScopeOfReferral () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 47 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSAScopeOfChaining OBJECT IDENTIFIER ::= {id-mat 48}
/// ```
///
///
#[inline]
pub fn id_mat_dSAScopeOfChaining () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 48 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-peerEntityAuthenticationPolicy OBJECT IDENTIFIER ::= {id-mat 49}
/// ```
///
///
#[inline]
pub fn id_mat_peerEntityAuthenticationPolicy () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 49 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-requestAuthenticationPolicy OBJECT IDENTIFIER ::= {id-mat 50}
/// ```
///
///
#[inline]
pub fn id_mat_requestAuthenticationPolicy () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 50 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-resultAuthenticationPolicy OBJECT IDENTIFIER ::= {id-mat 51}
/// ```
///
///
#[inline]
pub fn id_mat_resultAuthenticationPolicy () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 51 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSPAssociationEstablishment OBJECT IDENTIFIER ::= {id-mat 52}
/// ```
///
///
#[inline]
pub fn id_mat_dSPAssociationEstablishment () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 52 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dOPAssociationEstablishment OBJECT IDENTIFIER ::= {id-mat 53}
/// ```
///
///
#[inline]
pub fn id_mat_dOPAssociationEstablishment () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 53 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dISPAssociationEstablishment OBJECT IDENTIFIER ::= {id-mat 54}
/// ```
///
///
#[inline]
pub fn id_mat_dISPAssociationEstablishment () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 54 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDAPAssociations OBJECT IDENTIFIER ::= {id-mat 55}
/// ```
///
///
#[inline]
pub fn id_mat_maxDAPAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 55 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDSPAssociations OBJECT IDENTIFIER ::= {id-mat 56}
/// ```
///
///
#[inline]
pub fn id_mat_maxDSPAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 56 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDOPAssociations OBJECT IDENTIFIER ::= {id-mat 57}
/// ```
///
///
#[inline]
pub fn id_mat_maxDOPAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 57 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDISPAssociations OBJECT IDENTIFIER ::= {id-mat 58}
/// ```
///
///
#[inline]
pub fn id_mat_maxDISPAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 58 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dAPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 59}
/// ```
///
///
#[inline]
pub fn id_mat_dAPAssociationTimeout () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 59 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 60}
/// ```
///
///
#[inline]
pub fn id_mat_dSPAssociationTimeout () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 60 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dOPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 61}
/// ```
///
///
#[inline]
pub fn id_mat_dOPAssociationTimeout () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 61 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dISPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 62}
/// ```
///
///
#[inline]
pub fn id_mat_dISPAssociationTimeout () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 62 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSAActiveAssociations OBJECT IDENTIFIER ::= {id-mat 63}
/// ```
///
///
#[inline]
pub fn id_mat_dSAActiveAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 63 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-pagedResultsMaxIDs OBJECT IDENTIFIER ::= {id-mat 64}
/// ```
///
///
#[inline]
pub fn id_mat_pagedResultsMaxIDs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 64 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-pagedResultsTimer OBJECT IDENTIFIER ::= {id-mat 65}
/// ```
///
///
#[inline]
pub fn id_mat_pagedResultsTimer () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 65 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-homeDSA OBJECT IDENTIFIER ::= {id-mat 66}
/// ```
///
///
#[inline]
pub fn id_mat_homeDSA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 66 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dUATimeout OBJECT IDENTIFIER ::= {id-mat 68}
/// ```
///
///
#[inline]
pub fn id_mat_dUATimeout () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 68 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-supportedApplicationContexts OBJECT IDENTIFIER ::= {id-mat 69}
/// ```
///
///
#[inline]
pub fn id_mat_supportedApplicationContexts () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 69 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-reverseCredentials OBJECT IDENTIFIER ::= {id-mat 70}
/// ```
///
///
#[inline]
pub fn id_mat_reverseCredentials () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 70 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-remoteAccessPoint OBJECT IDENTIFIER ::= {id-mat 71}
/// ```
///
///
#[inline]
pub fn id_mat_remoteAccessPoint () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 71 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxInboundAssociations OBJECT IDENTIFIER ::= {id-mat 72}
/// ```
///
///
#[inline]
pub fn id_mat_maxInboundAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 72 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxOutboundAssociations OBJECT IDENTIFIER ::= {id-mat 73}
/// ```
///
///
#[inline]
pub fn id_mat_maxOutboundAssociations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 73 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-currentActiveAssocs OBJECT IDENTIFIER ::= {id-mat 74}
/// ```
///
///
#[inline]
pub fn id_mat_currentActiveAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 74 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-currentActiveInboundAssocs OBJECT IDENTIFIER ::= {id-mat 75}
/// ```
///
///
#[inline]
pub fn id_mat_currentActiveInboundAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 75 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-currentActiveOutboundAssocs OBJECT IDENTIFIER ::= {id-mat 76}
/// ```
///
///
#[inline]
pub fn id_mat_currentActiveOutboundAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 76 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumAssocs OBJECT IDENTIFIER ::= {id-mat 77}
/// ```
///
///
#[inline]
pub fn id_mat_accumAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 77 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumInboundAssocs OBJECT IDENTIFIER ::= {id-mat 78}
/// ```
///
///
#[inline]
pub fn id_mat_accumInboundAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 78 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumOutboundAssocs OBJECT IDENTIFIER ::= {id-mat 79}
/// ```
///
///
#[inline]
pub fn id_mat_accumOutboundAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 79 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumFailedInboundAssocs OBJECT IDENTIFIER ::= {id-mat 80}
/// ```
///
///
#[inline]
pub fn id_mat_accumFailedInboundAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 80 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumFailedOutboundAssocs OBJECT IDENTIFIER ::= {id-mat 81}
/// ```
///
///
#[inline]
pub fn id_mat_accumFailedOutboundAssocs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 81 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeOfLastAttempt OBJECT IDENTIFIER ::= {id-mat 82}
/// ```
///
///
#[inline]
pub fn id_mat_timeOfLastAttempt () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 82 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeOfLastSuccess OBJECT IDENTIFIER ::= {id-mat 83}
/// ```
///
///
#[inline]
pub fn id_mat_timeOfLastSuccess () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 83 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-requestCounter OBJECT IDENTIFIER ::= {id-mat 84}
/// ```
///
///
#[inline]
pub fn id_mat_requestCounter () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 84 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-replyCounter OBJECT IDENTIFIER ::= {id-mat 85}
/// ```
///
///
#[inline]
pub fn id_mat_replyCounter () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 85 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-requestsFailedCounter OBJECT IDENTIFIER ::= {id-mat 86}
/// ```
///
///
#[inline]
pub fn id_mat_requestsFailedCounter () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 86 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeOfLastAccess OBJECT IDENTIFIER ::= {id-mat 87}
/// ```
///
///
#[inline]
pub fn id_mat_timeOfLastAccess () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 87 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-agreementID OBJECT IDENTIFIER ::= {id-mat 88}
/// ```
///
///
#[inline]
pub fn id_mat_agreementID () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 88 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-agreementVersion OBJECT IDENTIFIER ::= {id-mat 89}
/// ```
///
///
#[inline]
pub fn id_mat_agreementVersion () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 89 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-hOBRole OBJECT IDENTIFIER ::= {id-mat 90}
/// ```
///
///
#[inline]
pub fn id_mat_hOBRole () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 90 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-shadowingSubject OBJECT IDENTIFIER ::= {id-mat 91}
/// ```
///
///
#[inline]
pub fn id_mat_shadowingSubject () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 91 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-updateMode OBJECT IDENTIFIER ::= {id-mat 92}
/// ```
///
///
#[inline]
pub fn id_mat_updateMode () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 92 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-masterAccessPoint OBJECT IDENTIFIER ::= {id-mat 93}
/// ```
///
///
#[inline]
pub fn id_mat_masterAccessPoint () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 93 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-secondaryShadows OBJECT IDENTIFIER ::= {id-mat 94}
/// ```
///
///
#[inline]
pub fn id_mat_secondaryShadows () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 94 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-shadowingRole OBJECT IDENTIFIER ::= {id-mat 95}
/// ```
///
///
#[inline]
pub fn id_mat_shadowingRole () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 95 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-lastUpdateTime OBJECT IDENTIFIER ::= {id-mat 96}
/// ```
///
///
#[inline]
pub fn id_mat_lastUpdateTime () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 96 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-shadowingSchedule OBJECT IDENTIFIER ::= {id-mat 97}
/// ```
///
///
#[inline]
pub fn id_mat_shadowingSchedule () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 97 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nextUpdateTime OBJECT IDENTIFIER ::= {id-mat 98}
/// ```
///
///
#[inline]
pub fn id_mat_nextUpdateTime () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 98 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-useDOP OBJECT IDENTIFIER ::= {id-mat 99}
/// ```
///
///
#[inline]
pub fn id_mat_useDOP () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 99 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accessor OBJECT IDENTIFIER ::= {id-mat 100}
/// ```
///
///
#[inline]
pub fn id_mat_accessor () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 100 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-allowedInfoService OBJECT IDENTIFIER ::= {id-mat 101}
/// ```
///
///
#[inline]
pub fn id_mat_allowedInfoService () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 101 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-applicationContextInUse OBJECT IDENTIFIER ::= {id-mat 102}
/// ```
///
///
#[inline]
pub fn id_mat_applicationContextInUse () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 102 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-associationId OBJECT IDENTIFIER ::= {id-mat 103}
/// ```
///
///
#[inline]
pub fn id_mat_associationId () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 103 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-callingAETitle OBJECT IDENTIFIER ::= {id-mat 104}
/// ```
///
///
#[inline]
pub fn id_mat_callingAETitle () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 104 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-disAllowedInfoService OBJECT IDENTIFIER ::= {id-mat 105}
/// ```
///
///
#[inline]
pub fn id_mat_disAllowedInfoService () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 105 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxEntriesReturned OBJECT IDENTIFIER ::= {id-mat 106}
/// ```
///
///
#[inline]
pub fn id_mat_maxEntriesReturned () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 106 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxTimeForResult OBJECT IDENTIFIER ::= {id-mat 107}
/// ```
///
///
#[inline]
pub fn id_mat_maxTimeForResult () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 107 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyDNRenameOnlyOpsProc OBJECT IDENTIFIER ::= {id-mat 108}
/// ```
///
///
#[inline]
pub fn id_mat_modifyDNRenameOnlyOpsProc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 108 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-serviceDesc OBJECT IDENTIFIER ::= {id-mat 109}
/// ```
///
///
#[inline]
pub fn id_mat_serviceDesc () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 109 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-serviceId OBJECT IDENTIFIER ::= {id-mat 110}
/// ```
///
///
#[inline]
pub fn id_mat_serviceId () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 110 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-subSchema OBJECT IDENTIFIER ::= {id-mat 111}
/// ```
///
///
#[inline]
pub fn id_mat_subSchema () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 111 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-sizeLimit OBJECT IDENTIFIER ::= {id-mat 112}
/// ```
///
///
#[inline]
pub fn id_mat_sizeLimit () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 112 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeLimit OBJECT IDENTIFIER ::= {id-mat 113}
/// ```
///
///
#[inline]
pub fn id_mat_timeLimit () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 113 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirCustName OBJECT IDENTIFIER ::= {id-mat 114}
/// ```
///
///
#[inline]
pub fn id_mat_dirCustName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 114 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirUserName OBJECT IDENTIFIER ::= {id-mat 115}
/// ```
///
///
#[inline]
pub fn id_mat_dirUserName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 115 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirCustAddr OBJECT IDENTIFIER ::= {id-mat 116}
/// ```
///
///
#[inline]
pub fn id_mat_dirCustAddr () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 116 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dMDName OBJECT IDENTIFIER ::= {id-mat 117}
/// ```
///
///
#[inline]
pub fn id_mat_dMDName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 117 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accessControlScheme OBJECT IDENTIFIER ::= {id-mat 119}
/// ```
///
///
#[inline]
pub fn id_mat_accessControlScheme () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 119 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-administrativeRole OBJECT IDENTIFIER ::= {id-mat 120}
/// ```
///
///
#[inline]
pub fn id_mat_administrativeRole () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 120 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasedEntryName OBJECT IDENTIFIER ::= {id-mat 121}
/// ```
///
///
#[inline]
pub fn id_mat_aliasedEntryName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 121 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-attributeTypes OBJECT IDENTIFIER ::= {id-mat 122}
/// ```
///
///
#[inline]
pub fn id_mat_attributeTypes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 122 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-collectiveExclusions OBJECT IDENTIFIER ::= {id-mat 123}
/// ```
///
///
#[inline]
pub fn id_mat_collectiveExclusions () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 123 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-consumerKnowledge OBJECT IDENTIFIER ::= {id-mat 124}
/// ```
///
///
#[inline]
pub fn id_mat_consumerKnowledge () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 124 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-createTimestamp OBJECT IDENTIFIER ::= {id-mat 125}
/// ```
///
///
#[inline]
pub fn id_mat_createTimestamp () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 125 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-creatorsName OBJECT IDENTIFIER ::= {id-mat 126}
/// ```
///
///
#[inline]
pub fn id_mat_creatorsName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 126 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-credentials OBJECT IDENTIFIER ::= {id-mat 127}
/// ```
///
///
#[inline]
pub fn id_mat_credentials () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 127 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-distName OBJECT IDENTIFIER ::= {id-mat 128}
/// ```
///
///
#[inline]
pub fn id_mat_distName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dITContentRules OBJECT IDENTIFIER ::= {id-mat 129}
/// ```
///
///
#[inline]
pub fn id_mat_dITContentRules () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dITStructureRule OBJECT IDENTIFIER ::= {id-mat 130}
/// ```
///
///
#[inline]
pub fn id_mat_dITStructureRule () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dseType OBJECT IDENTIFIER ::= {id-mat 131}
/// ```
///
///
#[inline]
pub fn id_mat_dseType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-entryACI OBJECT IDENTIFIER ::= {id-mat 132}
/// ```
///
///
#[inline]
pub fn id_mat_entryACI () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-governingSR OBJECT IDENTIFIER ::= {id-mat 133}
/// ```
///
///
#[inline]
pub fn id_mat_governingSR () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-matchingRules OBJECT IDENTIFIER ::= {id-mat 134}
/// ```
///
///
#[inline]
pub fn id_mat_matchingRules () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-matchingRuleUse OBJECT IDENTIFIER ::= {id-mat 135}
/// ```
///
///
#[inline]
pub fn id_mat_matchingRuleUse () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifiersName OBJECT IDENTIFIER ::= {id-mat 136}
/// ```
///
///
#[inline]
pub fn id_mat_modifiersName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyTimestamp OBJECT IDENTIFIER ::= {id-mat 137}
/// ```
///
///
#[inline]
pub fn id_mat_modifyTimestamp () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-myAccessPoint OBJECT IDENTIFIER ::= {id-mat 138}
/// ```
///
///
#[inline]
pub fn id_mat_myAccessPoint () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nonSpecificKnowledge OBJECT IDENTIFIER ::= {id-mat 139}
/// ```
///
///
#[inline]
pub fn id_mat_nonSpecificKnowledge () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-objectClass OBJECT IDENTIFIER ::= {id-mat 140}
/// ```
///
///
#[inline]
pub fn id_mat_objectClass () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-objectClasses OBJECT IDENTIFIER ::= {id-mat 141}
/// ```
///
///
#[inline]
pub fn id_mat_objectClasses () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-prescriptiveACI OBJECT IDENTIFIER ::= {id-mat 142}
/// ```
///
///
#[inline]
pub fn id_mat_prescriptiveACI () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nameForms OBJECT IDENTIFIER ::= {id-mat 143}
/// ```
///
///
#[inline]
pub fn id_mat_nameForms () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-specificKnowledge OBJECT IDENTIFIER ::= {id-mat 144}
/// ```
///
///
#[inline]
pub fn id_mat_specificKnowledge () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-structuralObjectClass OBJECT IDENTIFIER ::= {id-mat 145}
/// ```
///
///
#[inline]
pub fn id_mat_structuralObjectClass () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-subentryACI OBJECT IDENTIFIER ::= {id-mat 146}
/// ```
///
///
#[inline]
pub fn id_mat_subentryACI () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 18 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-subtreeSpecification OBJECT IDENTIFIER ::= {id-mat 147}
/// ```
///
///
#[inline]
pub fn id_mat_subtreeSpecification () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 19 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-superiorKnowledge OBJECT IDENTIFIER ::= {id-mat 148}
/// ```
///
///
#[inline]
pub fn id_mat_superiorKnowledge () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 20 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-supplierKnowledge OBJECT IDENTIFIER ::= {id-mat 149}
/// ```
///
///
#[inline]
pub fn id_mat_supplierKnowledge () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 21 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirCommonName OBJECT IDENTIFIER ::= {id-mat 150}
/// ```
///
///
#[inline]
pub fn id_mat_dirCommonName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 1, 0x81, 22 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dsa OBJECT IDENTIFIER ::= {id-moc 0}
/// ```
///
///
#[inline]
pub fn id_moc_dsa () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dse OBJECT IDENTIFIER ::= {id-moc 1}
/// ```
///
///
#[inline]
pub fn id_moc_dse () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-knownDSA OBJECT IDENTIFIER ::= {id-moc 2}
/// ```
///
///
#[inline]
pub fn id_moc_knownDSA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-knownDUA OBJECT IDENTIFIER ::= {id-moc 3}
/// ```
///
///
#[inline]
pub fn id_moc_knownDUA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dUA OBJECT IDENTIFIER ::= {id-moc 4}
/// ```
///
///
#[inline]
pub fn id_moc_dUA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-nHOBMO OBJECT IDENTIFIER ::= {id-moc 5}
/// ```
///
///
#[inline]
pub fn id_moc_nHOBMO () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-hOBMO OBJECT IDENTIFIER ::= {id-moc 6}
/// ```
///
///
#[inline]
pub fn id_moc_hOBMO () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-shadowingAgreement OBJECT IDENTIFIER ::= {id-moc 7}
/// ```
///
///
#[inline]
pub fn id_moc_shadowingAgreement () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-ULconnEnd OBJECT IDENTIFIER ::= {id-moc 8}
/// ```
///
///
#[inline]
pub fn id_moc_ULconnEnd () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-disManagedObject OBJECT IDENTIFIER ::= {id-moc 9}
/// ```
///
///
#[inline]
pub fn id_moc_disManagedObject () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dirCust OBJECT IDENTIFIER ::= {id-moc 10}
/// ```
///
///
#[inline]
pub fn id_moc_dirCust () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dirUser OBJECT IDENTIFIER ::= {id-moc 11}
/// ```
///
///
#[inline]
pub fn id_moc_dirUser () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dMD OBJECT IDENTIFIER ::= {id-moc 12}
/// ```
///
///
#[inline]
pub fn id_moc_dMD () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 2, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-dsa-name-binding OBJECT IDENTIFIER ::= {id-mnb 0}
/// ```
///
///
#[inline]
pub fn id_mnb_dsa_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-dse-name-binding OBJECT IDENTIFIER ::= {id-mnb 1}
/// ```
///
///
#[inline]
pub fn id_mnb_dse_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-knownDSA-dSA-name-binding OBJECT IDENTIFIER ::= {id-mnb 2}
/// ```
///
///
#[inline]
pub fn id_mnb_knownDSA_dSA_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-knownDUA-dSA-name-binding OBJECT IDENTIFIER ::= {id-mnb 3}
/// ```
///
///
#[inline]
pub fn id_mnb_knownDUA_dSA_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-acseInvoc-knownDSA OBJECT IDENTIFIER ::= {id-mnb 4}
/// ```
///
///
#[inline]
pub fn id_mnb_acseInvoc_knownDSA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-acseInvoc-knownDUA OBJECT IDENTIFIER ::= {id-mnb 5}
/// ```
///
///
#[inline]
pub fn id_mnb_acseInvoc_knownDUA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-nHOB-name-binding OBJECT IDENTIFIER ::= {id-mnb 6}
/// ```
///
///
#[inline]
pub fn id_mnb_nHOB_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-hOB-name-binding OBJECT IDENTIFIER ::= {id-mnb 7}
/// ```
///
///
#[inline]
pub fn id_mnb_hOB_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-shadowingAgreement-nb OBJECT IDENTIFIER ::= {id-mnb 8}
/// ```
///
///
#[inline]
pub fn id_mnb_shadowingAgreement_nb () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-ULconnEnd-knownDSA OBJECT IDENTIFIER ::= {id-mnb 9}
/// ```
///
///
#[inline]
pub fn id_mnb_ULconnEnd_knownDSA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-ULconnEnd-knownDUA OBJECT IDENTIFIER ::= {id-mnb 10}
/// ```
///
///
#[inline]
pub fn id_mnb_ULconnEnd_knownDUA () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-dis-Customer-name-binding OBJECT IDENTIFIER ::= {id-mnb 11}
/// ```
///
///
#[inline]
pub fn id_mnb_dis_Customer_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-knownDSA-dUA-name-binding OBJECT IDENTIFIER ::= {id-mnb 12}
/// ```
///
///
#[inline]
pub fn id_mnb_knownDSA_dUA_name_binding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-DirCust-DMD OBJECT IDENTIFIER ::= {id-mnb 13}
/// ```
///
///
#[inline]
pub fn id_mnb_DirCust_DMD () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-DirUser-DirCust OBJECT IDENTIFIER ::= {id-mnb 14}
/// ```
///
///
#[inline]
pub fn id_mnb_DirUser_DirCust () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 3, 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dsaPackage OBJECT IDENTIFIER ::= {id-mp 0}
/// ```
///
///
#[inline]
pub fn id_mp_dsaPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-readPackage OBJECT IDENTIFIER ::= {id-mp 1}
/// ```
///
///
#[inline]
pub fn id_mp_readPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-comparePackage OBJECT IDENTIFIER ::= {id-mp 2}
/// ```
///
///
#[inline]
pub fn id_mp_comparePackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-abandonPackage OBJECT IDENTIFIER ::= {id-mp 3}
/// ```
///
///
#[inline]
pub fn id_mp_abandonPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-listPackage OBJECT IDENTIFIER ::= {id-mp 4}
/// ```
///
///
#[inline]
pub fn id_mp_listPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-searchPackage OBJECT IDENTIFIER ::= {id-mp 5}
/// ```
///
///
#[inline]
pub fn id_mp_searchPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-addPackage OBJECT IDENTIFIER ::= {id-mp 6}
/// ```
///
///
#[inline]
pub fn id_mp_addPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-removePackage OBJECT IDENTIFIER ::= {id-mp 7}
/// ```
///
///
#[inline]
pub fn id_mp_removePackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-modifyPackage OBJECT IDENTIFIER ::= {id-mp 8}
/// ```
///
///
#[inline]
pub fn id_mp_modifyPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-modifyDNPackage OBJECT IDENTIFIER ::= {id-mp 9}
/// ```
///
///
#[inline]
pub fn id_mp_modifyDNPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedReadPackage OBJECT IDENTIFIER ::= {id-mp 10}
/// ```
///
///
#[inline]
pub fn id_mp_chainedReadPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedComparePackage OBJECT IDENTIFIER ::= {id-mp 11}
/// ```
///
///
#[inline]
pub fn id_mp_chainedComparePackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedAbandonPackage OBJECT IDENTIFIER ::= {id-mp 12}
/// ```
///
///
#[inline]
pub fn id_mp_chainedAbandonPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedListPackage OBJECT IDENTIFIER ::= {id-mp 13}
/// ```
///
///
#[inline]
pub fn id_mp_chainedListPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedSearchPackage OBJECT IDENTIFIER ::= {id-mp 14}
/// ```
///
///
#[inline]
pub fn id_mp_chainedSearchPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedAddPackage OBJECT IDENTIFIER ::= {id-mp 15}
/// ```
///
///
#[inline]
pub fn id_mp_chainedAddPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedRemovePackage OBJECT IDENTIFIER ::= {id-mp 16}
/// ```
///
///
#[inline]
pub fn id_mp_chainedRemovePackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedModifyPackage OBJECT IDENTIFIER ::= {id-mp 17}
/// ```
///
///
#[inline]
pub fn id_mp_chainedModifyPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedModifyDNPackage OBJECT IDENTIFIER ::= {id-mp 18}
/// ```
///
///
#[inline]
pub fn id_mp_chainedModifyDNPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 18 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dsePackage OBJECT IDENTIFIER ::= {id-mp 19}
/// ```
///
///
#[inline]
pub fn id_mp_dsePackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 19 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-knownDSAPackage OBJECT IDENTIFIER ::= {id-mp 20}
/// ```
///
///
#[inline]
pub fn id_mp_knownDSAPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 20 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-knownDUAPackage OBJECT IDENTIFIER ::= {id-mp 21}
/// ```
///
///
#[inline]
pub fn id_mp_knownDUAPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 21 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dUAPackage OBJECT IDENTIFIER ::= {id-mp 22}
/// ```
///
///
#[inline]
pub fn id_mp_dUAPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 22 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-nHOBPackage OBJECT IDENTIFIER ::= {id-mp 23}
/// ```
///
///
#[inline]
pub fn id_mp_nHOBPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 23 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-hOBPackage OBJECT IDENTIFIER ::= {id-mp 24}
/// ```
///
///
#[inline]
pub fn id_mp_hOBPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 24 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-shadowingAgreementPackage OBJECT IDENTIFIER ::= {id-mp 25}
/// ```
///
///
#[inline]
pub fn id_mp_shadowingAgreementPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 25 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-ULconnEndPackage OBJECT IDENTIFIER ::= {id-mp 26}
/// ```
///
///
#[inline]
pub fn id_mp_ULconnEndPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 26 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-disPackage OBJECT IDENTIFIER ::= {id-mp 27}
/// ```
///
///
#[inline]
pub fn id_mp_disPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 27 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dcsPackage OBJECT IDENTIFIER ::= {id-mp 28}
/// ```
///
///
#[inline]
pub fn id_mp_dcsPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 28 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dirCust OBJECT IDENTIFIER ::= {id-mp 29}
/// ```
///
///
#[inline]
pub fn id_mp_dirCust () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 29 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dirUser OBJECT IDENTIFIER ::= {id-mp 30}
/// ```
///
///
#[inline]
pub fn id_mp_dirUser () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 30 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dMD OBJECT IDENTIFIER ::= {id-mp 31}
/// ```
///
///
#[inline]
pub fn id_mp_dMD () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 31 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dsPackage OBJECT IDENTIFIER ::= {id-mp 32}
/// ```
///
///
#[inline]
pub fn id_mp_dsPackage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 4, 32 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-nameProblem OBJECT IDENTIFIER ::= {id-mpa 1}
/// ```
///
///
#[inline]
pub fn id_mpa_nameProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-traceInformation OBJECT IDENTIFIER ::= {id-mpa 2}
/// ```
///
///
#[inline]
pub fn id_mpa_traceInformation () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-serviceProblem OBJECT IDENTIFIER ::= {id-mpa 3}
/// ```
///
///
#[inline]
pub fn id_mpa_serviceProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-entryName OBJECT IDENTIFIER ::= {id-mpa 4}
/// ```
///
///
#[inline]
pub fn id_mpa_entryName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-operation OBJECT IDENTIFIER ::= {id-mpa 5}
/// ```
///
///
#[inline]
pub fn id_mpa_operation () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-attributeProblem OBJECT IDENTIFIER ::= {id-mpa 6}
/// ```
///
///
#[inline]
pub fn id_mpa_attributeProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-attributeType OBJECT IDENTIFIER ::= {id-mpa 7}
/// ```
///
///
#[inline]
pub fn id_mpa_attributeType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-shadowProblem OBJECT IDENTIFIER ::= {id-mpa 8}
/// ```
///
///
#[inline]
pub fn id_mpa_shadowProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-attributeValue OBJECT IDENTIFIER ::= {id-mpa 9}
/// ```
///
///
#[inline]
pub fn id_mpa_attributeValue () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-resource OBJECT IDENTIFIER ::= {id-mpa 10}
/// ```
///
///
#[inline]
pub fn id_mpa_resource () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-authenReason OBJECT IDENTIFIER ::= {id-mpa 11}
/// ```
///
///
#[inline]
pub fn id_mpa_authenReason () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-updateProblem OBJECT IDENTIFIER ::= {id-mpa 12}
/// ```
///
///
#[inline]
pub fn id_mpa_updateProblem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-extensions OBJECT IDENTIFIER ::= {id-mpa 15}
/// ```
///
///
#[inline]
pub fn id_mpa_extensions () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-aliasedRDNs OBJECT IDENTIFIER ::= {id-mpa 16}
/// ```
///
///
#[inline]
pub fn id_mpa_aliasedRDNs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-aliasDereferenced OBJECT IDENTIFIER ::= {id-mpa 17}
/// ```
///
///
#[inline]
pub fn id_mpa_aliasDereferenced () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-referenceType OBJECT IDENTIFIER ::= {id-mpa 18}
/// ```
///
///
#[inline]
pub fn id_mpa_referenceType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 18 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-operationProgress OBJECT IDENTIFIER ::= {id-mpa 19}
/// ```
///
///
#[inline]
pub fn id_mpa_operationProgress () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 19 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-pDU OBJECT IDENTIFIER ::= {id-mpa 20}
/// ```
///
///
#[inline]
pub fn id_mpa_pDU () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 20 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-opId OBJECT IDENTIFIER ::= {id-mpa 21}
/// ```
///
///
#[inline]
pub fn id_mpa_opId () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 21 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-nhob-bind-id OBJECT IDENTIFIER ::= {id-mpa 22}
/// ```
///
///
#[inline]
pub fn id_mpa_nhob_bind_id () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 22 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-mhob-dop-prob OBJECT IDENTIFIER ::= {id-mpa 23}
/// ```
///
///
#[inline]
pub fn id_mpa_mhob_dop_prob () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 23 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-hob-bind-id OBJECT IDENTIFIER ::= {id-mpa 24}
/// ```
///
///
#[inline]
pub fn id_mpa_hob_bind_id () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 24 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-hob-dop-prob OBJECT IDENTIFIER ::= {id-mpa 25}
/// ```
///
///
#[inline]
pub fn id_mpa_hob_dop_prob () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 25 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-shadowing-dop-prob OBJECT IDENTIFIER ::= {id-mpa 26}
/// ```
///
///
#[inline]
pub fn id_mpa_shadowing_dop_prob () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 26 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-opIdDN OBJECT IDENTIFIER ::= {id-mpa 27}
/// ```
///
///
#[inline]
pub fn id_mpa_opIdDN () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 30, 5, 27 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryInformationServiceElement-operationType ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type DirectoryInformationServiceElement_operationType = BIT_STRING;

pub const DirectoryInformationServiceElement_operationType_read: BIT_INDEX = 0; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_compare: BIT_INDEX = 1; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_abandon: BIT_INDEX = 2; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_list: BIT_INDEX = 3; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_search: BIT_INDEX = 4; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_addEntry: BIT_INDEX = 5; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_removeEntry: BIT_INDEX = 6; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_modifyEntry: BIT_INDEX = 7; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_modifyDN: BIT_INDEX = 8; /* LONG_NAMED_BIT */

pub fn _decode_DirectoryInformationServiceElement_operationType(
    el: &X690Element,
) -> ASN1Result<DirectoryInformationServiceElement_operationType> {
    BER.decode_bit_string(&el)
}

pub fn _encode_DirectoryInformationServiceElement_operationType(
    value_: &DirectoryInformationServiceElement_operationType,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_DirectoryInformationServiceElement_operationType(
    el: &X690Element,
) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubSchemaSyntax-Item-subSchema ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct SubSchemaSyntax_Item_subSchema {
    pub structureRules: OPTIONAL<Vec<DITStructureRuleDescription>>,
    pub contentRules: OPTIONAL<Vec<DITContentRuleDescription>>,
    pub matchingRules: OPTIONAL<Vec<MatchingRuleDescription>>,
    pub attributeTypes: OPTIONAL<Vec<AttributeTypeDescription>>,
    pub objectClasses: OPTIONAL<Vec<ObjectClassDescription>>,
    pub nameForms: OPTIONAL<Vec<NameFormDescription>>,
    pub matchRuleUses: OPTIONAL<Vec<MatchingRuleUseDescription>>,
}
impl SubSchemaSyntax_Item_subSchema {
    pub fn new(
        structureRules: OPTIONAL<Vec<DITStructureRuleDescription>>,
        contentRules: OPTIONAL<Vec<DITContentRuleDescription>>,
        matchingRules: OPTIONAL<Vec<MatchingRuleDescription>>,
        attributeTypes: OPTIONAL<Vec<AttributeTypeDescription>>,
        objectClasses: OPTIONAL<Vec<ObjectClassDescription>>,
        nameForms: OPTIONAL<Vec<NameFormDescription>>,
        matchRuleUses: OPTIONAL<Vec<MatchingRuleUseDescription>>,
    ) -> Self {
        SubSchemaSyntax_Item_subSchema {
            structureRules,
            contentRules,
            matchingRules,
            attributeTypes,
            objectClasses,
            nameForms,
            matchRuleUses,
        }
    }
}
impl Default for SubSchemaSyntax_Item_subSchema {
    fn default() -> Self {
        SubSchemaSyntax_Item_subSchema {
            structureRules: None,
            contentRules: None,
            matchingRules: None,
            attributeTypes: None,
            objectClasses: None,
            nameForms: None,
            matchRuleUses: None,
        }
    }
}
impl TryFrom<&X690Element> for SubSchemaSyntax_Item_subSchema {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SubSchemaSyntax_Item_subSchema(el)
    }
}

pub const _rctl1_components_for_SubSchemaSyntax_Item_subSchema: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "structureRules",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contentRules",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchingRules",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeTypes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectClasses",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameForms",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchRuleUses",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubSchemaSyntax_Item_subSchema: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubSchemaSyntax_Item_subSchema: &[ComponentSpec; 0] = &[];

pub fn _decode_SubSchemaSyntax_Item_subSchema(
    el: &X690Element,
) -> ASN1Result<SubSchemaSyntax_Item_subSchema> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SubSchemaSyntax-Item-subSchema",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubSchemaSyntax_Item_subSchema,
        _eal_components_for_SubSchemaSyntax_Item_subSchema,
        _rctl2_components_for_SubSchemaSyntax_Item_subSchema,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut structureRules_: OPTIONAL<Vec<DITStructureRuleDescription>> = None;
    let mut contentRules_: OPTIONAL<Vec<DITContentRuleDescription>> = None;
    let mut matchingRules_: OPTIONAL<Vec<MatchingRuleDescription>> = None;
    let mut attributeTypes_: OPTIONAL<Vec<AttributeTypeDescription>> = None;
    let mut objectClasses_: OPTIONAL<Vec<ObjectClassDescription>> = None;
    let mut nameForms_: OPTIONAL<Vec<NameFormDescription>> = None;
    let mut matchRuleUses_: OPTIONAL<Vec<MatchingRuleUseDescription>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "structureRules" => {
                structureRules_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<DITStructureRuleDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<
                            SEQUENCE_OF<DITStructureRuleDescription>,
                        > {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "structureRules",
                                    ))
                                }
                            };
                            let mut items: SEQUENCE_OF<DITStructureRuleDescription> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_DITStructureRuleDescription(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?)
                    }(_el)?,
                )
            }
            "contentRules" => {
                contentRules_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<DITContentRuleDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<DITContentRuleDescription>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contentRules")),
	};
	let mut items: SEQUENCE_OF<DITContentRuleDescription> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_DITContentRuleDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(_el)?,
                )
            }
            "matchingRules" => {
                matchingRules_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<MatchingRuleDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MatchingRuleDescription>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchingRules")),
	};
	let mut items: SEQUENCE_OF<MatchingRuleDescription> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_MatchingRuleDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(_el)?,
                )
            }
            "attributeTypes" => {
                attributeTypes_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<AttributeTypeDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeTypeDescription>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeTypes")),
	};
	let mut items: SEQUENCE_OF<AttributeTypeDescription> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_AttributeTypeDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(_el)?,
                )
            }
            "objectClasses" => {
                objectClasses_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<ObjectClassDescription>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ObjectClassDescription>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "objectClasses",
                                        ))
                                    }
                                };
                                let mut items: SEQUENCE_OF<ObjectClassDescription> =
                                    Vec::with_capacity(elements.len());
                                for el in elements.iter() {
                                    items.push(_decode_ObjectClassDescription(el)?);
                                }
                                Ok(items)
                            }(&el.inner()?)?,
                        )
                    }(_el)?,
                )
            }
            "nameForms" => {
                nameForms_ = Some(|el: &X690Element| -> ASN1Result<Vec<NameFormDescription>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<NameFormDescription>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "nameForms",
                                    ))
                                }
                            };
                            let mut items: SEQUENCE_OF<NameFormDescription> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_NameFormDescription(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            "matchRuleUses" => {
                matchRuleUses_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<MatchingRuleUseDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MatchingRuleUseDescription>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchRuleUses")),
	};
	let mut items: SEQUENCE_OF<MatchingRuleUseDescription> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_MatchingRuleUseDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(_el)?,
                )
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "SubSchemaSyntax-Item-subSchema",
                ))
            }
        }
    }
    Ok(SubSchemaSyntax_Item_subSchema {
        structureRules: structureRules_,
        contentRules: contentRules_,
        matchingRules: matchingRules_,
        attributeTypes: attributeTypes_,
        objectClasses: objectClasses_,
        nameForms: nameForms_,
        matchRuleUses: matchRuleUses_,
    })
}

pub fn _encode_SubSchemaSyntax_Item_subSchema(
    value_: &SubSchemaSyntax_Item_subSchema,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.structureRules {
        components_.push(
            |v_1: &Vec<DITStructureRuleDescription>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(|value_: &SEQUENCE_OF<
                        DITStructureRuleDescription,
                    >|
                     -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_DITStructureRuleDescription(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.contentRules {
        components_.push(
            |v_1: &Vec<DITContentRuleDescription>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(|value_: &SEQUENCE_OF<
                        DITContentRuleDescription,
                    >|
                     -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_DITContentRuleDescription(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.matchingRules {
        components_.push(|v_1: &Vec<MatchingRuleDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 3), X690Value::from_explicit(|value_: &SEQUENCE_OF<MatchingRuleDescription>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_MatchingRuleDescription(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?))) }(&v_)?);
    }
    if let Some(v_) = &value_.attributeTypes {
        components_.push(|v_1: &Vec<AttributeTypeDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 4), X690Value::from_explicit(|value_: &SEQUENCE_OF<AttributeTypeDescription>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AttributeTypeDescription(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?))) }(&v_)?);
    }
    if let Some(v_) = &value_.objectClasses {
        components_.push(|v_1: &Vec<ObjectClassDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 5), X690Value::from_explicit(|value_: &SEQUENCE_OF<ObjectClassDescription>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_ObjectClassDescription(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?))) }(&v_)?);
    }
    if let Some(v_) = &value_.nameForms {
        components_.push(
            |v_1: &Vec<NameFormDescription>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 6),
                    X690Value::from_explicit(
                        |value_: &SEQUENCE_OF<NameFormDescription>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_NameFormDescription(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(
                                    TagClass::UNIVERSAL,
                                    UNIV_TAG_SEQUENCE_OF,
                                ),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.matchRuleUses {
        components_.push(
            |v_1: &Vec<MatchingRuleUseDescription>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 7),
                    X690Value::from_explicit(|value_: &SEQUENCE_OF<
                        MatchingRuleUseDescription,
                    >|
                     -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MatchingRuleUseDescription(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SubSchemaSyntax_Item_subSchema(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SubSchemaSyntax-Item-subSchema",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubSchemaSyntax_Item_subSchema,
        _eal_components_for_SubSchemaSyntax_Item_subSchema,
        _rctl2_components_for_SubSchemaSyntax_Item_subSchema,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "structureRules" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "structureRules")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_DITStructureRuleDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "structureRules",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "contentRules" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contentRules")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_DITContentRuleDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "contentRules",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "matchingRules" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchingRules")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MatchingRuleDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "matchingRules",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "attributeTypes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeTypes")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_AttributeTypeDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "attributeTypes",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "objectClasses" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "objectClasses")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ObjectClassDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "objectClasses",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "nameForms" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nameForms")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_NameFormDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nameForms")
                        ),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "matchRuleUses" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchRuleUses")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MatchingRuleUseDescription(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "matchRuleUses",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "SubSchemaSyntax-Item-subSchema",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubSchemaSyntax-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct SubSchemaSyntax_Item {
    pub name: Name,
    pub subSchema: SubSchemaSyntax_Item_subSchema,
}
impl SubSchemaSyntax_Item {
    pub fn new(name: Name, subSchema: SubSchemaSyntax_Item_subSchema) -> Self {
        SubSchemaSyntax_Item { name, subSchema }
    }
}
impl TryFrom<&X690Element> for SubSchemaSyntax_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SubSchemaSyntax_Item(el)
    }
}

pub const _rctl1_components_for_SubSchemaSyntax_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subSchema",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubSchemaSyntax_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubSchemaSyntax_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_SubSchemaSyntax_Item(el: &X690Element) -> ASN1Result<SubSchemaSyntax_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubSchemaSyntax-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubSchemaSyntax_Item,
        _eal_components_for_SubSchemaSyntax_Item,
        _rctl2_components_for_SubSchemaSyntax_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut name_: OPTIONAL<Name> = None;
    let mut subSchema_: OPTIONAL<SubSchemaSyntax_Item_subSchema> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "name" => {
                name_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "subSchema" => {
                subSchema_ = Some(
                    |el: &X690Element| -> ASN1Result<SubSchemaSyntax_Item_subSchema> {
                        Ok(_decode_SubSchemaSyntax_Item_subSchema(&el.inner()?)?)
                    }(_el)?,
                )
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "SubSchemaSyntax-Item",
                ))
            }
        }
    }
    Ok(SubSchemaSyntax_Item {
        name: name_.unwrap(),
        subSchema: subSchema_.unwrap(),
    })
}

pub fn _encode_SubSchemaSyntax_Item(value_: &SubSchemaSyntax_Item) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(_encode_Name(&v_1)?),
        ))
    }(&value_.name)?);
    components_.push(
        |v_1: &SubSchemaSyntax_Item_subSchema| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(_encode_SubSchemaSyntax_Item_subSchema(&v_1)?),
            ))
        }(&value_.subSchema)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SubSchemaSyntax_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubSchemaSyntax-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubSchemaSyntax_Item,
        _eal_components_for_SubSchemaSyntax_Item,
        _rctl2_components_for_SubSchemaSyntax_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "name"));
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "subSchema" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subSchema")
                    );
                }
                Ok(_validate_SubSchemaSyntax_Item_subSchema(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "SubSchemaSyntax-Item",
                ))
            }
        }
    }
    Ok(())
}
