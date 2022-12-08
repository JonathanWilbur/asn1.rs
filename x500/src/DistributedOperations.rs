#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DistributedOperations
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DistributedOperations`.
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
use crate::BasicAccessControl::*;
use crate::CommonProtocolSpecification::*;
use crate::DirectoryAbstractService::*;
use crate::InformationFramework::*;
use crate::SelectedAttributeTypes::*;
use crate::ServiceAdministration::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsaReferral ERROR ::= {
///   PARAMETER           OPTIONALLY-PROTECTED { DsaReferralData }
///   CODE                id-errcode-dsaReferral }
/// ```
///
///
pub fn dsaReferral() -> ERROR {
    ERROR {
        errorCode: Some(id_errcode_dsaReferral), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DsaReferralData ::= SET {
///   reference      [0]  ContinuationReference,
///   contextPrefix  [1]  DistinguishedName OPTIONAL,
///   ...,
///   ...,
///   COMPONENTS OF       CommonResults }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DsaReferralData {
    pub reference: ContinuationReference,
    pub contextPrefix: OPTIONAL<DistinguishedName>,
    pub _unrecognized: Vec<X690Element>,
    pub securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
    pub performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
    pub notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
}
impl DsaReferralData {
    pub fn new(
        reference: ContinuationReference,
        contextPrefix: OPTIONAL<DistinguishedName>,
        _unrecognized: Vec<X690Element>,
        securityParameters: OPTIONAL<SecurityParameters>, /* REPLICATED_COMPONENT */
        performer: OPTIONAL<DistinguishedName>,           /* REPLICATED_COMPONENT */
        aliasDereferenced: OPTIONAL<BOOLEAN>,             /* REPLICATED_COMPONENT */
        notification: OPTIONAL<Vec<Attribute>>,           /* REPLICATED_COMPONENT */
    ) -> Self {
        DsaReferralData {
            reference,
            contextPrefix,
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
impl TryFrom<X690Element> for DsaReferralData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DsaReferralData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DsaReferralData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DsaReferralData(el)
    }
}

pub const _rctl1_components_for_DsaReferralData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "reference",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextPrefix",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DsaReferralData: &[ComponentSpec; 4] = &[
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

pub const _eal_components_for_DsaReferralData: &[ComponentSpec; 0] = &[];

pub fn _decode_DsaReferralData(el: &X690Element) -> ASN1Result<DsaReferralData> {
    |el_: &X690Element| -> ASN1Result<DsaReferralData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_DsaReferralData,
            _eal_components_for_DsaReferralData,
            _rctl2_components_for_DsaReferralData,
            70,
        )?;
        let reference = |el: &X690Element| -> ASN1Result<ContinuationReference> {
            Ok(_decode_ContinuationReference(&el.inner()?)?)
        }(_components.get("reference").unwrap())?;
        let contextPrefix: OPTIONAL<DistinguishedName> = match _components.get("contextPrefix") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
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
        Ok(DsaReferralData {
            reference,
            contextPrefix,
            _unrecognized,
            securityParameters,
            performer,
            aliasDereferenced,
            notification,
        })
    }(&el)
}

pub fn _encode_DsaReferralData(value_: &DsaReferralData) -> ASN1Result<X690Element> {
    |value_: &DsaReferralData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &ContinuationReference| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_ContinuationReference(&v_1)?,
                ))),
            ))
        }(&value_.reference)?);
        if let Some(v_) = &value_.contextPrefix {
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
            if *v_ != DsaReferralData::_default_value_for_aliasDereferenced() {
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
/// ChainingArguments ::= SET {
///   originator                 [0]  DistinguishedName OPTIONAL,
///   targetObject               [1]  DistinguishedName OPTIONAL,
///   operationProgress          [2]  OperationProgress
///                                DEFAULT {nameResolutionPhase notStarted},
///   traceInformation           [3]  TraceInformation,
///   aliasDereferenced          [4]  BOOLEAN DEFAULT FALSE,
///   aliasedRDNs                [5]  INTEGER OPTIONAL,
///   returnCrossRefs            [6]  BOOLEAN DEFAULT FALSE,
///   referenceType              [7]  ReferenceType DEFAULT superior,
///   info                       [8]  DomainInfo OPTIONAL,
///   timeLimit                  [9]  Time OPTIONAL,
///   securityParameters         [10] SecurityParameters DEFAULT {},
///   entryOnly                  [11] BOOLEAN DEFAULT FALSE,
///   uniqueIdentifier           [12] UniqueIdentifier OPTIONAL,
///   authenticationLevel        [13] AuthenticationLevel OPTIONAL,
///   exclusions                 [14] Exclusions OPTIONAL,
///   excludeShadows             [15] BOOLEAN DEFAULT FALSE,
///   nameResolveOnMaster        [16] BOOLEAN DEFAULT FALSE,
///   operationIdentifier        [17] INTEGER OPTIONAL,
///   searchRuleId               [18] SearchRuleId OPTIONAL,
///   chainedRelaxation          [19] MRMapping OPTIONAL,
///   relatedEntry               [20] INTEGER OPTIONAL,
///   dspPaging                  [21] BOOLEAN DEFAULT FALSE,
///   --                         [22] Not to be used
///   --                         [23] Not to be used
///   excludeWriteableCopies     [24] BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ChainingArguments {
    pub originator: OPTIONAL<DistinguishedName>,
    pub targetObject: OPTIONAL<DistinguishedName>,
    pub operationProgress: OPTIONAL<OperationProgress>,
    pub traceInformation: TraceInformation,
    pub aliasDereferenced: OPTIONAL<BOOLEAN>,
    pub aliasedRDNs: OPTIONAL<INTEGER>,
    pub returnCrossRefs: OPTIONAL<BOOLEAN>,
    pub referenceType: OPTIONAL<ReferenceType>,
    pub info: OPTIONAL<DomainInfo>,
    pub timeLimit: OPTIONAL<Time>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub entryOnly: OPTIONAL<BOOLEAN>,
    pub uniqueIdentifier: OPTIONAL<UniqueIdentifier>,
    pub authenticationLevel: OPTIONAL<AuthenticationLevel>,
    pub exclusions: OPTIONAL<Exclusions>,
    pub excludeShadows: OPTIONAL<BOOLEAN>,
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,
    pub operationIdentifier: OPTIONAL<INTEGER>,
    pub searchRuleId: OPTIONAL<SearchRuleId>,
    pub chainedRelaxation: OPTIONAL<MRMapping>,
    pub relatedEntry: OPTIONAL<INTEGER>,
    pub dspPaging: OPTIONAL<BOOLEAN>,
    pub excludeWriteableCopies: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl ChainingArguments {
    pub fn new(
        originator: OPTIONAL<DistinguishedName>,
        targetObject: OPTIONAL<DistinguishedName>,
        operationProgress: OPTIONAL<OperationProgress>,
        traceInformation: TraceInformation,
        aliasDereferenced: OPTIONAL<BOOLEAN>,
        aliasedRDNs: OPTIONAL<INTEGER>,
        returnCrossRefs: OPTIONAL<BOOLEAN>,
        referenceType: OPTIONAL<ReferenceType>,
        info: OPTIONAL<DomainInfo>,
        timeLimit: OPTIONAL<Time>,
        securityParameters: OPTIONAL<SecurityParameters>,
        entryOnly: OPTIONAL<BOOLEAN>,
        uniqueIdentifier: OPTIONAL<UniqueIdentifier>,
        authenticationLevel: OPTIONAL<AuthenticationLevel>,
        exclusions: OPTIONAL<Exclusions>,
        excludeShadows: OPTIONAL<BOOLEAN>,
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,
        operationIdentifier: OPTIONAL<INTEGER>,
        searchRuleId: OPTIONAL<SearchRuleId>,
        chainedRelaxation: OPTIONAL<MRMapping>,
        relatedEntry: OPTIONAL<INTEGER>,
        dspPaging: OPTIONAL<BOOLEAN>,
        excludeWriteableCopies: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ChainingArguments {
            originator,
            targetObject,
            operationProgress,
            traceInformation,
            aliasDereferenced,
            aliasedRDNs,
            returnCrossRefs,
            referenceType,
            info,
            timeLimit,
            securityParameters,
            entryOnly,
            uniqueIdentifier,
            authenticationLevel,
            exclusions,
            excludeShadows,
            nameResolveOnMaster,
            operationIdentifier,
            searchRuleId,
            chainedRelaxation,
            relatedEntry,
            dspPaging,
            excludeWriteableCopies,
            _unrecognized,
        }
    }
    pub fn _default_value_for_operationProgress() -> OperationProgress {
        OperationProgress {
            nameResolutionPhase: OperationProgress_nameResolutionPhase_notStarted,
            nextRDNToBeResolved: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_aliasDereferenced() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_returnCrossRefs() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_referenceType() -> ReferenceType {
        ReferenceType_superior
    }
    pub fn _default_value_for_securityParameters() -> SecurityParameters {
        SecurityParameters {
            certification_path: None,
            name: None,
            time: None,
            random: None,
            target: None,
            operationCode: None,
            errorProtection: None,
            errorCode: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_excludeShadows() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_dspPaging() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_excludeWriteableCopies() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ChainingArguments {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ChainingArguments(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ChainingArguments {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ChainingArguments(el)
    }
}

pub const _rctl1_components_for_ChainingArguments: &[ComponentSpec; 23] = &[
    ComponentSpec::new(
        "originator",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "targetObject",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "traceInformation",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasDereferenced",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "returnCrossRefs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "info",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeLimit",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
    ComponentSpec::new(
        "uniqueIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 12)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authenticationLevel",
        true,
        TagSelector::tag((TagClass::CONTEXT, 13)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 14)),
        None,
        None,
    ),
    ComponentSpec::new(
        "excludeShadows",
        true,
        TagSelector::tag((TagClass::CONTEXT, 15)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "searchRuleId",
        true,
        TagSelector::tag((TagClass::CONTEXT, 18)),
        None,
        None,
    ),
    ComponentSpec::new(
        "chainedRelaxation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "relatedEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dspPaging",
        true,
        TagSelector::tag((TagClass::CONTEXT, 21)),
        None,
        None,
    ),
    ComponentSpec::new(
        "excludeWriteableCopies",
        true,
        TagSelector::tag((TagClass::CONTEXT, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ChainingArguments: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ChainingArguments: &[ComponentSpec; 0] = &[];

pub fn _decode_ChainingArguments(el: &X690Element) -> ASN1Result<ChainingArguments> {
    |el_: &X690Element| -> ASN1Result<ChainingArguments> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ChainingArguments,
            _eal_components_for_ChainingArguments,
            _rctl2_components_for_ChainingArguments,
            240,
        )?;
        let originator: OPTIONAL<DistinguishedName> = match _components.get("originator") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistinguishedName> {
                Ok(_decode_DistinguishedName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let targetObject: OPTIONAL<DistinguishedName> = match _components.get("targetObject") {
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
        let traceInformation = |el: &X690Element| -> ASN1Result<TraceInformation> {
            Ok(_decode_TraceInformation(&el.inner()?)?)
        }(_components.get("traceInformation").unwrap())?;
        let aliasDereferenced: OPTIONAL<BOOLEAN> = match _components.get("aliasDereferenced") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let returnCrossRefs: OPTIONAL<BOOLEAN> = match _components.get("returnCrossRefs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType: OPTIONAL<ReferenceType> = match _components.get("referenceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ReferenceType> {
                Ok(_decode_ReferenceType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let info: OPTIONAL<DomainInfo> = match _components.get("info") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DomainInfo> {
                Ok(_decode_DomainInfo(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let timeLimit: OPTIONAL<Time> = match _components.get("timeLimit") {
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
        let entryOnly: OPTIONAL<BOOLEAN> = match _components.get("entryOnly") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let uniqueIdentifier: OPTIONAL<UniqueIdentifier> = match _components.get("uniqueIdentifier")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<UniqueIdentifier> {
                Ok(_decode_UniqueIdentifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let authenticationLevel: OPTIONAL<AuthenticationLevel> =
            match _components.get("authenticationLevel") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<AuthenticationLevel> {
                    Ok(_decode_AuthenticationLevel(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let exclusions: OPTIONAL<Exclusions> = match _components.get("exclusions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let excludeShadows: OPTIONAL<BOOLEAN> = match _components.get("excludeShadows") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationIdentifier: OPTIONAL<INTEGER> = match _components.get("operationIdentifier") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let searchRuleId: OPTIONAL<SearchRuleId> = match _components.get("searchRuleId") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SearchRuleId> {
                Ok(_decode_SearchRuleId(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let chainedRelaxation: OPTIONAL<MRMapping> = match _components.get("chainedRelaxation") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<MRMapping> {
                Ok(_decode_MRMapping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let relatedEntry: OPTIONAL<INTEGER> = match _components.get("relatedEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let dspPaging: OPTIONAL<BOOLEAN> = match _components.get("dspPaging") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let excludeWriteableCopies: OPTIONAL<BOOLEAN> =
            match _components.get("excludeWriteableCopies") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                    Ok(ber_decode_boolean(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(ChainingArguments {
            originator,
            targetObject,
            operationProgress,
            traceInformation,
            aliasDereferenced,
            aliasedRDNs,
            returnCrossRefs,
            referenceType,
            info,
            timeLimit,
            securityParameters,
            entryOnly,
            uniqueIdentifier,
            authenticationLevel,
            exclusions,
            excludeShadows,
            nameResolveOnMaster,
            operationIdentifier,
            searchRuleId,
            chainedRelaxation,
            relatedEntry,
            dspPaging,
            excludeWriteableCopies,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ChainingArguments(value_: &ChainingArguments) -> ASN1Result<X690Element> {
    |value_: &ChainingArguments| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(33);
        if let Some(v_) = &value_.originator {
            components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.targetObject {
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
        if let Some(v_) = &value_.operationProgress {
            if *v_ != ChainingArguments::_default_value_for_operationProgress() {
                components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        components_.push(|v_1: &TraceInformation| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_TraceInformation(
                    &v_1,
                )?))),
            ))
        }(&value_.traceInformation)?);
        if let Some(v_) = &value_.aliasDereferenced {
            if *v_ != ChainingArguments::_default_value_for_aliasDereferenced() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.returnCrossRefs {
            if *v_ != ChainingArguments::_default_value_for_returnCrossRefs() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        6,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.referenceType {
            if *v_ != ChainingArguments::_default_value_for_referenceType() {
                components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        7,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.info {
            components_.push(|v_1: &DomainInfo| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DomainInfo(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.timeLimit {
            components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    10,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryOnly {
            if *v_ != ChainingArguments::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        11,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.uniqueIdentifier {
            components_.push(|v_1: &UniqueIdentifier| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    12,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UniqueIdentifier(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.authenticationLevel {
            components_.push(|v_1: &AuthenticationLevel| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    13,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_AuthenticationLevel(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    14,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.excludeShadows {
            if *v_ != ChainingArguments::_default_value_for_excludeShadows() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        15,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != ChainingArguments::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        16,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.operationIdentifier {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    17,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.searchRuleId {
            components_.push(|v_1: &SearchRuleId| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    18,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_SearchRuleId(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.chainedRelaxation {
            components_.push(|v_1: &MRMapping| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    19,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_MRMapping(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.relatedEntry {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    20,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.dspPaging {
            if *v_ != ChainingArguments::_default_value_for_dspPaging() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        21,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.excludeWriteableCopies {
            if *v_ != ChainingArguments::_default_value_for_excludeWriteableCopies() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        24,
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
/// Time  ::=  CHOICE {
///   utcTime          UTCTime,
///   generalizedTime  GeneralizedTime,
///   ... }
/// ```
#[derive(Debug, Clone)]
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
/// DomainInfo  ::=  ABSTRACT-SYNTAX.&Type
/// ```
pub type DomainInfo = X690Element; // ObjectClassFieldType

pub fn _decode_DomainInfo(el: &X690Element) -> ASN1Result<DomainInfo> {
    x690_identity(&el)
}

pub fn _encode_DomainInfo(value_: &DomainInfo) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChainingResults ::= SET {
///   info                [0]  DomainInfo OPTIONAL,
///   crossReferences     [1]  SEQUENCE SIZE (1..MAX) OF CrossReference OPTIONAL,
///   securityParameters  [2]  SecurityParameters DEFAULT {},
///   alreadySearched     [3]  Exclusions OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ChainingResults {
    pub info: OPTIONAL<DomainInfo>,
    pub crossReferences: OPTIONAL<Vec<CrossReference>>,
    pub securityParameters: OPTIONAL<SecurityParameters>,
    pub alreadySearched: OPTIONAL<Exclusions>,
    pub _unrecognized: Vec<X690Element>,
}
impl ChainingResults {
    pub fn new(
        info: OPTIONAL<DomainInfo>,
        crossReferences: OPTIONAL<Vec<CrossReference>>,
        securityParameters: OPTIONAL<SecurityParameters>,
        alreadySearched: OPTIONAL<Exclusions>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ChainingResults {
            info,
            crossReferences,
            securityParameters,
            alreadySearched,
            _unrecognized,
        }
    }
    pub fn _default_value_for_securityParameters() -> SecurityParameters {
        SecurityParameters {
            certification_path: None,
            name: None,
            time: None,
            random: None,
            target: None,
            operationCode: None,
            errorProtection: None,
            errorCode: None,
            ..Default::default()
        }
    }
}
impl Default for ChainingResults {
    fn default() -> Self {
        ChainingResults {
            info: None,
            crossReferences: None,
            securityParameters: None,
            alreadySearched: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ChainingResults {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ChainingResults(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ChainingResults {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ChainingResults(el)
    }
}

pub const _rctl1_components_for_ChainingResults: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "info",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "crossReferences",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityParameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "alreadySearched",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ChainingResults: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ChainingResults: &[ComponentSpec; 0] = &[];

pub fn _decode_ChainingResults(el: &X690Element) -> ASN1Result<ChainingResults> {
    |el_: &X690Element| -> ASN1Result<ChainingResults> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ChainingResults,
            _eal_components_for_ChainingResults,
            _rctl2_components_for_ChainingResults,
            50,
        )?;
        let info: OPTIONAL<DomainInfo> = match _components.get("info") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DomainInfo> {
                Ok(_decode_DomainInfo(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let crossReferences: OPTIONAL<Vec<CrossReference>> = match _components
            .get("crossReferences")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<CrossReference>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CrossReference>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<CrossReference> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_CrossReference(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
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
        let alreadySearched: OPTIONAL<Exclusions> = match _components.get("alreadySearched") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Exclusions> {
                Ok(_decode_Exclusions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ChainingResults {
            info,
            crossReferences,
            securityParameters,
            alreadySearched,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ChainingResults(value_: &ChainingResults) -> ASN1Result<X690Element> {
    |value_: &ChainingResults| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.info {
            components_.push(|v_1: &DomainInfo| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DomainInfo(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.crossReferences {
            components_.push(|v_1: &Vec<CrossReference>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        CrossReference,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_CrossReference(&v)?);
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
        if let Some(v_) = &value_.securityParameters {
            components_.push(|v_1: &SecurityParameters| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_SecurityParameters(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.alreadySearched {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
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
/// CrossReference ::= SET {
///   contextPrefix  [0]  DistinguishedName,
///   accessPoint    [1]  AccessPointInformation,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CrossReference {
    pub contextPrefix: DistinguishedName,
    pub accessPoint: AccessPointInformation,
    pub _unrecognized: Vec<X690Element>,
}
impl CrossReference {
    pub fn new(
        contextPrefix: DistinguishedName,
        accessPoint: AccessPointInformation,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CrossReference {
            contextPrefix,
            accessPoint,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CrossReference {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CrossReference(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CrossReference {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CrossReference(el)
    }
}

pub const _rctl1_components_for_CrossReference: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "contextPrefix",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoint",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CrossReference: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CrossReference: &[ComponentSpec; 0] = &[];

pub fn _decode_CrossReference(el: &X690Element) -> ASN1Result<CrossReference> {
    |el_: &X690Element| -> ASN1Result<CrossReference> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_CrossReference,
            _eal_components_for_CrossReference,
            _rctl2_components_for_CrossReference,
            30,
        )?;
        let contextPrefix = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("contextPrefix").unwrap())?;
        let accessPoint = |el: &X690Element| -> ASN1Result<AccessPointInformation> {
            Ok(_decode_AccessPointInformation(&el.inner()?)?)
        }(_components.get("accessPoint").unwrap())?;
        Ok(CrossReference {
            contextPrefix,
            accessPoint,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CrossReference(value_: &CrossReference) -> ASN1Result<X690Element> {
    |value_: &CrossReference| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.contextPrefix)?);
        components_.push(|v_1: &AccessPointInformation| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_AccessPointInformation(&v_1)?,
                ))),
            ))
        }(&value_.accessPoint)?);
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
/// OperationProgress ::= SET {
///   nameResolutionPhase  [0]  ENUMERATED {
///     notStarted  (1),
///     proceeding  (2),
///     completed   (3),
///     ... },
///   nextRDNToBeResolved  [1]  INTEGER OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct OperationProgress {
    pub nameResolutionPhase: OperationProgress_nameResolutionPhase,
    pub nextRDNToBeResolved: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl OperationProgress {
    pub fn new(
        nameResolutionPhase: OperationProgress_nameResolutionPhase,
        nextRDNToBeResolved: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        OperationProgress {
            nameResolutionPhase,
            nextRDNToBeResolved,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for OperationProgress {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OperationProgress(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OperationProgress {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OperationProgress(el)
    }
}

pub const _rctl1_components_for_OperationProgress: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "nameResolutionPhase",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nextRDNToBeResolved",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OperationProgress: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OperationProgress: &[ComponentSpec; 0] = &[];

pub fn _decode_OperationProgress(el: &X690Element) -> ASN1Result<OperationProgress> {
    |el_: &X690Element| -> ASN1Result<OperationProgress> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_OperationProgress,
            _eal_components_for_OperationProgress,
            _rctl2_components_for_OperationProgress,
            30,
        )?;
        let nameResolutionPhase =
            |el: &X690Element| -> ASN1Result<OperationProgress_nameResolutionPhase> {
                Ok(_decode_OperationProgress_nameResolutionPhase(&el.inner()?)?)
            }(_components.get("nameResolutionPhase").unwrap())?;
        let nextRDNToBeResolved: OPTIONAL<INTEGER> = match _components.get("nextRDNToBeResolved") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(OperationProgress {
            nameResolutionPhase,
            nextRDNToBeResolved,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_OperationProgress(value_: &OperationProgress) -> ASN1Result<X690Element> {
    |value_: &OperationProgress| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(
            |v_1: &OperationProgress_nameResolutionPhase| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_OperationProgress_nameResolutionPhase(&v_1)?,
                    ))),
                ))
            }(&value_.nameResolutionPhase)?,
        );
        if let Some(v_) = &value_.nextRDNToBeResolved {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
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
/// TraceInformation  ::=  SEQUENCE OF TraceItem
/// ```
pub type TraceInformation = Vec<TraceItem>; // SequenceOfType

pub fn _decode_TraceInformation(el: &X690Element) -> ASN1Result<TraceInformation> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<TraceItem>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<TraceItem> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_TraceItem(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_TraceInformation(value_: &TraceInformation) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<TraceItem>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_TraceItem(&v)?);
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
/// TraceItem ::= SET {
///   dsa                [0]  Name,
///   targetObject       [1]  Name OPTIONAL,
///   operationProgress  [2]  OperationProgress,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TraceItem {
    pub dsa: Name,
    pub targetObject: OPTIONAL<Name>,
    pub operationProgress: OperationProgress,
    pub _unrecognized: Vec<X690Element>,
}
impl TraceItem {
    pub fn new(
        dsa: Name,
        targetObject: OPTIONAL<Name>,
        operationProgress: OperationProgress,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TraceItem {
            dsa,
            targetObject,
            operationProgress,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TraceItem {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TraceItem(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TraceItem {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TraceItem(el)
    }
}

pub const _rctl1_components_for_TraceItem: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "dsa",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "targetObject",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TraceItem: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TraceItem: &[ComponentSpec; 0] = &[];

pub fn _decode_TraceItem(el: &X690Element) -> ASN1Result<TraceItem> {
    |el_: &X690Element| -> ASN1Result<TraceItem> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_TraceItem,
            _eal_components_for_TraceItem,
            _rctl2_components_for_TraceItem,
            40,
        )?;
        let dsa = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("dsa").unwrap(),
        )?;
        let targetObject: OPTIONAL<Name> = match _components.get("targetObject") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress = |el: &X690Element| -> ASN1Result<OperationProgress> {
            Ok(_decode_OperationProgress(&el.inner()?)?)
        }(_components.get("operationProgress").unwrap())?;
        Ok(TraceItem {
            dsa,
            targetObject,
            operationProgress,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TraceItem(value_: &TraceItem) -> ASN1Result<X690Element> {
    |value_: &TraceItem| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.dsa)?);
        if let Some(v_) = &value_.targetObject {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                    &v_1,
                )?))),
            ))
        }(&value_.operationProgress)?);
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
/// ReferenceType  ::=  ENUMERATED {
///   superior               (1),
///   subordinate            (2),
///   cross                  (3),
///   nonSpecificSubordinate (4),
///   supplier               (5),
///   master                 (6),
///   immediateSuperior      (7),
///   self                   (8),
///   ditBridge              (9),
///   ... }
/// ```
pub type ReferenceType = ENUMERATED;

pub const ReferenceType_superior: ReferenceType = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_subordinate: ReferenceType = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_cross: ReferenceType = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_nonSpecificSubordinate: ReferenceType = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_supplier: ReferenceType = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_master: ReferenceType = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_immediateSuperior: ReferenceType = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_self_: ReferenceType = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ReferenceType_ditBridge: ReferenceType = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ReferenceType(el: &X690Element) -> ASN1Result<ReferenceType> {
    ber_decode_enumerated(&el)
}

pub fn _encode_ReferenceType(value_: &ReferenceType) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AccessPoint ::= SET {
///   ae-title             [0]  Name,
///   address              [1]  PresentationAddress,
///   protocolInformation  [2]  SET SIZE (1..MAX) OF ProtocolInformation OPTIONAL,
///   --                   [6]  Not to be used
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub ae_title: Name,
    pub address: PresentationAddress,
    pub protocolInformation: OPTIONAL<Vec<ProtocolInformation>>,
    pub _unrecognized: Vec<X690Element>,
}
impl AccessPoint {
    pub fn new(
        ae_title: Name,
        address: PresentationAddress,
        protocolInformation: OPTIONAL<Vec<ProtocolInformation>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AccessPoint {
            ae_title,
            address,
            protocolInformation,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AccessPoint {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AccessPoint(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AccessPoint {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AccessPoint(el)
    }
}

pub const _rctl1_components_for_AccessPoint: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "ae-title",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "address",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "protocolInformation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AccessPoint: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AccessPoint: &[ComponentSpec; 0] = &[];

pub fn _decode_AccessPoint(el: &X690Element) -> ASN1Result<AccessPoint> {
    |el_: &X690Element| -> ASN1Result<AccessPoint> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_AccessPoint,
            _eal_components_for_AccessPoint,
            _rctl2_components_for_AccessPoint,
            40,
        )?;
        let ae_title = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("ae-title").unwrap(),
        )?;
        let address = |el: &X690Element| -> ASN1Result<PresentationAddress> {
            Ok(_decode_PresentationAddress(&el.inner()?)?)
        }(_components.get("address").unwrap())?;
        let protocolInformation: OPTIONAL<Vec<ProtocolInformation>> = match _components
            .get("protocolInformation")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ProtocolInformation>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SET_OF<ProtocolInformation>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<ProtocolInformation> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_ProtocolInformation(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        Ok(AccessPoint {
            ae_title,
            address,
            protocolInformation,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AccessPoint(value_: &AccessPoint) -> ASN1Result<X690Element> {
    |value_: &AccessPoint| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.ae_title)?);
        components_.push(|v_1: &PresentationAddress| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_PresentationAddress(&v_1)?,
                ))),
            ))
        }(&value_.address)?);
        if let Some(v_) = &value_.protocolInformation {
            components_.push(
                |v_1: &Vec<ProtocolInformation>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SET_OF<ProtocolInformation>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_ProtocolInformation(&v)?);
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
/// MasterOrShadowAccessPoint ::= SET {
///   COMPONENTS OF          AccessPoint,
///   category          [3]  ENUMERATED {
///     master            (0),
///     shadow            (1),
///     writeableCopy     (2),
///     ... } DEFAULT master,
///   chainingRequired  [5]  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MasterOrShadowAccessPoint {
    pub ae_title: Name,               /* REPLICATED_COMPONENT */
    pub address: PresentationAddress, /* REPLICATED_COMPONENT */
    pub protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
    pub category: OPTIONAL<MasterOrShadowAccessPoint_category>,
    pub chainingRequired: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl MasterOrShadowAccessPoint {
    pub fn new(
        ae_title: Name,                                          /* REPLICATED_COMPONENT */
        address: PresentationAddress,                            /* REPLICATED_COMPONENT */
        protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
        category: OPTIONAL<MasterOrShadowAccessPoint_category>,
        chainingRequired: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MasterOrShadowAccessPoint {
            ae_title,
            address,
            protocolInformation,
            category,
            chainingRequired,
            _unrecognized,
        }
    }
    pub fn _default_value_for_category() -> MasterOrShadowAccessPoint_category {
        MasterOrShadowAccessPoint_category_master
    }
    pub fn _default_value_for_chainingRequired() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for MasterOrShadowAccessPoint {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MasterOrShadowAccessPoint(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MasterOrShadowAccessPoint {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MasterOrShadowAccessPoint(el)
    }
}

pub const _rctl1_components_for_MasterOrShadowAccessPoint: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "ae-title",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "address",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "protocolInformation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "category",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "chainingRequired",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MasterOrShadowAccessPoint: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MasterOrShadowAccessPoint: &[ComponentSpec; 0] = &[];

pub fn _decode_MasterOrShadowAccessPoint(
    el: &X690Element,
) -> ASN1Result<MasterOrShadowAccessPoint> {
    |el_: &X690Element| -> ASN1Result<MasterOrShadowAccessPoint> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_MasterOrShadowAccessPoint,
            _eal_components_for_MasterOrShadowAccessPoint,
            _rctl2_components_for_MasterOrShadowAccessPoint,
            60,
        )?;
        let ae_title = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("ae-title").unwrap(),
        )?;
        let address = |el: &X690Element| -> ASN1Result<PresentationAddress> {
            Ok(_decode_PresentationAddress(&el.inner()?)?)
        }(_components.get("address").unwrap())?;
        let protocolInformation: OPTIONAL<Vec<ProtocolInformation>> = match _components
            .get("protocolInformation")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ProtocolInformation>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SET_OF<ProtocolInformation>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<ProtocolInformation> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_ProtocolInformation(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let category: OPTIONAL<MasterOrShadowAccessPoint_category> =
            match _components.get("category") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<MasterOrShadowAccessPoint_category> {
                        Ok(_decode_MasterOrShadowAccessPoint_category(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let chainingRequired: OPTIONAL<BOOLEAN> = match _components.get("chainingRequired") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(MasterOrShadowAccessPoint {
            ae_title,
            address,
            protocolInformation,
            category,
            chainingRequired,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MasterOrShadowAccessPoint(
    value_: &MasterOrShadowAccessPoint,
) -> ASN1Result<X690Element> {
    |value_: &MasterOrShadowAccessPoint| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.ae_title)?);
        components_.push(|v_1: &PresentationAddress| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_PresentationAddress(&v_1)?,
                ))),
            ))
        }(&value_.address)?);
        if let Some(v_) = &value_.protocolInformation {
            components_.push(
                |v_1: &Vec<ProtocolInformation>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SET_OF<ProtocolInformation>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_ProtocolInformation(&v)?);
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
        if let Some(v_) = &value_.category {
            if *v_ != MasterOrShadowAccessPoint::_default_value_for_category() {
                components_.push(
                    |v_1: &MasterOrShadowAccessPoint_category| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            3,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_MasterOrShadowAccessPoint_category(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.chainingRequired {
            if *v_ != MasterOrShadowAccessPoint::_default_value_for_chainingRequired() {
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
/// MasterAndShadowAccessPoints  ::=  SET SIZE (1..MAX) OF MasterOrShadowAccessPoint
/// ```
pub type MasterAndShadowAccessPoints = Vec<MasterOrShadowAccessPoint>; // SetOfType

pub fn _decode_MasterAndShadowAccessPoints(
    el: &X690Element,
) -> ASN1Result<MasterAndShadowAccessPoints> {
    |el: &X690Element| -> ASN1Result<SET_OF<MasterOrShadowAccessPoint>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<MasterOrShadowAccessPoint> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_MasterOrShadowAccessPoint(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_MasterAndShadowAccessPoints(
    value_: &MasterAndShadowAccessPoints,
) -> ASN1Result<X690Element> {
    |value_: &SET_OF<MasterOrShadowAccessPoint>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_MasterOrShadowAccessPoint(&v)?);
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
/// AccessPointInformation ::= SET {
///   COMPONENTS OF          MasterOrShadowAccessPoint,
///   additionalPoints  [4]  MasterAndShadowAccessPoints OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AccessPointInformation {
    pub ae_title: Name,               /* REPLICATED_COMPONENT */
    pub address: PresentationAddress, /* REPLICATED_COMPONENT */
    pub protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
    pub category: OPTIONAL<MasterOrShadowAccessPoint_category>, /* REPLICATED_COMPONENT */
    pub chainingRequired: OPTIONAL<BOOLEAN>, /* REPLICATED_COMPONENT */
    pub additionalPoints: OPTIONAL<MasterAndShadowAccessPoints>,
    pub _unrecognized: Vec<X690Element>,
}
impl AccessPointInformation {
    pub fn new(
        ae_title: Name,                                          /* REPLICATED_COMPONENT */
        address: PresentationAddress,                            /* REPLICATED_COMPONENT */
        protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
        category: OPTIONAL<MasterOrShadowAccessPoint_category>,  /* REPLICATED_COMPONENT */
        chainingRequired: OPTIONAL<BOOLEAN>,                     /* REPLICATED_COMPONENT */
        additionalPoints: OPTIONAL<MasterAndShadowAccessPoints>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AccessPointInformation {
            ae_title,
            address,
            protocolInformation,
            category,
            chainingRequired,
            additionalPoints,
            _unrecognized,
        }
    }
    pub fn _default_value_for_category() -> MasterOrShadowAccessPoint_category {
        MasterOrShadowAccessPoint_category_master
    }
    pub fn _default_value_for_chainingRequired() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for AccessPointInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AccessPointInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AccessPointInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AccessPointInformation(el)
    }
}

pub const _rctl1_components_for_AccessPointInformation: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "ae-title",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "address",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "protocolInformation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "category",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "chainingRequired",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "additionalPoints",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AccessPointInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AccessPointInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_AccessPointInformation(el: &X690Element) -> ASN1Result<AccessPointInformation> {
    |el_: &X690Element| -> ASN1Result<AccessPointInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_AccessPointInformation,
            _eal_components_for_AccessPointInformation,
            _rctl2_components_for_AccessPointInformation,
            70,
        )?;
        let ae_title = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("ae-title").unwrap(),
        )?;
        let address = |el: &X690Element| -> ASN1Result<PresentationAddress> {
            Ok(_decode_PresentationAddress(&el.inner()?)?)
        }(_components.get("address").unwrap())?;
        let protocolInformation: OPTIONAL<Vec<ProtocolInformation>> = match _components
            .get("protocolInformation")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ProtocolInformation>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SET_OF<ProtocolInformation>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<ProtocolInformation> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_ProtocolInformation(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let category: OPTIONAL<MasterOrShadowAccessPoint_category> =
            match _components.get("category") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<MasterOrShadowAccessPoint_category> {
                        Ok(_decode_MasterOrShadowAccessPoint_category(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let chainingRequired: OPTIONAL<BOOLEAN> = match _components.get("chainingRequired") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let additionalPoints: OPTIONAL<MasterAndShadowAccessPoints> =
            match _components.get("additionalPoints") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<MasterAndShadowAccessPoints> {
                        Ok(_decode_MasterAndShadowAccessPoints(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(AccessPointInformation {
            ae_title,
            address,
            protocolInformation,
            category,
            chainingRequired,
            additionalPoints,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AccessPointInformation(value_: &AccessPointInformation) -> ASN1Result<X690Element> {
    |value_: &AccessPointInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.ae_title)?);
        components_.push(|v_1: &PresentationAddress| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_PresentationAddress(&v_1)?,
                ))),
            ))
        }(&value_.address)?);
        if let Some(v_) = &value_.protocolInformation {
            components_.push(
                |v_1: &Vec<ProtocolInformation>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SET_OF<ProtocolInformation>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_ProtocolInformation(&v)?);
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
        if let Some(v_) = &value_.category {
            if *v_ != AccessPointInformation::_default_value_for_category() {
                components_.push(
                    |v_1: &MasterOrShadowAccessPoint_category| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            TagClass::CONTEXT,
                            3,
                            Arc::new(X690Encoding::EXPLICIT(Box::new(
                                _encode_MasterOrShadowAccessPoint_category(&v_1)?,
                            ))),
                        ))
                    }(&v_)?,
                );
            }
        }
        if let Some(v_) = &value_.chainingRequired {
            if *v_ != AccessPointInformation::_default_value_for_chainingRequired() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.additionalPoints {
            components_.push(
                |v_1: &MasterAndShadowAccessPoints| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_MasterAndShadowAccessPoints(&v_1)?,
                        ))),
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
/// DitBridgeKnowledge ::= SEQUENCE {
///   domainLocalID  UnboundedDirectoryString OPTIONAL,
///   accessPoints   MasterAndShadowAccessPoints,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DitBridgeKnowledge {
    pub domainLocalID: OPTIONAL<UnboundedDirectoryString>,
    pub accessPoints: MasterAndShadowAccessPoints,
    pub _unrecognized: Vec<X690Element>,
}
impl DitBridgeKnowledge {
    pub fn new(
        domainLocalID: OPTIONAL<UnboundedDirectoryString>,
        accessPoints: MasterAndShadowAccessPoints,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DitBridgeKnowledge {
            domainLocalID,
            accessPoints,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DitBridgeKnowledge {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DitBridgeKnowledge(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DitBridgeKnowledge {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DitBridgeKnowledge(el)
    }
}

pub const _rctl1_components_for_DitBridgeKnowledge: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "domainLocalID",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoints",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DitBridgeKnowledge: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DitBridgeKnowledge: &[ComponentSpec; 0] = &[];

pub fn _decode_DitBridgeKnowledge(el: &X690Element) -> ASN1Result<DitBridgeKnowledge> {
    |el_: &X690Element| -> ASN1Result<DitBridgeKnowledge> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DitBridgeKnowledge,
            _eal_components_for_DitBridgeKnowledge,
            _rctl2_components_for_DitBridgeKnowledge,
        )?;
        let domainLocalID: OPTIONAL<UnboundedDirectoryString> =
            match _components.get("domainLocalID") {
                Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
                _ => None,
            };
        let accessPoints =
            _decode_MasterAndShadowAccessPoints(_components.get("accessPoints").unwrap())?;
        Ok(DitBridgeKnowledge {
            domainLocalID,
            accessPoints,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DitBridgeKnowledge(value_: &DitBridgeKnowledge) -> ASN1Result<X690Element> {
    |value_: &DitBridgeKnowledge| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.domainLocalID {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        components_.push(_encode_MasterAndShadowAccessPoints(&value_.accessPoints)?);
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
/// Exclusions  ::=  SET SIZE (1..MAX) OF RDNSequence
/// ```
pub type Exclusions = Vec<RDNSequence>; // SetOfType

pub fn _decode_Exclusions(el: &X690Element) -> ASN1Result<Exclusions> {
    |el: &X690Element| -> ASN1Result<SET_OF<RDNSequence>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<RDNSequence> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_RDNSequence(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Exclusions(value_: &Exclusions) -> ASN1Result<X690Element> {
    |value_: &SET_OF<RDNSequence>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_RDNSequence(&v)?);
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
/// ContinuationReference ::= SET {
///   targetObject         [0]  Name,
///   aliasedRDNs          [1]  INTEGER OPTIONAL, -- only present in first edition systems
///   operationProgress    [2]  OperationProgress,
///   rdnsResolved         [3]  INTEGER OPTIONAL,
///   referenceType        [4]  ReferenceType,
///   accessPoints         [5]  SET OF AccessPointInformation,
///   entryOnly            [6]  BOOLEAN DEFAULT FALSE,
///   exclusions           [7]  Exclusions OPTIONAL,
///   returnToDUA          [8]  BOOLEAN DEFAULT FALSE,
///   nameResolveOnMaster  [9]  BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ContinuationReference {
    pub targetObject: Name,
    pub aliasedRDNs: OPTIONAL<INTEGER>,
    pub operationProgress: OperationProgress,
    pub rdnsResolved: OPTIONAL<INTEGER>,
    pub referenceType: ReferenceType,
    pub accessPoints: Vec<AccessPointInformation>,
    pub entryOnly: OPTIONAL<BOOLEAN>,
    pub exclusions: OPTIONAL<Exclusions>,
    pub returnToDUA: OPTIONAL<BOOLEAN>,
    pub nameResolveOnMaster: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl ContinuationReference {
    pub fn new(
        targetObject: Name,
        aliasedRDNs: OPTIONAL<INTEGER>,
        operationProgress: OperationProgress,
        rdnsResolved: OPTIONAL<INTEGER>,
        referenceType: ReferenceType,
        accessPoints: Vec<AccessPointInformation>,
        entryOnly: OPTIONAL<BOOLEAN>,
        exclusions: OPTIONAL<Exclusions>,
        returnToDUA: OPTIONAL<BOOLEAN>,
        nameResolveOnMaster: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ContinuationReference {
            targetObject,
            aliasedRDNs,
            operationProgress,
            rdnsResolved,
            referenceType,
            accessPoints,
            entryOnly,
            exclusions,
            returnToDUA,
            nameResolveOnMaster,
            _unrecognized,
        }
    }
    pub fn _default_value_for_entryOnly() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_returnToDUA() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_nameResolveOnMaster() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ContinuationReference {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContinuationReference(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContinuationReference {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContinuationReference(el)
    }
}

pub const _rctl1_components_for_ContinuationReference: &[ComponentSpec; 10] = &[
    ComponentSpec::new(
        "targetObject",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aliasedRDNs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "operationProgress",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "rdnsResolved",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referenceType",
        false,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoints",
        false,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryOnly",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "exclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "returnToDUA",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameResolveOnMaster",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContinuationReference: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContinuationReference: &[ComponentSpec; 0] = &[];

pub fn _decode_ContinuationReference(el: &X690Element) -> ASN1Result<ContinuationReference> {
    |el_: &X690Element| -> ASN1Result<ContinuationReference> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_ContinuationReference,
            _eal_components_for_ContinuationReference,
            _rctl2_components_for_ContinuationReference,
            110,
        )?;
        let targetObject =
            |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
                _components.get("targetObject").unwrap(),
            )?;
        let aliasedRDNs: OPTIONAL<INTEGER> = match _components.get("aliasedRDNs") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let operationProgress = |el: &X690Element| -> ASN1Result<OperationProgress> {
            Ok(_decode_OperationProgress(&el.inner()?)?)
        }(_components.get("operationProgress").unwrap())?;
        let rdnsResolved: OPTIONAL<INTEGER> = match _components.get("rdnsResolved") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let referenceType = |el: &X690Element| -> ASN1Result<ReferenceType> {
            Ok(_decode_ReferenceType(&el.inner()?)?)
        }(_components.get("referenceType").unwrap())?;
        let accessPoints = |el: &X690Element| -> ASN1Result<Vec<AccessPointInformation>> {
            Ok(
                |el: &X690Element| -> ASN1Result<SET_OF<AccessPointInformation>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<AccessPointInformation> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_AccessPointInformation(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(_components.get("accessPoints").unwrap())?;
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
        let returnToDUA: OPTIONAL<BOOLEAN> = match _components.get("returnToDUA") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameResolveOnMaster: OPTIONAL<BOOLEAN> = match _components.get("nameResolveOnMaster") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ContinuationReference {
            targetObject,
            aliasedRDNs,
            operationProgress,
            rdnsResolved,
            referenceType,
            accessPoints,
            entryOnly,
            exclusions,
            returnToDUA,
            nameResolveOnMaster,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ContinuationReference(value_: &ContinuationReference) -> ASN1Result<X690Element> {
    |value_: &ContinuationReference| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(20);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.targetObject)?);
        if let Some(v_) = &value_.aliasedRDNs {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &OperationProgress| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OperationProgress(
                    &v_1,
                )?))),
            ))
        }(&value_.operationProgress)?);
        if let Some(v_) = &value_.rdnsResolved {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &ReferenceType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                4,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ReferenceType(
                    &v_1,
                )?))),
            ))
        }(&value_.referenceType)?);
        components_.push(
            |v_1: &Vec<AccessPointInformation>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        AccessPointInformation,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AccessPointInformation(&v)?);
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
            }(&value_.accessPoints)?,
        );
        if let Some(v_) = &value_.entryOnly {
            if *v_ != ContinuationReference::_default_value_for_entryOnly() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        6,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.exclusions {
            components_.push(|v_1: &Exclusions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Exclusions(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.returnToDUA {
            if *v_ != ContinuationReference::_default_value_for_returnToDUA() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        8,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.nameResolveOnMaster {
            if *v_ != ContinuationReference::_default_value_for_nameResolveOnMaster() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        9,
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
/// dSABind OPERATION ::= {
///   ARGUMENT     DSABindArgument
///   RESULT       DSABindResult
///   ERRORS       { directoryBindError } }
/// ```
///
///
pub fn dSABind() -> OPERATION {
    OPERATION {
        Errors: Some(Vec::<_>::from([directoryBindError()])), /* OBJECT_FIELD_SETTING */
        ..Default::default()
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSABindArgument ::= SET  {
///   credentials  [0]  DSACredentials OPTIONAL,
///   versions     [1]  Versions DEFAULT {v1},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DSABindArgument {
    pub credentials: OPTIONAL<DSACredentials>,
    pub versions: OPTIONAL<Versions>,
    pub _unrecognized: Vec<X690Element>,
}
impl DSABindArgument {
    pub fn new(
        credentials: OPTIONAL<DSACredentials>,
        versions: OPTIONAL<Versions>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DSABindArgument {
            credentials,
            versions,
            _unrecognized,
        }
    }
    pub fn _default_value_for_versions() -> Versions {
        BIT_STRING::with_bits_set(&[Versions_v1])
    }
}
impl Default for DSABindArgument {
    fn default() -> Self {
        DSABindArgument {
            credentials: None,
            versions: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for DSABindArgument {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DSABindArgument(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DSABindArgument {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DSABindArgument(el)
    }
}

pub const _rctl1_components_for_DSABindArgument: &[ComponentSpec; 2] = &[
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

pub const _rctl2_components_for_DSABindArgument: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DSABindArgument: &[ComponentSpec; 0] = &[];

pub fn _decode_DSABindArgument(el: &X690Element) -> ASN1Result<DSABindArgument> {
    |el_: &X690Element| -> ASN1Result<DSABindArgument> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_DSABindArgument,
            _eal_components_for_DSABindArgument,
            _rctl2_components_for_DSABindArgument,
            30,
        )?;
        let credentials: OPTIONAL<DSACredentials> = match _components.get("credentials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DSACredentials> {
                Ok(_decode_DSACredentials(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let versions: OPTIONAL<Versions> = match _components.get("versions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Versions> {
                Ok(_decode_Versions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(DSABindArgument {
            credentials,
            versions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DSABindArgument(value_: &DSABindArgument) -> ASN1Result<X690Element> {
    |value_: &DSABindArgument| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.credentials {
            components_.push(|v_1: &DSACredentials| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DSACredentials(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.versions {
            if *v_ != DSABindArgument::_default_value_for_versions() {
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
/// DSACredentials   ::=   CHOICE  {
///   simple             [0]  SimpleCredentials,
///   strong             [1]  StrongCredentials,
///   externalProcedure  [2]  EXTERNAL,
///   spkm               [3]  SpkmCredentials,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum DSACredentials {
    simple(SimpleCredentials),
    strong(StrongCredentials),
    externalProcedure(EXTERNAL),
    spkm(SpkmCredentials),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for DSACredentials {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DSACredentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DSACredentials {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DSACredentials(el)
    }
}

pub fn _decode_DSACredentials(el: &X690Element) -> ASN1Result<DSACredentials> {
    |el: &X690Element| -> ASN1Result<DSACredentials> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(DSACredentials::simple(_decode_SimpleCredentials(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(DSACredentials::strong(_decode_StrongCredentials(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 2) => Ok(DSACredentials::externalProcedure(ber_decode_external(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 3) => Ok(DSACredentials::spkm(|el: &X690Element| -> ASN1Result<
                SpkmCredentials,
            > {
                Ok(_decode_SpkmCredentials(&el.inner()?)?)
            }(&el)?)),
            _ => Ok(DSACredentials::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_DSACredentials(value_: &DSACredentials) -> ASN1Result<X690Element> {
    |value: &DSACredentials| -> ASN1Result<X690Element> {
        match value {
            DSACredentials::simple(v) => |v_1: &SimpleCredentials| -> ASN1Result<X690Element> {
                let el_1 = _encode_SimpleCredentials(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            DSACredentials::strong(v) => |v_1: &StrongCredentials| -> ASN1Result<X690Element> {
                let el_1 = _encode_StrongCredentials(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            DSACredentials::externalProcedure(v) => |v_1: &EXTERNAL| -> ASN1Result<X690Element> {
                let el_1 = ber_encode_external(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            DSACredentials::spkm(v) => |v_1: &SpkmCredentials| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_SpkmCredentials(
                        &v_1,
                    )?))),
                ))
            }(&v),
            DSACredentials::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSABindResult   ::=   DSABindArgument
/// ```
pub type DSABindResult = DSABindArgument; // DefinedType

pub fn _decode_DSABindResult(el: &X690Element) -> ASN1Result<DSABindResult> {
    _decode_DSABindArgument(&el)
}

pub fn _encode_DSABindResult(value_: &DSABindResult) -> ASN1Result<X690Element> {
    _encode_DSABindArgument(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Chained-ArgumentType-OPTIONALLY-PROTECTED-Parameter1 {OPERATION:operation} ::= SET {
///     chainedArgument      ChainingArguments,
///     argument        [0]  operation.&ArgumentType }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1 {
    pub chainedArgument: ChainingArguments,
    pub argument: X690Element,
}
impl Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1 {
    pub fn new(chainedArgument: ChainingArguments, argument: X690Element) -> Self {
        Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1 {
            chainedArgument,
            argument,
        }
    }
}
impl TryFrom<X690Element> for Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1 {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1 {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1(el)
    }
}

pub const _rctl1_components_for_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     2] = &[
    ComponentSpec::new(
        "chainedArgument",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "argument",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     0] = &[];

pub const _eal_components_for_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     0] = &[];

pub fn _decode_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1(
    el: &X690Element,
) -> ASN1Result<Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1> {
    |el_: &X690Element| -> ASN1Result<Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1,
            _eal_components_for_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1,
            _rctl2_components_for_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1,
            20,
        )?;
        let chainedArgument =
            _decode_ChainingArguments(_components.get("chainedArgument").unwrap())?;
        let argument =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("argument").unwrap(),
            )?;
        Ok(Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1 {
            chainedArgument,
            argument,
        })
    }(&el)
}

pub fn _encode_Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1(
    value_: &Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1,
) -> ASN1Result<X690Element> {
    |value_: &Chained_ArgumentType_OPTIONALLY_PROTECTED_Parameter1| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_ChainingArguments(&value_.chainedArgument)?);
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.argument)?);
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
/// Chained-ResultType-OPTIONALLY-PROTECTED-Parameter1 {OPERATION:operation} ::= SET {
///     chainedResult        ChainingResults,
///     result          [0]  operation.&ResultType }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1 {
    pub chainedResult: ChainingResults,
    pub result: X690Element,
}
impl Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1 {
    pub fn new(chainedResult: ChainingResults, result: X690Element) -> Self {
        Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1 {
            chainedResult,
            result,
        }
    }
}
impl TryFrom<X690Element> for Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1 {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1 {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1(el)
    }
}

pub const _rctl1_components_for_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     2] = &[
    ComponentSpec::new(
        "chainedResult",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "result",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     0] = &[];

pub const _eal_components_for_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1: &[ComponentSpec;
     0] = &[];

pub fn _decode_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1(
    el: &X690Element,
) -> ASN1Result<Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1> {
    |el_: &X690Element| -> ASN1Result<Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1,
            _eal_components_for_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1,
            _rctl2_components_for_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1,
            20,
        )?;
        let chainedResult = _decode_ChainingResults(_components.get("chainedResult").unwrap())?;
        let result =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("result").unwrap(),
            )?;
        Ok(Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1 {
            chainedResult,
            result,
        })
    }(&el)
}

pub fn _encode_Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1(
    value_: &Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1,
) -> ASN1Result<X690Element> {
    |value_: &Chained_ResultType_OPTIONALLY_PROTECTED_Parameter1| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_ChainingResults(&value_.chainedResult)?);
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.result)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition
///
/// ```
/// chained{OPERATION:operation} OPERATION ::= {
///     ARGUMENT    OPTIONALLY-PROTECTED {Chained-ArgumentType-OPTIONALLY-PROTECTED-Parameter1}
///     RESULT      OPTIONALLY-PROTECTED {Chained-ResultType-OPTIONALLY-PROTECTED-Parameter1}
///     ERRORS      {operation.&Errors EXCEPT referral | dsaReferral}
///     CODE        operation.&operationCode }
/// ```
fn chained(operation: OPERATION) -> OPERATION {
    OPERATION {
        Errors: operation.Errors,
        operationCode: operation.operationCode,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedRead               OPERATION ::= chained{read}
/// ```
///
///
pub fn chainedRead() -> OPERATION {
    chained(read())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedCompare            OPERATION ::= chained{compare}
/// ```
///
///
pub fn chainedCompare() -> OPERATION {
    chained(compare())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedAbandon            OPERATION ::= abandon
/// ```
///
///
pub fn chainedAbandon() -> OPERATION {
    abandon()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedList               OPERATION ::= chained{list}
/// ```
///
///
pub fn chainedList() -> OPERATION {
    chained(list())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedSearch             OPERATION ::= chained{search}
/// ```
///
///
pub fn chainedSearch() -> OPERATION {
    chained(search())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedAddEntry           OPERATION ::= chained{addEntry}
/// ```
///
///
pub fn chainedAddEntry() -> OPERATION {
    chained(addEntry())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedRemoveEntry        OPERATION ::= chained{removeEntry}
/// ```
///
///
pub fn chainedRemoveEntry() -> OPERATION {
    chained(removeEntry())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedModifyEntry        OPERATION ::= chained{modifyEntry}
/// ```
///
///
pub fn chainedModifyEntry() -> OPERATION {
    chained(modifyEntry())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedModifyDN           OPERATION ::= chained{modifyDN}
/// ```
///
///
pub fn chainedModifyDN() -> OPERATION {
    chained(modifyDN())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedChangePassword     OPERATION ::= chained{changePassword}
/// ```
///
///
pub fn chainedChangePassword() -> OPERATION {
    chained(changePassword())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedAdministerPassword OPERATION ::= chained{administerPassword}
/// ```
///
///
pub fn chainedAdministerPassword() -> OPERATION {
    chained(administerPassword())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedLdapTransport      OPERATION ::= chained{ldapTransport}
/// ```
///
///
pub fn chainedLdapTransport() -> OPERATION {
    chained(ldapTransport())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// chainedLinkedLDAP         OPERATION ::= chained{linkedLDAP}
/// ```
///
///
pub fn chainedLinkedLDAP() -> OPERATION {
    chained(linkedLDAP())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OperationProgress-nameResolutionPhase ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type OperationProgress_nameResolutionPhase = ENUMERATED;

pub const OperationProgress_nameResolutionPhase_notStarted: OperationProgress_nameResolutionPhase =
    1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OperationProgress_nameResolutionPhase_proceeding: OperationProgress_nameResolutionPhase =
    2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OperationProgress_nameResolutionPhase_completed: OperationProgress_nameResolutionPhase =
    3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_OperationProgress_nameResolutionPhase(
    el: &X690Element,
) -> ASN1Result<OperationProgress_nameResolutionPhase> {
    ber_decode_enumerated(&el)
}

pub fn _encode_OperationProgress_nameResolutionPhase(
    value_: &OperationProgress_nameResolutionPhase,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MasterOrShadowAccessPoint-category ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type MasterOrShadowAccessPoint_category = ENUMERATED;

pub const MasterOrShadowAccessPoint_category_master: MasterOrShadowAccessPoint_category = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const MasterOrShadowAccessPoint_category_shadow: MasterOrShadowAccessPoint_category = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const MasterOrShadowAccessPoint_category_writeableCopy: MasterOrShadowAccessPoint_category = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_MasterOrShadowAccessPoint_category(
    el: &X690Element,
) -> ASN1Result<MasterOrShadowAccessPoint_category> {
    ber_decode_enumerated(&el)
}

pub fn _encode_MasterOrShadowAccessPoint_category(
    value_: &MasterOrShadowAccessPoint_category,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}
