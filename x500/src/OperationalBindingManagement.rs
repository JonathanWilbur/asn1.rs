#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # OperationalBindingManagement
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `OperationalBindingManagement`.
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
use crate::DirectoryAbstractService::*;
use crate::DirectoryOSIProtocols::*;
use crate::DirectoryShadowAbstractService::*;
use crate::DistributedOperations::*;
use crate::EnhancedSecurity::*;
use crate::HierarchicalOperationalBindings::*;
use crate::InformationFramework::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// dSAOperationalBindingManagementBind OPERATION ::= dSABind
/// ```
///
///
pub fn dSAOperationalBindingManagementBind() -> OPERATION {
    dSABind()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OPERATIONAL-BINDING ::= CLASS {
///   &Agreement           ,
///   &Cooperation         OP-BINDING-COOP,
///   &both                OP-BIND-ROLE OPTIONAL,
///   &roleA               OP-BIND-ROLE OPTIONAL,
///   &roleB               OP-BIND-ROLE OPTIONAL,
///   &id                  OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   AGREEMENT            &Agreement
///   APPLICATION CONTEXTS &Cooperation
///   [SYMMETRIC           &both]
///   [ASYMMETRIC
///     [ROLE-A              &roleA]
///     [ROLE-B              &roleB]]
///   ID                   &id }
/// ```
///
#[derive(Debug)]
pub struct OPERATIONAL_BINDING {
    pub Cooperation: Vec<OP_BINDING_COOP>,
    pub both: OPTIONAL<OP_BIND_ROLE>,
    pub roleA: OPTIONAL<OP_BIND_ROLE>,
    pub roleB: OPTIONAL<OP_BIND_ROLE>,
    pub id: OBJECT_IDENTIFIER,
}
impl OPERATIONAL_BINDING {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OP-BINDING-COOP ::= CLASS {
///   &applContext  APPLICATION-CONTEXT,
///   &Operations   OPERATION OPTIONAL }
/// WITH SYNTAX {
///                 &applContext
///   [APPLIES TO   &Operations] }
/// ```
///
#[derive(Debug)]
pub struct OP_BINDING_COOP {
    pub applContext: APPLICATION_CONTEXT,
    pub Operations: OPTIONAL<Vec<OPERATION>>,
}
impl OP_BINDING_COOP {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OP-BIND-ROLE ::= CLASS {
///   &establish                BOOLEAN DEFAULT FALSE,
///   &EstablishParam,
///   &modify                   BOOLEAN DEFAULT FALSE,
///   &ModifyParam              OPTIONAL,
///   &terminate                BOOLEAN DEFAULT FALSE,
///   &TerminateParam           OPTIONAL }
/// WITH SYNTAX {
///   [ESTABLISHMENT-INITIATOR  &establish]
///   ESTABLISHMENT-PARAMETER   &EstablishParam
///   [MODIFICATION-INITIATOR   &modify]
///   [MODIFICATION-PARAMETER   &ModifyParam]
///   [TERMINATION-INITIATOR    &terminate]
///   [TERMINATION-PARAMETER    &TerminateParam] }
/// ```
///
#[derive(Debug)]
pub struct OP_BIND_ROLE {
    pub establish: OPTIONAL<BOOLEAN>,
    pub modify: OPTIONAL<BOOLEAN>,
    pub terminate: OPTIONAL<BOOLEAN>,
}
impl OP_BIND_ROLE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// establishOperationalBinding OPERATION ::= {
///   ARGUMENT   EstablishOperationalBindingArgument
///   RESULT     EstablishOperationalBindingResult
///   ERRORS     {operationalBindingError | securityError}
///   CODE       id-op-establishOperationalBinding }
/// ```
///
///
pub fn establishOperationalBinding() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([operationalBindingError(), securityError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_op_establishOperationalBinding), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EstablishOperationalBindingArgument  ::=
///   OPTIONALLY-PROTECTED-SEQ { EstablishOperationalBindingArgumentData }
/// ```
pub type EstablishOperationalBindingArgument =
    OPTIONALLY_PROTECTED_SEQ<EstablishOperationalBindingArgumentData>; // DefinedType

pub fn _decode_EstablishOperationalBindingArgument(
    el: &X690Element,
) -> ASN1Result<EstablishOperationalBindingArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<EstablishOperationalBindingArgumentData>(
        _decode_EstablishOperationalBindingArgumentData,
        &el,
    )
}

pub fn _encode_EstablishOperationalBindingArgument(
    value_: &EstablishOperationalBindingArgument,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<EstablishOperationalBindingArgumentData>(
        _encode_EstablishOperationalBindingArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EstablishOperationalBindingArgumentData ::= SEQUENCE {
///   bindingType        [0]  OPERATIONAL-BINDING.&id({OpBindingSet}),
///   bindingID          [1]  OperationalBindingID OPTIONAL,
///   accessPoint        [2]  AccessPoint,
///                -- symmetric, Role A initiates, or Role B initiates
///   initiator               CHOICE {
///     symmetric          [3]  OPERATIONAL-BINDING.&both.&EstablishParam
///                             ({OpBindingSet}{@bindingType}),
///     roleA-initiates    [4]  OPERATIONAL-BINDING.&roleA.&EstablishParam
///                             ({OpBindingSet}{@bindingType}),
///     roleB-initiates    [5]  OPERATIONAL-BINDING.&roleB.&EstablishParam
///                               ({OpBindingSet}{@bindingType})},
///   agreement          [6]  OPERATIONAL-BINDING.&Agreement
///                             ({OpBindingSet}{@bindingType}),
///   valid              [7]  Validity DEFAULT {},
///   securityParameters [8]  SecurityParameters OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EstablishOperationalBindingArgumentData {
    pub bindingType: OBJECT_IDENTIFIER,
    pub bindingID: OPTIONAL<OperationalBindingID>,
    pub accessPoint: AccessPoint,
    pub initiator: EstablishOperationalBindingArgumentData_initiator,
    pub agreement: X690Element,
    pub valid: OPTIONAL<Validity>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl EstablishOperationalBindingArgumentData {
    pub fn new(
        bindingType: OBJECT_IDENTIFIER,
        bindingID: OPTIONAL<OperationalBindingID>,
        accessPoint: AccessPoint,
        initiator: EstablishOperationalBindingArgumentData_initiator,
        agreement: X690Element,
        valid: OPTIONAL<Validity>,
        securityParameters: OPTIONAL<SecurityParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EstablishOperationalBindingArgumentData {
            bindingType,
            bindingID,
            accessPoint,
            initiator,
            agreement,
            valid,
            securityParameters,
            _unrecognized,
        }
    }
    pub fn _default_value_for_valid() -> Validity {
        Validity {
            validFrom: None,
            validUntil: None,
            ..Default::default()
        }
    }
}
impl TryFrom<X690Element> for EstablishOperationalBindingArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EstablishOperationalBindingArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingArgumentData(el)
    }
}

pub const _rctl1_components_for_EstablishOperationalBindingArgumentData: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "bindingType",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoint",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new("initiator", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "agreement",
        false,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "valid",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EstablishOperationalBindingArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EstablishOperationalBindingArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_EstablishOperationalBindingArgumentData(
    el: &X690Element,
) -> ASN1Result<EstablishOperationalBindingArgumentData> {
    |el_: &X690Element| -> ASN1Result<EstablishOperationalBindingArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EstablishOperationalBindingArgumentData,
            _eal_components_for_EstablishOperationalBindingArgumentData,
            _rctl2_components_for_EstablishOperationalBindingArgumentData,
        )?;
        let bindingType = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("bindingType").unwrap())?;
        let bindingID: OPTIONAL<OperationalBindingID> = match _components.get("bindingID") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationalBindingID> {
                Ok(_decode_OperationalBindingID(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let accessPoint = |el: &X690Element| -> ASN1Result<AccessPoint> {
            Ok(_decode_AccessPoint(&el.inner()?)?)
        }(_components.get("accessPoint").unwrap())?;
        let initiator = _decode_EstablishOperationalBindingArgumentData_initiator(
            _components.get("initiator").unwrap(),
        )?;
        let agreement =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("agreement").unwrap(),
            )?;
        let valid: OPTIONAL<Validity> = match _components.get("valid") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Validity> {
                Ok(_decode_Validity(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(EstablishOperationalBindingArgumentData {
            bindingType,
            bindingID,
            accessPoint,
            initiator,
            agreement,
            valid,
            securityParameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EstablishOperationalBindingArgumentData(
    value_: &EstablishOperationalBindingArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &EstablishOperationalBindingArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.bindingType)?);
        if let Some(v_) = &value_.bindingID {
            components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_OperationalBindingID(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &AccessPoint| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AccessPoint(&v_1)?))),
            ))
        }(&value_.accessPoint)?);
        components_.push(_encode_EstablishOperationalBindingArgumentData_initiator(
            &value_.initiator,
        )?);
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                6,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.agreement)?);
        if let Some(v_) = &value_.valid {
            if *v_ != EstablishOperationalBindingArgumentData::_default_value_for_valid() {
                components_.push(|v_1: &Validity| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        7,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Validity(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OpBindingSet OPERATIONAL-BINDING ::= {
///   shadowOperationalBinding |
///   hierarchicalOperationalBinding |
///   nonSpecificHierarchicalOperationalBinding }
/// ```
///
///
pub fn OpBindingSet() -> Vec<OPERATIONAL_BINDING> {
    Vec::from([
        shadowOperationalBinding(),
        hierarchicalOperationalBinding(),
        nonSpecificHierarchicalOperationalBinding(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OperationalBindingID ::= SEQUENCE {
///   identifier  INTEGER,
///   version     INTEGER,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct OperationalBindingID {
    pub identifier: INTEGER,
    pub version: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl OperationalBindingID {
    pub fn new(identifier: INTEGER, version: INTEGER, _unrecognized: Vec<X690Element>) -> Self {
        OperationalBindingID {
            identifier,
            version,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for OperationalBindingID {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OperationalBindingID(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OperationalBindingID {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OperationalBindingID(el)
    }
}

pub const _rctl1_components_for_OperationalBindingID: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OperationalBindingID: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OperationalBindingID: &[ComponentSpec; 0] = &[];

pub fn _decode_OperationalBindingID(el: &X690Element) -> ASN1Result<OperationalBindingID> {
    |el_: &X690Element| -> ASN1Result<OperationalBindingID> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_OperationalBindingID,
            _eal_components_for_OperationalBindingID,
            _rctl2_components_for_OperationalBindingID,
        )?;
        let identifier = ber_decode_integer(_components.get("identifier").unwrap())?;
        let version = ber_decode_integer(_components.get("version").unwrap())?;
        Ok(OperationalBindingID {
            identifier,
            version,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_OperationalBindingID(value_: &OperationalBindingID) -> ASN1Result<X690Element> {
    |value_: &OperationalBindingID| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_integer(&value_.identifier)?);
        components_.push(ber_encode_integer(&value_.version)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Validity ::= SEQUENCE {
///   validFrom            [0]  CHOICE {
///     now                  [0]  NULL,
///     time                 [1]  Time,
///     ...} DEFAULT now:NULL,
///   validUntil           [1]  CHOICE {
///     explicitTermination  [0]  NULL,
///     time                 [1]  Time,
///     ... } DEFAULT explicitTermination:NULL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct Validity {
    pub validFrom: OPTIONAL<Validity_validFrom>,
    pub validUntil: OPTIONAL<Validity_validUntil>,
    pub _unrecognized: Vec<X690Element>,
}
impl Validity {
    pub fn new(
        validFrom: OPTIONAL<Validity_validFrom>,
        validUntil: OPTIONAL<Validity_validUntil>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Validity {
            validFrom,
            validUntil,
            _unrecognized,
        }
    }
    pub fn _default_value_for_validFrom() -> Validity_validFrom {
        Validity_validFrom::now(())
    }
    pub fn _default_value_for_validUntil() -> Validity_validUntil {
        Validity_validUntil::explicitTermination(())
    }
}
impl Default for Validity {
    fn default() -> Self {
        Validity {
            validFrom: None,
            validUntil: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for Validity {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Validity(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Validity {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Validity(el)
    }
}

pub const _rctl1_components_for_Validity: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "validFrom",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "validUntil",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Validity: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Validity: &[ComponentSpec; 0] = &[];

pub fn _decode_Validity(el: &X690Element) -> ASN1Result<Validity> {
    |el_: &X690Element| -> ASN1Result<Validity> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Validity,
            _eal_components_for_Validity,
            _rctl2_components_for_Validity,
        )?;
        let validFrom: OPTIONAL<Validity_validFrom> = match _components.get("validFrom") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Validity_validFrom> {
                Ok(_decode_Validity_validFrom(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let validUntil: OPTIONAL<Validity_validUntil> = match _components.get("validUntil") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Validity_validUntil> {
                Ok(_decode_Validity_validUntil(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(Validity {
            validFrom,
            validUntil,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Validity(value_: &Validity) -> ASN1Result<X690Element> {
    |value_: &Validity| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.validFrom {
            if *v_ != Validity::_default_value_for_validFrom() {
                components_.push(|v_1: &Validity_validFrom| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_Validity_validFrom(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.validUntil {
            if *v_ != Validity::_default_value_for_validUntil() {
                components_.push(|v_1: &Validity_validUntil| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_Validity_validUntil(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Time  ::=  CHOICE {
///   utcTime          UTCTime,
///   generalizedTime  GeneralizedTime,
///   ... }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Time {
    utcTime(UTCTime),
    generalizedTime(GeneralizedTime),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Time {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Time(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Time {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Time(el)
    }
}

pub fn _decode_Time(el: &X690Element) -> ASN1Result<Time> {
    |el: &X690Element| -> ASN1Result<Time> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 23) => Ok(Time::utcTime(ber_decode_utc_time(&el)?)),
            (TagClass::UNIVERSAL, 24) => {
                Ok(Time::generalizedTime(ber_decode_generalized_time(&el)?))
            }
            _ => Ok(Time::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Time(value_: &Time) -> ASN1Result<X690Element> {
    |value: &Time| -> ASN1Result<X690Element> {
        match value {
            Time::utcTime(v) => ber_encode_utc_time(&v),
            Time::generalizedTime(v) => ber_encode_generalized_time(&v),
            Time::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EstablishOperationalBindingResult  ::=  OPTIONALLY-PROTECTED-SEQ { EstablishOperationalBindingResultData }
/// ```
pub type EstablishOperationalBindingResult =
    OPTIONALLY_PROTECTED_SEQ<EstablishOperationalBindingResultData>; // DefinedType

pub fn _decode_EstablishOperationalBindingResult(
    el: &X690Element,
) -> ASN1Result<EstablishOperationalBindingResult> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<EstablishOperationalBindingResultData>(
        _decode_EstablishOperationalBindingResultData,
        &el,
    )
}

pub fn _encode_EstablishOperationalBindingResult(
    value_: &EstablishOperationalBindingResult,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<EstablishOperationalBindingResultData>(
        _encode_EstablishOperationalBindingResultData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EstablishOperationalBindingResultData ::= SEQUENCE {
///   bindingType   [0]  OPERATIONAL-BINDING.&id({OpBindingSet}),
///   bindingID     [1]  OperationalBindingID OPTIONAL,
///   accessPoint   [2]  AccessPoint,
///   -- symmetric, Role A replies, or Role B replies
///   initiator          CHOICE {
///     symmetric     [3]  OPERATIONAL-BINDING.&both.&EstablishParam
///                          ({OpBindingSet}{@bindingType}),
///     roleA-replies [4]  OPERATIONAL-BINDING.&roleA.&EstablishParam
///                          ({OpBindingSet}{@bindingType}),
///     roleB-replies [5]  OPERATIONAL-BINDING.&roleB.&EstablishParam
///                          ({OpBindingSet}{@bindingType})},
///   ...,
///   ...,
///   COMPONENTS OF      CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EstablishOperationalBindingResultData {
    pub bindingType: OBJECT_IDENTIFIER,
    pub bindingID: OPTIONAL<OperationalBindingID>,
    pub accessPoint: AccessPoint,
    pub initiator: EstablishOperationalBindingResultData_initiator,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl EstablishOperationalBindingResultData {
    pub fn new(
        bindingType: OBJECT_IDENTIFIER,
        bindingID: OPTIONAL<OperationalBindingID>,
        accessPoint: AccessPoint,
        initiator: EstablishOperationalBindingResultData_initiator,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        EstablishOperationalBindingResultData {
            bindingType,
            bindingID,
            accessPoint,
            initiator,
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
impl TryFrom<X690Element> for EstablishOperationalBindingResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EstablishOperationalBindingResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingResultData(el)
    }
}

pub const _rctl1_components_for_EstablishOperationalBindingResultData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "bindingType",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoint",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new("initiator", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_EstablishOperationalBindingResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_EstablishOperationalBindingResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_EstablishOperationalBindingResultData(
    el: &X690Element,
) -> ASN1Result<EstablishOperationalBindingResultData> {
    |el_: &X690Element| -> ASN1Result<EstablishOperationalBindingResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EstablishOperationalBindingResultData,
            _eal_components_for_EstablishOperationalBindingResultData,
            _rctl2_components_for_EstablishOperationalBindingResultData,
        )?;
        let bindingType = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("bindingType").unwrap())?;
        let bindingID: OPTIONAL<OperationalBindingID> = match _components.get("bindingID") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationalBindingID> {
                Ok(_decode_OperationalBindingID(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let accessPoint = |el: &X690Element| -> ASN1Result<AccessPoint> {
            Ok(_decode_AccessPoint(&el.inner()?)?)
        }(_components.get("accessPoint").unwrap())?;
        let initiator = _decode_EstablishOperationalBindingResultData_initiator(
            _components.get("initiator").unwrap(),
        )?;
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
        Ok(EstablishOperationalBindingResultData {
            bindingType,
            bindingID,
            accessPoint,
            initiator,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_EstablishOperationalBindingResultData(
    value_: &EstablishOperationalBindingResultData,
) -> ASN1Result<X690Element> {
    |value_: &EstablishOperationalBindingResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.bindingType)?);
        if let Some(v_) = &value_.bindingID {
            components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_OperationalBindingID(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &AccessPoint| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AccessPoint(&v_1)?))),
            ))
        }(&value_.accessPoint)?);
        components_.push(_encode_EstablishOperationalBindingResultData_initiator(
            &value_.initiator,
        )?);
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
            if *v_ != EstablishOperationalBindingResultData::_default_value_for_aliasDereferenced()
            {
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
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// modifyOperationalBinding OPERATION ::= {
///   ARGUMENT  ModifyOperationalBindingArgument
///   RESULT    ModifyOperationalBindingResult
///   ERRORS    {operationalBindingError | securityError}
///   CODE      id-op-modifyOperationalBinding }
/// ```
///
///
pub fn modifyOperationalBinding() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([operationalBindingError(), securityError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_op_modifyOperationalBinding), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyOperationalBindingArgument  ::=
///   OPTIONALLY-PROTECTED-SEQ { ModifyOperationalBindingArgumentData }
/// ```
pub type ModifyOperationalBindingArgument =
    OPTIONALLY_PROTECTED_SEQ<ModifyOperationalBindingArgumentData>; // DefinedType

pub fn _decode_ModifyOperationalBindingArgument(
    el: &X690Element,
) -> ASN1Result<ModifyOperationalBindingArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<ModifyOperationalBindingArgumentData>(
        _decode_ModifyOperationalBindingArgumentData,
        &el,
    )
}

pub fn _encode_ModifyOperationalBindingArgument(
    value_: &ModifyOperationalBindingArgument,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<ModifyOperationalBindingArgumentData>(
        _encode_ModifyOperationalBindingArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyOperationalBindingArgumentData ::= SEQUENCE {
///   bindingType       [0]  OPERATIONAL-BINDING.&id({OpBindingSet}),
///   bindingID         [1]  OperationalBindingID,
///   accessPoint       [2]  AccessPoint OPTIONAL,
///   -- symmetric, Role A initiates, or Role B initiates
///   initiator              CHOICE {
///     symmetric         [3]  OPERATIONAL-BINDING.&both.&ModifyParam
///                           ({OpBindingSet}{@bindingType}),
///     roleA-initiates   [4]  OPERATIONAL-BINDING.&roleA.&ModifyParam
///                           ({OpBindingSet}{@bindingType}),
///     roleB-initiates   [5]  OPERATIONAL-BINDING.&roleB.&ModifyParam
///                           ({OpBindingSet}{@bindingType})} OPTIONAL,
///   newBindingID      [6]  OperationalBindingID,
///   newAgreement      [7]  OPERATIONAL-BINDING.&Agreement
///                        ({OpBindingSet}{@bindingType}) OPTIONAL,
///   valid               [8]  ModifiedValidity OPTIONAL,
///   securityParameters  [9]  SecurityParameters OPTIONAL,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyOperationalBindingArgumentData {
    pub bindingType: OBJECT_IDENTIFIER,
    pub bindingID: OperationalBindingID,
    pub accessPoint: OPTIONAL<AccessPoint>,
    pub initiator: OPTIONAL<ModifyOperationalBindingArgumentData_initiator>,
    pub newBindingID: OperationalBindingID,
    pub newAgreement: OPTIONAL<X690Element>,
    pub valid: OPTIONAL<ModifiedValidity>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl ModifyOperationalBindingArgumentData {
    pub fn new(
        bindingType: OBJECT_IDENTIFIER,
        bindingID: OperationalBindingID,
        accessPoint: OPTIONAL<AccessPoint>,
        initiator: OPTIONAL<ModifyOperationalBindingArgumentData_initiator>,
        newBindingID: OperationalBindingID,
        newAgreement: OPTIONAL<X690Element>,
        valid: OPTIONAL<ModifiedValidity>,
        securityParameters: OPTIONAL<SecurityParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ModifyOperationalBindingArgumentData {
            bindingType,
            bindingID,
            accessPoint,
            initiator,
            newBindingID,
            newAgreement,
            valid,
            securityParameters,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ModifyOperationalBindingArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyOperationalBindingArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingArgumentData(el)
    }
}

pub const _rctl1_components_for_ModifyOperationalBindingArgumentData: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "bindingType",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoint",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "initiator",
        true,
        TagSelector::or(&[&TagSelector::any, &TagSelector::any, &TagSelector::any]),
        None,
        None,
    ),
    ComponentSpec::new(
        "newBindingID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newAgreement",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "valid",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyOperationalBindingArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModifyOperationalBindingArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyOperationalBindingArgumentData(
    el: &X690Element,
) -> ASN1Result<ModifyOperationalBindingArgumentData> {
    |el_: &X690Element| -> ASN1Result<ModifyOperationalBindingArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyOperationalBindingArgumentData,
            _eal_components_for_ModifyOperationalBindingArgumentData,
            _rctl2_components_for_ModifyOperationalBindingArgumentData,
        )?;
        let bindingType = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("bindingType").unwrap())?;
        let bindingID = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
            Ok(_decode_OperationalBindingID(&el.inner()?)?)
        }(_components.get("bindingID").unwrap())?;
        let accessPoint: OPTIONAL<AccessPoint> = match _components.get("accessPoint") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AccessPoint> {
                Ok(_decode_AccessPoint(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let initiator: OPTIONAL<ModifyOperationalBindingArgumentData_initiator> =
            match _components.get("initiator") {
                Some(c_) => Some(_decode_ModifyOperationalBindingArgumentData_initiator(c_)?),
                _ => None,
            };
        let newBindingID = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
            Ok(_decode_OperationalBindingID(&el.inner()?)?)
        }(_components.get("newBindingID").unwrap())?;
        let newAgreement: OPTIONAL<X690Element> = match _components.get("newAgreement") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<X690Element> {
                Ok(x690_identity(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let valid: OPTIONAL<ModifiedValidity> = match _components.get("valid") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ModifiedValidity> {
                Ok(_decode_ModifiedValidity(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(ModifyOperationalBindingArgumentData {
            bindingType,
            bindingID,
            accessPoint,
            initiator,
            newBindingID,
            newAgreement,
            valid,
            securityParameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModifyOperationalBindingArgumentData(
    value_: &ModifyOperationalBindingArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &ModifyOperationalBindingArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.bindingType)?);
        components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_OperationalBindingID(&v_1)?,
                ))),
            ))
        }(&value_.bindingID)?);
        if let Some(v_) = &value_.accessPoint {
            components_.push(|v_1: &AccessPoint| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AccessPoint(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.initiator {
            components_.push(_encode_ModifyOperationalBindingArgumentData_initiator(&v_)?);
        }
        components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                6,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_OperationalBindingID(&v_1)?,
                ))),
            ))
        }(&value_.newBindingID)?);
        if let Some(v_) = &value_.newAgreement {
            components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.valid {
            components_.push(|v_1: &ModifiedValidity| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ModifiedValidity(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifiedValidity ::= SEQUENCE {
///   validFrom            [0]  CHOICE {
///     now                  [0]  NULL,
///     time                 [1]  Time,
///     ...} DEFAULT now:NULL,
///   validUntil           [1]  CHOICE {
///     explicitTermination  [0]  NULL,
///     time                 [1]  Time,
///     unchanged            [2]  NULL,
///     ... } DEFAULT unchanged:NULL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifiedValidity {
    pub validFrom: OPTIONAL<ModifiedValidity_validFrom>,
    pub validUntil: OPTIONAL<ModifiedValidity_validUntil>,
    pub _unrecognized: Vec<X690Element>,
}
impl ModifiedValidity {
    pub fn new(
        validFrom: OPTIONAL<ModifiedValidity_validFrom>,
        validUntil: OPTIONAL<ModifiedValidity_validUntil>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ModifiedValidity {
            validFrom,
            validUntil,
            _unrecognized,
        }
    }
    pub fn _default_value_for_validFrom() -> ModifiedValidity_validFrom {
        ModifiedValidity_validFrom::now(())
    }
    pub fn _default_value_for_validUntil() -> ModifiedValidity_validUntil {
        ModifiedValidity_validUntil::unchanged(())
    }
}
impl Default for ModifiedValidity {
    fn default() -> Self {
        ModifiedValidity {
            validFrom: None,
            validUntil: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ModifiedValidity {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifiedValidity(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifiedValidity {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifiedValidity(el)
    }
}

pub const _rctl1_components_for_ModifiedValidity: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "validFrom",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "validUntil",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifiedValidity: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModifiedValidity: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifiedValidity(el: &X690Element) -> ASN1Result<ModifiedValidity> {
    |el_: &X690Element| -> ASN1Result<ModifiedValidity> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifiedValidity,
            _eal_components_for_ModifiedValidity,
            _rctl2_components_for_ModifiedValidity,
        )?;
        let validFrom: OPTIONAL<ModifiedValidity_validFrom> = match _components.get("validFrom") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<ModifiedValidity_validFrom> {
                    Ok(_decode_ModifiedValidity_validFrom(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let validUntil: OPTIONAL<ModifiedValidity_validUntil> = match _components.get("validUntil")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<ModifiedValidity_validUntil> {
                    Ok(_decode_ModifiedValidity_validUntil(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        Ok(ModifiedValidity {
            validFrom,
            validUntil,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModifiedValidity(value_: &ModifiedValidity) -> ASN1Result<X690Element> {
    |value_: &ModifiedValidity| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.validFrom {
            if *v_ != ModifiedValidity::_default_value_for_validFrom() {
                components_.push(
                    |v_1: &ModifiedValidity_validFrom| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            0,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_ModifiedValidity_validFrom(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.validUntil {
            if *v_ != ModifiedValidity::_default_value_for_validUntil() {
                components_.push(
                    |v_1: &ModifiedValidity_validUntil| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            1,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_ModifiedValidity_validUntil(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyOperationalBindingResult  ::=  CHOICE {
///   null            NULL,
///   protected  [1]  OPTIONALLY-PROTECTED-SEQ{ ModifyOperationalBindingResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ModifyOperationalBindingResult {
    null(NULL),
    protected(OPTIONALLY_PROTECTED_SEQ<ModifyOperationalBindingResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ModifyOperationalBindingResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyOperationalBindingResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingResult(el)
    }
}

pub fn _decode_ModifyOperationalBindingResult(
    el: &X690Element,
) -> ASN1Result<ModifyOperationalBindingResult> {
    |el: &X690Element| -> ASN1Result<ModifyOperationalBindingResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => {
                Ok(ModifyOperationalBindingResult::null(ber_decode_null(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(
                ModifyOperationalBindingResult::protected(|el: &X690Element| -> ASN1Result<
                    OPTIONALLY_PROTECTED_SEQ<ModifyOperationalBindingResultData>,
                > {
                    Ok(_decode_OPTIONALLY_PROTECTED_SEQ::<
                        ModifyOperationalBindingResultData,
                    >(
                        _decode_ModifyOperationalBindingResultData, &el.inner()?
                    )?)
                }(&el)?),
            ),
            _ => Ok(ModifyOperationalBindingResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ModifyOperationalBindingResult(
    value_: &ModifyOperationalBindingResult,
) -> ASN1Result<X690Element> {
    |value: &ModifyOperationalBindingResult| -> ASN1Result<X690Element> {
        match value {
		ModifyOperationalBindingResult::null(v) => ber_encode_null(&v),
		ModifyOperationalBindingResult::protected(v) => |v_1: &OPTIONALLY_PROTECTED_SEQ<ModifyOperationalBindingResultData>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 1, Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OPTIONALLY_PROTECTED_SEQ::<ModifyOperationalBindingResultData>(_encode_ModifyOperationalBindingResultData, &v_1)?))))) }(&v),
		ModifyOperationalBindingResult::_unrecognized(el) => Ok(el.clone()),
	}
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyOperationalBindingResultData ::= SEQUENCE {
///     newBindingID    OperationalBindingID,
///     bindingType     OPERATIONAL-BINDING.&id({OpBindingSet}),
///     newAgreement    OPERATIONAL-BINDING.&Agreement ({OpBindingSet}{@.bindingType}),
///     valid           Validity OPTIONAL,
///     ...,
///     ...,
///     COMPONENTS OF   CommonResultsSeq
///     }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyOperationalBindingResultData {
    pub newBindingID: OperationalBindingID,
    pub bindingType: OBJECT_IDENTIFIER,
    pub newAgreement: X690Element,
    pub valid: OPTIONAL<Validity>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ModifyOperationalBindingResultData {
    pub fn new(
        newBindingID: OperationalBindingID,
        bindingType: OBJECT_IDENTIFIER,
        newAgreement: X690Element,
        valid: OPTIONAL<Validity>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ModifyOperationalBindingResultData {
            newBindingID,
            bindingType,
            newAgreement,
            valid,
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
impl TryFrom<X690Element> for ModifyOperationalBindingResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyOperationalBindingResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingResultData(el)
    }
}

pub const _rctl1_components_for_ModifyOperationalBindingResultData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "newBindingID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("newAgreement", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "valid",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyOperationalBindingResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ModifyOperationalBindingResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyOperationalBindingResultData(
    el: &X690Element,
) -> ASN1Result<ModifyOperationalBindingResultData> {
    |el_: &X690Element| -> ASN1Result<ModifyOperationalBindingResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyOperationalBindingResultData,
            _eal_components_for_ModifyOperationalBindingResultData,
            _rctl2_components_for_ModifyOperationalBindingResultData,
        )?;
        let newBindingID = _decode_OperationalBindingID(_components.get("newBindingID").unwrap())?;
        let bindingType = ber_decode_object_identifier(_components.get("bindingType").unwrap())?;
        let newAgreement = x690_identity(_components.get("newAgreement").unwrap())?;
        let valid: OPTIONAL<Validity> = match _components.get("valid") {
            Some(c_) => Some(_decode_Validity(c_)?),
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
        Ok(ModifyOperationalBindingResultData {
            newBindingID,
            bindingType,
            newAgreement,
            valid,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ModifyOperationalBindingResultData(
    value_: &ModifyOperationalBindingResultData,
) -> ASN1Result<X690Element> {
    |value_: &ModifyOperationalBindingResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        components_.push(_encode_OperationalBindingID(&value_.newBindingID)?);
        components_.push(ber_encode_object_identifier(&value_.bindingType)?);
        components_.push(x690_identity(&value_.newAgreement)?);
        if let Some(v_) = &value_.valid {
            components_.push(_encode_Validity(&v_)?);
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
            if *v_ != ModifyOperationalBindingResultData::_default_value_for_aliasDereferenced() {
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
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// terminateOperationalBinding OPERATION ::= {
///   ARGUMENT  TerminateOperationalBindingArgument
///   RESULT    TerminateOperationalBindingResult
///   ERRORS    {operationalBindingError | securityError}
///   CODE      id-op-terminateOperationalBinding }
/// ```
///
///
pub fn terminateOperationalBinding() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([operationalBindingError(), securityError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_op_terminateOperationalBinding), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminateOperationalBindingArgument  ::=
///   OPTIONALLY-PROTECTED-SEQ { TerminateOperationalBindingArgumentData }
/// ```
pub type TerminateOperationalBindingArgument =
    OPTIONALLY_PROTECTED_SEQ<TerminateOperationalBindingArgumentData>; // DefinedType

pub fn _decode_TerminateOperationalBindingArgument(
    el: &X690Element,
) -> ASN1Result<TerminateOperationalBindingArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<TerminateOperationalBindingArgumentData>(
        _decode_TerminateOperationalBindingArgumentData,
        &el,
    )
}

pub fn _encode_TerminateOperationalBindingArgument(
    value_: &TerminateOperationalBindingArgument,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<TerminateOperationalBindingArgumentData>(
        _encode_TerminateOperationalBindingArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminateOperationalBindingArgumentData ::= SEQUENCE {
///   bindingType         [0]  OPERATIONAL-BINDING.&id({OpBindingSet}),
///   bindingID           [1]  OperationalBindingID,
///   -- symmetric, Role A initiates, or Role B initiates
///   initiator                CHOICE {
///     symmetric           [2]  OPERATIONAL-BINDING.&both.&TerminateParam
///                             ({OpBindingSet}{@bindingType}),
///     roleA-initiates     [3]  OPERATIONAL-BINDING.&roleA.&TerminateParam
///                             ({OpBindingSet}{@bindingType}),
///     roleB-initiates     [4]  OPERATIONAL-BINDING.&roleB.&TerminateParam
///                             ({OpBindingSet}{@bindingType})} OPTIONAL,
///   terminateAt         [5]  Time OPTIONAL,
///   securityParameters  [6]  SecurityParameters OPTIONAL,
///   ...}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TerminateOperationalBindingArgumentData {
    pub bindingType: OBJECT_IDENTIFIER,
    pub bindingID: OperationalBindingID,
    pub initiator: OPTIONAL<TerminateOperationalBindingArgumentData_initiator>,
    pub terminateAt: OPTIONAL<Time>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl TerminateOperationalBindingArgumentData {
    pub fn new(
        bindingType: OBJECT_IDENTIFIER,
        bindingID: OperationalBindingID,
        initiator: OPTIONAL<TerminateOperationalBindingArgumentData_initiator>,
        terminateAt: OPTIONAL<Time>,
        securityParameters: OPTIONAL<SecurityParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TerminateOperationalBindingArgumentData {
            bindingType,
            bindingID,
            initiator,
            terminateAt,
            securityParameters,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TerminateOperationalBindingArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TerminateOperationalBindingArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingArgumentData(el)
    }
}

pub const _rctl1_components_for_TerminateOperationalBindingArgumentData: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "bindingType",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "initiator",
        true,
        TagSelector::or(&[&TagSelector::any, &TagSelector::any, &TagSelector::any]),
        None,
        None,
    ),
    ComponentSpec::new(
        "terminateAt",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TerminateOperationalBindingArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TerminateOperationalBindingArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_TerminateOperationalBindingArgumentData(
    el: &X690Element,
) -> ASN1Result<TerminateOperationalBindingArgumentData> {
    |el_: &X690Element| -> ASN1Result<TerminateOperationalBindingArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TerminateOperationalBindingArgumentData,
            _eal_components_for_TerminateOperationalBindingArgumentData,
            _rctl2_components_for_TerminateOperationalBindingArgumentData,
        )?;
        let bindingType = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("bindingType").unwrap())?;
        let bindingID = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
            Ok(_decode_OperationalBindingID(&el.inner()?)?)
        }(_components.get("bindingID").unwrap())?;
        let initiator: OPTIONAL<TerminateOperationalBindingArgumentData_initiator> =
            match _components.get("initiator") {
                Some(c_) => Some(_decode_TerminateOperationalBindingArgumentData_initiator(
                    c_,
                )?),
                _ => None,
            };
        let terminateAt: OPTIONAL<Time> = match _components.get("terminateAt") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Time> {
                Ok(_decode_Time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(TerminateOperationalBindingArgumentData {
            bindingType,
            bindingID,
            initiator,
            terminateAt,
            securityParameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TerminateOperationalBindingArgumentData(
    value_: &TerminateOperationalBindingArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &TerminateOperationalBindingArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.bindingType)?);
        components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_OperationalBindingID(&v_1)?,
                ))),
            ))
        }(&value_.bindingID)?);
        if let Some(v_) = &value_.initiator {
            components_.push(_encode_TerminateOperationalBindingArgumentData_initiator(
                &v_,
            )?);
        }
        if let Some(v_) = &value_.terminateAt {
            components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminateOperationalBindingResult  ::=  CHOICE {
///   null            NULL,
///   protected  [1]  OPTIONALLY-PROTECTED-SEQ{ TerminateOperationalBindingResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum TerminateOperationalBindingResult {
    null(NULL),
    protected(OPTIONALLY_PROTECTED_SEQ<TerminateOperationalBindingResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TerminateOperationalBindingResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TerminateOperationalBindingResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingResult(el)
    }
}

pub fn _decode_TerminateOperationalBindingResult(
    el: &X690Element,
) -> ASN1Result<TerminateOperationalBindingResult> {
    |el: &X690Element| -> ASN1Result<TerminateOperationalBindingResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(TerminateOperationalBindingResult::null(
                ber_decode_null(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(TerminateOperationalBindingResult::protected(
                |el: &X690Element| -> ASN1Result<
                    OPTIONALLY_PROTECTED_SEQ<TerminateOperationalBindingResultData>,
                > {
                    Ok(_decode_OPTIONALLY_PROTECTED_SEQ::<
                        TerminateOperationalBindingResultData,
                    >(
                        _decode_TerminateOperationalBindingResultData,
                        &el.inner()?,
                    )?)
                }(&el)?,
            )),
            _ => Ok(TerminateOperationalBindingResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TerminateOperationalBindingResult(
    value_: &TerminateOperationalBindingResult,
) -> ASN1Result<X690Element> {
    |value: &TerminateOperationalBindingResult| -> ASN1Result<X690Element> {
        match value {
		TerminateOperationalBindingResult::null(v) => ber_encode_null(&v),
		TerminateOperationalBindingResult::protected(v) => |v_1: &OPTIONALLY_PROTECTED_SEQ<TerminateOperationalBindingResultData>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 1, Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OPTIONALLY_PROTECTED_SEQ::<TerminateOperationalBindingResultData>(_encode_TerminateOperationalBindingResultData, &v_1)?))))) }(&v),
		TerminateOperationalBindingResult::_unrecognized(el) => Ok(el.clone()),
	}
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminateOperationalBindingResultData ::= SEQUENCE {
///   bindingID       OperationalBindingID,
///   bindingType     OPERATIONAL-BINDING.&id({OpBindingSet}),
///   terminateAt     GeneralizedTime OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF   CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TerminateOperationalBindingResultData {
    pub bindingID: OperationalBindingID,
    pub bindingType: OBJECT_IDENTIFIER,
    pub terminateAt: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl TerminateOperationalBindingResultData {
    pub fn new(
        bindingID: OperationalBindingID,
        bindingType: OBJECT_IDENTIFIER,
        terminateAt: OPTIONAL<GeneralizedTime>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        TerminateOperationalBindingResultData {
            bindingID,
            bindingType,
            terminateAt,
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
impl TryFrom<X690Element> for TerminateOperationalBindingResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TerminateOperationalBindingResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingResultData(el)
    }
}

pub const _rctl1_components_for_TerminateOperationalBindingResultData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "bindingID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "terminateAt",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TerminateOperationalBindingResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_TerminateOperationalBindingResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_TerminateOperationalBindingResultData(
    el: &X690Element,
) -> ASN1Result<TerminateOperationalBindingResultData> {
    |el_: &X690Element| -> ASN1Result<TerminateOperationalBindingResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TerminateOperationalBindingResultData,
            _eal_components_for_TerminateOperationalBindingResultData,
            _rctl2_components_for_TerminateOperationalBindingResultData,
        )?;
        let bindingID = _decode_OperationalBindingID(_components.get("bindingID").unwrap())?;
        let bindingType = ber_decode_object_identifier(_components.get("bindingType").unwrap())?;
        let terminateAt: OPTIONAL<GeneralizedTime> = match _components.get("terminateAt") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
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
        Ok(TerminateOperationalBindingResultData {
            bindingID,
            bindingType,
            terminateAt,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_TerminateOperationalBindingResultData(
    value_: &TerminateOperationalBindingResultData,
) -> ASN1Result<X690Element> {
    |value_: &TerminateOperationalBindingResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(_encode_OperationalBindingID(&value_.bindingID)?);
        components_.push(ber_encode_object_identifier(&value_.bindingType)?);
        if let Some(v_) = &value_.terminateAt {
            components_.push(ber_encode_generalized_time(&v_)?);
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
            if *v_ != TerminateOperationalBindingResultData::_default_value_for_aliasDereferenced()
            {
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
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalBindingError ERROR ::= {
///   PARAMETER OPTIONALLY-PROTECTED-SEQ  {OpBindingErrorParam}
///   CODE      id-err-operationalBindingError }
/// ```
///
///
pub fn operationalBindingError() -> ERROR {
    ERROR {
        errorCode: Some(id_err_operationalBindingError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OpBindingErrorParam ::= SEQUENCE {
///   problem            [0]  ENUMERATED {
///     invalidID              (0),
///     duplicateID            (1),
///     unsupportedBindingType (2),
///     notAllowedForRole      (3),
///     parametersMissing      (4),
///     roleAssignment         (5),
///     invalidStartTime       (6),
///     invalidEndTime         (7),
///     invalidAgreement       (8),
///     currentlyNotDecidable  (9),
///     modificationNotAllowed (10),
///     invalidBindingType     (11),
///     invalidNewID           (12),
///     ... },
///   bindingType        [1]  OPERATIONAL-BINDING.&id({OpBindingSet}) OPTIONAL,
///   agreementProposal  [2]  OPERATIONAL-BINDING.&Agreement
///                           ({OpBindingSet}{@bindingType}) OPTIONAL,
///   retryAt            [3]  Time OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF           CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct OpBindingErrorParam {
    pub problem: OpBindingErrorParam_problem,
    pub bindingType: OPTIONAL<OBJECT_IDENTIFIER>,
    pub agreementProposal: OPTIONAL<X690Element>,
    pub retryAt: OPTIONAL<Time>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl OpBindingErrorParam {
    pub fn new(
        problem: OpBindingErrorParam_problem,
        bindingType: OPTIONAL<OBJECT_IDENTIFIER>,
        agreementProposal: OPTIONAL<X690Element>,
        retryAt: OPTIONAL<Time>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        OpBindingErrorParam {
            problem,
            bindingType,
            agreementProposal,
            retryAt,
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
impl TryFrom<X690Element> for OpBindingErrorParam {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OpBindingErrorParam(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OpBindingErrorParam {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OpBindingErrorParam(el)
    }
}

pub const _rctl1_components_for_OpBindingErrorParam: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bindingType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "agreementProposal",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "retryAt",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OpBindingErrorParam: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_OpBindingErrorParam: &[ComponentSpec; 0] = &[];

pub fn _decode_OpBindingErrorParam(el: &X690Element) -> ASN1Result<OpBindingErrorParam> {
    |el_: &X690Element| -> ASN1Result<OpBindingErrorParam> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_OpBindingErrorParam,
            _eal_components_for_OpBindingErrorParam,
            _rctl2_components_for_OpBindingErrorParam,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<OpBindingErrorParam_problem> {
            Ok(_decode_OpBindingErrorParam_problem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
        let bindingType: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("bindingType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let agreementProposal: OPTIONAL<X690Element> = match _components.get("agreementProposal") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<X690Element> {
                Ok(x690_identity(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let retryAt: OPTIONAL<Time> = match _components.get("retryAt") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Time> {
                Ok(_decode_Time(&el.inner()?)?)
            }(c_)?),
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
        Ok(OpBindingErrorParam {
            problem,
            bindingType,
            agreementProposal,
            retryAt,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_OpBindingErrorParam(value_: &OpBindingErrorParam) -> ASN1Result<X690Element> {
    |value_: &OpBindingErrorParam| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        components_.push(
            |v_1: &OpBindingErrorParam_problem| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_OpBindingErrorParam_problem(&v_1)?,
                    ))),
                ))
            }(&value_.problem)?,
        );
        if let Some(v_) = &value_.bindingType {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.agreementProposal {
            components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.retryAt {
            components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v_)?);
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
            if *v_ != OpBindingErrorParam::_default_value_for_aliasDereferenced() {
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
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EstablishOperationalBindingArgumentData-initiator ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum EstablishOperationalBindingArgumentData_initiator {
    symmetric(X690Element),
    roleA_initiates(X690Element),
    roleB_initiates(X690Element),
}

impl TryFrom<X690Element> for EstablishOperationalBindingArgumentData_initiator {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingArgumentData_initiator(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EstablishOperationalBindingArgumentData_initiator {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingArgumentData_initiator(el)
    }
}

pub fn _decode_EstablishOperationalBindingArgumentData_initiator(
    el: &X690Element,
) -> ASN1Result<EstablishOperationalBindingArgumentData_initiator> {
    |el: &X690Element| -> ASN1Result<EstablishOperationalBindingArgumentData_initiator> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 3) => Ok(
                EstablishOperationalBindingArgumentData_initiator::symmetric(x690_identity(
                    &el.inner()?,
                )?),
            ),
            (TagClass::CONTEXT, 4) => Ok(
                EstablishOperationalBindingArgumentData_initiator::roleA_initiates(x690_identity(
                    &el.inner()?,
                )?),
            ),
            (TagClass::CONTEXT, 5) => Ok(
                EstablishOperationalBindingArgumentData_initiator::roleB_initiates(x690_identity(
                    &el.inner()?,
                )?),
            ),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_EstablishOperationalBindingArgumentData_initiator(
    value_: &EstablishOperationalBindingArgumentData_initiator,
) -> ASN1Result<X690Element> {
    |value: &EstablishOperationalBindingArgumentData_initiator| -> ASN1Result<X690Element> {
        match value {
            EstablishOperationalBindingArgumentData_initiator::symmetric(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EstablishOperationalBindingArgumentData_initiator::roleA_initiates(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EstablishOperationalBindingArgumentData_initiator::roleB_initiates(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Validity-validFrom ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Validity_validFrom {
    now(NULL),
    time(Time),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Validity_validFrom {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Validity_validFrom(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Validity_validFrom {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Validity_validFrom(el)
    }
}

pub fn _decode_Validity_validFrom(el: &X690Element) -> ASN1Result<Validity_validFrom> {
    |el: &X690Element| -> ASN1Result<Validity_validFrom> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Validity_validFrom::now(ber_decode_null(&el.inner()?)?)),
            (TagClass::CONTEXT, 1) => Ok(Validity_validFrom::time(
                |el2: &X690Element| -> ASN1Result<Time> { Ok(_decode_Time(&el2)?) }(&el.inner()?)?,
            )),
            _ => Ok(Validity_validFrom::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Validity_validFrom(value_: &Validity_validFrom) -> ASN1Result<X690Element> {
    |value: &Validity_validFrom| -> ASN1Result<X690Element> {
        match value {
            Validity_validFrom::now(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_null(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Validity_validFrom::time(v) => |v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v),
            Validity_validFrom::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Validity-validUntil ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Validity_validUntil {
    explicitTermination(NULL),
    time(Time),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Validity_validUntil {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Validity_validUntil(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Validity_validUntil {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Validity_validUntil(el)
    }
}

pub fn _decode_Validity_validUntil(el: &X690Element) -> ASN1Result<Validity_validUntil> {
    |el: &X690Element| -> ASN1Result<Validity_validUntil> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Validity_validUntil::explicitTermination(
                ber_decode_null(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(Validity_validUntil::time(
                |el2: &X690Element| -> ASN1Result<Time> { Ok(_decode_Time(&el2)?) }(&el.inner()?)?,
            )),
            _ => Ok(Validity_validUntil::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Validity_validUntil(value_: &Validity_validUntil) -> ASN1Result<X690Element> {
    |value: &Validity_validUntil| -> ASN1Result<X690Element> {
        match value {
            Validity_validUntil::explicitTermination(v) => {
                |v_1: &NULL| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_null(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            Validity_validUntil::time(v) => |v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v),
            Validity_validUntil::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EstablishOperationalBindingResultData-initiator ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum EstablishOperationalBindingResultData_initiator {
    symmetric(X690Element),
    roleA_replies(X690Element),
    roleB_replies(X690Element),
}

impl TryFrom<X690Element> for EstablishOperationalBindingResultData_initiator {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingResultData_initiator(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EstablishOperationalBindingResultData_initiator {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EstablishOperationalBindingResultData_initiator(el)
    }
}

pub fn _decode_EstablishOperationalBindingResultData_initiator(
    el: &X690Element,
) -> ASN1Result<EstablishOperationalBindingResultData_initiator> {
    |el: &X690Element| -> ASN1Result<EstablishOperationalBindingResultData_initiator> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 3) => {
                Ok(EstablishOperationalBindingResultData_initiator::symmetric(
                    x690_identity(&el.inner()?)?,
                ))
            }
            (TagClass::CONTEXT, 4) => Ok(
                EstablishOperationalBindingResultData_initiator::roleA_replies(x690_identity(
                    &el.inner()?,
                )?),
            ),
            (TagClass::CONTEXT, 5) => Ok(
                EstablishOperationalBindingResultData_initiator::roleB_replies(x690_identity(
                    &el.inner()?,
                )?),
            ),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_EstablishOperationalBindingResultData_initiator(
    value_: &EstablishOperationalBindingResultData_initiator,
) -> ASN1Result<X690Element> {
    |value: &EstablishOperationalBindingResultData_initiator| -> ASN1Result<X690Element> {
        match value {
            EstablishOperationalBindingResultData_initiator::symmetric(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EstablishOperationalBindingResultData_initiator::roleA_replies(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EstablishOperationalBindingResultData_initiator::roleB_replies(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyOperationalBindingArgumentData-initiator ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ModifyOperationalBindingArgumentData_initiator {
    symmetric(X690Element),
    roleA_initiates(X690Element),
    roleB_initiates(X690Element),
}

impl TryFrom<X690Element> for ModifyOperationalBindingArgumentData_initiator {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingArgumentData_initiator(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyOperationalBindingArgumentData_initiator {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyOperationalBindingArgumentData_initiator(el)
    }
}

pub fn _decode_ModifyOperationalBindingArgumentData_initiator(
    el: &X690Element,
) -> ASN1Result<ModifyOperationalBindingArgumentData_initiator> {
    |el: &X690Element| -> ASN1Result<ModifyOperationalBindingArgumentData_initiator> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 3) => {
                Ok(ModifyOperationalBindingArgumentData_initiator::symmetric(
                    x690_identity(&el.inner()?)?,
                ))
            }
            (TagClass::CONTEXT, 4) => Ok(
                ModifyOperationalBindingArgumentData_initiator::roleA_initiates(x690_identity(
                    &el.inner()?,
                )?),
            ),
            (TagClass::CONTEXT, 5) => Ok(
                ModifyOperationalBindingArgumentData_initiator::roleB_initiates(x690_identity(
                    &el.inner()?,
                )?),
            ),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_ModifyOperationalBindingArgumentData_initiator(
    value_: &ModifyOperationalBindingArgumentData_initiator,
) -> ASN1Result<X690Element> {
    |value: &ModifyOperationalBindingArgumentData_initiator| -> ASN1Result<X690Element> {
        match value {
            ModifyOperationalBindingArgumentData_initiator::symmetric(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ModifyOperationalBindingArgumentData_initiator::roleA_initiates(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ModifyOperationalBindingArgumentData_initiator::roleB_initiates(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifiedValidity-validFrom ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ModifiedValidity_validFrom {
    now(NULL),
    time(Time),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ModifiedValidity_validFrom {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifiedValidity_validFrom(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifiedValidity_validFrom {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifiedValidity_validFrom(el)
    }
}

pub fn _decode_ModifiedValidity_validFrom(
    el: &X690Element,
) -> ASN1Result<ModifiedValidity_validFrom> {
    |el: &X690Element| -> ASN1Result<ModifiedValidity_validFrom> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ModifiedValidity_validFrom::now(ber_decode_null(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(ModifiedValidity_validFrom::time(
                |el2: &X690Element| -> ASN1Result<Time> { Ok(_decode_Time(&el2)?) }(&el.inner()?)?,
            )),
            _ => Ok(ModifiedValidity_validFrom::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ModifiedValidity_validFrom(
    value_: &ModifiedValidity_validFrom,
) -> ASN1Result<X690Element> {
    |value: &ModifiedValidity_validFrom| -> ASN1Result<X690Element> {
        match value {
            ModifiedValidity_validFrom::now(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_null(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            ModifiedValidity_validFrom::time(v) => |v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v),
            ModifiedValidity_validFrom::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifiedValidity-validUntil ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ModifiedValidity_validUntil {
    explicitTermination(NULL),
    time(Time),
    unchanged(NULL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ModifiedValidity_validUntil {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifiedValidity_validUntil(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifiedValidity_validUntil {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifiedValidity_validUntil(el)
    }
}

pub fn _decode_ModifiedValidity_validUntil(
    el: &X690Element,
) -> ASN1Result<ModifiedValidity_validUntil> {
    |el: &X690Element| -> ASN1Result<ModifiedValidity_validUntil> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ModifiedValidity_validUntil::explicitTermination(
                ber_decode_null(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(ModifiedValidity_validUntil::time(
                |el2: &X690Element| -> ASN1Result<Time> { Ok(_decode_Time(&el2)?) }(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(ModifiedValidity_validUntil::unchanged(ber_decode_null(
                &el.inner()?,
            )?)),
            _ => Ok(ModifiedValidity_validUntil::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ModifiedValidity_validUntil(
    value_: &ModifiedValidity_validUntil,
) -> ASN1Result<X690Element> {
    |value: &ModifiedValidity_validUntil| -> ASN1Result<X690Element> {
        match value {
            ModifiedValidity_validUntil::explicitTermination(v) => {
                |v_1: &NULL| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_null(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ModifiedValidity_validUntil::time(v) => |v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v),
            ModifiedValidity_validUntil::unchanged(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_null(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            ModifiedValidity_validUntil::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminateOperationalBindingArgumentData-initiator ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TerminateOperationalBindingArgumentData_initiator {
    symmetric(X690Element),
    roleA_initiates(X690Element),
    roleB_initiates(X690Element),
}

impl TryFrom<X690Element> for TerminateOperationalBindingArgumentData_initiator {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingArgumentData_initiator(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TerminateOperationalBindingArgumentData_initiator {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TerminateOperationalBindingArgumentData_initiator(el)
    }
}

pub fn _decode_TerminateOperationalBindingArgumentData_initiator(
    el: &X690Element,
) -> ASN1Result<TerminateOperationalBindingArgumentData_initiator> {
    |el: &X690Element| -> ASN1Result<TerminateOperationalBindingArgumentData_initiator> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 2) => Ok(
                TerminateOperationalBindingArgumentData_initiator::symmetric(x690_identity(
                    &el.inner()?,
                )?),
            ),
            (TagClass::CONTEXT, 3) => Ok(
                TerminateOperationalBindingArgumentData_initiator::roleA_initiates(x690_identity(
                    &el.inner()?,
                )?),
            ),
            (TagClass::CONTEXT, 4) => Ok(
                TerminateOperationalBindingArgumentData_initiator::roleB_initiates(x690_identity(
                    &el.inner()?,
                )?),
            ),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_TerminateOperationalBindingArgumentData_initiator(
    value_: &TerminateOperationalBindingArgumentData_initiator,
) -> ASN1Result<X690Element> {
    |value: &TerminateOperationalBindingArgumentData_initiator| -> ASN1Result<X690Element> {
        match value {
            TerminateOperationalBindingArgumentData_initiator::symmetric(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            TerminateOperationalBindingArgumentData_initiator::roleA_initiates(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            TerminateOperationalBindingArgumentData_initiator::roleB_initiates(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OpBindingErrorParam-problem ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type OpBindingErrorParam_problem = ENUMERATED;

pub const OpBindingErrorParam_problem_invalidID: OpBindingErrorParam_problem = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_duplicateID: OpBindingErrorParam_problem = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_unsupportedBindingType: OpBindingErrorParam_problem = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_notAllowedForRole: OpBindingErrorParam_problem = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_parametersMissing: OpBindingErrorParam_problem = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_roleAssignment: OpBindingErrorParam_problem = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_invalidStartTime: OpBindingErrorParam_problem = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_invalidEndTime: OpBindingErrorParam_problem = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_invalidAgreement: OpBindingErrorParam_problem = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_currentlyNotDecidable: OpBindingErrorParam_problem = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_modificationNotAllowed: OpBindingErrorParam_problem = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_invalidBindingType: OpBindingErrorParam_problem = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OpBindingErrorParam_problem_invalidNewID: OpBindingErrorParam_problem = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_OpBindingErrorParam_problem(
    el: &X690Element,
) -> ASN1Result<OpBindingErrorParam_problem> {
    ber_decode_enumerated(&el)
}

pub fn _encode_OpBindingErrorParam_problem(
    value_: &OpBindingErrorParam_problem,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}
