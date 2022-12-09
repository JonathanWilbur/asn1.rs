#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # AttributeCertificateDefinitions
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `AttributeCertificateDefinitions`.
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
use crate::AuthenticationFramework::*;
use crate::CertificateExtensions::*;
use crate::InformationFramework::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificate  ::=  SIGNED{TBSAttributeCertificate}
/// ```
pub type AttributeCertificate = SIGNED<TBSAttributeCertificate>; // DefinedType

pub fn _decode_AttributeCertificate(el: &X690Element) -> ASN1Result<AttributeCertificate> {
    _decode_SIGNED::<TBSAttributeCertificate>(_decode_TBSAttributeCertificate, &el)
}

pub fn _encode_AttributeCertificate(value_: &AttributeCertificate) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSAttributeCertificate>(_encode_TBSAttributeCertificate, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSAttributeCertificate ::= SEQUENCE {
///   version                 AttCertVersion, -- version is v2
///   holder                  Holder,
///   issuer                  AttCertIssuer,
///   signature               AlgorithmIdentifier{{SupportedAlgorithms}},
///   serialNumber            CertificateSerialNumber,
///   attrCertValidityPeriod  AttCertValidityPeriod,
///   attributes              SEQUENCE OF Attribute{{SupportedAttributes}},
///   issuerUniqueID          UniqueIdentifier OPTIONAL,
///   ...,
///   ...,
///   extensions              Extensions OPTIONAL
///  }  (CONSTRAINED BY { -- shall be DER encoded -- } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TBSAttributeCertificate {
    pub version: AttCertVersion,
    pub holder: Holder,
    pub issuer: AttCertIssuer,
    pub signature: AlgorithmIdentifier,
    pub serialNumber: CertificateSerialNumber,
    pub attrCertValidityPeriod: AttCertValidityPeriod,
    pub attributes: Vec<Attribute>,
    pub issuerUniqueID: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
    pub extensions: OPTIONAL<Extensions>,
}
impl TBSAttributeCertificate {
    pub fn new(
        version: AttCertVersion,
        holder: Holder,
        issuer: AttCertIssuer,
        signature: AlgorithmIdentifier,
        serialNumber: CertificateSerialNumber,
        attrCertValidityPeriod: AttCertValidityPeriod,
        attributes: Vec<Attribute>,
        issuerUniqueID: OPTIONAL<UniqueIdentifier>,
        _unrecognized: Vec<X690Element>,
        extensions: OPTIONAL<Extensions>,
    ) -> Self {
        TBSAttributeCertificate {
            version,
            holder,
            issuer,
            signature,
            serialNumber,
            attrCertValidityPeriod,
            attributes,
            issuerUniqueID,
            extensions,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TBSAttributeCertificate {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSAttributeCertificate(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSAttributeCertificate {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TBSAttributeCertificate(el)
    }
}

pub const _rctl1_components_for_TBSAttributeCertificate: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "holder",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attrCertValidityPeriod",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuerUniqueID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBSAttributeCertificate: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    )];

pub const _eal_components_for_TBSAttributeCertificate: &[ComponentSpec; 0] = &[];

pub fn _decode_TBSAttributeCertificate(el: &X690Element) -> ASN1Result<TBSAttributeCertificate> {
    |el_: &X690Element| -> ASN1Result<TBSAttributeCertificate> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TBSAttributeCertificate,
            _eal_components_for_TBSAttributeCertificate,
            _rctl2_components_for_TBSAttributeCertificate,
        )?;
        let version = _decode_AttCertVersion(_components.get("version").unwrap())?;
        let holder = _decode_Holder(_components.get("holder").unwrap())?;
        let issuer = _decode_AttCertIssuer(_components.get("issuer").unwrap())?;
        let signature = _decode_AlgorithmIdentifier(_components.get("signature").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let attrCertValidityPeriod =
            _decode_AttCertValidityPeriod(_components.get("attrCertValidityPeriod").unwrap())?;
        let attributes = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_Attribute(el)?);
            }
            Ok(items)
        }(_components.get("attributes").unwrap())?;
        let issuerUniqueID: OPTIONAL<UniqueIdentifier> = match _components.get("issuerUniqueID") {
            Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
            _ => None,
        };
        let extensions: OPTIONAL<Extensions> = match _components.get("extensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(TBSAttributeCertificate {
            version,
            holder,
            issuer,
            signature,
            serialNumber,
            attrCertValidityPeriod,
            attributes,
            issuerUniqueID,
            _unrecognized,
            extensions,
        })
    }(&el)
}

pub fn _encode_TBSAttributeCertificate(
    value_: &TBSAttributeCertificate,
) -> ASN1Result<X690Element> {
    |value_: &TBSAttributeCertificate| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(19);
        components_.push(_encode_AttCertVersion(&value_.version)?);
        components_.push(_encode_Holder(&value_.holder)?);
        components_.push(_encode_AttCertIssuer(&value_.issuer)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_AttCertValidityPeriod(
            &value_.attrCertValidityPeriod,
        )?);
        components_.push(
            |value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Attribute(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.attributes)?,
        );
        if let Some(v_) = &value_.issuerUniqueID {
            components_.push(_encode_UniqueIdentifier(&v_)?);
        }
        if let Some(v_) = &value_.extensions {
            components_.push(_encode_Extensions(&v_)?);
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
/// AttCertVersion  ::=  INTEGER {v2(1)}
/// ```
pub type AttCertVersion = INTEGER;

pub const AttCertVersion_v2: AttCertVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AttCertVersion(el: &X690Element) -> ASN1Result<AttCertVersion> {
    ber_decode_integer(&el)
}

pub fn _encode_AttCertVersion(value_: &AttCertVersion) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Holder ::= SEQUENCE {
///   baseCertificateID  [0]  IssuerSerial OPTIONAL,
///   entityName         [1]  GeneralNames OPTIONAL,
///   objectDigestInfo   [2]  ObjectDigestInfo OPTIONAL }
///   (WITH COMPONENTS {..., baseCertificateID  PRESENT } |
///    WITH COMPONENTS {..., entityName  PRESENT } |
///    WITH COMPONENTS {..., objectDigestInfo  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Holder {
    pub baseCertificateID: OPTIONAL<IssuerSerial>,
    pub entityName: OPTIONAL<GeneralNames>,
    pub objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
}
impl Holder {
    pub fn new(
        baseCertificateID: OPTIONAL<IssuerSerial>,
        entityName: OPTIONAL<GeneralNames>,
        objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
    ) -> Self {
        Holder {
            baseCertificateID,
            entityName,
            objectDigestInfo,
        }
    }
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            baseCertificateID: None,
            entityName: None,
            objectDigestInfo: None,
        }
    }
}
impl TryFrom<X690Element> for Holder {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Holder(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Holder {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Holder(el)
    }
}

pub const _rctl1_components_for_Holder: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "baseCertificateID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entityName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectDigestInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Holder: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Holder: &[ComponentSpec; 0] = &[];

pub fn _decode_Holder(el: &X690Element) -> ASN1Result<Holder> {
    |el_: &X690Element| -> ASN1Result<Holder> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Holder,
            _eal_components_for_Holder,
            _rctl2_components_for_Holder,
        )?;
        let baseCertificateID: OPTIONAL<IssuerSerial> = match _components.get("baseCertificateID") {
            Some(c_) => Some(_decode_IssuerSerial(c_)?),
            _ => None,
        };
        let entityName: OPTIONAL<GeneralNames> = match _components.get("entityName") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let objectDigestInfo: OPTIONAL<ObjectDigestInfo> = match _components.get("objectDigestInfo")
        {
            Some(c_) => Some(_decode_ObjectDigestInfo(c_)?),
            _ => None,
        };
        Ok(Holder {
            baseCertificateID,
            entityName,
            objectDigestInfo,
        })
    }(&el)
}

pub fn _encode_Holder(value_: &Holder) -> ASN1Result<X690Element> {
    |value_: &Holder| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        if let Some(v_) = &value_.baseCertificateID {
            components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_IssuerSerial(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.entityName {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.objectDigestInfo {
            components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ObjectDigestInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
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
/// IssuerSerial ::= SEQUENCE {
///   issuer     GeneralNames,
///   serial     CertificateSerialNumber,
///   issuerUID  UniqueIdentifier OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IssuerSerial {
    pub issuer: GeneralNames,
    pub serial: CertificateSerialNumber,
    pub issuerUID: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
}
impl IssuerSerial {
    pub fn new(
        issuer: GeneralNames,
        serial: CertificateSerialNumber,
        issuerUID: OPTIONAL<UniqueIdentifier>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IssuerSerial {
            issuer,
            serial,
            issuerUID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for IssuerSerial {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerSerial(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IssuerSerial {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerSerial(el)
    }
}

pub const _rctl1_components_for_IssuerSerial: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "issuer",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serial",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuerUID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IssuerSerial: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IssuerSerial: &[ComponentSpec; 0] = &[];

pub fn _decode_IssuerSerial(el: &X690Element) -> ASN1Result<IssuerSerial> {
    |el_: &X690Element| -> ASN1Result<IssuerSerial> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IssuerSerial,
            _eal_components_for_IssuerSerial,
            _rctl2_components_for_IssuerSerial,
        )?;
        let issuer = _decode_GeneralNames(_components.get("issuer").unwrap())?;
        let serial = _decode_CertificateSerialNumber(_components.get("serial").unwrap())?;
        let issuerUID: OPTIONAL<UniqueIdentifier> = match _components.get("issuerUID") {
            Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
            _ => None,
        };
        Ok(IssuerSerial {
            issuer,
            serial,
            issuerUID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IssuerSerial(value_: &IssuerSerial) -> ASN1Result<X690Element> {
    |value_: &IssuerSerial| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_GeneralNames(&value_.issuer)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serial)?);
        if let Some(v_) = &value_.issuerUID {
            components_.push(_encode_UniqueIdentifier(&v_)?);
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
/// ObjectDigestInfo ::= SEQUENCE {
///   digestedObjectType   ENUMERATED {
///     publicKey        (0),
///     publicKeyCert    (1),
///     otherObjectTypes (2)},
///   otherObjectTypeID   OBJECT IDENTIFIER OPTIONAL,
///   digestAlgorithm     AlgorithmIdentifier{{SupportedAlgorithms}},
///   objectDigest        BIT STRING,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ObjectDigestInfo {
    pub digestedObjectType: ObjectDigestInfo_digestedObjectType,
    pub otherObjectTypeID: OPTIONAL<OBJECT_IDENTIFIER>,
    pub digestAlgorithm: AlgorithmIdentifier,
    pub objectDigest: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl ObjectDigestInfo {
    pub fn new(
        digestedObjectType: ObjectDigestInfo_digestedObjectType,
        otherObjectTypeID: OPTIONAL<OBJECT_IDENTIFIER>,
        digestAlgorithm: AlgorithmIdentifier,
        objectDigest: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ObjectDigestInfo {
            digestedObjectType,
            otherObjectTypeID,
            digestAlgorithm,
            objectDigest,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ObjectDigestInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectDigestInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ObjectDigestInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectDigestInfo(el)
    }
}

pub const _rctl1_components_for_ObjectDigestInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "digestedObjectType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "otherObjectTypeID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "digestAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectDigest",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ObjectDigestInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ObjectDigestInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ObjectDigestInfo(el: &X690Element) -> ASN1Result<ObjectDigestInfo> {
    |el_: &X690Element| -> ASN1Result<ObjectDigestInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ObjectDigestInfo,
            _eal_components_for_ObjectDigestInfo,
            _rctl2_components_for_ObjectDigestInfo,
        )?;
        let digestedObjectType = _decode_ObjectDigestInfo_digestedObjectType(
            _components.get("digestedObjectType").unwrap(),
        )?;
        let otherObjectTypeID: OPTIONAL<OBJECT_IDENTIFIER> =
            match _components.get("otherObjectTypeID") {
                Some(c_) => Some(ber_decode_object_identifier(c_)?),
                _ => None,
            };
        let digestAlgorithm =
            _decode_AlgorithmIdentifier(_components.get("digestAlgorithm").unwrap())?;
        let objectDigest = ber_decode_bit_string(_components.get("objectDigest").unwrap())?;
        Ok(ObjectDigestInfo {
            digestedObjectType,
            otherObjectTypeID,
            digestAlgorithm,
            objectDigest,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ObjectDigestInfo(value_: &ObjectDigestInfo) -> ASN1Result<X690Element> {
    |value_: &ObjectDigestInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_ObjectDigestInfo_digestedObjectType(
            &value_.digestedObjectType,
        )?);
        if let Some(v_) = &value_.otherObjectTypeID {
            components_.push(ber_encode_object_identifier(&v_)?);
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.digestAlgorithm)?);
        components_.push(ber_encode_bit_string(&value_.objectDigest)?);
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
/// AttCertIssuer ::= [0]  SEQUENCE {
///   issuerName              GeneralNames OPTIONAL,
///   baseCertificateID  [0]  IssuerSerial OPTIONAL,
///   objectDigestInfo   [1]  ObjectDigestInfo OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., issuerName  PRESENT } |
///    WITH COMPONENTS {..., baseCertificateID  PRESENT } |
///    WITH COMPONENTS {..., objectDigestInfo  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttCertIssuer {
    pub issuerName: OPTIONAL<GeneralNames>,
    pub baseCertificateID: OPTIONAL<IssuerSerial>,
    pub objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttCertIssuer {
    pub fn new(
        issuerName: OPTIONAL<GeneralNames>,
        baseCertificateID: OPTIONAL<IssuerSerial>,
        objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttCertIssuer {
            issuerName,
            baseCertificateID,
            objectDigestInfo,
            _unrecognized,
        }
    }
}
impl Default for AttCertIssuer {
    fn default() -> Self {
        AttCertIssuer {
            issuerName: None,
            baseCertificateID: None,
            objectDigestInfo: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AttCertIssuer {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertIssuer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttCertIssuer {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertIssuer(el)
    }
}

pub const _rctl1_components_for_AttCertIssuer: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "issuerName",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "baseCertificateID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectDigestInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttCertIssuer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttCertIssuer: &[ComponentSpec; 0] = &[];

pub fn _decode_AttCertIssuer(el: &X690Element) -> ASN1Result<AttCertIssuer> {
    |el_: &X690Element| -> ASN1Result<AttCertIssuer> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttCertIssuer,
            _eal_components_for_AttCertIssuer,
            _rctl2_components_for_AttCertIssuer,
        )?;
        let issuerName: OPTIONAL<GeneralNames> = match _components.get("issuerName") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let baseCertificateID: OPTIONAL<IssuerSerial> = match _components.get("baseCertificateID") {
            Some(c_) => Some(_decode_IssuerSerial(c_)?),
            _ => None,
        };
        let objectDigestInfo: OPTIONAL<ObjectDigestInfo> = match _components.get("objectDigestInfo")
        {
            Some(c_) => Some(_decode_ObjectDigestInfo(c_)?),
            _ => None,
        };
        Ok(AttCertIssuer {
            issuerName,
            baseCertificateID,
            objectDigestInfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttCertIssuer(value_: &AttCertIssuer) -> ASN1Result<X690Element> {
    |v_1: &AttCertIssuer| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &AttCertIssuer| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(13);
            if let Some(v_) = &value_.issuerName {
                components_.push(_encode_GeneralNames(&v_)?);
            }
            if let Some(v_) = &value_.baseCertificateID {
                components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_IssuerSerial(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.objectDigestInfo {
                components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ObjectDigestInfo(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
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
/// AttCertValidityPeriod ::= SEQUENCE {
///   notBeforeTime  GeneralizedTime,
///   notAfterTime   GeneralizedTime,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttCertValidityPeriod {
    pub notBeforeTime: GeneralizedTime,
    pub notAfterTime: GeneralizedTime,
    pub _unrecognized: Vec<X690Element>,
}
impl AttCertValidityPeriod {
    pub fn new(
        notBeforeTime: GeneralizedTime,
        notAfterTime: GeneralizedTime,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttCertValidityPeriod {
            notBeforeTime,
            notAfterTime,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttCertValidityPeriod {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertValidityPeriod(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttCertValidityPeriod {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertValidityPeriod(el)
    }
}

pub const _rctl1_components_for_AttCertValidityPeriod: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "notBeforeTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notAfterTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttCertValidityPeriod: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttCertValidityPeriod: &[ComponentSpec; 0] = &[];

pub fn _decode_AttCertValidityPeriod(el: &X690Element) -> ASN1Result<AttCertValidityPeriod> {
    |el_: &X690Element| -> ASN1Result<AttCertValidityPeriod> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttCertValidityPeriod,
            _eal_components_for_AttCertValidityPeriod,
            _rctl2_components_for_AttCertValidityPeriod,
        )?;
        let notBeforeTime = ber_decode_generalized_time(_components.get("notBeforeTime").unwrap())?;
        let notAfterTime = ber_decode_generalized_time(_components.get("notAfterTime").unwrap())?;
        Ok(AttCertValidityPeriod {
            notBeforeTime,
            notAfterTime,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttCertValidityPeriod(value_: &AttCertValidityPeriod) -> ASN1Result<X690Element> {
    |value_: &AttCertValidityPeriod| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_generalized_time(&value_.notBeforeTime)?);
        components_.push(ber_encode_generalized_time(&value_.notAfterTime)?);
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
/// AttributeCertificationPath ::= SEQUENCE {
///   attributeCertificate  AttributeCertificate,
///   acPath                SEQUENCE OF ACPathData OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeCertificationPath {
    pub attributeCertificate: AttributeCertificate,
    pub acPath: OPTIONAL<Vec<ACPathData>>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeCertificationPath {
    pub fn new(
        attributeCertificate: AttributeCertificate,
        acPath: OPTIONAL<Vec<ACPathData>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeCertificationPath {
            attributeCertificate,
            acPath,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeCertificationPath {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificationPath(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeCertificationPath {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificationPath(el)
    }
}

pub const _rctl1_components_for_AttributeCertificationPath: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "attributeCertificate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "acPath",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeCertificationPath: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeCertificationPath: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeCertificationPath(
    el: &X690Element,
) -> ASN1Result<AttributeCertificationPath> {
    |el_: &X690Element| -> ASN1Result<AttributeCertificationPath> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeCertificationPath,
            _eal_components_for_AttributeCertificationPath,
            _rctl2_components_for_AttributeCertificationPath,
        )?;
        let attributeCertificate =
            _decode_AttributeCertificate(_components.get("attributeCertificate").unwrap())?;
        let acPath: OPTIONAL<Vec<ACPathData>> = match _components.get("acPath") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<ACPathData>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<ACPathData> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_ACPathData(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        Ok(AttributeCertificationPath {
            attributeCertificate,
            acPath,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeCertificationPath(
    value_: &AttributeCertificationPath,
) -> ASN1Result<X690Element> {
    |value_: &AttributeCertificationPath| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeCertificate(&value_.attributeCertificate)?);
        if let Some(v_) = &value_.acPath {
            components_.push(
                |value_: &SEQUENCE_OF<ACPathData>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_ACPathData(&v)?);
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
/// ACPathData ::= SEQUENCE {
///   certificate           [0]  Certificate OPTIONAL,
///   attributeCertificate  [1]  AttributeCertificate OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ACPathData {
    pub certificate: OPTIONAL<Certificate>,
    pub attributeCertificate: OPTIONAL<AttributeCertificate>,
    pub _unrecognized: Vec<X690Element>,
}
impl ACPathData {
    pub fn new(
        certificate: OPTIONAL<Certificate>,
        attributeCertificate: OPTIONAL<AttributeCertificate>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ACPathData {
            certificate,
            attributeCertificate,
            _unrecognized,
        }
    }
}
impl Default for ACPathData {
    fn default() -> Self {
        ACPathData {
            certificate: None,
            attributeCertificate: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ACPathData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ACPathData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ACPathData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ACPathData(el)
    }
}

pub const _rctl1_components_for_ACPathData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "certificate",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeCertificate",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ACPathData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ACPathData: &[ComponentSpec; 0] = &[];

pub fn _decode_ACPathData(el: &X690Element) -> ASN1Result<ACPathData> {
    |el_: &X690Element| -> ASN1Result<ACPathData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ACPathData,
            _eal_components_for_ACPathData,
            _rctl2_components_for_ACPathData,
        )?;
        let certificate: OPTIONAL<Certificate> = match _components.get("certificate") {
            Some(c_) => Some(_decode_Certificate(c_)?),
            _ => None,
        };
        let attributeCertificate: OPTIONAL<AttributeCertificate> =
            match _components.get("attributeCertificate") {
                Some(c_) => Some(_decode_AttributeCertificate(c_)?),
                _ => None,
            };
        Ok(ACPathData {
            certificate,
            attributeCertificate,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ACPathData(value_: &ACPathData) -> ASN1Result<X690Element> {
    |value_: &ACPathData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.certificate {
            components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Certificate(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.attributeCertificate {
            components_.push(|v_1: &AttributeCertificate| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeCertificate(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
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
/// PrivilegePolicy  ::=  OBJECT IDENTIFIER
/// ```
pub type PrivilegePolicy = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_PrivilegePolicy(el: &X690Element) -> ASN1Result<PrivilegePolicy> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_PrivilegePolicy(value_: &PrivilegePolicy) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// role ATTRIBUTE ::= {
///   WITH SYNTAX  RoleSyntax
///   ID           id-at-role }
/// ```
///
///
pub fn role() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_role(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RoleSyntax ::= SEQUENCE {
///   roleAuthority  [0]  GeneralNames OPTIONAL,
///   roleName       [1]  GeneralName,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RoleSyntax {
    pub roleAuthority: OPTIONAL<GeneralNames>,
    pub roleName: GeneralName,
    pub _unrecognized: Vec<X690Element>,
}
impl RoleSyntax {
    pub fn new(
        roleAuthority: OPTIONAL<GeneralNames>,
        roleName: GeneralName,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RoleSyntax {
            roleAuthority,
            roleName,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for RoleSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RoleSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RoleSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RoleSyntax(el)
    }
}

pub const _rctl1_components_for_RoleSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "roleAuthority",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "roleName",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RoleSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RoleSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_RoleSyntax(el: &X690Element) -> ASN1Result<RoleSyntax> {
    |el_: &X690Element| -> ASN1Result<RoleSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RoleSyntax,
            _eal_components_for_RoleSyntax,
            _rctl2_components_for_RoleSyntax,
        )?;
        let roleAuthority: OPTIONAL<GeneralNames> = match _components.get("roleAuthority") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let roleName = |el: &X690Element| -> ASN1Result<GeneralName> {
            Ok(_decode_GeneralName(&el.inner()?)?)
        }(_components.get("roleName").unwrap())?;
        Ok(RoleSyntax {
            roleAuthority,
            roleName,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RoleSyntax(value_: &RoleSyntax) -> ASN1Result<X690Element> {
    |value_: &RoleSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.roleAuthority {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
            ))
        }(&value_.roleName)?);
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
/// xmlPrivilegeInfo ATTRIBUTE ::= {
///   WITH SYNTAX  UTF8String --contains XML-encoded privilege information
///   ID           id-at-xMLPrivilegeInfo }
/// ```
///
///
pub fn xmlPrivilegeInfo() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_xMLPrivilegeInfo(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// permission ATTRIBUTE ::= {
///   WITH SYNTAX             DualStringSyntax
///   EQUALITY MATCHING RULE  dualStringMatch
///   ID                      id-at-permission }
/// ```
///
///
pub fn permission() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(dualStringMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_permission(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DualStringSyntax ::= SEQUENCE {
///   operation  [0]  UnboundedDirectoryString,
///   object     [1]  UnboundedDirectoryString,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DualStringSyntax {
    pub operation: UnboundedDirectoryString,
    pub object: UnboundedDirectoryString,
    pub _unrecognized: Vec<X690Element>,
}
impl DualStringSyntax {
    pub fn new(
        operation: UnboundedDirectoryString,
        object: UnboundedDirectoryString,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DualStringSyntax {
            operation,
            object,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DualStringSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DualStringSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DualStringSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DualStringSyntax(el)
    }
}

pub const _rctl1_components_for_DualStringSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "operation",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DualStringSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DualStringSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_DualStringSyntax(el: &X690Element) -> ASN1Result<DualStringSyntax> {
    |el_: &X690Element| -> ASN1Result<DualStringSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DualStringSyntax,
            _eal_components_for_DualStringSyntax,
            _rctl2_components_for_DualStringSyntax,
        )?;
        let operation = |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
            Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
        }(_components.get("operation").unwrap())?;
        let object = |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
            Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
        }(_components.get("object").unwrap())?;
        Ok(DualStringSyntax {
            operation,
            object,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DualStringSyntax(value_: &DualStringSyntax) -> ASN1Result<X690Element> {
    |value_: &DualStringSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UnboundedDirectoryString(&v_1)?,
                    ))),
                ))
            }(&value_.operation)?,
        );
        components_.push(
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UnboundedDirectoryString(&v_1)?,
                    ))),
                ))
            }(&value_.object)?,
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
/// dualStringMatch MATCHING-RULE ::= {
///   SYNTAX  DualStringSyntax
///   ID      id-mr-dualStringMatch }
/// ```
///
///
pub fn dualStringMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_dualStringMatch(), /* OBJECT_FIELD_SETTING */
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
/// timeSpecification EXTENSION ::= {
///   SYNTAX         TimeSpecification
///   IDENTIFIED BY  id-ce-timeSpecification }
/// ```
///
///
pub fn timeSpecification() -> EXTENSION {
    EXTENSION {
        id: id_ce_timeSpecification(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// timeSpecificationMatch MATCHING-RULE ::= {
///   SYNTAX  TimeSpecification
///   ID      id-mr-timeSpecMatch }
/// ```
///
///
pub fn timeSpecificationMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_timeSpecMatch(), /* OBJECT_FIELD_SETTING */
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
/// targetingInformation EXTENSION ::= {
///   SYNTAX         SEQUENCE SIZE (1..MAX) OF Targets
///   IDENTIFIED BY  id-ce-targetingInformation }
/// ```
///
///
pub fn targetingInformation() -> EXTENSION {
    EXTENSION {
        id: id_ce_targetingInformation(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Targets  ::=  SEQUENCE SIZE (1..MAX) OF Target
/// ```
pub type Targets = Vec<Target>; // SequenceOfType

pub fn _decode_Targets(el: &X690Element) -> ASN1Result<Targets> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Target>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Target> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Target(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Targets(value_: &Targets) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Target>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Target(&v)?);
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
/// Target  ::=  CHOICE {
///   targetName   [0]  GeneralName,
///   targetGroup  [1]  GeneralName,
///   targetCert   [2]  TargetCert,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Target {
    targetName(GeneralName),
    targetGroup(GeneralName),
    targetCert(TargetCert),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Target {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Target(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Target {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Target(el)
    }
}

pub fn _decode_Target(el: &X690Element) -> ASN1Result<Target> {
    |el: &X690Element| -> ASN1Result<Target> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Target::targetName(
                |el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(Target::targetGroup(
                |el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(Target::targetCert(_decode_TargetCert(&el)?)),
            _ => Ok(Target::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Target(value_: &Target) -> ASN1Result<X690Element> {
    |value: &Target| -> ASN1Result<X690Element> {
        match value {
            Target::targetName(v) => |v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v),
            Target::targetGroup(v) => |v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v),
            Target::targetCert(v) => |v_1: &TargetCert| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_TargetCert(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            Target::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TargetCert ::= SEQUENCE {
///   targetCertificate  IssuerSerial,
///   targetName         GeneralName OPTIONAL,
///   certDigestInfo     ObjectDigestInfo OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TargetCert {
    pub targetCertificate: IssuerSerial,
    pub targetName: OPTIONAL<GeneralName>,
    pub certDigestInfo: OPTIONAL<ObjectDigestInfo>,
}
impl TargetCert {
    pub fn new(
        targetCertificate: IssuerSerial,
        targetName: OPTIONAL<GeneralName>,
        certDigestInfo: OPTIONAL<ObjectDigestInfo>,
    ) -> Self {
        TargetCert {
            targetCertificate,
            targetName,
            certDigestInfo,
        }
    }
}
impl TryFrom<X690Element> for TargetCert {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TargetCert(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TargetCert {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TargetCert(el)
    }
}

pub const _rctl1_components_for_TargetCert: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "targetCertificate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "targetName",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
            &TagSelector::tag((TagClass::CONTEXT, 2)),
            &TagSelector::tag((TagClass::CONTEXT, 3)),
            &TagSelector::tag((TagClass::CONTEXT, 4)),
            &TagSelector::tag((TagClass::CONTEXT, 5)),
            &TagSelector::tag((TagClass::CONTEXT, 6)),
            &TagSelector::tag((TagClass::CONTEXT, 7)),
            &TagSelector::tag((TagClass::CONTEXT, 8)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "certDigestInfo",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TargetCert: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TargetCert: &[ComponentSpec; 0] = &[];

pub fn _decode_TargetCert(el: &X690Element) -> ASN1Result<TargetCert> {
    |el_: &X690Element| -> ASN1Result<TargetCert> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TargetCert,
            _eal_components_for_TargetCert,
            _rctl2_components_for_TargetCert,
        )?;
        let targetCertificate =
            _decode_IssuerSerial(_components.get("targetCertificate").unwrap())?;
        let targetName: OPTIONAL<GeneralName> = match _components.get("targetName") {
            Some(c_) => Some(_decode_GeneralName(c_)?),
            _ => None,
        };
        let certDigestInfo: OPTIONAL<ObjectDigestInfo> = match _components.get("certDigestInfo") {
            Some(c_) => Some(_decode_ObjectDigestInfo(c_)?),
            _ => None,
        };
        Ok(TargetCert {
            targetCertificate,
            targetName,
            certDigestInfo,
        })
    }(&el)
}

pub fn _encode_TargetCert(value_: &TargetCert) -> ASN1Result<X690Element> {
    |value_: &TargetCert| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_IssuerSerial(&value_.targetCertificate)?);
        if let Some(v_) = &value_.targetName {
            components_.push(_encode_GeneralName(&v_)?);
        }
        if let Some(v_) = &value_.certDigestInfo {
            components_.push(_encode_ObjectDigestInfo(&v_)?);
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
/// userNotice EXTENSION ::= {
///   SYNTAX         SEQUENCE SIZE (1..MAX) OF UserNotice
///   IDENTIFIED BY  id-ce-userNotice }
/// ```
///
///
pub fn userNotice() -> EXTENSION {
    EXTENSION {
        id: id_ce_userNotice(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserNotice ::= SEQUENCE {
///   noticeRef     NoticeReference OPTIONAL,
///   explicitText  DisplayText OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UserNotice {
    pub noticeRef: OPTIONAL<NoticeReference>,
    pub explicitText: OPTIONAL<DisplayText>,
}
impl UserNotice {
    pub fn new(noticeRef: OPTIONAL<NoticeReference>, explicitText: OPTIONAL<DisplayText>) -> Self {
        UserNotice {
            noticeRef,
            explicitText,
        }
    }
}
impl Default for UserNotice {
    fn default() -> Self {
        UserNotice {
            noticeRef: None,
            explicitText: None,
        }
    }
}
impl TryFrom<X690Element> for UserNotice {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UserNotice(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UserNotice {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UserNotice(el)
    }
}

pub const _rctl1_components_for_UserNotice: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "noticeRef",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "explicitText",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 26)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UserNotice: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UserNotice: &[ComponentSpec; 0] = &[];

pub fn _decode_UserNotice(el: &X690Element) -> ASN1Result<UserNotice> {
    |el_: &X690Element| -> ASN1Result<UserNotice> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UserNotice,
            _eal_components_for_UserNotice,
            _rctl2_components_for_UserNotice,
        )?;
        let noticeRef: OPTIONAL<NoticeReference> = match _components.get("noticeRef") {
            Some(c_) => Some(_decode_NoticeReference(c_)?),
            _ => None,
        };
        let explicitText: OPTIONAL<DisplayText> = match _components.get("explicitText") {
            Some(c_) => Some(_decode_DisplayText(c_)?),
            _ => None,
        };
        Ok(UserNotice {
            noticeRef,
            explicitText,
        })
    }(&el)
}

pub fn _encode_UserNotice(value_: &UserNotice) -> ASN1Result<X690Element> {
    |value_: &UserNotice| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.noticeRef {
            components_.push(_encode_NoticeReference(&v_)?);
        }
        if let Some(v_) = &value_.explicitText {
            components_.push(_encode_DisplayText(&v_)?);
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
/// NoticeReference ::= SEQUENCE {
///   organization   DisplayText,
///   noticeNumbers  SEQUENCE OF INTEGER }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NoticeReference {
    pub organization: DisplayText,
    pub noticeNumbers: Vec<INTEGER>,
}
impl NoticeReference {
    pub fn new(organization: DisplayText, noticeNumbers: Vec<INTEGER>) -> Self {
        NoticeReference {
            organization,
            noticeNumbers,
        }
    }
}
impl TryFrom<X690Element> for NoticeReference {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NoticeReference(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NoticeReference {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NoticeReference(el)
    }
}

pub const _rctl1_components_for_NoticeReference: &[ComponentSpec; 2] = &[
    ComponentSpec::new("organization", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "noticeNumbers",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NoticeReference: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NoticeReference: &[ComponentSpec; 0] = &[];

pub fn _decode_NoticeReference(el: &X690Element) -> ASN1Result<NoticeReference> {
    |el_: &X690Element| -> ASN1Result<NoticeReference> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NoticeReference,
            _eal_components_for_NoticeReference,
            _rctl2_components_for_NoticeReference,
        )?;
        let organization = _decode_DisplayText(_components.get("organization").unwrap())?;
        let noticeNumbers = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<INTEGER>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<INTEGER> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(ber_decode_integer(el)?);
            }
            Ok(items)
        }(_components.get("noticeNumbers").unwrap())?;
        Ok(NoticeReference {
            organization,
            noticeNumbers,
        })
    }(&el)
}

pub fn _encode_NoticeReference(value_: &NoticeReference) -> ASN1Result<X690Element> {
    |value_: &NoticeReference| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_DisplayText(&value_.organization)?);
        components_.push(|value_: &SEQUENCE_OF<INTEGER>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(ber_encode_integer(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&value_.noticeNumbers)?);
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
/// DisplayText  ::=  CHOICE {
///   visibleString  VisibleString(SIZE (1..200)),
///   bmpString      BMPString(SIZE (1..200)),
///   utf8String     UTF8String(SIZE (1..200)) }
/// ```
#[derive(Debug, Clone)]
pub enum DisplayText {
    visibleString(VisibleString),
    bmpString(BMPString),
    utf8String(UTF8String),
}

impl TryFrom<X690Element> for DisplayText {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DisplayText(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DisplayText {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DisplayText(el)
    }
}

pub fn _decode_DisplayText(el: &X690Element) -> ASN1Result<DisplayText> {
    |el: &X690Element| -> ASN1Result<DisplayText> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 26) => {
                Ok(DisplayText::visibleString(ber_decode_visible_string(&el)?))
            }
            (TagClass::UNIVERSAL, 30) => Ok(DisplayText::bmpString(ber_decode_bmp_string(&el)?)),
            (TagClass::UNIVERSAL, 12) => Ok(DisplayText::utf8String(ber_decode_utf8_string(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_DisplayText(value_: &DisplayText) -> ASN1Result<X690Element> {
    |value: &DisplayText| -> ASN1Result<X690Element> {
        match value {
            DisplayText::visibleString(v) => ber_encode_visible_string(&v),
            DisplayText::bmpString(v) => ber_encode_bmp_string(&v),
            DisplayText::utf8String(v) => ber_encode_utf8_string(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// acceptablePrivilegePolicies EXTENSION ::= {
///   SYNTAX         AcceptablePrivilegePoliciesSyntax
///   IDENTIFIED BY  id-ce-acceptablePrivilegePolicies }
/// ```
///
///
pub fn acceptablePrivilegePolicies() -> EXTENSION {
    EXTENSION {
        id: id_ce_acceptablePrivilegePolicies(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AcceptablePrivilegePoliciesSyntax  ::=  SEQUENCE SIZE (1..MAX) OF PrivilegePolicy
/// ```
pub type AcceptablePrivilegePoliciesSyntax = Vec<PrivilegePolicy>; // SequenceOfType

pub fn _decode_AcceptablePrivilegePoliciesSyntax(
    el: &X690Element,
) -> ASN1Result<AcceptablePrivilegePoliciesSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PrivilegePolicy>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<PrivilegePolicy> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_PrivilegePolicy(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AcceptablePrivilegePoliciesSyntax(
    value_: &AcceptablePrivilegePoliciesSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<PrivilegePolicy>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_PrivilegePolicy(&v)?);
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
/// singleUse EXTENSION ::= {
///   SYNTAX         NULL
///   IDENTIFIED BY  id-ce-singleUse }
/// ```
///
///
pub fn singleUse() -> EXTENSION {
    EXTENSION {
        id: id_ce_singleUse(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// groupAC EXTENSION ::= {
///   SYNTAX         NULL
///   IDENTIFIED BY  id-ce-groupAC }
/// ```
///
///
pub fn groupAC() -> EXTENSION {
    EXTENSION {
        id: id_ce_groupAC(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// noRevAvail EXTENSION ::= {
///   SYNTAX         NULL
///   IDENTIFIED BY  id-ce-noRevAvail }
/// ```
///
///
pub fn noRevAvail() -> EXTENSION {
    EXTENSION {
        id: id_ce_noRevAvail(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sOAIdentifier EXTENSION ::= {
///   SYNTAX         NULL
///   IDENTIFIED BY  id-ce-sOAIdentifier }
/// ```
///
///
pub fn sOAIdentifier() -> EXTENSION {
    EXTENSION {
        id: id_ce_sOAIdentifier(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sOAIdentifierMatch MATCHING-RULE ::= {
///   SYNTAX  NULL
///   ID      id-mr-sOAIdentifierMatch }
/// ```
///
///
pub fn sOAIdentifierMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_sOAIdentifierMatch(), /* OBJECT_FIELD_SETTING */
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
/// attributeDescriptor EXTENSION ::= {
///   SYNTAX         AttributeDescriptorSyntax
///   IDENTIFIED BY  {id-ce-attributeDescriptor} }
/// ```
///
///
pub fn attributeDescriptor() -> EXTENSION {
    EXTENSION {
        id: id_ce_attributeDescriptor(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeDescriptorSyntax ::= SEQUENCE {
///   identifier             AttributeIdentifier,
///   attributeSyntax        OCTET STRING(SIZE (1..MAX)),
///   name              [0]  AttributeName OPTIONAL,
///   description       [1]  AttributeDescription OPTIONAL,
///   dominationRule         PrivilegePolicyIdentifier,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeDescriptorSyntax {
    pub identifier: AttributeIdentifier,
    pub attributeSyntax: OCTET_STRING,
    pub name: OPTIONAL<AttributeName>,
    pub description: OPTIONAL<AttributeDescription>,
    pub dominationRule: PrivilegePolicyIdentifier,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeDescriptorSyntax {
    pub fn new(
        identifier: AttributeIdentifier,
        attributeSyntax: OCTET_STRING,
        name: OPTIONAL<AttributeName>,
        description: OPTIONAL<AttributeDescription>,
        dominationRule: PrivilegePolicyIdentifier,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeDescriptorSyntax {
            identifier,
            attributeSyntax,
            name,
            description,
            dominationRule,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeDescriptorSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeDescriptorSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeDescriptorSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeDescriptorSyntax(el)
    }
}

pub const _rctl1_components_for_AttributeDescriptorSyntax: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeSyntax",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dominationRule",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeDescriptorSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeDescriptorSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeDescriptorSyntax(
    el: &X690Element,
) -> ASN1Result<AttributeDescriptorSyntax> {
    |el_: &X690Element| -> ASN1Result<AttributeDescriptorSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeDescriptorSyntax,
            _eal_components_for_AttributeDescriptorSyntax,
            _rctl2_components_for_AttributeDescriptorSyntax,
        )?;
        let identifier = _decode_AttributeIdentifier(_components.get("identifier").unwrap())?;
        let attributeSyntax = ber_decode_octet_string(_components.get("attributeSyntax").unwrap())?;
        let name: OPTIONAL<AttributeName> = match _components.get("name") {
            Some(c_) => Some(_decode_AttributeName(c_)?),
            _ => None,
        };
        let description: OPTIONAL<AttributeDescription> = match _components.get("description") {
            Some(c_) => Some(_decode_AttributeDescription(c_)?),
            _ => None,
        };
        let dominationRule =
            _decode_PrivilegePolicyIdentifier(_components.get("dominationRule").unwrap())?;
        Ok(AttributeDescriptorSyntax {
            identifier,
            attributeSyntax,
            name,
            description,
            dominationRule,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeDescriptorSyntax(
    value_: &AttributeDescriptorSyntax,
) -> ASN1Result<X690Element> {
    |value_: &AttributeDescriptorSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_AttributeIdentifier(&value_.identifier)?);
        components_.push(ber_encode_octet_string(&value_.attributeSyntax)?);
        if let Some(v_) = &value_.name {
            components_.push(|v_1: &AttributeName| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeName(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.description {
            components_.push(|v_1: &AttributeDescription| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeDescription(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_PrivilegePolicyIdentifier(&value_.dominationRule)?);
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
/// AttributeIdentifier  ::=  ATTRIBUTE.&id({AttributeIDs})
/// ```
pub type AttributeIdentifier = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_AttributeIdentifier(el: &X690Element) -> ASN1Result<AttributeIdentifier> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_AttributeIdentifier(value_: &AttributeIdentifier) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeIDs ATTRIBUTE ::= {...}
/// ```
///
///
pub fn AttributeIDs() -> Vec<ATTRIBUTE> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeName  ::=  UTF8String(SIZE (1..MAX))
/// ```
pub type AttributeName = UTF8String; // UTF8String

pub fn _decode_AttributeName(el: &X690Element) -> ASN1Result<AttributeName> {
    ber_decode_utf8_string(&el)
}

pub fn _encode_AttributeName(value_: &AttributeName) -> ASN1Result<X690Element> {
    ber_encode_utf8_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeDescription  ::=  UTF8String(SIZE (1..MAX))
/// ```
pub type AttributeDescription = UTF8String; // UTF8String

pub fn _decode_AttributeDescription(el: &X690Element) -> ASN1Result<AttributeDescription> {
    ber_decode_utf8_string(&el)
}

pub fn _encode_AttributeDescription(value_: &AttributeDescription) -> ASN1Result<X690Element> {
    ber_encode_utf8_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PrivilegePolicyIdentifier ::= SEQUENCE {
///   privilegePolicy  PrivilegePolicy,
///   privPolSyntax    InfoSyntax,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PrivilegePolicyIdentifier {
    pub privilegePolicy: PrivilegePolicy,
    pub privPolSyntax: InfoSyntax,
    pub _unrecognized: Vec<X690Element>,
}
impl PrivilegePolicyIdentifier {
    pub fn new(
        privilegePolicy: PrivilegePolicy,
        privPolSyntax: InfoSyntax,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PrivilegePolicyIdentifier {
            privilegePolicy,
            privPolSyntax,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for PrivilegePolicyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PrivilegePolicyIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PrivilegePolicyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PrivilegePolicyIdentifier(el)
    }
}

pub const _rctl1_components_for_PrivilegePolicyIdentifier: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "privilegePolicy",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("privPolSyntax", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_PrivilegePolicyIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PrivilegePolicyIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_PrivilegePolicyIdentifier(
    el: &X690Element,
) -> ASN1Result<PrivilegePolicyIdentifier> {
    |el_: &X690Element| -> ASN1Result<PrivilegePolicyIdentifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PrivilegePolicyIdentifier,
            _eal_components_for_PrivilegePolicyIdentifier,
            _rctl2_components_for_PrivilegePolicyIdentifier,
        )?;
        let privilegePolicy = _decode_PrivilegePolicy(_components.get("privilegePolicy").unwrap())?;
        let privPolSyntax = _decode_InfoSyntax(_components.get("privPolSyntax").unwrap())?;
        Ok(PrivilegePolicyIdentifier {
            privilegePolicy,
            privPolSyntax,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PrivilegePolicyIdentifier(
    value_: &PrivilegePolicyIdentifier,
) -> ASN1Result<X690Element> {
    |value_: &PrivilegePolicyIdentifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_PrivilegePolicy(&value_.privilegePolicy)?);
        components_.push(_encode_InfoSyntax(&value_.privPolSyntax)?);
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
/// attDescriptor MATCHING-RULE ::= {
///   SYNTAX  AttributeDescriptorSyntax
///   ID      id-mr-attDescriptorMatch }
/// ```
///
///
pub fn attDescriptor() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_attDescriptorMatch(), /* OBJECT_FIELD_SETTING */
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
/// roleSpecCertIdentifier EXTENSION ::= {
///   SYNTAX         RoleSpecCertIdentifierSyntax
///   IDENTIFIED BY  {id-ce-roleSpecCertIdentifier} }
/// ```
///
///
pub fn roleSpecCertIdentifier() -> EXTENSION {
    EXTENSION {
        id: id_ce_roleSpecCertIdentifier(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RoleSpecCertIdentifierSyntax  ::=
///   SEQUENCE SIZE (1..MAX) OF RoleSpecCertIdentifier
/// ```
pub type RoleSpecCertIdentifierSyntax = Vec<RoleSpecCertIdentifier>; // SequenceOfType

pub fn _decode_RoleSpecCertIdentifierSyntax(
    el: &X690Element,
) -> ASN1Result<RoleSpecCertIdentifierSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<RoleSpecCertIdentifier>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<RoleSpecCertIdentifier> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_RoleSpecCertIdentifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RoleSpecCertIdentifierSyntax(
    value_: &RoleSpecCertIdentifierSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<RoleSpecCertIdentifier>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_RoleSpecCertIdentifier(&v)?);
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
/// RoleSpecCertIdentifier ::= SEQUENCE {
///   roleName              [0]  GeneralName,
///   roleCertIssuer        [1]  GeneralName,
///   roleCertSerialNumber  [2]  CertificateSerialNumber OPTIONAL,
///   roleCertLocator       [3]  GeneralNames OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RoleSpecCertIdentifier {
    pub roleName: GeneralName,
    pub roleCertIssuer: GeneralName,
    pub roleCertSerialNumber: OPTIONAL<CertificateSerialNumber>,
    pub roleCertLocator: OPTIONAL<GeneralNames>,
    pub _unrecognized: Vec<X690Element>,
}
impl RoleSpecCertIdentifier {
    pub fn new(
        roleName: GeneralName,
        roleCertIssuer: GeneralName,
        roleCertSerialNumber: OPTIONAL<CertificateSerialNumber>,
        roleCertLocator: OPTIONAL<GeneralNames>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RoleSpecCertIdentifier {
            roleName,
            roleCertIssuer,
            roleCertSerialNumber,
            roleCertLocator,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for RoleSpecCertIdentifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RoleSpecCertIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RoleSpecCertIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RoleSpecCertIdentifier(el)
    }
}

pub const _rctl1_components_for_RoleSpecCertIdentifier: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "roleName",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "roleCertIssuer",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "roleCertSerialNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "roleCertLocator",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RoleSpecCertIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RoleSpecCertIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_RoleSpecCertIdentifier(el: &X690Element) -> ASN1Result<RoleSpecCertIdentifier> {
    |el_: &X690Element| -> ASN1Result<RoleSpecCertIdentifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RoleSpecCertIdentifier,
            _eal_components_for_RoleSpecCertIdentifier,
            _rctl2_components_for_RoleSpecCertIdentifier,
        )?;
        let roleName = |el: &X690Element| -> ASN1Result<GeneralName> {
            Ok(_decode_GeneralName(&el.inner()?)?)
        }(_components.get("roleName").unwrap())?;
        let roleCertIssuer = |el: &X690Element| -> ASN1Result<GeneralName> {
            Ok(_decode_GeneralName(&el.inner()?)?)
        }(_components.get("roleCertIssuer").unwrap())?;
        let roleCertSerialNumber: OPTIONAL<CertificateSerialNumber> =
            match _components.get("roleCertSerialNumber") {
                Some(c_) => Some(_decode_CertificateSerialNumber(c_)?),
                _ => None,
            };
        let roleCertLocator: OPTIONAL<GeneralNames> = match _components.get("roleCertLocator") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        Ok(RoleSpecCertIdentifier {
            roleName,
            roleCertIssuer,
            roleCertSerialNumber,
            roleCertLocator,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RoleSpecCertIdentifier(value_: &RoleSpecCertIdentifier) -> ASN1Result<X690Element> {
    |value_: &RoleSpecCertIdentifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
            ))
        }(&value_.roleName)?);
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
            ))
        }(&value_.roleCertIssuer)?);
        if let Some(v_) = &value_.roleCertSerialNumber {
            components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.roleCertLocator {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
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
/// roleSpecCertIdMatch MATCHING-RULE ::= {
///   SYNTAX  RoleSpecCertIdentifierSyntax
///   ID      id-mr-roleSpecCertIdMatch }
/// ```
///
///
pub fn roleSpecCertIdMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_roleSpecCertIdMatch(), /* OBJECT_FIELD_SETTING */
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
/// basicAttConstraints EXTENSION ::= {
///   SYNTAX         BasicAttConstraintsSyntax
///   IDENTIFIED BY  {id-ce-basicAttConstraints} }
/// ```
///
///
pub fn basicAttConstraints() -> EXTENSION {
    EXTENSION {
        id: id_ce_basicAttConstraints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BasicAttConstraintsSyntax ::= SEQUENCE {
///   authority          BOOLEAN DEFAULT FALSE,
///   pathLenConstraint  INTEGER(0..MAX) OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BasicAttConstraintsSyntax {
    pub authority: OPTIONAL<BOOLEAN>,
    pub pathLenConstraint: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl BasicAttConstraintsSyntax {
    pub fn new(
        authority: OPTIONAL<BOOLEAN>,
        pathLenConstraint: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        BasicAttConstraintsSyntax {
            authority,
            pathLenConstraint,
            _unrecognized,
        }
    }
    pub fn _default_value_for_authority() -> BOOLEAN {
        false
    }
}
impl Default for BasicAttConstraintsSyntax {
    fn default() -> Self {
        BasicAttConstraintsSyntax {
            authority: None,
            pathLenConstraint: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for BasicAttConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BasicAttConstraintsSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BasicAttConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BasicAttConstraintsSyntax(el)
    }
}

pub const _rctl1_components_for_BasicAttConstraintsSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "authority",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pathLenConstraint",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BasicAttConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BasicAttConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_BasicAttConstraintsSyntax(
    el: &X690Element,
) -> ASN1Result<BasicAttConstraintsSyntax> {
    |el_: &X690Element| -> ASN1Result<BasicAttConstraintsSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BasicAttConstraintsSyntax,
            _eal_components_for_BasicAttConstraintsSyntax,
            _rctl2_components_for_BasicAttConstraintsSyntax,
        )?;
        let authority: OPTIONAL<BOOLEAN> = match _components.get("authority") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let pathLenConstraint: OPTIONAL<INTEGER> = match _components.get("pathLenConstraint") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        Ok(BasicAttConstraintsSyntax {
            authority,
            pathLenConstraint,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_BasicAttConstraintsSyntax(
    value_: &BasicAttConstraintsSyntax,
) -> ASN1Result<X690Element> {
    |value_: &BasicAttConstraintsSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.authority {
            if *v_ != BasicAttConstraintsSyntax::_default_value_for_authority() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.pathLenConstraint {
            components_.push(ber_encode_integer(&v_)?);
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
/// basicAttConstraintsMatch MATCHING-RULE ::= {
///   SYNTAX  BasicAttConstraintsSyntax
///   ID      id-mr-basicAttConstraintsMatch }
/// ```
///
///
pub fn basicAttConstraintsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_basicAttConstraintsMatch(), /* OBJECT_FIELD_SETTING */
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
/// delegatedNameConstraints EXTENSION ::= {
///   SYNTAX         NameConstraintsSyntax
///   IDENTIFIED BY  id-ce-delegatedNameConstraints }
/// ```
///
///
pub fn delegatedNameConstraints() -> EXTENSION {
    EXTENSION {
        id: id_ce_delegatedNameConstraints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// delegatedNameConstraintsMatch MATCHING-RULE ::= {
///   SYNTAX  NameConstraintsSyntax
///   ID      id-mr-delegatedNameConstraintsMatch }
/// ```
///
///
pub fn delegatedNameConstraintsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_delegatedNameConstraintsMatch(), /* OBJECT_FIELD_SETTING */
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
/// acceptableCertPolicies EXTENSION ::= {
///   SYNTAX         AcceptableCertPoliciesSyntax
///   IDENTIFIED BY  id-ce-acceptableCertPolicies }
/// ```
///
///
pub fn acceptableCertPolicies() -> EXTENSION {
    EXTENSION {
        id: id_ce_acceptableCertPolicies(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AcceptableCertPoliciesSyntax  ::=  SEQUENCE SIZE (1..MAX) OF CertPolicyId
/// ```
pub type AcceptableCertPoliciesSyntax = Vec<CertPolicyId>; // SequenceOfType

pub fn _decode_AcceptableCertPoliciesSyntax(
    el: &X690Element,
) -> ASN1Result<AcceptableCertPoliciesSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertPolicyId>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertPolicyId> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertPolicyId(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AcceptableCertPoliciesSyntax(
    value_: &AcceptableCertPoliciesSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertPolicyId>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertPolicyId(&v)?);
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
/// CertPolicyId  ::=  OBJECT IDENTIFIER
/// ```
pub type CertPolicyId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CertPolicyId(el: &X690Element) -> ASN1Result<CertPolicyId> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_CertPolicyId(value_: &CertPolicyId) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// acceptableCertPoliciesMatch MATCHING-RULE ::= {
///   SYNTAX  AcceptableCertPoliciesSyntax
///   ID      id-mr-acceptableCertPoliciesMatch }
/// ```
///
///
pub fn acceptableCertPoliciesMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_acceptableCertPoliciesMatch(), /* OBJECT_FIELD_SETTING */
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
/// authorityAttributeIdentifier EXTENSION ::= {
///   SYNTAX         AuthorityAttributeIdentifierSyntax
///   IDENTIFIED BY  {id-ce-authorityAttributeIdentifier} }
/// ```
///
///
pub fn authorityAttributeIdentifier() -> EXTENSION {
    EXTENSION {
        id: id_ce_authorityAttributeIdentifier(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthorityAttributeIdentifierSyntax  ::=  SEQUENCE SIZE (1..MAX) OF AuthAttId
/// ```
pub type AuthorityAttributeIdentifierSyntax = Vec<AuthAttId>; // SequenceOfType

pub fn _decode_AuthorityAttributeIdentifierSyntax(
    el: &X690Element,
) -> ASN1Result<AuthorityAttributeIdentifierSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AuthAttId>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AuthAttId> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AuthAttId(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AuthorityAttributeIdentifierSyntax(
    value_: &AuthorityAttributeIdentifierSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AuthAttId>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AuthAttId(&v)?);
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
/// AuthAttId  ::=  IssuerSerial
/// ```
pub type AuthAttId = IssuerSerial; // DefinedType

pub fn _decode_AuthAttId(el: &X690Element) -> ASN1Result<AuthAttId> {
    _decode_IssuerSerial(&el)
}

pub fn _encode_AuthAttId(value_: &AuthAttId) -> ASN1Result<X690Element> {
    _encode_IssuerSerial(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// authAttIdMatch MATCHING-RULE ::= {
///   SYNTAX  AuthorityAttributeIdentifierSyntax
///   ID      id-mr-authAttIdMatch }
/// ```
///
///
pub fn authAttIdMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_authAttIdMatch(), /* OBJECT_FIELD_SETTING */
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
/// indirectIssuer EXTENSION ::= {
///   SYNTAX         NULL
///   IDENTIFIED BY  id-ce-indirectIssuer }
/// ```
///
///
pub fn indirectIssuer() -> EXTENSION {
    EXTENSION {
        id: id_ce_indirectIssuer(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// issuedOnBehalfOf EXTENSION ::= {
///   SYNTAX         GeneralName
///   IDENTIFIED BY  id-ce-issuedOnBehalfOf }
/// ```
///
///
pub fn issuedOnBehalfOf() -> EXTENSION {
    EXTENSION {
        id: id_ce_issuedOnBehalfOf(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// noAssertion EXTENSION ::= {
///   SYNTAX         NULL
///   IDENTIFIED BY  id-ce-noAssertion }
/// ```
///
///
pub fn noAssertion() -> EXTENSION {
    EXTENSION {
        id: id_ce_noAssertion(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// allowedAttributeAssignments EXTENSION ::= {
///   SYNTAX         AllowedAttributeAssignments
///   IDENTIFIED BY  id-ce-allowedAttributeAssignments }
/// ```
///
///
pub fn allowedAttributeAssignments() -> EXTENSION {
    EXTENSION {
        id: id_ce_allowedAttributeAssignments(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AllowedAttributeAssignments  ::=  SET OF SEQUENCE {
///   attributes              [0]  SET OF CHOICE {
///     attributeType           [0]  AttributeType,
///     attributeTypeandValues  [1]  Attribute{{SupportedAttributes}},
///     ... },
///   holderDomain            [1]  GeneralName,
///   ... }
/// ```
pub type AllowedAttributeAssignments = Vec<AllowedAttributeAssignments_Item>; // SetOfType

pub fn _decode_AllowedAttributeAssignments(
    el: &X690Element,
) -> ASN1Result<AllowedAttributeAssignments> {
    |el: &X690Element| -> ASN1Result<SET_OF<AllowedAttributeAssignments_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<AllowedAttributeAssignments_Item> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AllowedAttributeAssignments_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AllowedAttributeAssignments(
    value_: &AllowedAttributeAssignments,
) -> ASN1Result<X690Element> {
    |value_: &SET_OF<AllowedAttributeAssignments_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AllowedAttributeAssignments_Item(&v)?);
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
/// attributeMappings EXTENSION ::= {
///   SYNTAX         AttributeMappings
///   IDENTIFIED BY  id-ce-attributeMappings }
/// ```
///
///
pub fn attributeMappings() -> EXTENSION {
    EXTENSION {
        id: id_ce_attributeMappings(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeMappings  ::=  SET OF CHOICE {
///   typeMappings      [0]  SEQUENCE {
///     local             [0]  AttributeType,
///     remote            [1]  AttributeType,
///     ... },
///   typeValueMappings [1]  SEQUENCE {
///     local             [0]  AttributeTypeAndValue,
///     remote            [1]  AttributeTypeAndValue,
///     ... } }
/// ```
pub type AttributeMappings = Vec<AttributeMappings_Item>; // SetOfType

pub fn _decode_AttributeMappings(el: &X690Element) -> ASN1Result<AttributeMappings> {
    |el: &X690Element| -> ASN1Result<SET_OF<AttributeMappings_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<AttributeMappings_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AttributeMappings_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttributeMappings(value_: &AttributeMappings) -> ASN1Result<X690Element> {
    |value_: &SET_OF<AttributeMappings_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AttributeMappings_Item(&v)?);
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
/// holderNameConstraints EXTENSION ::= {
///   SYNTAX         HolderNameConstraintsSyntax
///   IDENTIFIED BY  id-ce-holderNameConstraints }
/// ```
///
///
pub fn holderNameConstraints() -> EXTENSION {
    EXTENSION {
        id: id_ce_holderNameConstraints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HolderNameConstraintsSyntax ::= SEQUENCE {
///   permittedSubtrees  [0]  GeneralSubtrees,
///   excludedSubtrees   [1]  GeneralSubtrees OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct HolderNameConstraintsSyntax {
    pub permittedSubtrees: GeneralSubtrees,
    pub excludedSubtrees: OPTIONAL<GeneralSubtrees>,
    pub _unrecognized: Vec<X690Element>,
}
impl HolderNameConstraintsSyntax {
    pub fn new(
        permittedSubtrees: GeneralSubtrees,
        excludedSubtrees: OPTIONAL<GeneralSubtrees>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        HolderNameConstraintsSyntax {
            permittedSubtrees,
            excludedSubtrees,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for HolderNameConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_HolderNameConstraintsSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for HolderNameConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_HolderNameConstraintsSyntax(el)
    }
}

pub const _rctl1_components_for_HolderNameConstraintsSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "permittedSubtrees",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "excludedSubtrees",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_HolderNameConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_HolderNameConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_HolderNameConstraintsSyntax(
    el: &X690Element,
) -> ASN1Result<HolderNameConstraintsSyntax> {
    |el_: &X690Element| -> ASN1Result<HolderNameConstraintsSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_HolderNameConstraintsSyntax,
            _eal_components_for_HolderNameConstraintsSyntax,
            _rctl2_components_for_HolderNameConstraintsSyntax,
        )?;
        let permittedSubtrees =
            _decode_GeneralSubtrees(_components.get("permittedSubtrees").unwrap())?;
        let excludedSubtrees: OPTIONAL<GeneralSubtrees> = match _components.get("excludedSubtrees")
        {
            Some(c_) => Some(_decode_GeneralSubtrees(c_)?),
            _ => None,
        };
        Ok(HolderNameConstraintsSyntax {
            permittedSubtrees,
            excludedSubtrees,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_HolderNameConstraintsSyntax(
    value_: &HolderNameConstraintsSyntax,
) -> ASN1Result<X690Element> {
    |value_: &HolderNameConstraintsSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.permittedSubtrees)?);
        if let Some(v_) = &value_.excludedSubtrees {
            components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
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
/// GeneralSubtrees  ::=  SEQUENCE SIZE (1..MAX) OF GeneralSubtree
/// ```
pub type GeneralSubtrees = Vec<GeneralSubtree>; // SequenceOfType

pub fn _decode_GeneralSubtrees(el: &X690Element) -> ASN1Result<GeneralSubtrees> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<GeneralSubtree>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<GeneralSubtree> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_GeneralSubtree(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_GeneralSubtrees(value_: &GeneralSubtrees) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<GeneralSubtree>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_GeneralSubtree(&v)?);
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
/// GeneralSubtree ::= SEQUENCE {
///   base          GeneralName,
///   minimum  [0]  BaseDistance DEFAULT 0,
///   maximum  [1]  BaseDistance OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct GeneralSubtree {
    pub base: GeneralName,
    pub minimum: OPTIONAL<BaseDistance>,
    pub maximum: OPTIONAL<BaseDistance>,
    pub _unrecognized: Vec<X690Element>,
}
impl GeneralSubtree {
    pub fn new(
        base: GeneralName,
        minimum: OPTIONAL<BaseDistance>,
        maximum: OPTIONAL<BaseDistance>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        GeneralSubtree {
            base,
            minimum,
            maximum,
            _unrecognized,
        }
    }
    pub fn _default_value_for_minimum() -> BaseDistance {
        0
    }
}
impl TryFrom<X690Element> for GeneralSubtree {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralSubtree(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for GeneralSubtree {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralSubtree(el)
    }
}

pub const _rctl1_components_for_GeneralSubtree: &[ComponentSpec; 3] = &[
    ComponentSpec::new("base", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "minimum",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maximum",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_GeneralSubtree: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_GeneralSubtree: &[ComponentSpec; 0] = &[];

pub fn _decode_GeneralSubtree(el: &X690Element) -> ASN1Result<GeneralSubtree> {
    |el_: &X690Element| -> ASN1Result<GeneralSubtree> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_GeneralSubtree,
            _eal_components_for_GeneralSubtree,
            _rctl2_components_for_GeneralSubtree,
        )?;
        let base = _decode_GeneralName(_components.get("base").unwrap())?;
        let minimum: OPTIONAL<BaseDistance> = match _components.get("minimum") {
            Some(c_) => Some(_decode_BaseDistance(c_)?),
            _ => None,
        };
        let maximum: OPTIONAL<BaseDistance> = match _components.get("maximum") {
            Some(c_) => Some(_decode_BaseDistance(c_)?),
            _ => None,
        };
        Ok(GeneralSubtree {
            base,
            minimum,
            maximum,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_GeneralSubtree(value_: &GeneralSubtree) -> ASN1Result<X690Element> {
    |value_: &GeneralSubtree| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_GeneralName(&value_.base)?);
        if let Some(v_) = &value_.minimum {
            if *v_ != GeneralSubtree::_default_value_for_minimum() {
                components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_BaseDistance(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.maximum {
            components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_BaseDistance(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
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
/// BaseDistance  ::=  INTEGER(0..MAX)
/// ```
pub type BaseDistance = INTEGER;

pub fn _decode_BaseDistance(el: &X690Element) -> ASN1Result<BaseDistance> {
    ber_decode_integer(&el)
}

pub fn _encode_BaseDistance(value_: &BaseDistance) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pmiUser OBJECT-CLASS ::= {
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {attributeCertificateAttribute}
///   ID           id-oc-pmiUser }
/// ```
///
///
pub fn pmiUser() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([attributeCertificateAttribute()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiUser(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pmiAA OBJECT-CLASS ::= { -- a PMI AA
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {aACertificate |
///                 attributeCertificateRevocationList |
///                 eeAttrCertificateRevocationList |
///                 attributeAuthorityRevocationList}
///   ID           id-oc-pmiAA }
/// ```
///
///
pub fn pmiAA() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
            aACertificate(),
            attributeCertificateRevocationList(),
            eeAttrCertificateRevocationList(),
            attributeAuthorityRevocationList(),
        ])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiAA(),                           /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pmiSOA OBJECT-CLASS ::= { -- a PMI Source of Authority
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {attributeCertificateRevocationList |
///                 eeAttrCertificateRevocationList |
///                 attributeAuthorityRevocationList |
///                 attributeDescriptorCertificate}
///   ID           id-oc-pmiSOA }
/// ```
///
///
pub fn pmiSOA() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
            attributeCertificateRevocationList(),
            eeAttrCertificateRevocationList(),
            attributeAuthorityRevocationList(),
            attributeDescriptorCertificate(),
        ])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiSOA(),                          /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attCertCRLDistributionPt OBJECT-CLASS ::= {
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {attributeCertificateRevocationList |
///                 eeAttrCertificateRevocationList |
///                 attributeAuthorityRevocationList}
///   ID           id-oc-attCertCRLDistributionPts }
/// ```
///
///
pub fn attCertCRLDistributionPt() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
            attributeCertificateRevocationList(),
            eeAttrCertificateRevocationList(),
            attributeAuthorityRevocationList(),
        ])), /* OBJECT_FIELD_SETTING */
        id: id_oc_attCertCRLDistributionPts(),       /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pmiDelegationPath OBJECT-CLASS ::= {
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {delegationPath}
///   ID           id-oc-pmiDelegationPath }
/// ```
///
///
pub fn pmiDelegationPath() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([delegationPath()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiDelegationPath(),               /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// privilegePolicy OBJECT-CLASS ::= {
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {privPolicy}
///   ID           id-oc-privilegePolicy }
/// ```
///
///
pub fn privilegePolicy() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([privPolicy()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_privilegePolicy(),                 /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// protectedPrivilegePolicy OBJECT-CLASS ::= {
///   SUBCLASS OF  {top}
///   KIND         auxiliary
///   MAY CONTAIN  {protPrivPolicy}
///   ID           id-oc-protectedPrivilegePolicy }
/// ```
///
///
pub fn protectedPrivilegePolicy() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([protPrivPolicy()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_protectedPrivilegePolicy(),        /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeCertificateAttribute ATTRIBUTE ::= {
///   WITH SYNTAX             AttributeCertificate
///   EQUALITY MATCHING RULE  attributeCertificateExactMatch
///   ID                      id-at-attributeCertificate }
/// ```
///
///
pub fn attributeCertificateAttribute() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(attributeCertificateExactMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_attributeCertificate(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aACertificate ATTRIBUTE ::= {
///   WITH SYNTAX             AttributeCertificate
///   EQUALITY MATCHING RULE  attributeCertificateExactMatch
///   ID                      id-at-aACertificate }
/// ```
///
///
pub fn aACertificate() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(attributeCertificateExactMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_aACertificate(),                                        /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeDescriptorCertificate ATTRIBUTE ::= {
///   WITH SYNTAX             AttributeCertificate
///   EQUALITY MATCHING RULE  attributeCertificateExactMatch
///   ID                      id-at-attributeDescriptorCertificate }
/// ```
///
///
pub fn attributeDescriptorCertificate() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(attributeCertificateExactMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_attributeDescriptorCertificate(),                       /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeCertificateRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX             CertificateList
///   EQUALITY MATCHING RULE  certificateListExactMatch
///   LDAP-SYNTAX             x509CertificateList.&id
///   LDAP-NAME               {"AttrCertificateRevocationList"}
///   LDAP-DESC               "X.509 Attr certificate revocation list"
///   ID                      id-at-attributeCertificateRevocationList }
/// ```
///
///
pub fn attributeCertificateRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("AttrCertificateRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Attr certificate revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_attributeCertificateRevocationList(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// eeAttrCertificateRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX             CertificateList
///   EQUALITY MATCHING RULE  certificateListExactMatch
///   LDAP-SYNTAX             x509CertificateList.&id
///   LDAP-NAME               {"EEAttrCertificateRevocationList"}
///   LDAP-DESC               "X.509 EEAttr certificate revocation list"
///   ID                      id-at-eeAttrCertificateRevocationList }
/// ```
///
///
pub fn eeAttrCertificateRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("EEAttrCertificateRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 EEAttr certificate revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_eeAttrCertificateRevocationList(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeAuthorityRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX             CertificateList
///   EQUALITY MATCHING RULE  certificateListExactMatch
///   LDAP-SYNTAX             x509CertificateList.&id
///   LDAP-NAME               {"AACertificateRevocationList"}
///   LDAP-DESC               "X.509 AA certificate revocation list"
///   ID                      id-at-attributeAuthorityRevocationList }
/// ```
///
///
pub fn attributeAuthorityRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("AACertificateRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 AA certificate revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_attributeAuthorityRevocationList(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// delegationPath ATTRIBUTE ::= {
///   WITH SYNTAX  AttCertPath
///   ID           id-at-delegationPath }
/// ```
///
///
pub fn delegationPath() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_delegationPath(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertPath  ::=  SEQUENCE OF AttributeCertificate
/// ```
pub type AttCertPath = Vec<AttributeCertificate>; // SequenceOfType

pub fn _decode_AttCertPath(el: &X690Element) -> ASN1Result<AttCertPath> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeCertificate>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AttributeCertificate> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AttributeCertificate(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttCertPath(value_: &AttCertPath) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AttributeCertificate>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AttributeCertificate(&v)?);
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
/// privPolicy ATTRIBUTE ::= {
///   WITH SYNTAX  PolicySyntax
///   ID           id-at-privPolicy }
/// ```
///
///
pub fn privPolicy() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_privPolicy(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// protPrivPolicy ATTRIBUTE ::= {
///   WITH SYNTAX             AttributeCertificate
///   EQUALITY MATCHING RULE  attributeCertificateExactMatch
///   ID                      id-at-protPrivPolicy }
/// ```
///
///
pub fn protPrivPolicy() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(attributeCertificateExactMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_protPrivPolicy(),                                       /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// xmlPrivPolicy ATTRIBUTE ::= {
///   WITH SYNTAX  UTF8String -- XML-encoded privilege policy information
///   ID           id-at-xmlPrivPolicy }
/// ```
///
///
pub fn xmlPrivPolicy() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_xmlPrivPolicy(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeCertificateExactMatch MATCHING-RULE ::= {
///   SYNTAX  AttributeCertificateExactAssertion
///   ID      id-mr-attributeCertificateExactMatch }
/// ```
///
///
pub fn attributeCertificateExactMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_attributeCertificateExactMatch(), /* OBJECT_FIELD_SETTING */
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
/// AttributeCertificateExactAssertion ::= SEQUENCE {
///   serialNumber  CertificateSerialNumber,
///   issuer        AttCertIssuer,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeCertificateExactAssertion {
    pub serialNumber: CertificateSerialNumber,
    pub issuer: AttCertIssuer,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeCertificateExactAssertion {
    pub fn new(
        serialNumber: CertificateSerialNumber,
        issuer: AttCertIssuer,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeCertificateExactAssertion {
            serialNumber,
            issuer,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeCertificateExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateExactAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeCertificateExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateExactAssertion(el)
    }
}

pub const _rctl1_components_for_AttributeCertificateExactAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeCertificateExactAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeCertificateExactAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeCertificateExactAssertion(
    el: &X690Element,
) -> ASN1Result<AttributeCertificateExactAssertion> {
    |el_: &X690Element| -> ASN1Result<AttributeCertificateExactAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeCertificateExactAssertion,
            _eal_components_for_AttributeCertificateExactAssertion,
            _rctl2_components_for_AttributeCertificateExactAssertion,
        )?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let issuer = _decode_AttCertIssuer(_components.get("issuer").unwrap())?;
        Ok(AttributeCertificateExactAssertion {
            serialNumber,
            issuer,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeCertificateExactAssertion(
    value_: &AttributeCertificateExactAssertion,
) -> ASN1Result<X690Element> {
    |value_: &AttributeCertificateExactAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_AttCertIssuer(&value_.issuer)?);
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
/// attributeCertificateMatch MATCHING-RULE ::= {
///   SYNTAX  AttributeCertificateAssertion
///   ID      id-mr-attributeCertificateMatch }
/// ```
///
///
pub fn attributeCertificateMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_attributeCertificateMatch(), /* OBJECT_FIELD_SETTING */
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
/// AttributeCertificateAssertion ::= SEQUENCE {
///   holder             [0]  CHOICE {
///     baseCertificateID  [0]  IssuerSerial,
///     holderName         [1]  GeneralNames,
///     ...} OPTIONAL,
///   issuer             [1]  GeneralNames OPTIONAL,
///   attCertValidity    [2]  GeneralizedTime OPTIONAL,
///   attType            [3]  SET OF AttributeType OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeCertificateAssertion {
    pub holder: OPTIONAL<AttributeCertificateAssertion_holder>,
    pub issuer: OPTIONAL<GeneralNames>,
    pub attCertValidity: OPTIONAL<GeneralizedTime>,
    pub attType: OPTIONAL<Vec<AttributeType>>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeCertificateAssertion {
    pub fn new(
        holder: OPTIONAL<AttributeCertificateAssertion_holder>,
        issuer: OPTIONAL<GeneralNames>,
        attCertValidity: OPTIONAL<GeneralizedTime>,
        attType: OPTIONAL<Vec<AttributeType>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeCertificateAssertion {
            holder,
            issuer,
            attCertValidity,
            attType,
            _unrecognized,
        }
    }
}
impl Default for AttributeCertificateAssertion {
    fn default() -> Self {
        AttributeCertificateAssertion {
            holder: None,
            issuer: None,
            attCertValidity: None,
            attType: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AttributeCertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeCertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateAssertion(el)
    }
}

pub const _rctl1_components_for_AttributeCertificateAssertion: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "holder",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attCertValidity",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeCertificateAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeCertificateAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeCertificateAssertion(
    el: &X690Element,
) -> ASN1Result<AttributeCertificateAssertion> {
    |el_: &X690Element| -> ASN1Result<AttributeCertificateAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeCertificateAssertion,
            _eal_components_for_AttributeCertificateAssertion,
            _rctl2_components_for_AttributeCertificateAssertion,
        )?;
        let holder: OPTIONAL<AttributeCertificateAssertion_holder> = match _components.get("holder")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<AttributeCertificateAssertion_holder> {
                    Ok(_decode_AttributeCertificateAssertion_holder(&el.inner()?)?)
                }(c_)?,
            ),
            _ => None,
        };
        let issuer: OPTIONAL<GeneralNames> = match _components.get("issuer") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let attCertValidity: OPTIONAL<GeneralizedTime> = match _components.get("attCertValidity") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let attType: OPTIONAL<Vec<AttributeType>> = match _components.get("attType") {
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
        Ok(AttributeCertificateAssertion {
            holder,
            issuer,
            attCertValidity,
            attType,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeCertificateAssertion(
    value_: &AttributeCertificateAssertion,
) -> ASN1Result<X690Element> {
    |value_: &AttributeCertificateAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.holder {
            components_.push(
                |v_1: &AttributeCertificateAssertion_holder| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_AttributeCertificateAssertion_holder(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.issuer {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.attCertValidity {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.attType {
            components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
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
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
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
/// holderIssuerMatch MATCHING-RULE ::= {
///   SYNTAX  HolderIssuerAssertion
///   ID      id-mr-holderIssuerMatch }
/// ```
///
///
pub fn holderIssuerMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_holderIssuerMatch(), /* OBJECT_FIELD_SETTING */
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
/// HolderIssuerAssertion ::= SEQUENCE {
///   holder  [0]  Holder OPTIONAL,
///   issuer  [1]  AttCertIssuer OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct HolderIssuerAssertion {
    pub holder: OPTIONAL<Holder>,
    pub issuer: OPTIONAL<AttCertIssuer>,
    pub _unrecognized: Vec<X690Element>,
}
impl HolderIssuerAssertion {
    pub fn new(
        holder: OPTIONAL<Holder>,
        issuer: OPTIONAL<AttCertIssuer>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        HolderIssuerAssertion {
            holder,
            issuer,
            _unrecognized,
        }
    }
}
impl Default for HolderIssuerAssertion {
    fn default() -> Self {
        HolderIssuerAssertion {
            holder: None,
            issuer: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for HolderIssuerAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_HolderIssuerAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for HolderIssuerAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_HolderIssuerAssertion(el)
    }
}

pub const _rctl1_components_for_HolderIssuerAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "holder",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_HolderIssuerAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_HolderIssuerAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_HolderIssuerAssertion(el: &X690Element) -> ASN1Result<HolderIssuerAssertion> {
    |el_: &X690Element| -> ASN1Result<HolderIssuerAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_HolderIssuerAssertion,
            _eal_components_for_HolderIssuerAssertion,
            _rctl2_components_for_HolderIssuerAssertion,
        )?;
        let holder: OPTIONAL<Holder> = match _components.get("holder") {
            Some(c_) => Some(_decode_Holder(c_)?),
            _ => None,
        };
        let issuer: OPTIONAL<AttCertIssuer> = match _components.get("issuer") {
            Some(c_) => Some(_decode_AttCertIssuer(c_)?),
            _ => None,
        };
        Ok(HolderIssuerAssertion {
            holder,
            issuer,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_HolderIssuerAssertion(value_: &HolderIssuerAssertion) -> ASN1Result<X690Element> {
    |value_: &HolderIssuerAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.holder {
            components_.push(|v_1: &Holder| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Holder(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.issuer {
            components_.push(|v_1: &AttCertIssuer| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttCertIssuer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
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
/// delegationPathMatch MATCHING-RULE ::= {
///   SYNTAX  DelMatchSyntax
///   ID      id-mr-delegationPathMatch }
/// ```
///
///
pub fn delegationPathMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_delegationPathMatch(), /* OBJECT_FIELD_SETTING */
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
/// DelMatchSyntax ::= SEQUENCE {
///   firstIssuer  AttCertIssuer,
///   lastHolder   Holder,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DelMatchSyntax {
    pub firstIssuer: AttCertIssuer,
    pub lastHolder: Holder,
    pub _unrecognized: Vec<X690Element>,
}
impl DelMatchSyntax {
    pub fn new(
        firstIssuer: AttCertIssuer,
        lastHolder: Holder,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DelMatchSyntax {
            firstIssuer,
            lastHolder,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DelMatchSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DelMatchSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DelMatchSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DelMatchSyntax(el)
    }
}

pub const _rctl1_components_for_DelMatchSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "firstIssuer",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastHolder",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DelMatchSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DelMatchSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_DelMatchSyntax(el: &X690Element) -> ASN1Result<DelMatchSyntax> {
    |el_: &X690Element| -> ASN1Result<DelMatchSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DelMatchSyntax,
            _eal_components_for_DelMatchSyntax,
            _rctl2_components_for_DelMatchSyntax,
        )?;
        let firstIssuer = _decode_AttCertIssuer(_components.get("firstIssuer").unwrap())?;
        let lastHolder = _decode_Holder(_components.get("lastHolder").unwrap())?;
        Ok(DelMatchSyntax {
            firstIssuer,
            lastHolder,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DelMatchSyntax(value_: &DelMatchSyntax) -> ASN1Result<X690Element> {
    |value_: &DelMatchSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttCertIssuer(&value_.firstIssuer)?);
        components_.push(_encode_Holder(&value_.lastHolder)?);
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
/// extensionPresenceMatch MATCHING-RULE ::= {
///   SYNTAX  EXTENSION.&id
///   ID      id-mr-extensionPresenceMatch }
/// ```
///
///
pub fn extensionPresenceMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_extensionPresenceMatch(), /* OBJECT_FIELD_SETTING */
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
/// id-oc-pmiUser                            OBJECT IDENTIFIER ::= {id-oc 24}
/// ```
///
///
pub fn id_oc_pmiUser() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pmiAA                              OBJECT IDENTIFIER ::= {id-oc 25}
/// ```
///
///
pub fn id_oc_pmiAA() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pmiSOA                             OBJECT IDENTIFIER ::= {id-oc 26}
/// ```
///
///
pub fn id_oc_pmiSOA() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-attCertCRLDistributionPts          OBJECT IDENTIFIER ::= {id-oc 27}
/// ```
///
///
pub fn id_oc_attCertCRLDistributionPts() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-privilegePolicy                    OBJECT IDENTIFIER ::= {id-oc 32}
/// ```
///
///
pub fn id_oc_privilegePolicy() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pmiDelegationPath                  OBJECT IDENTIFIER ::= {id-oc 33}
/// ```
///
///
pub fn id_oc_pmiDelegationPath() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([33])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-protectedPrivilegePolicy           OBJECT IDENTIFIER ::= {id-oc 34}
/// ```
///
///
pub fn id_oc_protectedPrivilegePolicy() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([34])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeCertificate               OBJECT IDENTIFIER ::= {id-at 58}
/// ```
///
///
pub fn id_at_attributeCertificate() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([58])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeCertificateRevocationList OBJECT IDENTIFIER ::= {id-at 59}
/// ```
///
///
pub fn id_at_attributeCertificateRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([59])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-aACertificate                      OBJECT IDENTIFIER ::= {id-at 61}
/// ```
///
///
pub fn id_at_aACertificate() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([61])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeDescriptorCertificate     OBJECT IDENTIFIER ::= {id-at 62}
/// ```
///
///
pub fn id_at_attributeDescriptorCertificate() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([62])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeAuthorityRevocationList   OBJECT IDENTIFIER ::= {id-at 63}
/// ```
///
///
pub fn id_at_attributeAuthorityRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([63])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-privPolicy                         OBJECT IDENTIFIER ::= {id-at 71}
/// ```
///
///
pub fn id_at_privPolicy() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([71])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-role                               OBJECT IDENTIFIER ::= {id-at 72}
/// ```
///
///
pub fn id_at_role() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([72])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-delegationPath                     OBJECT IDENTIFIER ::= {id-at 73}
/// ```
///
///
pub fn id_at_delegationPath() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([73])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-protPrivPolicy                     OBJECT IDENTIFIER ::= {id-at 74}
/// ```
///
///
pub fn id_at_protPrivPolicy() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([74])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-xMLPrivilegeInfo                   OBJECT IDENTIFIER ::= {id-at 75}
/// ```
///
///
pub fn id_at_xMLPrivilegeInfo() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([75])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-xmlPrivPolicy                      OBJECT IDENTIFIER ::= {id-at 76}
/// ```
///
///
pub fn id_at_xmlPrivPolicy() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([76])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-permission                         OBJECT IDENTIFIER ::= {id-at 82}
/// ```
///
///
pub fn id_at_permission() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([82])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-eeAttrCertificateRevocationList    OBJECT IDENTIFIER ::= {id-at 102}
/// ```
///
///
pub fn id_at_eeAttrCertificateRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([102])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-authorityAttributeIdentifier       OBJECT IDENTIFIER ::= {id-ce 38}
/// ```
///
///
pub fn id_ce_authorityAttributeIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([38])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-roleSpecCertIdentifier             OBJECT IDENTIFIER ::= {id-ce 39}
/// ```
///
///
pub fn id_ce_roleSpecCertIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([39])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-basicAttConstraints                OBJECT IDENTIFIER ::= {id-ce 41}
/// ```
///
///
pub fn id_ce_basicAttConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([41])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-delegatedNameConstraints           OBJECT IDENTIFIER ::= {id-ce 42}
/// ```
///
///
pub fn id_ce_delegatedNameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([42])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-timeSpecification                  OBJECT IDENTIFIER ::= {id-ce 43}
/// ```
///
///
pub fn id_ce_timeSpecification() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([43])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-attributeDescriptor                OBJECT IDENTIFIER ::= {id-ce 48}
/// ```
///
///
pub fn id_ce_attributeDescriptor() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([48])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-userNotice                         OBJECT IDENTIFIER ::= {id-ce 49}
/// ```
///
///
pub fn id_ce_userNotice() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([49])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-sOAIdentifier                      OBJECT IDENTIFIER ::= {id-ce 50}
/// ```
///
///
pub fn id_ce_sOAIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([50])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-acceptableCertPolicies             OBJECT IDENTIFIER ::= {id-ce 52}
/// ```
///
///
pub fn id_ce_acceptableCertPolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([52])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-targetingInformation               OBJECT IDENTIFIER ::= {id-ce 55}
/// ```
///
///
pub fn id_ce_targetingInformation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([55])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-noRevAvail                         OBJECT IDENTIFIER ::= {id-ce 56}
/// ```
///
///
pub fn id_ce_noRevAvail() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([56])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-acceptablePrivilegePolicies        OBJECT IDENTIFIER ::= {id-ce 57}
/// ```
///
///
pub fn id_ce_acceptablePrivilegePolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([57])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-indirectIssuer                     OBJECT IDENTIFIER ::= {id-ce 61}
/// ```
///
///
pub fn id_ce_indirectIssuer() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([61])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-noAssertion                        OBJECT IDENTIFIER ::= {id-ce 62}
/// ```
///
///
pub fn id_ce_noAssertion() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([62])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-issuedOnBehalfOf                   OBJECT IDENTIFIER ::= {id-ce 64}
/// ```
///
///
pub fn id_ce_issuedOnBehalfOf() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([64])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-singleUse                          OBJECT IDENTIFIER ::= {id-ce 65}
/// ```
///
///
pub fn id_ce_singleUse() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([65])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-groupAC                            OBJECT IDENTIFIER ::= {id-ce 66}
/// ```
///
///
pub fn id_ce_groupAC() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([66])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-allowedAttributeAssignments        OBJECT IDENTIFIER ::= {id-ce 67}
/// ```
///
///
pub fn id_ce_allowedAttributeAssignments() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([67])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-attributeMappings                  OBJECT IDENTIFIER ::= {id-ce 68}
/// ```
///
///
pub fn id_ce_attributeMappings() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([68])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-holderNameConstraints              OBJECT IDENTIFIER ::= {id-ce 69}
/// ```
///
///
pub fn id_ce_holderNameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([69])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-attributeCertificateMatch          OBJECT IDENTIFIER ::= {id-mr 42}
/// ```
///
///
pub fn id_mr_attributeCertificateMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([42])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-attributeCertificateExactMatch     OBJECT IDENTIFIER ::= {id-mr 45}
/// ```
///
///
pub fn id_mr_attributeCertificateExactMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([45])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-holderIssuerMatch                  OBJECT IDENTIFIER ::= {id-mr 46}
/// ```
///
///
pub fn id_mr_holderIssuerMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([46])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-authAttIdMatch                     OBJECT IDENTIFIER ::= {id-mr 53}
/// ```
///
///
pub fn id_mr_authAttIdMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([53])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-roleSpecCertIdMatch                OBJECT IDENTIFIER ::= {id-mr 54}
/// ```
///
///
pub fn id_mr_roleSpecCertIdMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([54])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-basicAttConstraintsMatch           OBJECT IDENTIFIER ::= {id-mr 55}
/// ```
///
///
pub fn id_mr_basicAttConstraintsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([55])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-delegatedNameConstraintsMatch      OBJECT IDENTIFIER ::= {id-mr 56}
/// ```
///
///
pub fn id_mr_delegatedNameConstraintsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([56])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-timeSpecMatch                      OBJECT IDENTIFIER ::= {id-mr 57}
/// ```
///
///
pub fn id_mr_timeSpecMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([57])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-attDescriptorMatch                 OBJECT IDENTIFIER ::= {id-mr 58}
/// ```
///
///
pub fn id_mr_attDescriptorMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([58])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-acceptableCertPoliciesMatch        OBJECT IDENTIFIER ::= {id-mr 59}
/// ```
///
///
pub fn id_mr_acceptableCertPoliciesMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([59])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-delegationPathMatch                OBJECT IDENTIFIER ::= {id-mr 61}
/// ```
///
///
pub fn id_mr_delegationPathMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([61])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-sOAIdentifierMatch                 OBJECT IDENTIFIER ::= {id-mr 66}
/// ```
///
///
pub fn id_mr_sOAIdentifierMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([66])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-extensionPresenceMatch             OBJECT IDENTIFIER ::= {id-mr 67}
/// ```
///
///
pub fn id_mr_extensionPresenceMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([67])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-dualStringMatch                    OBJECT IDENTIFIER ::= {id-mr 69}
/// ```
///
///
pub fn id_mr_dualStringMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([69])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ObjectDigestInfo-digestedObjectType ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ObjectDigestInfo_digestedObjectType = ENUMERATED;

pub const ObjectDigestInfo_digestedObjectType_publicKey: ObjectDigestInfo_digestedObjectType = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ObjectDigestInfo_digestedObjectType_publicKeyCert: ObjectDigestInfo_digestedObjectType =
    1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ObjectDigestInfo_digestedObjectType_otherObjectTypes:
    ObjectDigestInfo_digestedObjectType = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ObjectDigestInfo_digestedObjectType(
    el: &X690Element,
) -> ASN1Result<ObjectDigestInfo_digestedObjectType> {
    ber_decode_enumerated(&el)
}

pub fn _encode_ObjectDigestInfo_digestedObjectType(
    value_: &ObjectDigestInfo_digestedObjectType,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AllowedAttributeAssignments-Item-attributes-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum AllowedAttributeAssignments_Item_attributes_Item {
    attributeType(AttributeType),
    attributeTypeandValues(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AllowedAttributeAssignments_Item_attributes_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AllowedAttributeAssignments_Item_attributes_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AllowedAttributeAssignments_Item_attributes_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AllowedAttributeAssignments_Item_attributes_Item(el)
    }
}

pub fn _decode_AllowedAttributeAssignments_Item_attributes_Item(
    el: &X690Element,
) -> ASN1Result<AllowedAttributeAssignments_Item_attributes_Item> {
    |el: &X690Element| -> ASN1Result<AllowedAttributeAssignments_Item_attributes_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(
                AllowedAttributeAssignments_Item_attributes_Item::attributeType(
                    _decode_AttributeType(&el)?,
                ),
            ),
            (TagClass::CONTEXT, 1) => Ok(
                AllowedAttributeAssignments_Item_attributes_Item::attributeTypeandValues(
                    _decode_Attribute(&el)?,
                ),
            ),
            _ => Ok(AllowedAttributeAssignments_Item_attributes_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AllowedAttributeAssignments_Item_attributes_Item(
    value_: &AllowedAttributeAssignments_Item_attributes_Item,
) -> ASN1Result<X690Element> {
    |value: &AllowedAttributeAssignments_Item_attributes_Item| -> ASN1Result<X690Element> {
        match value {
            AllowedAttributeAssignments_Item_attributes_Item::attributeType(v) => {
                |v_1: &AttributeType| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeType(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            AllowedAttributeAssignments_Item_attributes_Item::attributeTypeandValues(v) => {
                |v_1: &Attribute| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Attribute(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            AllowedAttributeAssignments_Item_attributes_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AllowedAttributeAssignments-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AllowedAttributeAssignments_Item {
    pub attributes: Vec<AllowedAttributeAssignments_Item_attributes_Item>,
    pub holderDomain: GeneralName,
    pub _unrecognized: Vec<X690Element>,
}
impl AllowedAttributeAssignments_Item {
    pub fn new(
        attributes: Vec<AllowedAttributeAssignments_Item_attributes_Item>,
        holderDomain: GeneralName,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AllowedAttributeAssignments_Item {
            attributes,
            holderDomain,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AllowedAttributeAssignments_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AllowedAttributeAssignments_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AllowedAttributeAssignments_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AllowedAttributeAssignments_Item(el)
    }
}

pub const _rctl1_components_for_AllowedAttributeAssignments_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "holderDomain",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AllowedAttributeAssignments_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AllowedAttributeAssignments_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_AllowedAttributeAssignments_Item(
    el: &X690Element,
) -> ASN1Result<AllowedAttributeAssignments_Item> {
    |el_: &X690Element| -> ASN1Result<AllowedAttributeAssignments_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AllowedAttributeAssignments_Item,
            _eal_components_for_AllowedAttributeAssignments_Item,
            _rctl2_components_for_AllowedAttributeAssignments_Item,
        )?;
        let attributes = |el: &X690Element| -> ASN1Result<
            SET_OF<AllowedAttributeAssignments_Item_attributes_Item>,
        > {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<AllowedAttributeAssignments_Item_attributes_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_AllowedAttributeAssignments_Item_attributes_Item(
                    el,
                )?);
            }
            Ok(items)
        }(_components.get("attributes").unwrap())?;
        let holderDomain = |el: &X690Element| -> ASN1Result<GeneralName> {
            Ok(_decode_GeneralName(&el.inner()?)?)
        }(_components.get("holderDomain").unwrap())?;
        Ok(AllowedAttributeAssignments_Item {
            attributes,
            holderDomain,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AllowedAttributeAssignments_Item(
    value_: &AllowedAttributeAssignments_Item,
) -> ASN1Result<X690Element> {
    |value_: &AllowedAttributeAssignments_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &Vec<AllowedAttributeAssignments_Item_attributes_Item>| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SET_OF<AllowedAttributeAssignments_Item_attributes_Item>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AllowedAttributeAssignments_Item_attributes_Item(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?; el_1.tag_class = TagClass::CONTEXT; el_1.tag_number = 0; Ok(el_1) }(&value_.attributes)?);
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
            ))
        }(&value_.holderDomain)?);
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
/// AttributeMappings-Item-typeMappings ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeMappings_Item_typeMappings {
    pub local: AttributeType,
    pub remote: AttributeType,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeMappings_Item_typeMappings {
    pub fn new(
        local: AttributeType,
        remote: AttributeType,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeMappings_Item_typeMappings {
            local,
            remote,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeMappings_Item_typeMappings {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item_typeMappings(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeMappings_Item_typeMappings {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item_typeMappings(el)
    }
}

pub const _rctl1_components_for_AttributeMappings_Item_typeMappings: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "local",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "remote",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeMappings_Item_typeMappings: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeMappings_Item_typeMappings: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeMappings_Item_typeMappings(
    el: &X690Element,
) -> ASN1Result<AttributeMappings_Item_typeMappings> {
    |el_: &X690Element| -> ASN1Result<AttributeMappings_Item_typeMappings> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeMappings_Item_typeMappings,
            _eal_components_for_AttributeMappings_Item_typeMappings,
            _rctl2_components_for_AttributeMappings_Item_typeMappings,
        )?;
        let local = _decode_AttributeType(_components.get("local").unwrap())?;
        let remote = _decode_AttributeType(_components.get("remote").unwrap())?;
        Ok(AttributeMappings_Item_typeMappings {
            local,
            remote,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeMappings_Item_typeMappings(
    value_: &AttributeMappings_Item_typeMappings,
) -> ASN1Result<X690Element> {
    |value_: &AttributeMappings_Item_typeMappings| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeType(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.local)?);
        components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeType(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 1;
            Ok(el_1)
        }(&value_.remote)?);
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
/// AttributeMappings-Item-typeValueMappings ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeMappings_Item_typeValueMappings {
    pub local: AttributeTypeAndValue,
    pub remote: AttributeTypeAndValue,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeMappings_Item_typeValueMappings {
    pub fn new(
        local: AttributeTypeAndValue,
        remote: AttributeTypeAndValue,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeMappings_Item_typeValueMappings {
            local,
            remote,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeMappings_Item_typeValueMappings {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item_typeValueMappings(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeMappings_Item_typeValueMappings {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item_typeValueMappings(el)
    }
}

pub const _rctl1_components_for_AttributeMappings_Item_typeValueMappings: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "local",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "remote",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeMappings_Item_typeValueMappings: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeMappings_Item_typeValueMappings: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeMappings_Item_typeValueMappings(
    el: &X690Element,
) -> ASN1Result<AttributeMappings_Item_typeValueMappings> {
    |el_: &X690Element| -> ASN1Result<AttributeMappings_Item_typeValueMappings> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeMappings_Item_typeValueMappings,
            _eal_components_for_AttributeMappings_Item_typeValueMappings,
            _rctl2_components_for_AttributeMappings_Item_typeValueMappings,
        )?;
        let local = _decode_AttributeTypeAndValue(_components.get("local").unwrap())?;
        let remote = _decode_AttributeTypeAndValue(_components.get("remote").unwrap())?;
        Ok(AttributeMappings_Item_typeValueMappings {
            local,
            remote,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeMappings_Item_typeValueMappings(
    value_: &AttributeMappings_Item_typeValueMappings,
) -> ASN1Result<X690Element> {
    |value_: &AttributeMappings_Item_typeValueMappings| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &AttributeTypeAndValue| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeTypeAndValue(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.local)?);
        components_.push(|v_1: &AttributeTypeAndValue| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeTypeAndValue(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 1;
            Ok(el_1)
        }(&value_.remote)?);
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
/// AttributeMappings-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum AttributeMappings_Item {
    typeMappings(AttributeMappings_Item_typeMappings),
    typeValueMappings(AttributeMappings_Item_typeValueMappings),
}

impl TryFrom<X690Element> for AttributeMappings_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeMappings_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item(el)
    }
}

pub fn _decode_AttributeMappings_Item(el: &X690Element) -> ASN1Result<AttributeMappings_Item> {
    |el: &X690Element| -> ASN1Result<AttributeMappings_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(AttributeMappings_Item::typeMappings(
                _decode_AttributeMappings_Item_typeMappings(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(AttributeMappings_Item::typeValueMappings(
                _decode_AttributeMappings_Item_typeValueMappings(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_AttributeMappings_Item(value_: &AttributeMappings_Item) -> ASN1Result<X690Element> {
    |value: &AttributeMappings_Item| -> ASN1Result<X690Element> {
        match value {
            AttributeMappings_Item::typeMappings(v) => {
                |v_1: &AttributeMappings_Item_typeMappings| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeMappings_Item_typeMappings(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            AttributeMappings_Item::typeValueMappings(v) => {
                |v_1: &AttributeMappings_Item_typeValueMappings| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeMappings_Item_typeValueMappings(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificateAssertion-holder ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum AttributeCertificateAssertion_holder {
    baseCertificateID(IssuerSerial),
    holderName(GeneralNames),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AttributeCertificateAssertion_holder {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateAssertion_holder(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeCertificateAssertion_holder {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateAssertion_holder(el)
    }
}

pub fn _decode_AttributeCertificateAssertion_holder(
    el: &X690Element,
) -> ASN1Result<AttributeCertificateAssertion_holder> {
    |el: &X690Element| -> ASN1Result<AttributeCertificateAssertion_holder> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(AttributeCertificateAssertion_holder::baseCertificateID(
                _decode_IssuerSerial(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(AttributeCertificateAssertion_holder::holderName(
                _decode_GeneralNames(&el)?,
            )),
            _ => Ok(AttributeCertificateAssertion_holder::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_AttributeCertificateAssertion_holder(
    value_: &AttributeCertificateAssertion_holder,
) -> ASN1Result<X690Element> {
    |value: &AttributeCertificateAssertion_holder| -> ASN1Result<X690Element> {
        match value {
            AttributeCertificateAssertion_holder::baseCertificateID(v) => {
                |v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_IssuerSerial(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            AttributeCertificateAssertion_holder::holderName(v) => {
                |v_1: &GeneralNames| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_GeneralNames(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            AttributeCertificateAssertion_holder::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}
