#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DirectoryAbstractService
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DirectoryAbstractService`.
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
use crate::AttributeCertificateDefinitions::*;
use crate::AuthenticationFramework::*;
use crate::AuthenticationFramework::{
    CertificationPath, _decode_CertificationPath, _encode_CertificationPath,
};
use crate::CommonProtocolSpecification::*;
use crate::DirectoryShadowAbstractService::*;
use crate::DistributedOperations::*;
use crate::EnhancedSecurity::*;
// use crate::EnhancedSecurity::{
// 	OPTIONALLY_PROTECTED,
// 	OPTIONALLY_PROTECTED_SEQ,
// 	_encode_OPTIONALLY_PROTECTED,
// 	_encode_OPTIONALLY_PROTECTED_SEQ,
// 	_decode_OPTIONALLY_PROTECTED,
// 	_encode_OPTIONALLY_PROTECTED,
// };
use crate::InformationFramework::*;
use crate::PasswordPolicy::*;
use crate::SelectedAttributeTypes::*;
use crate::ServiceAdministration::*;
use crate::SpkmGssTokens::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use ldap::{LDAPMessage, _decode_LDAPMessage, _encode_LDAPMessage};
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommonArguments ::= SET {
///   serviceControls      [30]  ServiceControls    DEFAULT {},
///   securityParameters   [29]  SecurityParameters OPTIONAL,
///   requestor            [28]  DistinguishedName  OPTIONAL,
///   operationProgress    [27]  OperationProgress
///                              DEFAULT {nameResolutionPhase notStarted},
///   aliasedRDNs          [26]  INTEGER            OPTIONAL,
///   criticalExtensions   [25]  BIT STRING         OPTIONAL,
///   referenceType        [24]  ReferenceType      OPTIONAL,
///   entryOnly            [23]  BOOLEAN            DEFAULT TRUE,
///   exclusions           [22]  Exclusions         OPTIONAL,
///   nameResolveOnMaster  [21]  BOOLEAN            DEFAULT FALSE,
///   operationContexts    [20]  ContextSelection   OPTIONAL,
///   familyGrouping       [19]  FamilyGrouping     DEFAULT entryOnly,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CommonArguments {
    pub serviceControls: OPTIONAL<ServiceControls>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub requestor: OPTIONAL<DistinguishedName>,
    pub operationProgress: OPTIONAL<OperationProgress>,
    pub aliasedRDNs: OPTIONAL<INTEGER>,
    pub criticalExtensions: OPTIONAL<BIT_STRING>,
    pub referenceType: OPTIONAL<ReferenceType>,
    pub entryOnly: OPTIONAL<BOOLEAN>,
    pub exclusions: OPTIONAL<Exclusions>,
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,
    pub operationContexts: OPTIONAL<ContextSelection>,
    pub familyGrouping: OPTIONAL<FamilyGrouping>,
    pub _unrecognized: Vec<X690Element>,
}
impl CommonArguments {
    pub fn new(
        serviceControls: OPTIONAL<ServiceControls>,
        securityParameters: OPTIONAL<SecurityParameters>,
        requestor: OPTIONAL<DistinguishedName>,
        operationProgress: OPTIONAL<OperationProgress>,
        aliasedRDNs: OPTIONAL<INTEGER>,
        criticalExtensions: OPTIONAL<BIT_STRING>,
        referenceType: OPTIONAL<ReferenceType>,
        entryOnly: OPTIONAL<BOOLEAN>,
        exclusions: OPTIONAL<Exclusions>,
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,
        operationContexts: OPTIONAL<ContextSelection>,
        familyGrouping: OPTIONAL<FamilyGrouping>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CommonArguments {
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl Default for CommonArguments {
    fn default() -> Self {
        CommonArguments {
            serviceControls: None,
            securityParameters: None,
            requestor: None,
            operationProgress: None,
            aliasedRDNs: None,
            criticalExtensions: None,
            referenceType: None,
            entryOnly: None,
            exclusions: None,
            nameResolveOnMaster: None,
            operationContexts: None,
            familyGrouping: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CommonArguments {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CommonArguments(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CommonArguments {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CommonArguments(el)
    }
}

pub const _rctl1_components_for_CommonArguments: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CommonArguments: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CommonArguments: &[ComponentSpec; 0] = &[];

pub fn _decode_CommonArguments(el: &X690Element) -> ASN1Result<CommonArguments> {
    |el_: &X690Element| -> ASN1Result<CommonArguments> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_CommonArguments,
            _eal_components_for_CommonArguments,
            _rctl2_components_for_CommonArguments,
            130,
        )?;
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(CommonArguments {
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CommonArguments(value_: &CommonArguments) -> ASN1Result<X690Element> {
    |value_: &CommonArguments| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(22);
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != CommonArguments::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != CommonArguments::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != CommonArguments::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != CommonArguments::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// CommonArgumentsSeq ::= SEQUENCE {
///   serviceControls      [30]  ServiceControls    DEFAULT {},
///   securityParameters   [29]  SecurityParameters OPTIONAL,
///   requestor            [28]  DistinguishedName  OPTIONAL,
///   operationProgress    [27]  OperationProgress
///                              DEFAULT {nameResolutionPhase notStarted},
///   aliasedRDNs          [26]  INTEGER            OPTIONAL,
///   criticalExtensions   [25]  BIT STRING         OPTIONAL,
///   referenceType        [24]  ReferenceType      OPTIONAL,
///   entryOnly            [23]  BOOLEAN            DEFAULT TRUE,
///   exclusions           [22]  Exclusions         OPTIONAL,
///   nameResolveOnMaster  [21]  BOOLEAN            DEFAULT FALSE,
///   operationContexts    [20]  ContextSelection   OPTIONAL,
///   familyGrouping       [19]  FamilyGrouping     DEFAULT entryOnly,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CommonArgumentsSeq {
    pub serviceControls: OPTIONAL<ServiceControls>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub requestor: OPTIONAL<DistinguishedName>,
    pub operationProgress: OPTIONAL<OperationProgress>,
    pub aliasedRDNs: OPTIONAL<INTEGER>,
    pub criticalExtensions: OPTIONAL<BIT_STRING>,
    pub referenceType: OPTIONAL<ReferenceType>,
    pub entryOnly: OPTIONAL<BOOLEAN>,
    pub exclusions: OPTIONAL<Exclusions>,
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,
    pub operationContexts: OPTIONAL<ContextSelection>,
    pub familyGrouping: OPTIONAL<FamilyGrouping>,
    pub _unrecognized: Vec<X690Element>,
}
impl CommonArgumentsSeq {
    pub fn new(
        serviceControls: OPTIONAL<ServiceControls>,
        securityParameters: OPTIONAL<SecurityParameters>,
        requestor: OPTIONAL<DistinguishedName>,
        operationProgress: OPTIONAL<OperationProgress>,
        aliasedRDNs: OPTIONAL<INTEGER>,
        criticalExtensions: OPTIONAL<BIT_STRING>,
        referenceType: OPTIONAL<ReferenceType>,
        entryOnly: OPTIONAL<BOOLEAN>,
        exclusions: OPTIONAL<Exclusions>,
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,
        operationContexts: OPTIONAL<ContextSelection>,
        familyGrouping: OPTIONAL<FamilyGrouping>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CommonArgumentsSeq {
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl Default for CommonArgumentsSeq {
    fn default() -> Self {
        CommonArgumentsSeq {
            serviceControls: None,
            securityParameters: None,
            requestor: None,
            operationProgress: None,
            aliasedRDNs: None,
            criticalExtensions: None,
            referenceType: None,
            entryOnly: None,
            exclusions: None,
            nameResolveOnMaster: None,
            operationContexts: None,
            familyGrouping: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CommonArgumentsSeq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CommonArgumentsSeq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CommonArgumentsSeq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CommonArgumentsSeq(el)
    }
}

pub const _rctl1_components_for_CommonArgumentsSeq: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CommonArgumentsSeq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CommonArgumentsSeq: &[ComponentSpec; 0] = &[];

pub fn _decode_CommonArgumentsSeq(el: &X690Element) -> ASN1Result<CommonArgumentsSeq> {
    |el_: &X690Element| -> ASN1Result<CommonArgumentsSeq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CommonArgumentsSeq,
            _eal_components_for_CommonArgumentsSeq,
            _rctl2_components_for_CommonArgumentsSeq,
        )?;
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(CommonArgumentsSeq {
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CommonArgumentsSeq(value_: &CommonArgumentsSeq) -> ASN1Result<X690Element> {
    |value_: &CommonArgumentsSeq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(22);
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != CommonArgumentsSeq::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != CommonArgumentsSeq::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != CommonArgumentsSeq::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != CommonArgumentsSeq::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
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
/// FamilyGrouping  ::=  ENUMERATED {
///   entryOnly     (1),
///   compoundEntry (2),
///   strands       (3),
///   multiStrand   (4),
///   ... }
/// ```
pub type FamilyGrouping = ENUMERATED;

pub const FamilyGrouping_entryOnly: FamilyGrouping = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const FamilyGrouping_compoundEntry: FamilyGrouping = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const FamilyGrouping_strands: FamilyGrouping = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const FamilyGrouping_multiStrand: FamilyGrouping = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_FamilyGrouping(el: &X690Element) -> ASN1Result<FamilyGrouping> {
    ber_decode_enumerated(&el)
}

pub fn _encode_FamilyGrouping(value_: &FamilyGrouping) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommonResults ::= SET {
///   securityParameters  [30]  SecurityParameters  OPTIONAL,
///   performer           [29]  DistinguishedName   OPTIONAL,
///   aliasDereferenced   [28]  BOOLEAN             DEFAULT FALSE,
///   notification        [27]  SEQUENCE SIZE (1..MAX) OF Attribute
///                             {{SupportedAttributes}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CommonResults {
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub performer: OPTIONAL<DistinguishedName>,
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,
    pub notification: OPTIONAL<Vec<Attribute>>,
    pub _unrecognized: Vec<X690Element>,
}
impl CommonResults {
    pub fn new(
        securityParameters: OPTIONAL<SecurityParameters>,
        performer: OPTIONAL<DistinguishedName>,
        aliasDereferenced: OPTIONAL<BOOLEAN>,
        notification: OPTIONAL<Vec<Attribute>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CommonResults {
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
impl Default for CommonResults {
    fn default() -> Self {
        CommonResults {
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CommonResults {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CommonResults(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CommonResults {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CommonResults(el)
    }
}

pub const _rctl1_components_for_CommonResults: &[ComponentSpec; 4] = &[
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

pub const _rctl2_components_for_CommonResults: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CommonResults: &[ComponentSpec; 0] = &[];

pub fn _decode_CommonResults(el: &X690Element) -> ASN1Result<CommonResults> {
    |el_: &X690Element| -> ASN1Result<CommonResults> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_CommonResults,
            _eal_components_for_CommonResults,
            _rctl2_components_for_CommonResults,
            50,
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
        Ok(CommonResults {
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CommonResults(value_: &CommonResults) -> ASN1Result<X690Element> {
    |value_: &CommonResults| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
            if *v_ != CommonResults::_default_value_for_aliasDereferenced() {
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
/// CommonResultsSeq ::= SEQUENCE {
///   securityParameters  [30]  SecurityParameters OPTIONAL,
///   performer           [29]  DistinguishedName OPTIONAL,
///   aliasDereferenced   [28]  BOOLEAN DEFAULT FALSE,
///   notification        [27]  SEQUENCE SIZE (1..MAX) OF Attribute
///                             {{SupportedAttributes}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CommonResultsSeq {
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub performer: OPTIONAL<DistinguishedName>,
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,
    pub notification: OPTIONAL<Vec<Attribute>>,
    pub _unrecognized: Vec<X690Element>,
}
impl CommonResultsSeq {
    pub fn new(
        securityParameters: OPTIONAL<SecurityParameters>,
        performer: OPTIONAL<DistinguishedName>,
        aliasDereferenced: OPTIONAL<BOOLEAN>,
        notification: OPTIONAL<Vec<Attribute>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CommonResultsSeq {
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
impl Default for CommonResultsSeq {
    fn default() -> Self {
        CommonResultsSeq {
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CommonResultsSeq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CommonResultsSeq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CommonResultsSeq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CommonResultsSeq(el)
    }
}

pub const _rctl1_components_for_CommonResultsSeq: &[ComponentSpec; 4] = &[
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

pub const _rctl2_components_for_CommonResultsSeq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CommonResultsSeq: &[ComponentSpec; 0] = &[];

pub fn _decode_CommonResultsSeq(el: &X690Element) -> ASN1Result<CommonResultsSeq> {
    |el_: &X690Element| -> ASN1Result<CommonResultsSeq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CommonResultsSeq,
            _eal_components_for_CommonResultsSeq,
            _rctl2_components_for_CommonResultsSeq,
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
        Ok(CommonResultsSeq {
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CommonResultsSeq(value_: &CommonResultsSeq) -> ASN1Result<X690Element> {
    |value_: &CommonResultsSeq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
            if *v_ != CommonResultsSeq::_default_value_for_aliasDereferenced() {
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
/// ServiceControls ::= SET {
///   options              [0]  ServiceControlOptions DEFAULT {},
///   priority             [1]  INTEGER {low(0), medium(1), high(2)} DEFAULT medium,
///   timeLimit            [2]  INTEGER OPTIONAL,
///   sizeLimit            [3]  INTEGER OPTIONAL,
///   scopeOfReferral      [4]  INTEGER {dmd(0), country(1)} OPTIONAL,
///   attributeSizeLimit   [5]  INTEGER OPTIONAL,
///   manageDSAITPlaneRef  [6]  SEQUENCE {
///     dsaName                   Name,
///     agreementID               AgreementID,
///     ...} OPTIONAL,
///   serviceType          [7]  OBJECT IDENTIFIER OPTIONAL,
///   userClass            [8]  INTEGER OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ServiceControls {
    pub options: OPTIONAL<ServiceControlOptions>,
    pub priority: OPTIONAL<ServiceControls_priority>,
    pub timeLimit: OPTIONAL<INTEGER>,
    pub sizeLimit: OPTIONAL<INTEGER>,
    pub scopeOfReferral: OPTIONAL<ServiceControls_scopeOfReferral>,
    pub attributeSizeLimit: OPTIONAL<INTEGER>,
    pub manageDSAITPlaneRef: OPTIONAL<ServiceControls_manageDSAITPlaneRef>,
    pub serviceType: OPTIONAL<OBJECT_IDENTIFIER>,
    pub userClass: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl ServiceControls {
    pub fn new(
        options: OPTIONAL<ServiceControlOptions>,
        priority: OPTIONAL<ServiceControls_priority>,
        timeLimit: OPTIONAL<INTEGER>,
        sizeLimit: OPTIONAL<INTEGER>,
        scopeOfReferral: OPTIONAL<ServiceControls_scopeOfReferral>,
        attributeSizeLimit: OPTIONAL<INTEGER>,
        manageDSAITPlaneRef: OPTIONAL<ServiceControls_manageDSAITPlaneRef>,
        serviceType: OPTIONAL<OBJECT_IDENTIFIER>,
        userClass: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ServiceControls {
            options,
            priority,
            timeLimit,
            sizeLimit,
            scopeOfReferral,
            attributeSizeLimit,
            manageDSAITPlaneRef,
            serviceType,
            userClass,
            _unrecognized,
        }
    }
    pub fn _default_value_for_options() -> ServiceControlOptions {
        BIT_STRING::new()
    }
    pub fn _default_value_for_priority() -> ServiceControls_priority {
        vec![ServiceControls_priority_medium as u8]
    }
}
impl Default for ServiceControls {
    fn default() -> Self {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ServiceControls {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ServiceControls(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ServiceControls {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ServiceControls(el)
    }
}

pub const _rctl1_components_for_ServiceControls: &[ComponentSpec; 9] = &[
    ComponentSpec::new(
        "options",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "priority",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeLimit",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sizeLimit",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "scopeOfReferral",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeSizeLimit",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "manageDSAITPlaneRef",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serviceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userClass",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ServiceControls: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ServiceControls: &[ComponentSpec; 0] = &[];

pub fn _decode_ServiceControls(el: &X690Element) -> ASN1Result<ServiceControls> {
    |el_: &X690Element| -> ASN1Result<ServiceControls> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ServiceControls,
            _eal_components_for_ServiceControls,
            _rctl2_components_for_ServiceControls,
            100,
        )?;
        let options: OPTIONAL<ServiceControlOptions> = match _components.get("options") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControlOptions> {
                Ok(_decode_ServiceControlOptions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let priority: OPTIONAL<ServiceControls_priority> = match _components.get("priority") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls_priority> {
                Ok(_decode_ServiceControls_priority(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let timeLimit: OPTIONAL<INTEGER> = match _components.get("timeLimit") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let sizeLimit: OPTIONAL<INTEGER> = match _components.get("sizeLimit") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let scopeOfReferral: OPTIONAL<ServiceControls_scopeOfReferral> =
            match _components.get("scopeOfReferral") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<ServiceControls_scopeOfReferral> {
                        Ok(_decode_ServiceControls_scopeOfReferral(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let attributeSizeLimit: OPTIONAL<INTEGER> = match _components.get("attributeSizeLimit") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let manageDSAITPlaneRef: OPTIONAL<ServiceControls_manageDSAITPlaneRef> =
            match _components.get("manageDSAITPlaneRef") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<ServiceControls_manageDSAITPlaneRef> {
                        Ok(_decode_ServiceControls_manageDSAITPlaneRef(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let serviceType: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("serviceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let userClass: OPTIONAL<INTEGER> = match _components.get("userClass") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ServiceControls {
            options,
            priority,
            timeLimit,
            sizeLimit,
            scopeOfReferral,
            attributeSizeLimit,
            manageDSAITPlaneRef,
            serviceType,
            userClass,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ServiceControls(value_: &ServiceControls) -> ASN1Result<X690Element> {
    |value_: &ServiceControls| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(19);
        if let Some(v_) = &value_.options {
            if *v_ != ServiceControls::_default_value_for_options() {
                components_.push(|v_1: &ServiceControlOptions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_ServiceControlOptions(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.priority {
            if *v_ != ServiceControls::_default_value_for_priority() {
                components_.push(
                    |v_1: &ServiceControls_priority| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            1,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_ServiceControls_priority(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.timeLimit {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.sizeLimit {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.scopeOfReferral {
            components_.push(
                |v_1: &ServiceControls_scopeOfReferral| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_ServiceControls_scopeOfReferral(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.attributeSizeLimit {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.manageDSAITPlaneRef {
            components_.push(
                |v_1: &ServiceControls_manageDSAITPlaneRef| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        6,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_ServiceControls_manageDSAITPlaneRef(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.serviceType {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.userClass {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
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
/// ServiceControlOptions  ::=  BIT STRING {
///   preferChaining          (0),
///   chainingProhibited      (1),
///   localScope              (2),
///   dontUseCopy             (3),
///   dontDereferenceAliases  (4),
///   subentries              (5),
///   copyShallDo             (6),
///   partialNameResolution   (7),
///   manageDSAIT             (8),
///   noSubtypeMatch          (9),
///   noSubtypeSelection      (10),
///   countFamily             (11),
///   dontSelectFriends       (12),
///   dontMatchFriends        (13),
///   allowWriteableCopy      (14)}
/// ```
pub type ServiceControlOptions = BIT_STRING;

pub const ServiceControlOptions_preferChaining: BIT = 0; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_chainingProhibited: BIT = 1; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_localScope: BIT = 2; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_dontUseCopy: BIT = 3; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_dontDereferenceAliases: BIT = 4; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_subentries: BIT = 5; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_copyShallDo: BIT = 6; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_partialNameResolution: BIT = 7; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_manageDSAIT: BIT = 8; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_noSubtypeMatch: BIT = 9; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_noSubtypeSelection: BIT = 10; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_countFamily: BIT = 11; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_dontSelectFriends: BIT = 12; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_dontMatchFriends: BIT = 13; /* LONG_NAMED_BIT */

pub const ServiceControlOptions_allowWriteableCopy: BIT = 14; /* LONG_NAMED_BIT */

pub fn _decode_ServiceControlOptions(el: &X690Element) -> ASN1Result<ServiceControlOptions> {
    ber_decode_bit_string(&el)
}

pub fn _encode_ServiceControlOptions(value_: &ServiceControlOptions) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EntryInformationSelection ::= SET {
///   attributes                     CHOICE {
///     allUserAttributes         [0]  NULL,
///     select                    [1]  SET OF AttributeType
///     -- empty set implies no attributes are requested -- } DEFAULT allUserAttributes:NULL,
///     infoTypes               [2]  INTEGER {
///       attributeTypesOnly        (0),
///       attributeTypesAndValues   (1)} DEFAULT attributeTypesAndValues,
///   extraAttributes                CHOICE {
///     allOperationalAttributes  [3]  NULL,
///     select                    [4]  SET SIZE (1..MAX) OF AttributeType } OPTIONAL,
///   contextSelection               ContextSelection OPTIONAL,
///   returnContexts                 BOOLEAN DEFAULT FALSE,
///   familyReturn                   FamilyReturn DEFAULT
///                                    {memberSelect contributingEntriesOnly} }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EntryInformationSelection {
    pub attributes: OPTIONAL<EntryInformationSelection_attributes>,
    pub infoTypes: OPTIONAL<EntryInformationSelection_infoTypes>,
    pub extraAttributes: OPTIONAL<EntryInformationSelection_extraAttributes>,
    pub contextSelection: OPTIONAL<ContextSelection>,
    pub returnContexts: OPTIONAL<BOOLEAN>,
    pub familyReturn: OPTIONAL<FamilyReturn>,
}
impl EntryInformationSelection {
    pub fn new(
        attributes: OPTIONAL<EntryInformationSelection_attributes>,
        infoTypes: OPTIONAL<EntryInformationSelection_infoTypes>,
        extraAttributes: OPTIONAL<EntryInformationSelection_extraAttributes>,
        contextSelection: OPTIONAL<ContextSelection>,
        returnContexts: OPTIONAL<BOOLEAN>,
        familyReturn: OPTIONAL<FamilyReturn>,
    ) -> Self {
        EntryInformationSelection {
            attributes,
            infoTypes,
            extraAttributes,
            contextSelection,
            returnContexts,
            familyReturn,
        }
    }
    pub fn _default_value_for_attributes() -> EntryInformationSelection_attributes {
        EntryInformationSelection_attributes::allUserAttributes(())
    }
    pub fn _default_value_for_infoTypes() -> EntryInformationSelection_infoTypes {
        vec![EntryInformationSelection_infoTypes_attributeTypesAndValues as u8]
    }
    pub fn _default_value_for_returnContexts() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyReturn() -> FamilyReturn {
        FamilyReturn {
            memberSelect: FamilyReturn_memberSelect_contributingEntriesOnly,
            familySelect: None,
            _unrecognized: vec![],
        }
    }
}
impl Default for EntryInformationSelection {
    fn default() -> Self {
        EntryInformationSelection {
            attributes: None,
            infoTypes: None,
            extraAttributes: None,
            contextSelection: None,
            returnContexts: None,
            familyReturn: None,
        }
    }
}
impl TryFrom<X690Element> for EntryInformationSelection {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformationSelection(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryInformationSelection {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformationSelection(el)
    }
}

pub const _rctl1_components_for_EntryInformationSelection: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "attributes",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "infoTypes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extraAttributes",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 3)),
            &TagSelector::tag((TagClass::CONTEXT, 4)),
        ]),
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
        "returnContexts",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyReturn",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EntryInformationSelection: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EntryInformationSelection: &[ComponentSpec; 0] = &[];

pub fn _decode_EntryInformationSelection(
    el: &X690Element,
) -> ASN1Result<EntryInformationSelection> {
    |el_: &X690Element| -> ASN1Result<EntryInformationSelection> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_EntryInformationSelection,
            _eal_components_for_EntryInformationSelection,
            _rctl2_components_for_EntryInformationSelection,
            60,
        )?;
        let attributes: OPTIONAL<EntryInformationSelection_attributes> =
            match _components.get("attributes") {
                Some(c_) => Some(_decode_EntryInformationSelection_attributes(c_)?),
                _ => None,
            };
        let infoTypes: OPTIONAL<EntryInformationSelection_infoTypes> =
            match _components.get("infoTypes") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<EntryInformationSelection_infoTypes> {
                        Ok(_decode_EntryInformationSelection_infoTypes(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let extraAttributes: OPTIONAL<EntryInformationSelection_extraAttributes> =
            match _components.get("extraAttributes") {
                Some(c_) => Some(_decode_EntryInformationSelection_extraAttributes(c_)?),
                _ => None,
            };
        let contextSelection: OPTIONAL<ContextSelection> = match _components.get("contextSelection")
        {
            Some(c_) => Some(_decode_ContextSelection(c_)?),
            _ => None,
        };
        let returnContexts: OPTIONAL<BOOLEAN> = match _components.get("returnContexts") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let familyReturn: OPTIONAL<FamilyReturn> = match _components.get("familyReturn") {
            Some(c_) => Some(_decode_FamilyReturn(c_)?),
            _ => None,
        };
        Ok(EntryInformationSelection {
            attributes,
            infoTypes,
            extraAttributes,
            contextSelection,
            returnContexts,
            familyReturn,
        })
    }(&el)
}

pub fn _encode_EntryInformationSelection(
    value_: &EntryInformationSelection,
) -> ASN1Result<X690Element> {
    |value_: &EntryInformationSelection| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        if let Some(v_) = &value_.attributes {
            if *v_ != EntryInformationSelection::_default_value_for_attributes() {
                components_.push(_encode_EntryInformationSelection_attributes(&v_)?);
            }
        }
        if let Some(v_) = &value_.infoTypes {
            if *v_ != EntryInformationSelection::_default_value_for_infoTypes() {
                components_.push(
                    |v_1: &EntryInformationSelection_infoTypes| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            2,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_EntryInformationSelection_infoTypes(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.extraAttributes {
            components_.push(_encode_EntryInformationSelection_extraAttributes(&v_)?);
        }
        if let Some(v_) = &value_.contextSelection {
            components_.push(_encode_ContextSelection(&v_)?);
        }
        if let Some(v_) = &value_.returnContexts {
            if *v_ != EntryInformationSelection::_default_value_for_returnContexts() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.familyReturn {
            if *v_ != EntryInformationSelection::_default_value_for_familyReturn() {
                components_.push(_encode_FamilyReturn(&v_)?);
            }
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
/// ContextSelection  ::=  CHOICE {
///   allContexts       NULL,
///   selectedContexts  SET SIZE (1..MAX) OF TypeAndContextAssertion,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ContextSelection {
    allContexts(NULL),
    selectedContexts(Vec<TypeAndContextAssertion>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ContextSelection {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContextSelection(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContextSelection {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContextSelection(el)
    }
}

pub fn _decode_ContextSelection(el: &X690Element) -> ASN1Result<ContextSelection> {
    |el: &X690Element| -> ASN1Result<ContextSelection> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(ContextSelection::allContexts(())),
            (TagClass::UNIVERSAL, 17) => Ok(ContextSelection::selectedContexts(
                |el: &X690Element| -> ASN1Result<SET_OF<TypeAndContextAssertion>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<TypeAndContextAssertion> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_TypeAndContextAssertion(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            _ => Ok(ContextSelection::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ContextSelection(value_: &ContextSelection) -> ASN1Result<X690Element> {
    |value: &ContextSelection| -> ASN1Result<X690Element> {
        match value {
            ContextSelection::allContexts(v) => ber_encode_null(&v),
            ContextSelection::selectedContexts(v) => {
                |value_: &SET_OF<TypeAndContextAssertion>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_TypeAndContextAssertion(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v)
            }
            ContextSelection::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TypeAndContextAssertion ::= SEQUENCE {
///   type               AttributeType,
///   contextAssertions  CHOICE {
///     preference         SEQUENCE OF ContextAssertion,
///     all                SET OF ContextAssertion,
///     ...},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TypeAndContextAssertion {
    pub type_: AttributeType,
    pub contextAssertions: TypeAndContextAssertion_contextAssertions,
    pub _unrecognized: Vec<X690Element>,
}
impl TypeAndContextAssertion {
    pub fn new(
        type_: AttributeType,
        contextAssertions: TypeAndContextAssertion_contextAssertions,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TypeAndContextAssertion {
            type_,
            contextAssertions,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TypeAndContextAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TypeAndContextAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TypeAndContextAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TypeAndContextAssertion(el)
    }
}

pub const _rctl1_components_for_TypeAndContextAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("contextAssertions", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_TypeAndContextAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TypeAndContextAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_TypeAndContextAssertion(el: &X690Element) -> ASN1Result<TypeAndContextAssertion> {
    |el_: &X690Element| -> ASN1Result<TypeAndContextAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TypeAndContextAssertion,
            _eal_components_for_TypeAndContextAssertion,
            _rctl2_components_for_TypeAndContextAssertion,
        )?;
        let type_ = _decode_AttributeType(_components.get("type").unwrap())?;
        let contextAssertions = _decode_TypeAndContextAssertion_contextAssertions(
            _components.get("contextAssertions").unwrap(),
        )?;
        Ok(TypeAndContextAssertion {
            type_,
            contextAssertions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TypeAndContextAssertion(
    value_: &TypeAndContextAssertion,
) -> ASN1Result<X690Element> {
    |value_: &TypeAndContextAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeType(&value_.type_)?);
        components_.push(_encode_TypeAndContextAssertion_contextAssertions(
            &value_.contextAssertions,
        )?);
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
/// FamilyReturn ::= SEQUENCE {
///   memberSelect   ENUMERATED {
///     contributingEntriesOnly   (1),
///     participatingEntriesOnly  (2),
///     compoundEntry             (3),
///     ...},
///   familySelect   SEQUENCE SIZE (1..MAX) OF OBJECT-CLASS.&id OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyReturn {
    pub memberSelect: FamilyReturn_memberSelect,
    pub familySelect: OPTIONAL<SEQUENCE_OF<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl FamilyReturn {
    pub fn new(
        memberSelect: FamilyReturn_memberSelect,
        familySelect: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FamilyReturn {
            memberSelect,
            familySelect,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for FamilyReturn {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyReturn(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FamilyReturn {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyReturn(el)
    }
}

pub const _rctl1_components_for_FamilyReturn: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "memberSelect",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familySelect",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FamilyReturn: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FamilyReturn: &[ComponentSpec; 0] = &[];

pub fn _decode_FamilyReturn(el: &X690Element) -> ASN1Result<FamilyReturn> {
    |el_: &X690Element| -> ASN1Result<FamilyReturn> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FamilyReturn,
            _eal_components_for_FamilyReturn,
            _rctl2_components_for_FamilyReturn,
        )?;
        let memberSelect =
            _decode_FamilyReturn_memberSelect(_components.get("memberSelect").unwrap())?;
        let familySelect: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("familySelect") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<OBJECT_IDENTIFIER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<OBJECT_IDENTIFIER> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_object_identifier(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        Ok(FamilyReturn {
            memberSelect,
            familySelect,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FamilyReturn(value_: &FamilyReturn) -> ASN1Result<X690Element> {
    |value_: &FamilyReturn| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_FamilyReturn_memberSelect(&value_.memberSelect)?);
        if let Some(v_) = &value_.familySelect {
            components_.push(
                |value_: &SEQUENCE_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_object_identifier(&v)?);
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
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EntryInformation ::= SEQUENCE {
///   name                  Name,
///   fromEntry             BOOLEAN DEFAULT TRUE,
///   information           SET SIZE (1..MAX) OF CHOICE {
///     attributeType         AttributeType,
///     attribute             Attribute{{SupportedAttributes}},
///     ...} OPTIONAL,
///   incompleteEntry  [3]  BOOLEAN DEFAULT FALSE,
///   partialName      [4]  BOOLEAN DEFAULT FALSE,
///   derivedEntry     [5]  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EntryInformation {
    pub name: Name,
    pub fromEntry: OPTIONAL<BOOLEAN>,
    pub information: OPTIONAL<Vec<EntryInformation_information_Item>>,
    pub incompleteEntry: OPTIONAL<BOOLEAN>,
    pub partialName: OPTIONAL<BOOLEAN>,
    pub derivedEntry: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl EntryInformation {
    pub fn new(
        name: Name,
        fromEntry: OPTIONAL<BOOLEAN>,
        information: OPTIONAL<Vec<EntryInformation_information_Item>>,
        incompleteEntry: OPTIONAL<BOOLEAN>,
        partialName: OPTIONAL<BOOLEAN>,
        derivedEntry: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EntryInformation {
            name,
            fromEntry,
            information,
            incompleteEntry,
            partialName,
            derivedEntry,
            _unrecognized,
        }
    }
    pub fn _default_value_for_fromEntry() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_incompleteEntry() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_partialName() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_derivedEntry() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for EntryInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformation(el)
    }
}

pub const _rctl1_components_for_EntryInformation: &[ComponentSpec; 6] = &[
    ComponentSpec::new("name", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "fromEntry",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "incompleteEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "partialName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "derivedEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EntryInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EntryInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_EntryInformation(el: &X690Element) -> ASN1Result<EntryInformation> {
    |el_: &X690Element| -> ASN1Result<EntryInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EntryInformation,
            _eal_components_for_EntryInformation,
            _rctl2_components_for_EntryInformation,
        )?;
        let name = _decode_Name(_components.get("name").unwrap())?;
        let fromEntry: OPTIONAL<BOOLEAN> = match _components.get("fromEntry") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information: OPTIONAL<Vec<EntryInformation_information_Item>> =
            match _components.get("information") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<
                    SET_OF<EntryInformation_information_Item>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<EntryInformation_information_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_EntryInformation_information_Item(el)?);
                    }
                    Ok(items)
                }(c_)?),
                _ => None,
            };
        let incompleteEntry: OPTIONAL<BOOLEAN> = match _components.get("incompleteEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let partialName: OPTIONAL<BOOLEAN> = match _components.get("partialName") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let derivedEntry: OPTIONAL<BOOLEAN> = match _components.get("derivedEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(EntryInformation {
            name,
            fromEntry,
            information,
            incompleteEntry,
            partialName,
            derivedEntry,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EntryInformation(value_: &EntryInformation) -> ASN1Result<X690Element> {
    |value_: &EntryInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(_encode_Name(&value_.name)?);
        if let Some(v_) = &value_.fromEntry {
            if *v_ != EntryInformation::_default_value_for_fromEntry() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.information {
            components_.push(
                |value_: &SET_OF<EntryInformation_information_Item>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_EntryInformation_information_Item(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.incompleteEntry {
            if *v_ != EntryInformation::_default_value_for_incompleteEntry() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.partialName {
            if *v_ != EntryInformation::_default_value_for_partialName() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.derivedEntry {
            if *v_ != EntryInformation::_default_value_for_derivedEntry() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
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
/// family-information ATTRIBUTE ::= {
///   WITH SYNTAX  FamilyEntries
///   USAGE        directoryOperation
///   ID           id-at-family-information }
/// ```
///
///
pub fn family_information() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_at_family_information(),                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FamilyEntries ::= SEQUENCE {
///   family-class   OBJECT-CLASS.&id, -- structural object class value
///   familyEntries  SEQUENCE OF FamilyEntry,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct FamilyEntries {
    pub family_class: OBJECT_IDENTIFIER,
    pub familyEntries: Vec<FamilyEntry>,
    pub _unrecognized: Vec<X690Element>,
}
impl FamilyEntries {
    pub fn new(
        family_class: OBJECT_IDENTIFIER,
        familyEntries: Vec<FamilyEntry>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FamilyEntries {
            family_class,
            familyEntries,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for FamilyEntries {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyEntries(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FamilyEntries {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyEntries(el)
    }
}

pub const _rctl1_components_for_FamilyEntries: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "family-class",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyEntries",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FamilyEntries: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FamilyEntries: &[ComponentSpec; 0] = &[];

pub fn _decode_FamilyEntries(el: &X690Element) -> ASN1Result<FamilyEntries> {
    |el_: &X690Element| -> ASN1Result<FamilyEntries> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FamilyEntries,
            _eal_components_for_FamilyEntries,
            _rctl2_components_for_FamilyEntries,
        )?;
        let family_class = ber_decode_object_identifier(_components.get("family-class").unwrap())?;
        let familyEntries = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<FamilyEntry>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<FamilyEntry> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_FamilyEntry(el)?);
            }
            Ok(items)
        }(_components.get("familyEntries").unwrap())?;
        Ok(FamilyEntries {
            family_class,
            familyEntries,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FamilyEntries(value_: &FamilyEntries) -> ASN1Result<X690Element> {
    |value_: &FamilyEntries| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.family_class)?);
        components_.push(
            |value_: &SEQUENCE_OF<FamilyEntry>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_FamilyEntry(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.familyEntries)?,
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
/// FamilyEntry ::= SEQUENCE {
///   rdn            RelativeDistinguishedName,
///   information    SEQUENCE OF CHOICE {
///     attributeType  AttributeType,
///     attribute      Attribute{{SupportedAttributes}},
///     ...},
///   family-info    SEQUENCE SIZE (1..MAX) OF FamilyEntries OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct FamilyEntry {
    pub rdn: RelativeDistinguishedName,
    pub information: Vec<FamilyEntry_information_Item>,
    pub family_info: OPTIONAL<Vec<FamilyEntries>>,
    pub _unrecognized: Vec<X690Element>,
}
impl FamilyEntry {
    pub fn new(
        rdn: RelativeDistinguishedName,
        information: Vec<FamilyEntry_information_Item>,
        family_info: OPTIONAL<Vec<FamilyEntries>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FamilyEntry {
            rdn,
            information,
            family_info,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for FamilyEntry {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyEntry(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FamilyEntry {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyEntry(el)
    }
}

pub const _rctl1_components_for_FamilyEntry: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "rdn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "family-info",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FamilyEntry: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FamilyEntry: &[ComponentSpec; 0] = &[];

pub fn _decode_FamilyEntry(el: &X690Element) -> ASN1Result<FamilyEntry> {
    |el_: &X690Element| -> ASN1Result<FamilyEntry> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FamilyEntry,
            _eal_components_for_FamilyEntry,
            _rctl2_components_for_FamilyEntry,
        )?;
        let rdn = _decode_RelativeDistinguishedName(_components.get("rdn").unwrap())?;
        let information =
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<FamilyEntry_information_Item>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<FamilyEntry_information_Item> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_FamilyEntry_information_Item(el)?);
                }
                Ok(items)
            }(_components.get("information").unwrap())?;
        let family_info: OPTIONAL<Vec<FamilyEntries>> = match _components.get("family-info") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<FamilyEntries>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<FamilyEntries> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_FamilyEntries(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        Ok(FamilyEntry {
            rdn,
            information,
            family_info,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FamilyEntry(value_: &FamilyEntry) -> ASN1Result<X690Element> {
    |value_: &FamilyEntry| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_RelativeDistinguishedName(&value_.rdn)?);
        components_.push(
            |value_: &SEQUENCE_OF<FamilyEntry_information_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_FamilyEntry_information_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.information)?,
        );
        if let Some(v_) = &value_.family_info {
            components_.push(
                |value_: &SEQUENCE_OF<FamilyEntries>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_FamilyEntries(&v)?);
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
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Filter  ::=  CHOICE {
///   item  [0]  FilterItem,
///   and   [1]  SET OF Filter,
///   or    [2]  SET OF Filter,
///   not   [3]  Filter,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Filter {
    item(FilterItem),
    and(Vec<Box<Filter>>),
    or(Vec<Box<Filter>>),
    not(Box<Filter>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Filter {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Filter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Filter {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Filter(el)
    }
}

pub fn _decode_Filter(el: &X690Element) -> ASN1Result<Filter> {
    |el: &X690Element| -> ASN1Result<Filter> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(Filter::item(|el: &X690Element| -> ASN1Result<FilterItem> {
                    Ok(_decode_FilterItem(&el.inner()?)?)
                }(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(Filter::and(
                |el: &X690Element| -> ASN1Result<SET_OF<Box<Filter>>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Box<Filter>> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_Filter(el)?));
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(Filter::or(
                |el: &X690Element| -> ASN1Result<SET_OF<Box<Filter>>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Box<Filter>> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_Filter(el)?));
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 3) => {
                Ok(Filter::not(|el: &X690Element| -> ASN1Result<Box<Filter>> {
                    Ok(Box::new(_decode_Filter(&el.inner()?)?))
                }(&el)?))
            }
            _ => Ok(Filter::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Filter(value_: &Filter) -> ASN1Result<X690Element> {
    |value: &Filter| -> ASN1Result<X690Element> {
        match value {
            Filter::item(v) => |v_1: &FilterItem| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FilterItem(&v_1)?))),
                ))
            }(&v),
            Filter::and(v) => |v_1: &Vec<Box<Filter>>| -> ASN1Result<X690Element> {
                let el_1 = |value_: &SET_OF<Box<Filter>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Filter(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Filter::or(v) => |v_1: &Vec<Box<Filter>>| -> ASN1Result<X690Element> {
                let el_1 = |value_: &SET_OF<Box<Filter>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Filter(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Filter::not(v) => |v_1: &Box<Filter>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v),
            Filter::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FilterItem  ::=  CHOICE {
///   equality          [0]  AttributeValueAssertion,
///   substrings        [1]  SEQUENCE {
///     type                   ATTRIBUTE.&id({SupportedAttributes}),
///     strings                SEQUENCE OF CHOICE {
///       initial           [0]  ATTRIBUTE.&Type
///                               ({SupportedAttributes}{@substrings.type}),
///       any               [1]  ATTRIBUTE.&Type
///                               ({SupportedAttributes}{@substrings.type}),
///       final             [2]  ATTRIBUTE.&Type
///                               ({SupportedAttributes}{@substrings.type}),
///       control                Attribute{{SupportedAttributes}},
///                     -- Used to specify interpretation of following items
///       ... },
///     ... },
///   greaterOrEqual    [2]  AttributeValueAssertion,
///   lessOrEqual       [3]  AttributeValueAssertion,
///   present           [4]  AttributeType,
///   approximateMatch  [5]  AttributeValueAssertion,
///   extensibleMatch   [6]  MatchingRuleAssertion,
///   contextPresent    [7]  AttributeTypeAssertion,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum FilterItem {
    equality(AttributeValueAssertion),
    substrings(FilterItem_substrings),
    greaterOrEqual(AttributeValueAssertion),
    lessOrEqual(AttributeValueAssertion),
    present(AttributeType),
    approximateMatch(AttributeValueAssertion),
    extensibleMatch(MatchingRuleAssertion),
    contextPresent(AttributeTypeAssertion),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for FilterItem {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FilterItem(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FilterItem {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FilterItem(el)
    }
}

pub fn _decode_FilterItem(el: &X690Element) -> ASN1Result<FilterItem> {
    |el: &X690Element| -> ASN1Result<FilterItem> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(FilterItem::equality(_decode_AttributeValueAssertion(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(FilterItem::substrings(_decode_FilterItem_substrings(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 2) => Ok(FilterItem::greaterOrEqual(
                _decode_AttributeValueAssertion(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 3) => Ok(FilterItem::lessOrEqual(_decode_AttributeValueAssertion(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 4) => Ok(FilterItem::present(_decode_AttributeType(&el)?)),
            (TagClass::CONTEXT, 5) => Ok(FilterItem::approximateMatch(
                _decode_AttributeValueAssertion(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 6) => Ok(FilterItem::extensibleMatch(
                _decode_MatchingRuleAssertion(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 7) => Ok(FilterItem::contextPresent(
                _decode_AttributeTypeAssertion(&el.inner()?)?,
            )),
            _ => Ok(FilterItem::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_FilterItem(value_: &FilterItem) -> ASN1Result<X690Element> {
    |value: &FilterItem| -> ASN1Result<X690Element> {
        match value {
            FilterItem::equality(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                let el_1 = _encode_AttributeValueAssertion(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            FilterItem::substrings(v) => |v_1: &FilterItem_substrings| -> ASN1Result<X690Element> {
                let el_1 = _encode_FilterItem_substrings(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            FilterItem::greaterOrEqual(v) => {
                |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeValueAssertion(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem::lessOrEqual(v) => {
                |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeValueAssertion(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem::present(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let el_1 = _encode_AttributeType(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            FilterItem::approximateMatch(v) => {
                |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeValueAssertion(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem::extensibleMatch(v) => {
                |v_1: &MatchingRuleAssertion| -> ASN1Result<X690Element> {
                    let el_1 = _encode_MatchingRuleAssertion(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        6,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem::contextPresent(v) => {
                |v_1: &AttributeTypeAssertion| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeTypeAssertion(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        7,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingRuleAssertion ::= SEQUENCE {
///   matchingRule  [1]  SET SIZE (1..MAX) OF MATCHING-RULE.&id,
///   type          [2]  AttributeType OPTIONAL,
///   matchValue    [3]  MATCHING-RULE.&AssertionType (CONSTRAINED BY {
///     -- matchValue shall be a value of  type specified by the &AssertionType field of
///     -- one of the MATCHING-RULE information objects identified by matchingRule -- }),
///   dnAttributes  [4]  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MatchingRuleAssertion {
    pub matchingRule: Vec<OBJECT_IDENTIFIER>,
    pub type_: OPTIONAL<AttributeType>,
    pub matchValue: X690Element,
    pub dnAttributes: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl MatchingRuleAssertion {
    pub fn new(
        matchingRule: Vec<OBJECT_IDENTIFIER>,
        type_: OPTIONAL<AttributeType>,
        matchValue: X690Element,
        dnAttributes: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MatchingRuleAssertion {
            matchingRule,
            type_,
            matchValue,
            dnAttributes,
            _unrecognized,
        }
    }
    pub fn _default_value_for_dnAttributes() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for MatchingRuleAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MatchingRuleAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleAssertion(el)
    }
}

pub const _rctl1_components_for_MatchingRuleAssertion: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "matchingRule",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "type",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchValue",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dnAttributes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MatchingRuleAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MatchingRuleAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_MatchingRuleAssertion(el: &X690Element) -> ASN1Result<MatchingRuleAssertion> {
    |el_: &X690Element| -> ASN1Result<MatchingRuleAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MatchingRuleAssertion,
            _eal_components_for_MatchingRuleAssertion,
            _rctl2_components_for_MatchingRuleAssertion,
        )?;
        let matchingRule = |el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
            Ok(
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
                }(&el.inner()?)?,
            )
        }(_components.get("matchingRule").unwrap())?;
        let type_: OPTIONAL<AttributeType> = match _components.get("type") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let matchValue =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("matchValue").unwrap(),
            )?;
        let dnAttributes: OPTIONAL<BOOLEAN> = match _components.get("dnAttributes") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(MatchingRuleAssertion {
            matchingRule,
            type_,
            matchValue,
            dnAttributes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MatchingRuleAssertion(value_: &MatchingRuleAssertion) -> ASN1Result<X690Element> {
    |value_: &MatchingRuleAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                    OBJECT_IDENTIFIER,
                >|
                 -> ASN1Result<
                    X690Element,
                > {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_object_identifier(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.matchingRule)?);
        if let Some(v_) = &value_.type_ {
            components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AttributeType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.matchValue)?);
        if let Some(v_) = &value_.dnAttributes {
            if *v_ != MatchingRuleAssertion::_default_value_for_dnAttributes() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
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
/// PagedResultsRequest  ::=  CHOICE {
///   newRequest         SEQUENCE {
///     pageSize           INTEGER,
///     sortKeys           SEQUENCE SIZE (1..MAX) OF SortKey OPTIONAL,
///     reverse       [1]  BOOLEAN DEFAULT FALSE,
///     unmerged      [2]  BOOLEAN DEFAULT FALSE,
///     pageNumber    [3]  INTEGER OPTIONAL,
///     ...},
///   queryReference     OCTET STRING,
///   abandonQuery  [0]  OCTET STRING,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum PagedResultsRequest {
    newRequest(PagedResultsRequest_newRequest),
    queryReference(OCTET_STRING),
    abandonQuery(OCTET_STRING),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for PagedResultsRequest {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PagedResultsRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PagedResultsRequest {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PagedResultsRequest(el)
    }
}

pub fn _decode_PagedResultsRequest(el: &X690Element) -> ASN1Result<PagedResultsRequest> {
    |el: &X690Element| -> ASN1Result<PagedResultsRequest> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(PagedResultsRequest::newRequest(
                _decode_PagedResultsRequest_newRequest(&el)?,
            )),
            (TagClass::UNIVERSAL, 4) => Ok(PagedResultsRequest::queryReference(
                ber_decode_octet_string(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(PagedResultsRequest::abandonQuery(
                ber_decode_octet_string(&el.inner()?)?,
            )),
            _ => Ok(PagedResultsRequest::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_PagedResultsRequest(value_: &PagedResultsRequest) -> ASN1Result<X690Element> {
    |value: &PagedResultsRequest| -> ASN1Result<X690Element> {
        match value {
            PagedResultsRequest::newRequest(v) => _encode_PagedResultsRequest_newRequest(&v),
            PagedResultsRequest::queryReference(v) => ber_encode_octet_string(&v),
            PagedResultsRequest::abandonQuery(v) => {
                |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_octet_string(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            PagedResultsRequest::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SortKey ::= SEQUENCE {
///   type          AttributeType,
///   orderingRule  MATCHING-RULE.&id OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SortKey {
    pub type_: AttributeType,
    pub orderingRule: OPTIONAL<OBJECT_IDENTIFIER>,
    pub _unrecognized: Vec<X690Element>,
}
impl SortKey {
    pub fn new(
        type_: AttributeType,
        orderingRule: OPTIONAL<OBJECT_IDENTIFIER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SortKey {
            type_,
            orderingRule,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SortKey {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SortKey(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SortKey {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SortKey(el)
    }
}

pub const _rctl1_components_for_SortKey: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "orderingRule",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SortKey: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SortKey: &[ComponentSpec; 0] = &[];

pub fn _decode_SortKey(el: &X690Element) -> ASN1Result<SortKey> {
    |el_: &X690Element| -> ASN1Result<SortKey> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SortKey,
            _eal_components_for_SortKey,
            _rctl2_components_for_SortKey,
        )?;
        let type_ = _decode_AttributeType(_components.get("type").unwrap())?;
        let orderingRule: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("orderingRule") {
            Some(c_) => Some(ber_decode_object_identifier(c_)?),
            _ => None,
        };
        Ok(SortKey {
            type_,
            orderingRule,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SortKey(value_: &SortKey) -> ASN1Result<X690Element> {
    |value_: &SortKey| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeType(&value_.type_)?);
        if let Some(v_) = &value_.orderingRule {
            components_.push(ber_encode_object_identifier(&v_)?);
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
/// SecurityParameters ::= SET {
///   certification-path          [0]  CertificationPath OPTIONAL,
///   name                        [1]  DistinguishedName OPTIONAL,
///   time                        [2]  Time OPTIONAL,
///   random                      [3]  BIT STRING OPTIONAL,
///   target                      [4]  ProtectionRequest OPTIONAL,
///   --                          [5]  Not to be used
///   operationCode               [6]  Code OPTIONAL,
///   --                          [7]  Not to be used
///   errorProtection             [8]  ErrorProtectionRequest OPTIONAL,
///   errorCode                   [9]  Code OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SecurityParameters {
    pub certification_path: OPTIONAL<CertificationPath>,
    pub name: OPTIONAL<DistinguishedName>,
    pub time: OPTIONAL<Time>,
    pub random: OPTIONAL<BIT_STRING>,
    pub target: OPTIONAL<ProtectionRequest>,
    pub operationCode: OPTIONAL<Code>,
    pub errorProtection: OPTIONAL<ErrorProtectionRequest>,
    pub errorCode: OPTIONAL<Code>,
    pub _unrecognized: Vec<X690Element>,
}
impl SecurityParameters {
    pub fn new(
        certification_path: OPTIONAL<CertificationPath>,
        name: OPTIONAL<DistinguishedName>,
        time: OPTIONAL<Time>,
        random: OPTIONAL<BIT_STRING>,
        target: OPTIONAL<ProtectionRequest>,
        operationCode: OPTIONAL<Code>,
        errorProtection: OPTIONAL<ErrorProtectionRequest>,
        errorCode: OPTIONAL<Code>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SecurityParameters {
            certification_path,
            name,
            time,
            random,
            target,
            operationCode,
            errorProtection,
            errorCode,
            _unrecognized,
        }
    }
}
impl Default for SecurityParameters {
    fn default() -> Self {
        SecurityParameters {
            certification_path: None,
            name: None,
            time: None,
            random: None,
            target: None,
            operationCode: None,
            errorProtection: None,
            errorCode: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for SecurityParameters {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityParameters(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SecurityParameters {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityParameters(el)
    }
}

pub const _rctl1_components_for_SecurityParameters: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "certification-path",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "random",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "target",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationCode",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "errorProtection",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "errorCode",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SecurityParameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SecurityParameters: &[ComponentSpec; 0] = &[];

pub fn _decode_SecurityParameters(el: &X690Element) -> ASN1Result<SecurityParameters> {
    |el_: &X690Element| -> ASN1Result<SecurityParameters> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SecurityParameters,
            _eal_components_for_SecurityParameters,
            _rctl2_components_for_SecurityParameters,
            90,
        )?;
        let certification_path: OPTIONAL<CertificationPath> =
            match _components.get("certification-path") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<CertificationPath> {
                    Ok(_decode_CertificationPath(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let name: OPTIONAL<DistinguishedName> = match _components.get("name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let time: OPTIONAL<Time> = match _components.get("time") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Time> {
                Ok(_decode_Time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let random: OPTIONAL<BIT_STRING> = match _components.get("random") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let target: OPTIONAL<ProtectionRequest> = match _components.get("target") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ProtectionRequest> {
                Ok(_decode_ProtectionRequest(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationCode: OPTIONAL<Code> = match _components.get("operationCode") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Code> {
                Ok(_decode_Code(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let errorProtection: OPTIONAL<ErrorProtectionRequest> =
            match _components.get("errorProtection") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ErrorProtectionRequest> {
                    Ok(_decode_ErrorProtectionRequest(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let errorCode: OPTIONAL<Code> = match _components.get("errorCode") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Code> {
                Ok(_decode_Code(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SecurityParameters {
            certification_path,
            name,
            time,
            random,
            target,
            operationCode,
            errorProtection,
            errorCode,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SecurityParameters(value_: &SecurityParameters) -> ASN1Result<X690Element> {
    |value_: &SecurityParameters| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        if let Some(v_) = &value_.certification_path {
            components_.push(|v_1: &CertificationPath| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_CertificationPath(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.name {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.time {
            components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.random {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.target {
            components_.push(|v_1: &ProtectionRequest| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ProtectionRequest(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationCode {
            components_.push(|v_1: &Code| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Code(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.errorProtection {
            components_.push(|v_1: &ErrorProtectionRequest| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_ErrorProtectionRequest(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.errorCode {
            components_.push(|v_1: &Code| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Code(&v_1)?))),
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
/// ProtectionRequest  ::=  INTEGER {none(0), signed(1)}
/// ```
pub type ProtectionRequest = INTEGER;

pub const ProtectionRequest_none: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ProtectionRequest_signed: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ProtectionRequest(el: &X690Element) -> ASN1Result<ProtectionRequest> {
    ber_decode_integer(&el)
}

pub fn _encode_ProtectionRequest(value_: &ProtectionRequest) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
/// ErrorProtectionRequest  ::=  INTEGER {none(0), signed(1)}
/// ```
pub type ErrorProtectionRequest = INTEGER;

pub const ErrorProtectionRequest_none: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ErrorProtectionRequest_signed: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ErrorProtectionRequest(el: &X690Element) -> ASN1Result<ErrorProtectionRequest> {
    ber_decode_integer(&el)
}

pub fn _encode_ErrorProtectionRequest(value_: &ErrorProtectionRequest) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryBind OPERATION ::= {
///   ARGUMENT  DirectoryBindArgument
///   RESULT    DirectoryBindResult
///   ERRORS    {directoryBindError} }
/// ```
///
///
pub fn directoryBind() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([directoryBindError()])), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryBindArgument ::= SET {
///   credentials  [0]  Credentials OPTIONAL,
///   versions     [1]  Versions DEFAULT {v1},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DirectoryBindArgument {
    pub credentials: OPTIONAL<Credentials>,
    pub versions: OPTIONAL<Versions>,
    pub _unrecognized: Vec<X690Element>,
}
impl DirectoryBindArgument {
    pub fn new(
        credentials: OPTIONAL<Credentials>,
        versions: OPTIONAL<Versions>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DirectoryBindArgument {
            credentials,
            versions,
            _unrecognized,
        }
    }
    pub fn _default_value_for_versions() -> Versions {
        BIT_STRING::with_bits_set(&[Versions_v1])
    }
}
impl Default for DirectoryBindArgument {
    fn default() -> Self {
        DirectoryBindArgument {
            credentials: None,
            versions: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for DirectoryBindArgument {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindArgument(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DirectoryBindArgument {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindArgument(el)
    }
}

pub const _rctl1_components_for_DirectoryBindArgument: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "credentials",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "versions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DirectoryBindArgument: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DirectoryBindArgument: &[ComponentSpec; 0] = &[];

pub fn _decode_DirectoryBindArgument(el: &X690Element) -> ASN1Result<DirectoryBindArgument> {
    |el_: &X690Element| -> ASN1Result<DirectoryBindArgument> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_DirectoryBindArgument,
            _eal_components_for_DirectoryBindArgument,
            _rctl2_components_for_DirectoryBindArgument,
            30,
        )?;
        let credentials: OPTIONAL<Credentials> = match _components.get("credentials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Credentials> {
                Ok(_decode_Credentials(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let versions: OPTIONAL<Versions> = match _components.get("versions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Versions> {
                Ok(_decode_Versions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(DirectoryBindArgument {
            credentials,
            versions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DirectoryBindArgument(value_: &DirectoryBindArgument) -> ASN1Result<X690Element> {
    |value_: &DirectoryBindArgument| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.credentials {
            components_.push(|v_1: &Credentials| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Credentials(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.versions {
            if *v_ != DirectoryBindArgument::_default_value_for_versions() {
                components_.push(|v_1: &Versions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Versions(&v_1)?))),
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
/// Credentials  ::=  CHOICE {
///   simple             [0]  SimpleCredentials,
///   strong             [1]  StrongCredentials,
///   externalProcedure  [2]  EXTERNAL,
///   spkm               [3]  SpkmCredentials,
///   sasl               [4]  SaslCredentials,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Credentials {
    simple(SimpleCredentials),
    strong(StrongCredentials),
    externalProcedure(EXTERNAL),
    spkm(SpkmCredentials),
    sasl(SaslCredentials),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Credentials {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Credentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Credentials {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Credentials(el)
    }
}

pub fn _decode_Credentials(el: &X690Element) -> ASN1Result<Credentials> {
    |el: &X690Element| -> ASN1Result<Credentials> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Credentials::simple(_decode_SimpleCredentials(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(Credentials::strong(_decode_StrongCredentials(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 2) => Ok(Credentials::externalProcedure(ber_decode_external(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 3) => Ok(Credentials::spkm(
                |el: &X690Element| -> ASN1Result<SpkmCredentials> {
                    Ok(_decode_SpkmCredentials(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 4) => Ok(Credentials::sasl(_decode_SaslCredentials(&el.inner()?)?)),
            _ => Ok(Credentials::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Credentials(value_: &Credentials) -> ASN1Result<X690Element> {
    |value: &Credentials| -> ASN1Result<X690Element> {
        match value {
            Credentials::simple(v) => |v_1: &SimpleCredentials| -> ASN1Result<X690Element> {
                let el_1 = _encode_SimpleCredentials(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Credentials::strong(v) => |v_1: &StrongCredentials| -> ASN1Result<X690Element> {
                let el_1 = _encode_StrongCredentials(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Credentials::externalProcedure(v) => |v_1: &EXTERNAL| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_external(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Credentials::spkm(v) => |v_1: &SpkmCredentials| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_SpkmCredentials(
                        &v_1,
                    )?))),
                ))
            }(&v),
            Credentials::sasl(v) => |v_1: &SaslCredentials| -> ASN1Result<X690Element> {
                let el_1 = _encode_SaslCredentials(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            Credentials::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SimpleCredentials ::= SEQUENCE {
///   name      [0]  DistinguishedName,
///   validity  [1]  SET {
///     time1     [0]  CHOICE {
///       utc            UTCTime,
///       gt             GeneralizedTime} OPTIONAL,
///     time2     [1]  CHOICE {
///       utc            UTCTime,
///       gt             GeneralizedTime} OPTIONAL,
///     random1   [2]  BIT STRING OPTIONAL,
///     random2   [3]  BIT STRING OPTIONAL} OPTIONAL,
///   password  [2]  CHOICE {
///     unprotected    OCTET STRING,
///     protected      HASH{OCTET STRING},
///     ...,
///     userPwd   [0]  UserPwd } OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SimpleCredentials {
    pub name: DistinguishedName,
    pub validity: OPTIONAL<SimpleCredentials_validity>,
    pub password: OPTIONAL<SimpleCredentials_password>,
}
impl SimpleCredentials {
    pub fn new(
        name: DistinguishedName,
        validity: OPTIONAL<SimpleCredentials_validity>,
        password: OPTIONAL<SimpleCredentials_password>,
    ) -> Self {
        SimpleCredentials {
            name,
            validity,
            password,
        }
    }
}
impl TryFrom<X690Element> for SimpleCredentials {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SimpleCredentials {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials(el)
    }
}

pub const _rctl1_components_for_SimpleCredentials: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "validity",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "password",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SimpleCredentials: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SimpleCredentials: &[ComponentSpec; 0] = &[];

pub fn _decode_SimpleCredentials(el: &X690Element) -> ASN1Result<SimpleCredentials> {
    |el_: &X690Element| -> ASN1Result<SimpleCredentials> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SimpleCredentials,
            _eal_components_for_SimpleCredentials,
            _rctl2_components_for_SimpleCredentials,
        )?;
        let name = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("name").unwrap())?;
        let validity: OPTIONAL<SimpleCredentials_validity> = match _components.get("validity") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SimpleCredentials_validity> {
                    Ok(_decode_SimpleCredentials_validity(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let password: OPTIONAL<SimpleCredentials_password> = match _components.get("password") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SimpleCredentials_password> {
                    Ok(_decode_SimpleCredentials_password(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        Ok(SimpleCredentials {
            name,
            validity,
            password,
        })
    }(&el)
}

pub fn _encode_SimpleCredentials(value_: &SimpleCredentials) -> ASN1Result<X690Element> {
    |value_: &SimpleCredentials| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.name)?);
        if let Some(v_) = &value_.validity {
            components_.push(
                |v_1: &SimpleCredentials_validity| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SimpleCredentials_validity(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.password {
            components_.push(
                |v_1: &SimpleCredentials_password| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SimpleCredentials_password(&v_1)?,
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
/// StrongCredentials ::= SET {
///   certification-path          [0]  CertificationPath OPTIONAL,
///   bind-token                  [1]  Token,
///   name                        [2]  DistinguishedName OPTIONAL,
///   attributeCertificationPath  [3]  AttributeCertificationPath OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct StrongCredentials {
    pub certification_path: OPTIONAL<CertificationPath>,
    pub bind_token: Token,
    pub name: OPTIONAL<DistinguishedName>,
    pub attributeCertificationPath: OPTIONAL<AttributeCertificationPath>,
    pub _unrecognized: Vec<X690Element>,
}
impl StrongCredentials {
    pub fn new(
        certification_path: OPTIONAL<CertificationPath>,
        bind_token: Token,
        name: OPTIONAL<DistinguishedName>,
        attributeCertificationPath: OPTIONAL<AttributeCertificationPath>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        StrongCredentials {
            certification_path,
            bind_token,
            name,
            attributeCertificationPath,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for StrongCredentials {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_StrongCredentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for StrongCredentials {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_StrongCredentials(el)
    }
}

pub const _rctl1_components_for_StrongCredentials: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "certification-path",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "bind-token",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeCertificationPath",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_StrongCredentials: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_StrongCredentials: &[ComponentSpec; 0] = &[];

pub fn _decode_StrongCredentials(el: &X690Element) -> ASN1Result<StrongCredentials> {
    |el_: &X690Element| -> ASN1Result<StrongCredentials> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_StrongCredentials,
            _eal_components_for_StrongCredentials,
            _rctl2_components_for_StrongCredentials,
            50,
        )?;
        let certification_path: OPTIONAL<CertificationPath> =
            match _components.get("certification-path") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<CertificationPath> {
                    Ok(_decode_CertificationPath(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let bind_token =
            |el: &X690Element| -> ASN1Result<Token> { Ok(_decode_Token(&el.inner()?)?) }(
                _components.get("bind-token").unwrap(),
            )?;
        let name: OPTIONAL<DistinguishedName> = match _components.get("name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let attributeCertificationPath: OPTIONAL<AttributeCertificationPath> =
            match _components.get("attributeCertificationPath") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<AttributeCertificationPath> {
                        Ok(_decode_AttributeCertificationPath(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(StrongCredentials {
            certification_path,
            bind_token,
            name,
            attributeCertificationPath,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_StrongCredentials(value_: &StrongCredentials) -> ASN1Result<X690Element> {
    |value_: &StrongCredentials| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.certification_path {
            components_.push(|v_1: &CertificationPath| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_CertificationPath(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &Token| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Token(&v_1)?))),
            ))
        }(&value_.bind_token)?);
        if let Some(v_) = &value_.name {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.attributeCertificationPath {
            components_.push(
                |v_1: &AttributeCertificationPath| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_AttributeCertificationPath(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
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
/// SpkmCredentials  ::=  CHOICE {
///   req            [0]  SPKM-REQ,
///   rep            [1]  SPKM-REP-TI,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum SpkmCredentials {
    req(SPKM_REQ),
    rep(SPKM_REP_TI),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for SpkmCredentials {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SpkmCredentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SpkmCredentials {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SpkmCredentials(el)
    }
}

pub fn _decode_SpkmCredentials(el: &X690Element) -> ASN1Result<SpkmCredentials> {
    |el: &X690Element| -> ASN1Result<SpkmCredentials> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(SpkmCredentials::req(_decode_SPKM_REQ(&el.inner()?)?)),
            (TagClass::CONTEXT, 1) => Ok(SpkmCredentials::rep(_decode_SPKM_REP_TI(&el.inner()?)?)),
            _ => Ok(SpkmCredentials::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_SpkmCredentials(value_: &SpkmCredentials) -> ASN1Result<X690Element> {
    |value: &SpkmCredentials| -> ASN1Result<X690Element> {
        match value {
            SpkmCredentials::req(v) => |v_1: &SPKM_REQ| -> ASN1Result<X690Element> {
                let el_1 = _encode_SPKM_REQ(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            SpkmCredentials::rep(v) => |v_1: &SPKM_REP_TI| -> ASN1Result<X690Element> {
                let el_1 = _encode_SPKM_REP_TI(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            SpkmCredentials::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SaslCredentials ::= SEQUENCE {
///   mechanism    [0]  DirectoryString{ub-saslMechanism},
///   credentials  [1]  OCTET STRING OPTIONAL,
///   saslAbort    [2]  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SaslCredentials {
    pub mechanism: DirectoryString,
    pub credentials: OPTIONAL<OCTET_STRING>,
    pub saslAbort: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl SaslCredentials {
    pub fn new(
        mechanism: DirectoryString,
        credentials: OPTIONAL<OCTET_STRING>,
        saslAbort: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SaslCredentials {
            mechanism,
            credentials,
            saslAbort,
            _unrecognized,
        }
    }
    pub fn _default_value_for_saslAbort() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for SaslCredentials {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SaslCredentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SaslCredentials {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SaslCredentials(el)
    }
}

pub const _rctl1_components_for_SaslCredentials: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "mechanism",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "credentials",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "saslAbort",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SaslCredentials: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SaslCredentials: &[ComponentSpec; 0] = &[];

pub fn _decode_SaslCredentials(el: &X690Element) -> ASN1Result<SaslCredentials> {
    |el_: &X690Element| -> ASN1Result<SaslCredentials> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SaslCredentials,
            _eal_components_for_SaslCredentials,
            _rctl2_components_for_SaslCredentials,
        )?;
        let mechanism = |el: &X690Element| -> ASN1Result<DirectoryString> {
            Ok(_decode_DirectoryString(&el.inner()?)?)
        }(_components.get("mechanism").unwrap())?;
        let credentials: OPTIONAL<OCTET_STRING> = match _components.get("credentials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(ber_decode_octet_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let saslAbort: OPTIONAL<BOOLEAN> = match _components.get("saslAbort") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SaslCredentials {
            mechanism,
            credentials,
            saslAbort,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SaslCredentials(value_: &SaslCredentials) -> ASN1Result<X690Element> {
    |value_: &SaslCredentials| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &DirectoryString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DirectoryString(
                    &v_1,
                )?))),
            ))
        }(&value_.mechanism)?);
        if let Some(v_) = &value_.credentials {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_octet_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.saslAbort {
            if *v_ != SaslCredentials::_default_value_for_saslAbort() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
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
/// ub-saslMechanism INTEGER ::= 20
/// ```
///
///
pub const ub_saslMechanism: i64 = 20;

/// ### ASN.1 Definition:
///
/// ```asn1
/// Token  ::=  SIGNED{TokenContent}
/// ```
pub type Token = SIGNED<TokenContent>; // DefinedType

pub fn _decode_Token(el: &X690Element) -> ASN1Result<Token> {
    _decode_SIGNED::<TokenContent>(_decode_TokenContent, &el)
}

pub fn _encode_Token(value_: &Token) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TokenContent>(_encode_TokenContent, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TokenContent ::= SEQUENCE {
///   algorithm  [0]  AlgorithmIdentifier{{SupportedAlgorithms}},
///   name       [1]  DistinguishedName,
///   time       [2]  Time,
///   random     [3]  BIT STRING,
///   response   [4]  BIT STRING OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TokenContent {
    pub algorithm: AlgorithmIdentifier,
    pub name: DistinguishedName,
    pub time: Time,
    pub random: BIT_STRING,
    pub response: OPTIONAL<BIT_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl TokenContent {
    pub fn new(
        algorithm: AlgorithmIdentifier,
        name: DistinguishedName,
        time: Time,
        random: BIT_STRING,
        response: OPTIONAL<BIT_STRING>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TokenContent {
            algorithm,
            name,
            time,
            random,
            response,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TokenContent {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TokenContent(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TokenContent {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TokenContent(el)
    }
}

pub const _rctl1_components_for_TokenContent: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "random",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "response",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TokenContent: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TokenContent: &[ComponentSpec; 0] = &[];

pub fn _decode_TokenContent(el: &X690Element) -> ASN1Result<TokenContent> {
    |el_: &X690Element| -> ASN1Result<TokenContent> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TokenContent,
            _eal_components_for_TokenContent,
            _rctl2_components_for_TokenContent,
        )?;
        let algorithm = |el: &X690Element| -> ASN1Result<AlgorithmIdentifier> {
            Ok(_decode_AlgorithmIdentifier(&el.inner()?)?)
        }(_components.get("algorithm").unwrap())?;
        let name = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("name").unwrap())?;
        let time = |el: &X690Element| -> ASN1Result<Time> { Ok(_decode_Time(&el.inner()?)?) }(
            _components.get("time").unwrap(),
        )?;
        let random = |el: &X690Element| -> ASN1Result<BIT_STRING> {
            Ok(ber_decode_bit_string(&el.inner()?)?)
        }(_components.get("random").unwrap())?;
        let response: OPTIONAL<BIT_STRING> = match _components.get("response") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(TokenContent {
            algorithm,
            name,
            time,
            random,
            response,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TokenContent(value_: &TokenContent) -> ASN1Result<X690Element> {
    |value_: &TokenContent| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_AlgorithmIdentifier(&v_1)?,
                ))),
            ))
        }(&value_.algorithm)?);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.name)?);
        components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
            ))
        }(&value_.time)?);
        components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                    &v_1,
                )?))),
            ))
        }(&value_.random)?);
        if let Some(v_) = &value_.response {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
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
/// Versions  ::=  BIT STRING {v1(0), v2(1)}
/// ```
pub type Versions = BIT_STRING;

pub const Versions_v1: BIT = 0; /* LONG_NAMED_BIT */

pub const Versions_v2: BIT = 1; /* LONG_NAMED_BIT */

pub fn _decode_Versions(el: &X690Element) -> ASN1Result<Versions> {
    ber_decode_bit_string(&el)
}

pub fn _encode_Versions(value_: &Versions) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryBindResult ::= SET {
///   credentials       [0]  Credentials OPTIONAL,
///   versions          [1]  Versions DEFAULT {v1},
///   ...,
///   pwdResponseValue  [2]  PwdResponseValue OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DirectoryBindResult {
    pub credentials: OPTIONAL<Credentials>,
    pub versions: OPTIONAL<Versions>,
    pub pwdResponseValue: OPTIONAL<PwdResponseValue>,
    pub _unrecognized: Vec<X690Element>,
}
impl DirectoryBindResult {
    pub fn new(
        credentials: OPTIONAL<Credentials>,
        versions: OPTIONAL<Versions>,
        pwdResponseValue: OPTIONAL<PwdResponseValue>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DirectoryBindResult {
            credentials,
            versions,
            pwdResponseValue,
            _unrecognized,
        }
    }
    pub fn _default_value_for_versions() -> Versions {
        BIT_STRING::with_bits_set(&[Versions_v1])
    }
}
impl Default for DirectoryBindResult {
    fn default() -> Self {
        DirectoryBindResult {
            credentials: None,
            versions: None,
            pwdResponseValue: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for DirectoryBindResult {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DirectoryBindResult {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindResult(el)
    }
}

pub const _rctl1_components_for_DirectoryBindResult: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "credentials",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "versions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DirectoryBindResult: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DirectoryBindResult: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "pwdResponseValue",
    true,
    TagSelector::tag((TagClass::CONTEXT, 2)),
    None,
    None,
)];

pub fn _decode_DirectoryBindResult(el: &X690Element) -> ASN1Result<DirectoryBindResult> {
    |el_: &X690Element| -> ASN1Result<DirectoryBindResult> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_DirectoryBindResult,
            _eal_components_for_DirectoryBindResult,
            _rctl2_components_for_DirectoryBindResult,
            40,
        )?;
        let credentials: OPTIONAL<Credentials> = match _components.get("credentials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Credentials> {
                Ok(_decode_Credentials(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let versions: OPTIONAL<Versions> = match _components.get("versions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Versions> {
                Ok(_decode_Versions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let pwdResponseValue: OPTIONAL<PwdResponseValue> = match _components.get("pwdResponseValue")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<PwdResponseValue> {
                Ok(_decode_PwdResponseValue(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(DirectoryBindResult {
            credentials,
            versions,
            pwdResponseValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DirectoryBindResult(value_: &DirectoryBindResult) -> ASN1Result<X690Element> {
    |value_: &DirectoryBindResult| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.credentials {
            components_.push(|v_1: &Credentials| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Credentials(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.versions {
            if *v_ != DirectoryBindResult::_default_value_for_versions() {
                components_.push(|v_1: &Versions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Versions(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.pwdResponseValue {
            components_.push(|v_1: &PwdResponseValue| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_PwdResponseValue(
                        &v_1,
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
/// PwdResponseValue ::= SEQUENCE {
///   warning CHOICE {
///     timeLeft        [0]  INTEGER (0..MAX),
///     graceRemaining  [1]  INTEGER (0..MAX),
///     ... } OPTIONAL,
///   error   ENUMERATED {
///     passwordExpired  (0),
///     changeAfterReset (1),
///     ... } OPTIONAL}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PwdResponseValue {
    pub warning: OPTIONAL<PwdResponseValue_warning>,
    pub error: OPTIONAL<PwdResponseValue_error>,
}
impl PwdResponseValue {
    pub fn new(
        warning: OPTIONAL<PwdResponseValue_warning>,
        error: OPTIONAL<PwdResponseValue_error>,
    ) -> Self {
        PwdResponseValue { warning, error }
    }
}
impl Default for PwdResponseValue {
    fn default() -> Self {
        PwdResponseValue {
            warning: None,
            error: None,
        }
    }
}
impl TryFrom<X690Element> for PwdResponseValue {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponseValue(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PwdResponseValue {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponseValue(el)
    }
}

pub const _rctl1_components_for_PwdResponseValue: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "warning",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "error",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PwdResponseValue: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PwdResponseValue: &[ComponentSpec; 0] = &[];

pub fn _decode_PwdResponseValue(el: &X690Element) -> ASN1Result<PwdResponseValue> {
    |el_: &X690Element| -> ASN1Result<PwdResponseValue> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PwdResponseValue,
            _eal_components_for_PwdResponseValue,
            _rctl2_components_for_PwdResponseValue,
        )?;
        let warning: OPTIONAL<PwdResponseValue_warning> = match _components.get("warning") {
            Some(c_) => Some(_decode_PwdResponseValue_warning(c_)?),
            _ => None,
        };
        let error: OPTIONAL<PwdResponseValue_error> = match _components.get("error") {
            Some(c_) => Some(_decode_PwdResponseValue_error(c_)?),
            _ => None,
        };
        Ok(PwdResponseValue { warning, error })
    }(&el)
}

pub fn _encode_PwdResponseValue(value_: &PwdResponseValue) -> ASN1Result<X690Element> {
    |value_: &PwdResponseValue| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.warning {
            components_.push(_encode_PwdResponseValue_warning(&v_)?);
        }
        if let Some(v_) = &value_.error {
            components_.push(_encode_PwdResponseValue_error(&v_)?);
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
/// DirectoryBindError-OPTIONALLY-PROTECTED-Parameter1 ::= SET {
///   versions              [0]  Versions DEFAULT {v1},
///   error                      CHOICE {
///     serviceError          [1]  ServiceProblem,
///     securityError         [2]  SecurityProblem,
///     ...},
///   securityParameters    [30]  SecurityParameters OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1 {
    pub versions: OPTIONAL<Versions>,
    pub error: DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error,
    pub securityParameters: OPTIONAL<SecurityParameters>,
}
impl DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1 {
    pub fn new(
        versions: OPTIONAL<Versions>,
        error: DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error,
        securityParameters: OPTIONAL<SecurityParameters>,
    ) -> Self {
        DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1 {
            versions,
            error,
            securityParameters,
        }
    }
    pub fn _default_value_for_versions() -> Versions {
        BIT_STRING::with_bits_set(&[Versions_v1])
    }
}
impl TryFrom<X690Element> for DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1 {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1 {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1(el)
    }
}

pub const _rctl1_components_for_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     3] = &[
    ComponentSpec::new(
        "versions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "error",
        false,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 1)),
            &TagSelector::tag((TagClass::CONTEXT, 2)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     0] = &[];

pub const _eal_components_for_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     0] = &[];

pub fn _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1(
    el: &X690Element,
) -> ASN1Result<DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1> {
    |el_: &X690Element| -> ASN1Result<DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
            _eal_components_for_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
            _rctl2_components_for_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
            30,
        )?;
        let versions: OPTIONAL<Versions> = match _components.get("versions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Versions> {
                Ok(_decode_Versions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let error = _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error(
            _components.get("error").unwrap(),
        )?;
        let securityParameters: OPTIONAL<SecurityParameters> =
            match _components.get("securityParameters") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SecurityParameters> {
                    Ok(_decode_SecurityParameters(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1 {
            versions,
            error,
            securityParameters,
        })
    }(&el)
}

pub fn _encode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1(
    value_: &DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
) -> ASN1Result<X690Element> {
    |value_: &DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        if let Some(v_) = &value_.versions {
            if *v_
                != DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1::_default_value_for_versions()
            {
                components_.push(|v_1: &Versions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Versions(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        components_
            .push(_encode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error(&value_.error)?);
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
/// directoryBindError ERROR ::= {
///   PARAMETER OPTIONALLY-PROTECTED {DirectoryBindError-OPTIONALLY-PROTECTED-Parameter1}}
/// ```
///
///
pub fn directoryBindError() -> ERROR {
    ERROR { errorCode: None }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BindKeyInfo  ::=  ENCRYPTED{BIT STRING}
/// ```
pub type BindKeyInfo = ENCRYPTED; // DefinedType

pub fn _decode_BindKeyInfo(el: &X690Element) -> ASN1Result<BindKeyInfo> {
    _decode_ENCRYPTED(&el)
}

pub fn _encode_BindKeyInfo(value_: &BindKeyInfo) -> ASN1Result<X690Element> {
    _encode_ENCRYPTED(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// read OPERATION ::= {
///   ARGUMENT  ReadArgument
///   RESULT    ReadResult
///   ERRORS    {attributeError |
///              nameError |
///              serviceError |
///              referral |
///              abandoned |
///              securityError}
///   CODE      id-opcode-read }
/// ```
///
///
pub fn read() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            attributeError(),
            nameError(),
            serviceError(),
            referral(),
            abandoned(),
            securityError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_read), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReadArgument  ::=  OPTIONALLY-PROTECTED { ReadArgumentData }
/// ```
pub type ReadArgument = OPTIONALLY_PROTECTED<ReadArgumentData>; // DefinedType

pub fn _decode_ReadArgument(el: &X690Element) -> ASN1Result<ReadArgument> {
    _decode_OPTIONALLY_PROTECTED::<ReadArgumentData>(_decode_ReadArgumentData, &el)
}

pub fn _encode_ReadArgument(value_: &ReadArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<ReadArgumentData>(_encode_ReadArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReadArgumentData ::= SET {
///   object               [0]  Name,
///   selection            [1]  EntryInformationSelection DEFAULT {},
///   modifyRightsRequest  [2]  BOOLEAN DEFAULT FALSE,
///   ...,
///   ...,
///   COMPONENTS OF             CommonArguments }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ReadArgumentData {
    pub object: Name,
    pub selection: OPTIONAL<EntryInformationSelection>,
    pub modifyRightsRequest: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl ReadArgumentData {
    pub fn new(
        object: Name,
        selection: OPTIONAL<EntryInformationSelection>,
        modifyRightsRequest: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        ReadArgumentData {
            object,
            selection,
            modifyRightsRequest,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_selection() -> EntryInformationSelection {
        EntryInformationSelection {
            attributes: None,
            infoTypes: None,
            extraAttributes: None,
            contextSelection: None,
            returnContexts: None,
            familyReturn: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_modifyRightsRequest() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for ReadArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReadArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReadArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReadArgumentData(el)
    }
}

pub const _rctl1_components_for_ReadArgumentData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "selection",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "modifyRightsRequest",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ReadArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_ReadArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_ReadArgumentData(el: &X690Element) -> ASN1Result<ReadArgumentData> {
    |el_: &X690Element| -> ASN1Result<ReadArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ReadArgumentData,
            _eal_components_for_ReadArgumentData,
            _rctl2_components_for_ReadArgumentData,
            160,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let selection: OPTIONAL<EntryInformationSelection> = match _components.get("selection") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<EntryInformationSelection> {
                    Ok(_decode_EntryInformationSelection(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let modifyRightsRequest: OPTIONAL<BOOLEAN> = match _components.get("modifyRightsRequest") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ReadArgumentData {
            object,
            selection,
            modifyRightsRequest,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_ReadArgumentData(value_: &ReadArgumentData) -> ASN1Result<X690Element> {
    |value_: &ReadArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(25);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        if let Some(v_) = &value_.selection {
            components_.push(
                |v_1: &EntryInformationSelection| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_EntryInformationSelection(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.modifyRightsRequest {
            if *v_ != ReadArgumentData::_default_value_for_modifyRightsRequest() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != ReadArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != ReadArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != ReadArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != ReadArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// ReadResult  ::=  OPTIONALLY-PROTECTED { ReadResultData }
/// ```
pub type ReadResult = OPTIONALLY_PROTECTED<ReadResultData>; // DefinedType

pub fn _decode_ReadResult(el: &X690Element) -> ASN1Result<ReadResult> {
    _decode_OPTIONALLY_PROTECTED::<ReadResultData>(_decode_ReadResultData, &el)
}

pub fn _encode_ReadResult(value_: &ReadResult) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<ReadResultData>(_encode_ReadResultData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReadResultData ::= SET {
///   entry         [0]  EntryInformation,
///   modifyRights  [1]  ModifyRights OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF      CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ReadResultData {
    pub entry: EntryInformation,
    pub modifyRights: OPTIONAL<ModifyRights>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ReadResultData {
    pub fn new(
        entry: EntryInformation,
        modifyRights: OPTIONAL<ModifyRights>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ReadResultData {
            entry,
            modifyRights,
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
impl TryFrom<X690Element> for ReadResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReadResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReadResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReadResultData(el)
    }
}

pub const _rctl1_components_for_ReadResultData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "entry",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "modifyRights",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ReadResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ReadResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_ReadResultData(el: &X690Element) -> ASN1Result<ReadResultData> {
    |el_: &X690Element| -> ASN1Result<ReadResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ReadResultData,
            _eal_components_for_ReadResultData,
            _rctl2_components_for_ReadResultData,
            70,
        )?;
        let entry = |el: &X690Element| -> ASN1Result<EntryInformation> {
            Ok(_decode_EntryInformation(&el.inner()?)?)
        }(_components.get("entry").unwrap())?;
        let modifyRights: OPTIONAL<ModifyRights> = match _components.get("modifyRights") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ModifyRights> {
                Ok(_decode_ModifyRights(&el.inner()?)?)
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
        Ok(ReadResultData {
            entry,
            modifyRights,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ReadResultData(value_: &ReadResultData) -> ASN1Result<X690Element> {
    |value_: &ReadResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &EntryInformation| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_EntryInformation(
                    &v_1,
                )?))),
            ))
        }(&value_.entry)?);
        if let Some(v_) = &value_.modifyRights {
            components_.push(|v_1: &ModifyRights| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ModifyRights(
                        &v_1,
                    )?))),
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
            if *v_ != ReadResultData::_default_value_for_aliasDereferenced() {
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
/// ModifyRights  ::=  SET OF SEQUENCE {
///   item      CHOICE {
///     entry      [0]  NULL,
///     attribute  [1]  AttributeType,
///     value      [2]  AttributeValueAssertion,
///     ...},
///   permission   [3]  BIT STRING {
///     add     (0),
///     remove  (1),
///     rename  (2),
///     move    (3)},
///   ... }
/// ```
pub type ModifyRights = Vec<ModifyRights_Item>; // SetOfType

pub fn _decode_ModifyRights(el: &X690Element) -> ASN1Result<ModifyRights> {
    |el: &X690Element| -> ASN1Result<SET_OF<ModifyRights_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<ModifyRights_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_ModifyRights_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_ModifyRights(value_: &ModifyRights) -> ASN1Result<X690Element> {
    |value_: &SET_OF<ModifyRights_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_ModifyRights_Item(&v)?);
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
/// compare OPERATION ::= {
///   ARGUMENT  CompareArgument
///   RESULT    CompareResult
///   ERRORS    {attributeError |
///              nameError |
///              serviceError |
///              referral |
///              abandoned |
///              securityError}
///   CODE      id-opcode-compare }
/// ```
///
///
pub fn compare() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            attributeError(),
            nameError(),
            serviceError(),
            referral(),
            abandoned(),
            securityError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_compare), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareArgument  ::=  OPTIONALLY-PROTECTED { CompareArgumentData }
/// ```
pub type CompareArgument = OPTIONALLY_PROTECTED<CompareArgumentData>; // DefinedType

pub fn _decode_CompareArgument(el: &X690Element) -> ASN1Result<CompareArgument> {
    _decode_OPTIONALLY_PROTECTED::<CompareArgumentData>(_decode_CompareArgumentData, &el)
}

pub fn _encode_CompareArgument(value_: &CompareArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<CompareArgumentData>(_encode_CompareArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareArgumentData ::= SET {
///   object       [0]  Name,
///   purported    [1]  AttributeValueAssertion,
///   ...,
///   ...,
///   COMPONENTS OF     CommonArguments }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CompareArgumentData {
    pub object: Name,
    pub purported: AttributeValueAssertion,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl CompareArgumentData {
    pub fn new(
        object: Name,
        purported: AttributeValueAssertion,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        CompareArgumentData {
            object,
            purported,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for CompareArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CompareArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CompareArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CompareArgumentData(el)
    }
}

pub const _rctl1_components_for_CompareArgumentData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "purported",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CompareArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_CompareArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_CompareArgumentData(el: &X690Element) -> ASN1Result<CompareArgumentData> {
    |el_: &X690Element| -> ASN1Result<CompareArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_CompareArgumentData,
            _eal_components_for_CompareArgumentData,
            _rctl2_components_for_CompareArgumentData,
            150,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let purported = |el: &X690Element| -> ASN1Result<AttributeValueAssertion> {
            Ok(_decode_AttributeValueAssertion(&el.inner()?)?)
        }(_components.get("purported").unwrap())?;
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(CompareArgumentData {
            object,
            purported,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_CompareArgumentData(value_: &CompareArgumentData) -> ASN1Result<X690Element> {
    |value_: &CompareArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(24);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        components_.push(|v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_AttributeValueAssertion(&v_1)?,
                ))),
            ))
        }(&value_.purported)?);
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != CompareArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != CompareArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != CompareArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != CompareArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// CompareResult  ::=  OPTIONALLY-PROTECTED { CompareResultData }
/// ```
pub type CompareResult = OPTIONALLY_PROTECTED<CompareResultData>; // DefinedType

pub fn _decode_CompareResult(el: &X690Element) -> ASN1Result<CompareResult> {
    _decode_OPTIONALLY_PROTECTED::<CompareResultData>(_decode_CompareResultData, &el)
}

pub fn _encode_CompareResult(value_: &CompareResult) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<CompareResultData>(_encode_CompareResultData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareResultData ::= SET {
///   name                 Name OPTIONAL,
///   matched         [0]  BOOLEAN,
///   fromEntry       [1]  BOOLEAN DEFAULT TRUE,
///   matchedSubtype  [2]  AttributeType OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF        CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CompareResultData {
    pub name: OPTIONAL<Name>,
    pub matched: BOOLEAN,
    pub fromEntry: OPTIONAL<BOOLEAN>,
    pub matchedSubtype: OPTIONAL<AttributeType>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl CompareResultData {
    pub fn new(
        name: OPTIONAL<Name>,
        matched: BOOLEAN,
        fromEntry: OPTIONAL<BOOLEAN>,
        matchedSubtype: OPTIONAL<AttributeType>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        CompareResultData {
            name,
            matched,
            fromEntry,
            matchedSubtype,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_fromEntry() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for CompareResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CompareResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CompareResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CompareResultData(el)
    }
}

pub const _rctl1_components_for_CompareResultData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "name",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::UNIVERSAL, 16))]),
        None,
        None,
    ),
    ComponentSpec::new(
        "matched",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "fromEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchedSubtype",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CompareResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_CompareResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_CompareResultData(el: &X690Element) -> ASN1Result<CompareResultData> {
    |el_: &X690Element| -> ASN1Result<CompareResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_CompareResultData,
            _eal_components_for_CompareResultData,
            _rctl2_components_for_CompareResultData,
            90,
        )?;
        let name: OPTIONAL<Name> = match _components.get("name") {
            Some(c_) => Some(_decode_Name(c_)?),
            _ => None,
        };
        let matched =
            |el: &X690Element| -> ASN1Result<BOOLEAN> { Ok(ber_decode_boolean(&el.inner()?)?) }(
                _components.get("matched").unwrap(),
            )?;
        let fromEntry: OPTIONAL<BOOLEAN> = match _components.get("fromEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let matchedSubtype: OPTIONAL<AttributeType> = match _components.get("matchedSubtype") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
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
        Ok(CompareResultData {
            name,
            matched,
            fromEntry,
            matchedSubtype,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_CompareResultData(value_: &CompareResultData) -> ASN1Result<X690Element> {
    |value_: &CompareResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        if let Some(v_) = &value_.name {
            components_.push(_encode_Name(&v_)?);
        }
        components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
            ))
        }(&value_.matched)?);
        if let Some(v_) = &value_.fromEntry {
            if *v_ != CompareResultData::_default_value_for_fromEntry() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.matchedSubtype {
            components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AttributeType(
                        &v_1,
                    )?))),
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
            if *v_ != CompareResultData::_default_value_for_aliasDereferenced() {
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
/// abandon OPERATION ::= {
///   ARGUMENT  AbandonArgument
///   RESULT    AbandonResult
///   ERRORS    {abandonFailed}
///   CODE      id-opcode-abandon }
/// ```
///
///
pub fn abandon() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([abandonFailed()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_abandon),          /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonArgument  ::=
///   OPTIONALLY-PROTECTED-SEQ { AbandonArgumentData }
/// ```
pub type AbandonArgument = OPTIONALLY_PROTECTED_SEQ<AbandonArgumentData>; // DefinedType

pub fn _decode_AbandonArgument(el: &X690Element) -> ASN1Result<AbandonArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<AbandonArgumentData>(_decode_AbandonArgumentData, &el)
}

pub fn _encode_AbandonArgument(value_: &AbandonArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<AbandonArgumentData>(_encode_AbandonArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonArgumentData ::= SEQUENCE {
///   invokeID  [0]  InvokeId,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AbandonArgumentData {
    pub invokeID: InvokeId,
    pub _unrecognized: Vec<X690Element>,
}
impl AbandonArgumentData {
    pub fn new(invokeID: InvokeId, _unrecognized: Vec<X690Element>) -> Self {
        AbandonArgumentData {
            invokeID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AbandonArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AbandonArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonArgumentData(el)
    }
}

pub const _rctl1_components_for_AbandonArgumentData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "invokeID",
    false,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_AbandonArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AbandonArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_AbandonArgumentData(el: &X690Element) -> ASN1Result<AbandonArgumentData> {
    |el_: &X690Element| -> ASN1Result<AbandonArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AbandonArgumentData,
            _eal_components_for_AbandonArgumentData,
            _rctl2_components_for_AbandonArgumentData,
        )?;
        let invokeID =
            |el: &X690Element| -> ASN1Result<InvokeId> { Ok(_decode_InvokeId(&el.inner()?)?) }(
                _components.get("invokeID").unwrap(),
            )?;
        Ok(AbandonArgumentData {
            invokeID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AbandonArgumentData(value_: &AbandonArgumentData) -> ASN1Result<X690Element> {
    |value_: &AbandonArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(|v_1: &InvokeId| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_InvokeId(&v_1)?))),
            ))
        }(&value_.invokeID)?);
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
/// AbandonResult  ::=  CHOICE {
///   null          NULL,
///   information   OPTIONALLY-PROTECTED-SEQ { AbandonResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum AbandonResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<AbandonResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AbandonResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AbandonResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonResult(el)
    }
}

pub fn _decode_AbandonResult(el: &X690Element) -> ASN1Result<AbandonResult> {
    |el: &X690Element| -> ASN1Result<AbandonResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(AbandonResult::null(())),
            (TagClass::CONTEXT, 0) => Ok(AbandonResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<AbandonResultData>(
                    _decode_AbandonResultData,
                    &el,
                )?,
            )),
            _ => Ok(AbandonResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AbandonResult(value_: &AbandonResult) -> ASN1Result<X690Element> {
    |value: &AbandonResult| -> ASN1Result<X690Element> {
        match value {
            AbandonResult::null(v) => ber_encode_null(&v),
            AbandonResult::information(v) => {
                _encode_OPTIONALLY_PROTECTED_SEQ::<AbandonResultData>(_encode_AbandonResultData, &v)
            }
            AbandonResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonResultData ::= SEQUENCE {
///   invokeID      InvokeId,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AbandonResultData {
    pub invokeID: InvokeId,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl AbandonResultData {
    pub fn new(
        invokeID: InvokeId,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        AbandonResultData {
            invokeID,
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
impl TryFrom<X690Element> for AbandonResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AbandonResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonResultData(el)
    }
}

pub const _rctl1_components_for_AbandonResultData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "invokeID",
    false,
    TagSelector::any,
    None,
    None,
)];

pub const _rctl2_components_for_AbandonResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_AbandonResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_AbandonResultData(el: &X690Element) -> ASN1Result<AbandonResultData> {
    |el_: &X690Element| -> ASN1Result<AbandonResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AbandonResultData,
            _eal_components_for_AbandonResultData,
            _rctl2_components_for_AbandonResultData,
        )?;
        let invokeID = _decode_InvokeId(_components.get("invokeID").unwrap())?;
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
        Ok(AbandonResultData {
            invokeID,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_AbandonResultData(value_: &AbandonResultData) -> ASN1Result<X690Element> {
    |value_: &AbandonResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_InvokeId(&value_.invokeID)?);
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
            if *v_ != AbandonResultData::_default_value_for_aliasDereferenced() {
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
/// list OPERATION ::= {
///   ARGUMENT  ListArgument
///   RESULT    ListResult
///   ERRORS    {nameError |
///              serviceError |
///              referral |
///              abandoned |
///              securityError}
///   CODE      id-opcode-list }
/// ```
///
///
pub fn list() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            nameError(),
            serviceError(),
            referral(),
            abandoned(),
            securityError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_list), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ListArgument  ::=  OPTIONALLY-PROTECTED { ListArgumentData }
/// ```
pub type ListArgument = OPTIONALLY_PROTECTED<ListArgumentData>; // DefinedType

pub fn _decode_ListArgument(el: &X690Element) -> ASN1Result<ListArgument> {
    _decode_OPTIONALLY_PROTECTED::<ListArgumentData>(_decode_ListArgumentData, &el)
}

pub fn _encode_ListArgument(value_: &ListArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<ListArgumentData>(_encode_ListArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ListArgumentData ::= SET {
///   object        [0]  Name,
///   pagedResults  [1]  PagedResultsRequest OPTIONAL,
///   listFamily    [2]  BOOLEAN DEFAULT FALSE,
///   ...,
///   ...,
///   COMPONENTS OF      CommonArguments
///   }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ListArgumentData {
    pub object: Name,
    pub pagedResults: OPTIONAL<PagedResultsRequest>,
    pub listFamily: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl ListArgumentData {
    pub fn new(
        object: Name,
        pagedResults: OPTIONAL<PagedResultsRequest>,
        listFamily: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        ListArgumentData {
            object,
            pagedResults,
            listFamily,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_listFamily() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for ListArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ListArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ListArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ListArgumentData(el)
    }
}

pub const _rctl1_components_for_ListArgumentData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pagedResults",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "listFamily",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ListArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_ListArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_ListArgumentData(el: &X690Element) -> ASN1Result<ListArgumentData> {
    |el_: &X690Element| -> ASN1Result<ListArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ListArgumentData,
            _eal_components_for_ListArgumentData,
            _rctl2_components_for_ListArgumentData,
            160,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let pagedResults: OPTIONAL<PagedResultsRequest> = match _components.get("pagedResults") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<PagedResultsRequest> {
                Ok(_decode_PagedResultsRequest(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let listFamily: OPTIONAL<BOOLEAN> = match _components.get("listFamily") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ListArgumentData {
            object,
            pagedResults,
            listFamily,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_ListArgumentData(value_: &ListArgumentData) -> ASN1Result<X690Element> {
    |value_: &ListArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(25);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        if let Some(v_) = &value_.pagedResults {
            components_.push(|v_1: &PagedResultsRequest| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_PagedResultsRequest(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.listFamily {
            if *v_ != ListArgumentData::_default_value_for_listFamily() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != ListArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != ListArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != ListArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != ListArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// ListResult  ::=  OPTIONALLY-PROTECTED { ListResultData }
/// ```
pub type ListResult = OPTIONALLY_PROTECTED<ListResultData>; // DefinedType

pub fn _decode_ListResult(el: &X690Element) -> ASN1Result<ListResult> {
    _decode_OPTIONALLY_PROTECTED::<ListResultData>(_decode_ListResultData, &el)
}

pub fn _encode_ListResult(value_: &ListResult) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<ListResultData>(_encode_ListResultData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ListResultData  ::=  CHOICE {
///   listInfo                     SET {
///     name                         Name OPTIONAL,
///     subordinates            [1]  SET OF SEQUENCE {
///       rdn                          RelativeDistinguishedName,
///       aliasEntry              [0]  BOOLEAN DEFAULT FALSE,
///       fromEntry               [1]  BOOLEAN DEFAULT TRUE,
///       ... },
///     partialOutcomeQualifier [2]  PartialOutcomeQualifier OPTIONAL,
///     ...,
///     ...,
///     COMPONENTS OF                CommonResults
///     },
///   uncorrelatedListInfo    [0]  SET OF ListResult,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ListResultData {
    listInfo(ListResultData_listInfo),
    uncorrelatedListInfo(Vec<ListResult>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ListResultData {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ListResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ListResultData {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ListResultData(el)
    }
}

pub fn _decode_ListResultData(el: &X690Element) -> ASN1Result<ListResultData> {
    |el: &X690Element| -> ASN1Result<ListResultData> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 17) => Ok(ListResultData::listInfo(
                _decode_ListResultData_listInfo(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(ListResultData::uncorrelatedListInfo(
                |el: &X690Element| -> ASN1Result<SET_OF<ListResult>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<ListResult> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_ListResult(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )),
            _ => Ok(ListResultData::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ListResultData(value_: &ListResultData) -> ASN1Result<X690Element> {
    |value: &ListResultData| -> ASN1Result<X690Element> {
        match value {
            ListResultData::listInfo(v) => _encode_ListResultData_listInfo(&v),
            ListResultData::uncorrelatedListInfo(v) => {
                |v_1: &Vec<ListResult>| -> ASN1Result<X690Element> {
                    let el_1 = |value_: &SET_OF<ListResult>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ListResult(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ListResultData::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PartialOutcomeQualifier ::= SET {
///   limitProblem                  [0]  LimitProblem OPTIONAL,
///   unexplored                    [1]  SET SIZE (1..MAX) OF ContinuationReference OPTIONAL,
///   unavailableCriticalExtensions [2]  BOOLEAN DEFAULT FALSE,
///   unknownErrors                 [3]  SET SIZE (1..MAX) OF ABSTRACT-SYNTAX.&Type OPTIONAL,
///   queryReference                [4]  OCTET STRING OPTIONAL,
///   overspecFilter                [5]  Filter OPTIONAL,
///   notification                  [6]  SEQUENCE SIZE (1..MAX) OF
///                                        Attribute{{SupportedAttributes}} OPTIONAL,
///   entryCount                         CHOICE {
///     bestEstimate                  [7]  INTEGER,
///     lowEstimate                   [8]  INTEGER,
///     exact                         [9]  INTEGER,
///     ...} OPTIONAL
///   --                            [10] Not to be used -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PartialOutcomeQualifier {
    pub limitProblem: OPTIONAL<LimitProblem>,
    pub unexplored: OPTIONAL<Vec<ContinuationReference>>,
    pub unavailableCriticalExtensions: OPTIONAL<BOOLEAN>,
    pub unknownErrors: OPTIONAL<Vec<X690Element>>,
    pub queryReference: OPTIONAL<OCTET_STRING>,
    pub overspecFilter: OPTIONAL<Filter>,
    pub notification: OPTIONAL<Vec<Attribute>>,
    pub entryCount: OPTIONAL<PartialOutcomeQualifier_entryCount>,
}
impl PartialOutcomeQualifier {
    pub fn new(
        limitProblem: OPTIONAL<LimitProblem>,
        unexplored: OPTIONAL<Vec<ContinuationReference>>,
        unavailableCriticalExtensions: OPTIONAL<BOOLEAN>,
        unknownErrors: OPTIONAL<Vec<X690Element>>,
        queryReference: OPTIONAL<OCTET_STRING>,
        overspecFilter: OPTIONAL<Filter>,
        notification: OPTIONAL<Vec<Attribute>>,
        entryCount: OPTIONAL<PartialOutcomeQualifier_entryCount>,
    ) -> Self {
        PartialOutcomeQualifier {
            limitProblem,
            unexplored,
            unavailableCriticalExtensions,
            unknownErrors,
            queryReference,
            overspecFilter,
            notification,
            entryCount,
        }
    }
    pub fn _default_value_for_unavailableCriticalExtensions() -> BOOLEAN {
        false
    }
}
impl Default for PartialOutcomeQualifier {
    fn default() -> Self {
        PartialOutcomeQualifier {
            limitProblem: None,
            unexplored: None,
            unavailableCriticalExtensions: None,
            unknownErrors: None,
            queryReference: None,
            overspecFilter: None,
            notification: None,
            entryCount: None,
        }
    }
}
impl TryFrom<X690Element> for PartialOutcomeQualifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PartialOutcomeQualifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PartialOutcomeQualifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PartialOutcomeQualifier(el)
    }
}

pub const _rctl1_components_for_PartialOutcomeQualifier: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "limitProblem",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "unexplored",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "unavailableCriticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "unknownErrors",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "queryReference",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "overspecFilter",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notification",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryCount",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 7)),
            &TagSelector::tag((TagClass::CONTEXT, 8)),
            &TagSelector::tag((TagClass::CONTEXT, 9)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PartialOutcomeQualifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PartialOutcomeQualifier: &[ComponentSpec; 0] = &[];

pub fn _decode_PartialOutcomeQualifier(el: &X690Element) -> ASN1Result<PartialOutcomeQualifier> {
    |el_: &X690Element| -> ASN1Result<PartialOutcomeQualifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_PartialOutcomeQualifier,
            _eal_components_for_PartialOutcomeQualifier,
            _rctl2_components_for_PartialOutcomeQualifier,
            80,
        )?;
        let limitProblem: OPTIONAL<LimitProblem> = match _components.get("limitProblem") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<LimitProblem> {
                Ok(_decode_LimitProblem(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let unexplored: OPTIONAL<Vec<ContinuationReference>> = match _components.get("unexplored") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<ContinuationReference>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<ContinuationReference>> {
                            let elements = match el.value.borrow() {
                                X690Encoding::Constructed(children) => children,
                                _ => {
                                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction))
                                }
                            };
                            let mut items: SET_OF<ContinuationReference> =
                                Vec::with_capacity(elements.len());
                            for el in elements {
                                items.push(_decode_ContinuationReference(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(c_)?,
            ),
            _ => None,
        };
        let unavailableCriticalExtensions: OPTIONAL<BOOLEAN> =
            match _components.get("unavailableCriticalExtensions") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                    Ok(ber_decode_boolean(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let unknownErrors: OPTIONAL<Vec<X690Element>> = match _components.get("unknownErrors") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<X690Element>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let queryReference: OPTIONAL<OCTET_STRING> = match _components.get("queryReference") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(ber_decode_octet_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let overspecFilter: OPTIONAL<Filter> = match _components.get("overspecFilter") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Filter> {
                Ok(_decode_Filter(&el.inner()?)?)
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
        let entryCount: OPTIONAL<PartialOutcomeQualifier_entryCount> =
            match _components.get("entryCount") {
                Some(c_) => Some(_decode_PartialOutcomeQualifier_entryCount(c_)?),
                _ => None,
            };
        Ok(PartialOutcomeQualifier {
            limitProblem,
            unexplored,
            unavailableCriticalExtensions,
            unknownErrors,
            queryReference,
            overspecFilter,
            notification,
            entryCount,
        })
    }(&el)
}

pub fn _encode_PartialOutcomeQualifier(
    value_: &PartialOutcomeQualifier,
) -> ASN1Result<X690Element> {
    |value_: &PartialOutcomeQualifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.limitProblem {
            components_.push(|v_1: &LimitProblem| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_LimitProblem(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.unexplored {
            components_.push(
                |v_1: &Vec<ContinuationReference>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SET_OF<ContinuationReference>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_ContinuationReference(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.unavailableCriticalExtensions {
            if *v_ != PartialOutcomeQualifier::_default_value_for_unavailableCriticalExtensions() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.unknownErrors {
            components_.push(|v_1: &Vec<X690Element>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        X690Element,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(x690_identity(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.queryReference {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_octet_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.overspecFilter {
            components_.push(|v_1: &Filter| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.notification {
            components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
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
        if let Some(v_) = &value_.entryCount {
            components_.push(_encode_PartialOutcomeQualifier_entryCount(&v_)?);
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
/// LimitProblem  ::=  INTEGER {
///   timeLimitExceeded           (0),
///   sizeLimitExceeded           (1),
///   administrativeLimitExceeded (2) }
/// ```
pub type LimitProblem = INTEGER;

pub const LimitProblem_timeLimitExceeded: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const LimitProblem_sizeLimitExceeded: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const LimitProblem_administrativeLimitExceeded: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_LimitProblem(el: &X690Element) -> ASN1Result<LimitProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_LimitProblem(value_: &LimitProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// search OPERATION ::= {
///   ARGUMENT  SearchArgument
///   RESULT    SearchResult
///   ERRORS    {attributeError |
///              nameError |
///              serviceError |
///              referral |
///              abandoned |
///              securityError}
///   CODE      id-opcode-search }
/// ```
///
///
pub fn search() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            attributeError(),
            nameError(),
            serviceError(),
            referral(),
            abandoned(),
            securityError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_search), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchArgument  ::=  OPTIONALLY-PROTECTED { SearchArgumentData }
/// ```
pub type SearchArgument = OPTIONALLY_PROTECTED<SearchArgumentData>; // DefinedType

pub fn _decode_SearchArgument(el: &X690Element) -> ASN1Result<SearchArgument> {
    _decode_OPTIONALLY_PROTECTED::<SearchArgumentData>(_decode_SearchArgumentData, &el)
}

pub fn _encode_SearchArgument(value_: &SearchArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<SearchArgumentData>(_encode_SearchArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchArgumentData ::= SET {
///   baseObject            [0]  Name,
///   subset                [1]  INTEGER {
///     baseObject    (0),
///     oneLevel      (1),
///     wholeSubtree  (2)} DEFAULT baseObject,
///   filter                [2]  Filter DEFAULT and:{},
///   searchAliases         [3]  BOOLEAN DEFAULT TRUE,
///   selection             [4]  EntryInformationSelection DEFAULT {},
///   pagedResults          [5]  PagedResultsRequest OPTIONAL,
///   matchedValuesOnly     [6]  BOOLEAN DEFAULT FALSE,
///   extendedFilter        [7]  Filter OPTIONAL,
///   checkOverspecified    [8]  BOOLEAN DEFAULT FALSE,
///   relaxation            [9]  RelaxationPolicy OPTIONAL,
///   extendedArea          [10] INTEGER OPTIONAL,
///   hierarchySelections   [11] HierarchySelections DEFAULT {self},
///   searchControlOptions  [12] SearchControlOptions DEFAULT {searchAliases},
///   joinArguments         [13] SEQUENCE SIZE (1..MAX) OF JoinArgument OPTIONAL,
///   joinType              [14] ENUMERATED {
///     innerJoin      (0),
///     leftOuterJoin  (1),
///     fullOuterJoin  (2)} DEFAULT leftOuterJoin,
///   ...,
///   ...,
///   COMPONENTS OF              CommonArguments }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchArgumentData {
    pub baseObject: Name,
    pub subset: OPTIONAL<SearchArgumentData_subset>,
    pub filter: OPTIONAL<Filter>,
    pub searchAliases: OPTIONAL<BOOLEAN>,
    pub selection: OPTIONAL<EntryInformationSelection>,
    pub pagedResults: OPTIONAL<PagedResultsRequest>,
    pub matchedValuesOnly: OPTIONAL<BOOLEAN>,
    pub extendedFilter: OPTIONAL<Filter>,
    pub checkOverspecified: OPTIONAL<BOOLEAN>,
    pub relaxation: OPTIONAL<RelaxationPolicy>,
    pub extendedArea: OPTIONAL<INTEGER>,
    pub hierarchySelections: OPTIONAL<HierarchySelections>,
    pub searchControlOptions: OPTIONAL<SearchControlOptions>,
    pub joinArguments: OPTIONAL<Vec<JoinArgument>>,
    pub joinType: OPTIONAL<SearchArgumentData_joinType>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl SearchArgumentData {
    pub fn new(
        baseObject: Name,
        subset: OPTIONAL<SearchArgumentData_subset>,
        filter: OPTIONAL<Filter>,
        searchAliases: OPTIONAL<BOOLEAN>,
        selection: OPTIONAL<EntryInformationSelection>,
        pagedResults: OPTIONAL<PagedResultsRequest>,
        matchedValuesOnly: OPTIONAL<BOOLEAN>,
        extendedFilter: OPTIONAL<Filter>,
        checkOverspecified: OPTIONAL<BOOLEAN>,
        relaxation: OPTIONAL<RelaxationPolicy>,
        extendedArea: OPTIONAL<INTEGER>,
        hierarchySelections: OPTIONAL<HierarchySelections>,
        searchControlOptions: OPTIONAL<SearchControlOptions>,
        joinArguments: OPTIONAL<Vec<JoinArgument>>,
        joinType: OPTIONAL<SearchArgumentData_joinType>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        SearchArgumentData {
            baseObject,
            subset,
            filter,
            searchAliases,
            selection,
            pagedResults,
            matchedValuesOnly,
            extendedFilter,
            checkOverspecified,
            relaxation,
            extendedArea,
            hierarchySelections,
            searchControlOptions,
            joinArguments,
            joinType,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_subset() -> SearchArgumentData_subset {
        vec![SearchArgumentData_subset_baseObject as u8]
    }
    pub fn _default_value_for_filter() -> Filter {
        Filter::and(vec![])
    }
    pub fn _default_value_for_searchAliases() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_selection() -> EntryInformationSelection {
        EntryInformationSelection {
            attributes: None,
            infoTypes: None,
            extraAttributes: None,
            contextSelection: None,
            returnContexts: None,
            familyReturn: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_matchedValuesOnly() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_checkOverspecified() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_hierarchySelections() -> HierarchySelections {
        BIT_STRING::with_bits_set(&[HierarchySelections_self_])
    }
    pub fn _default_value_for_searchControlOptions() -> SearchControlOptions {
        BIT_STRING::with_bits_set(&[SearchControlOptions_searchAliases])
    }
    pub fn _default_value_for_joinType() -> SearchArgumentData_joinType {
        SearchArgumentData_joinType_leftOuterJoin
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for SearchArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchArgumentData(el)
    }
}

pub const _rctl1_components_for_SearchArgumentData: &[ComponentSpec; 15] = &[
    ComponentSpec::new(
        "baseObject",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subset",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "filter",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "searchAliases",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "selection",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pagedResults",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchedValuesOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extendedFilter",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "checkOverspecified",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "relaxation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extendedArea",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "hierarchySelections",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
    ComponentSpec::new(
        "searchControlOptions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 12)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinArguments",
        true,
        TagSelector::tag((TagClass::CONTEXT, 13)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 14)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_SearchArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchArgumentData(el: &X690Element) -> ASN1Result<SearchArgumentData> {
    |el_: &X690Element| -> ASN1Result<SearchArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SearchArgumentData,
            _eal_components_for_SearchArgumentData,
            _rctl2_components_for_SearchArgumentData,
            280,
        )?;
        let baseObject =
            |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
                _components.get("baseObject").unwrap(),
            )?;
        let subset: OPTIONAL<SearchArgumentData_subset> = match _components.get("subset") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SearchArgumentData_subset> {
                    Ok(_decode_SearchArgumentData_subset(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let filter: OPTIONAL<Filter> = match _components.get("filter") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Filter> {
                Ok(_decode_Filter(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let searchAliases: OPTIONAL<BOOLEAN> = match _components.get("searchAliases") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let selection: OPTIONAL<EntryInformationSelection> = match _components.get("selection") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<EntryInformationSelection> {
                    Ok(_decode_EntryInformationSelection(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let pagedResults: OPTIONAL<PagedResultsRequest> = match _components.get("pagedResults") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<PagedResultsRequest> {
                Ok(_decode_PagedResultsRequest(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let matchedValuesOnly: OPTIONAL<BOOLEAN> = match _components.get("matchedValuesOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let extendedFilter: OPTIONAL<Filter> = match _components.get("extendedFilter") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Filter> {
                Ok(_decode_Filter(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let checkOverspecified: OPTIONAL<BOOLEAN> = match _components.get("checkOverspecified") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let relaxation: OPTIONAL<RelaxationPolicy> = match _components.get("relaxation") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<RelaxationPolicy> {
                Ok(_decode_RelaxationPolicy(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let extendedArea: OPTIONAL<INTEGER> = match _components.get("extendedArea") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let hierarchySelections: OPTIONAL<HierarchySelections> =
            match _components.get("hierarchySelections") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<HierarchySelections> {
                    Ok(_decode_HierarchySelections(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let searchControlOptions: OPTIONAL<SearchControlOptions> =
            match _components.get("searchControlOptions") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SearchControlOptions> {
                    Ok(_decode_SearchControlOptions(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let joinArguments: OPTIONAL<Vec<JoinArgument>> = match _components.get("joinArguments") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<JoinArgument>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<JoinArgument>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<JoinArgument> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_JoinArgument(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let joinType: OPTIONAL<SearchArgumentData_joinType> = match _components.get("joinType") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SearchArgumentData_joinType> {
                    Ok(_decode_SearchArgumentData_joinType(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SearchArgumentData {
            baseObject,
            subset,
            filter,
            searchAliases,
            selection,
            pagedResults,
            matchedValuesOnly,
            extendedFilter,
            checkOverspecified,
            relaxation,
            extendedArea,
            hierarchySelections,
            searchControlOptions,
            joinArguments,
            joinType,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_SearchArgumentData(value_: &SearchArgumentData) -> ASN1Result<X690Element> {
    |value_: &SearchArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(37);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.baseObject)?);
        if let Some(v_) = &value_.subset {
            if *v_ != SearchArgumentData::_default_value_for_subset() {
                components_.push(
                    |v_1: &SearchArgumentData_subset| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            1,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_SearchArgumentData_subset(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.filter {
            components_.push(|v_1: &Filter| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.searchAliases {
            if *v_ != SearchArgumentData::_default_value_for_searchAliases() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.selection {
            components_.push(
                |v_1: &EntryInformationSelection| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_EntryInformationSelection(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.pagedResults {
            components_.push(|v_1: &PagedResultsRequest| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_PagedResultsRequest(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.matchedValuesOnly {
            if *v_ != SearchArgumentData::_default_value_for_matchedValuesOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        6,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.extendedFilter {
            components_.push(|v_1: &Filter| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.checkOverspecified {
            if *v_ != SearchArgumentData::_default_value_for_checkOverspecified() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        8,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.relaxation {
            components_.push(|v_1: &RelaxationPolicy| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_RelaxationPolicy(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.extendedArea {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    10,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.hierarchySelections {
            if *v_ != SearchArgumentData::_default_value_for_hierarchySelections() {
                components_.push(|v_1: &HierarchySelections| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        11,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_HierarchySelections(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.searchControlOptions {
            if *v_ != SearchArgumentData::_default_value_for_searchControlOptions() {
                components_.push(|v_1: &SearchControlOptions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        12,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SearchControlOptions(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.joinArguments {
            components_.push(|v_1: &Vec<JoinArgument>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    13,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        JoinArgument,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_JoinArgument(&v)?);
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
        if let Some(v_) = &value_.joinType {
            if *v_ != SearchArgumentData::_default_value_for_joinType() {
                components_.push(
                    |v_1: &SearchArgumentData_joinType| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            14,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_SearchArgumentData_joinType(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != SearchArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != SearchArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != SearchArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != SearchArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// HierarchySelections  ::=  BIT STRING {
///   self                  (0),
///   children              (1),
///   parent                (2),
///   hierarchy             (3),
///   top                   (4),
///   subtree               (5),
///   siblings              (6),
///   siblingChildren       (7),
///   siblingSubtree        (8),
///   all                   (9) }
/// ```
pub type HierarchySelections = BIT_STRING;

pub const HierarchySelections_self_: BIT = 0; /* LONG_NAMED_BIT */

pub const HierarchySelections_children: BIT = 1; /* LONG_NAMED_BIT */

pub const HierarchySelections_parent: BIT = 2; /* LONG_NAMED_BIT */

pub const HierarchySelections_hierarchy: BIT = 3; /* LONG_NAMED_BIT */

pub const HierarchySelections_top: BIT = 4; /* LONG_NAMED_BIT */

pub const HierarchySelections_subtree: BIT = 5; /* LONG_NAMED_BIT */

pub const HierarchySelections_siblings: BIT = 6; /* LONG_NAMED_BIT */

pub const HierarchySelections_siblingChildren: BIT = 7; /* LONG_NAMED_BIT */

pub const HierarchySelections_siblingSubtree: BIT = 8; /* LONG_NAMED_BIT */

pub const HierarchySelections_all: BIT = 9; /* LONG_NAMED_BIT */

pub fn _decode_HierarchySelections(el: &X690Element) -> ASN1Result<HierarchySelections> {
    ber_decode_bit_string(&el)
}

pub fn _encode_HierarchySelections(value_: &HierarchySelections) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchControlOptions  ::=  BIT STRING {
///   searchAliases         (0),
///   matchedValuesOnly     (1),
///   checkOverspecified    (2),
///   performExactly        (3),
///   includeAllAreas       (4),
///   noSystemRelaxation    (5),
///   dnAttribute           (6),
///   matchOnResidualName   (7),
///   entryCount            (8),
///   useSubset             (9),
///   separateFamilyMembers (10),
///   searchFamily          (11) }
/// ```
pub type SearchControlOptions = BIT_STRING;

pub const SearchControlOptions_searchAliases: BIT = 0; /* LONG_NAMED_BIT */

pub const SearchControlOptions_matchedValuesOnly: BIT = 1; /* LONG_NAMED_BIT */

pub const SearchControlOptions_checkOverspecified: BIT = 2; /* LONG_NAMED_BIT */

pub const SearchControlOptions_performExactly: BIT = 3; /* LONG_NAMED_BIT */

pub const SearchControlOptions_includeAllAreas: BIT = 4; /* LONG_NAMED_BIT */

pub const SearchControlOptions_noSystemRelaxation: BIT = 5; /* LONG_NAMED_BIT */

pub const SearchControlOptions_dnAttribute: BIT = 6; /* LONG_NAMED_BIT */

pub const SearchControlOptions_matchOnResidualName: BIT = 7; /* LONG_NAMED_BIT */

pub const SearchControlOptions_entryCount: BIT = 8; /* LONG_NAMED_BIT */

pub const SearchControlOptions_useSubset: BIT = 9; /* LONG_NAMED_BIT */

pub const SearchControlOptions_separateFamilyMembers: BIT = 10; /* LONG_NAMED_BIT */

pub const SearchControlOptions_searchFamily: BIT = 11; /* LONG_NAMED_BIT */

pub fn _decode_SearchControlOptions(el: &X690Element) -> ASN1Result<SearchControlOptions> {
    ber_decode_bit_string(&el)
}

pub fn _encode_SearchControlOptions(value_: &SearchControlOptions) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// JoinArgument ::= SEQUENCE {
///   joinBaseObject  [0]  Name,
///   domainLocalID   [1]  DomainLocalID OPTIONAL,
///   joinSubset      [2]  ENUMERATED {
///     baseObject   (0),
///     oneLevel     (1),
///     wholeSubtree (2),
///     ... } DEFAULT baseObject,
///   joinFilter      [3]  Filter OPTIONAL,
///   joinAttributes  [4]  SEQUENCE SIZE (1..MAX) OF JoinAttPair OPTIONAL,
///   joinSelection   [5]  EntryInformationSelection,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct JoinArgument {
    pub joinBaseObject: Name,
    pub domainLocalID: OPTIONAL<DomainLocalID>,
    pub joinSubset: OPTIONAL<JoinArgument_joinSubset>,
    pub joinFilter: OPTIONAL<Filter>,
    pub joinAttributes: OPTIONAL<Vec<JoinAttPair>>,
    pub joinSelection: EntryInformationSelection,
    pub _unrecognized: Vec<X690Element>,
}
impl JoinArgument {
    pub fn new(
        joinBaseObject: Name,
        domainLocalID: OPTIONAL<DomainLocalID>,
        joinSubset: OPTIONAL<JoinArgument_joinSubset>,
        joinFilter: OPTIONAL<Filter>,
        joinAttributes: OPTIONAL<Vec<JoinAttPair>>,
        joinSelection: EntryInformationSelection,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        JoinArgument {
            joinBaseObject,
            domainLocalID,
            joinSubset,
            joinFilter,
            joinAttributes,
            joinSelection,
            _unrecognized,
        }
    }
    pub fn _default_value_for_joinSubset() -> JoinArgument_joinSubset {
        JoinArgument_joinSubset_baseObject
    }
}
impl TryFrom<X690Element> for JoinArgument {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_JoinArgument(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for JoinArgument {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_JoinArgument(el)
    }
}

pub const _rctl1_components_for_JoinArgument: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "joinBaseObject",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "domainLocalID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinSubset",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinFilter",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinAttributes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinSelection",
        false,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_JoinArgument: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_JoinArgument: &[ComponentSpec; 0] = &[];

pub fn _decode_JoinArgument(el: &X690Element) -> ASN1Result<JoinArgument> {
    |el_: &X690Element| -> ASN1Result<JoinArgument> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_JoinArgument,
            _eal_components_for_JoinArgument,
            _rctl2_components_for_JoinArgument,
        )?;
        let joinBaseObject =
            |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
                _components.get("joinBaseObject").unwrap(),
            )?;
        let domainLocalID: OPTIONAL<DomainLocalID> = match _components.get("domainLocalID") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DomainLocalID> {
                Ok(_decode_DomainLocalID(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let joinSubset: OPTIONAL<JoinArgument_joinSubset> = match _components.get("joinSubset") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<JoinArgument_joinSubset> {
                Ok(_decode_JoinArgument_joinSubset(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let joinFilter: OPTIONAL<Filter> = match _components.get("joinFilter") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Filter> {
                Ok(_decode_Filter(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let joinAttributes: OPTIONAL<Vec<JoinAttPair>> = match _components.get("joinAttributes") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<JoinAttPair>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<JoinAttPair>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<JoinAttPair> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_JoinAttPair(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let joinSelection = |el: &X690Element| -> ASN1Result<EntryInformationSelection> {
            Ok(_decode_EntryInformationSelection(&el.inner()?)?)
        }(_components.get("joinSelection").unwrap())?;
        Ok(JoinArgument {
            joinBaseObject,
            domainLocalID,
            joinSubset,
            joinFilter,
            joinAttributes,
            joinSelection,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_JoinArgument(value_: &JoinArgument) -> ASN1Result<X690Element> {
    |value_: &JoinArgument| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.joinBaseObject)?);
        if let Some(v_) = &value_.domainLocalID {
            components_.push(|v_1: &DomainLocalID| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DomainLocalID(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.joinSubset {
            if *v_ != JoinArgument::_default_value_for_joinSubset() {
                components_.push(|v_1: &JoinArgument_joinSubset| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_JoinArgument_joinSubset(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.joinFilter {
            components_.push(|v_1: &Filter| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.joinAttributes {
            components_.push(|v_1: &Vec<JoinAttPair>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        JoinAttPair,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_JoinAttPair(&v)?);
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
        components_.push(
            |v_1: &EntryInformationSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_EntryInformationSelection(&v_1)?,
                    ))),
                ))
            }(&value_.joinSelection)?,
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
/// DomainLocalID  ::=  UnboundedDirectoryString
/// ```
pub type DomainLocalID = UnboundedDirectoryString; // DefinedType

pub fn _decode_DomainLocalID(el: &X690Element) -> ASN1Result<DomainLocalID> {
    _decode_UnboundedDirectoryString(&el)
}

pub fn _encode_DomainLocalID(value_: &DomainLocalID) -> ASN1Result<X690Element> {
    _encode_UnboundedDirectoryString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// JoinAttPair ::= SEQUENCE {
///   baseAtt      AttributeType,
///   joinAtt      AttributeType,
///   joinContext  SEQUENCE SIZE (1..MAX) OF JoinContextType OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct JoinAttPair {
    pub baseAtt: AttributeType,
    pub joinAtt: AttributeType,
    pub joinContext: OPTIONAL<Vec<JoinContextType>>,
    pub _unrecognized: Vec<X690Element>,
}
impl JoinAttPair {
    pub fn new(
        baseAtt: AttributeType,
        joinAtt: AttributeType,
        joinContext: OPTIONAL<Vec<JoinContextType>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        JoinAttPair {
            baseAtt,
            joinAtt,
            joinContext,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for JoinAttPair {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_JoinAttPair(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for JoinAttPair {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_JoinAttPair(el)
    }
}

pub const _rctl1_components_for_JoinAttPair: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "baseAtt",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinAtt",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "joinContext",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_JoinAttPair: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_JoinAttPair: &[ComponentSpec; 0] = &[];

pub fn _decode_JoinAttPair(el: &X690Element) -> ASN1Result<JoinAttPair> {
    |el_: &X690Element| -> ASN1Result<JoinAttPair> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_JoinAttPair,
            _eal_components_for_JoinAttPair,
            _rctl2_components_for_JoinAttPair,
        )?;
        let baseAtt = _decode_AttributeType(_components.get("baseAtt").unwrap())?;
        let joinAtt = _decode_AttributeType(_components.get("joinAtt").unwrap())?;
        let joinContext: OPTIONAL<Vec<JoinContextType>> = match _components.get("joinContext") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<JoinContextType>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<JoinContextType> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_JoinContextType(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        Ok(JoinAttPair {
            baseAtt,
            joinAtt,
            joinContext,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_JoinAttPair(value_: &JoinAttPair) -> ASN1Result<X690Element> {
    |value_: &JoinAttPair| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_AttributeType(&value_.baseAtt)?);
        components_.push(_encode_AttributeType(&value_.joinAtt)?);
        if let Some(v_) = &value_.joinContext {
            components_.push(
                |value_: &SEQUENCE_OF<JoinContextType>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_JoinContextType(&v)?);
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
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// JoinContextType  ::=  CONTEXT.&id({SupportedContexts})
/// ```
pub type JoinContextType = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_JoinContextType(el: &X690Element) -> ASN1Result<JoinContextType> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_JoinContextType(value_: &JoinContextType) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResult  ::=  OPTIONALLY-PROTECTED { SearchResultData }
/// ```
pub type SearchResult = OPTIONALLY_PROTECTED<SearchResultData>; // DefinedType

pub fn _decode_SearchResult(el: &X690Element) -> ASN1Result<SearchResult> {
    _decode_OPTIONALLY_PROTECTED::<SearchResultData>(_decode_SearchResultData, &el)
}

pub fn _encode_SearchResult(value_: &SearchResult) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<SearchResultData>(_encode_SearchResultData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResultData  ::=  CHOICE {
///   searchInfo                    SET {
///     name                          Name OPTIONAL,
///     entries                  [0]  SET OF EntryInformation,
///     partialOutcomeQualifier  [2]  PartialOutcomeQualifier OPTIONAL,
///     altMatching              [3]  BOOLEAN DEFAULT FALSE,
///     ...,
///     ...,
///     COMPONENTS OF                 CommonResults
///     },
///   uncorrelatedSearchInfo   [0]  SET OF SearchResult,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum SearchResultData {
    searchInfo(SearchResultData_searchInfo),
    uncorrelatedSearchInfo(Vec<SearchResult>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for SearchResultData {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchResultData {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchResultData(el)
    }
}

pub fn _decode_SearchResultData(el: &X690Element) -> ASN1Result<SearchResultData> {
    |el: &X690Element| -> ASN1Result<SearchResultData> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 17) => Ok(SearchResultData::searchInfo(
                _decode_SearchResultData_searchInfo(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(SearchResultData::uncorrelatedSearchInfo(
                |el: &X690Element| -> ASN1Result<SET_OF<SearchResult>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<SearchResult> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_SearchResult(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )),
            _ => Ok(SearchResultData::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_SearchResultData(value_: &SearchResultData) -> ASN1Result<X690Element> {
    |value: &SearchResultData| -> ASN1Result<X690Element> {
        match value {
            SearchResultData::searchInfo(v) => _encode_SearchResultData_searchInfo(&v),
            SearchResultData::uncorrelatedSearchInfo(v) => {
                |v_1: &Vec<SearchResult>| -> ASN1Result<X690Element> {
                    let el_1 = |value_: &SET_OF<SearchResult>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_SearchResult(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            SearchResultData::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// addEntry OPERATION ::= {
///   ARGUMENT  AddEntryArgument
///   RESULT    AddEntryResult
///   ERRORS    {attributeError |
///              nameError |
///              serviceError |
///              referral |
///              securityError |
///              updateError}
///   CODE      id-opcode-addEntry }
/// ```
///
///
pub fn addEntry() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            attributeError(),
            nameError(),
            serviceError(),
            referral(),
            securityError(),
            updateError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_addEntry), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddEntryArgument  ::=  OPTIONALLY-PROTECTED { AddEntryArgumentData }
/// ```
pub type AddEntryArgument = OPTIONALLY_PROTECTED<AddEntryArgumentData>; // DefinedType

pub fn _decode_AddEntryArgument(el: &X690Element) -> ASN1Result<AddEntryArgument> {
    _decode_OPTIONALLY_PROTECTED::<AddEntryArgumentData>(_decode_AddEntryArgumentData, &el)
}

pub fn _encode_AddEntryArgument(value_: &AddEntryArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<AddEntryArgumentData>(_encode_AddEntryArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddEntryArgumentData ::= SET {
///   object        [0]  Name,
///   entry         [1]  SET OF Attribute{{SupportedAttributes}},
///   targetSystem  [2]  AccessPoint OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF      CommonArguments }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddEntryArgumentData {
    pub object: Name,
    pub entry: Vec<Attribute>,
    pub targetSystem: OPTIONAL<AccessPoint>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl AddEntryArgumentData {
    pub fn new(
        object: Name,
        entry: Vec<Attribute>,
        targetSystem: OPTIONAL<AccessPoint>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        AddEntryArgumentData {
            object,
            entry,
            targetSystem,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for AddEntryArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddEntryArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddEntryArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddEntryArgumentData(el)
    }
}

pub const _rctl1_components_for_AddEntryArgumentData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entry",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "targetSystem",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AddEntryArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_AddEntryArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_AddEntryArgumentData(el: &X690Element) -> ASN1Result<AddEntryArgumentData> {
    |el_: &X690Element| -> ASN1Result<AddEntryArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_AddEntryArgumentData,
            _eal_components_for_AddEntryArgumentData,
            _rctl2_components_for_AddEntryArgumentData,
            160,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let entry = |el: &X690Element| -> ASN1Result<Vec<Attribute>> {
            Ok(|el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_Attribute(el)?);
                }
                Ok(items)
            }(&el.inner()?)?)
        }(_components.get("entry").unwrap())?;
        let targetSystem: OPTIONAL<AccessPoint> = match _components.get("targetSystem") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AccessPoint> {
                Ok(_decode_AccessPoint(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(AddEntryArgumentData {
            object,
            entry,
            targetSystem,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_AddEntryArgumentData(value_: &AddEntryArgumentData) -> ASN1Result<X690Element> {
    |value_: &AddEntryArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(25);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
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
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.entry)?);
        if let Some(v_) = &value_.targetSystem {
            components_.push(|v_1: &AccessPoint| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AccessPoint(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != AddEntryArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != AddEntryArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != AddEntryArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != AddEntryArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// AddEntryResult  ::=  CHOICE {
///   null          NULL,
///   information   OPTIONALLY-PROTECTED-SEQ { AddEntryResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum AddEntryResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<AddEntryResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AddEntryResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddEntryResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddEntryResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddEntryResult(el)
    }
}

pub fn _decode_AddEntryResult(el: &X690Element) -> ASN1Result<AddEntryResult> {
    |el: &X690Element| -> ASN1Result<AddEntryResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(AddEntryResult::null(ber_decode_null(&el)?)),
            (TagClass::CONTEXT, 0) => Ok(AddEntryResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<AddEntryResultData>(
                    _decode_AddEntryResultData,
                    &el,
                )?,
            )),
            _ => Ok(AddEntryResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AddEntryResult(value_: &AddEntryResult) -> ASN1Result<X690Element> {
    |value: &AddEntryResult| -> ASN1Result<X690Element> {
        match value {
            AddEntryResult::null(v) => ber_encode_null(&v),
            AddEntryResult::information(v) => {
                _encode_OPTIONALLY_PROTECTED_SEQ::<AddEntryResultData>(
                    _encode_AddEntryResultData,
                    &v,
                )
            }
            AddEntryResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddEntryResultData ::= SEQUENCE {
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddEntryResultData {
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl AddEntryResultData {
    pub fn new(
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        AddEntryResultData {
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
impl Default for AddEntryResultData {
    fn default() -> Self {
        AddEntryResultData {
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AddEntryResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddEntryResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddEntryResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddEntryResultData(el)
    }
}

pub const _rctl1_components_for_AddEntryResultData: &[ComponentSpec; 0] = &[];

pub const _rctl2_components_for_AddEntryResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_AddEntryResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_AddEntryResultData(el: &X690Element) -> ASN1Result<AddEntryResultData> {
    |el_: &X690Element| -> ASN1Result<AddEntryResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AddEntryResultData,
            _eal_components_for_AddEntryResultData,
            _rctl2_components_for_AddEntryResultData,
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
        Ok(AddEntryResultData {
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_AddEntryResultData(value_: &AddEntryResultData) -> ASN1Result<X690Element> {
    |value_: &AddEntryResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
            if *v_ != AddEntryResultData::_default_value_for_aliasDereferenced() {
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
/// removeEntry OPERATION ::= {
///   ARGUMENT  RemoveEntryArgument
///   RESULT    RemoveEntryResult
///   ERRORS    {nameError |
///              serviceError |
///              referral |
///              securityError |
///              updateError}
///   CODE      id-opcode-removeEntry }
/// ```
///
///
pub fn removeEntry() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            nameError(),
            serviceError(),
            referral(),
            securityError(),
            updateError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_removeEntry), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RemoveEntryArgument  ::=  OPTIONALLY-PROTECTED { RemoveEntryArgumentData }
/// ```
pub type RemoveEntryArgument = OPTIONALLY_PROTECTED<RemoveEntryArgumentData>; // DefinedType

pub fn _decode_RemoveEntryArgument(el: &X690Element) -> ASN1Result<RemoveEntryArgument> {
    _decode_OPTIONALLY_PROTECTED::<RemoveEntryArgumentData>(_decode_RemoveEntryArgumentData, &el)
}

pub fn _encode_RemoveEntryArgument(value_: &RemoveEntryArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<RemoveEntryArgumentData>(
        _encode_RemoveEntryArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RemoveEntryArgumentData ::= SET {
///   object     [0]  Name,
///   ...,
///   ...,
///   COMPONENTS OF   CommonArguments
///   }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RemoveEntryArgumentData {
    pub object: Name,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl RemoveEntryArgumentData {
    pub fn new(
        object: Name,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        RemoveEntryArgumentData {
            object,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for RemoveEntryArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RemoveEntryArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RemoveEntryArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RemoveEntryArgumentData(el)
    }
}

pub const _rctl1_components_for_RemoveEntryArgumentData: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    )];

pub const _rctl2_components_for_RemoveEntryArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_RemoveEntryArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_RemoveEntryArgumentData(el: &X690Element) -> ASN1Result<RemoveEntryArgumentData> {
    |el_: &X690Element| -> ASN1Result<RemoveEntryArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_RemoveEntryArgumentData,
            _eal_components_for_RemoveEntryArgumentData,
            _rctl2_components_for_RemoveEntryArgumentData,
            140,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(RemoveEntryArgumentData {
            object,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_RemoveEntryArgumentData(
    value_: &RemoveEntryArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &RemoveEntryArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(23);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != RemoveEntryArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != RemoveEntryArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != RemoveEntryArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != RemoveEntryArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// RemoveEntryResult  ::=  CHOICE {
///   null          NULL,
///   information   OPTIONALLY-PROTECTED-SEQ { RemoveEntryResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum RemoveEntryResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<RemoveEntryResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for RemoveEntryResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RemoveEntryResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RemoveEntryResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RemoveEntryResult(el)
    }
}

pub fn _decode_RemoveEntryResult(el: &X690Element) -> ASN1Result<RemoveEntryResult> {
    |el: &X690Element| -> ASN1Result<RemoveEntryResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(RemoveEntryResult::null(ber_decode_null(&el)?)),
            (TagClass::CONTEXT, 0) => Ok(RemoveEntryResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<RemoveEntryResultData>(
                    _decode_RemoveEntryResultData,
                    &el,
                )?,
            )),
            _ => Ok(RemoveEntryResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_RemoveEntryResult(value_: &RemoveEntryResult) -> ASN1Result<X690Element> {
    |value: &RemoveEntryResult| -> ASN1Result<X690Element> {
        match value {
            RemoveEntryResult::null(v) => ber_encode_null(&v),
            RemoveEntryResult::information(v) => _encode_OPTIONALLY_PROTECTED_SEQ::<
                RemoveEntryResultData,
            >(_encode_RemoveEntryResultData, &v),
            RemoveEntryResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RemoveEntryResultData ::= SEQUENCE {
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RemoveEntryResultData {
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl RemoveEntryResultData {
    pub fn new(
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        RemoveEntryResultData {
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
impl Default for RemoveEntryResultData {
    fn default() -> Self {
        RemoveEntryResultData {
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for RemoveEntryResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RemoveEntryResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RemoveEntryResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RemoveEntryResultData(el)
    }
}

pub const _rctl1_components_for_RemoveEntryResultData: &[ComponentSpec; 0] = &[];

pub const _rctl2_components_for_RemoveEntryResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_RemoveEntryResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_RemoveEntryResultData(el: &X690Element) -> ASN1Result<RemoveEntryResultData> {
    |el_: &X690Element| -> ASN1Result<RemoveEntryResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RemoveEntryResultData,
            _eal_components_for_RemoveEntryResultData,
            _rctl2_components_for_RemoveEntryResultData,
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
        Ok(RemoveEntryResultData {
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_RemoveEntryResultData(value_: &RemoveEntryResultData) -> ASN1Result<X690Element> {
    |value_: &RemoveEntryResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
            if *v_ != RemoveEntryResultData::_default_value_for_aliasDereferenced() {
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
/// modifyEntry OPERATION ::= {
///   ARGUMENT  ModifyEntryArgument
///   RESULT    ModifyEntryResult
///   ERRORS    {attributeError |
///              nameError |
///              serviceError |
///              referral |
///              securityError |
///              updateError}
///   CODE      id-opcode-modifyEntry }
/// ```
///
///
pub fn modifyEntry() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            attributeError(),
            nameError(),
            serviceError(),
            referral(),
            securityError(),
            updateError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_modifyEntry), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyEntryArgument  ::=  OPTIONALLY-PROTECTED { ModifyEntryArgumentData }
/// ```
pub type ModifyEntryArgument = OPTIONALLY_PROTECTED<ModifyEntryArgumentData>; // DefinedType

pub fn _decode_ModifyEntryArgument(el: &X690Element) -> ASN1Result<ModifyEntryArgument> {
    _decode_OPTIONALLY_PROTECTED::<ModifyEntryArgumentData>(_decode_ModifyEntryArgumentData, &el)
}

pub fn _encode_ModifyEntryArgument(value_: &ModifyEntryArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<ModifyEntryArgumentData>(
        _encode_ModifyEntryArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyEntryArgumentData ::= SET {
///   object     [0]  Name,
///   changes    [1]  SEQUENCE OF EntryModification,
///   selection  [2]  EntryInformationSelection OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF   CommonArguments }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyEntryArgumentData {
    pub object: Name,
    pub changes: Vec<EntryModification>,
    pub selection: OPTIONAL<EntryInformationSelection>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl ModifyEntryArgumentData {
    pub fn new(
        object: Name,
        changes: Vec<EntryModification>,
        selection: OPTIONAL<EntryInformationSelection>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        ModifyEntryArgumentData {
            object,
            changes,
            selection,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for ModifyEntryArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyEntryArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyEntryArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyEntryArgumentData(el)
    }
}

pub const _rctl1_components_for_ModifyEntryArgumentData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "changes",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "selection",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyEntryArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_ModifyEntryArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyEntryArgumentData(el: &X690Element) -> ASN1Result<ModifyEntryArgumentData> {
    |el_: &X690Element| -> ASN1Result<ModifyEntryArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyEntryArgumentData,
            _eal_components_for_ModifyEntryArgumentData,
            _rctl2_components_for_ModifyEntryArgumentData,
            160,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let changes = |el: &X690Element| -> ASN1Result<Vec<EntryModification>> {
            Ok(
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
                }(&el.inner()?)?,
            )
        }(_components.get("changes").unwrap())?;
        let selection: OPTIONAL<EntryInformationSelection> = match _components.get("selection") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<EntryInformationSelection> {
                    Ok(_decode_EntryInformationSelection(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ModifyEntryArgumentData {
            object,
            changes,
            selection,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_ModifyEntryArgumentData(
    value_: &ModifyEntryArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &ModifyEntryArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(25);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        components_.push(|v_1: &Vec<EntryModification>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                    EntryModification,
                >|
                 -> ASN1Result<
                    X690Element,
                > {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_EntryModification(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.changes)?);
        if let Some(v_) = &value_.selection {
            components_.push(
                |v_1: &EntryInformationSelection| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_EntryInformationSelection(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != ModifyEntryArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != ModifyEntryArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != ModifyEntryArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != ModifyEntryArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// ModifyEntryResult  ::=  CHOICE {
///   null         NULL,
///   information  OPTIONALLY-PROTECTED-SEQ { ModifyEntryResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ModifyEntryResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<ModifyEntryResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ModifyEntryResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyEntryResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyEntryResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyEntryResult(el)
    }
}

pub fn _decode_ModifyEntryResult(el: &X690Element) -> ASN1Result<ModifyEntryResult> {
    |el: &X690Element| -> ASN1Result<ModifyEntryResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(ModifyEntryResult::null(ber_decode_null(&el)?)),
            (TagClass::CONTEXT, 0) => Ok(ModifyEntryResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<ModifyEntryResultData>(
                    _decode_ModifyEntryResultData,
                    &el,
                )?,
            )),
            _ => Ok(ModifyEntryResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ModifyEntryResult(value_: &ModifyEntryResult) -> ASN1Result<X690Element> {
    |value: &ModifyEntryResult| -> ASN1Result<X690Element> {
        match value {
            ModifyEntryResult::null(v) => ber_encode_null(&v),
            ModifyEntryResult::information(v) => _encode_OPTIONALLY_PROTECTED_SEQ::<
                ModifyEntryResultData,
            >(_encode_ModifyEntryResultData, &v),
            ModifyEntryResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyEntryResultData ::= SEQUENCE {
///   entry    [0]  EntryInformation OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyEntryResultData {
    pub entry: OPTIONAL<EntryInformation>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ModifyEntryResultData {
    pub fn new(
        entry: OPTIONAL<EntryInformation>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ModifyEntryResultData {
            entry,
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
impl Default for ModifyEntryResultData {
    fn default() -> Self {
        ModifyEntryResultData {
            entry: None,
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ModifyEntryResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyEntryResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyEntryResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyEntryResultData(el)
    }
}

pub const _rctl1_components_for_ModifyEntryResultData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "entry",
    true,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_ModifyEntryResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ModifyEntryResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyEntryResultData(el: &X690Element) -> ASN1Result<ModifyEntryResultData> {
    |el_: &X690Element| -> ASN1Result<ModifyEntryResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyEntryResultData,
            _eal_components_for_ModifyEntryResultData,
            _rctl2_components_for_ModifyEntryResultData,
        )?;
        let entry: OPTIONAL<EntryInformation> = match _components.get("entry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<EntryInformation> {
                Ok(_decode_EntryInformation(&el.inner()?)?)
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
        Ok(ModifyEntryResultData {
            entry,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ModifyEntryResultData(value_: &ModifyEntryResultData) -> ASN1Result<X690Element> {
    |value_: &ModifyEntryResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.entry {
            components_.push(|v_1: &EntryInformation| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_EntryInformation(
                        &v_1,
                    )?))),
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
            if *v_ != ModifyEntryResultData::_default_value_for_aliasDereferenced() {
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
/// EntryModification  ::=  CHOICE {
///   addAttribute     [0]  Attribute{{SupportedAttributes}},
///   removeAttribute  [1]  AttributeType,
///   addValues        [2]  Attribute{{SupportedAttributes}},
///   removeValues     [3]  Attribute{{SupportedAttributes}},
///   alterValues      [4]  AttributeTypeAndValue,
///   resetValue       [5]  AttributeType,
///   replaceValues    [6]  Attribute{{SupportedAttributes}},
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum EntryModification {
    addAttribute(Attribute),
    removeAttribute(AttributeType),
    addValues(Attribute),
    removeValues(Attribute),
    alterValues(AttributeTypeAndValue),
    resetValue(AttributeType),
    replaceValues(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for EntryModification {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryModification(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryModification {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryModification(el)
    }
}

pub fn _decode_EntryModification(el: &X690Element) -> ASN1Result<EntryModification> {
    |el: &X690Element| -> ASN1Result<EntryModification> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(EntryModification::addAttribute(_decode_Attribute(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(EntryModification::removeAttribute(
                _decode_AttributeType(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(EntryModification::addValues(_decode_Attribute(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 3) => Ok(EntryModification::removeValues(_decode_Attribute(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 4) => Ok(EntryModification::alterValues(
                _decode_AttributeTypeAndValue(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 5) => Ok(EntryModification::resetValue(_decode_AttributeType(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 6) => Ok(EntryModification::replaceValues(_decode_Attribute(
                &el.inner()?,
            )?)),
            _ => Ok(EntryModification::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_EntryModification(value_: &EntryModification) -> ASN1Result<X690Element> {
    |value: &EntryModification| -> ASN1Result<X690Element> {
        match value {
            EntryModification::addAttribute(v) => |v_1: &Attribute| -> ASN1Result<X690Element> {
                let el_1 = _encode_Attribute(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            EntryModification::removeAttribute(v) => {
                |v_1: &AttributeType| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeType(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EntryModification::addValues(v) => |v_1: &Attribute| -> ASN1Result<X690Element> {
                let el_1 = _encode_Attribute(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            EntryModification::removeValues(v) => |v_1: &Attribute| -> ASN1Result<X690Element> {
                let el_1 = _encode_Attribute(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            EntryModification::alterValues(v) => {
                |v_1: &AttributeTypeAndValue| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeTypeAndValue(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EntryModification::resetValue(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let el_1 = _encode_AttributeType(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            EntryModification::replaceValues(v) => |v_1: &Attribute| -> ASN1Result<X690Element> {
                let el_1 = _encode_Attribute(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            EntryModification::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// modifyDN OPERATION ::= {
///   ARGUMENT  ModifyDNArgument
///   RESULT    ModifyDNResult
///   ERRORS    {nameError |
///              serviceError |
///              referral |
///              securityError |
///              updateError}
///   CODE      id-opcode-modifyDN }
/// ```
///
///
pub fn modifyDN() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([
            nameError(),
            serviceError(),
            referral(),
            securityError(),
            updateError(),
        ])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_modifyDN), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNArgument  ::=  OPTIONALLY-PROTECTED { ModifyDNArgumentData }
/// ```
pub type ModifyDNArgument = OPTIONALLY_PROTECTED<ModifyDNArgumentData>; // DefinedType

pub fn _decode_ModifyDNArgument(el: &X690Element) -> ASN1Result<ModifyDNArgument> {
    _decode_OPTIONALLY_PROTECTED::<ModifyDNArgumentData>(_decode_ModifyDNArgumentData, &el)
}

pub fn _encode_ModifyDNArgument(value_: &ModifyDNArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED::<ModifyDNArgumentData>(_encode_ModifyDNArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNArgumentData ::= SET {
///   object        [0]  DistinguishedName,
///   newRDN        [1]  RelativeDistinguishedName,
///   deleteOldRDN  [2]  BOOLEAN DEFAULT FALSE,
///   newSuperior   [3]  DistinguishedName OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF      CommonArguments }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyDNArgumentData {
    pub object: DistinguishedName,
    pub newRDN: RelativeDistinguishedName,
    pub deleteOldRDN: OPTIONAL<BOOLEAN>,
    pub newSuperior: OPTIONAL<DistinguishedName>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl ModifyDNArgumentData {
    pub fn new(
        object: DistinguishedName,
        newRDN: RelativeDistinguishedName,
        deleteOldRDN: OPTIONAL<BOOLEAN>,
        newSuperior: OPTIONAL<DistinguishedName>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        ModifyDNArgumentData {
            object,
            newRDN,
            deleteOldRDN,
            newSuperior,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_deleteOldRDN() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for ModifyDNArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyDNArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNArgumentData(el)
    }
}

pub const _rctl1_components_for_ModifyDNArgumentData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newRDN",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "deleteOldRDN",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newSuperior",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyDNArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_ModifyDNArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyDNArgumentData(el: &X690Element) -> ASN1Result<ModifyDNArgumentData> {
    |el_: &X690Element| -> ASN1Result<ModifyDNArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyDNArgumentData,
            _eal_components_for_ModifyDNArgumentData,
            _rctl2_components_for_ModifyDNArgumentData,
            170,
        )?;
        let object = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("object").unwrap())?;
        let newRDN = |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
        }(_components.get("newRDN").unwrap())?;
        let deleteOldRDN: OPTIONAL<BOOLEAN> = match _components.get("deleteOldRDN") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let newSuperior: OPTIONAL<DistinguishedName> = match _components.get("newSuperior") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ModifyDNArgumentData {
            object,
            newRDN,
            deleteOldRDN,
            newSuperior,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_ModifyDNArgumentData(value_: &ModifyDNArgumentData) -> ASN1Result<X690Element> {
    |value_: &ModifyDNArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(26);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.object)?);
        components_.push(
            |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_RelativeDistinguishedName(&v_1)?,
                    ))),
                ))
            }(&value_.newRDN)?,
        );
        if let Some(v_) = &value_.deleteOldRDN {
            if *v_ != ModifyDNArgumentData::_default_value_for_deleteOldRDN() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.newSuperior {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != ModifyDNArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != ModifyDNArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != ModifyDNArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != ModifyDNArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
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
/// ModifyDNResult  ::=  CHOICE {
///   null         NULL,
///   information  OPTIONALLY-PROTECTED-SEQ { ModifyDNResultData },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ModifyDNResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<ModifyDNResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ModifyDNResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyDNResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNResult(el)
    }
}

pub fn _decode_ModifyDNResult(el: &X690Element) -> ASN1Result<ModifyDNResult> {
    |el: &X690Element| -> ASN1Result<ModifyDNResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(ModifyDNResult::null(())),
            (TagClass::CONTEXT, 0) => Ok(ModifyDNResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<ModifyDNResultData>(
                    _decode_ModifyDNResultData,
                    &el,
                )?,
            )),
            _ => Ok(ModifyDNResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ModifyDNResult(value_: &ModifyDNResult) -> ASN1Result<X690Element> {
    |value: &ModifyDNResult| -> ASN1Result<X690Element> {
        match value {
            ModifyDNResult::null(v) => ber_encode_null(&v),
            ModifyDNResult::information(v) => {
                _encode_OPTIONALLY_PROTECTED_SEQ::<ModifyDNResultData>(
                    _encode_ModifyDNResultData,
                    &v,
                )
            }
            ModifyDNResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNResultData ::= SEQUENCE {
///   newRDN        RelativeDistinguishedName,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyDNResultData {
    pub newRDN: RelativeDistinguishedName,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ModifyDNResultData {
    pub fn new(
        newRDN: RelativeDistinguishedName,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ModifyDNResultData {
            newRDN,
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
impl TryFrom<X690Element> for ModifyDNResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyDNResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNResultData(el)
    }
}

pub const _rctl1_components_for_ModifyDNResultData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "newRDN",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 17)),
    None,
    None,
)];

pub const _rctl2_components_for_ModifyDNResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ModifyDNResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyDNResultData(el: &X690Element) -> ASN1Result<ModifyDNResultData> {
    |el_: &X690Element| -> ASN1Result<ModifyDNResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyDNResultData,
            _eal_components_for_ModifyDNResultData,
            _rctl2_components_for_ModifyDNResultData,
        )?;
        let newRDN = _decode_RelativeDistinguishedName(_components.get("newRDN").unwrap())?;
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
        Ok(ModifyDNResultData {
            newRDN,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ModifyDNResultData(value_: &ModifyDNResultData) -> ASN1Result<X690Element> {
    |value_: &ModifyDNResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_RelativeDistinguishedName(&value_.newRDN)?);
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
            if *v_ != ModifyDNResultData::_default_value_for_aliasDereferenced() {
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
/// changePassword OPERATION ::= {
///   ARGUMENT  ChangePasswordArgument
///   RESULT    ChangePasswordResult
///   ERRORS    {securityError |
///              updateError }
///   CODE      id-opcode-changePassword }
/// ```
///
///
pub fn changePassword() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([securityError(), updateError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_changePassword),                  /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChangePasswordArgument  ::=  OPTIONALLY-PROTECTED-SEQ { ChangePasswordArgumentData }
/// ```
pub type ChangePasswordArgument = OPTIONALLY_PROTECTED_SEQ<ChangePasswordArgumentData>; // DefinedType

pub fn _decode_ChangePasswordArgument(el: &X690Element) -> ASN1Result<ChangePasswordArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<ChangePasswordArgumentData>(
        _decode_ChangePasswordArgumentData,
        &el,
    )
}

pub fn _encode_ChangePasswordArgument(value_: &ChangePasswordArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<ChangePasswordArgumentData>(
        _encode_ChangePasswordArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChangePasswordArgumentData ::= SEQUENCE {
///   object   [0]  DistinguishedName,
///   oldPwd   [1]  UserPwd,
///   newPwd   [2]  UserPwd,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ChangePasswordArgumentData {
    pub object: DistinguishedName,
    pub oldPwd: UserPwd,
    pub newPwd: UserPwd,
    pub _unrecognized: Vec<X690Element>,
}
impl ChangePasswordArgumentData {
    pub fn new(
        object: DistinguishedName,
        oldPwd: UserPwd,
        newPwd: UserPwd,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ChangePasswordArgumentData {
            object,
            oldPwd,
            newPwd,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ChangePasswordArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ChangePasswordArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ChangePasswordArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ChangePasswordArgumentData(el)
    }
}

pub const _rctl1_components_for_ChangePasswordArgumentData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "oldPwd",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newPwd",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ChangePasswordArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ChangePasswordArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_ChangePasswordArgumentData(
    el: &X690Element,
) -> ASN1Result<ChangePasswordArgumentData> {
    |el_: &X690Element| -> ASN1Result<ChangePasswordArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ChangePasswordArgumentData,
            _eal_components_for_ChangePasswordArgumentData,
            _rctl2_components_for_ChangePasswordArgumentData,
        )?;
        let object = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("object").unwrap())?;
        let oldPwd =
            |el: &X690Element| -> ASN1Result<UserPwd> { Ok(_decode_UserPwd(&el.inner()?)?) }(
                _components.get("oldPwd").unwrap(),
            )?;
        let newPwd =
            |el: &X690Element| -> ASN1Result<UserPwd> { Ok(_decode_UserPwd(&el.inner()?)?) }(
                _components.get("newPwd").unwrap(),
            )?;
        Ok(ChangePasswordArgumentData {
            object,
            oldPwd,
            newPwd,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ChangePasswordArgumentData(
    value_: &ChangePasswordArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &ChangePasswordArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.object)?);
        components_.push(|v_1: &UserPwd| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UserPwd(&v_1)?))),
            ))
        }(&value_.oldPwd)?);
        components_.push(|v_1: &UserPwd| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UserPwd(&v_1)?))),
            ))
        }(&value_.newPwd)?);
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
/// ChangePasswordResult  ::=  CHOICE {
///   null        NULL,
///   information OPTIONALLY-PROTECTED-SEQ { ChangePasswordResultData },
///   ...}
/// ```
#[derive(Debug, Clone)]
pub enum ChangePasswordResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<ChangePasswordResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ChangePasswordResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ChangePasswordResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ChangePasswordResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ChangePasswordResult(el)
    }
}

pub fn _decode_ChangePasswordResult(el: &X690Element) -> ASN1Result<ChangePasswordResult> {
    |el: &X690Element| -> ASN1Result<ChangePasswordResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(ChangePasswordResult::null(ber_decode_null(&el)?)),
            (TagClass::CONTEXT, 0) => Ok(ChangePasswordResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<ChangePasswordResultData>(
                    _decode_ChangePasswordResultData,
                    &el,
                )?,
            )),
            _ => Ok(ChangePasswordResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ChangePasswordResult(value_: &ChangePasswordResult) -> ASN1Result<X690Element> {
    |value: &ChangePasswordResult| -> ASN1Result<X690Element> {
        match value {
            ChangePasswordResult::null(v) => ber_encode_null(&v),
            ChangePasswordResult::information(v) => _encode_OPTIONALLY_PROTECTED_SEQ::<
                ChangePasswordResultData,
            >(
                _encode_ChangePasswordResultData, &v
            ),
            ChangePasswordResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChangePasswordResultData ::= SEQUENCE {
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ChangePasswordResultData {
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ChangePasswordResultData {
    pub fn new(
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ChangePasswordResultData {
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
impl Default for ChangePasswordResultData {
    fn default() -> Self {
        ChangePasswordResultData {
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ChangePasswordResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ChangePasswordResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ChangePasswordResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ChangePasswordResultData(el)
    }
}

pub const _rctl1_components_for_ChangePasswordResultData: &[ComponentSpec; 0] = &[];

pub const _rctl2_components_for_ChangePasswordResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ChangePasswordResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_ChangePasswordResultData(el: &X690Element) -> ASN1Result<ChangePasswordResultData> {
    |el_: &X690Element| -> ASN1Result<ChangePasswordResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ChangePasswordResultData,
            _eal_components_for_ChangePasswordResultData,
            _rctl2_components_for_ChangePasswordResultData,
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
        Ok(ChangePasswordResultData {
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ChangePasswordResultData(
    value_: &ChangePasswordResultData,
) -> ASN1Result<X690Element> {
    |value_: &ChangePasswordResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
            if *v_ != ChangePasswordResultData::_default_value_for_aliasDereferenced() {
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
/// administerPassword OPERATION ::= {
///   ARGUMENT  AdministerPasswordArgument
///   RESULT    AdministerPasswordResult
///   ERRORS    {securityError |
///              updateError}
///   CODE      id-opcode-administerPassword }
/// ```
///
///
pub fn administerPassword() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([securityError(), updateError()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_administerPassword),              /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AdministerPasswordArgument  ::=
///   OPTIONALLY-PROTECTED-SEQ { AdministerPasswordArgumentData }
/// ```
pub type AdministerPasswordArgument = OPTIONALLY_PROTECTED_SEQ<AdministerPasswordArgumentData>; // DefinedType

pub fn _decode_AdministerPasswordArgument(
    el: &X690Element,
) -> ASN1Result<AdministerPasswordArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<AdministerPasswordArgumentData>(
        _decode_AdministerPasswordArgumentData,
        &el,
    )
}

pub fn _encode_AdministerPasswordArgument(
    value_: &AdministerPasswordArgument,
) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<AdministerPasswordArgumentData>(
        _encode_AdministerPasswordArgumentData,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AdministerPasswordArgumentData ::= SEQUENCE {
///   object  [0]  DistinguishedName,
///   newPwd  [1]  UserPwd,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AdministerPasswordArgumentData {
    pub object: DistinguishedName,
    pub newPwd: UserPwd,
    pub _unrecognized: Vec<X690Element>,
}
impl AdministerPasswordArgumentData {
    pub fn new(
        object: DistinguishedName,
        newPwd: UserPwd,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AdministerPasswordArgumentData {
            object,
            newPwd,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AdministerPasswordArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AdministerPasswordArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AdministerPasswordArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AdministerPasswordArgumentData(el)
    }
}

pub const _rctl1_components_for_AdministerPasswordArgumentData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newPwd",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AdministerPasswordArgumentData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AdministerPasswordArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_AdministerPasswordArgumentData(
    el: &X690Element,
) -> ASN1Result<AdministerPasswordArgumentData> {
    |el_: &X690Element| -> ASN1Result<AdministerPasswordArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AdministerPasswordArgumentData,
            _eal_components_for_AdministerPasswordArgumentData,
            _rctl2_components_for_AdministerPasswordArgumentData,
        )?;
        let object = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("object").unwrap())?;
        let newPwd =
            |el: &X690Element| -> ASN1Result<UserPwd> { Ok(_decode_UserPwd(&el.inner()?)?) }(
                _components.get("newPwd").unwrap(),
            )?;
        Ok(AdministerPasswordArgumentData {
            object,
            newPwd,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AdministerPasswordArgumentData(
    value_: &AdministerPasswordArgumentData,
) -> ASN1Result<X690Element> {
    |value_: &AdministerPasswordArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.object)?);
        components_.push(|v_1: &UserPwd| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UserPwd(&v_1)?))),
            ))
        }(&value_.newPwd)?);
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
/// AdministerPasswordResult  ::=  CHOICE {
///   null NULL,
///   information OPTIONALLY-PROTECTED-SEQ { AdministerPasswordResultData },
///   ...}
/// ```
#[derive(Debug, Clone)]
pub enum AdministerPasswordResult {
    null(NULL),
    information(OPTIONALLY_PROTECTED_SEQ<AdministerPasswordResultData>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AdministerPasswordResult {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AdministerPasswordResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AdministerPasswordResult {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AdministerPasswordResult(el)
    }
}

pub fn _decode_AdministerPasswordResult(el: &X690Element) -> ASN1Result<AdministerPasswordResult> {
    |el: &X690Element| -> ASN1Result<AdministerPasswordResult> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(AdministerPasswordResult::null(ber_decode_null(&el)?)),
            (TagClass::CONTEXT, 0) => Ok(AdministerPasswordResult::information(
                _decode_OPTIONALLY_PROTECTED_SEQ::<AdministerPasswordResultData>(
                    _decode_AdministerPasswordResultData,
                    &el,
                )?,
            )),
            _ => Ok(AdministerPasswordResult::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AdministerPasswordResult(
    value_: &AdministerPasswordResult,
) -> ASN1Result<X690Element> {
    |value: &AdministerPasswordResult| -> ASN1Result<X690Element> {
        match value {
            AdministerPasswordResult::null(v) => ber_encode_null(&v),
            AdministerPasswordResult::information(v) => _encode_OPTIONALLY_PROTECTED_SEQ::<
                AdministerPasswordResultData,
            >(
                _encode_AdministerPasswordResultData, &v
            ),
            AdministerPasswordResult::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AdministerPasswordResultData ::= SEQUENCE {
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AdministerPasswordResultData {
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl AdministerPasswordResultData {
    pub fn new(
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        AdministerPasswordResultData {
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
impl Default for AdministerPasswordResultData {
    fn default() -> Self {
        AdministerPasswordResultData {
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AdministerPasswordResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AdministerPasswordResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AdministerPasswordResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AdministerPasswordResultData(el)
    }
}

pub const _rctl1_components_for_AdministerPasswordResultData: &[ComponentSpec; 0] = &[];

pub const _rctl2_components_for_AdministerPasswordResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_AdministerPasswordResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_AdministerPasswordResultData(
    el: &X690Element,
) -> ASN1Result<AdministerPasswordResultData> {
    |el_: &X690Element| -> ASN1Result<AdministerPasswordResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AdministerPasswordResultData,
            _eal_components_for_AdministerPasswordResultData,
            _rctl2_components_for_AdministerPasswordResultData,
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
        Ok(AdministerPasswordResultData {
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_AdministerPasswordResultData(
    value_: &AdministerPasswordResultData,
) -> ASN1Result<X690Element> {
    |value_: &AdministerPasswordResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
            if *v_ != AdministerPasswordResultData::_default_value_for_aliasDereferenced() {
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
/// ldapTransport OPERATION ::= {
///   ARGUMENT    LdapArgument
///   RESULT      SEQUENCE OF LDAPMessage
///   ERRORS      { abandonFailed | abandoned }
///   CODE        id-opcode-ldapTransport }
/// ```
///
///
pub fn ldapTransport() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([abandonFailed(), abandoned()])), /* OBJECT_FIELD_SETTING */
        operationCode: Some(id_opcode_ldapTransport),                 /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LdapArgument  ::=  OPTIONALLY-PROTECTED-SEQ { LdapArgumentData }
/// ```
pub type LdapArgument = OPTIONALLY_PROTECTED_SEQ<LdapArgumentData>; // DefinedType

pub fn _decode_LdapArgument(el: &X690Element) -> ASN1Result<LdapArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<LdapArgumentData>(_decode_LdapArgumentData, &el)
}

pub fn _encode_LdapArgument(value_: &LdapArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<LdapArgumentData>(_encode_LdapArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LdapArgumentData ::= SEQUENCE {
///   object        DistinguishedName,
///   ldapMessage   LDAPMessage,
///   linkId        LinkId  OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonArgumentsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LdapArgumentData {
    pub object: DistinguishedName,
    pub ldapMessage: LDAPMessage,
    pub linkId: OPTIONAL<LinkId>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl LdapArgumentData {
    pub fn new(
        object: DistinguishedName,
        ldapMessage: LDAPMessage,
        linkId: OPTIONAL<LinkId>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        LdapArgumentData {
            object,
            ldapMessage,
            linkId,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for LdapArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LdapArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LdapArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LdapArgumentData(el)
    }
}

pub const _rctl1_components_for_LdapArgumentData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ldapMessage",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "linkId",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_LdapArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_LdapArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_LdapArgumentData(el: &X690Element) -> ASN1Result<LdapArgumentData> {
    |el_: &X690Element| -> ASN1Result<LdapArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_LdapArgumentData,
            _eal_components_for_LdapArgumentData,
            _rctl2_components_for_LdapArgumentData,
        )?;
        let object = _decode_DistinguishedName(_components.get("object").unwrap())?;
        let ldapMessage = _decode_LDAPMessage(_components.get("ldapMessage").unwrap())?;
        let linkId: OPTIONAL<LinkId> = match _components.get("linkId") {
            Some(c_) => Some(_decode_LinkId(c_)?),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(LdapArgumentData {
            object,
            ldapMessage,
            linkId,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_LdapArgumentData(value_: &LdapArgumentData) -> ASN1Result<X690Element> {
    |value_: &LdapArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(25);
        components_.push(_encode_DistinguishedName(&value_.object)?);
        components_.push(_encode_LDAPMessage(&value_.ldapMessage)?);
        if let Some(v_) = &value_.linkId {
            components_.push(_encode_LinkId(&v_)?);
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != LdapArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != LdapArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != LdapArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != LdapArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
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
/// LinkId  ::=  INTEGER
/// ```
pub type LinkId = INTEGER;

pub fn _decode_LinkId(el: &X690Element) -> ASN1Result<LinkId> {
    ber_decode_integer(&el)
}

pub fn _encode_LinkId(value_: &LinkId) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LdapResult  ::=  OPTIONALLY-PROTECTED-SEQ { LdapResultData }
/// ```
pub type LdapResult = OPTIONALLY_PROTECTED_SEQ<LdapResultData>; // DefinedType

pub fn _decode_LdapResult(el: &X690Element) -> ASN1Result<LdapResult> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<LdapResultData>(_decode_LdapResultData, &el)
}

pub fn _encode_LdapResult(value_: &LdapResult) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<LdapResultData>(_encode_LdapResultData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LdapResultData ::= SEQUENCE {
///   ldapMessages   SEQUENCE SIZE (1..MAX) OF LDAPMessage OPTIONAL,
///   returnToClient BOOLEAN DEFAULT FALSE,
///   ...,
///   ...,
///   COMPONENTS OF CommonResultsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LdapResultData {
    pub ldapMessages: OPTIONAL<Vec<LDAPMessage>>,
    pub returnToClient: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl LdapResultData {
    pub fn new(
        ldapMessages: OPTIONAL<Vec<LDAPMessage>>,
        returnToClient: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        LdapResultData {
            ldapMessages,
            returnToClient,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_returnToClient() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl Default for LdapResultData {
    fn default() -> Self {
        LdapResultData {
            ldapMessages: None,
            returnToClient: None,
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for LdapResultData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LdapResultData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LdapResultData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LdapResultData(el)
    }
}

pub const _rctl1_components_for_LdapResultData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "ldapMessages",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "returnToClient",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_LdapResultData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_LdapResultData: &[ComponentSpec; 0] = &[];

pub fn _decode_LdapResultData(el: &X690Element) -> ASN1Result<LdapResultData> {
    |el_: &X690Element| -> ASN1Result<LdapResultData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_LdapResultData,
            _eal_components_for_LdapResultData,
            _rctl2_components_for_LdapResultData,
        )?;
        let ldapMessages: OPTIONAL<Vec<LDAPMessage>> = match _components.get("ldapMessages") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<LDAPMessage>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<LDAPMessage> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_LDAPMessage(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        let returnToClient: OPTIONAL<BOOLEAN> = match _components.get("returnToClient") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
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
        Ok(LdapResultData {
            ldapMessages,
            returnToClient,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_LdapResultData(value_: &LdapResultData) -> ASN1Result<X690Element> {
    |value_: &LdapResultData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.ldapMessages {
            components_.push(
                |value_: &SEQUENCE_OF<LDAPMessage>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_LDAPMessage(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.returnToClient {
            if *v_ != LdapResultData::_default_value_for_returnToClient() {
                components_.push(ber_encode_boolean(&v_)?);
            }
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
            if *v_ != LdapResultData::_default_value_for_aliasDereferenced() {
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
/// linkedLDAP OPERATION ::= {
///   ARGUMENT    LinkedArgument
///   RESULT      LinkedResult
///   CODE        id-opcode-linkedLDAP }
/// ```
///
///
pub fn linkedLDAP() -> OPERATION {
    OPERATION {
        operationCode: Some(id_opcode_linkedLDAP), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LinkedArgument  ::=  OPTIONALLY-PROTECTED-SEQ { LinkedArgumentData }
/// ```
pub type LinkedArgument = OPTIONALLY_PROTECTED_SEQ<LinkedArgumentData>; // DefinedType

pub fn _decode_LinkedArgument(el: &X690Element) -> ASN1Result<LinkedArgument> {
    _decode_OPTIONALLY_PROTECTED_SEQ::<LinkedArgumentData>(_decode_LinkedArgumentData, &el)
}

pub fn _encode_LinkedArgument(value_: &LinkedArgument) -> ASN1Result<X690Element> {
    _encode_OPTIONALLY_PROTECTED_SEQ::<LinkedArgumentData>(_encode_LinkedArgumentData, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LinkedArgumentData ::= SEQUENCE {
///   object         DistinguishedName,
///   ldapMessage    LDAPMessage,
///   linkId         LinkId,
///   returnToClient BOOLEAN DEFAULT FALSE,
///   ...,
///   ...,
///   COMPONENTS OF  CommonArgumentsSeq }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LinkedArgumentData {
    pub object: DistinguishedName,
    pub ldapMessage: LDAPMessage,
    pub linkId: LinkId,
    pub returnToClient: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
    pub serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
    pub operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
    pub aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
    pub criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
    pub referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
    pub entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
    pub exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
    pub operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
}
impl LinkedArgumentData {
    pub fn new(
        object: DistinguishedName,
        ldapMessage: LDAPMessage,
        linkId: LinkId,
        returnToClient: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
        serviceControls: OPTIONAL<ServiceControls>, /* REPLICATED_COMPONENT */
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        requestor: OPTIONAL<DistinguishedName>,     /* REPLICATED_COMPONENT */
        operationProgress: OPTIONAL<OperationProgress>, /* REPLICATED_COMPONENT */
        aliasedRDNs: OPTIONAL<INTEGER>,             /* REPLICATED_COMPONENT */
        criticalExtensions: OPTIONAL<BIT_STRING>,   /* REPLICATED_COMPONENT */
        referenceType: OPTIONAL<ReferenceType>,     /* REPLICATED_COMPONENT */
        entryOnly: OPTIONAL<BOOLEAN>,               /* REPLICATED_COMPONENT */
        exclusions: OPTIONAL<Exclusions>,           /* REPLICATED_COMPONENT */
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,     /* REPLICATED_COMPONENT */
        operationContexts: OPTIONAL<ContextSelection>, /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,   /* REPLICATED_COMPONENT */
    ) -> Self {
        LinkedArgumentData {
            object,
            ldapMessage,
            linkId,
            returnToClient,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
            _unrecognized,
        }
    }
    pub fn _default_value_for_returnToClient() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_serviceControls() -> ServiceControls {
        ServiceControls {
            options: None,
            priority: None,
            timeLimit: None,
            sizeLimit: None,
            scopeOfReferral: None,
            attributeSizeLimit: None,
            manageDSAITPlaneRef: None,
            serviceType: None,
            userClass: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_familyGrouping() -> FamilyGrouping {
        FamilyGrouping_entryOnly
    }
}
impl TryFrom<X690Element> for LinkedArgumentData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LinkedArgumentData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LinkedArgumentData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LinkedArgumentData(el)
    }
}

pub const _rctl1_components_for_LinkedArgumentData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ldapMessage",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "linkId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "returnToClient",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_LinkedArgumentData: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestor",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 27)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 26)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticalExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 25)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
];

pub const _eal_components_for_LinkedArgumentData: &[ComponentSpec; 0] = &[];

pub fn _decode_LinkedArgumentData(el: &X690Element) -> ASN1Result<LinkedArgumentData> {
    |el_: &X690Element| -> ASN1Result<LinkedArgumentData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_LinkedArgumentData,
            _eal_components_for_LinkedArgumentData,
            _rctl2_components_for_LinkedArgumentData,
        )?;
        let object = _decode_DistinguishedName(_components.get("object").unwrap())?;
        let ldapMessage = _decode_LDAPMessage(_components.get("ldapMessage").unwrap())?;
        let linkId = _decode_LinkId(_components.get("linkId").unwrap())?;
        let returnToClient: OPTIONAL<BOOLEAN> = match _components.get("returnToClient") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let serviceControls: OPTIONAL<ServiceControls> = match _components.get("serviceControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControls> {
                Ok(_decode_ServiceControls(&el.inner()?)?)
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
        let requestor: OPTIONAL<DistinguishedName> = match _components.get("requestor") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress: OPTIONAL<OperationProgress> =
            match _components.get("operationProgress") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OperationProgress> {
                    Ok(_decode_OperationProgress(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criticalExtensions: OPTIONAL<BIT_STRING> = match _components.get("criticalExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationContexts: OPTIONAL<ContextSelection> =
            match _components.get("operationContexts") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextSelection> {
                    Ok(_decode_ContextSelection(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(LinkedArgumentData {
            object,
            ldapMessage,
            linkId,
            returnToClient,
            _unrecognized,
            serviceControls,
            securityParameters,
            requestor,
            operationProgress,
            aliasedRDNs,
            criticalExtensions,
            referenceType,
            entryOnly,
            exclusions,
            nameResolveOnMaster,
            operationContexts,
            familyGrouping,
        })
    }(&el)
}

pub fn _encode_LinkedArgumentData(value_: &LinkedArgumentData) -> ASN1Result<X690Element> {
    |value_: &LinkedArgumentData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(26);
        components_.push(_encode_DistinguishedName(&value_.object)?);
        components_.push(_encode_LDAPMessage(&value_.ldapMessage)?);
        components_.push(_encode_LinkId(&value_.linkId)?);
        if let Some(v_) = &value_.returnToClient {
            if *v_ != LinkedArgumentData::_default_value_for_returnToClient() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.serviceControls {
            components_.push(|v_1: &ServiceControls| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceControls(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    29,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.requestor {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    28,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.operationProgress {
            if *v_ != LinkedArgumentData::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        27,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    26,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.criticalExtensions {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    25,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.referenceType {
            components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    24,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != LinkedArgumentData::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        23,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    22,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != LinkedArgumentData::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationContexts {
            components_.push(|v_1: &ContextSelection| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ContextSelection(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            if *v_ != LinkedArgumentData::_default_value_for_familyGrouping() {
                components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        19,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
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
/// LinkedResult  ::=  NULL
/// ```
pub type LinkedResult = NULL; // NullType

pub fn _decode_LinkedResult(el: &X690Element) -> ASN1Result<LinkedResult> {
    ber_decode_null(&el)
}

pub fn _encode_LinkedResult(value_: &LinkedResult) -> ASN1Result<X690Element> {
    ber_encode_null(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// abandoned ERROR ::= {-- not literally an "error"
///   PARAMETER     OPTIONALLY-PROTECTED { AbandonedData }
///   CODE          id-errcode-abandoned }
/// ```
///
///
pub fn abandoned() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_abandoned), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonedData ::= SET {
///     problem       AbandonedProblem OPTIONAL,
///     ...,
///     ...,
///     COMPONENTS OF CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AbandonedData {
    pub problem: OPTIONAL<AbandonedProblem>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl AbandonedData {
    pub fn new(
        problem: OPTIONAL<AbandonedProblem>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        AbandonedData {
            problem,
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
impl Default for AbandonedData {
    fn default() -> Self {
        AbandonedData {
            problem: None,
            securityParameters: None,
            performer: None,
            aliasDereferenced: None,
            notification: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AbandonedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AbandonedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonedData(el)
    }
}

pub const _rctl1_components_for_AbandonedData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "problem",
    true,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_AbandonedData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_AbandonedData: &[ComponentSpec; 0] = &[];

pub fn _decode_AbandonedData(el: &X690Element) -> ASN1Result<AbandonedData> {
    |el_: &X690Element| -> ASN1Result<AbandonedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_AbandonedData,
            _eal_components_for_AbandonedData,
            _rctl2_components_for_AbandonedData,
            60,
        )?;
        let problem: OPTIONAL<AbandonedProblem> = match _components.get("problem") {
            Some(c_) => Some(_decode_AbandonedProblem(c_)?),
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
        Ok(AbandonedData {
            problem,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_AbandonedData(value_: &AbandonedData) -> ASN1Result<X690Element> {
    |value_: &AbandonedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.problem {
            components_.push(_encode_AbandonedProblem(&v_)?);
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
            if *v_ != AbandonedData::_default_value_for_aliasDereferenced() {
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
/// AbandonedProblem   ::=  ENUMERATED {
///   pagingAbandoned (0) }
/// ```
pub type AbandonedProblem = ENUMERATED;

pub const AbandonedProblem_pagingAbandoned: AbandonedProblem = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AbandonedProblem(el: &X690Element) -> ASN1Result<AbandonedProblem> {
    ber_decode_enumerated(&el)
}

pub fn _encode_AbandonedProblem(value_: &AbandonedProblem) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// abandonFailed ERROR ::= {
///   PARAMETER OPTIONALLY-PROTECTED { AbandonFailedData }
///   CODE      id-errcode-abandonFailed }
/// ```
///
///
pub fn abandonFailed() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_abandonFailed), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonFailedData ::= SET {
///   problem    [0]  AbandonProblem,
///   operation  [1]  InvokeId,
///   ...,
///   ...,
///   COMPONENTS OF   CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AbandonFailedData {
    pub problem: AbandonProblem,
    pub operation: InvokeId,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl AbandonFailedData {
    pub fn new(
        problem: AbandonProblem,
        operation: InvokeId,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        AbandonFailedData {
            problem,
            operation,
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
impl TryFrom<X690Element> for AbandonFailedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonFailedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AbandonFailedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AbandonFailedData(el)
    }
}

pub const _rctl1_components_for_AbandonFailedData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operation",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AbandonFailedData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_AbandonFailedData: &[ComponentSpec; 0] = &[];

pub fn _decode_AbandonFailedData(el: &X690Element) -> ASN1Result<AbandonFailedData> {
    |el_: &X690Element| -> ASN1Result<AbandonFailedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_AbandonFailedData,
            _eal_components_for_AbandonFailedData,
            _rctl2_components_for_AbandonFailedData,
            70,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<AbandonProblem> {
            Ok(_decode_AbandonProblem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
        let operation =
            |el: &X690Element| -> ASN1Result<InvokeId> { Ok(_decode_InvokeId(&el.inner()?)?) }(
                _components.get("operation").unwrap(),
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
        Ok(AbandonFailedData {
            problem,
            operation,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_AbandonFailedData(value_: &AbandonFailedData) -> ASN1Result<X690Element> {
    |value_: &AbandonFailedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &AbandonProblem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AbandonProblem(
                    &v_1,
                )?))),
            ))
        }(&value_.problem)?);
        components_.push(|v_1: &InvokeId| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_InvokeId(&v_1)?))),
            ))
        }(&value_.operation)?);
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
            if *v_ != AbandonFailedData::_default_value_for_aliasDereferenced() {
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
/// AbandonProblem  ::=  INTEGER {
///   noSuchOperation (1),
///   tooLate         (2),
///   cannotAbandon   (3) }
/// ```
pub type AbandonProblem = INTEGER;

pub const AbandonProblem_noSuchOperation: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const AbandonProblem_tooLate: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const AbandonProblem_cannotAbandon: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AbandonProblem(el: &X690Element) -> ASN1Result<AbandonProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_AbandonProblem(value_: &AbandonProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeError ERROR ::= {
///   PARAMETER     OPTIONALLY-PROTECTED { AttributeErrorData }
///   CODE          id-errcode-attributeError }
/// ```
///
///
pub fn attributeError() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_attributeError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeErrorData ::= SET {
///   object   [0]  Name,
///   problems [1]  SET OF SEQUENCE {
///     problem  [0]  AttributeProblem,
///     type     [1]  AttributeType,
///     value    [2]  AttributeValue OPTIONAL,
///     ...},
///   ...,
///   ...,
///   COMPONENTS OF CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeErrorData {
    pub object: Name,
    pub problems: Vec<AttributeErrorData_problems_Item>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl AttributeErrorData {
    pub fn new(
        object: Name,
        problems: Vec<AttributeErrorData_problems_Item>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        AttributeErrorData {
            object,
            problems,
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
impl TryFrom<X690Element> for AttributeErrorData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeErrorData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeErrorData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeErrorData(el)
    }
}

pub const _rctl1_components_for_AttributeErrorData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "problems",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeErrorData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_AttributeErrorData: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeErrorData(el: &X690Element) -> ASN1Result<AttributeErrorData> {
    |el_: &X690Element| -> ASN1Result<AttributeErrorData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeErrorData,
            _eal_components_for_AttributeErrorData,
            _rctl2_components_for_AttributeErrorData,
            70,
        )?;
        let object = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("object").unwrap(),
        )?;
        let problems = |el: &X690Element| -> ASN1Result<Vec<AttributeErrorData_problems_Item>> {
            Ok(|el: &X690Element| -> ASN1Result<
                SET_OF<AttributeErrorData_problems_Item>,
            > {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<AttributeErrorData_problems_Item> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_AttributeErrorData_problems_Item(el)?);
                }
                Ok(items)
            }(&el.inner()?)?)
        }(_components.get("problems").unwrap())?;
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
        Ok(AttributeErrorData {
            object,
            problems,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_AttributeErrorData(value_: &AttributeErrorData) -> ASN1Result<X690Element> {
    |value_: &AttributeErrorData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.object)?);
        components_.push(
            |v_1: &Vec<AttributeErrorData_problems_Item>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        AttributeErrorData_problems_Item,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeErrorData_problems_Item(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&value_.problems)?,
        );
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
            if *v_ != AttributeErrorData::_default_value_for_aliasDereferenced() {
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
/// AttributeProblem  ::=  INTEGER {
///   noSuchAttributeOrValue        (1),
///   invalidAttributeSyntax        (2),
///   undefinedAttributeType        (3),
///   inappropriateMatching         (4),
///   constraintViolation           (5),
///   attributeOrValueAlreadyExists (6),
///   contextViolation              (7) }
/// ```
pub type AttributeProblem = INTEGER;

pub const AttributeProblem_noSuchAttributeOrValue: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const AttributeProblem_invalidAttributeSyntax: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const AttributeProblem_undefinedAttributeType: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const AttributeProblem_inappropriateMatching: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const AttributeProblem_constraintViolation: i32 = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const AttributeProblem_attributeOrValueAlreadyExists: i32 = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const AttributeProblem_contextViolation: i32 = 7; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AttributeProblem(el: &X690Element) -> ASN1Result<AttributeProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_AttributeProblem(value_: &AttributeProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameError ERROR ::= {
///   PARAMETER     OPTIONALLY-PROTECTED { NameErrorData }
///   CODE          id-errcode-nameError }
/// ```
///
///
pub fn nameError() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_nameError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NameErrorData ::= SET {
///   problem  [0]  NameProblem,
///   matched  [1]  Name,
///   ...,
///   ...,
///   COMPONENTS OF CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NameErrorData {
    pub problem: NameProblem,
    pub matched: Name,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl NameErrorData {
    pub fn new(
        problem: NameProblem,
        matched: Name,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        NameErrorData {
            problem,
            matched,
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
impl TryFrom<X690Element> for NameErrorData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NameErrorData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NameErrorData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NameErrorData(el)
    }
}

pub const _rctl1_components_for_NameErrorData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matched",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NameErrorData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_NameErrorData: &[ComponentSpec; 0] = &[];

pub fn _decode_NameErrorData(el: &X690Element) -> ASN1Result<NameErrorData> {
    |el_: &X690Element| -> ASN1Result<NameErrorData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_NameErrorData,
            _eal_components_for_NameErrorData,
            _rctl2_components_for_NameErrorData,
            70,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<NameProblem> {
            Ok(_decode_NameProblem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
        let matched = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("matched").unwrap(),
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
        Ok(NameErrorData {
            problem,
            matched,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_NameErrorData(value_: &NameErrorData) -> ASN1Result<X690Element> {
    |value_: &NameErrorData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &NameProblem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NameProblem(&v_1)?))),
            ))
        }(&value_.problem)?);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.matched)?);
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
            if *v_ != NameErrorData::_default_value_for_aliasDereferenced() {
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
/// NameProblem  ::=  INTEGER {
///   noSuchObject              (1),
///   aliasProblem              (2),
///   invalidAttributeSyntax    (3),
///   aliasDereferencingProblem (4)
///   -- not to be used         (5)-- }
/// ```
pub type NameProblem = INTEGER;

pub const NameProblem_noSuchObject: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const NameProblem_aliasProblem: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const NameProblem_invalidAttributeSyntax: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const NameProblem_aliasDereferencingProblem: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_NameProblem(el: &X690Element) -> ASN1Result<NameProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_NameProblem(value_: &NameProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// referral ERROR ::= { -- not literally an "error"
///   PARAMETER      OPTIONALLY-PROTECTED { ReferralData }
///   CODE           id-errcode-referral }
/// ```
///
///
pub fn referral() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_referral), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReferralData ::= SET {
///   candidate  [0] ContinuationReference,
///   ...,
///   ...,
///   COMPONENTS OF  CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ReferralData {
    pub candidate: ContinuationReference,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ReferralData {
    pub fn new(
        candidate: ContinuationReference,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ReferralData {
            candidate,
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
impl TryFrom<X690Element> for ReferralData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReferralData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReferralData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReferralData(el)
    }
}

pub const _rctl1_components_for_ReferralData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "candidate",
    false,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_ReferralData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ReferralData: &[ComponentSpec; 0] = &[];

pub fn _decode_ReferralData(el: &X690Element) -> ASN1Result<ReferralData> {
    |el_: &X690Element| -> ASN1Result<ReferralData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ReferralData,
            _eal_components_for_ReferralData,
            _rctl2_components_for_ReferralData,
            60,
        )?;
        let candidate = |el: &X690Element| -> ASN1Result<ContinuationReference> {
            Ok(_decode_ContinuationReference(&el.inner()?)?)
        }(_components.get("candidate").unwrap())?;
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
        Ok(ReferralData {
            candidate,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ReferralData(value_: &ReferralData) -> ASN1Result<X690Element> {
    |value_: &ReferralData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(|v_1: &ContinuationReference| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_ContinuationReference(&v_1)?,
                ))),
            ))
        }(&value_.candidate)?);
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
            if *v_ != ReferralData::_default_value_for_aliasDereferenced() {
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
/// securityError  ERROR ::= {
///   PARAMETER   OPTIONALLY-PROTECTED { SecurityErrorData }
///   CODE        id-errcode-securityError }
/// ```
///
///
pub fn securityError() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_securityError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecurityErrorData ::= SET {
///   problem      [0]  SecurityProblem,
///   spkmInfo     [1]  SPKM-ERROR OPTIONAL,
///   encPwdInfo   [2]  EncPwdInfo OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SecurityErrorData {
    pub problem: SecurityProblem,
    pub spkmInfo: OPTIONAL<SPKM_ERROR>,
    pub encPwdInfo: OPTIONAL<EncPwdInfo>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl SecurityErrorData {
    pub fn new(
        problem: SecurityProblem,
        spkmInfo: OPTIONAL<SPKM_ERROR>,
        encPwdInfo: OPTIONAL<EncPwdInfo>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        SecurityErrorData {
            problem,
            spkmInfo,
            encPwdInfo,
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
impl TryFrom<X690Element> for SecurityErrorData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityErrorData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SecurityErrorData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityErrorData(el)
    }
}

pub const _rctl1_components_for_SecurityErrorData: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "spkmInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encPwdInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SecurityErrorData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_SecurityErrorData: &[ComponentSpec; 0] = &[];

pub fn _decode_SecurityErrorData(el: &X690Element) -> ASN1Result<SecurityErrorData> {
    |el_: &X690Element| -> ASN1Result<SecurityErrorData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SecurityErrorData,
            _eal_components_for_SecurityErrorData,
            _rctl2_components_for_SecurityErrorData,
            80,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<SecurityProblem> {
            Ok(_decode_SecurityProblem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
        let spkmInfo: OPTIONAL<SPKM_ERROR> = match _components.get("spkmInfo") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SPKM_ERROR> {
                Ok(_decode_SPKM_ERROR(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let encPwdInfo: OPTIONAL<EncPwdInfo> = match _components.get("encPwdInfo") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<EncPwdInfo> {
                Ok(_decode_EncPwdInfo(&el.inner()?)?)
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
        Ok(SecurityErrorData {
            problem,
            spkmInfo,
            encPwdInfo,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_SecurityErrorData(value_: &SecurityErrorData) -> ASN1Result<X690Element> {
    |value_: &SecurityErrorData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(|v_1: &SecurityProblem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_SecurityProblem(
                    &v_1,
                )?))),
            ))
        }(&value_.problem)?);
        if let Some(v_) = &value_.spkmInfo {
            components_.push(|v_1: &SPKM_ERROR| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_SPKM_ERROR(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.encPwdInfo {
            components_.push(|v_1: &EncPwdInfo| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_EncPwdInfo(&v_1)?))),
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
            if *v_ != SecurityErrorData::_default_value_for_aliasDereferenced() {
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
/// SecurityProblem  ::=  INTEGER {
///   inappropriateAuthentication     (1),
///   invalidCredentials              (2),
///   insufficientAccessRights        (3),
///   invalidSignature                (4),
///   protectionRequired              (5),
///   noInformation                   (6),
///   blockedCredentials              (7),
///   -- invalidQOPMatch              (8), obsolete
///   spkmError                       (9),
///   unsupportedAuthenticationMethod (10),
///   passwordExpired                 (11),
///   inappropriateAlgorithms         (12) }
/// ```
pub type SecurityProblem = INTEGER;

pub const SecurityProblem_inappropriateAuthentication: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_invalidCredentials: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_insufficientAccessRights: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_invalidSignature: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_protectionRequired: i32 = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_noInformation: i32 = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_blockedCredentials: i32 = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_spkmError: i32 = 9; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_unsupportedAuthenticationMethod: i32 = 10; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_passwordExpired: i32 = 11; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityProblem_inappropriateAlgorithms: i32 = 12; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_SecurityProblem(el: &X690Element) -> ASN1Result<SecurityProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_SecurityProblem(value_: &SecurityProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncPwdInfo ::= SEQUENCE {
///   algorithms     [0]  SEQUENCE OF AlgorithmIdentifier
///                         {{SupportedAlgorithms}} OPTIONAL,
///   pwdQualityRule [1]  SEQUENCE OF AttributeTypeAndValue OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncPwdInfo {
    pub algorithms: OPTIONAL<Vec<AlgorithmIdentifier>>,
    pub pwdQualityRule: OPTIONAL<Vec<AttributeTypeAndValue>>,
    pub _unrecognized: Vec<X690Element>,
}
impl EncPwdInfo {
    pub fn new(
        algorithms: OPTIONAL<Vec<AlgorithmIdentifier>>,
        pwdQualityRule: OPTIONAL<Vec<AttributeTypeAndValue>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EncPwdInfo {
            algorithms,
            pwdQualityRule,
            _unrecognized,
        }
    }
}
impl Default for EncPwdInfo {
    fn default() -> Self {
        EncPwdInfo {
            algorithms: None,
            pwdQualityRule: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for EncPwdInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncPwdInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncPwdInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EncPwdInfo(el)
    }
}

pub const _rctl1_components_for_EncPwdInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithms",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pwdQualityRule",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncPwdInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncPwdInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EncPwdInfo(el: &X690Element) -> ASN1Result<EncPwdInfo> {
    |el_: &X690Element| -> ASN1Result<EncPwdInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncPwdInfo,
            _eal_components_for_EncPwdInfo,
            _rctl2_components_for_EncPwdInfo,
        )?;
        let algorithms: OPTIONAL<Vec<AlgorithmIdentifier>> = match _components.get("algorithms") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<AlgorithmIdentifier>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<AlgorithmIdentifier> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_AlgorithmIdentifier(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let pwdQualityRule: OPTIONAL<Vec<AttributeTypeAndValue>> = match _components
            .get("pwdQualityRule")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<AttributeTypeAndValue>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeTypeAndValue>> {
                            let elements = match el.value.borrow() {
                                X690Encoding::Constructed(children) => children,
                                _ => {
                                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction))
                                }
                            };
                            let mut items: SEQUENCE_OF<AttributeTypeAndValue> =
                                Vec::with_capacity(elements.len());
                            for el in elements {
                                items.push(_decode_AttributeTypeAndValue(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(c_)?,
            ),
            _ => None,
        };
        Ok(EncPwdInfo {
            algorithms,
            pwdQualityRule,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EncPwdInfo(value_: &EncPwdInfo) -> ASN1Result<X690Element> {
    |value_: &EncPwdInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.algorithms {
            components_.push(|v_1: &Vec<AlgorithmIdentifier>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 0, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AlgorithmIdentifier(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.pwdQualityRule {
            components_.push(|v_1: &Vec<AttributeTypeAndValue>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 1, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<AttributeTypeAndValue>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AttributeTypeAndValue(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
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
/// serviceError ERROR ::= {
///   PARAMETER   OPTIONALLY-PROTECTED { ServiceErrorData }
///   CODE        id-errcode-serviceError }
/// ```
///
///
pub fn serviceError() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_serviceError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ServiceErrorData ::= SET {
///   problem   [0]  ServiceProblem,
///   ...,
///   ...,
///   COMPONENTS OF  CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ServiceErrorData {
    pub problem: ServiceProblem,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ServiceErrorData {
    pub fn new(
        problem: ServiceProblem,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ServiceErrorData {
            problem,
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
impl TryFrom<X690Element> for ServiceErrorData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ServiceErrorData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ServiceErrorData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ServiceErrorData(el)
    }
}

pub const _rctl1_components_for_ServiceErrorData: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "problem",
    false,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_ServiceErrorData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ServiceErrorData: &[ComponentSpec; 0] = &[];

pub fn _decode_ServiceErrorData(el: &X690Element) -> ASN1Result<ServiceErrorData> {
    |el_: &X690Element| -> ASN1Result<ServiceErrorData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ServiceErrorData,
            _eal_components_for_ServiceErrorData,
            _rctl2_components_for_ServiceErrorData,
            60,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<ServiceProblem> {
            Ok(_decode_ServiceProblem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
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
        Ok(ServiceErrorData {
            problem,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ServiceErrorData(value_: &ServiceErrorData) -> ASN1Result<X690Element> {
    |value_: &ServiceErrorData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(|v_1: &ServiceProblem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ServiceProblem(
                    &v_1,
                )?))),
            ))
        }(&value_.problem)?);
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
            if *v_ != ServiceErrorData::_default_value_for_aliasDereferenced() {
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
/// ServiceProblem  ::=  INTEGER {
///   busy                         (1),
///   unavailable                  (2),
///   unwillingToPerform           (3),
///   chainingRequired             (4),
///   unableToProceed              (5),
///   invalidReference             (6),
///   timeLimitExceeded            (7),
///   administrativeLimitExceeded  (8),
///   loopDetected                 (9),
///   unavailableCriticalExtension (10),
///   outOfScope                   (11),
///   ditError                     (12),
///   invalidQueryReference        (13),
///   requestedServiceNotAvailable (14),
///   unsupportedMatchingUse       (15),
///   ambiguousKeyAttributes       (16),
///   saslBindInProgress           (17),
///   notSupportedByLDAP           (18) }
/// ```
pub type ServiceProblem = INTEGER;

pub const ServiceProblem_busy: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_unavailable: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_unwillingToPerform: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_chainingRequired: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_unableToProceed: i32 = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_invalidReference: i32 = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_timeLimitExceeded: i32 = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_administrativeLimitExceeded: i32 = 8; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_loopDetected: i32 = 9; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_unavailableCriticalExtension: i32 = 10; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_outOfScope: i32 = 11; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_ditError: i32 = 12; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_invalidQueryReference: i32 = 13; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_requestedServiceNotAvailable: i32 = 14; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_unsupportedMatchingUse: i32 = 15; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_ambiguousKeyAttributes: i32 = 16; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_saslBindInProgress: i32 = 17; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceProblem_notSupportedByLDAP: i32 = 18; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ServiceProblem(el: &X690Element) -> ASN1Result<ServiceProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_ServiceProblem(value_: &ServiceProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// updateError ERROR ::= {
///   PARAMETER   OPTIONALLY-PROTECTED { UpdateErrorData }
///   CODE        id-errcode-updateError }
/// ```
///
///
pub fn updateError() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_updateError), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UpdateErrorData ::= SET {
///   problem        [0]  UpdateProblem,
///   attributeInfo  [1]  SET SIZE (1..MAX) OF CHOICE {
///     attributeType       AttributeType,
///     attribute           Attribute{{SupportedAttributes}},
///     ... } OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF       CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UpdateErrorData {
    pub problem: UpdateProblem,
    pub attributeInfo: OPTIONAL<Vec<UpdateErrorData_attributeInfo_Item>>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl UpdateErrorData {
    pub fn new(
        problem: UpdateProblem,
        attributeInfo: OPTIONAL<Vec<UpdateErrorData_attributeInfo_Item>>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        UpdateErrorData {
            problem,
            attributeInfo,
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
impl TryFrom<X690Element> for UpdateErrorData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateErrorData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateErrorData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateErrorData(el)
    }
}

pub const _rctl1_components_for_UpdateErrorData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UpdateErrorData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_UpdateErrorData: &[ComponentSpec; 0] = &[];

pub fn _decode_UpdateErrorData(el: &X690Element) -> ASN1Result<UpdateErrorData> {
    |el_: &X690Element| -> ASN1Result<UpdateErrorData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_UpdateErrorData,
            _eal_components_for_UpdateErrorData,
            _rctl2_components_for_UpdateErrorData,
            70,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<UpdateProblem> {
            Ok(_decode_UpdateProblem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
        let attributeInfo: OPTIONAL<Vec<UpdateErrorData_attributeInfo_Item>> = match _components
            .get("attributeInfo")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<UpdateErrorData_attributeInfo_Item>> {
                    Ok(|el: &X690Element| -> ASN1Result<
                        SET_OF<UpdateErrorData_attributeInfo_Item>,
                    > {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<UpdateErrorData_attributeInfo_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_UpdateErrorData_attributeInfo_Item(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(c_)?,
            ),
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
        Ok(UpdateErrorData {
            problem,
            attributeInfo,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_UpdateErrorData(value_: &UpdateErrorData) -> ASN1Result<X690Element> {
    |value_: &UpdateErrorData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &UpdateProblem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UpdateProblem(
                    &v_1,
                )?))),
            ))
        }(&value_.problem)?);
        if let Some(v_) = &value_.attributeInfo {
            components_.push(|v_1: &Vec<UpdateErrorData_attributeInfo_Item>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 1, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<UpdateErrorData_attributeInfo_Item>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_UpdateErrorData_attributeInfo_Item(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
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
            if *v_ != UpdateErrorData::_default_value_for_aliasDereferenced() {
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
/// UpdateProblem  ::=  INTEGER {
///   namingViolation                   (1),
///   objectClassViolation              (2),
///   notAllowedOnNonLeaf               (3),
///   notAllowedOnRDN                   (4),
///   entryAlreadyExists                (5),
///   affectsMultipleDSAs               (6),
///   objectClassModificationProhibited (7),
///   noSuchSuperior                    (8),
///   notAncestor                       (9),
///   parentNotAncestor                 (10),
///   hierarchyRuleViolation            (11),
///   familyRuleViolation               (12),
///   insufficientPasswordQuality       (13),
///   passwordInHistory                 (14),
///   noPasswordSlot                    (15) }
/// ```
pub type UpdateProblem = INTEGER;

pub const UpdateProblem_namingViolation: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_objectClassViolation: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_notAllowedOnNonLeaf: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_notAllowedOnRDN: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_entryAlreadyExists: i32 = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_affectsMultipleDSAs: i32 = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_objectClassModificationProhibited: i32 = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_noSuchSuperior: i32 = 8; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_notAncestor: i32 = 9; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_parentNotAncestor: i32 = 10; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_hierarchyRuleViolation: i32 = 11; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_familyRuleViolation: i32 = 12; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_insufficientPasswordQuality: i32 = 13; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_passwordInHistory: i32 = 14; /* LONG_NAMED_INTEGER_VALUE */

pub const UpdateProblem_noPasswordSlot: i32 = 15; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_UpdateProblem(el: &X690Element) -> ASN1Result<UpdateProblem> {
    ber_decode_integer(&el)
}

pub fn _encode_UpdateProblem(value_: &UpdateProblem) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-family-information OBJECT IDENTIFIER ::= {id-at 64}
/// ```
///
///
pub fn id_at_family_information() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([64])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ServiceControls-priority ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ServiceControls_priority = INTEGER;

pub const ServiceControls_priority_low: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceControls_priority_medium: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceControls_priority_high: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ServiceControls_priority(el: &X690Element) -> ASN1Result<ServiceControls_priority> {
    ber_decode_integer(&el)
}

pub fn _encode_ServiceControls_priority(
    value_: &ServiceControls_priority,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ServiceControls-scopeOfReferral ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ServiceControls_scopeOfReferral = INTEGER;

pub const ServiceControls_scopeOfReferral_dmd: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ServiceControls_scopeOfReferral_country: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ServiceControls_scopeOfReferral(
    el: &X690Element,
) -> ASN1Result<ServiceControls_scopeOfReferral> {
    ber_decode_integer(&el)
}

pub fn _encode_ServiceControls_scopeOfReferral(
    value_: &ServiceControls_scopeOfReferral,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ServiceControls-manageDSAITPlaneRef ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ServiceControls_manageDSAITPlaneRef {
    pub dsaName: Name,
    pub agreementID: AgreementID,
    pub _unrecognized: Vec<X690Element>,
}
impl ServiceControls_manageDSAITPlaneRef {
    pub fn new(dsaName: Name, agreementID: AgreementID, _unrecognized: Vec<X690Element>) -> Self {
        ServiceControls_manageDSAITPlaneRef {
            dsaName,
            agreementID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ServiceControls_manageDSAITPlaneRef {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ServiceControls_manageDSAITPlaneRef(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ServiceControls_manageDSAITPlaneRef {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ServiceControls_manageDSAITPlaneRef(el)
    }
}

pub const _rctl1_components_for_ServiceControls_manageDSAITPlaneRef: &[ComponentSpec; 2] = &[
    ComponentSpec::new("dsaName", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "agreementID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ServiceControls_manageDSAITPlaneRef: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ServiceControls_manageDSAITPlaneRef: &[ComponentSpec; 0] = &[];

pub fn _decode_ServiceControls_manageDSAITPlaneRef(
    el: &X690Element,
) -> ASN1Result<ServiceControls_manageDSAITPlaneRef> {
    |el_: &X690Element| -> ASN1Result<ServiceControls_manageDSAITPlaneRef> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ServiceControls_manageDSAITPlaneRef,
            _eal_components_for_ServiceControls_manageDSAITPlaneRef,
            _rctl2_components_for_ServiceControls_manageDSAITPlaneRef,
        )?;
        let dsaName = _decode_Name(_components.get("dsaName").unwrap())?;
        let agreementID = _decode_AgreementID(_components.get("agreementID").unwrap())?;
        Ok(ServiceControls_manageDSAITPlaneRef {
            dsaName,
            agreementID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ServiceControls_manageDSAITPlaneRef(
    value_: &ServiceControls_manageDSAITPlaneRef,
) -> ASN1Result<X690Element> {
    |value_: &ServiceControls_manageDSAITPlaneRef| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.dsaName)?);
        components_.push(_encode_AgreementID(&value_.agreementID)?);
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
/// EntryInformationSelection-attributes ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryInformationSelection_attributes {
    allUserAttributes(NULL),
    select(SET_OF<AttributeType>),
}

impl TryFrom<X690Element> for EntryInformationSelection_attributes {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformationSelection_attributes(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryInformationSelection_attributes {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformationSelection_attributes(el)
    }
}

pub fn _decode_EntryInformationSelection_attributes(
    el: &X690Element,
) -> ASN1Result<EntryInformationSelection_attributes> {
    |el: &X690Element| -> ASN1Result<EntryInformationSelection_attributes> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(EntryInformationSelection_attributes::allUserAttributes(
                ber_decode_null(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(EntryInformationSelection_attributes::select(
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
                }(&el.inner()?)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_EntryInformationSelection_attributes(
    value_: &EntryInformationSelection_attributes,
) -> ASN1Result<X690Element> {
    |value: &EntryInformationSelection_attributes| -> ASN1Result<X690Element> {
        match value {
            EntryInformationSelection_attributes::allUserAttributes(v) => {
                |v_1: &NULL| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_null(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EntryInformationSelection_attributes::select(v) => {
                |v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                    let el_1 = |value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
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
/// EntryInformationSelection-infoTypes ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type EntryInformationSelection_infoTypes = INTEGER;

pub const EntryInformationSelection_infoTypes_attributeTypesOnly: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const EntryInformationSelection_infoTypes_attributeTypesAndValues: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_EntryInformationSelection_infoTypes(
    el: &X690Element,
) -> ASN1Result<EntryInformationSelection_infoTypes> {
    ber_decode_integer(&el)
}

pub fn _encode_EntryInformationSelection_infoTypes(
    value_: &EntryInformationSelection_infoTypes,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EntryInformationSelection-extraAttributes ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum EntryInformationSelection_extraAttributes {
    allOperationalAttributes(NULL),
    select(Vec<AttributeType>),
}

impl TryFrom<X690Element> for EntryInformationSelection_extraAttributes {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformationSelection_extraAttributes(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryInformationSelection_extraAttributes {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformationSelection_extraAttributes(el)
    }
}

pub fn _decode_EntryInformationSelection_extraAttributes(
    el: &X690Element,
) -> ASN1Result<EntryInformationSelection_extraAttributes> {
    |el: &X690Element| -> ASN1Result<EntryInformationSelection_extraAttributes> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 3) => Ok(
                EntryInformationSelection_extraAttributes::allOperationalAttributes(
                    ber_decode_null(&el.inner()?)?,
                ),
            ),
            (TagClass::CONTEXT, 4) => Ok(EntryInformationSelection_extraAttributes::select(
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
                }(&el.inner()?)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_EntryInformationSelection_extraAttributes(
    value_: &EntryInformationSelection_extraAttributes,
) -> ASN1Result<X690Element> {
    |value: &EntryInformationSelection_extraAttributes| -> ASN1Result<X690Element> {
        match value {
            EntryInformationSelection_extraAttributes::allOperationalAttributes(v) => {
                |v_1: &NULL| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_null(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            EntryInformationSelection_extraAttributes::select(v) => {
                |v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                    let el_1 = |value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_1)?;
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
/// TypeAndContextAssertion-contextAssertions ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TypeAndContextAssertion_contextAssertions {
    preference(Vec<ContextAssertion>),
    all(Vec<ContextAssertion>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TypeAndContextAssertion_contextAssertions {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TypeAndContextAssertion_contextAssertions(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TypeAndContextAssertion_contextAssertions {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TypeAndContextAssertion_contextAssertions(el)
    }
}

pub fn _decode_TypeAndContextAssertion_contextAssertions(
    el: &X690Element,
) -> ASN1Result<TypeAndContextAssertion_contextAssertions> {
    |el: &X690Element| -> ASN1Result<TypeAndContextAssertion_contextAssertions> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(TypeAndContextAssertion_contextAssertions::preference(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextAssertion>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<ContextAssertion> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_ContextAssertion(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::UNIVERSAL, 17) => Ok(TypeAndContextAssertion_contextAssertions::all(
                |el: &X690Element| -> ASN1Result<SET_OF<ContextAssertion>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<ContextAssertion> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_ContextAssertion(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            _ => Ok(TypeAndContextAssertion_contextAssertions::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_TypeAndContextAssertion_contextAssertions(
    value_: &TypeAndContextAssertion_contextAssertions,
) -> ASN1Result<X690Element> {
    |value: &TypeAndContextAssertion_contextAssertions| -> ASN1Result<X690Element> {
        match value {
            TypeAndContextAssertion_contextAssertions::preference(v) => {
                |value_: &SEQUENCE_OF<ContextAssertion>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_ContextAssertion(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v)
            }
            TypeAndContextAssertion_contextAssertions::all(v) => {
                |value_: &SET_OF<ContextAssertion>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_ContextAssertion(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v)
            }
            TypeAndContextAssertion_contextAssertions::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FamilyReturn-memberSelect ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type FamilyReturn_memberSelect = ENUMERATED;

pub const FamilyReturn_memberSelect_contributingEntriesOnly: FamilyReturn_memberSelect = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const FamilyReturn_memberSelect_participatingEntriesOnly: FamilyReturn_memberSelect = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const FamilyReturn_memberSelect_compoundEntry: FamilyReturn_memberSelect = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_FamilyReturn_memberSelect(
    el: &X690Element,
) -> ASN1Result<FamilyReturn_memberSelect> {
    ber_decode_enumerated(&el)
}

pub fn _encode_FamilyReturn_memberSelect(
    value_: &FamilyReturn_memberSelect,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EntryInformation-information-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum EntryInformation_information_Item {
    attributeType(AttributeType),
    attribute(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for EntryInformation_information_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformation_information_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryInformation_information_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryInformation_information_Item(el)
    }
}

pub fn _decode_EntryInformation_information_Item(
    el: &X690Element,
) -> ASN1Result<EntryInformation_information_Item> {
    |el: &X690Element| -> ASN1Result<EntryInformation_information_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 6) => Ok(EntryInformation_information_Item::attributeType(
                _decode_AttributeType(&el)?,
            )),
            (TagClass::UNIVERSAL, 16) => Ok(EntryInformation_information_Item::attribute(
                _decode_Attribute(&el)?,
            )),
            _ => Ok(EntryInformation_information_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_EntryInformation_information_Item(
    value_: &EntryInformation_information_Item,
) -> ASN1Result<X690Element> {
    |value: &EntryInformation_information_Item| -> ASN1Result<X690Element> {
        match value {
            EntryInformation_information_Item::attributeType(v) => _encode_AttributeType(&v),
            EntryInformation_information_Item::attribute(v) => _encode_Attribute(&v),
            EntryInformation_information_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FamilyEntry-information-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum FamilyEntry_information_Item {
    attributeType(AttributeType),
    attribute(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for FamilyEntry_information_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyEntry_information_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FamilyEntry_information_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FamilyEntry_information_Item(el)
    }
}

pub fn _decode_FamilyEntry_information_Item(
    el: &X690Element,
) -> ASN1Result<FamilyEntry_information_Item> {
    |el: &X690Element| -> ASN1Result<FamilyEntry_information_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 6) => Ok(FamilyEntry_information_Item::attributeType(
                _decode_AttributeType(&el)?,
            )),
            (TagClass::UNIVERSAL, 16) => Ok(FamilyEntry_information_Item::attribute(
                _decode_Attribute(&el)?,
            )),
            _ => Ok(FamilyEntry_information_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_FamilyEntry_information_Item(
    value_: &FamilyEntry_information_Item,
) -> ASN1Result<X690Element> {
    |value: &FamilyEntry_information_Item| -> ASN1Result<X690Element> {
        match value {
            FamilyEntry_information_Item::attributeType(v) => _encode_AttributeType(&v),
            FamilyEntry_information_Item::attribute(v) => _encode_Attribute(&v),
            FamilyEntry_information_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FilterItem-substrings-strings-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum FilterItem_substrings_strings_Item {
    initial(X690Element),
    any(X690Element),
    final_(X690Element),
    control(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for FilterItem_substrings_strings_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FilterItem_substrings_strings_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FilterItem_substrings_strings_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FilterItem_substrings_strings_Item(el)
    }
}

pub fn _decode_FilterItem_substrings_strings_Item(
    el: &X690Element,
) -> ASN1Result<FilterItem_substrings_strings_Item> {
    |el: &X690Element| -> ASN1Result<FilterItem_substrings_strings_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(FilterItem_substrings_strings_Item::initial(
                x690_identity(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(FilterItem_substrings_strings_Item::any(x690_identity(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 2) => Ok(FilterItem_substrings_strings_Item::final_(
                x690_identity(&el.inner()?)?,
            )),
            (TagClass::UNIVERSAL, 16) => Ok(FilterItem_substrings_strings_Item::control(
                _decode_Attribute(&el)?,
            )),
            _ => Ok(FilterItem_substrings_strings_Item::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_FilterItem_substrings_strings_Item(
    value_: &FilterItem_substrings_strings_Item,
) -> ASN1Result<X690Element> {
    |value: &FilterItem_substrings_strings_Item| -> ASN1Result<X690Element> {
        match value {
            FilterItem_substrings_strings_Item::initial(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem_substrings_strings_Item::any(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem_substrings_strings_Item::final_(v) => {
                |v_1: &X690Element| -> ASN1Result<X690Element> {
                    let el_1 = x690_identity(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            FilterItem_substrings_strings_Item::control(v) => _encode_Attribute(&v),
            FilterItem_substrings_strings_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FilterItem-substrings ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct FilterItem_substrings {
    pub type_: OBJECT_IDENTIFIER,
    pub strings: Vec<FilterItem_substrings_strings_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl FilterItem_substrings {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        strings: Vec<FilterItem_substrings_strings_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FilterItem_substrings {
            type_,
            strings,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for FilterItem_substrings {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FilterItem_substrings(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FilterItem_substrings {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FilterItem_substrings(el)
    }
}

pub const _rctl1_components_for_FilterItem_substrings: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "strings",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FilterItem_substrings: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FilterItem_substrings: &[ComponentSpec; 0] = &[];

pub fn _decode_FilterItem_substrings(el: &X690Element) -> ASN1Result<FilterItem_substrings> {
    |el_: &X690Element| -> ASN1Result<FilterItem_substrings> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FilterItem_substrings,
            _eal_components_for_FilterItem_substrings,
            _rctl2_components_for_FilterItem_substrings,
        )?;
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let strings =
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<FilterItem_substrings_strings_Item>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<FilterItem_substrings_strings_Item> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_FilterItem_substrings_strings_Item(el)?);
                }
                Ok(items)
            }(_components.get("strings").unwrap())?;
        Ok(FilterItem_substrings {
            type_,
            strings,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FilterItem_substrings(value_: &FilterItem_substrings) -> ASN1Result<X690Element> {
    |value_: &FilterItem_substrings| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.type_)?);
        components_.push(|value_: &SEQUENCE_OF<
            FilterItem_substrings_strings_Item,
        >|
         -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_FilterItem_substrings_strings_Item(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&value_.strings)?);
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
/// PagedResultsRequest-newRequest ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PagedResultsRequest_newRequest {
    pub pageSize: INTEGER,
    pub sortKeys: OPTIONAL<Vec<SortKey>>,
    pub reverse: OPTIONAL<BOOLEAN>,
    pub unmerged: OPTIONAL<BOOLEAN>,
    pub pageNumber: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl PagedResultsRequest_newRequest {
    pub fn new(
        pageSize: INTEGER,
        sortKeys: OPTIONAL<Vec<SortKey>>,
        reverse: OPTIONAL<BOOLEAN>,
        unmerged: OPTIONAL<BOOLEAN>,
        pageNumber: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PagedResultsRequest_newRequest {
            pageSize,
            sortKeys,
            reverse,
            unmerged,
            pageNumber,
            _unrecognized,
        }
    }
    pub fn _default_value_for_reverse() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_unmerged() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for PagedResultsRequest_newRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PagedResultsRequest_newRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PagedResultsRequest_newRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PagedResultsRequest_newRequest(el)
    }
}

pub const _rctl1_components_for_PagedResultsRequest_newRequest: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "pageSize",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sortKeys",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reverse",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "unmerged",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pageNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PagedResultsRequest_newRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PagedResultsRequest_newRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_PagedResultsRequest_newRequest(
    el: &X690Element,
) -> ASN1Result<PagedResultsRequest_newRequest> {
    |el_: &X690Element| -> ASN1Result<PagedResultsRequest_newRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PagedResultsRequest_newRequest,
            _eal_components_for_PagedResultsRequest_newRequest,
            _rctl2_components_for_PagedResultsRequest_newRequest,
        )?;
        let pageSize = ber_decode_integer(_components.get("pageSize").unwrap())?;
        let sortKeys: OPTIONAL<Vec<SortKey>> = match _components.get("sortKeys") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<SortKey>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<SortKey> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_SortKey(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        let reverse: OPTIONAL<BOOLEAN> = match _components.get("reverse") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let unmerged: OPTIONAL<BOOLEAN> = match _components.get("unmerged") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let pageNumber: OPTIONAL<INTEGER> = match _components.get("pageNumber") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(PagedResultsRequest_newRequest {
            pageSize,
            sortKeys,
            reverse,
            unmerged,
            pageNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PagedResultsRequest_newRequest(
    value_: &PagedResultsRequest_newRequest,
) -> ASN1Result<X690Element> {
    |value_: &PagedResultsRequest_newRequest| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_integer(&value_.pageSize)?);
        if let Some(v_) = &value_.sortKeys {
            components_.push(|value_: &SEQUENCE_OF<SortKey>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_SortKey(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.reverse {
            if *v_ != PagedResultsRequest_newRequest::_default_value_for_reverse() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.unmerged {
            if *v_ != PagedResultsRequest_newRequest::_default_value_for_unmerged() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.pageNumber {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
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
/// SimpleCredentials-validity-time1 ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum SimpleCredentials_validity_time1 {
    utc(UTCTime),
    gt(GeneralizedTime),
}

impl TryFrom<X690Element> for SimpleCredentials_validity_time1 {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_validity_time1(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SimpleCredentials_validity_time1 {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_validity_time1(el)
    }
}

pub fn _decode_SimpleCredentials_validity_time1(
    el: &X690Element,
) -> ASN1Result<SimpleCredentials_validity_time1> {
    |el: &X690Element| -> ASN1Result<SimpleCredentials_validity_time1> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 23) => Ok(SimpleCredentials_validity_time1::utc(
                ber_decode_utc_time(&el)?,
            )),
            (TagClass::UNIVERSAL, 24) => Ok(SimpleCredentials_validity_time1::gt(
                ber_decode_generalized_time(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_SimpleCredentials_validity_time1(
    value_: &SimpleCredentials_validity_time1,
) -> ASN1Result<X690Element> {
    |value: &SimpleCredentials_validity_time1| -> ASN1Result<X690Element> {
        match value {
            SimpleCredentials_validity_time1::utc(v) => ber_encode_utc_time(&v),
            SimpleCredentials_validity_time1::gt(v) => ber_encode_generalized_time(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SimpleCredentials-validity-time2 ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum SimpleCredentials_validity_time2 {
    utc(UTCTime),
    gt(GeneralizedTime),
}

impl TryFrom<X690Element> for SimpleCredentials_validity_time2 {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_validity_time2(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SimpleCredentials_validity_time2 {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_validity_time2(el)
    }
}

pub fn _decode_SimpleCredentials_validity_time2(
    el: &X690Element,
) -> ASN1Result<SimpleCredentials_validity_time2> {
    |el: &X690Element| -> ASN1Result<SimpleCredentials_validity_time2> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 23) => Ok(SimpleCredentials_validity_time2::utc(
                ber_decode_utc_time(&el)?,
            )),
            (TagClass::UNIVERSAL, 24) => Ok(SimpleCredentials_validity_time2::gt(
                ber_decode_generalized_time(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_SimpleCredentials_validity_time2(
    value_: &SimpleCredentials_validity_time2,
) -> ASN1Result<X690Element> {
    |value: &SimpleCredentials_validity_time2| -> ASN1Result<X690Element> {
        match value {
            SimpleCredentials_validity_time2::utc(v) => ber_encode_utc_time(&v),
            SimpleCredentials_validity_time2::gt(v) => ber_encode_generalized_time(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SimpleCredentials-validity ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SimpleCredentials_validity {
    pub time1: OPTIONAL<SimpleCredentials_validity_time1>,
    pub time2: OPTIONAL<SimpleCredentials_validity_time2>,
    pub random1: OPTIONAL<BIT_STRING>,
    pub random2: OPTIONAL<BIT_STRING>,
}
impl SimpleCredentials_validity {
    pub fn new(
        time1: OPTIONAL<SimpleCredentials_validity_time1>,
        time2: OPTIONAL<SimpleCredentials_validity_time2>,
        random1: OPTIONAL<BIT_STRING>,
        random2: OPTIONAL<BIT_STRING>,
    ) -> Self {
        SimpleCredentials_validity {
            time1,
            time2,
            random1,
            random2,
        }
    }
}
impl Default for SimpleCredentials_validity {
    fn default() -> Self {
        SimpleCredentials_validity {
            time1: None,
            time2: None,
            random1: None,
            random2: None,
        }
    }
}
impl TryFrom<X690Element> for SimpleCredentials_validity {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_validity(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SimpleCredentials_validity {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_validity(el)
    }
}

pub const _rctl1_components_for_SimpleCredentials_validity: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "time1",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time2",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "random1",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "random2",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SimpleCredentials_validity: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SimpleCredentials_validity: &[ComponentSpec; 0] = &[];

pub fn _decode_SimpleCredentials_validity(
    el: &X690Element,
) -> ASN1Result<SimpleCredentials_validity> {
    |el_: &X690Element| -> ASN1Result<SimpleCredentials_validity> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SimpleCredentials_validity,
            _eal_components_for_SimpleCredentials_validity,
            _rctl2_components_for_SimpleCredentials_validity,
            40,
        )?;
        let time1: OPTIONAL<SimpleCredentials_validity_time1> = match _components.get("time1") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SimpleCredentials_validity_time1> {
                    Ok(_decode_SimpleCredentials_validity_time1(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let time2: OPTIONAL<SimpleCredentials_validity_time2> = match _components.get("time2") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SimpleCredentials_validity_time2> {
                    Ok(_decode_SimpleCredentials_validity_time2(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let random1: OPTIONAL<BIT_STRING> = match _components.get("random1") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let random2: OPTIONAL<BIT_STRING> = match _components.get("random2") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BIT_STRING> {
                Ok(ber_decode_bit_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SimpleCredentials_validity {
            time1,
            time2,
            random1,
            random2,
        })
    }(&el)
}

pub fn _encode_SimpleCredentials_validity(
    value_: &SimpleCredentials_validity,
) -> ASN1Result<X690Element> {
    |value_: &SimpleCredentials_validity| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        if let Some(v_) = &value_.time1 {
            components_.push(
                |v_1: &SimpleCredentials_validity_time1| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SimpleCredentials_validity_time1(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.time2 {
            components_.push(
                |v_1: &SimpleCredentials_validity_time2| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SimpleCredentials_validity_time2(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.random1 {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.random2 {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_bit_string(
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
/// SimpleCredentials-password ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum SimpleCredentials_password {
    unprotected(OCTET_STRING),
    protected(HASH),
    userPwd(UserPwd),           /* CHOICE_ALT_EXT */
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for SimpleCredentials_password {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_password(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SimpleCredentials_password {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SimpleCredentials_password(el)
    }
}

pub fn _decode_SimpleCredentials_password(
    el: &X690Element,
) -> ASN1Result<SimpleCredentials_password> {
    |el: &X690Element| -> ASN1Result<SimpleCredentials_password> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 4) => Ok(SimpleCredentials_password::unprotected(
                ber_decode_octet_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 16) => {
                Ok(SimpleCredentials_password::protected(_decode_HASH(&el)?))
            }
            (TagClass::CONTEXT, 0) => Ok(SimpleCredentials_password::userPwd(
                |el: &X690Element| -> ASN1Result<UserPwd> { Ok(_decode_UserPwd(&el.inner()?)?) }(
                    &el,
                )?,
            )),
            _ => Ok(SimpleCredentials_password::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_SimpleCredentials_password(
    value_: &SimpleCredentials_password,
) -> ASN1Result<X690Element> {
    |value: &SimpleCredentials_password| -> ASN1Result<X690Element> {
        match value {
            SimpleCredentials_password::unprotected(v) => ber_encode_octet_string(&v),
            SimpleCredentials_password::protected(v) => _encode_HASH(&v),
            SimpleCredentials_password::userPwd(v) => |v_1: &UserPwd| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UserPwd(&v_1)?))),
                ))
            }(&v),
            SimpleCredentials_password::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdResponseValue-warning ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum PwdResponseValue_warning {
    timeLeft(INTEGER),
    graceRemaining(INTEGER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for PwdResponseValue_warning {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponseValue_warning(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PwdResponseValue_warning {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponseValue_warning(el)
    }
}

pub fn _decode_PwdResponseValue_warning(el: &X690Element) -> ASN1Result<PwdResponseValue_warning> {
    |el: &X690Element| -> ASN1Result<PwdResponseValue_warning> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(PwdResponseValue_warning::timeLeft(ber_decode_integer(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(PwdResponseValue_warning::graceRemaining(
                ber_decode_integer(&el.inner()?)?,
            )),
            _ => Ok(PwdResponseValue_warning::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_PwdResponseValue_warning(
    value_: &PwdResponseValue_warning,
) -> ASN1Result<X690Element> {
    |value: &PwdResponseValue_warning| -> ASN1Result<X690Element> {
        match value {
            PwdResponseValue_warning::timeLeft(v) => |v_1: &INTEGER| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_integer(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            PwdResponseValue_warning::graceRemaining(v) => {
                |v_1: &INTEGER| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_integer(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            PwdResponseValue_warning::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdResponseValue-error ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type PwdResponseValue_error = ENUMERATED;

pub const PwdResponseValue_error_passwordExpired: PwdResponseValue_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PwdResponseValue_error_changeAfterReset: PwdResponseValue_error = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_PwdResponseValue_error(el: &X690Element) -> ASN1Result<PwdResponseValue_error> {
    ber_decode_enumerated(&el)
}

pub fn _encode_PwdResponseValue_error(value_: &PwdResponseValue_error) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryBindError-OPTIONALLY-PROTECTED-Parameter1-error ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error {
    serviceError(ServiceProblem),
    securityError(SecurityProblem),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error(el)
    }
}

pub fn _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error(
    el: &X690Element,
) -> ASN1Result<DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error> {
    |el: &X690Element| -> ASN1Result<DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 1) => Ok(
                DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error::serviceError(
                    _decode_ServiceProblem(&el.inner()?)?,
                ),
            ),
            (TagClass::CONTEXT, 2) => Ok(
                DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error::securityError(
                    _decode_SecurityProblem(&el.inner()?)?,
                ),
            ),
            _ => Ok(
                DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error::_unrecognized(el.clone()),
            ),
        }
    }(&el)
}

pub fn _encode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error(
    value_: &DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error,
) -> ASN1Result<X690Element> {
    |value: &DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error| -> ASN1Result<X690Element> {
        match value {
            DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error::serviceError(v) => {
                |v_1: &ServiceProblem| -> ASN1Result<X690Element> {
                    let el_1 = _encode_ServiceProblem(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error::securityError(v) => {
                |v_1: &SecurityProblem| -> ASN1Result<X690Element> {
                    let el_1 = _encode_SecurityProblem(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1_error::_unrecognized(el) => {
                Ok(el.clone())
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRights-Item-item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ModifyRights_Item_item {
    entry(NULL),
    attribute(AttributeType),
    value(AttributeValueAssertion),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ModifyRights_Item_item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRights_Item_item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyRights_Item_item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRights_Item_item(el)
    }
}

pub fn _decode_ModifyRights_Item_item(el: &X690Element) -> ASN1Result<ModifyRights_Item_item> {
    |el: &X690Element| -> ASN1Result<ModifyRights_Item_item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ModifyRights_Item_item::entry(ber_decode_null(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(ModifyRights_Item_item::attribute(_decode_AttributeType(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 2) => Ok(ModifyRights_Item_item::value(
                _decode_AttributeValueAssertion(&el.inner()?)?,
            )),
            _ => Ok(ModifyRights_Item_item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ModifyRights_Item_item(value_: &ModifyRights_Item_item) -> ASN1Result<X690Element> {
    |value: &ModifyRights_Item_item| -> ASN1Result<X690Element> {
        match value {
            ModifyRights_Item_item::entry(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_null(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            ModifyRights_Item_item::attribute(v) => {
                |v_1: &AttributeType| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeType(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ModifyRights_Item_item::value(v) => {
                |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                    let el_1 = _encode_AttributeValueAssertion(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ModifyRights_Item_item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRights-Item-permission ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ModifyRights_Item_permission = BIT_STRING;

pub const ModifyRights_Item_permission_add: BIT = 0; /* LONG_NAMED_BIT */

pub const ModifyRights_Item_permission_remove: BIT = 1; /* LONG_NAMED_BIT */

pub const ModifyRights_Item_permission_rename: BIT = 2; /* LONG_NAMED_BIT */

pub const ModifyRights_Item_permission_move_: BIT = 3; /* LONG_NAMED_BIT */

pub fn _decode_ModifyRights_Item_permission(
    el: &X690Element,
) -> ASN1Result<ModifyRights_Item_permission> {
    ber_decode_bit_string(&el)
}

pub fn _encode_ModifyRights_Item_permission(
    value_: &ModifyRights_Item_permission,
) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRights-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyRights_Item {
    pub item: ModifyRights_Item_item,
    pub permission: ModifyRights_Item_permission,
    pub _unrecognized: Vec<X690Element>,
}
impl ModifyRights_Item {
    pub fn new(
        item: ModifyRights_Item_item,
        permission: ModifyRights_Item_permission,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ModifyRights_Item {
            item,
            permission,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ModifyRights_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRights_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyRights_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRights_Item(el)
    }
}

pub const _rctl1_components_for_ModifyRights_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new("item", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "permission",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyRights_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModifyRights_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyRights_Item(el: &X690Element) -> ASN1Result<ModifyRights_Item> {
    |el_: &X690Element| -> ASN1Result<ModifyRights_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyRights_Item,
            _eal_components_for_ModifyRights_Item,
            _rctl2_components_for_ModifyRights_Item,
        )?;
        let item = _decode_ModifyRights_Item_item(_components.get("item").unwrap())?;
        let permission = |el: &X690Element| -> ASN1Result<ModifyRights_Item_permission> {
            Ok(_decode_ModifyRights_Item_permission(&el.inner()?)?)
        }(_components.get("permission").unwrap())?;
        Ok(ModifyRights_Item {
            item,
            permission,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModifyRights_Item(value_: &ModifyRights_Item) -> ASN1Result<X690Element> {
    |value_: &ModifyRights_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_ModifyRights_Item_item(&value_.item)?);
        components_.push(
            |v_1: &ModifyRights_Item_permission| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_ModifyRights_Item_permission(&v_1)?,
                    ))),
                ))
            }(&value_.permission)?,
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
/// ListResultData-listInfo-subordinates-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ListResultData_listInfo_subordinates_Item {
    pub rdn: RelativeDistinguishedName,
    pub aliasEntry: OPTIONAL<BOOLEAN>,
    pub fromEntry: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl ListResultData_listInfo_subordinates_Item {
    pub fn new(
        rdn: RelativeDistinguishedName,
        aliasEntry: OPTIONAL<BOOLEAN>,
        fromEntry: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ListResultData_listInfo_subordinates_Item {
            rdn,
            aliasEntry,
            fromEntry,
            _unrecognized,
        }
    }
    pub fn _default_value_for_aliasEntry() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_fromEntry() -> BOOLEAN {
        true
    }
}
impl TryFrom<X690Element> for ListResultData_listInfo_subordinates_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ListResultData_listInfo_subordinates_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ListResultData_listInfo_subordinates_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ListResultData_listInfo_subordinates_Item(el)
    }
}

pub const _rctl1_components_for_ListResultData_listInfo_subordinates_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "rdn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "fromEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ListResultData_listInfo_subordinates_Item: &[ComponentSpec; 0] =
    &[];

pub const _eal_components_for_ListResultData_listInfo_subordinates_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_ListResultData_listInfo_subordinates_Item(
    el: &X690Element,
) -> ASN1Result<ListResultData_listInfo_subordinates_Item> {
    |el_: &X690Element| -> ASN1Result<ListResultData_listInfo_subordinates_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ListResultData_listInfo_subordinates_Item,
            _eal_components_for_ListResultData_listInfo_subordinates_Item,
            _rctl2_components_for_ListResultData_listInfo_subordinates_Item,
        )?;
        let rdn = _decode_RelativeDistinguishedName(_components.get("rdn").unwrap())?;
        let aliasEntry: OPTIONAL<BOOLEAN> = match _components.get("aliasEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let fromEntry: OPTIONAL<BOOLEAN> = match _components.get("fromEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ListResultData_listInfo_subordinates_Item {
            rdn,
            aliasEntry,
            fromEntry,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ListResultData_listInfo_subordinates_Item(
    value_: &ListResultData_listInfo_subordinates_Item,
) -> ASN1Result<X690Element> {
    |value_: &ListResultData_listInfo_subordinates_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_RelativeDistinguishedName(&value_.rdn)?);
        if let Some(v_) = &value_.aliasEntry {
            if *v_ != ListResultData_listInfo_subordinates_Item::_default_value_for_aliasEntry() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.fromEntry {
            if *v_ != ListResultData_listInfo_subordinates_Item::_default_value_for_fromEntry() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
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
/// ListResultData-listInfo ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ListResultData_listInfo {
    pub name: OPTIONAL<Name>,
    pub subordinates: Vec<ListResultData_listInfo_subordinates_Item>,
    pub partialOutcomeQualifier: OPTIONAL<PartialOutcomeQualifier>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl ListResultData_listInfo {
    pub fn new(
        name: OPTIONAL<Name>,
        subordinates: Vec<ListResultData_listInfo_subordinates_Item>,
        partialOutcomeQualifier: OPTIONAL<PartialOutcomeQualifier>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        ListResultData_listInfo {
            name,
            subordinates,
            partialOutcomeQualifier,
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
impl TryFrom<X690Element> for ListResultData_listInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ListResultData_listInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ListResultData_listInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ListResultData_listInfo(el)
    }
}

pub const _rctl1_components_for_ListResultData_listInfo: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "name",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::UNIVERSAL, 16))]),
        None,
        None,
    ),
    ComponentSpec::new(
        "subordinates",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "partialOutcomeQualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ListResultData_listInfo: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_ListResultData_listInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ListResultData_listInfo(el: &X690Element) -> ASN1Result<ListResultData_listInfo> {
    |el_: &X690Element| -> ASN1Result<ListResultData_listInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ListResultData_listInfo,
            _eal_components_for_ListResultData_listInfo,
            _rctl2_components_for_ListResultData_listInfo,
            80,
        )?;
        let name: OPTIONAL<Name> = match _components.get("name") {
            Some(c_) => Some(_decode_Name(c_)?),
            _ => None,
        };
        let subordinates =
            |el: &X690Element| -> ASN1Result<Vec<ListResultData_listInfo_subordinates_Item>> {
                Ok(|el: &X690Element| -> ASN1Result<
                    SET_OF<ListResultData_listInfo_subordinates_Item>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<ListResultData_listInfo_subordinates_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_ListResultData_listInfo_subordinates_Item(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(_components.get("subordinates").unwrap())?;
        let partialOutcomeQualifier: OPTIONAL<PartialOutcomeQualifier> =
            match _components.get("partialOutcomeQualifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<PartialOutcomeQualifier> {
                    Ok(_decode_PartialOutcomeQualifier(&el.inner()?)?)
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
        Ok(ListResultData_listInfo {
            name,
            subordinates,
            partialOutcomeQualifier,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_ListResultData_listInfo(
    value_: &ListResultData_listInfo,
) -> ASN1Result<X690Element> {
    |value_: &ListResultData_listInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        if let Some(v_) = &value_.name {
            components_.push(_encode_Name(&v_)?);
        }
        components_.push(
            |v_1: &Vec<ListResultData_listInfo_subordinates_Item>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        ListResultData_listInfo_subordinates_Item,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ListResultData_listInfo_subordinates_Item(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&value_.subordinates)?,
        );
        if let Some(v_) = &value_.partialOutcomeQualifier {
            components_.push(|v_1: &PartialOutcomeQualifier| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_PartialOutcomeQualifier(&v_1)?,
                    ))),
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
            if *v_ != ListResultData_listInfo::_default_value_for_aliasDereferenced() {
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
/// PartialOutcomeQualifier-entryCount ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum PartialOutcomeQualifier_entryCount {
    bestEstimate(INTEGER),
    lowEstimate(INTEGER),
    exact(INTEGER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for PartialOutcomeQualifier_entryCount {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PartialOutcomeQualifier_entryCount(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PartialOutcomeQualifier_entryCount {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PartialOutcomeQualifier_entryCount(el)
    }
}

pub fn _decode_PartialOutcomeQualifier_entryCount(
    el: &X690Element,
) -> ASN1Result<PartialOutcomeQualifier_entryCount> {
    |el: &X690Element| -> ASN1Result<PartialOutcomeQualifier_entryCount> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 7) => Ok(PartialOutcomeQualifier_entryCount::bestEstimate(
                ber_decode_integer(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 8) => Ok(PartialOutcomeQualifier_entryCount::lowEstimate(
                ber_decode_integer(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 9) => Ok(PartialOutcomeQualifier_entryCount::exact(
                ber_decode_integer(&el.inner()?)?,
            )),
            _ => Ok(PartialOutcomeQualifier_entryCount::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_PartialOutcomeQualifier_entryCount(
    value_: &PartialOutcomeQualifier_entryCount,
) -> ASN1Result<X690Element> {
    |value: &PartialOutcomeQualifier_entryCount| -> ASN1Result<X690Element> {
        match value {
            PartialOutcomeQualifier_entryCount::bestEstimate(v) => {
                |v_1: &INTEGER| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_integer(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        7,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            PartialOutcomeQualifier_entryCount::lowEstimate(v) => {
                |v_1: &INTEGER| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_integer(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        8,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            PartialOutcomeQualifier_entryCount::exact(v) => {
                |v_1: &INTEGER| -> ASN1Result<X690Element> {
                    let el_1 = ber_encode_integer(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        9,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            PartialOutcomeQualifier_entryCount::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchArgumentData-subset ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type SearchArgumentData_subset = INTEGER;

pub const SearchArgumentData_subset_baseObject: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const SearchArgumentData_subset_oneLevel: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const SearchArgumentData_subset_wholeSubtree: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_SearchArgumentData_subset(
    el: &X690Element,
) -> ASN1Result<SearchArgumentData_subset> {
    ber_decode_integer(&el)
}

pub fn _encode_SearchArgumentData_subset(
    value_: &SearchArgumentData_subset,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchArgumentData-joinType ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type SearchArgumentData_joinType = ENUMERATED;

pub const SearchArgumentData_joinType_innerJoin: SearchArgumentData_joinType = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchArgumentData_joinType_leftOuterJoin: SearchArgumentData_joinType = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchArgumentData_joinType_fullOuterJoin: SearchArgumentData_joinType = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SearchArgumentData_joinType(
    el: &X690Element,
) -> ASN1Result<SearchArgumentData_joinType> {
    ber_decode_enumerated(&el)
}

pub fn _encode_SearchArgumentData_joinType(
    value_: &SearchArgumentData_joinType,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// JoinArgument-joinSubset ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type JoinArgument_joinSubset = ENUMERATED;

pub const JoinArgument_joinSubset_baseObject: JoinArgument_joinSubset = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const JoinArgument_joinSubset_oneLevel: JoinArgument_joinSubset = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const JoinArgument_joinSubset_wholeSubtree: JoinArgument_joinSubset = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_JoinArgument_joinSubset(el: &X690Element) -> ASN1Result<JoinArgument_joinSubset> {
    ber_decode_enumerated(&el)
}

pub fn _encode_JoinArgument_joinSubset(
    value_: &JoinArgument_joinSubset,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResultData-searchInfo ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchResultData_searchInfo {
    pub name: OPTIONAL<Name>,
    pub entries: Vec<EntryInformation>,
    pub partialOutcomeQualifier: OPTIONAL<PartialOutcomeQualifier>,
    pub altMatching: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl SearchResultData_searchInfo {
    pub fn new(
        name: OPTIONAL<Name>,
        entries: Vec<EntryInformation>,
        partialOutcomeQualifier: OPTIONAL<PartialOutcomeQualifier>,
        altMatching: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        SearchResultData_searchInfo {
            name,
            entries,
            partialOutcomeQualifier,
            altMatching,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
            _unrecognized,
        }
    }
    pub fn _default_value_for_altMatching() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for SearchResultData_searchInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchResultData_searchInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchResultData_searchInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchResultData_searchInfo(el)
    }
}

pub const _rctl1_components_for_SearchResultData_searchInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "name",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::UNIVERSAL, 16))]),
        None,
        None,
    ),
    ComponentSpec::new(
        "entries",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "partialOutcomeQualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "altMatching",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchResultData_searchInfo: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_SearchResultData_searchInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchResultData_searchInfo(
    el: &X690Element,
) -> ASN1Result<SearchResultData_searchInfo> {
    |el_: &X690Element| -> ASN1Result<SearchResultData_searchInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SearchResultData_searchInfo,
            _eal_components_for_SearchResultData_searchInfo,
            _rctl2_components_for_SearchResultData_searchInfo,
            90,
        )?;
        let name: OPTIONAL<Name> = match _components.get("name") {
            Some(c_) => Some(_decode_Name(c_)?),
            _ => None,
        };
        let entries = |el: &X690Element| -> ASN1Result<Vec<EntryInformation>> {
            Ok(|el: &X690Element| -> ASN1Result<SET_OF<EntryInformation>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<EntryInformation> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_EntryInformation(el)?);
                }
                Ok(items)
            }(&el.inner()?)?)
        }(_components.get("entries").unwrap())?;
        let partialOutcomeQualifier: OPTIONAL<PartialOutcomeQualifier> =
            match _components.get("partialOutcomeQualifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<PartialOutcomeQualifier> {
                    Ok(_decode_PartialOutcomeQualifier(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let altMatching: OPTIONAL<BOOLEAN> = match _components.get("altMatching") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
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
        Ok(SearchResultData_searchInfo {
            name,
            entries,
            partialOutcomeQualifier,
            altMatching,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_SearchResultData_searchInfo(
    value_: &SearchResultData_searchInfo,
) -> ASN1Result<X690Element> {
    |value_: &SearchResultData_searchInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        if let Some(v_) = &value_.name {
            components_.push(_encode_Name(&v_)?);
        }
        components_.push(|v_1: &Vec<EntryInformation>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                    EntryInformation,
                >|
                 -> ASN1Result<
                    X690Element,
                > {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_EntryInformation(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.entries)?);
        if let Some(v_) = &value_.partialOutcomeQualifier {
            components_.push(|v_1: &PartialOutcomeQualifier| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_PartialOutcomeQualifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.altMatching {
            if *v_ != SearchResultData_searchInfo::_default_value_for_altMatching() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
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
            if *v_ != SearchResultData_searchInfo::_default_value_for_aliasDereferenced() {
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
/// AttributeErrorData-problems-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeErrorData_problems_Item {
    pub problem: AttributeProblem,
    pub type_: AttributeType,
    pub value: OPTIONAL<AttributeValue>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeErrorData_problems_Item {
    pub fn new(
        problem: AttributeProblem,
        type_: AttributeType,
        value: OPTIONAL<AttributeValue>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeErrorData_problems_Item {
            problem,
            type_,
            value,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeErrorData_problems_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeErrorData_problems_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeErrorData_problems_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeErrorData_problems_Item(el)
    }
}

pub const _rctl1_components_for_AttributeErrorData_problems_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "problem",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeErrorData_problems_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeErrorData_problems_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeErrorData_problems_Item(
    el: &X690Element,
) -> ASN1Result<AttributeErrorData_problems_Item> {
    |el_: &X690Element| -> ASN1Result<AttributeErrorData_problems_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeErrorData_problems_Item,
            _eal_components_for_AttributeErrorData_problems_Item,
            _rctl2_components_for_AttributeErrorData_problems_Item,
        )?;
        let problem = |el: &X690Element| -> ASN1Result<AttributeProblem> {
            Ok(_decode_AttributeProblem(&el.inner()?)?)
        }(_components.get("problem").unwrap())?;
        let type_ = |el: &X690Element| -> ASN1Result<AttributeType> {
            Ok(_decode_AttributeType(&el.inner()?)?)
        }(_components.get("type").unwrap())?;
        let value: OPTIONAL<AttributeValue> = match _components.get("value") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AttributeValue> {
                Ok(_decode_AttributeValue(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(AttributeErrorData_problems_Item {
            problem,
            type_,
            value,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeErrorData_problems_Item(
    value_: &AttributeErrorData_problems_Item,
) -> ASN1Result<X690Element> {
    |value_: &AttributeErrorData_problems_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &AttributeProblem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AttributeProblem(
                    &v_1,
                )?))),
            ))
        }(&value_.problem)?);
        components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AttributeType(
                    &v_1,
                )?))),
            ))
        }(&value_.type_)?);
        if let Some(v_) = &value_.value {
            components_.push(|v_1: &AttributeValue| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AttributeValue(
                        &v_1,
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
/// UpdateErrorData-attributeInfo-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum UpdateErrorData_attributeInfo_Item {
    attributeType(AttributeType),
    attribute(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for UpdateErrorData_attributeInfo_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateErrorData_attributeInfo_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UpdateErrorData_attributeInfo_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UpdateErrorData_attributeInfo_Item(el)
    }
}

pub fn _decode_UpdateErrorData_attributeInfo_Item(
    el: &X690Element,
) -> ASN1Result<UpdateErrorData_attributeInfo_Item> {
    |el: &X690Element| -> ASN1Result<UpdateErrorData_attributeInfo_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 6) => Ok(UpdateErrorData_attributeInfo_Item::attributeType(
                _decode_AttributeType(&el)?,
            )),
            (TagClass::UNIVERSAL, 16) => Ok(UpdateErrorData_attributeInfo_Item::attribute(
                _decode_Attribute(&el)?,
            )),
            _ => Ok(UpdateErrorData_attributeInfo_Item::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_UpdateErrorData_attributeInfo_Item(
    value_: &UpdateErrorData_attributeInfo_Item,
) -> ASN1Result<X690Element> {
    |value: &UpdateErrorData_attributeInfo_Item| -> ASN1Result<X690Element> {
        match value {
            UpdateErrorData_attributeInfo_Item::attributeType(v) => _encode_AttributeType(&v),
            UpdateErrorData_attributeInfo_Item::attribute(v) => _encode_Attribute(&v),
            UpdateErrorData_attributeInfo_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}
