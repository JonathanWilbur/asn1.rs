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
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// Accessors  ::=  SET OF Name
/// ```
pub type Accessors = Vec<Name>; // SetOfType

pub fn _decode_Accessors(el: &X690Element) -> ASN1Result<Accessors> {
    |el: &X690Element| -> ASN1Result<SET_OF<Name>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<Name> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Name(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Accessors(value_: &Accessors) -> ASN1Result<X690Element> {
    |value_: &SET_OF<Name>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Name(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AdministrativeRole  ::=  OBJECT-CLASS.&id
/// ```
pub type AdministrativeRole = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_AdministrativeRole(el: &X690Element) -> ASN1Result<AdministrativeRole> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_AdministrativeRole(value_: &AdministrativeRole) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ApplicationContext  ::=  OBJECT IDENTIFIER
/// ```
pub type ApplicationContext = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_ApplicationContext(el: &X690Element) -> ASN1Result<ApplicationContext> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_ApplicationContext(value_: &ApplicationContext) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AssociationEstablishment  ::=  BIT STRING {inward(0), outward(1)}
/// ```
pub type AssociationEstablishment = BIT_STRING;

pub const AssociationEstablishment_inward: BIT = 0; /* LONG_NAMED_BIT */

pub const AssociationEstablishment_outward: BIT = 1; /* LONG_NAMED_BIT */

pub fn _decode_AssociationEstablishment(el: &X690Element) -> ASN1Result<AssociationEstablishment> {
    ber_decode_bit_string(&el)
}

pub fn _encode_AssociationEstablishment(
    value_: &AssociationEstablishment,
) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AssociationId  ::=  INTEGER
/// ```
pub type AssociationId = INTEGER;

pub fn _decode_AssociationId(el: &X690Element) -> ASN1Result<AssociationId> {
    ber_decode_integer(&el)
}

pub fn _encode_AssociationId(value_: &AssociationId) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenReasonSyntax  ::=  INTEGER {
///   unknownUser(0), incorrectPassword(1), inaccessiblePassword(2),
///   passwordVerificationLoop(3), unrecognizedUser(4)}
/// ```
pub type AuthenReasonSyntax = INTEGER;

pub const AuthenReasonSyntax_unknownUser: AuthenReasonSyntax = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_incorrectPassword: AuthenReasonSyntax = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_inaccessiblePassword: AuthenReasonSyntax = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_passwordVerificationLoop: AuthenReasonSyntax = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const AuthenReasonSyntax_unrecognizedUser: AuthenReasonSyntax = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AuthenReasonSyntax(el: &X690Element) -> ASN1Result<AuthenReasonSyntax> {
    ber_decode_integer(&el)
}

pub fn _encode_AuthenReasonSyntax(value_: &AuthenReasonSyntax) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
impl TryFrom<X690Element> for DirectoryInformationServiceElement {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryInformationServiceElement(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DirectoryInformationServiceElement {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<DirectoryInformationServiceElement> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DirectoryInformationServiceElement,
            _eal_components_for_DirectoryInformationServiceElement,
            _rctl2_components_for_DirectoryInformationServiceElement,
        )?;
        let operationType: OPTIONAL<DirectoryInformationServiceElement_operationType> =
            match _components.get("operationType") {
                Some(c_) => Some(_decode_DirectoryInformationServiceElement_operationType(
                    c_,
                )?),
                _ => None,
            };
        let attributeType: OPTIONAL<AttributeType> = match _components.get("attributeType") {
            Some(c_) => Some(_decode_AttributeType(c_)?),
            _ => None,
        };
        let attributeValue: OPTIONAL<AttributeValue> = match _components.get("attributeValue") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AttributeValue> {
                Ok(_decode_AttributeValue(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(DirectoryInformationServiceElement {
            operationType,
            attributeType,
            attributeValue,
        })
    }(&el)
}

pub fn _encode_DirectoryInformationServiceElement(
    value_: &DirectoryInformationServiceElement,
) -> ASN1Result<X690Element> {
    |value_: &DirectoryInformationServiceElement| -> ASN1Result<X690Element> {
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
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AttributeValue(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSAScopeOfChainingValue  ::=  INTEGER {dmd(0), country(1), global(2)}
/// ```
pub type DSAScopeOfChainingValue = INTEGER;

pub const DSAScopeOfChainingValue_dmd: DSAScopeOfChainingValue = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfChainingValue_country: DSAScopeOfChainingValue = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfChainingValue_global: DSAScopeOfChainingValue = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_DSAScopeOfChainingValue(el: &X690Element) -> ASN1Result<DSAScopeOfChainingValue> {
    ber_decode_integer(&el)
}

pub fn _encode_DSAScopeOfChainingValue(
    value_: &DSAScopeOfChainingValue,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSAScopeOfReferralValue  ::=  INTEGER {dmd(0), country(1), global(2)}
/// ```
pub type DSAScopeOfReferralValue = INTEGER;

pub const DSAScopeOfReferralValue_dmd: DSAScopeOfReferralValue = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfReferralValue_country: DSAScopeOfReferralValue = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const DSAScopeOfReferralValue_global: DSAScopeOfReferralValue = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_DSAScopeOfReferralValue(el: &X690Element) -> ASN1Result<DSAScopeOfReferralValue> {
    ber_decode_integer(&el)
}

pub fn _encode_DSAScopeOfReferralValue(
    value_: &DSAScopeOfReferralValue,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HOBRole  ::=  INTEGER {superior(0), subordinate(1)}
/// ```
pub type HOBRole = INTEGER;

pub const HOBRole_superior: HOBRole = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const HOBRole_subordinate: HOBRole = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_HOBRole(el: &X690Element) -> ASN1Result<HOBRole> {
    ber_decode_integer(&el)
}

pub fn _encode_HOBRole(value_: &HOBRole) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtBitString  ::=  BIT STRING
/// ```
pub type MgtBitString = BIT_STRING;

pub fn _decode_MgtBitString(el: &X690Element) -> ASN1Result<MgtBitString> {
    ber_decode_bit_string(&el)
}

pub fn _encode_MgtBitString(value_: &MgtBitString) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtBoolean  ::=  BOOLEAN
/// ```
pub type MgtBoolean = BOOLEAN; // BooleanType

pub fn _decode_MgtBoolean(el: &X690Element) -> ASN1Result<MgtBoolean> {
    ber_decode_boolean(&el)
}

pub fn _encode_MgtBoolean(value_: &MgtBoolean) -> ASN1Result<X690Element> {
    ber_encode_boolean(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtGeneralizedTime  ::=  GeneralizedTime
/// ```
pub type MgtGeneralizedTime = GeneralizedTime; // GeneralizedTime

pub fn _decode_MgtGeneralizedTime(el: &X690Element) -> ASN1Result<MgtGeneralizedTime> {
    ber_decode_generalized_time(&el)
}

pub fn _encode_MgtGeneralizedTime(value_: &MgtGeneralizedTime) -> ASN1Result<X690Element> {
    ber_encode_generalized_time(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtInteger  ::=  INTEGER
/// ```
pub type MgtInteger = INTEGER;

pub fn _decode_MgtInteger(el: &X690Element) -> ASN1Result<MgtInteger> {
    ber_decode_integer(&el)
}

pub fn _encode_MgtInteger(value_: &MgtInteger) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtOctetString  ::=  OCTET STRING
/// ```
pub type MgtOctetString = OCTET_STRING; // OctetStringType

pub fn _decode_MgtOctetString(el: &X690Element) -> ASN1Result<MgtOctetString> {
    ber_decode_octet_string(&el)
}

pub fn _encode_MgtOctetString(value_: &MgtOctetString) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtOID  ::=  OBJECT IDENTIFIER
/// ```
pub type MgtOID = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_MgtOID(el: &X690Element) -> ASN1Result<MgtOID> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_MgtOID(value_: &MgtOID) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MgtPrintableString  ::=  PrintableString
/// ```
pub type MgtPrintableString = PrintableString; // PrintableString

pub fn _decode_MgtPrintableString(el: &X690Element) -> ASN1Result<MgtPrintableString> {
    ber_decode_printable_string(&el)
}

pub fn _encode_MgtPrintableString(value_: &MgtPrintableString) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PeerEntityAuthenticationPolicy  ::=  BIT STRING {
///   none(0), nameOnly(1), simpleUnprotected(2), simpleProtected(3), strong(4),
///   external(5)}
/// ```
pub type PeerEntityAuthenticationPolicy = BIT_STRING;

pub const PeerEntityAuthenticationPolicy_none: BIT = 0; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_nameOnly: BIT = 1; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_simpleUnprotected: BIT = 2; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_simpleProtected: BIT = 3; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_strong: BIT = 4; /* LONG_NAMED_BIT */

pub const PeerEntityAuthenticationPolicy_external: BIT = 5; /* LONG_NAMED_BIT */

pub fn _decode_PeerEntityAuthenticationPolicy(
    el: &X690Element,
) -> ASN1Result<PeerEntityAuthenticationPolicy> {
    ber_decode_bit_string(&el)
}

pub fn _encode_PeerEntityAuthenticationPolicy(
    value_: &PeerEntityAuthenticationPolicy,
) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RemoteDSAList  ::=  SET OF AccessPoint
/// ```
pub type RemoteDSAList = Vec<AccessPoint>; // SetOfType

pub fn _decode_RemoteDSAList(el: &X690Element) -> ASN1Result<RemoteDSAList> {
    |el: &X690Element| -> ASN1Result<SET_OF<AccessPoint>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<AccessPoint> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AccessPoint(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RemoteDSAList(value_: &RemoteDSAList) -> ASN1Result<X690Element> {
    |value_: &SET_OF<AccessPoint>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AccessPoint(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestAuthenticationPolicy  ::=  BIT STRING {none(0), simpleName(1), strong(2)}
/// ```
pub type RequestAuthenticationPolicy = BIT_STRING;

pub const RequestAuthenticationPolicy_none: BIT = 0; /* LONG_NAMED_BIT */

pub const RequestAuthenticationPolicy_simpleName: BIT = 1; /* LONG_NAMED_BIT */

pub const RequestAuthenticationPolicy_strong: BIT = 2; /* LONG_NAMED_BIT */

pub fn _decode_RequestAuthenticationPolicy(
    el: &X690Element,
) -> ASN1Result<RequestAuthenticationPolicy> {
    ber_decode_bit_string(&el)
}

pub fn _encode_RequestAuthenticationPolicy(
    value_: &RequestAuthenticationPolicy,
) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ResourceSyntax  ::=  INTEGER {
///   insufficientMemory(0), insufficientAssociations(1), insufficientDiskSpace(2),
///   miscellaneousResourceExhausted(4)}
/// ```
pub type ResourceSyntax = INTEGER;

pub const ResourceSyntax_insufficientMemory: ResourceSyntax = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ResourceSyntax_insufficientAssociations: ResourceSyntax = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ResourceSyntax_insufficientDiskSpace: ResourceSyntax = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const ResourceSyntax_miscellaneousResourceExhausted: ResourceSyntax = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ResourceSyntax(el: &X690Element) -> ASN1Result<ResourceSyntax> {
    ber_decode_integer(&el)
}

pub fn _encode_ResourceSyntax(value_: &ResourceSyntax) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecondaryShadows  ::=  SET OF SupplierAndConsumers
/// ```
pub type SecondaryShadows = Vec<SupplierAndConsumers>; // SetOfType

pub fn _decode_SecondaryShadows(el: &X690Element) -> ASN1Result<SecondaryShadows> {
    |el: &X690Element| -> ASN1Result<SET_OF<SupplierAndConsumers>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<SupplierAndConsumers> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_SupplierAndConsumers(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SecondaryShadows(value_: &SecondaryShadows) -> ASN1Result<X690Element> {
    |value_: &SET_OF<SupplierAndConsumers>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_SupplierAndConsumers(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowingRole  ::=  INTEGER {supplier(0), consumer(1)}
/// ```
pub type ShadowingRole = INTEGER;

pub const ShadowingRole_supplier: ShadowingRole = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowingRole_consumer: ShadowingRole = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ShadowingRole(el: &X690Element) -> ASN1Result<ShadowingRole> {
    ber_decode_integer(&el)
}

pub fn _encode_ShadowingRole(value_: &ShadowingRole) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<SubSchemaSyntax_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<SubSchemaSyntax_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_SubSchemaSyntax_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SubSchemaSyntax(value_: &SubSchemaSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<SubSchemaSyntax_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_SubSchemaSyntax_Item(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
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
    |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(ber_decode_object_identifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SupportedApplicationContexts(
    value_: &SupportedApplicationContexts,
) -> ASN1Result<X690Element> {
    |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(ber_encode_object_identifier(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// zero INTEGER ::= 0
/// ```
///
///
pub const zero: INTEGER = 0;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac OBJECT IDENTIFIER ::= {id-mgt 0}
/// ```
///
///
pub fn id_mac() -> OBJECT_IDENTIFIER {
    [id_mgt(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat OBJECT IDENTIFIER ::= {id-mgt 1}
/// ```
///
///
pub fn id_mat() -> OBJECT_IDENTIFIER {
    [id_mgt(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc OBJECT IDENTIFIER ::= {id-mgt 2}
/// ```
///
///
pub fn id_moc() -> OBJECT_IDENTIFIER {
    [id_mgt(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb OBJECT IDENTIFIER ::= {id-mgt 3}
/// ```
///
///
pub fn id_mnb() -> OBJECT_IDENTIFIER {
    [id_mgt(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp OBJECT IDENTIFIER ::= {id-mgt 4}
/// ```
///
///
pub fn id_mp() -> OBJECT_IDENTIFIER {
    [id_mgt(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa OBJECT IDENTIFIER ::= {id-mgt 5}
/// ```
///
///
pub fn id_mpa() -> OBJECT_IDENTIFIER {
    [id_mgt(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac-useRemoteDSA OBJECT IDENTIFIER ::= {id-mac 0}
/// ```
///
///
pub fn id_mac_useRemoteDSA() -> OBJECT_IDENTIFIER {
    [id_mac(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac-useHomeDSA OBJECT IDENTIFIER ::= {id-mac 1}
/// ```
///
///
pub fn id_mac_useHomeDSA() -> OBJECT_IDENTIFIER {
    [id_mac(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mac-update OBJECT IDENTIFIER ::= {id-mac 2}
/// ```
///
///
pub fn id_mac_update() -> OBJECT_IDENTIFIER {
    [id_mac(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accessPoint OBJECT IDENTIFIER ::= {id-mat 0}
/// ```
///
///
pub fn id_mat_accessPoint() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-masterEntries OBJECT IDENTIFIER ::= {id-mat 1}
/// ```
///
///
pub fn id_mat_masterEntries() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-copyEntries OBJECT IDENTIFIER ::= {id-mat 2}
/// ```
///
///
pub fn id_mat_copyEntries() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-loopsDetected OBJECT IDENTIFIER ::= {id-mat 3}
/// ```
///
///
pub fn id_mat_loopsDetected() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-securityErrors OBJECT IDENTIFIER ::= {id-mat 4}
/// ```
///
///
pub fn id_mat_securityErrors() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nameErrors OBJECT IDENTIFIER ::= {id-mat 5}
/// ```
///
///
pub fn id_mat_nameErrors() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-foundLocalEntries OBJECT IDENTIFIER ::= {id-mat 6}
/// ```
///
///
pub fn id_mat_foundLocalEntries() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-referrals OBJECT IDENTIFIER ::= {id-mat 7}
/// ```
///
///
pub fn id_mat_referrals() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-serviceErrors OBJECT IDENTIFIER ::= {id-mat 8}
/// ```
///
///
pub fn id_mat_serviceErrors() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasDereferences OBJECT IDENTIFIER ::= {id-mat 9}
/// ```
///
///
pub fn id_mat_aliasDereferences() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chainings OBJECT IDENTIFIER ::= {id-mat 10}
/// ```
///
///
pub fn id_mat_chainings() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-invalidReferences OBJECT IDENTIFIER ::= {id-mat 11}
/// ```
///
///
pub fn id_mat_invalidReferences() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-unableToProceed OBJECT IDENTIFIER ::= {id-mat 12}
/// ```
///
///
pub fn id_mat_unableToProceed() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-outOfScope OBJECT IDENTIFIER ::= {id-mat 13}
/// ```
///
///
pub fn id_mat_outOfScope() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-noSuchObject OBJECT IDENTIFIER ::= {id-mat 14}
/// ```
///
///
pub fn id_mat_noSuchObject() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasProblem OBJECT IDENTIFIER ::= {id-mat 15}
/// ```
///
///
pub fn id_mat_aliasProblem() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasDereferencingProblem OBJECT IDENTIFIER ::= {id-mat 16}
/// ```
///
///
pub fn id_mat_aliasDereferencingProblem() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-affectsMultipleDSAs OBJECT IDENTIFIER ::= {id-mat 17}
/// ```
///
///
pub fn id_mat_affectsMultipleDSAs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-unavailableCriticalExtension OBJECT IDENTIFIER ::= {id-mat 18}
/// ```
///
///
pub fn id_mat_unavailableCriticalExtension() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeLimitExceeded OBJECT IDENTIFIER ::= {id-mat 19}
/// ```
///
///
pub fn id_mat_timeLimitExceeded() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-sizeLimitExceeded OBJECT IDENTIFIER ::= {id-mat 20}
/// ```
///
///
pub fn id_mat_sizeLimitExceeded() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-adminLimitExceeded OBJECT IDENTIFIER ::= {id-mat 21}
/// ```
///
///
pub fn id_mat_adminLimitExceeded() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-prohibitChaining OBJECT IDENTIFIER ::= {id-mat 24}
/// ```
///
///
pub fn id_mat_prohibitChaining() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-readOpsProc OBJECT IDENTIFIER ::= {id-mat 25}
/// ```
///
///
pub fn id_mat_readOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-compareOpsProc OBJECT IDENTIFIER ::= {id-mat 26}
/// ```
///
///
pub fn id_mat_compareOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-abandonOpsProc OBJECT IDENTIFIER ::= {id-mat 27}
/// ```
///
///
pub fn id_mat_abandonOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-listOpsProc OBJECT IDENTIFIER ::= {id-mat 28}
/// ```
///
///
pub fn id_mat_listOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-searchBaseOpsProc OBJECT IDENTIFIER ::= {id-mat 29}
/// ```
///
///
pub fn id_mat_searchBaseOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([29])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-search1LevelOpsProc OBJECT IDENTIFIER ::= {id-mat 30}
/// ```
///
///
pub fn id_mat_search1LevelOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-searchSubtreeOpsProc OBJECT IDENTIFIER ::= {id-mat 31}
/// ```
///
///
pub fn id_mat_searchSubtreeOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-addEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 32}
/// ```
///
///
pub fn id_mat_addEntryOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-removeEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 33}
/// ```
///
///
pub fn id_mat_removeEntryOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([33])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 34}
/// ```
///
///
pub fn id_mat_modifyEntryOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([34])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyDNOpsProc OBJECT IDENTIFIER ::= {id-mat 35}
/// ```
///
///
pub fn id_mat_modifyDNOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([35])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chReadOpsProc OBJECT IDENTIFIER ::= {id-mat 36}
/// ```
///
///
pub fn id_mat_chReadOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([36])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chCompareOpsProc OBJECT IDENTIFIER ::= {id-mat 37}
/// ```
///
///
pub fn id_mat_chCompareOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([37])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chAbandonOpsProc OBJECT IDENTIFIER ::= {id-mat 38}
/// ```
///
///
pub fn id_mat_chAbandonOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([38])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chListOpsProc OBJECT IDENTIFIER ::= {id-mat 39}
/// ```
///
///
pub fn id_mat_chListOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([39])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chSearchBaseOpsProc OBJECT IDENTIFIER ::= {id-mat 40}
/// ```
///
///
pub fn id_mat_chSearchBaseOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([40])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chSearch1LevelOpsProc OBJECT IDENTIFIER ::= {id-mat 41}
/// ```
///
///
pub fn id_mat_chSearch1LevelOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([41])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chSearchSubtreeOpsProc OBJECT IDENTIFIER ::= {id-mat 42}
/// ```
///
///
pub fn id_mat_chSearchSubtreeOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([42])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chAddEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 43}
/// ```
///
///
pub fn id_mat_chAddEntryOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([43])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chRemoveEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 44}
/// ```
///
///
pub fn id_mat_chRemoveEntryOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([44])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chModifyEntryOpsProc OBJECT IDENTIFIER ::= {id-mat 45}
/// ```
///
///
pub fn id_mat_chModifyEntryOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([45])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-chModifyDNOpsProc OBJECT IDENTIFIER ::= {id-mat 46}
/// ```
///
///
pub fn id_mat_chModifyDNOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([46])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSAScopeOfReferral OBJECT IDENTIFIER ::= {id-mat 47}
/// ```
///
///
pub fn id_mat_dSAScopeOfReferral() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([47])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSAScopeOfChaining OBJECT IDENTIFIER ::= {id-mat 48}
/// ```
///
///
pub fn id_mat_dSAScopeOfChaining() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([48])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-peerEntityAuthenticationPolicy OBJECT IDENTIFIER ::= {id-mat 49}
/// ```
///
///
pub fn id_mat_peerEntityAuthenticationPolicy() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([49])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-requestAuthenticationPolicy OBJECT IDENTIFIER ::= {id-mat 50}
/// ```
///
///
pub fn id_mat_requestAuthenticationPolicy() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([50])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-resultAuthenticationPolicy OBJECT IDENTIFIER ::= {id-mat 51}
/// ```
///
///
pub fn id_mat_resultAuthenticationPolicy() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([51])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSPAssociationEstablishment OBJECT IDENTIFIER ::= {id-mat 52}
/// ```
///
///
pub fn id_mat_dSPAssociationEstablishment() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([52])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dOPAssociationEstablishment OBJECT IDENTIFIER ::= {id-mat 53}
/// ```
///
///
pub fn id_mat_dOPAssociationEstablishment() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([53])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dISPAssociationEstablishment OBJECT IDENTIFIER ::= {id-mat 54}
/// ```
///
///
pub fn id_mat_dISPAssociationEstablishment() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([54])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDAPAssociations OBJECT IDENTIFIER ::= {id-mat 55}
/// ```
///
///
pub fn id_mat_maxDAPAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([55])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDSPAssociations OBJECT IDENTIFIER ::= {id-mat 56}
/// ```
///
///
pub fn id_mat_maxDSPAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([56])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDOPAssociations OBJECT IDENTIFIER ::= {id-mat 57}
/// ```
///
///
pub fn id_mat_maxDOPAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([57])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxDISPAssociations OBJECT IDENTIFIER ::= {id-mat 58}
/// ```
///
///
pub fn id_mat_maxDISPAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([58])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dAPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 59}
/// ```
///
///
pub fn id_mat_dAPAssociationTimeout() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([59])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 60}
/// ```
///
///
pub fn id_mat_dSPAssociationTimeout() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([60])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dOPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 61}
/// ```
///
///
pub fn id_mat_dOPAssociationTimeout() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([61])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dISPAssociationTimeout OBJECT IDENTIFIER ::= {id-mat 62}
/// ```
///
///
pub fn id_mat_dISPAssociationTimeout() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([62])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dSAActiveAssociations OBJECT IDENTIFIER ::= {id-mat 63}
/// ```
///
///
pub fn id_mat_dSAActiveAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([63])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-pagedResultsMaxIDs OBJECT IDENTIFIER ::= {id-mat 64}
/// ```
///
///
pub fn id_mat_pagedResultsMaxIDs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([64])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-pagedResultsTimer OBJECT IDENTIFIER ::= {id-mat 65}
/// ```
///
///
pub fn id_mat_pagedResultsTimer() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([65])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-homeDSA OBJECT IDENTIFIER ::= {id-mat 66}
/// ```
///
///
pub fn id_mat_homeDSA() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([66])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dUATimeout OBJECT IDENTIFIER ::= {id-mat 68}
/// ```
///
///
pub fn id_mat_dUATimeout() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([68])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-supportedApplicationContexts OBJECT IDENTIFIER ::= {id-mat 69}
/// ```
///
///
pub fn id_mat_supportedApplicationContexts() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([69])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-reverseCredentials OBJECT IDENTIFIER ::= {id-mat 70}
/// ```
///
///
pub fn id_mat_reverseCredentials() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([70])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-remoteAccessPoint OBJECT IDENTIFIER ::= {id-mat 71}
/// ```
///
///
pub fn id_mat_remoteAccessPoint() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([71])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxInboundAssociations OBJECT IDENTIFIER ::= {id-mat 72}
/// ```
///
///
pub fn id_mat_maxInboundAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([72])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxOutboundAssociations OBJECT IDENTIFIER ::= {id-mat 73}
/// ```
///
///
pub fn id_mat_maxOutboundAssociations() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([73])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-currentActiveAssocs OBJECT IDENTIFIER ::= {id-mat 74}
/// ```
///
///
pub fn id_mat_currentActiveAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([74])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-currentActiveInboundAssocs OBJECT IDENTIFIER ::= {id-mat 75}
/// ```
///
///
pub fn id_mat_currentActiveInboundAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([75])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-currentActiveOutboundAssocs OBJECT IDENTIFIER ::= {id-mat 76}
/// ```
///
///
pub fn id_mat_currentActiveOutboundAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([76])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumAssocs OBJECT IDENTIFIER ::= {id-mat 77}
/// ```
///
///
pub fn id_mat_accumAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([77])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumInboundAssocs OBJECT IDENTIFIER ::= {id-mat 78}
/// ```
///
///
pub fn id_mat_accumInboundAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([78])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumOutboundAssocs OBJECT IDENTIFIER ::= {id-mat 79}
/// ```
///
///
pub fn id_mat_accumOutboundAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([79])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumFailedInboundAssocs OBJECT IDENTIFIER ::= {id-mat 80}
/// ```
///
///
pub fn id_mat_accumFailedInboundAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([80])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accumFailedOutboundAssocs OBJECT IDENTIFIER ::= {id-mat 81}
/// ```
///
///
pub fn id_mat_accumFailedOutboundAssocs() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([81])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeOfLastAttempt OBJECT IDENTIFIER ::= {id-mat 82}
/// ```
///
///
pub fn id_mat_timeOfLastAttempt() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([82])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeOfLastSuccess OBJECT IDENTIFIER ::= {id-mat 83}
/// ```
///
///
pub fn id_mat_timeOfLastSuccess() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([83])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-requestCounter OBJECT IDENTIFIER ::= {id-mat 84}
/// ```
///
///
pub fn id_mat_requestCounter() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([84])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-replyCounter OBJECT IDENTIFIER ::= {id-mat 85}
/// ```
///
///
pub fn id_mat_replyCounter() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([85])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-requestsFailedCounter OBJECT IDENTIFIER ::= {id-mat 86}
/// ```
///
///
pub fn id_mat_requestsFailedCounter() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([86])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeOfLastAccess OBJECT IDENTIFIER ::= {id-mat 87}
/// ```
///
///
pub fn id_mat_timeOfLastAccess() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([87])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-agreementID OBJECT IDENTIFIER ::= {id-mat 88}
/// ```
///
///
pub fn id_mat_agreementID() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([88])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-agreementVersion OBJECT IDENTIFIER ::= {id-mat 89}
/// ```
///
///
pub fn id_mat_agreementVersion() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([89])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-hOBRole OBJECT IDENTIFIER ::= {id-mat 90}
/// ```
///
///
pub fn id_mat_hOBRole() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([90])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-shadowingSubject OBJECT IDENTIFIER ::= {id-mat 91}
/// ```
///
///
pub fn id_mat_shadowingSubject() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([91])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-updateMode OBJECT IDENTIFIER ::= {id-mat 92}
/// ```
///
///
pub fn id_mat_updateMode() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([92])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-masterAccessPoint OBJECT IDENTIFIER ::= {id-mat 93}
/// ```
///
///
pub fn id_mat_masterAccessPoint() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([93])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-secondaryShadows OBJECT IDENTIFIER ::= {id-mat 94}
/// ```
///
///
pub fn id_mat_secondaryShadows() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([94])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-shadowingRole OBJECT IDENTIFIER ::= {id-mat 95}
/// ```
///
///
pub fn id_mat_shadowingRole() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([95])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-lastUpdateTime OBJECT IDENTIFIER ::= {id-mat 96}
/// ```
///
///
pub fn id_mat_lastUpdateTime() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([96])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-shadowingSchedule OBJECT IDENTIFIER ::= {id-mat 97}
/// ```
///
///
pub fn id_mat_shadowingSchedule() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([97])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nextUpdateTime OBJECT IDENTIFIER ::= {id-mat 98}
/// ```
///
///
pub fn id_mat_nextUpdateTime() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([98])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-useDOP OBJECT IDENTIFIER ::= {id-mat 99}
/// ```
///
///
pub fn id_mat_useDOP() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([99])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accessor OBJECT IDENTIFIER ::= {id-mat 100}
/// ```
///
///
pub fn id_mat_accessor() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([100])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-allowedInfoService OBJECT IDENTIFIER ::= {id-mat 101}
/// ```
///
///
pub fn id_mat_allowedInfoService() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([101])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-applicationContextInUse OBJECT IDENTIFIER ::= {id-mat 102}
/// ```
///
///
pub fn id_mat_applicationContextInUse() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([102])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-associationId OBJECT IDENTIFIER ::= {id-mat 103}
/// ```
///
///
pub fn id_mat_associationId() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([103])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-callingAETitle OBJECT IDENTIFIER ::= {id-mat 104}
/// ```
///
///
pub fn id_mat_callingAETitle() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([104])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-disAllowedInfoService OBJECT IDENTIFIER ::= {id-mat 105}
/// ```
///
///
pub fn id_mat_disAllowedInfoService() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([105])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxEntriesReturned OBJECT IDENTIFIER ::= {id-mat 106}
/// ```
///
///
pub fn id_mat_maxEntriesReturned() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([106])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-maxTimeForResult OBJECT IDENTIFIER ::= {id-mat 107}
/// ```
///
///
pub fn id_mat_maxTimeForResult() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([107])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyDNRenameOnlyOpsProc OBJECT IDENTIFIER ::= {id-mat 108}
/// ```
///
///
pub fn id_mat_modifyDNRenameOnlyOpsProc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([108])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-serviceDesc OBJECT IDENTIFIER ::= {id-mat 109}
/// ```
///
///
pub fn id_mat_serviceDesc() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([109])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-serviceId OBJECT IDENTIFIER ::= {id-mat 110}
/// ```
///
///
pub fn id_mat_serviceId() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([110])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-subSchema OBJECT IDENTIFIER ::= {id-mat 111}
/// ```
///
///
pub fn id_mat_subSchema() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([111])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-sizeLimit OBJECT IDENTIFIER ::= {id-mat 112}
/// ```
///
///
pub fn id_mat_sizeLimit() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([112])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-timeLimit OBJECT IDENTIFIER ::= {id-mat 113}
/// ```
///
///
pub fn id_mat_timeLimit() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([113])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirCustName OBJECT IDENTIFIER ::= {id-mat 114}
/// ```
///
///
pub fn id_mat_dirCustName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([114])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirUserName OBJECT IDENTIFIER ::= {id-mat 115}
/// ```
///
///
pub fn id_mat_dirUserName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([115])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirCustAddr OBJECT IDENTIFIER ::= {id-mat 116}
/// ```
///
///
pub fn id_mat_dirCustAddr() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([116])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dMDName OBJECT IDENTIFIER ::= {id-mat 117}
/// ```
///
///
pub fn id_mat_dMDName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([117])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-accessControlScheme OBJECT IDENTIFIER ::= {id-mat 119}
/// ```
///
///
pub fn id_mat_accessControlScheme() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([119])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-administrativeRole OBJECT IDENTIFIER ::= {id-mat 120}
/// ```
///
///
pub fn id_mat_administrativeRole() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([120])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-aliasedEntryName OBJECT IDENTIFIER ::= {id-mat 121}
/// ```
///
///
pub fn id_mat_aliasedEntryName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([121])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-attributeTypes OBJECT IDENTIFIER ::= {id-mat 122}
/// ```
///
///
pub fn id_mat_attributeTypes() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([122])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-collectiveExclusions OBJECT IDENTIFIER ::= {id-mat 123}
/// ```
///
///
pub fn id_mat_collectiveExclusions() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([123])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-consumerKnowledge OBJECT IDENTIFIER ::= {id-mat 124}
/// ```
///
///
pub fn id_mat_consumerKnowledge() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([124])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-createTimestamp OBJECT IDENTIFIER ::= {id-mat 125}
/// ```
///
///
pub fn id_mat_createTimestamp() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([125])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-creatorsName OBJECT IDENTIFIER ::= {id-mat 126}
/// ```
///
///
pub fn id_mat_creatorsName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([126])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-credentials OBJECT IDENTIFIER ::= {id-mat 127}
/// ```
///
///
pub fn id_mat_credentials() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([127])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-distName OBJECT IDENTIFIER ::= {id-mat 128}
/// ```
///
///
pub fn id_mat_distName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([128])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dITContentRules OBJECT IDENTIFIER ::= {id-mat 129}
/// ```
///
///
pub fn id_mat_dITContentRules() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([129])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dITStructureRule OBJECT IDENTIFIER ::= {id-mat 130}
/// ```
///
///
pub fn id_mat_dITStructureRule() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([130])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dseType OBJECT IDENTIFIER ::= {id-mat 131}
/// ```
///
///
pub fn id_mat_dseType() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([131])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-entryACI OBJECT IDENTIFIER ::= {id-mat 132}
/// ```
///
///
pub fn id_mat_entryACI() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([132])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-governingSR OBJECT IDENTIFIER ::= {id-mat 133}
/// ```
///
///
pub fn id_mat_governingSR() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([133])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-matchingRules OBJECT IDENTIFIER ::= {id-mat 134}
/// ```
///
///
pub fn id_mat_matchingRules() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([134])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-matchingRuleUse OBJECT IDENTIFIER ::= {id-mat 135}
/// ```
///
///
pub fn id_mat_matchingRuleUse() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([135])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifiersName OBJECT IDENTIFIER ::= {id-mat 136}
/// ```
///
///
pub fn id_mat_modifiersName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([136])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-modifyTimestamp OBJECT IDENTIFIER ::= {id-mat 137}
/// ```
///
///
pub fn id_mat_modifyTimestamp() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([137])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-myAccessPoint OBJECT IDENTIFIER ::= {id-mat 138}
/// ```
///
///
pub fn id_mat_myAccessPoint() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([138])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nonSpecificKnowledge OBJECT IDENTIFIER ::= {id-mat 139}
/// ```
///
///
pub fn id_mat_nonSpecificKnowledge() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([139])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-objectClass OBJECT IDENTIFIER ::= {id-mat 140}
/// ```
///
///
pub fn id_mat_objectClass() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([140])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-objectClasses OBJECT IDENTIFIER ::= {id-mat 141}
/// ```
///
///
pub fn id_mat_objectClasses() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([141])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-prescriptiveACI OBJECT IDENTIFIER ::= {id-mat 142}
/// ```
///
///
pub fn id_mat_prescriptiveACI() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([142])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-nameForms OBJECT IDENTIFIER ::= {id-mat 143}
/// ```
///
///
pub fn id_mat_nameForms() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([143])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-specificKnowledge OBJECT IDENTIFIER ::= {id-mat 144}
/// ```
///
///
pub fn id_mat_specificKnowledge() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([144])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-structuralObjectClass OBJECT IDENTIFIER ::= {id-mat 145}
/// ```
///
///
pub fn id_mat_structuralObjectClass() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([145])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-subentryACI OBJECT IDENTIFIER ::= {id-mat 146}
/// ```
///
///
pub fn id_mat_subentryACI() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([146])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-subtreeSpecification OBJECT IDENTIFIER ::= {id-mat 147}
/// ```
///
///
pub fn id_mat_subtreeSpecification() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([147])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-superiorKnowledge OBJECT IDENTIFIER ::= {id-mat 148}
/// ```
///
///
pub fn id_mat_superiorKnowledge() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([148])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-supplierKnowledge OBJECT IDENTIFIER ::= {id-mat 149}
/// ```
///
///
pub fn id_mat_supplierKnowledge() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([149])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mat-dirCommonName OBJECT IDENTIFIER ::= {id-mat 150}
/// ```
///
///
pub fn id_mat_dirCommonName() -> OBJECT_IDENTIFIER {
    [id_mat(), Vec::<u32>::from([150])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dsa OBJECT IDENTIFIER ::= {id-moc 0}
/// ```
///
///
pub fn id_moc_dsa() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dse OBJECT IDENTIFIER ::= {id-moc 1}
/// ```
///
///
pub fn id_moc_dse() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-knownDSA OBJECT IDENTIFIER ::= {id-moc 2}
/// ```
///
///
pub fn id_moc_knownDSA() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-knownDUA OBJECT IDENTIFIER ::= {id-moc 3}
/// ```
///
///
pub fn id_moc_knownDUA() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dUA OBJECT IDENTIFIER ::= {id-moc 4}
/// ```
///
///
pub fn id_moc_dUA() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-nHOBMO OBJECT IDENTIFIER ::= {id-moc 5}
/// ```
///
///
pub fn id_moc_nHOBMO() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-hOBMO OBJECT IDENTIFIER ::= {id-moc 6}
/// ```
///
///
pub fn id_moc_hOBMO() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-shadowingAgreement OBJECT IDENTIFIER ::= {id-moc 7}
/// ```
///
///
pub fn id_moc_shadowingAgreement() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-ULconnEnd OBJECT IDENTIFIER ::= {id-moc 8}
/// ```
///
///
pub fn id_moc_ULconnEnd() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-disManagedObject OBJECT IDENTIFIER ::= {id-moc 9}
/// ```
///
///
pub fn id_moc_disManagedObject() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dirCust OBJECT IDENTIFIER ::= {id-moc 10}
/// ```
///
///
pub fn id_moc_dirCust() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dirUser OBJECT IDENTIFIER ::= {id-moc 11}
/// ```
///
///
pub fn id_moc_dirUser() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-moc-dMD OBJECT IDENTIFIER ::= {id-moc 12}
/// ```
///
///
pub fn id_moc_dMD() -> OBJECT_IDENTIFIER {
    [id_moc(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-dsa-name-binding OBJECT IDENTIFIER ::= {id-mnb 0}
/// ```
///
///
pub fn id_mnb_dsa_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-dse-name-binding OBJECT IDENTIFIER ::= {id-mnb 1}
/// ```
///
///
pub fn id_mnb_dse_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-knownDSA-dSA-name-binding OBJECT IDENTIFIER ::= {id-mnb 2}
/// ```
///
///
pub fn id_mnb_knownDSA_dSA_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-knownDUA-dSA-name-binding OBJECT IDENTIFIER ::= {id-mnb 3}
/// ```
///
///
pub fn id_mnb_knownDUA_dSA_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-acseInvoc-knownDSA OBJECT IDENTIFIER ::= {id-mnb 4}
/// ```
///
///
pub fn id_mnb_acseInvoc_knownDSA() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-acseInvoc-knownDUA OBJECT IDENTIFIER ::= {id-mnb 5}
/// ```
///
///
pub fn id_mnb_acseInvoc_knownDUA() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-nHOB-name-binding OBJECT IDENTIFIER ::= {id-mnb 6}
/// ```
///
///
pub fn id_mnb_nHOB_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-hOB-name-binding OBJECT IDENTIFIER ::= {id-mnb 7}
/// ```
///
///
pub fn id_mnb_hOB_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-shadowingAgreement-nb OBJECT IDENTIFIER ::= {id-mnb 8}
/// ```
///
///
pub fn id_mnb_shadowingAgreement_nb() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-ULconnEnd-knownDSA OBJECT IDENTIFIER ::= {id-mnb 9}
/// ```
///
///
pub fn id_mnb_ULconnEnd_knownDSA() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-ULconnEnd-knownDUA OBJECT IDENTIFIER ::= {id-mnb 10}
/// ```
///
///
pub fn id_mnb_ULconnEnd_knownDUA() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-dis-Customer-name-binding OBJECT IDENTIFIER ::= {id-mnb 11}
/// ```
///
///
pub fn id_mnb_dis_Customer_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-knownDSA-dUA-name-binding OBJECT IDENTIFIER ::= {id-mnb 12}
/// ```
///
///
pub fn id_mnb_knownDSA_dUA_name_binding() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-DirCust-DMD OBJECT IDENTIFIER ::= {id-mnb 13}
/// ```
///
///
pub fn id_mnb_DirCust_DMD() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mnb-DirUser-DirCust OBJECT IDENTIFIER ::= {id-mnb 14}
/// ```
///
///
pub fn id_mnb_DirUser_DirCust() -> OBJECT_IDENTIFIER {
    [id_mnb(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dsaPackage OBJECT IDENTIFIER ::= {id-mp 0}
/// ```
///
///
pub fn id_mp_dsaPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-readPackage OBJECT IDENTIFIER ::= {id-mp 1}
/// ```
///
///
pub fn id_mp_readPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-comparePackage OBJECT IDENTIFIER ::= {id-mp 2}
/// ```
///
///
pub fn id_mp_comparePackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-abandonPackage OBJECT IDENTIFIER ::= {id-mp 3}
/// ```
///
///
pub fn id_mp_abandonPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-listPackage OBJECT IDENTIFIER ::= {id-mp 4}
/// ```
///
///
pub fn id_mp_listPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-searchPackage OBJECT IDENTIFIER ::= {id-mp 5}
/// ```
///
///
pub fn id_mp_searchPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-addPackage OBJECT IDENTIFIER ::= {id-mp 6}
/// ```
///
///
pub fn id_mp_addPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-removePackage OBJECT IDENTIFIER ::= {id-mp 7}
/// ```
///
///
pub fn id_mp_removePackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-modifyPackage OBJECT IDENTIFIER ::= {id-mp 8}
/// ```
///
///
pub fn id_mp_modifyPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-modifyDNPackage OBJECT IDENTIFIER ::= {id-mp 9}
/// ```
///
///
pub fn id_mp_modifyDNPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedReadPackage OBJECT IDENTIFIER ::= {id-mp 10}
/// ```
///
///
pub fn id_mp_chainedReadPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedComparePackage OBJECT IDENTIFIER ::= {id-mp 11}
/// ```
///
///
pub fn id_mp_chainedComparePackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedAbandonPackage OBJECT IDENTIFIER ::= {id-mp 12}
/// ```
///
///
pub fn id_mp_chainedAbandonPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedListPackage OBJECT IDENTIFIER ::= {id-mp 13}
/// ```
///
///
pub fn id_mp_chainedListPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedSearchPackage OBJECT IDENTIFIER ::= {id-mp 14}
/// ```
///
///
pub fn id_mp_chainedSearchPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedAddPackage OBJECT IDENTIFIER ::= {id-mp 15}
/// ```
///
///
pub fn id_mp_chainedAddPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedRemovePackage OBJECT IDENTIFIER ::= {id-mp 16}
/// ```
///
///
pub fn id_mp_chainedRemovePackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedModifyPackage OBJECT IDENTIFIER ::= {id-mp 17}
/// ```
///
///
pub fn id_mp_chainedModifyPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-chainedModifyDNPackage OBJECT IDENTIFIER ::= {id-mp 18}
/// ```
///
///
pub fn id_mp_chainedModifyDNPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dsePackage OBJECT IDENTIFIER ::= {id-mp 19}
/// ```
///
///
pub fn id_mp_dsePackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-knownDSAPackage OBJECT IDENTIFIER ::= {id-mp 20}
/// ```
///
///
pub fn id_mp_knownDSAPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-knownDUAPackage OBJECT IDENTIFIER ::= {id-mp 21}
/// ```
///
///
pub fn id_mp_knownDUAPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dUAPackage OBJECT IDENTIFIER ::= {id-mp 22}
/// ```
///
///
pub fn id_mp_dUAPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([22])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-nHOBPackage OBJECT IDENTIFIER ::= {id-mp 23}
/// ```
///
///
pub fn id_mp_nHOBPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-hOBPackage OBJECT IDENTIFIER ::= {id-mp 24}
/// ```
///
///
pub fn id_mp_hOBPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-shadowingAgreementPackage OBJECT IDENTIFIER ::= {id-mp 25}
/// ```
///
///
pub fn id_mp_shadowingAgreementPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-ULconnEndPackage OBJECT IDENTIFIER ::= {id-mp 26}
/// ```
///
///
pub fn id_mp_ULconnEndPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-disPackage OBJECT IDENTIFIER ::= {id-mp 27}
/// ```
///
///
pub fn id_mp_disPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dcsPackage OBJECT IDENTIFIER ::= {id-mp 28}
/// ```
///
///
pub fn id_mp_dcsPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dirCust OBJECT IDENTIFIER ::= {id-mp 29}
/// ```
///
///
pub fn id_mp_dirCust() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([29])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dirUser OBJECT IDENTIFIER ::= {id-mp 30}
/// ```
///
///
pub fn id_mp_dirUser() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dMD OBJECT IDENTIFIER ::= {id-mp 31}
/// ```
///
///
pub fn id_mp_dMD() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mp-dsPackage OBJECT IDENTIFIER ::= {id-mp 32}
/// ```
///
///
pub fn id_mp_dsPackage() -> OBJECT_IDENTIFIER {
    [id_mp(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-nameProblem OBJECT IDENTIFIER ::= {id-mpa 1}
/// ```
///
///
pub fn id_mpa_nameProblem() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-traceInformation OBJECT IDENTIFIER ::= {id-mpa 2}
/// ```
///
///
pub fn id_mpa_traceInformation() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-serviceProblem OBJECT IDENTIFIER ::= {id-mpa 3}
/// ```
///
///
pub fn id_mpa_serviceProblem() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-entryName OBJECT IDENTIFIER ::= {id-mpa 4}
/// ```
///
///
pub fn id_mpa_entryName() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-operation OBJECT IDENTIFIER ::= {id-mpa 5}
/// ```
///
///
pub fn id_mpa_operation() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-attributeProblem OBJECT IDENTIFIER ::= {id-mpa 6}
/// ```
///
///
pub fn id_mpa_attributeProblem() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-attributeType OBJECT IDENTIFIER ::= {id-mpa 7}
/// ```
///
///
pub fn id_mpa_attributeType() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-shadowProblem OBJECT IDENTIFIER ::= {id-mpa 8}
/// ```
///
///
pub fn id_mpa_shadowProblem() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-attributeValue OBJECT IDENTIFIER ::= {id-mpa 9}
/// ```
///
///
pub fn id_mpa_attributeValue() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-resource OBJECT IDENTIFIER ::= {id-mpa 10}
/// ```
///
///
pub fn id_mpa_resource() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-authenReason OBJECT IDENTIFIER ::= {id-mpa 11}
/// ```
///
///
pub fn id_mpa_authenReason() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-updateProblem OBJECT IDENTIFIER ::= {id-mpa 12}
/// ```
///
///
pub fn id_mpa_updateProblem() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-extensions OBJECT IDENTIFIER ::= {id-mpa 15}
/// ```
///
///
pub fn id_mpa_extensions() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-aliasedRDNs OBJECT IDENTIFIER ::= {id-mpa 16}
/// ```
///
///
pub fn id_mpa_aliasedRDNs() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-aliasDereferenced OBJECT IDENTIFIER ::= {id-mpa 17}
/// ```
///
///
pub fn id_mpa_aliasDereferenced() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-referenceType OBJECT IDENTIFIER ::= {id-mpa 18}
/// ```
///
///
pub fn id_mpa_referenceType() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-operationProgress OBJECT IDENTIFIER ::= {id-mpa 19}
/// ```
///
///
pub fn id_mpa_operationProgress() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-pDU OBJECT IDENTIFIER ::= {id-mpa 20}
/// ```
///
///
pub fn id_mpa_pDU() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-opId OBJECT IDENTIFIER ::= {id-mpa 21}
/// ```
///
///
pub fn id_mpa_opId() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-nhob-bind-id OBJECT IDENTIFIER ::= {id-mpa 22}
/// ```
///
///
pub fn id_mpa_nhob_bind_id() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([22])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-mhob-dop-prob OBJECT IDENTIFIER ::= {id-mpa 23}
/// ```
///
///
pub fn id_mpa_mhob_dop_prob() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-hob-bind-id OBJECT IDENTIFIER ::= {id-mpa 24}
/// ```
///
///
pub fn id_mpa_hob_bind_id() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-hob-dop-prob OBJECT IDENTIFIER ::= {id-mpa 25}
/// ```
///
///
pub fn id_mpa_hob_dop_prob() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-shadowing-dop-prob OBJECT IDENTIFIER ::= {id-mpa 26}
/// ```
///
///
pub fn id_mpa_shadowing_dop_prob() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mpa-opIdDN OBJECT IDENTIFIER ::= {id-mpa 27}
/// ```
///
///
pub fn id_mpa_opIdDN() -> OBJECT_IDENTIFIER {
    [id_mpa(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryInformationServiceElement-operationType ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type DirectoryInformationServiceElement_operationType = BIT_STRING;

pub const DirectoryInformationServiceElement_operationType_read: BIT = 0; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_compare: BIT = 1; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_abandon: BIT = 2; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_list: BIT = 3; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_search: BIT = 4; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_addEntry: BIT = 5; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_removeEntry: BIT = 6; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_modifyEntry: BIT = 7; /* LONG_NAMED_BIT */

pub const DirectoryInformationServiceElement_operationType_modifyDN: BIT = 8; /* LONG_NAMED_BIT */

pub fn _decode_DirectoryInformationServiceElement_operationType(
    el: &X690Element,
) -> ASN1Result<DirectoryInformationServiceElement_operationType> {
    ber_decode_bit_string(&el)
}

pub fn _encode_DirectoryInformationServiceElement_operationType(
    value_: &DirectoryInformationServiceElement_operationType,
) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubSchemaSyntax-Item-subSchema ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for SubSchemaSyntax_Item_subSchema {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubSchemaSyntax_Item_subSchema(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubSchemaSyntax_Item_subSchema {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SubSchemaSyntax_Item_subSchema> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubSchemaSyntax_Item_subSchema,
            _eal_components_for_SubSchemaSyntax_Item_subSchema,
            _rctl2_components_for_SubSchemaSyntax_Item_subSchema,
        )?;
        let structureRules: OPTIONAL<Vec<DITStructureRuleDescription>> = match _components
            .get("structureRules")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<DITStructureRuleDescription>> {
                    Ok(|el: &X690Element| -> ASN1Result<
                        SEQUENCE_OF<DITStructureRuleDescription>,
                    > {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<DITStructureRuleDescription> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_DITStructureRuleDescription(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let contentRules: OPTIONAL<Vec<DITContentRuleDescription>> = match _components
            .get("contentRules")
        {
            Some(c_) => {
                Some(
                    |el: &X690Element| -> ASN1Result<Vec<DITContentRuleDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<DITContentRuleDescription>> {
	let elements = match el.value.borrow() {
		X690Encoding::Constructed(children) => children,
		_ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
	};
	let mut items: SEQUENCE_OF<DITContentRuleDescription> = Vec::with_capacity(elements.len());
	for el in elements {
		items.push(_decode_DITContentRuleDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(c_)?,
                )
            }
            _ => None,
        };
        let matchingRules: OPTIONAL<Vec<MatchingRuleDescription>> =
            match _components.get("matchingRules") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<Vec<MatchingRuleDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MatchingRuleDescription>> {
	let elements = match el.value.borrow() {
		X690Encoding::Constructed(children) => children,
		_ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
	};
	let mut items: SEQUENCE_OF<MatchingRuleDescription> = Vec::with_capacity(elements.len());
	for el in elements {
		items.push(_decode_MatchingRuleDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let attributeTypes: OPTIONAL<Vec<AttributeTypeDescription>> =
            match _components.get("attributeTypes") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<Vec<AttributeTypeDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeTypeDescription>> {
	let elements = match el.value.borrow() {
		X690Encoding::Constructed(children) => children,
		_ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
	};
	let mut items: SEQUENCE_OF<AttributeTypeDescription> = Vec::with_capacity(elements.len());
	for el in elements {
		items.push(_decode_AttributeTypeDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let objectClasses: OPTIONAL<Vec<ObjectClassDescription>> = match _components
            .get("objectClasses")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<ObjectClassDescription>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ObjectClassDescription>> {
                            let elements = match el.value.borrow() {
                                X690Encoding::Constructed(children) => children,
                                _ => {
                                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction))
                                }
                            };
                            let mut items: SEQUENCE_OF<ObjectClassDescription> =
                                Vec::with_capacity(elements.len());
                            for el in elements {
                                items.push(_decode_ObjectClassDescription(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(c_)?,
            ),
            _ => None,
        };
        let nameForms: OPTIONAL<Vec<NameFormDescription>> = match _components.get("nameForms") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<NameFormDescription>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<NameFormDescription>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<NameFormDescription> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_NameFormDescription(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let matchRuleUses: OPTIONAL<Vec<MatchingRuleUseDescription>> = match _components
            .get("matchRuleUses")
        {
            Some(c_) => {
                Some(
                    |el: &X690Element| -> ASN1Result<Vec<MatchingRuleUseDescription>> {
                        Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MatchingRuleUseDescription>> {
	let elements = match el.value.borrow() {
		X690Encoding::Constructed(children) => children,
		_ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
	};
	let mut items: SEQUENCE_OF<MatchingRuleUseDescription> = Vec::with_capacity(elements.len());
	for el in elements {
		items.push(_decode_MatchingRuleUseDescription(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                    }(c_)?,
                )
            }
            _ => None,
        };
        Ok(SubSchemaSyntax_Item_subSchema {
            structureRules,
            contentRules,
            matchingRules,
            attributeTypes,
            objectClasses,
            nameForms,
            matchRuleUses,
        })
    }(&el)
}

pub fn _encode_SubSchemaSyntax_Item_subSchema(
    value_: &SubSchemaSyntax_Item_subSchema,
) -> ASN1Result<X690Element> {
    |value_: &SubSchemaSyntax_Item_subSchema| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.structureRules {
            components_.push(|v_1: &Vec<DITStructureRuleDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 1, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<DITStructureRuleDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_DITStructureRuleDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.contentRules {
            components_.push(|v_1: &Vec<DITContentRuleDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 2, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<DITContentRuleDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_DITContentRuleDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.matchingRules {
            components_.push(|v_1: &Vec<MatchingRuleDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 3, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<MatchingRuleDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_MatchingRuleDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.attributeTypes {
            components_.push(|v_1: &Vec<AttributeTypeDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 4, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<AttributeTypeDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AttributeTypeDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.objectClasses {
            components_.push(|v_1: &Vec<ObjectClassDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 5, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<ObjectClassDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_ObjectClassDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.nameForms {
            components_.push(|v_1: &Vec<NameFormDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 6, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<NameFormDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_NameFormDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.matchRuleUses {
            components_.push(|v_1: &Vec<MatchingRuleUseDescription>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 7, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<MatchingRuleUseDescription>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_MatchingRuleUseDescription(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubSchemaSyntax-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for SubSchemaSyntax_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubSchemaSyntax_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubSchemaSyntax_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SubSchemaSyntax_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubSchemaSyntax_Item,
            _eal_components_for_SubSchemaSyntax_Item,
            _rctl2_components_for_SubSchemaSyntax_Item,
        )?;
        let name = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("name").unwrap(),
        )?;
        let subSchema = |el: &X690Element| -> ASN1Result<SubSchemaSyntax_Item_subSchema> {
            Ok(_decode_SubSchemaSyntax_Item_subSchema(&el.inner()?)?)
        }(_components.get("subSchema").unwrap())?;
        Ok(SubSchemaSyntax_Item { name, subSchema })
    }(&el)
}

pub fn _encode_SubSchemaSyntax_Item(value_: &SubSchemaSyntax_Item) -> ASN1Result<X690Element> {
    |value_: &SubSchemaSyntax_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.name)?);
        components_.push(
            |v_1: &SubSchemaSyntax_Item_subSchema| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SubSchemaSyntax_Item_subSchema(&v_1)?,
                    ))),
                ))
            }(&value_.subSchema)?,
        );
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}
