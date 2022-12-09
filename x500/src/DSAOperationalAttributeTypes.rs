#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DSAOperationalAttributeTypes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DSAOperationalAttributeTypes`.
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
use crate::DistributedOperations::*;
use crate::InformationFramework::*;
use crate::OperationalBindingManagement::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// dseType ATTRIBUTE ::= {
///   WITH SYNTAX             DSEType
///   EQUALITY MATCHING RULE  bitStringMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-dseType }
/// ```
///
///
pub fn dseType() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(bitStringMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                        /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                 /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),         /* OBJECT_FIELD_SETTING */
        id: id_doa_dseType(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSEType  ::=  BIT STRING {
///   root           (0),  -- root DSE
///   glue           (1),  -- represents knowledge of a name only
///   cp             (2),  -- context prefix
///   entry          (3),  -- object entry
///   alias          (4),  -- alias entry
///   subr           (5),  -- subordinate reference
///   nssr           (6),  -- non-specific subordinate reference
///   supr           (7),  -- superior reference
///   xr             (8),  -- cross reference
///   admPoint       (9),  -- administrative point
///   subentry       (10), -- subentry
///   shadow         (11), -- shadow copy
///   immSupr        (13), -- immediate superior reference
///   rhob           (14), -- rhob information
///   sa             (15), -- subordinate reference to alias entry
///   dsSubentry     (16), -- DSA Specific subentry
///   familyMember   (17), -- family member
///   ditBridge      (18)}
/// ```
pub type DSEType = BIT_STRING;

pub const DSEType_root: BIT = 0; /* LONG_NAMED_BIT */

pub const DSEType_glue: BIT = 1; /* LONG_NAMED_BIT */

pub const DSEType_cp: BIT = 2; /* LONG_NAMED_BIT */

pub const DSEType_entry: BIT = 3; /* LONG_NAMED_BIT */

pub const DSEType_alias: BIT = 4; /* LONG_NAMED_BIT */

pub const DSEType_subr: BIT = 5; /* LONG_NAMED_BIT */

pub const DSEType_nssr: BIT = 6; /* LONG_NAMED_BIT */

pub const DSEType_supr: BIT = 7; /* LONG_NAMED_BIT */

pub const DSEType_xr: BIT = 8; /* LONG_NAMED_BIT */

pub const DSEType_admPoint: BIT = 9; /* LONG_NAMED_BIT */

pub const DSEType_subentry: BIT = 10; /* LONG_NAMED_BIT */

pub const DSEType_shadow: BIT = 11; /* LONG_NAMED_BIT */

pub const DSEType_immSupr: BIT = 13; /* LONG_NAMED_BIT */

pub const DSEType_rhob: BIT = 14; /* LONG_NAMED_BIT */

pub const DSEType_sa: BIT = 15; /* LONG_NAMED_BIT */

pub const DSEType_dsSubentry: BIT = 16; /* LONG_NAMED_BIT */

pub const DSEType_familyMember: BIT = 17; /* LONG_NAMED_BIT */

pub const DSEType_ditBridge: BIT = 18; /* LONG_NAMED_BIT */

pub fn _decode_DSEType(el: &X690Element) -> ASN1Result<DSEType> {
    ber_decode_bit_string(&el)
}

pub fn _encode_DSEType(value_: &DSEType) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// myAccessPoint ATTRIBUTE ::= {
///   WITH SYNTAX             AccessPoint
///   EQUALITY MATCHING RULE  accessPointMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-myAccessPoint }
/// ```
///
///
pub fn myAccessPoint() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(accessPointMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                          /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                   /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),           /* OBJECT_FIELD_SETTING */
        id: id_doa_myAccessPoint(),                         /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// superiorKnowledge ATTRIBUTE ::= {
///   WITH SYNTAX             AccessPoint
///   EQUALITY MATCHING RULE  accessPointMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-superiorKnowledge }
/// ```
///
///
pub fn superiorKnowledge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(accessPointMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                   /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),           /* OBJECT_FIELD_SETTING */
        id: id_doa_superiorKnowledge(),                     /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// specificKnowledge ATTRIBUTE ::= {
///   WITH SYNTAX             MasterAndShadowAccessPoints
///   EQUALITY MATCHING RULE  masterAndShadowAccessPointsMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   distributedOperation
///   ID                      id-doa-specificKnowledge }
/// ```
///
///
pub fn specificKnowledge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(masterAndShadowAccessPointsMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),        /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_distributedOperation), /* OBJECT_FIELD_SETTING */
        id: id_doa_specificKnowledge(),   /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nonSpecificKnowledge ATTRIBUTE ::= {
///   WITH SYNTAX             MasterAndShadowAccessPoints
///   EQUALITY MATCHING RULE  masterAndShadowAccessPointsMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   distributedOperation
///   ID                      id-doa-nonSpecificKnowledge }
/// ```
///
///
pub fn nonSpecificKnowledge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(masterAndShadowAccessPointsMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_distributedOperation), /* OBJECT_FIELD_SETTING */
        id: id_doa_nonSpecificKnowledge(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupplierOrConsumer ::= SET {
///   COMPONENTS OF              AccessPoint, -- supplier or consumer
///   agreementID           [3]  OperationalBindingID,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SupplierOrConsumer {
    pub ae_title: Name,               /* REPLICATED_COMPONENT */
    pub address: PresentationAddress, /* REPLICATED_COMPONENT */
    pub protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
    pub agreementID: OperationalBindingID,
    pub _unrecognized: Vec<X690Element>,
}
impl SupplierOrConsumer {
    pub fn new(
        ae_title: Name,                                          /* REPLICATED_COMPONENT */
        address: PresentationAddress,                            /* REPLICATED_COMPONENT */
        protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
        agreementID: OperationalBindingID,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SupplierOrConsumer {
            ae_title,
            address,
            protocolInformation,
            agreementID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SupplierOrConsumer {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierOrConsumer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SupplierOrConsumer {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierOrConsumer(el)
    }
}

pub const _rctl1_components_for_SupplierOrConsumer: &[ComponentSpec; 4] = &[
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
        "agreementID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SupplierOrConsumer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SupplierOrConsumer: &[ComponentSpec; 0] = &[];

pub fn _decode_SupplierOrConsumer(el: &X690Element) -> ASN1Result<SupplierOrConsumer> {
    |el_: &X690Element| -> ASN1Result<SupplierOrConsumer> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SupplierOrConsumer,
            _eal_components_for_SupplierOrConsumer,
            _rctl2_components_for_SupplierOrConsumer,
            50,
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
        let agreementID = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
            Ok(_decode_OperationalBindingID(&el.inner()?)?)
        }(_components.get("agreementID").unwrap())?;
        Ok(SupplierOrConsumer {
            ae_title,
            address,
            protocolInformation,
            agreementID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SupplierOrConsumer(value_: &SupplierOrConsumer) -> ASN1Result<X690Element> {
    |value_: &SupplierOrConsumer| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
        components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_OperationalBindingID(&v_1)?,
                ))),
            ))
        }(&value_.agreementID)?);
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
/// SupplierInformation ::= SET {
///   COMPONENTS OF              SupplierOrConsumer, -- supplier
///   supplier-is-master    [4]  BOOLEAN DEFAULT TRUE,
///   non-supplying-master  [5]  AccessPoint OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SupplierInformation {
    pub ae_title: Name,               /* REPLICATED_COMPONENT */
    pub address: PresentationAddress, /* REPLICATED_COMPONENT */
    pub protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
    pub agreementID: OperationalBindingID, /* REPLICATED_COMPONENT */
    pub supplier_is_master: OPTIONAL<BOOLEAN>,
    pub non_supplying_master: OPTIONAL<AccessPoint>,
    pub _unrecognized: Vec<X690Element>,
}
impl SupplierInformation {
    pub fn new(
        ae_title: Name,                                          /* REPLICATED_COMPONENT */
        address: PresentationAddress,                            /* REPLICATED_COMPONENT */
        protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
        agreementID: OperationalBindingID,                       /* REPLICATED_COMPONENT */
        supplier_is_master: OPTIONAL<BOOLEAN>,
        non_supplying_master: OPTIONAL<AccessPoint>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SupplierInformation {
            ae_title,
            address,
            protocolInformation,
            agreementID,
            supplier_is_master,
            non_supplying_master,
            _unrecognized,
        }
    }
    pub fn _default_value_for_supplier_is_master() -> BOOLEAN {
        true
    }
}
impl TryFrom<X690Element> for SupplierInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SupplierInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierInformation(el)
    }
}

pub const _rctl1_components_for_SupplierInformation: &[ComponentSpec; 6] = &[
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
        "agreementID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "supplier-is-master",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "non-supplying-master",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SupplierInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SupplierInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_SupplierInformation(el: &X690Element) -> ASN1Result<SupplierInformation> {
    |el_: &X690Element| -> ASN1Result<SupplierInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SupplierInformation,
            _eal_components_for_SupplierInformation,
            _rctl2_components_for_SupplierInformation,
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
        let agreementID = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
            Ok(_decode_OperationalBindingID(&el.inner()?)?)
        }(_components.get("agreementID").unwrap())?;
        let supplier_is_master: OPTIONAL<BOOLEAN> = match _components.get("supplier-is-master") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let non_supplying_master: OPTIONAL<AccessPoint> =
            match _components.get("non-supplying-master") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<AccessPoint> {
                    Ok(_decode_AccessPoint(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(SupplierInformation {
            ae_title,
            address,
            protocolInformation,
            agreementID,
            supplier_is_master,
            non_supplying_master,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SupplierInformation(value_: &SupplierInformation) -> ASN1Result<X690Element> {
    |value_: &SupplierInformation| -> ASN1Result<X690Element> {
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
        components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_OperationalBindingID(&v_1)?,
                ))),
            ))
        }(&value_.agreementID)?);
        if let Some(v_) = &value_.supplier_is_master {
            if *v_ != SupplierInformation::_default_value_for_supplier_is_master() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.non_supplying_master {
            components_.push(|v_1: &AccessPoint| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AccessPoint(&v_1)?))),
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
/// supplierKnowledge ATTRIBUTE ::= {
///   WITH SYNTAX             SupplierInformation
///   EQUALITY MATCHING RULE  supplierOrConsumerInformationMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-supplierKnowledge }
/// ```
///
///
pub fn supplierKnowledge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(supplierOrConsumerInformationMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        id: id_doa_supplierKnowledge(),   /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ConsumerInformation  ::=  SupplierOrConsumer
/// ```
pub type ConsumerInformation = SupplierOrConsumer; // DefinedType

pub fn _decode_ConsumerInformation(el: &X690Element) -> ASN1Result<ConsumerInformation> {
    _decode_SupplierOrConsumer(&el)
}

pub fn _encode_ConsumerInformation(value_: &ConsumerInformation) -> ASN1Result<X690Element> {
    _encode_SupplierOrConsumer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// consumerKnowledge ATTRIBUTE ::= {
///   WITH SYNTAX             ConsumerInformation
///   EQUALITY MATCHING RULE  supplierOrConsumerInformationMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-consumerKnowledge }
/// ```
///
///
pub fn consumerKnowledge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(supplierOrConsumerInformationMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        id: id_doa_consumerKnowledge(),   /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupplierAndConsumers ::= SET {
///   COMPONENTS OF         AccessPoint, -- supplier
///   consumers        [3]  SET OF AccessPoint,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SupplierAndConsumers {
    pub ae_title: Name,               /* REPLICATED_COMPONENT */
    pub address: PresentationAddress, /* REPLICATED_COMPONENT */
    pub protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
    pub consumers: Vec<AccessPoint>,
    pub _unrecognized: Vec<X690Element>,
}
impl SupplierAndConsumers {
    pub fn new(
        ae_title: Name,                                          /* REPLICATED_COMPONENT */
        address: PresentationAddress,                            /* REPLICATED_COMPONENT */
        protocolInformation: OPTIONAL<Vec<ProtocolInformation>>, /* REPLICATED_COMPONENT */
        consumers: Vec<AccessPoint>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SupplierAndConsumers {
            ae_title,
            address,
            protocolInformation,
            consumers,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SupplierAndConsumers {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierAndConsumers(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SupplierAndConsumers {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SupplierAndConsumers(el)
    }
}

pub const _rctl1_components_for_SupplierAndConsumers: &[ComponentSpec; 4] = &[
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
        "consumers",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SupplierAndConsumers: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SupplierAndConsumers: &[ComponentSpec; 0] = &[];

pub fn _decode_SupplierAndConsumers(el: &X690Element) -> ASN1Result<SupplierAndConsumers> {
    |el_: &X690Element| -> ASN1Result<SupplierAndConsumers> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SupplierAndConsumers,
            _eal_components_for_SupplierAndConsumers,
            _rctl2_components_for_SupplierAndConsumers,
            50,
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
        let consumers = |el: &X690Element| -> ASN1Result<Vec<AccessPoint>> {
            Ok(|el: &X690Element| -> ASN1Result<SET_OF<AccessPoint>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<AccessPoint> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_AccessPoint(el)?);
                }
                Ok(items)
            }(&el.inner()?)?)
        }(_components.get("consumers").unwrap())?;
        Ok(SupplierAndConsumers {
            ae_title,
            address,
            protocolInformation,
            consumers,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SupplierAndConsumers(value_: &SupplierAndConsumers) -> ASN1Result<X690Element> {
    |value_: &SupplierAndConsumers| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
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
        components_.push(|v_1: &Vec<AccessPoint>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                    AccessPoint,
                >|
                 -> ASN1Result<
                    X690Element,
                > {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_AccessPoint(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.consumers)?);
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
/// secondaryShadows ATTRIBUTE ::= {
///   WITH SYNTAX             SupplierAndConsumers
///   EQUALITY MATCHING RULE  supplierAndConsumersMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-secondaryShadows }
/// ```
///
///
pub fn secondaryShadows() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(supplierAndConsumersMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                            /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),                    /* OBJECT_FIELD_SETTING */
        id: id_doa_secondaryShadows(),                               /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ditBridgeKnowledge ATTRIBUTE ::= {
///   WITH SYNTAX             DitBridgeKnowledge
///   EQUALITY MATCHING RULE  directoryStringFirstComponentMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   dSAOperation
///   ID                      id-doa-ditBridgeKnowledge }
/// ```
///
///
pub fn ditBridgeKnowledge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(directoryStringFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        id: id_doa_ditBridgeKnowledge(),  /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessPointMatch MATCHING-RULE ::= {
///   SYNTAX  Name
///   ID      id-kmr-accessPointMatch }
/// ```
///
///
pub fn accessPointMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_kmr_accessPointMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// masterAndShadowAccessPointsMatch MATCHING-RULE ::= {
///   SYNTAX  SET OF Name
///   ID      id-kmr-masterShadowMatch }
/// ```
///
///
pub fn masterAndShadowAccessPointsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_kmr_masterShadowMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// supplierOrConsumerInformationMatch MATCHING-RULE ::= {
///   SYNTAX SET {
///     ae-title              [0]  Name,
///     agreement-identifier  [2]  INTEGER}
///   ID      id-kmr-supplierConsumerMatch }
/// ```
///
///
pub fn supplierOrConsumerInformationMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_kmr_supplierConsumerMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// supplierAndConsumersMatch MATCHING-RULE ::= {
///   SYNTAX  Name
///   ID      id-kmr-supplierConsumersMatch }
/// ```
///
///
pub fn supplierAndConsumersMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_kmr_supplierConsumersMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-dseType                  OBJECT IDENTIFIER ::= {id-doa 0}
/// ```
///
///
pub fn id_doa_dseType() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-myAccessPoint            OBJECT IDENTIFIER ::= {id-doa 1}
/// ```
///
///
pub fn id_doa_myAccessPoint() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-superiorKnowledge        OBJECT IDENTIFIER ::= {id-doa 2}
/// ```
///
///
pub fn id_doa_superiorKnowledge() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-specificKnowledge        OBJECT IDENTIFIER ::= {id-doa 3}
/// ```
///
///
pub fn id_doa_specificKnowledge() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-nonSpecificKnowledge     OBJECT IDENTIFIER ::= {id-doa 4}
/// ```
///
///
pub fn id_doa_nonSpecificKnowledge() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-supplierKnowledge        OBJECT IDENTIFIER ::= {id-doa 5}
/// ```
///
///
pub fn id_doa_supplierKnowledge() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-consumerKnowledge        OBJECT IDENTIFIER ::= {id-doa 6}
/// ```
///
///
pub fn id_doa_consumerKnowledge() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-secondaryShadows         OBJECT IDENTIFIER ::= {id-doa 7}
/// ```
///
///
pub fn id_doa_secondaryShadows() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-ditBridgeKnowledge       OBJECT IDENTIFIER ::= {id-doa 8}
/// ```
///
///
pub fn id_doa_ditBridgeKnowledge() -> OBJECT_IDENTIFIER {
    [id_doa(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-accessPointMatch         OBJECT IDENTIFIER ::= {id-kmr 0}
/// ```
///
///
pub fn id_kmr_accessPointMatch() -> OBJECT_IDENTIFIER {
    [id_kmr(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-masterShadowMatch        OBJECT IDENTIFIER ::= {id-kmr 1}
/// ```
///
///
pub fn id_kmr_masterShadowMatch() -> OBJECT_IDENTIFIER {
    [id_kmr(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-supplierConsumerMatch    OBJECT IDENTIFIER ::= {id-kmr 2}
/// ```
///
///
pub fn id_kmr_supplierConsumerMatch() -> OBJECT_IDENTIFIER {
    [id_kmr(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-supplierConsumersMatch   OBJECT IDENTIFIER ::= {id-kmr 3}
/// ```
///
///
pub fn id_kmr_supplierConsumersMatch() -> OBJECT_IDENTIFIER {
    [id_kmr(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// supplierOrConsumerInformationMatch-AssertionType ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct supplierOrConsumerInformationMatch_AssertionType {
    pub ae_title: Name,
    pub agreement_identifier: INTEGER,
}
impl supplierOrConsumerInformationMatch_AssertionType {
    pub fn new(ae_title: Name, agreement_identifier: INTEGER) -> Self {
        supplierOrConsumerInformationMatch_AssertionType {
            ae_title,
            agreement_identifier,
        }
    }
}
impl TryFrom<X690Element> for supplierOrConsumerInformationMatch_AssertionType {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_supplierOrConsumerInformationMatch_AssertionType(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for supplierOrConsumerInformationMatch_AssertionType {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_supplierOrConsumerInformationMatch_AssertionType(el)
    }
}

pub const _rctl1_components_for_supplierOrConsumerInformationMatch_AssertionType: &[ComponentSpec;
     2] = &[
    ComponentSpec::new(
        "ae-title",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "agreement-identifier",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_supplierOrConsumerInformationMatch_AssertionType: &[ComponentSpec;
     0] = &[];

pub const _eal_components_for_supplierOrConsumerInformationMatch_AssertionType: &[ComponentSpec;
     0] = &[];

pub fn _decode_supplierOrConsumerInformationMatch_AssertionType(
    el: &X690Element,
) -> ASN1Result<supplierOrConsumerInformationMatch_AssertionType> {
    |el_: &X690Element| -> ASN1Result<supplierOrConsumerInformationMatch_AssertionType> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_supplierOrConsumerInformationMatch_AssertionType,
            _eal_components_for_supplierOrConsumerInformationMatch_AssertionType,
            _rctl2_components_for_supplierOrConsumerInformationMatch_AssertionType,
            20,
        )?;
        let ae_title = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
            _components.get("ae-title").unwrap(),
        )?;
        let agreement_identifier =
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(ber_decode_integer(&el.inner()?)?) }(
                _components.get("agreement-identifier").unwrap(),
            )?;
        Ok(supplierOrConsumerInformationMatch_AssertionType {
            ae_title,
            agreement_identifier,
        })
    }(&el)
}

pub fn _encode_supplierOrConsumerInformationMatch_AssertionType(
    value_: &supplierOrConsumerInformationMatch_AssertionType,
) -> ASN1Result<X690Element> {
    |value_: &supplierOrConsumerInformationMatch_AssertionType| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
            ))
        }(&value_.ae_title)?);
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
            ))
        }(&value_.agreement_identifier)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}
