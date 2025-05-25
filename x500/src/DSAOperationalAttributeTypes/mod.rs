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
use wildboar_asn1::*;
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

pub mod dseType {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DSEType; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DSEType(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DSEType(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DSEType(el)
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
    BER.decode_bit_string(&el)
}

pub fn _encode_DSEType(value_: &DSEType) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_DSEType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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

pub mod myAccessPoint {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AccessPoint; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AccessPoint(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AccessPoint(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AccessPoint(el)
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

pub mod superiorKnowledge {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AccessPoint; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AccessPoint(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AccessPoint(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AccessPoint(el)
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

pub mod specificKnowledge {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MasterAndShadowAccessPoints; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MasterAndShadowAccessPoints(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MasterAndShadowAccessPoints(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MasterAndShadowAccessPoints(el)
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

pub mod nonSpecificKnowledge {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MasterAndShadowAccessPoints; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MasterAndShadowAccessPoints(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MasterAndShadowAccessPoints(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MasterAndShadowAccessPoints(el)
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
impl TryFrom<&X690Element> for SupplierOrConsumer {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupplierOrConsumer")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_SupplierOrConsumer,
        _eal_components_for_SupplierOrConsumer,
        _rctl2_components_for_SupplierOrConsumer,
        50,
    )?;
    let ae_title_ = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
        _components.get("ae-title").unwrap(),
    )?;
    let address_ = |el: &X690Element| -> ASN1Result<PresentationAddress> {
        Ok(_decode_PresentationAddress(&el.inner()?)?)
    }(_components.get("address").unwrap())?;
    let protocolInformation_: OPTIONAL<Vec<ProtocolInformation>> = match _components
        .get("protocolInformation")
    {
        Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ProtocolInformation>> {
            Ok(
                |el: &X690Element| -> ASN1Result<SET_OF<ProtocolInformation>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "protocolInformation",
                            ))
                        }
                    };
                    let mut items: SET_OF<ProtocolInformation> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ProtocolInformation(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(c_)?),
        _ => None,
    };
    let agreementID_ = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
        Ok(_decode_OperationalBindingID(&el.inner()?)?)
    }(_components.get("agreementID").unwrap())?;
    Ok(SupplierOrConsumer {
        ae_title: ae_title_,
        address: address_,
        protocolInformation: protocolInformation_,
        agreementID: agreementID_,
        _unrecognized,
    })
}

pub fn _encode_SupplierOrConsumer(value_: &SupplierOrConsumer) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_Name(&v_1)?),
        ))
    }(&value_.ae_title)?);
    components_.push(|v_1: &PresentationAddress| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_PresentationAddress(&v_1)?),
        ))
    }(&value_.address)?);
    if let Some(v_) = &value_.protocolInformation {
        components_.push(
            |v_1: &Vec<ProtocolInformation>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(
                        &|value_: &SET_OF<ProtocolInformation>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_ProtocolInformation(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v_)?,
        );
    }
    components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 3),
            X690Value::from_explicit(&_encode_OperationalBindingID(&v_1)?),
        ))
    }(&value_.agreementID)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SupplierOrConsumer(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupplierOrConsumer")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_SupplierOrConsumer,
        _eal_components_for_SupplierOrConsumer,
        _rctl2_components_for_SupplierOrConsumer,
        50,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ae-title"));
        }
        Ok(_validate_Name(&el.inner()?)?)
    }(_components.get("ae-title").unwrap())?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "address"));
        }
        Ok(_validate_PresentationAddress(&el.inner()?)?)
    }(_components.get("address").unwrap())?;
    match _components.get("protocolInformation") {
        Some(c_) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "protocolInformation",
                ));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ProtocolInformation(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "protocolInformation",
                    )),
                }
            }(&el.inner()?)?)
        }(c_)?,
        _ => (),
    };
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "agreementID"));
        }
        Ok(_validate_OperationalBindingID(&el.inner()?)?)
    }(_components.get("agreementID").unwrap())?;
    Ok(())
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
impl TryFrom<&X690Element> for SupplierInformation {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupplierInformation")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_SupplierInformation,
        _eal_components_for_SupplierInformation,
        _rctl2_components_for_SupplierInformation,
        70,
    )?;
    let ae_title_ = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
        _components.get("ae-title").unwrap(),
    )?;
    let address_ = |el: &X690Element| -> ASN1Result<PresentationAddress> {
        Ok(_decode_PresentationAddress(&el.inner()?)?)
    }(_components.get("address").unwrap())?;
    let protocolInformation_: OPTIONAL<Vec<ProtocolInformation>> = match _components
        .get("protocolInformation")
    {
        Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ProtocolInformation>> {
            Ok(
                |el: &X690Element| -> ASN1Result<SET_OF<ProtocolInformation>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "protocolInformation",
                            ))
                        }
                    };
                    let mut items: SET_OF<ProtocolInformation> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ProtocolInformation(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(c_)?),
        _ => None,
    };
    let agreementID_ = |el: &X690Element| -> ASN1Result<OperationalBindingID> {
        Ok(_decode_OperationalBindingID(&el.inner()?)?)
    }(_components.get("agreementID").unwrap())?;
    let supplier_is_master_: OPTIONAL<BOOLEAN> = match _components.get("supplier-is-master") {
        Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
            Ok(BER.decode_boolean(&el.inner()?)?)
        }(c_)?),
        _ => None,
    };
    let non_supplying_master_: OPTIONAL<AccessPoint> = match _components.get("non-supplying-master")
    {
        Some(c_) => Some(|el: &X690Element| -> ASN1Result<AccessPoint> {
            Ok(_decode_AccessPoint(&el.inner()?)?)
        }(c_)?),
        _ => None,
    };
    Ok(SupplierInformation {
        ae_title: ae_title_,
        address: address_,
        protocolInformation: protocolInformation_,
        agreementID: agreementID_,
        supplier_is_master: supplier_is_master_,
        non_supplying_master: non_supplying_master_,
        _unrecognized,
    })
}

pub fn _encode_SupplierInformation(value_: &SupplierInformation) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(16);
    components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_Name(&v_1)?),
        ))
    }(&value_.ae_title)?);
    components_.push(|v_1: &PresentationAddress| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_PresentationAddress(&v_1)?),
        ))
    }(&value_.address)?);
    if let Some(v_) = &value_.protocolInformation {
        components_.push(
            |v_1: &Vec<ProtocolInformation>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(
                        &|value_: &SET_OF<ProtocolInformation>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_ProtocolInformation(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v_)?,
        );
    }
    components_.push(|v_1: &OperationalBindingID| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 3),
            X690Value::from_explicit(&_encode_OperationalBindingID(&v_1)?),
        ))
    }(&value_.agreementID)?);
    if let Some(v_) = &value_.supplier_is_master {
        if *v_ != SupplierInformation::_default_value_for_supplier_is_master() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 4),
                    X690Value::from_explicit(&BER.encode_boolean(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.non_supplying_master {
        components_.push(|v_1: &AccessPoint| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::from_explicit(&_encode_AccessPoint(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SupplierInformation(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupplierInformation")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_SupplierInformation,
        _eal_components_for_SupplierInformation,
        _rctl2_components_for_SupplierInformation,
        70,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ae-title"));
        }
        Ok(_validate_Name(&el.inner()?)?)
    }(_components.get("ae-title").unwrap())?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "address"));
        }
        Ok(_validate_PresentationAddress(&el.inner()?)?)
    }(_components.get("address").unwrap())?;
    match _components.get("protocolInformation") {
        Some(c_) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "protocolInformation",
                ));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ProtocolInformation(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "protocolInformation",
                    )),
                }
            }(&el.inner()?)?)
        }(c_)?,
        _ => (),
    };
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "agreementID"));
        }
        Ok(_validate_OperationalBindingID(&el.inner()?)?)
    }(_components.get("agreementID").unwrap())?;
    match _components.get("supplier-is-master") {
        Some(c_) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "supplier-is-master")
                );
            }
            Ok(BER.validate_boolean(&el.inner()?)?)
        }(c_)?,
        _ => (),
    };
    match _components.get("non-supplying-master") {
        Some(c_) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "non-supplying-master",
                ));
            }
            Ok(_validate_AccessPoint(&el.inner()?)?)
        }(c_)?,
        _ => (),
    };
    Ok(())
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

pub mod supplierKnowledge {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupplierInformation; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupplierInformation(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupplierInformation(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupplierInformation(el)
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

pub fn _validate_ConsumerInformation(el: &X690Element) -> ASN1Result<()> {
    _validate_SupplierOrConsumer(&el)
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

pub mod consumerKnowledge {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ConsumerInformation; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ConsumerInformation(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ConsumerInformation(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ConsumerInformation(el)
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
impl TryFrom<&X690Element> for SupplierAndConsumers {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupplierAndConsumers")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_SupplierAndConsumers,
        _eal_components_for_SupplierAndConsumers,
        _rctl2_components_for_SupplierAndConsumers,
        50,
    )?;
    let ae_title_ = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
        _components.get("ae-title").unwrap(),
    )?;
    let address_ = |el: &X690Element| -> ASN1Result<PresentationAddress> {
        Ok(_decode_PresentationAddress(&el.inner()?)?)
    }(_components.get("address").unwrap())?;
    let protocolInformation_: OPTIONAL<Vec<ProtocolInformation>> = match _components
        .get("protocolInformation")
    {
        Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ProtocolInformation>> {
            Ok(
                |el: &X690Element| -> ASN1Result<SET_OF<ProtocolInformation>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "protocolInformation",
                            ))
                        }
                    };
                    let mut items: SET_OF<ProtocolInformation> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ProtocolInformation(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(c_)?),
        _ => None,
    };
    let consumers_ = |el: &X690Element| -> ASN1Result<Vec<AccessPoint>> {
        Ok(|el: &X690Element| -> ASN1Result<SET_OF<AccessPoint>> {
            let elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "consumers")
                    )
                }
            };
            let mut items: SET_OF<AccessPoint> = Vec::with_capacity(elements.len());
            for el in elements.iter() {
                items.push(_decode_AccessPoint(el)?);
            }
            Ok(items)
        }(&el.inner()?)?)
    }(_components.get("consumers").unwrap())?;
    Ok(SupplierAndConsumers {
        ae_title: ae_title_,
        address: address_,
        protocolInformation: protocolInformation_,
        consumers: consumers_,
        _unrecognized,
    })
}

pub fn _encode_SupplierAndConsumers(value_: &SupplierAndConsumers) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_Name(&v_1)?),
        ))
    }(&value_.ae_title)?);
    components_.push(|v_1: &PresentationAddress| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_PresentationAddress(&v_1)?),
        ))
    }(&value_.address)?);
    if let Some(v_) = &value_.protocolInformation {
        components_.push(
            |v_1: &Vec<ProtocolInformation>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(
                        &|value_: &SET_OF<ProtocolInformation>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_ProtocolInformation(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v_)?,
        );
    }
    components_.push(|v_1: &Vec<AccessPoint>| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 3),
            X690Value::from_explicit(&|value_: &SET_OF<AccessPoint>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_AccessPoint(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?),
        ))
    }(&value_.consumers)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SupplierAndConsumers(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupplierAndConsumers")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_SupplierAndConsumers,
        _eal_components_for_SupplierAndConsumers,
        _rctl2_components_for_SupplierAndConsumers,
        50,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ae-title"));
        }
        Ok(_validate_Name(&el.inner()?)?)
    }(_components.get("ae-title").unwrap())?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "address"));
        }
        Ok(_validate_PresentationAddress(&el.inner()?)?)
    }(_components.get("address").unwrap())?;
    match _components.get("protocolInformation") {
        Some(c_) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "protocolInformation",
                ));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ProtocolInformation(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "protocolInformation",
                    )),
                }
            }(&el.inner()?)?)
        }(c_)?,
        _ => (),
    };
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "consumers"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            match &el.value {
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        _validate_AccessPoint(&sub)?;
                    }
                    Ok(())
                }
                _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "consumers")),
            }
        }(&el.inner()?)?)
    }(_components.get("consumers").unwrap())?;
    Ok(())
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

pub mod secondaryShadows {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupplierAndConsumers; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupplierAndConsumers(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupplierAndConsumers(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupplierAndConsumers(el)
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

pub mod ditBridgeKnowledge {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DitBridgeKnowledge; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DitBridgeKnowledge(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DitBridgeKnowledge(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DitBridgeKnowledge(el)
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

pub mod accessPointMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = Name; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_Name(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_Name(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_Name(el)
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

pub mod masterAndShadowAccessPointsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = Vec<Name>; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&AssertionType")
                )
            }
        };
        let mut items: SET_OF<Name> = Vec::with_capacity(elements.len());
        for el in elements.iter() {
            items.push(_decode_Name(el)?);
        }
        Ok(items)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Name(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    _validate_Name(&sub)?;
                }
                Ok(())
            }
            _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&AssertionType")),
        }
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

pub mod supplierOrConsumerInformationMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = supplierOrConsumerInformationMatch_AssertionType; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_supplierOrConsumerInformationMatch_AssertionType(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_supplierOrConsumerInformationMatch_AssertionType(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_supplierOrConsumerInformationMatch_AssertionType(el)
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

pub mod supplierAndConsumersMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = Name; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_Name(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_Name(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_Name(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-dseType                  OBJECT IDENTIFIER ::= {id-doa 0}
/// ```
///
///
#[inline]
pub fn id_doa_dseType () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-myAccessPoint            OBJECT IDENTIFIER ::= {id-doa 1}
/// ```
///
///
#[inline]
pub fn id_doa_myAccessPoint () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-superiorKnowledge        OBJECT IDENTIFIER ::= {id-doa 2}
/// ```
///
///
#[inline]
pub fn id_doa_superiorKnowledge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-specificKnowledge        OBJECT IDENTIFIER ::= {id-doa 3}
/// ```
///
///
#[inline]
pub fn id_doa_specificKnowledge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-nonSpecificKnowledge     OBJECT IDENTIFIER ::= {id-doa 4}
/// ```
///
///
#[inline]
pub fn id_doa_nonSpecificKnowledge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-supplierKnowledge        OBJECT IDENTIFIER ::= {id-doa 5}
/// ```
///
///
#[inline]
pub fn id_doa_supplierKnowledge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-consumerKnowledge        OBJECT IDENTIFIER ::= {id-doa 6}
/// ```
///
///
#[inline]
pub fn id_doa_consumerKnowledge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 6).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-secondaryShadows         OBJECT IDENTIFIER ::= {id-doa 7}
/// ```
///
///
#[inline]
pub fn id_doa_secondaryShadows () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 7).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa-ditBridgeKnowledge       OBJECT IDENTIFIER ::= {id-doa 8}
/// ```
///
///
#[inline]
pub fn id_doa_ditBridgeKnowledge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_doa(), 8).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-accessPointMatch         OBJECT IDENTIFIER ::= {id-kmr 0}
/// ```
///
///
#[inline]
pub fn id_kmr_accessPointMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_kmr(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-masterShadowMatch        OBJECT IDENTIFIER ::= {id-kmr 1}
/// ```
///
///
#[inline]
pub fn id_kmr_masterShadowMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_kmr(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-supplierConsumerMatch    OBJECT IDENTIFIER ::= {id-kmr 2}
/// ```
///
///
#[inline]
pub fn id_kmr_supplierConsumerMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_kmr(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr-supplierConsumersMatch   OBJECT IDENTIFIER ::= {id-kmr 3}
/// ```
///
///
#[inline]
pub fn id_kmr_supplierConsumersMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_kmr(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// supplierOrConsumerInformationMatch-AssertionType ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for supplierOrConsumerInformationMatch_AssertionType {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "supplierOrConsumerInformationMatch-AssertionType",
            ))
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_supplierOrConsumerInformationMatch_AssertionType,
        _eal_components_for_supplierOrConsumerInformationMatch_AssertionType,
        _rctl2_components_for_supplierOrConsumerInformationMatch_AssertionType,
        20,
    )?;
    let ae_title_ = |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(
        _components.get("ae-title").unwrap(),
    )?;
    let agreement_identifier_ =
        |el: &X690Element| -> ASN1Result<INTEGER> { Ok(BER.decode_integer(&el.inner()?)?) }(
            _components.get("agreement-identifier").unwrap(),
        )?;
    Ok(supplierOrConsumerInformationMatch_AssertionType {
        ae_title: ae_title_,
        agreement_identifier: agreement_identifier_,
    })
}

pub fn _encode_supplierOrConsumerInformationMatch_AssertionType(
    value_: &supplierOrConsumerInformationMatch_AssertionType,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_Name(&v_1)?),
        ))
    }(&value_.ae_title)?);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 2),
            X690Value::from_explicit(&BER.encode_integer(&v_1)?),
        ))
    }(&value_.agreement_identifier)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_supplierOrConsumerInformationMatch_AssertionType(
    el: &X690Element,
) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "supplierOrConsumerInformationMatch-AssertionType",
            ))
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_supplierOrConsumerInformationMatch_AssertionType,
        _eal_components_for_supplierOrConsumerInformationMatch_AssertionType,
        _rctl2_components_for_supplierOrConsumerInformationMatch_AssertionType,
        20,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ae-title"));
        }
        Ok(_validate_Name(&el.inner()?)?)
    }(_components.get("ae-title").unwrap())?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "agreement-identifier")
            );
        }
        Ok(BER.validate_integer(&el.inner()?)?)
    }(_components.get("agreement-identifier").unwrap())?;
    Ok(())
}
