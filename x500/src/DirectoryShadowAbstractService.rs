#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DirectoryShadowAbstractService
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DirectoryShadowAbstractService`.
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
use crate::CommonProtocolSpecification::*;
use crate::DSAOperationalAttributeTypes::*;
use crate::DirectoryAbstractService::*;
use crate::DirectoryOSIProtocols::*;
use crate::DirectoryOperationalBindingTypes::*;
use crate::DistributedOperations::*;
use crate::EnhancedSecurity::*;
use crate::InformationFramework::*;
use crate::OperationalBindingManagement::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// dSAShadowBind OPERATION ::= dSABind
/// ```
///
///
pub fn dSAShadowBind() -> OPERATION {
    dSABind()
}

// Manually added
pub fn shadowOperationalBinding_coop_1() -> OP_BINDING_COOP {
    OP_BINDING_COOP {
        applContext: shadowSupplierInitiatedAC(),
        Operations: Some(All_operations_supplier_initiated()),
    }
}

// Manually added
pub fn shadowOperationalBinding_coop_2() -> OP_BINDING_COOP {
    OP_BINDING_COOP {
        applContext: shadowConsumerInitiatedAC(),
        Operations: Some(All_operations_consumer_initiated()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowOperationalBinding OPERATIONAL-BINDING ::= {
///   AGREEMENT             ShadowingAgreementInfo
///   APPLICATION CONTEXTS
///     {{shadowSupplierInitiatedAC
///       APPLIES TO  {All-operations-supplier-initiated}} |
///     {shadowConsumerInitiatedAC
///       APPLIES TO  {All-operations-consumer-initiated}}}
///   ASYMMETRIC
///     ROLE-A { -- shadow supplier role
///       ESTABLISHMENT-INITIATOR  TRUE
///       ESTABLISHMENT-PARAMETER  NULL
///       MODIFICATION-INITIATOR   TRUE
///       TERMINATION-INITIATOR    TRUE }
///     ROLE-B { -- shadow consumer role
///       ESTABLISHMENT-INITIATOR  TRUE
///       ESTABLISHMENT-PARAMETER  NULL
///       MODIFICATION-INITIATOR   TRUE
///       MODIFICATION-PARAMETER   ModificationParameter
///       TERMINATION-INITIATOR    TRUE}
///   ID                    id-op-binding-shadow }
/// ```
///
///
pub fn shadowOperationalBinding() -> OPERATIONAL_BINDING {
    OPERATIONAL_BINDING {
        Cooperation: Vec::<_>::from([
            shadowOperationalBinding_coop_1(),
            shadowOperationalBinding_coop_2(),
        ]), /* OBJECT_FIELD_SETTING */
        roleA: Some(shadowOperationalBinding_roleA()), /* OBJECT_FIELD_SETTING */
        roleB: Some(shadowOperationalBinding_roleB()), /* OBJECT_FIELD_SETTING */
        id: id_op_binding_shadow(),                    /* OBJECT_FIELD_SETTING */
        both: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModificationParameter ::= SEQUENCE {
///   secondaryShadows  SET OF SupplierAndConsumers,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModificationParameter {
    pub secondaryShadows: Vec<SupplierAndConsumers>,
    pub _unrecognized: Vec<X690Element>,
}
impl ModificationParameter {
    pub fn new(
        secondaryShadows: Vec<SupplierAndConsumers>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ModificationParameter {
            secondaryShadows,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ModificationParameter {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModificationParameter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModificationParameter {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModificationParameter(el)
    }
}

pub const _rctl1_components_for_ModificationParameter: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "secondaryShadows",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 17)),
    None,
    None,
)];

pub const _rctl2_components_for_ModificationParameter: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModificationParameter: &[ComponentSpec; 0] = &[];

pub fn _decode_ModificationParameter(el: &X690Element) -> ASN1Result<ModificationParameter> {
    |el_: &X690Element| -> ASN1Result<ModificationParameter> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModificationParameter,
            _eal_components_for_ModificationParameter,
            _rctl2_components_for_ModificationParameter,
        )?;
        let secondaryShadows = |el: &X690Element| -> ASN1Result<SET_OF<SupplierAndConsumers>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<SupplierAndConsumers> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_SupplierAndConsumers(el)?);
            }
            Ok(items)
        }(_components.get("secondaryShadows").unwrap())?;
        Ok(ModificationParameter {
            secondaryShadows,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModificationParameter(value_: &ModificationParameter) -> ASN1Result<X690Element> {
    |value_: &ModificationParameter| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(
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
            }(&value_.secondaryShadows)?,
        );
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AgreementID  ::=  OperationalBindingID
/// ```
pub type AgreementID = OperationalBindingID; // DefinedType

pub fn _decode_AgreementID(el: &X690Element) -> ASN1Result<AgreementID> {
    _decode_OperationalBindingID(&el)
}

pub fn _encode_AgreementID(value_: &AgreementID) -> ASN1Result<X690Element> {
    _encode_OperationalBindingID(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowingAgreementInfo ::= SEQUENCE {
///   shadowSubject          UnitOfReplication,
///   updateMode             UpdateMode DEFAULT supplierInitiated:onChange:TRUE,
///   master                 AccessPoint OPTIONAL,
///   secondaryShadows  [2]  BOOLEAN DEFAULT FALSE }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ShadowingAgreementInfo {
    pub shadowSubject: UnitOfReplication,
    pub updateMode: OPTIONAL<UpdateMode>,
    pub master: OPTIONAL<AccessPoint>,
    pub secondaryShadows: OPTIONAL<BOOLEAN>,
}
impl ShadowingAgreementInfo {
    pub fn new(
        shadowSubject: UnitOfReplication,
        updateMode: OPTIONAL<UpdateMode>,
        master: OPTIONAL<AccessPoint>,
        secondaryShadows: OPTIONAL<BOOLEAN>,
    ) -> Self {
        ShadowingAgreementInfo {
            shadowSubject,
            updateMode,
            master,
            secondaryShadows,
        }
    }
    pub fn _default_value_for_updateMode() -> UpdateMode {
        UpdateMode::supplierInitiated(SupplierUpdateMode::onChange(TRUE))
    }
    pub fn _default_value_for_secondaryShadows() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ShadowingAgreementInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ShadowingAgreementInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ShadowingAgreementInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ShadowingAgreementInfo(el)
    }
}

pub const _rctl1_components_for_ShadowingAgreementInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "shadowSubject",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "updateMode",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "master",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "secondaryShadows",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ShadowingAgreementInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ShadowingAgreementInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ShadowingAgreementInfo(el: &X690Element) -> ASN1Result<ShadowingAgreementInfo> {
    |el_: &X690Element| -> ASN1Result<ShadowingAgreementInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ShadowingAgreementInfo,
            _eal_components_for_ShadowingAgreementInfo,
            _rctl2_components_for_ShadowingAgreementInfo,
        )?;
        let shadowSubject = _decode_UnitOfReplication(_components.get("shadowSubject").unwrap())?;
        let updateMode: OPTIONAL<UpdateMode> = match _components.get("updateMode") {
            Some(c_) => Some(_decode_UpdateMode(c_)?),
            _ => None,
        };
        let master: OPTIONAL<AccessPoint> = match _components.get("master") {
            Some(c_) => Some(_decode_AccessPoint(c_)?),
            _ => None,
        };
        let secondaryShadows: OPTIONAL<BOOLEAN> = match _components.get("secondaryShadows") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(ShadowingAgreementInfo {
            shadowSubject,
            updateMode,
            master,
            secondaryShadows,
        })
    }(&el)
}

pub fn _encode_ShadowingAgreementInfo(value_: &ShadowingAgreementInfo) -> ASN1Result<X690Element> {
    |value_: &ShadowingAgreementInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(_encode_UnitOfReplication(&value_.shadowSubject)?);
        if let Some(v_) = &value_.updateMode {
            if *v_ != ShadowingAgreementInfo::_default_value_for_updateMode() {
                components_.push(_encode_UpdateMode(&v_)?);
            }
        }
        if let Some(v_) = &value_.master {
            components_.push(_encode_AccessPoint(&v_)?);
        }
        if let Some(v_) = &value_.secondaryShadows {
            if *v_ != ShadowingAgreementInfo::_default_value_for_secondaryShadows() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnitOfReplication ::= SEQUENCE {
///   area                 AreaSpecification,
///   attributes           AttributeSelection,
///   knowledge            Knowledge OPTIONAL,
///   subordinates         BOOLEAN DEFAULT FALSE,
///   contextSelection     ContextSelection OPTIONAL,
///   supplyContexts  [0]  CHOICE {
///     allContexts         NULL,
///     selectedContexts    SET SIZE (1..MAX) OF CONTEXT.&id,
///     ... } OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UnitOfReplication {
    pub area: AreaSpecification,
    pub attributes: AttributeSelection,
    pub knowledge: OPTIONAL<Knowledge>,
    pub subordinates: OPTIONAL<BOOLEAN>,
    pub contextSelection: OPTIONAL<ContextSelection>,
    pub supplyContexts: OPTIONAL<UnitOfReplication_supplyContexts>,
}
impl UnitOfReplication {
    pub fn new(
        area: AreaSpecification,
        attributes: AttributeSelection,
        knowledge: OPTIONAL<Knowledge>,
        subordinates: OPTIONAL<BOOLEAN>,
        contextSelection: OPTIONAL<ContextSelection>,
        supplyContexts: OPTIONAL<UnitOfReplication_supplyContexts>,
    ) -> Self {
        UnitOfReplication {
            area,
            attributes,
            knowledge,
            subordinates,
            contextSelection,
            supplyContexts,
        }
    }
    pub fn _default_value_for_subordinates() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for UnitOfReplication {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UnitOfReplication(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UnitOfReplication {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UnitOfReplication(el)
    }
}

pub const _rctl1_components_for_UnitOfReplication: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "area",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "knowledge",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subordinates",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextSelection",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 5)),
            &TagSelector::tag((TagClass::UNIVERSAL, 17)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "supplyContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UnitOfReplication: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UnitOfReplication: &[ComponentSpec; 0] = &[];

pub fn _decode_UnitOfReplication(el: &X690Element) -> ASN1Result<UnitOfReplication> {
    |el_: &X690Element| -> ASN1Result<UnitOfReplication> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UnitOfReplication,
            _eal_components_for_UnitOfReplication,
            _rctl2_components_for_UnitOfReplication,
        )?;
        let area = _decode_AreaSpecification(_components.get("area").unwrap())?;
        let attributes = _decode_AttributeSelection(_components.get("attributes").unwrap())?;
        let knowledge: OPTIONAL<Knowledge> = match _components.get("knowledge") {
            Some(c_) => Some(_decode_Knowledge(c_)?),
            _ => None,
        };
        let subordinates: OPTIONAL<BOOLEAN> = match _components.get("subordinates") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let contextSelection: OPTIONAL<ContextSelection> = match _components.get("contextSelection")
        {
            Some(c_) => Some(_decode_ContextSelection(c_)?),
            _ => None,
        };
        let supplyContexts: OPTIONAL<UnitOfReplication_supplyContexts> =
            match _components.get("supplyContexts") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<UnitOfReplication_supplyContexts> {
                        Ok(_decode_UnitOfReplication_supplyContexts(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(UnitOfReplication {
            area,
            attributes,
            knowledge,
            subordinates,
            contextSelection,
            supplyContexts,
        })
    }(&el)
}

pub fn _encode_UnitOfReplication(value_: &UnitOfReplication) -> ASN1Result<X690Element> {
    |value_: &UnitOfReplication| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_AreaSpecification(&value_.area)?);
        components_.push(_encode_AttributeSelection(&value_.attributes)?);
        if let Some(v_) = &value_.knowledge {
            components_.push(_encode_Knowledge(&v_)?);
        }
        if let Some(v_) = &value_.subordinates {
            if *v_ != UnitOfReplication::_default_value_for_subordinates() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.contextSelection {
            components_.push(_encode_ContextSelection(&v_)?);
        }
        if let Some(v_) = &value_.supplyContexts {
            components_.push(
                |v_1: &UnitOfReplication_supplyContexts| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnitOfReplication_supplyContexts(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AreaSpecification ::= SEQUENCE {
///   contextPrefix    DistinguishedName,
///   replicationArea  SubtreeSpecification,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AreaSpecification {
    pub contextPrefix: DistinguishedName,
    pub replicationArea: SubtreeSpecification,
    pub _unrecognized: Vec<X690Element>,
}
impl AreaSpecification {
    pub fn new(
        contextPrefix: DistinguishedName,
        replicationArea: SubtreeSpecification,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AreaSpecification {
            contextPrefix,
            replicationArea,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AreaSpecification {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AreaSpecification(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AreaSpecification {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AreaSpecification(el)
    }
}

pub const _rctl1_components_for_AreaSpecification: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "contextPrefix",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "replicationArea",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AreaSpecification: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AreaSpecification: &[ComponentSpec; 0] = &[];

pub fn _decode_AreaSpecification(el: &X690Element) -> ASN1Result<AreaSpecification> {
    |el_: &X690Element| -> ASN1Result<AreaSpecification> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AreaSpecification,
            _eal_components_for_AreaSpecification,
            _rctl2_components_for_AreaSpecification,
        )?;
        let contextPrefix = _decode_DistinguishedName(_components.get("contextPrefix").unwrap())?;
        let replicationArea =
            _decode_SubtreeSpecification(_components.get("replicationArea").unwrap())?;
        Ok(AreaSpecification {
            contextPrefix,
            replicationArea,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AreaSpecification(value_: &AreaSpecification) -> ASN1Result<X690Element> {
    |value_: &AreaSpecification| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_DistinguishedName(&value_.contextPrefix)?);
        components_.push(_encode_SubtreeSpecification(&value_.replicationArea)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Knowledge ::= SEQUENCE {
///   knowledgeType      ENUMERATED {
///     master (0),
///     shadow (1),
///     both   (2)},
///   extendedKnowledge  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Knowledge {
    pub knowledgeType: Knowledge_knowledgeType,
    pub extendedKnowledge: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl Knowledge {
    pub fn new(
        knowledgeType: Knowledge_knowledgeType,
        extendedKnowledge: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Knowledge {
            knowledgeType,
            extendedKnowledge,
            _unrecognized,
        }
    }
    pub fn _default_value_for_extendedKnowledge() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for Knowledge {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Knowledge(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Knowledge {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Knowledge(el)
    }
}

pub const _rctl1_components_for_Knowledge: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "knowledgeType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extendedKnowledge",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Knowledge: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Knowledge: &[ComponentSpec; 0] = &[];

pub fn _decode_Knowledge(el: &X690Element) -> ASN1Result<Knowledge> {
    |el_: &X690Element| -> ASN1Result<Knowledge> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Knowledge,
            _eal_components_for_Knowledge,
            _rctl2_components_for_Knowledge,
        )?;
        let knowledgeType =
            _decode_Knowledge_knowledgeType(_components.get("knowledgeType").unwrap())?;
        let extendedKnowledge: OPTIONAL<BOOLEAN> = match _components.get("extendedKnowledge") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(Knowledge {
            knowledgeType,
            extendedKnowledge,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Knowledge(value_: &Knowledge) -> ASN1Result<X690Element> {
    |value_: &Knowledge| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Knowledge_knowledgeType(&value_.knowledgeType)?);
        if let Some(v_) = &value_.extendedKnowledge {
            if *v_ != Knowledge::_default_value_for_extendedKnowledge() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeSelection  ::=  SET OF ClassAttributeSelection
/// ```
pub type AttributeSelection = Vec<ClassAttributeSelection>; // SetOfType

pub fn _decode_AttributeSelection(el: &X690Element) -> ASN1Result<AttributeSelection> {
    |el: &X690Element| -> ASN1Result<SET_OF<ClassAttributeSelection>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<ClassAttributeSelection> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_ClassAttributeSelection(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttributeSelection(value_: &AttributeSelection) -> ASN1Result<X690Element> {
    |value_: &SET_OF<ClassAttributeSelection>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_ClassAttributeSelection(&v)?);
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
/// ClassAttributeSelection ::= SEQUENCE {
///   class            OBJECT IDENTIFIER OPTIONAL,
///   classAttributes  ClassAttributes DEFAULT allAttributes:NULL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ClassAttributeSelection {
    pub class: OPTIONAL<OBJECT_IDENTIFIER>,
    pub classAttributes: OPTIONAL<ClassAttributes>,
}
impl ClassAttributeSelection {
    pub fn new(
        class: OPTIONAL<OBJECT_IDENTIFIER>,
        classAttributes: OPTIONAL<ClassAttributes>,
    ) -> Self {
        ClassAttributeSelection {
            class,
            classAttributes,
        }
    }
    pub fn _default_value_for_classAttributes() -> ClassAttributes {
        ClassAttributes::allAttributes(())
    }
}
impl Default for ClassAttributeSelection {
    fn default() -> Self {
        ClassAttributeSelection {
            class: None,
            classAttributes: None,
        }
    }
}
impl TryFrom<X690Element> for ClassAttributeSelection {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ClassAttributeSelection(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ClassAttributeSelection {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ClassAttributeSelection(el)
    }
}

pub const _rctl1_components_for_ClassAttributeSelection: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "class",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "classAttributes",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 5)),
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ClassAttributeSelection: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ClassAttributeSelection: &[ComponentSpec; 0] = &[];

pub fn _decode_ClassAttributeSelection(el: &X690Element) -> ASN1Result<ClassAttributeSelection> {
    |el_: &X690Element| -> ASN1Result<ClassAttributeSelection> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ClassAttributeSelection,
            _eal_components_for_ClassAttributeSelection,
            _rctl2_components_for_ClassAttributeSelection,
        )?;
        let class: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("class") {
            Some(c_) => Some(ber_decode_object_identifier(c_)?),
            _ => None,
        };
        let classAttributes: OPTIONAL<ClassAttributes> = match _components.get("classAttributes") {
            Some(c_) => Some(_decode_ClassAttributes(c_)?),
            _ => None,
        };
        Ok(ClassAttributeSelection {
            class,
            classAttributes,
        })
    }(&el)
}

pub fn _encode_ClassAttributeSelection(
    value_: &ClassAttributeSelection,
) -> ASN1Result<X690Element> {
    |value_: &ClassAttributeSelection| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.class {
            components_.push(ber_encode_object_identifier(&v_)?);
        }
        if let Some(v_) = &value_.classAttributes {
            match v_ {
                ClassAttributes::allAttributes(()) => {}
                _ => components_.push(_encode_ClassAttributes(&v_)?),
            };
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ClassAttributes  ::=  CHOICE {
///   allAttributes  NULL,
///   include        [0]  AttributeTypes,
///   exclude        [1]  AttributeTypes,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ClassAttributes {
    allAttributes(NULL),
    include(AttributeTypes),
    exclude(AttributeTypes),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ClassAttributes {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ClassAttributes(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ClassAttributes {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ClassAttributes(el)
    }
}

pub fn _decode_ClassAttributes(el: &X690Element) -> ASN1Result<ClassAttributes> {
    |el: &X690Element| -> ASN1Result<ClassAttributes> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(ClassAttributes::allAttributes(())),
            (TagClass::CONTEXT, 0) => Ok(ClassAttributes::include(_decode_AttributeTypes(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(ClassAttributes::exclude(_decode_AttributeTypes(&el)?)),
            _ => Ok(ClassAttributes::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ClassAttributes(value_: &ClassAttributes) -> ASN1Result<X690Element> {
    |value: &ClassAttributes| -> ASN1Result<X690Element> {
        match value {
            ClassAttributes::allAttributes(v) => ber_encode_null(&v),
            ClassAttributes::include(v) => |v_1: &AttributeTypes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeTypes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            ClassAttributes::exclude(v) => |v_1: &AttributeTypes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeTypes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            ClassAttributes::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypes  ::=  SET OF AttributeType
/// ```
pub type AttributeTypes = Vec<AttributeType>; // SetOfType

pub fn _decode_AttributeTypes(el: &X690Element) -> ASN1Result<AttributeTypes> {
    |el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AttributeType(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttributeTypes(value_: &AttributeTypes) -> ASN1Result<X690Element> {
    |value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AttributeType(&v)?);
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
/// UpdateMode  ::=  CHOICE {
///   supplierInitiated  [0]  SupplierUpdateMode,
///   consumerInitiated  [1]  ConsumerUpdateMode,
///   ... }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateMode {
    supplierInitiated(SupplierUpdateMode),
    consumerInitiated(ConsumerUpdateMode),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for UpdateMode {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateMode(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateMode {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateMode(el)
    }
}

pub fn _decode_UpdateMode(el: &X690Element) -> ASN1Result<UpdateMode> {
    |el: &X690Element| -> ASN1Result<UpdateMode> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(UpdateMode::supplierInitiated(
                |el: &X690Element| -> ASN1Result<SupplierUpdateMode> {
                    Ok(_decode_SupplierUpdateMode(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(UpdateMode::consumerInitiated(
                _decode_ConsumerUpdateMode(&el)?,
            )),
            _ => Ok(UpdateMode::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_UpdateMode(value_: &UpdateMode) -> ASN1Result<X690Element> {
    |value: &UpdateMode| -> ASN1Result<X690Element> {
        match value {
            UpdateMode::supplierInitiated(v) => {
                |v_1: &SupplierUpdateMode| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SupplierUpdateMode(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            UpdateMode::consumerInitiated(v) => {
                |v_1: &ConsumerUpdateMode| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ConsumerUpdateMode(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            UpdateMode::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupplierUpdateMode  ::=  CHOICE {
///   onChange   BOOLEAN,
///   scheduled  SchedulingParameters,
///   ... }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum SupplierUpdateMode {
    onChange(BOOLEAN),
    scheduled(SchedulingParameters),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for SupplierUpdateMode {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierUpdateMode(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SupplierUpdateMode {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierUpdateMode(el)
    }
}

pub fn _decode_SupplierUpdateMode(el: &X690Element) -> ASN1Result<SupplierUpdateMode> {
    |el: &X690Element| -> ASN1Result<SupplierUpdateMode> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 1) => Ok(SupplierUpdateMode::onChange(ber_decode_boolean(&el)?)),
            (TagClass::UNIVERSAL, 16) => Ok(SupplierUpdateMode::scheduled(
                _decode_SchedulingParameters(&el)?,
            )),
            _ => Ok(SupplierUpdateMode::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_SupplierUpdateMode(value_: &SupplierUpdateMode) -> ASN1Result<X690Element> {
    |value: &SupplierUpdateMode| -> ASN1Result<X690Element> {
        match value {
            SupplierUpdateMode::onChange(v) => ber_encode_boolean(&v),
            SupplierUpdateMode::scheduled(v) => _encode_SchedulingParameters(&v),
            SupplierUpdateMode::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ConsumerUpdateMode  ::=  SchedulingParameters
/// ```
pub type ConsumerUpdateMode = SchedulingParameters; // DefinedType

pub fn _decode_ConsumerUpdateMode(el: &X690Element) -> ASN1Result<ConsumerUpdateMode> {
    _decode_SchedulingParameters(&el)
}

pub fn _encode_ConsumerUpdateMode(value_: &ConsumerUpdateMode) -> ASN1Result<X690Element> {
    _encode_SchedulingParameters(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SchedulingParameters ::= SEQUENCE {
///   periodic    PeriodicStrategy OPTIONAL, -- shall be present if othertimes
///   --                                        is set to FALSE
///   othertimes  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct SchedulingParameters {
    pub periodic: OPTIONAL<PeriodicStrategy>,
    pub othertimes: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl SchedulingParameters {
    pub fn new(
        periodic: OPTIONAL<PeriodicStrategy>,
        othertimes: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SchedulingParameters {
            periodic,
            othertimes,
            _unrecognized,
        }
    }
    pub fn _default_value_for_othertimes() -> BOOLEAN {
        false
    }
}
impl Default for SchedulingParameters {
    fn default() -> Self {
        SchedulingParameters {
            periodic: None,
            othertimes: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for SchedulingParameters {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SchedulingParameters(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SchedulingParameters {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SchedulingParameters(el)
    }
}

pub const _rctl1_components_for_SchedulingParameters: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "periodic",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "othertimes",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SchedulingParameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SchedulingParameters: &[ComponentSpec; 0] = &[];

pub fn _decode_SchedulingParameters(el: &X690Element) -> ASN1Result<SchedulingParameters> {
    |el_: &X690Element| -> ASN1Result<SchedulingParameters> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SchedulingParameters,
            _eal_components_for_SchedulingParameters,
            _rctl2_components_for_SchedulingParameters,
        )?;
        let periodic: OPTIONAL<PeriodicStrategy> = match _components.get("periodic") {
            Some(c_) => Some(_decode_PeriodicStrategy(c_)?),
            _ => None,
        };
        let othertimes: OPTIONAL<BOOLEAN> = match _components.get("othertimes") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(SchedulingParameters {
            periodic,
            othertimes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SchedulingParameters(value_: &SchedulingParameters) -> ASN1Result<X690Element> {
    |value_: &SchedulingParameters| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.periodic {
            components_.push(_encode_PeriodicStrategy(&v_)?);
        }
        if let Some(v_) = &value_.othertimes {
            if *v_ != SchedulingParameters::_default_value_for_othertimes() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PeriodicStrategy ::= SEQUENCE {
///   beginTime       Time OPTIONAL,
///   windowSize      INTEGER,
///   updateInterval  INTEGER,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct PeriodicStrategy {
    pub beginTime: OPTIONAL<Time>,
    pub windowSize: INTEGER,
    pub updateInterval: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl PeriodicStrategy {
    pub fn new(
        beginTime: OPTIONAL<Time>,
        windowSize: INTEGER,
        updateInterval: INTEGER,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PeriodicStrategy {
            beginTime,
            windowSize,
            updateInterval,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for PeriodicStrategy {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PeriodicStrategy(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PeriodicStrategy {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PeriodicStrategy(el)
    }
}

pub const _rctl1_components_for_PeriodicStrategy: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "beginTime",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "windowSize",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "updateInterval",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PeriodicStrategy: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PeriodicStrategy: &[ComponentSpec; 0] = &[];

pub fn _decode_PeriodicStrategy(el: &X690Element) -> ASN1Result<PeriodicStrategy> {
    |el_: &X690Element| -> ASN1Result<PeriodicStrategy> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PeriodicStrategy,
            _eal_components_for_PeriodicStrategy,
            _rctl2_components_for_PeriodicStrategy,
        )?;
        let beginTime: OPTIONAL<Time> = match _components.get("beginTime") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let windowSize = ber_decode_integer(_components.get("windowSize").unwrap())?;
        let updateInterval = ber_decode_integer(_components.get("updateInterval").unwrap())?;
        Ok(PeriodicStrategy {
            beginTime,
            windowSize,
            updateInterval,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PeriodicStrategy(value_: &PeriodicStrategy) -> ASN1Result<X690Element> {
    |value_: &PeriodicStrategy| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.beginTime {
            components_.push(_encode_Time(&v_)?);
        }
        components_.push(ber_encode_integer(&value_.windowSize)?);
        components_.push(ber_encode_integer(&value_.updateInterval)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Time  ::=  GeneralizedTime
/// ```
pub type Time = GeneralizedTime; // GeneralizedTime

pub fn _decode_Time(el: &X690Element) -> ASN1Result<Time> {
    ber_decode_generalized_time(&el)
}

pub fn _encode_Time(value_: &Time) -> ASN1Result<X690Element> {
    ber_encode_generalized_time(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// All-operations-consumer-initiated OPERATION ::= {requestShadowUpdate | updateShadow}
/// ```
///
///
pub fn All_operations_consumer_initiated() -> Vec<OPERATION> {
    Vec::from([requestShadowUpdate(), updateShadow()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// All-operations-supplier-initiated OPERATION ::= {coordinateShadowUpdate | updateShadow}
/// ```
///
///
pub fn All_operations_supplier_initiated() -> Vec<OPERATION> {
    Vec::from([coordinateShadowUpdate(), updateShadow()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// coordinateShadowUpdate OPERATION ::= {
///   ARGUMENT  CoordinateShadowUpdateArgument
///   RESULT    CoordinateShadowUpdateResult
///   ERRORS    {shadowError}
///   CODE      id-opcode-coordinateShadowUpdate
/// }
/// ```
///
///
pub fn coordinateShadowUpdate() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([shadowError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_coordinateShadowUpdate), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CoordinateShadowUpdateArgument  ::=
///   OPTIONALLY-PROTECTED { CoordinateShadowUpdateArgumentData }
/// ```
pub type CoordinateShadowUpdateArgument = OPTIONALLY_PROTECTED<CoordinateShadowUpdateArgumentData>; // DefinedType

pub fn _decode_CoordinateShadowUpdateArgument(
    el: &X690Element,
) -> ASN1Result<CoordinateShadowUpdateArgument> {
    _decode_OPTIONALLY_PROTECTED::<CoordinateShadowUpdateArgumentData>(
        _decode_CoordinateShadowUpdateArgumentData,
        &el,
    )
}

pub fn _encode_CoordinateShadowUpdateArgument(
    value_: &CoordinateShadowUpdateArgument,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<CoordinateShadowUpdateArgumentData>(
        _encode_CoordinateShadowUpdateArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CoordinateShadowUpdateArgumentData ::= [0]  SEQUENCE {
///   agreementID         AgreementID,
///   lastUpdate          Time OPTIONAL,
///   updateStrategy      CHOICE {
///     standard            ENUMERATED {
///       noChanges   (0),
///       incremental (1),
///       total       (2),
///       ...},
///     other               EXTERNAL,
///     ...},
///   securityParameters  SecurityParameters OPTIONAL,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CoordinateShadowUpdateArgumentData {
    pub agreementID: AgreementID,
    pub lastUpdate: OPTIONAL<Time>,
    pub updateStrategy: CoordinateShadowUpdateArgumentData_updateStrategy,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl CoordinateShadowUpdateArgumentData {
    pub fn new(
        agreementID: AgreementID,
        lastUpdate: OPTIONAL<Time>,
        updateStrategy: CoordinateShadowUpdateArgumentData_updateStrategy,
        securityParameters: OPTIONAL<SecurityParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CoordinateShadowUpdateArgumentData {
            agreementID,
            lastUpdate,
            updateStrategy,
            securityParameters,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CoordinateShadowUpdateArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CoordinateShadowUpdateArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateArgumentData(el)
    }
}

pub const _rctl1_components_for_CoordinateShadowUpdateArgumentData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new("updateStrategy", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CoordinateShadowUpdateArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CoordinateShadowUpdateArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_CoordinateShadowUpdateArgumentData(
    el: &X690Element,
) -> ASN1Result<CoordinateShadowUpdateArgumentData> {
    |el_: &X690Element| -> ASN1Result<CoordinateShadowUpdateArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CoordinateShadowUpdateArgumentData,
            _eal_components_for_CoordinateShadowUpdateArgumentData,
            _rctl2_components_for_CoordinateShadowUpdateArgumentData,
        )?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        let lastUpdate: OPTIONAL<Time> = match _components.get("lastUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let updateStrategy = _decode_CoordinateShadowUpdateArgumentData_updateStrategy(
            _components.get("updateStrategy").unwrap(),
        )?;
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(_decode_SecurityParameters(c_)?),
                _ => None,
            };
        Ok(CoordinateShadowUpdateArgumentData {
            agreementID,
            lastUpdate,
            updateStrategy,
            securityParameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CoordinateShadowUpdateArgumentData(
    value_: &CoordinateShadowUpdateArgumentData,
) -> ASN1Result<X690Element> {
    |v_1: &CoordinateShadowUpdateArgumentData| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &CoordinateShadowUpdateArgumentData| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(14);
            components_.push(_encode_AgreementID(&value_.agreementID)?);
            if let Some(v_) = &value_.lastUpdate {
                components_.push(_encode_Time(&v_)?);
            }
            components_.push(_encode_CoordinateShadowUpdateArgumentData_updateStrategy(
                &value_.updateStrategy,
            )?);
            if let Some(v_) = &value_.securityParameters {
                components_.push(_encode_SecurityParameters(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CoordinateShadowUpdateResult  ::=  CHOICE {
///   null         NULL,
///   information  OPTIONALLY-PROTECTED{ CoordinateShadowUpdateResultData },
///   ...}
/// ```
#[derive(Debug, Clone)]
pub enum CoordinateShadowUpdateResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED<CoordinateShadowUpdateResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CoordinateShadowUpdateResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CoordinateShadowUpdateResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateResult(el)
    }
}

pub fn _decode_CoordinateShadowUpdateResult(
    el: &X690Element,
) -> ASN1Result<CoordinateShadowUpdateResult> {
    |el: &X690Element| -> ASN1Result<CoordinateShadowUpdateResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => {
                ber_decode_null(&el)?;
                Ok(CoordinateShadowUpdateResult::null(()))
            }
            (TagClass::UNIVERSAL, 16) => Ok(CoordinateShadowUpdateResult::information(
                _decode_OPTIONALLY_PROTECTED::<CoordinateShadowUpdateResultData>(
                    _decode_CoordinateShadowUpdateResultData,
                    &el,
                )?,
            )),
            _ => Ok(CoordinateShadowUpdateResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CoordinateShadowUpdateResult(
    value_: &CoordinateShadowUpdateResult,
) -> ASN1Result<X690Element> {
    |value: &CoordinateShadowUpdateResult| -> ASN1Result<X690Element> {
        match value {
            CoordinateShadowUpdateResult::null(v) => ber_encode_null(&v),
            CoordinateShadowUpdateResult::information(v) => {
                _encode_OPTIONALLY_PROTECTED::<CoordinateShadowUpdateResultData>(
                    _encode_CoordinateShadowUpdateResultData,
                    &v,
                )
            }
            CoordinateShadowUpdateResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CoordinateShadowUpdateResultData ::= [0]  SEQUENCE {
///   agreementID  AgreementID,
///   lastUpdate   Time OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CoordinateShadowUpdateResultData {
    pub agreementID: AgreementID,
    pub lastUpdate: OPTIONAL<Time>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl CoordinateShadowUpdateResultData {
    pub fn new(
        agreementID: AgreementID,
        lastUpdate: OPTIONAL<Time>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        CoordinateShadowUpdateResultData {
            agreementID,
            lastUpdate,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for CoordinateShadowUpdateResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CoordinateShadowUpdateResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateResultData(el)
    }
}

pub const _rctl1_components_for_CoordinateShadowUpdateResultData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CoordinateShadowUpdateResultData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "performer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasDereferenced",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notification",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
];

pub const _eal_components_for_CoordinateShadowUpdateResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_CoordinateShadowUpdateResultData(
    el: &X690Element,
) -> ASN1Result<CoordinateShadowUpdateResultData> {
    |el_: &X690Element| -> ASN1Result<CoordinateShadowUpdateResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CoordinateShadowUpdateResultData,
            _eal_components_for_CoordinateShadowUpdateResultData,
            _rctl2_components_for_CoordinateShadowUpdateResultData,
        )?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        let lastUpdate: OPTIONAL<Time> = match _components.get("lastUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let performer: OPTIONAL<DistinguishedName> = match _components.get("performer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let aliasDereferenced: OPTIONAL<BOOLEAN> = match _components.get("aliasDereferenced") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let notification: OPTIONAL<Vec<Attribute>> = match _components.get("notification") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(CoordinateShadowUpdateResultData {
            agreementID,
            lastUpdate,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_CoordinateShadowUpdateResultData(
    value_: &CoordinateShadowUpdateResultData,
) -> ASN1Result<X690Element> {
    |v_1: &CoordinateShadowUpdateResultData| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &CoordinateShadowUpdateResultData| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(16);
            components_.push(_encode_AgreementID(&value_.agreementID)?);
            if let Some(v_) = &value_.lastUpdate {
                components_.push(_encode_Time(&v_)?);
            }
            if let Some(v_) = &value_.securityParameters {
                components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        30,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SecurityParameters(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.performer {
                components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        29,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.aliasDereferenced {
                if *v_ != CoordinateShadowUpdateResultData::_default_value_for_aliasDereferenced() {
                    components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            28,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                        ))
                    }(&v_)?);
                }
            }
            if let Some(v_) = &value_.notification {
                components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_Attribute(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// requestShadowUpdate OPERATION ::= {
///   ARGUMENT  RequestShadowUpdateArgument
///   RESULT    RequestShadowUpdateResult
///   ERRORS    {shadowError}
///   CODE      id-opcode-requestShadowUpdate
/// }
/// ```
///
///
pub fn requestShadowUpdate() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([shadowError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_requestShadowUpdate), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestShadowUpdateArgument  ::=  OPTIONALLY-PROTECTED { RequestShadowUpdateArgumentData }
/// ```
pub type RequestShadowUpdateArgument = OPTIONALLY_PROTECTED<RequestShadowUpdateArgumentData>; // DefinedType

pub fn _decode_RequestShadowUpdateArgument(
    el: &X690Element,
) -> ASN1Result<RequestShadowUpdateArgument> {
    _decode_OPTIONALLY_PROTECTED::<RequestShadowUpdateArgumentData>(
        _decode_RequestShadowUpdateArgumentData,
        &el,
    )
}

pub fn _encode_RequestShadowUpdateArgument(
    value_: &RequestShadowUpdateArgument,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<RequestShadowUpdateArgumentData>(
        _encode_RequestShadowUpdateArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestShadowUpdateArgumentData ::= [0]  SEQUENCE {
///   agreementID         AgreementID,
///   lastUpdate          Time OPTIONAL,
///   requestedStrategy   CHOICE {
///     standard  ENUMERATED {
///       incremental (1),
///       total       (2),
///       ...},
///     other     EXTERNAL,
///     ...},
///   securityParameters  SecurityParameters OPTIONAL,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RequestShadowUpdateArgumentData {
    pub agreementID: AgreementID,
    pub lastUpdate: OPTIONAL<Time>,
    pub requestedStrategy: RequestShadowUpdateArgumentData_requestedStrategy,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl RequestShadowUpdateArgumentData {
    pub fn new(
        agreementID: AgreementID,
        lastUpdate: OPTIONAL<Time>,
        requestedStrategy: RequestShadowUpdateArgumentData_requestedStrategy,
        securityParameters: OPTIONAL<SecurityParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RequestShadowUpdateArgumentData {
            agreementID,
            lastUpdate,
            requestedStrategy,
            securityParameters,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for RequestShadowUpdateArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RequestShadowUpdateArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateArgumentData(el)
    }
}

pub const _rctl1_components_for_RequestShadowUpdateArgumentData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new("requestedStrategy", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RequestShadowUpdateArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RequestShadowUpdateArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_RequestShadowUpdateArgumentData(
    el: &X690Element,
) -> ASN1Result<RequestShadowUpdateArgumentData> {
    |el_: &X690Element| -> ASN1Result<RequestShadowUpdateArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RequestShadowUpdateArgumentData,
            _eal_components_for_RequestShadowUpdateArgumentData,
            _rctl2_components_for_RequestShadowUpdateArgumentData,
        )?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        let lastUpdate: OPTIONAL<Time> = match _components.get("lastUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let requestedStrategy = _decode_RequestShadowUpdateArgumentData_requestedStrategy(
            _components.get("requestedStrategy").unwrap(),
        )?;
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(_decode_SecurityParameters(c_)?),
                _ => None,
            };
        Ok(RequestShadowUpdateArgumentData {
            agreementID,
            lastUpdate,
            requestedStrategy,
            securityParameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RequestShadowUpdateArgumentData(
    value_: &RequestShadowUpdateArgumentData,
) -> ASN1Result<X690Element> {
    |v_1: &RequestShadowUpdateArgumentData| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &RequestShadowUpdateArgumentData| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(14);
            components_.push(_encode_AgreementID(&value_.agreementID)?);
            if let Some(v_) = &value_.lastUpdate {
                components_.push(_encode_Time(&v_)?);
            }
            components_.push(_encode_RequestShadowUpdateArgumentData_requestedStrategy(
                &value_.requestedStrategy,
            )?);
            if let Some(v_) = &value_.securityParameters {
                components_.push(_encode_SecurityParameters(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestShadowUpdateResult  ::=  CHOICE {
///   null         NULL,
///   information OPTIONALLY-PROTECTED{ RequestShadowUpdateResultData },
///   ...
///   }
/// ```
#[derive(Debug, Clone)]
pub enum RequestShadowUpdateResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED<RequestShadowUpdateResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for RequestShadowUpdateResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RequestShadowUpdateResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateResult(el)
    }
}

pub fn _decode_RequestShadowUpdateResult(
    el: &X690Element,
) -> ASN1Result<RequestShadowUpdateResult> {
    |el: &X690Element| -> ASN1Result<RequestShadowUpdateResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => {
                ber_decode_null(&el)?;
                Ok(RequestShadowUpdateResult::null(()))
            }
            (TagClass::UNIVERSAL, 16) => Ok(RequestShadowUpdateResult::information(
                _decode_OPTIONALLY_PROTECTED::<RequestShadowUpdateResultData>(
                    _decode_RequestShadowUpdateResultData,
                    &el,
                )?,
            )),
            _ => Ok(RequestShadowUpdateResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_RequestShadowUpdateResult(
    value_: &RequestShadowUpdateResult,
) -> ASN1Result<X690Element> {
    |value: &RequestShadowUpdateResult| -> ASN1Result<X690Element> {
        match value {
            RequestShadowUpdateResult::null(v) => ber_encode_null(&v),
            RequestShadowUpdateResult::information(v) => {
                _encode_OPTIONALLY_PROTECTED::<RequestShadowUpdateResultData>(
                    _encode_RequestShadowUpdateResultData,
                    &v,
                )
            }
            RequestShadowUpdateResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestShadowUpdateResultData ::= [0]  SEQUENCE {
///   agreementID  AgreementID,
///   lastUpdate   Time OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RequestShadowUpdateResultData {
    pub agreementID: AgreementID,
    pub lastUpdate: OPTIONAL<Time>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl RequestShadowUpdateResultData {
    pub fn new(
        agreementID: AgreementID,
        lastUpdate: OPTIONAL<Time>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        RequestShadowUpdateResultData {
            agreementID,
            lastUpdate,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for RequestShadowUpdateResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RequestShadowUpdateResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateResultData(el)
    }
}

pub const _rctl1_components_for_RequestShadowUpdateResultData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RequestShadowUpdateResultData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "performer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasDereferenced",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notification",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
];

pub const _eal_components_for_RequestShadowUpdateResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_RequestShadowUpdateResultData(
    el: &X690Element,
) -> ASN1Result<RequestShadowUpdateResultData> {
    |el_: &X690Element| -> ASN1Result<RequestShadowUpdateResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RequestShadowUpdateResultData,
            _eal_components_for_RequestShadowUpdateResultData,
            _rctl2_components_for_RequestShadowUpdateResultData,
        )?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        let lastUpdate: OPTIONAL<Time> = match _components.get("lastUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let performer: OPTIONAL<DistinguishedName> = match _components.get("performer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let aliasDereferenced: OPTIONAL<BOOLEAN> = match _components.get("aliasDereferenced") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let notification: OPTIONAL<Vec<Attribute>> = match _components.get("notification") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(RequestShadowUpdateResultData {
            agreementID,
            lastUpdate,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_RequestShadowUpdateResultData(
    value_: &RequestShadowUpdateResultData,
) -> ASN1Result<X690Element> {
    |v_1: &RequestShadowUpdateResultData| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &RequestShadowUpdateResultData| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(16);
            components_.push(_encode_AgreementID(&value_.agreementID)?);
            if let Some(v_) = &value_.lastUpdate {
                components_.push(_encode_Time(&v_)?);
            }
            if let Some(v_) = &value_.securityParameters {
                components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        30,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SecurityParameters(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.performer {
                components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        29,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.aliasDereferenced {
                if *v_ != RequestShadowUpdateResultData::_default_value_for_aliasDereferenced() {
                    components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            28,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                        ))
                    }(&v_)?);
                }
            }
            if let Some(v_) = &value_.notification {
                components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_Attribute(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// updateShadow OPERATION ::= {
///   ARGUMENT  UpdateShadowArgument
///   RESULT    UpdateShadowResult
///   ERRORS    {shadowError}
///   CODE      id-opcode-updateShadow }
/// ```
///
///
pub fn updateShadow() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([shadowError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_updateShadow),   /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UpdateShadowArgument  ::=  OPTIONALLY-PROTECTED {UpdateShadowArgumentData }
/// ```
pub type UpdateShadowArgument = OPTIONALLY_PROTECTED<UpdateShadowArgumentData>; // DefinedType

pub fn _decode_UpdateShadowArgument(el: &X690Element) -> ASN1Result<UpdateShadowArgument> {
    _decode_OPTIONALLY_PROTECTED::<UpdateShadowArgumentData>(_decode_UpdateShadowArgumentData, &el)
}

pub fn _encode_UpdateShadowArgument(value_: &UpdateShadowArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<UpdateShadowArgumentData>(
        _encode_UpdateShadowArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UpdateShadowArgumentData ::= [0]  SEQUENCE {
///   agreementID         AgreementID,
///   updateTime          Time,
///   updateWindow        UpdateWindow OPTIONAL,
///   updatedInfo         RefreshInformation,
///   securityParameters  SecurityParameters OPTIONAL,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UpdateShadowArgumentData {
    pub agreementID: AgreementID,
    pub updateTime: Time,
    pub updateWindow: OPTIONAL<UpdateWindow>,
    pub updatedInfo: RefreshInformation,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl UpdateShadowArgumentData {
    pub fn new(
        agreementID: AgreementID,
        updateTime: Time,
        updateWindow: OPTIONAL<UpdateWindow>,
        updatedInfo: RefreshInformation,
        securityParameters: OPTIONAL<SecurityParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        UpdateShadowArgumentData {
            agreementID,
            updateTime,
            updateWindow,
            updatedInfo,
            securityParameters,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for UpdateShadowArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateShadowArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateShadowArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateShadowArgumentData(el)
    }
}

pub const _rctl1_components_for_UpdateShadowArgumentData: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "updateTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "updateWindow",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new("updatedInfo", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UpdateShadowArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UpdateShadowArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_UpdateShadowArgumentData(el: &X690Element) -> ASN1Result<UpdateShadowArgumentData> {
    |el_: &X690Element| -> ASN1Result<UpdateShadowArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UpdateShadowArgumentData,
            _eal_components_for_UpdateShadowArgumentData,
            _rctl2_components_for_UpdateShadowArgumentData,
        )?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        let updateTime = _decode_Time(_components.get("updateTime").unwrap())?;
        let updateWindow: OPTIONAL<UpdateWindow> = match _components.get("updateWindow") {
            Some(c_) => Some(_decode_UpdateWindow(c_)?),
            _ => None,
        };
        let updatedInfo = _decode_RefreshInformation(_components.get("updatedInfo").unwrap())?;
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(_decode_SecurityParameters(c_)?),
                _ => None,
            };
        Ok(UpdateShadowArgumentData {
            agreementID,
            updateTime,
            updateWindow,
            updatedInfo,
            securityParameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_UpdateShadowArgumentData(
    value_: &UpdateShadowArgumentData,
) -> ASN1Result<X690Element> {
    |v_1: &UpdateShadowArgumentData| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &UpdateShadowArgumentData| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(15);
            components_.push(_encode_AgreementID(&value_.agreementID)?);
            components_.push(_encode_Time(&value_.updateTime)?);
            if let Some(v_) = &value_.updateWindow {
                components_.push(_encode_UpdateWindow(&v_)?);
            }
            components_.push(_encode_RefreshInformation(&value_.updatedInfo)?);
            if let Some(v_) = &value_.securityParameters {
                components_.push(_encode_SecurityParameters(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UpdateShadowResult  ::=  CHOICE {
///   null         NULL,
///   information OPTIONALLY-PROTECTED{ UpdateShadowResultData },
///   ...}
/// ```
#[derive(Debug, Clone)]
pub enum UpdateShadowResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED<UpdateShadowResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for UpdateShadowResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateShadowResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateShadowResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateShadowResult(el)
    }
}

pub fn _decode_UpdateShadowResult(el: &X690Element) -> ASN1Result<UpdateShadowResult> {
    |el: &X690Element| -> ASN1Result<UpdateShadowResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => {
                ber_decode_null(&el)?;
                Ok(UpdateShadowResult::null(()))
            }
            (TagClass::UNIVERSAL, 16) => Ok(UpdateShadowResult::information(
                _decode_OPTIONALLY_PROTECTED::<UpdateShadowResultData>(
                    _decode_UpdateShadowResultData,
                    &el,
                )?,
            )),
            _ => Ok(UpdateShadowResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_UpdateShadowResult(value_: &UpdateShadowResult) -> ASN1Result<X690Element> {
    |value: &UpdateShadowResult| -> ASN1Result<X690Element> {
        match value {
            UpdateShadowResult::null(v) => ber_encode_null(&v),
            UpdateShadowResult::information(v) => _encode_OPTIONALLY_PROTECTED::<
                UpdateShadowResultData,
            >(_encode_UpdateShadowResultData, &v),
            UpdateShadowResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UpdateShadowResultData ::= [0]  SEQUENCE {
///   agreementID  AgreementID,
///   lastUpdate   Time OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UpdateShadowResultData {
    pub agreementID: AgreementID,
    pub lastUpdate: OPTIONAL<Time>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl UpdateShadowResultData {
    pub fn new(
        agreementID: AgreementID,
        lastUpdate: OPTIONAL<Time>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        UpdateShadowResultData {
            agreementID,
            lastUpdate,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for UpdateShadowResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateShadowResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateShadowResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateShadowResultData(el)
    }
}

pub const _rctl1_components_for_UpdateShadowResultData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UpdateShadowResultData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "performer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasDereferenced",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notification",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
];

pub const _eal_components_for_UpdateShadowResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_UpdateShadowResultData(el: &X690Element) -> ASN1Result<UpdateShadowResultData> {
    |el_: &X690Element| -> ASN1Result<UpdateShadowResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UpdateShadowResultData,
            _eal_components_for_UpdateShadowResultData,
            _rctl2_components_for_UpdateShadowResultData,
        )?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        let lastUpdate: OPTIONAL<Time> = match _components.get("lastUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let performer: OPTIONAL<DistinguishedName> = match _components.get("performer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let aliasDereferenced: OPTIONAL<BOOLEAN> = match _components.get("aliasDereferenced") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let notification: OPTIONAL<Vec<Attribute>> = match _components.get("notification") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(UpdateShadowResultData {
            agreementID,
            lastUpdate,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_UpdateShadowResultData(value_: &UpdateShadowResultData) -> ASN1Result<X690Element> {
    |v_1: &UpdateShadowResultData| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &UpdateShadowResultData| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(16);
            components_.push(_encode_AgreementID(&value_.agreementID)?);
            if let Some(v_) = &value_.lastUpdate {
                components_.push(_encode_Time(&v_)?);
            }
            if let Some(v_) = &value_.securityParameters {
                components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        30,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SecurityParameters(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.performer {
                components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        29,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.aliasDereferenced {
                if *v_ != UpdateShadowResultData::_default_value_for_aliasDereferenced() {
                    components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            28,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                        ))
                    }(&v_)?);
                }
            }
            if let Some(v_) = &value_.notification {
                components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_Attribute(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UpdateWindow ::= SEQUENCE {
///   start  Time,
///   stop   Time,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UpdateWindow {
    pub start: Time,
    pub stop: Time,
    pub _unrecognized: Vec<X690Element>,
}
impl UpdateWindow {
    pub fn new(start: Time, stop: Time, _unrecognized: Vec<X690Element>) -> Self {
        UpdateWindow {
            start,
            stop,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for UpdateWindow {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateWindow(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateWindow {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateWindow(el)
    }
}

pub const _rctl1_components_for_UpdateWindow: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "start",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "stop",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UpdateWindow: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UpdateWindow: &[ComponentSpec; 0] = &[];

pub fn _decode_UpdateWindow(el: &X690Element) -> ASN1Result<UpdateWindow> {
    |el_: &X690Element| -> ASN1Result<UpdateWindow> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UpdateWindow,
            _eal_components_for_UpdateWindow,
            _rctl2_components_for_UpdateWindow,
        )?;
        let start = _decode_Time(_components.get("start").unwrap())?;
        let stop = _decode_Time(_components.get("stop").unwrap())?;
        Ok(UpdateWindow {
            start,
            stop,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_UpdateWindow(value_: &UpdateWindow) -> ASN1Result<X690Element> {
    |value_: &UpdateWindow| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Time(&value_.start)?);
        components_.push(_encode_Time(&value_.stop)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RefreshInformation  ::=  CHOICE {
///   noRefresh      NULL,
///   total          [0]  TotalRefresh,
///   incremental    [1]  IncrementalRefresh,
///   otherStrategy       EXTERNAL,
///   ...}
/// ```
#[derive(Debug, Clone)]
pub enum RefreshInformation {
    noRefresh(NULL),
    total(TotalRefresh),
    incremental(IncrementalRefresh),
    otherStrategy(EXTERNAL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for RefreshInformation {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RefreshInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RefreshInformation {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RefreshInformation(el)
    }
}

pub fn _decode_RefreshInformation(el: &X690Element) -> ASN1Result<RefreshInformation> {
    |el: &X690Element| -> ASN1Result<RefreshInformation> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => {
                ber_decode_null(&el)?;
                Ok(RefreshInformation::noRefresh(()))
            }
            (TagClass::CONTEXT, 0) => Ok(RefreshInformation::total(_decode_TotalRefresh(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(RefreshInformation::incremental(
                _decode_IncrementalRefresh(&el)?,
            )),
            (TagClass::UNIVERSAL, 8) => {
                Ok(RefreshInformation::otherStrategy(ber_decode_external(&el)?))
            }
            _ => Ok(RefreshInformation::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_RefreshInformation(value_: &RefreshInformation) -> ASN1Result<X690Element> {
    |value: &RefreshInformation| -> ASN1Result<X690Element> {
        match value {
            RefreshInformation::noRefresh(v) => ber_encode_null(&v),
            RefreshInformation::total(v) => |v_1: &TotalRefresh| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_TotalRefresh(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            RefreshInformation::incremental(v) => {
                |v_1: &IncrementalRefresh| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_IncrementalRefresh(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            RefreshInformation::otherStrategy(v) => ber_encode_external(&v),
            RefreshInformation::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TotalRefresh ::= SEQUENCE {
///   sDSE     SDSEContent OPTIONAL,
///   subtree  SET SIZE (1..MAX) OF Subtree OPTIONAL,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TotalRefresh {
    pub sDSE: OPTIONAL<SDSEContent>,
    pub subtree: OPTIONAL<Vec<Subtree>>,
    pub _unrecognized: Vec<X690Element>,
}
impl TotalRefresh {
    pub fn new(
        sDSE: OPTIONAL<SDSEContent>,
        subtree: OPTIONAL<Vec<Subtree>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TotalRefresh {
            sDSE,
            subtree,
            _unrecognized,
        }
    }
}
impl Default for TotalRefresh {
    fn default() -> Self {
        TotalRefresh {
            sDSE: None,
            subtree: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for TotalRefresh {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TotalRefresh(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TotalRefresh {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TotalRefresh(el)
    }
}

pub const _rctl1_components_for_TotalRefresh: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "sDSE",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subtree",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TotalRefresh: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TotalRefresh: &[ComponentSpec; 0] = &[];

pub fn _decode_TotalRefresh(el: &X690Element) -> ASN1Result<TotalRefresh> {
    |el_: &X690Element| -> ASN1Result<TotalRefresh> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TotalRefresh,
            _eal_components_for_TotalRefresh,
            _rctl2_components_for_TotalRefresh,
        )?;
        let sDSE: OPTIONAL<SDSEContent> = match _components.get("sDSE") {
            Some(c_) => Some(_decode_SDSEContent(c_)?),
            _ => None,
        };
        let subtree: OPTIONAL<Vec<Subtree>> = match _components.get("subtree") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<Subtree>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Subtree> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_Subtree(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        Ok(TotalRefresh {
            sDSE,
            subtree,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TotalRefresh(value_: &TotalRefresh) -> ASN1Result<X690Element> {
    |value_: &TotalRefresh| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.sDSE {
            components_.push(_encode_SDSEContent(&v_)?);
        }
        if let Some(v_) = &value_.subtree {
            components_.push(|value_: &SET_OF<Subtree>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Subtree(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SDSEContent ::= SEQUENCE {
///   sDSEType          SDSEType,
///   subComplete       [0]  BOOLEAN DEFAULT FALSE,
///   attComplete       [1]  BOOLEAN OPTIONAL,
///   attributes        SET OF Attribute{{SupportedAttributes}},
///   attValIncomplete  SET OF AttributeType DEFAULT {},
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SDSEContent {
    pub sDSEType: SDSEType,
    pub subComplete: OPTIONAL<BOOLEAN>,
    pub attComplete: OPTIONAL<BOOLEAN>,
    pub attributes: Vec<Attribute>,
    pub attValIncomplete: OPTIONAL<Vec<AttributeType>>,
    pub _unrecognized: Vec<X690Element>,
}
impl SDSEContent {
    pub fn new(
        sDSEType: SDSEType,
        subComplete: OPTIONAL<BOOLEAN>,
        attComplete: OPTIONAL<BOOLEAN>,
        attributes: Vec<Attribute>,
        attValIncomplete: OPTIONAL<Vec<AttributeType>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SDSEContent {
            sDSEType,
            subComplete,
            attComplete,
            attributes,
            attValIncomplete,
            _unrecognized,
        }
    }
    pub fn _default_value_for_subComplete() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_attValIncomplete() -> Vec<AttributeType> {
        Vec::from([])
    }
}
impl TryFrom<X690Element> for SDSEContent {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SDSEContent(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SDSEContent {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SDSEContent(el)
    }
}

pub const _rctl1_components_for_SDSEContent: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "sDSEType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subComplete",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attComplete",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attValIncomplete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SDSEContent: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SDSEContent: &[ComponentSpec; 0] = &[];

pub fn _decode_SDSEContent(el: &X690Element) -> ASN1Result<SDSEContent> {
    |el_: &X690Element| -> ASN1Result<SDSEContent> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SDSEContent,
            _eal_components_for_SDSEContent,
            _rctl2_components_for_SDSEContent,
        )?;
        let sDSEType = _decode_SDSEType(_components.get("sDSEType").unwrap())?;
        let subComplete: OPTIONAL<BOOLEAN> = match _components.get("subComplete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let attComplete: OPTIONAL<BOOLEAN> = match _components.get("attComplete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let attributes = |el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_Attribute(el)?);
            }
            Ok(items)
        }(_components.get("attributes").unwrap())?;
        let attValIncomplete: OPTIONAL<Vec<AttributeType>> =
            match _components.get("attValIncomplete") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_AttributeType(el)?);
                    }
                    Ok(items)
                }(c_)?),
                _ => None,
            };
        Ok(SDSEContent {
            sDSEType,
            subComplete,
            attComplete,
            attributes,
            attValIncomplete,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SDSEContent(value_: &SDSEContent) -> ASN1Result<X690Element> {
    |value_: &SDSEContent| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_SDSEType(&value_.sDSEType)?);
        if let Some(v_) = &value_.subComplete {
            if *v_ != SDSEContent::_default_value_for_subComplete() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.attComplete {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_boolean(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(|value_: &SET_OF<Attribute>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_Attribute(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&value_.attributes)?);
        if let Some(v_) = &value_.attValIncomplete {
            if *v_ != SDSEContent::_default_value_for_attValIncomplete() {
                components_.push(
                    |value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_)?,
                );
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SDSEType  ::=  DSEType
/// ```
pub type SDSEType = DSEType; // DefinedType

pub fn _decode_SDSEType(el: &X690Element) -> ASN1Result<SDSEType> {
    _decode_DSEType(&el)
}

pub fn _encode_SDSEType(value_: &SDSEType) -> ASN1Result<X690Element> {
    _encode_DSEType(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Subtree ::= SEQUENCE {
///   rdn  RelativeDistinguishedName,
///   COMPONENTS OF TotalRefresh,
///   ...}
/// ```
///
///
// TODO: CHECK_RECURSIVE_DEFINITION
#[derive(Debug, Clone)]
pub struct Subtree {
    pub rdn: RelativeDistinguishedName,
    pub sDSE: OPTIONAL<SDSEContent>,     /* REPLICATED_COMPONENT */
    pub subtree: OPTIONAL<Vec<Subtree>>, /* REPLICATED_COMPONENT */
    pub _unrecognized: Vec<X690Element>,
}
impl Subtree {
    pub fn new(
        rdn: RelativeDistinguishedName,
        sDSE: OPTIONAL<SDSEContent>,     /* REPLICATED_COMPONENT */
        subtree: OPTIONAL<Vec<Subtree>>, /* REPLICATED_COMPONENT */
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Subtree {
            rdn,
            sDSE,
            subtree,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for Subtree {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Subtree(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Subtree {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Subtree(el)
    }
}

pub const _rctl1_components_for_Subtree: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "rdn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sDSE",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subtree",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Subtree: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Subtree: &[ComponentSpec; 0] = &[];

pub fn _decode_Subtree(el: &X690Element) -> ASN1Result<Subtree> {
    |el_: &X690Element| -> ASN1Result<Subtree> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Subtree,
            _eal_components_for_Subtree,
            _rctl2_components_for_Subtree,
        )?;
        let rdn = _decode_RelativeDistinguishedName(_components.get("rdn").unwrap())?;
        let sDSE: OPTIONAL<SDSEContent> = match _components.get("sDSE") {
            Some(c_) => Some(_decode_SDSEContent(c_)?),
            _ => None,
        };
        let subtree: OPTIONAL<Vec<Subtree>> = match _components.get("subtree") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<Subtree>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Subtree> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_Subtree(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        Ok(Subtree {
            rdn,
            sDSE,
            subtree,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Subtree(value_: &Subtree) -> ASN1Result<X690Element> {
    |value_: &Subtree| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_RelativeDistinguishedName(&value_.rdn)?);
        if let Some(v_) = &value_.sDSE {
            components_.push(_encode_SDSEContent(&v_)?);
        }
        if let Some(v_) = &value_.subtree {
            components_.push(|value_: &SET_OF<Subtree>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Subtree(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IncrementalRefresh  ::=  SEQUENCE OF IncrementalStepRefresh
/// ```
pub type IncrementalRefresh = Vec<IncrementalStepRefresh>; // SequenceOfType

pub fn _decode_IncrementalRefresh(el: &X690Element) -> ASN1Result<IncrementalRefresh> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<IncrementalStepRefresh>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<IncrementalStepRefresh> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_IncrementalStepRefresh(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_IncrementalRefresh(value_: &IncrementalRefresh) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<IncrementalStepRefresh>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_IncrementalStepRefresh(&v)?);
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
/// IncrementalStepRefresh ::= SEQUENCE {
///   sDSEChanges
///     CHOICE {add     [0]  SDSEContent,
///             remove  NULL,
///             modify  [1]  ContentChange,
///             ...} OPTIONAL,
///   subordinateUpdates  SEQUENCE SIZE (1..MAX) OF SubordinateChanges OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IncrementalStepRefresh {
    pub sDSEChanges: OPTIONAL<IncrementalStepRefresh_sDSEChanges>,
    pub subordinateUpdates: OPTIONAL<Vec<SubordinateChanges>>,
}
impl IncrementalStepRefresh {
    pub fn new(
        sDSEChanges: OPTIONAL<IncrementalStepRefresh_sDSEChanges>,
        subordinateUpdates: OPTIONAL<Vec<SubordinateChanges>>,
    ) -> Self {
        IncrementalStepRefresh {
            sDSEChanges,
            subordinateUpdates,
        }
    }
}
impl Default for IncrementalStepRefresh {
    fn default() -> Self {
        IncrementalStepRefresh {
            sDSEChanges: None,
            subordinateUpdates: None,
        }
    }
}
impl TryFrom<X690Element> for IncrementalStepRefresh {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IncrementalStepRefresh(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IncrementalStepRefresh {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IncrementalStepRefresh(el)
    }
}

pub const _rctl1_components_for_IncrementalStepRefresh: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "sDSEChanges",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::UNIVERSAL, 5)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "subordinateUpdates",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IncrementalStepRefresh: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IncrementalStepRefresh: &[ComponentSpec; 0] = &[];

pub fn _decode_IncrementalStepRefresh(el: &X690Element) -> ASN1Result<IncrementalStepRefresh> {
    |el_: &X690Element| -> ASN1Result<IncrementalStepRefresh> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IncrementalStepRefresh,
            _eal_components_for_IncrementalStepRefresh,
            _rctl2_components_for_IncrementalStepRefresh,
        )?;
        let sDSEChanges: OPTIONAL<IncrementalStepRefresh_sDSEChanges> =
            match _components.get("sDSEChanges") {
                Some(c_) => Some(_decode_IncrementalStepRefresh_sDSEChanges(c_)?),
                _ => None,
            };
        let subordinateUpdates: OPTIONAL<Vec<SubordinateChanges>> =
            match _components.get("subordinateUpdates") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<SubordinateChanges>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<SubordinateChanges> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_SubordinateChanges(el)?);
                        }
                        Ok(items)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(IncrementalStepRefresh {
            sDSEChanges,
            subordinateUpdates,
        })
    }(&el)
}

pub fn _encode_IncrementalStepRefresh(value_: &IncrementalStepRefresh) -> ASN1Result<X690Element> {
    |value_: &IncrementalStepRefresh| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.sDSEChanges {
            components_.push(_encode_IncrementalStepRefresh_sDSEChanges(&v_)?);
        }
        if let Some(v_) = &value_.subordinateUpdates {
            components_.push(
                |value_: &SEQUENCE_OF<SubordinateChanges>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_SubordinateChanges(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentChange ::= SEQUENCE {
///   rename
///     CHOICE {newRDN  RelativeDistinguishedName,
///             newDN   DistinguishedName} OPTIONAL,
///   attributeChanges
///     CHOICE {replace  [0]  SET SIZE (1..MAX) OF Attribute{{SupportedAttributes}},
///             changes  [1]  SEQUENCE SIZE (1..MAX) OF EntryModification} OPTIONAL,
///   sDSEType          SDSEType,
///   subComplete       [2]  BOOLEAN DEFAULT FALSE,
///   attComplete       [3]  BOOLEAN OPTIONAL,
///   attValIncomplete  SET OF AttributeType DEFAULT {},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ContentChange {
    pub rename: OPTIONAL<ContentChange_rename>,
    pub attributeChanges: OPTIONAL<ContentChange_attributeChanges>,
    pub sDSEType: SDSEType,
    pub subComplete: OPTIONAL<BOOLEAN>,
    pub attComplete: OPTIONAL<BOOLEAN>,
    pub attValIncomplete: OPTIONAL<Vec<AttributeType>>,
    pub _unrecognized: Vec<X690Element>,
}
impl ContentChange {
    pub fn new(
        rename: OPTIONAL<ContentChange_rename>,
        attributeChanges: OPTIONAL<ContentChange_attributeChanges>,
        sDSEType: SDSEType,
        subComplete: OPTIONAL<BOOLEAN>,
        attComplete: OPTIONAL<BOOLEAN>,
        attValIncomplete: OPTIONAL<Vec<AttributeType>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ContentChange {
            rename,
            attributeChanges,
            sDSEType,
            subComplete,
            attComplete,
            attValIncomplete,
            _unrecognized,
        }
    }
    pub fn _default_value_for_subComplete() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_attValIncomplete() -> Vec<AttributeType> {
        Vec::from([])
    }
}
impl TryFrom<X690Element> for ContentChange {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContentChange(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContentChange {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContentChange(el)
    }
}

pub const _rctl1_components_for_ContentChange: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "rename",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 17)),
            &TagSelector::tag((TagClass::UNIVERSAL, 16)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeChanges",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "sDSEType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subComplete",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attComplete",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attValIncomplete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContentChange: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContentChange: &[ComponentSpec; 0] = &[];

pub fn _decode_ContentChange(el: &X690Element) -> ASN1Result<ContentChange> {
    |el_: &X690Element| -> ASN1Result<ContentChange> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ContentChange,
            _eal_components_for_ContentChange,
            _rctl2_components_for_ContentChange,
        )?;
        let rename: OPTIONAL<ContentChange_rename> = match _components.get("rename") {
            Some(c_) => Some(_decode_ContentChange_rename(c_)?),
            _ => None,
        };
        let attributeChanges: OPTIONAL<ContentChange_attributeChanges> =
            match _components.get("attributeChanges") {
                Some(c_) => Some(_decode_ContentChange_attributeChanges(c_)?),
                _ => None,
            };
        let sDSEType = _decode_SDSEType(_components.get("sDSEType").unwrap())?;
        let subComplete: OPTIONAL<BOOLEAN> = match _components.get("subComplete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let attComplete: OPTIONAL<BOOLEAN> = match _components.get("attComplete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let attValIncomplete: OPTIONAL<Vec<AttributeType>> =
            match _components.get("attValIncomplete") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_AttributeType(el)?);
                    }
                    Ok(items)
                }(c_)?),
                _ => None,
            };
        Ok(ContentChange {
            rename,
            attributeChanges,
            sDSEType,
            subComplete,
            attComplete,
            attValIncomplete,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ContentChange(value_: &ContentChange) -> ASN1Result<X690Element> {
    |value_: &ContentChange| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.rename {
            components_.push(_encode_ContentChange_rename(&v_)?);
        }
        if let Some(v_) = &value_.attributeChanges {
            components_.push(_encode_ContentChange_attributeChanges(&v_)?);
        }
        components_.push(_encode_SDSEType(&value_.sDSEType)?);
        if let Some(v_) = &value_.subComplete {
            if *v_ != ContentChange::_default_value_for_subComplete() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.attComplete {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_boolean(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.attValIncomplete {
            if *v_ != ContentChange::_default_value_for_attValIncomplete() {
                components_.push(
                    |value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_)?,
                );
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubordinateChanges ::= SEQUENCE {
///   subordinate  RelativeDistinguishedName,
///   changes      IncrementalStepRefresh,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SubordinateChanges {
    pub subordinate: RelativeDistinguishedName,
    pub changes: IncrementalStepRefresh,
    pub _unrecognized: Vec<X690Element>,
}
impl SubordinateChanges {
    pub fn new(
        subordinate: RelativeDistinguishedName,
        changes: IncrementalStepRefresh,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SubordinateChanges {
            subordinate,
            changes,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SubordinateChanges {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubordinateChanges(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubordinateChanges {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubordinateChanges(el)
    }
}

pub const _rctl1_components_for_SubordinateChanges: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "subordinate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "changes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubordinateChanges: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubordinateChanges: &[ComponentSpec; 0] = &[];

pub fn _decode_SubordinateChanges(el: &X690Element) -> ASN1Result<SubordinateChanges> {
    |el_: &X690Element| -> ASN1Result<SubordinateChanges> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubordinateChanges,
            _eal_components_for_SubordinateChanges,
            _rctl2_components_for_SubordinateChanges,
        )?;
        let subordinate =
            _decode_RelativeDistinguishedName(_components.get("subordinate").unwrap())?;
        let changes = _decode_IncrementalStepRefresh(_components.get("changes").unwrap())?;
        Ok(SubordinateChanges {
            subordinate,
            changes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SubordinateChanges(value_: &SubordinateChanges) -> ASN1Result<X690Element> {
    |value_: &SubordinateChanges| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_RelativeDistinguishedName(&value_.subordinate)?);
        components_.push(_encode_IncrementalStepRefresh(&value_.changes)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowError ERROR ::= {
///   PARAMETER OPTIONALLY-PROTECTED-SEQ { ShadowErrorData }
///   CODE                               id-errcode-shadowError }
/// ```
///
///
pub fn shadowError() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_shadowError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowErrorData ::= SEQUENCE {
///   problem       ShadowProblem,
///   lastUpdate    Time OPTIONAL,
///   updateWindow  UpdateWindow OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ShadowErrorData {
    pub problem: ShadowProblem,
    pub lastUpdate: OPTIONAL<Time>,
    pub updateWindow: OPTIONAL<UpdateWindow>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ShadowErrorData {
    pub fn new(
        problem: ShadowProblem,
        lastUpdate: OPTIONAL<Time>,
        updateWindow: OPTIONAL<UpdateWindow>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ShadowErrorData {
            problem,
            lastUpdate,
            updateWindow,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ShadowErrorData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ShadowErrorData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ShadowErrorData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ShadowErrorData(el)
    }
}

pub const _rctl1_components_for_ShadowErrorData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "updateWindow",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ShadowErrorData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "performer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasDereferenced",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notification",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
];

pub const _eal_components_for_ShadowErrorData: &[ComponentSpec; 0] = &[];

pub fn _decode_ShadowErrorData(el: &X690Element) -> ASN1Result<ShadowErrorData> {
    |el_: &X690Element| -> ASN1Result<ShadowErrorData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ShadowErrorData,
            _eal_components_for_ShadowErrorData,
            _rctl2_components_for_ShadowErrorData,
        )?;
        let problem = _decode_ShadowProblem(_components.get("problem").unwrap())?;
        let lastUpdate: OPTIONAL<Time> = match _components.get("lastUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let updateWindow: OPTIONAL<UpdateWindow> = match _components.get("updateWindow") {
            Some(c_) => Some(_decode_UpdateWindow(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let performer: OPTIONAL<DistinguishedName> = match _components.get("performer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let aliasDereferenced: OPTIONAL<BOOLEAN> = match _components.get("aliasDereferenced") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let notification: OPTIONAL<Vec<Attribute>> = match _components.get("notification") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ShadowErrorData {
            problem,
            lastUpdate,
            updateWindow,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ShadowErrorData(value_: &ShadowErrorData) -> ASN1Result<X690Element> {
    |value_: &ShadowErrorData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(_encode_ShadowProblem(&value_.problem)?);
        if let Some(v_) = &value_.lastUpdate {
            components_.push(_encode_Time(&v_)?);
        }
        if let Some(v_) = &value_.updateWindow {
            components_.push(_encode_UpdateWindow(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.performer {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.aliasDereferenced {
            if *v_ != ShadowErrorData::_default_value_for_aliasDereferenced() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        28,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.notification {
            components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    27,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        Attribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Attribute(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowProblem  ::=  INTEGER {
///   invalidAgreementID         (1),
///   inactiveAgreement          (2),
///   invalidInformationReceived (3),
///   unsupportedStrategy        (4),
///   missedPrevious             (5),
///   fullUpdateRequired         (6),
///   unwillingToPerform         (7),
///   unsuitableTiming           (8),
///   updateAlreadyReceived      (9),
///   invalidSequencing          (10),
///   insufficientResources      (11) }
/// ```
pub type ShadowProblem = INTEGER;

pub const ShadowProblem_invalidAgreementID: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_inactiveAgreement: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_invalidInformationReceived: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_unsupportedStrategy: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_missedPrevious: i32 = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_fullUpdateRequired: i32 = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_unwillingToPerform: i32 = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_unsuitableTiming: i32 = 8; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_updateAlreadyReceived: i32 = 9; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_invalidSequencing: i32 = 10; /* LONG_NAMED_INTEGER_VALUE */

pub const ShadowProblem_insufficientResources: i32 = 11; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ShadowProblem(el: &X690Element) -> ASN1Result<ShadowProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_ShadowProblem(value_: &ShadowProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowOperationalBinding-roleA ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn shadowOperationalBinding_roleA() -> OP_BIND_ROLE {
    OP_BIND_ROLE {
        establish: Some(true), /* OBJECT_FIELD_SETTING */
        modify: Some(true),    /* OBJECT_FIELD_SETTING */
        terminate: Some(true), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowOperationalBinding-roleB ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn shadowOperationalBinding_roleB() -> OP_BIND_ROLE {
    OP_BIND_ROLE {
        establish: Some(true), /* OBJECT_FIELD_SETTING */
        modify: Some(true),    /* OBJECT_FIELD_SETTING */
        terminate: Some(true), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnitOfReplication-supplyContexts ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum UnitOfReplication_supplyContexts {
    allContexts(NULL),
    selectedContexts(Vec<OBJECT_IDENTIFIER>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for UnitOfReplication_supplyContexts {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UnitOfReplication_supplyContexts(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UnitOfReplication_supplyContexts {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UnitOfReplication_supplyContexts(el)
    }
}

pub fn _decode_UnitOfReplication_supplyContexts(
    el: &X690Element,
) -> ASN1Result<UnitOfReplication_supplyContexts> {
    |el: &X690Element| -> ASN1Result<UnitOfReplication_supplyContexts> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(UnitOfReplication_supplyContexts::allContexts(())),
            (TagClass::UNIVERSAL, 17) => Ok(UnitOfReplication_supplyContexts::selectedContexts(
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
                }(&el)?,
            )),
            _ => Ok(UnitOfReplication_supplyContexts::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_UnitOfReplication_supplyContexts(
    value_: &UnitOfReplication_supplyContexts,
) -> ASN1Result<X690Element> {
    |value: &UnitOfReplication_supplyContexts| -> ASN1Result<X690Element> {
        match value {
            UnitOfReplication_supplyContexts::allContexts(v) => ber_encode_null(&v),
            UnitOfReplication_supplyContexts::selectedContexts(v) => {
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
                }(&v)
            }
            UnitOfReplication_supplyContexts::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Knowledge-knowledgeType ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Knowledge_knowledgeType = ENUMERATED;

pub const Knowledge_knowledgeType_master: Knowledge_knowledgeType = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Knowledge_knowledgeType_shadow: Knowledge_knowledgeType = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Knowledge_knowledgeType_both: Knowledge_knowledgeType = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_Knowledge_knowledgeType(el: &X690Element) -> ASN1Result<Knowledge_knowledgeType> {
    ber_decode_enumerated(&el)
}

pub fn _encode_Knowledge_knowledgeType(
    value_: &Knowledge_knowledgeType,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CoordinateShadowUpdateArgumentData-updateStrategy-standard ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type CoordinateShadowUpdateArgumentData_updateStrategy_standard = ENUMERATED;

pub const CoordinateShadowUpdateArgumentData_updateStrategy_standard_noChanges:
    CoordinateShadowUpdateArgumentData_updateStrategy_standard = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CoordinateShadowUpdateArgumentData_updateStrategy_standard_incremental:
    CoordinateShadowUpdateArgumentData_updateStrategy_standard = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CoordinateShadowUpdateArgumentData_updateStrategy_standard_total:
    CoordinateShadowUpdateArgumentData_updateStrategy_standard = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CoordinateShadowUpdateArgumentData_updateStrategy_standard(
    el: &X690Element,
) -> ASN1Result<CoordinateShadowUpdateArgumentData_updateStrategy_standard> {
    ber_decode_enumerated(&el)
}

pub fn _encode_CoordinateShadowUpdateArgumentData_updateStrategy_standard(
    value_: &CoordinateShadowUpdateArgumentData_updateStrategy_standard,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CoordinateShadowUpdateArgumentData-updateStrategy ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CoordinateShadowUpdateArgumentData_updateStrategy {
    standard(CoordinateShadowUpdateArgumentData_updateStrategy_standard),
    other(EXTERNAL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CoordinateShadowUpdateArgumentData_updateStrategy {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateArgumentData_updateStrategy(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CoordinateShadowUpdateArgumentData_updateStrategy {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CoordinateShadowUpdateArgumentData_updateStrategy(el)
    }
}

pub fn _decode_CoordinateShadowUpdateArgumentData_updateStrategy(
    el: &X690Element,
) -> ASN1Result<CoordinateShadowUpdateArgumentData_updateStrategy> {
    |el: &X690Element| -> ASN1Result<CoordinateShadowUpdateArgumentData_updateStrategy> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 10) => {
                Ok(CoordinateShadowUpdateArgumentData_updateStrategy::standard(
                    _decode_CoordinateShadowUpdateArgumentData_updateStrategy_standard(&el)?,
                ))
            }
            (TagClass::UNIVERSAL, 8) => Ok(
                CoordinateShadowUpdateArgumentData_updateStrategy::other(ber_decode_external(&el)?),
            ),
            _ => Ok(CoordinateShadowUpdateArgumentData_updateStrategy::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CoordinateShadowUpdateArgumentData_updateStrategy(
    value_: &CoordinateShadowUpdateArgumentData_updateStrategy,
) -> ASN1Result<X690Element> {
    |value: &CoordinateShadowUpdateArgumentData_updateStrategy| -> ASN1Result<X690Element> {
        match value {
            CoordinateShadowUpdateArgumentData_updateStrategy::standard(v) => {
                _encode_CoordinateShadowUpdateArgumentData_updateStrategy_standard(&v)
            }
            CoordinateShadowUpdateArgumentData_updateStrategy::other(v) => ber_encode_external(&v),
            CoordinateShadowUpdateArgumentData_updateStrategy::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestShadowUpdateArgumentData-requestedStrategy-standard ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type RequestShadowUpdateArgumentData_requestedStrategy_standard = ENUMERATED;

pub const RequestShadowUpdateArgumentData_requestedStrategy_standard_incremental:
    RequestShadowUpdateArgumentData_requestedStrategy_standard = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const RequestShadowUpdateArgumentData_requestedStrategy_standard_total:
    RequestShadowUpdateArgumentData_requestedStrategy_standard = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_RequestShadowUpdateArgumentData_requestedStrategy_standard(
    el: &X690Element,
) -> ASN1Result<RequestShadowUpdateArgumentData_requestedStrategy_standard> {
    ber_decode_enumerated(&el)
}

pub fn _encode_RequestShadowUpdateArgumentData_requestedStrategy_standard(
    value_: &RequestShadowUpdateArgumentData_requestedStrategy_standard,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestShadowUpdateArgumentData-requestedStrategy ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum RequestShadowUpdateArgumentData_requestedStrategy {
    standard(RequestShadowUpdateArgumentData_requestedStrategy_standard),
    other(EXTERNAL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for RequestShadowUpdateArgumentData_requestedStrategy {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateArgumentData_requestedStrategy(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RequestShadowUpdateArgumentData_requestedStrategy {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RequestShadowUpdateArgumentData_requestedStrategy(el)
    }
}

pub fn _decode_RequestShadowUpdateArgumentData_requestedStrategy(
    el: &X690Element,
) -> ASN1Result<RequestShadowUpdateArgumentData_requestedStrategy> {
    |el: &X690Element| -> ASN1Result<RequestShadowUpdateArgumentData_requestedStrategy> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 10) => {
                Ok(RequestShadowUpdateArgumentData_requestedStrategy::standard(
                    _decode_RequestShadowUpdateArgumentData_requestedStrategy_standard(&el)?,
                ))
            }
            (TagClass::UNIVERSAL, 8) => Ok(
                RequestShadowUpdateArgumentData_requestedStrategy::other(ber_decode_external(&el)?),
            ),
            _ => Ok(RequestShadowUpdateArgumentData_requestedStrategy::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_RequestShadowUpdateArgumentData_requestedStrategy(
    value_: &RequestShadowUpdateArgumentData_requestedStrategy,
) -> ASN1Result<X690Element> {
    |value: &RequestShadowUpdateArgumentData_requestedStrategy| -> ASN1Result<X690Element> {
        match value {
            RequestShadowUpdateArgumentData_requestedStrategy::standard(v) => {
                _encode_RequestShadowUpdateArgumentData_requestedStrategy_standard(&v)
            }
            RequestShadowUpdateArgumentData_requestedStrategy::other(v) => ber_encode_external(&v),
            RequestShadowUpdateArgumentData_requestedStrategy::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IncrementalStepRefresh-sDSEChanges ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum IncrementalStepRefresh_sDSEChanges {
    add(SDSEContent),
    remove(NULL),
    modify(ContentChange),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for IncrementalStepRefresh_sDSEChanges {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IncrementalStepRefresh_sDSEChanges(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IncrementalStepRefresh_sDSEChanges {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IncrementalStepRefresh_sDSEChanges(el)
    }
}

pub fn _decode_IncrementalStepRefresh_sDSEChanges(
    el: &X690Element,
) -> ASN1Result<IncrementalStepRefresh_sDSEChanges> {
    |el: &X690Element| -> ASN1Result<IncrementalStepRefresh_sDSEChanges> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(IncrementalStepRefresh_sDSEChanges::add(
                _decode_SDSEContent(&el)?,
            )),
            (TagClass::UNIVERSAL, 5) => {
                ber_decode_null(&el)?;
                Ok(IncrementalStepRefresh_sDSEChanges::remove(()))
            }
            (TagClass::CONTEXT, 1) => Ok(IncrementalStepRefresh_sDSEChanges::modify(
                _decode_ContentChange(&el)?,
            )),
            _ => Ok(IncrementalStepRefresh_sDSEChanges::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_IncrementalStepRefresh_sDSEChanges(
    value_: &IncrementalStepRefresh_sDSEChanges,
) -> ASN1Result<X690Element> {
    |value: &IncrementalStepRefresh_sDSEChanges| -> ASN1Result<X690Element> {
        match value {
            IncrementalStepRefresh_sDSEChanges::add(v) => {
                |v_1: &SDSEContent| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_SDSEContent(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            IncrementalStepRefresh_sDSEChanges::remove(v) => ber_encode_null(&v),
            IncrementalStepRefresh_sDSEChanges::modify(v) => {
                |v_1: &ContentChange| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ContentChange(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            IncrementalStepRefresh_sDSEChanges::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentChange-rename ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ContentChange_rename {
    newRDN(RelativeDistinguishedName),
    newDN(DistinguishedName),
}

impl TryFrom<X690Element> for ContentChange_rename {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContentChange_rename(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContentChange_rename {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContentChange_rename(el)
    }
}

pub fn _decode_ContentChange_rename(el: &X690Element) -> ASN1Result<ContentChange_rename> {
    |el: &X690Element| -> ASN1Result<ContentChange_rename> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 17) => Ok(ContentChange_rename::newRDN(
                _decode_RelativeDistinguishedName(&el)?,
            )),
            (TagClass::UNIVERSAL, 16) => {
                Ok(ContentChange_rename::newDN(_decode_DistinguishedName(&el)?))
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_ContentChange_rename(value_: &ContentChange_rename) -> ASN1Result<X690Element> {
    |value: &ContentChange_rename| -> ASN1Result<X690Element> {
        match value {
            ContentChange_rename::newRDN(v) => _encode_RelativeDistinguishedName(&v),
            ContentChange_rename::newDN(v) => _encode_DistinguishedName(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentChange-attributeChanges ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ContentChange_attributeChanges {
    replace(Vec<Attribute>),
    changes(Vec<EntryModification>),
}

impl TryFrom<X690Element> for ContentChange_attributeChanges {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContentChange_attributeChanges(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContentChange_attributeChanges {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContentChange_attributeChanges(el)
    }
}

pub fn _decode_ContentChange_attributeChanges(
    el: &X690Element,
) -> ASN1Result<ContentChange_attributeChanges> {
    |el: &X690Element| -> ASN1Result<ContentChange_attributeChanges> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ContentChange_attributeChanges::replace(
                |el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(ContentChange_attributeChanges::changes(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<EntryModification>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<EntryModification> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_EntryModification(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_ContentChange_attributeChanges(
    value_: &ContentChange_attributeChanges,
) -> ASN1Result<X690Element> {
    |value: &ContentChange_attributeChanges| -> ASN1Result<X690Element> {
        match value {
            ContentChange_attributeChanges::replace(v) => {
                |v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                    let mut el_1 = |value_: &SET_OF<Attribute>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Attribute(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            ContentChange_attributeChanges::changes(v) => {
                |v_1: &Vec<EntryModification>| -> ASN1Result<X690Element> {
                    let mut el_1 =
                        |value_: &SEQUENCE_OF<EntryModification>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_EntryModification(&v)?);
                            }
                            Ok(X690Element::new(
                                TagClass::UNIVERSAL,
                                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                Arc::new(X690Encoding::Constructed(children)),
                            ))
                        }(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&value_)
}
