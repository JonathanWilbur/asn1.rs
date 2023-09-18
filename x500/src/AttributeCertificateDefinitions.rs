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
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificate  ::=  SIGNED{TBSAttributeCertificate}
/// ```
pub type AttributeCertificate = SIGNED<TBSAttributeCertificate>; // DefinedType

pub fn _decode_AttributeCertificate(el: &X690Element) -> ASN1Result<AttributeCertificate> {
    _decode_SIGNED::<TBSAttributeCertificate>(_decode_TBSAttributeCertificate, el)
}

pub fn _encode_AttributeCertificate(value_: &AttributeCertificate) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSAttributeCertificate>(_encode_TBSAttributeCertificate, value_)
}

pub fn _validate_AttributeCertificate(el: &X690Element) -> ASN1Result<()> {
    _validate_SIGNED::<TBSAttributeCertificate>(_validate_TBSAttributeCertificate, el)
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
impl TryFrom<&X690Element> for TBSAttributeCertificate {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TBSAttributeCertificate",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSAttributeCertificate,
        _eal_components_for_TBSAttributeCertificate,
        _rctl2_components_for_TBSAttributeCertificate,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AttCertVersion> = None;
    let mut holder_: OPTIONAL<Holder> = None;
    let mut issuer_: OPTIONAL<AttCertIssuer> = None;
    let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut attrCertValidityPeriod_: OPTIONAL<AttCertValidityPeriod> = None;
    let mut attributes_: OPTIONAL<Vec<Attribute>> = None;
    let mut issuerUniqueID_: OPTIONAL<UniqueIdentifier> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AttCertVersion(_el)?),
            "holder" => holder_ = Some(_decode_Holder(_el)?),
            "issuer" => issuer_ = Some(_decode_AttCertIssuer(_el)?),
            "signature" => signature_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "attrCertValidityPeriod" => {
                attrCertValidityPeriod_ = Some(_decode_AttCertValidityPeriod(_el)?)
            }
            "attributes" => {
                attributes_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "attributes",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "issuerUniqueID" => issuerUniqueID_ = Some(_decode_UniqueIdentifier(_el)?),
            "extensions" => extensions_ = Some(_decode_Extensions(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBSAttributeCertificate {
        version: version_.unwrap(),
        holder: holder_.unwrap(),
        issuer: issuer_.unwrap(),
        signature: signature_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        attrCertValidityPeriod: attrCertValidityPeriod_.unwrap(),
        attributes: attributes_.unwrap(),
        issuerUniqueID: issuerUniqueID_,
        _unrecognized,
        extensions: extensions_,
    })
}

pub fn _encode_TBSAttributeCertificate(
    value_: &TBSAttributeCertificate,
) -> ASN1Result<X690Element> {
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
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TBSAttributeCertificate(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TBSAttributeCertificate",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSAttributeCertificate,
        _eal_components_for_TBSAttributeCertificate,
        _rctl2_components_for_TBSAttributeCertificate,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AttCertVersion(_el)?,
            "holder" => _validate_Holder(_el)?,
            "issuer" => _validate_AttCertIssuer(_el)?,
            "signature" => _validate_AlgorithmIdentifier(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "attrCertValidityPeriod" => _validate_AttCertValidityPeriod(_el)?,
            "attributes" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_Attribute(&sub)?;
                        }
                        Ok(())
                    }
                    _ => {
                        Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributes"))
                    }
                }
            }(_el)?,
            "issuerUniqueID" => _validate_UniqueIdentifier(_el)?,
            "extensions" => _validate_Extensions(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertVersion  ::=  INTEGER {v2(1)}
/// ```
pub type AttCertVersion = i8;

pub const AttCertVersion_v2: AttCertVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AttCertVersion(el: &X690Element) -> ASN1Result<AttCertVersion> {
    BER.decode_i8(el)
}

pub fn _encode_AttCertVersion(value_: &AttCertVersion) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_AttCertVersion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
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
impl TryFrom<&X690Element> for Holder {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Holder,
        _eal_components_for_Holder,
        _rctl2_components_for_Holder,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut baseCertificateID_: OPTIONAL<IssuerSerial> = None;
    let mut entityName_: OPTIONAL<GeneralNames> = None;
    let mut objectDigestInfo_: OPTIONAL<ObjectDigestInfo> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "baseCertificateID" => baseCertificateID_ = Some(_decode_IssuerSerial(_el)?),
            "entityName" => entityName_ = Some(_decode_GeneralNames(_el)?),
            "objectDigestInfo" => objectDigestInfo_ = Some(_decode_ObjectDigestInfo(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
        }
    }
    Ok(Holder {
        baseCertificateID: baseCertificateID_,
        entityName: entityName_,
        objectDigestInfo: objectDigestInfo_,
    })
}

pub fn _encode_Holder(value_: &Holder) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.baseCertificateID {
        components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_IssuerSerial(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.entityName {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.objectDigestInfo {
        components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ObjectDigestInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Holder(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Holder,
        _eal_components_for_Holder,
        _rctl2_components_for_Holder,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "baseCertificateID" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "baseCertificateID",
                    ));
                }
                Ok(_validate_IssuerSerial(&el)?)
            }(_el)?,
            "entityName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "entityName")
                    );
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            "objectDigestInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "objectDigestInfo",
                    ));
                }
                Ok(_validate_ObjectDigestInfo(&el)?)
            }(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for IssuerSerial {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerSerial")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerSerial,
        _eal_components_for_IssuerSerial,
        _rctl2_components_for_IssuerSerial,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<GeneralNames> = None;
    let mut serial_: OPTIONAL<CertificateSerialNumber> = None;
    let mut issuerUID_: OPTIONAL<UniqueIdentifier> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_GeneralNames(_el)?),
            "serial" => serial_ = Some(_decode_CertificateSerialNumber(_el)?),
            "issuerUID" => issuerUID_ = Some(_decode_UniqueIdentifier(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IssuerSerial {
        issuer: issuer_.unwrap(),
        serial: serial_.unwrap(),
        issuerUID: issuerUID_,
        _unrecognized,
    })
}

pub fn _encode_IssuerSerial(value_: &IssuerSerial) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_GeneralNames(&value_.issuer)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serial)?);
    if let Some(v_) = &value_.issuerUID {
        components_.push(_encode_UniqueIdentifier(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IssuerSerial(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerSerial")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerSerial,
        _eal_components_for_IssuerSerial,
        _rctl2_components_for_IssuerSerial,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_GeneralNames(_el)?,
            "serial" => _validate_CertificateSerialNumber(_el)?,
            "issuerUID" => _validate_UniqueIdentifier(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for ObjectDigestInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ObjectDigestInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ObjectDigestInfo,
        _eal_components_for_ObjectDigestInfo,
        _rctl2_components_for_ObjectDigestInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut digestedObjectType_: OPTIONAL<ObjectDigestInfo_digestedObjectType> = None;
    let mut otherObjectTypeID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut digestAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut objectDigest_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "digestedObjectType" => {
                digestedObjectType_ = Some(_decode_ObjectDigestInfo_digestedObjectType(_el)?)
            }
            "otherObjectTypeID" => otherObjectTypeID_ = Some(BER.decode_object_identifier(_el)?),
            "digestAlgorithm" => digestAlgorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "objectDigest" => objectDigest_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ObjectDigestInfo {
        digestedObjectType: digestedObjectType_.unwrap(),
        otherObjectTypeID: otherObjectTypeID_,
        digestAlgorithm: digestAlgorithm_.unwrap(),
        objectDigest: objectDigest_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ObjectDigestInfo(value_: &ObjectDigestInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(_encode_ObjectDigestInfo_digestedObjectType(
        &value_.digestedObjectType,
    )?);
    if let Some(v_) = &value_.otherObjectTypeID {
        components_.push(BER.encode_object_identifier(&v_)?);
    }
    components_.push(_encode_AlgorithmIdentifier(&value_.digestAlgorithm)?);
    components_.push(BER.encode_bit_string(&value_.objectDigest)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ObjectDigestInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ObjectDigestInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ObjectDigestInfo,
        _eal_components_for_ObjectDigestInfo,
        _rctl2_components_for_ObjectDigestInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "digestedObjectType" => _validate_ObjectDigestInfo_digestedObjectType(_el)?,
            "otherObjectTypeID" => BER.validate_object_identifier(_el)?,
            "digestAlgorithm" => _validate_AlgorithmIdentifier(_el)?,
            "objectDigest" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for AttCertIssuer {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    |el: &X690Element| -> ASN1Result<AttCertIssuer> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertIssuer")
                )
            }
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_AttCertIssuer,
            _eal_components_for_AttCertIssuer,
            _rctl2_components_for_AttCertIssuer,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut issuerName_: OPTIONAL<GeneralNames> = None;
        let mut baseCertificateID_: OPTIONAL<IssuerSerial> = None;
        let mut objectDigestInfo_: OPTIONAL<ObjectDigestInfo> = None;
        let mut _unrecognized: Vec<X690Element> = vec![];
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "issuerName" => issuerName_ = Some(_decode_GeneralNames(_el)?),
                "baseCertificateID" => baseCertificateID_ = Some(_decode_IssuerSerial(_el)?),
                "objectDigestInfo" => objectDigestInfo_ = Some(_decode_ObjectDigestInfo(_el)?),
                _ => _unrecognized.push(_el.clone()),
            }
        }
        Ok(AttCertIssuer {
            issuerName: issuerName_,
            baseCertificateID: baseCertificateID_,
            objectDigestInfo: objectDigestInfo_,
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
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.objectDigestInfo {
                components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ObjectDigestInfo(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
                X690Value::Constructed(Arc::new(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_AttCertIssuer(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertIssuer"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertIssuer")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_AttCertIssuer,
                _eal_components_for_AttCertIssuer,
                _rctl2_components_for_AttCertIssuer,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "issuerName" => _validate_GeneralNames(_el)?,
                    "baseCertificateID" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "baseCertificateID",
                            ));
                        }
                        Ok(_validate_IssuerSerial(&el)?)
                    }(_el)?,
                    "objectDigestInfo" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "objectDigestInfo",
                            ));
                        }
                        Ok(_validate_ObjectDigestInfo(&el)?)
                    }(_el)?,
                    _ => (),
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
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
impl TryFrom<&X690Element> for AttCertValidityPeriod {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertValidityPeriod")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttCertValidityPeriod,
        _eal_components_for_AttCertValidityPeriod,
        _rctl2_components_for_AttCertValidityPeriod,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notBeforeTime_: OPTIONAL<GeneralizedTime> = None;
    let mut notAfterTime_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBeforeTime" => notBeforeTime_ = Some(BER.decode_generalized_time(_el)?),
            "notAfterTime" => notAfterTime_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttCertValidityPeriod {
        notBeforeTime: notBeforeTime_.unwrap(),
        notAfterTime: notAfterTime_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttCertValidityPeriod(value_: &AttCertValidityPeriod) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_generalized_time(&value_.notBeforeTime)?);
    components_.push(BER.encode_generalized_time(&value_.notAfterTime)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttCertValidityPeriod(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertValidityPeriod")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttCertValidityPeriod,
        _eal_components_for_AttCertValidityPeriod,
        _rctl2_components_for_AttCertValidityPeriod,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBeforeTime" => BER.validate_generalized_time(_el)?,
            "notAfterTime" => BER.validate_generalized_time(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for AttributeCertificationPath {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeCertificationPath",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeCertificationPath,
        _eal_components_for_AttributeCertificationPath,
        _rctl2_components_for_AttributeCertificationPath,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attributeCertificate_: OPTIONAL<AttributeCertificate> = None;
    let mut acPath_: OPTIONAL<Vec<ACPathData>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeCertificate" => {
                attributeCertificate_ = Some(_decode_AttributeCertificate(_el)?)
            }
            "acPath" => {
                acPath_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<ACPathData>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "acPath")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<ACPathData> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ACPathData(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeCertificationPath {
        attributeCertificate: attributeCertificate_.unwrap(),
        acPath: acPath_,
        _unrecognized,
    })
}

pub fn _encode_AttributeCertificationPath(
    value_: &AttributeCertificationPath,
) -> ASN1Result<X690Element> {
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
                    Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_)?,
        );
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeCertificationPath(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeCertificationPath",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeCertificationPath,
        _eal_components_for_AttributeCertificationPath,
        _rctl2_components_for_AttributeCertificationPath,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeCertificate" => _validate_AttributeCertificate(_el)?,
            "acPath" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ACPathData(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "acPath")),
                }
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for ACPathData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ACPathData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACPathData,
        _eal_components_for_ACPathData,
        _rctl2_components_for_ACPathData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut certificate_: OPTIONAL<Certificate> = None;
    let mut attributeCertificate_: OPTIONAL<AttributeCertificate> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificate" => certificate_ = Some(_decode_Certificate(_el)?),
            "attributeCertificate" => {
                attributeCertificate_ = Some(_decode_AttributeCertificate(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ACPathData {
        certificate: certificate_,
        attributeCertificate: attributeCertificate_,
        _unrecognized,
    })
}

pub fn _encode_ACPathData(value_: &ACPathData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.certificate {
        components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Certificate(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.attributeCertificate {
        components_.push(|v_1: &AttributeCertificate| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeCertificate(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ACPathData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ACPathData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACPathData,
        _eal_components_for_ACPathData,
        _rctl2_components_for_ACPathData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificate" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certificate")
                    );
                }
                Ok(_validate_Certificate(&el)?)
            }(_el)?,
            "attributeCertificate" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "attributeCertificate",
                    ));
                }
                Ok(_validate_AttributeCertificate(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PrivilegePolicy  ::=  OBJECT IDENTIFIER
/// ```
pub type PrivilegePolicy = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_PrivilegePolicy(el: &X690Element) -> ASN1Result<PrivilegePolicy> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_PrivilegePolicy(value_: &PrivilegePolicy) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_PrivilegePolicy(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
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

pub mod role {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = RoleSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_RoleSyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_RoleSyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_RoleSyntax(el)
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
impl TryFrom<&X690Element> for RoleSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RoleSyntax")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RoleSyntax,
        _eal_components_for_RoleSyntax,
        _rctl2_components_for_RoleSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut roleAuthority_: OPTIONAL<GeneralNames> = None;
    let mut roleName_: OPTIONAL<GeneralName> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "roleAuthority" => roleAuthority_ = Some(_decode_GeneralNames(_el)?),
            "roleName" => {
                roleName_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RoleSyntax {
        roleAuthority: roleAuthority_,
        roleName: roleName_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RoleSyntax(value_: &RoleSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.roleAuthority {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
        ))
    }(&value_.roleName)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RoleSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RoleSyntax")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RoleSyntax,
        _eal_components_for_RoleSyntax,
        _rctl2_components_for_RoleSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "roleAuthority" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "roleAuthority")
                    );
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            "roleName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "roleName")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod xmlPrivilegeInfo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UTF8String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_utf8_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_utf8_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utf8_string(el)
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
        id: id_at_permission(),                  /* OBJECT_FIELD_SETTING */
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

pub mod permission {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DualStringSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DualStringSyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DualStringSyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DualStringSyntax(el)
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
impl TryFrom<&X690Element> for DualStringSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DualStringSyntax")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DualStringSyntax,
        _eal_components_for_DualStringSyntax,
        _rctl2_components_for_DualStringSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut operation_: OPTIONAL<UnboundedDirectoryString> = None;
    let mut object_: OPTIONAL<UnboundedDirectoryString> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "operation" => {
                operation_ = Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                    Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
                }(_el)?)
            }
            "object" => {
                object_ = Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                    Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DualStringSyntax {
        operation: operation_.unwrap(),
        object: object_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DualStringSyntax(value_: &DualStringSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(
        |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
            ))
        }(&value_.operation)?,
    );
    components_.push(
        |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
            ))
        }(&value_.object)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DualStringSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DualStringSyntax")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DualStringSyntax,
        _eal_components_for_DualStringSyntax,
        _rctl2_components_for_DualStringSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "operation" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "operation")
                    );
                }
                Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
            }(_el)?,
            "object" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "object"));
                }
                Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod dualStringMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = DualStringSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_DualStringSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_DualStringSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_DualStringSyntax(el)
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

pub mod timeSpecification {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = TimeSpecification; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_TimeSpecification(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_TimeSpecification(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_TimeSpecification(el)
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

pub mod timeSpecificationMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = TimeSpecification; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_TimeSpecification(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_TimeSpecification(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_TimeSpecification(el)
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

pub mod targetingInformation {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = Vec<Targets>; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&ExtnType")),
        };
        let mut items: SEQUENCE_OF<Targets> = Vec::with_capacity(elements.len());
        for el in elements.iter() {
            items.push(_decode_Targets(el)?);
        }
        Ok(items)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Targets(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    _validate_Targets(&sub)?;
                }
                Ok(())
            }
            _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&ExtnType")),
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Targets  ::=  SEQUENCE SIZE (1..MAX) OF Target
/// ```
pub type Targets = Vec<Target>; // SequenceOfType

pub fn _decode_Targets(el: &X690Element) -> ASN1Result<Targets> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Targets")),
    };
    let mut items: SEQUENCE_OF<Target> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Target(el)?);
    }
    Ok(items)
}

pub fn _encode_Targets(value_: &Targets) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Target(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Targets(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Target(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Targets")),
    }
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

impl TryFrom<&X690Element> for Target {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Target(el)
    }
}

pub fn _decode_Target(el: &X690Element) -> ASN1Result<Target> {
    match (el.tag.tag_class, el.tag.tag_number) {
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
}

pub fn _encode_Target(value_: &Target) -> ASN1Result<X690Element> {
    match value_ {
        Target::targetName(v) => |v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v),
        Target::targetGroup(v) => |v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v),
        Target::targetCert(v) => |v_1: &TargetCert| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_TargetCert(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        Target::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Target(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "targetName"));
            }
            Ok(_validate_GeneralName(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "targetGroup")
                );
            }
            Ok(_validate_GeneralName(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "targetCert"));
            }
            Ok(_validate_TargetCert(&el)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for TargetCert {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TargetCert")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TargetCert,
        _eal_components_for_TargetCert,
        _rctl2_components_for_TargetCert,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut targetCertificate_: OPTIONAL<IssuerSerial> = None;
    let mut targetName_: OPTIONAL<GeneralName> = None;
    let mut certDigestInfo_: OPTIONAL<ObjectDigestInfo> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "targetCertificate" => targetCertificate_ = Some(_decode_IssuerSerial(_el)?),
            "targetName" => targetName_ = Some(_decode_GeneralName(_el)?),
            "certDigestInfo" => certDigestInfo_ = Some(_decode_ObjectDigestInfo(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TargetCert"))
            }
        }
    }
    Ok(TargetCert {
        targetCertificate: targetCertificate_.unwrap(),
        targetName: targetName_,
        certDigestInfo: certDigestInfo_,
    })
}

pub fn _encode_TargetCert(value_: &TargetCert) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_IssuerSerial(&value_.targetCertificate)?);
    if let Some(v_) = &value_.targetName {
        components_.push(_encode_GeneralName(&v_)?);
    }
    if let Some(v_) = &value_.certDigestInfo {
        components_.push(_encode_ObjectDigestInfo(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TargetCert(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TargetCert")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TargetCert,
        _eal_components_for_TargetCert,
        _rctl2_components_for_TargetCert,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "targetCertificate" => _validate_IssuerSerial(_el)?,
            "targetName" => _validate_GeneralName(_el)?,
            "certDigestInfo" => _validate_ObjectDigestInfo(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TargetCert"))
            }
        }
    }
    Ok(())
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

pub mod userNotice {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = Vec<UserNotice>; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&ExtnType")),
        };
        let mut items: SEQUENCE_OF<UserNotice> = Vec::with_capacity(elements.len());
        for el in elements.iter() {
            items.push(_decode_UserNotice(el)?);
        }
        Ok(items)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_UserNotice(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    _validate_UserNotice(&sub)?;
                }
                Ok(())
            }
            _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&ExtnType")),
        }
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
impl TryFrom<&X690Element> for UserNotice {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserNotice,
        _eal_components_for_UserNotice,
        _rctl2_components_for_UserNotice,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut noticeRef_: OPTIONAL<NoticeReference> = None;
    let mut explicitText_: OPTIONAL<DisplayText> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "noticeRef" => noticeRef_ = Some(_decode_NoticeReference(_el)?),
            "explicitText" => explicitText_ = Some(_decode_DisplayText(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice"))
            }
        }
    }
    Ok(UserNotice {
        noticeRef: noticeRef_,
        explicitText: explicitText_,
    })
}

pub fn _encode_UserNotice(value_: &UserNotice) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.noticeRef {
        components_.push(_encode_NoticeReference(&v_)?);
    }
    if let Some(v_) = &value_.explicitText {
        components_.push(_encode_DisplayText(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_UserNotice(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserNotice,
        _eal_components_for_UserNotice,
        _rctl2_components_for_UserNotice,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "noticeRef" => _validate_NoticeReference(_el)?,
            "explicitText" => _validate_DisplayText(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NoticeReference ::= SEQUENCE {
///   organization   DisplayText,
///   noticeNumbers  SEQUENCE OF INTEGER }
/// ```
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
impl TryFrom<&X690Element> for NoticeReference {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NoticeReference,
        _eal_components_for_NoticeReference,
        _rctl2_components_for_NoticeReference,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut organization_: OPTIONAL<DisplayText> = None;
    let mut noticeNumbers_: OPTIONAL<Vec<INTEGER>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "organization" => organization_ = Some(_decode_DisplayText(_el)?),
            "noticeNumbers" => {
                noticeNumbers_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<INTEGER>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "noticeNumbers",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<INTEGER> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(BER.decode_integer(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference")
                )
            }
        }
    }
    Ok(NoticeReference {
        organization: organization_.unwrap(),
        noticeNumbers: noticeNumbers_.unwrap(),
    })
}

pub fn _encode_NoticeReference(value_: &NoticeReference) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_DisplayText(&value_.organization)?);
    components_.push(|value_: &SEQUENCE_OF<INTEGER>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(BER.encode_integer(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.noticeNumbers)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_NoticeReference(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NoticeReference,
        _eal_components_for_NoticeReference,
        _rctl2_components_for_NoticeReference,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "organization" => _validate_DisplayText(_el)?,
            "noticeNumbers" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_integer(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "noticeNumbers")
                    ),
                }
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference")
                )
            }
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for DisplayText {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DisplayText(el)
    }
}

pub fn _decode_DisplayText(el: &X690Element) -> ASN1Result<DisplayText> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 26) => {
            Ok(DisplayText::visibleString(BER.decode_visible_string(&el)?))
        }
        (TagClass::UNIVERSAL, 30) => Ok(DisplayText::bmpString(BER.decode_bmp_string(&el)?)),
        (TagClass::UNIVERSAL, 12) => Ok(DisplayText::utf8String(BER.decode_utf8_string(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "DisplayText",
            ))
        }
    }
}

pub fn _encode_DisplayText(value_: &DisplayText) -> ASN1Result<X690Element> {
    match value_ {
        DisplayText::visibleString(v) => BER.encode_visible_string(&v),
        DisplayText::bmpString(v) => BER.encode_bmp_string(&v),
        DisplayText::utf8String(v) => BER.encode_utf8_string(&v),
    }
}

pub fn _validate_DisplayText(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 26) => BER.validate_visible_string(&el),
        (TagClass::UNIVERSAL, 30) => BER.validate_bmp_string(&el),
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "DisplayText",
            ))
        }
    }
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

pub mod acceptablePrivilegePolicies {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AcceptablePrivilegePoliciesSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AcceptablePrivilegePoliciesSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AcceptablePrivilegePoliciesSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AcceptablePrivilegePoliciesSyntax(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AcceptablePrivilegePoliciesSyntax",
            ))
        }
    };
    let mut items: SEQUENCE_OF<PrivilegePolicy> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_PrivilegePolicy(el)?);
    }
    Ok(items)
}

pub fn _encode_AcceptablePrivilegePoliciesSyntax(
    value_: &AcceptablePrivilegePoliciesSyntax,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_PrivilegePolicy(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AcceptablePrivilegePoliciesSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_PrivilegePolicy(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "AcceptablePrivilegePoliciesSyntax",
        )),
    }
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

pub mod singleUse {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_null(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod groupAC {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_null(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod noRevAvail {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_null(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod sOAIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_null(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod sOAIdentifierMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_null(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod attributeDescriptor {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AttributeDescriptorSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AttributeDescriptorSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AttributeDescriptorSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeDescriptorSyntax(el)
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
impl TryFrom<&X690Element> for AttributeDescriptorSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeDescriptorSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeDescriptorSyntax,
        _eal_components_for_AttributeDescriptorSyntax,
        _rctl2_components_for_AttributeDescriptorSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut identifier_: OPTIONAL<AttributeIdentifier> = None;
    let mut attributeSyntax_: OPTIONAL<OCTET_STRING> = None;
    let mut name_: OPTIONAL<AttributeName> = None;
    let mut description_: OPTIONAL<AttributeDescription> = None;
    let mut dominationRule_: OPTIONAL<PrivilegePolicyIdentifier> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "identifier" => identifier_ = Some(_decode_AttributeIdentifier(_el)?),
            "attributeSyntax" => attributeSyntax_ = Some(BER.decode_octet_string(_el)?),
            "name" => name_ = Some(_decode_AttributeName(_el)?),
            "description" => description_ = Some(_decode_AttributeDescription(_el)?),
            "dominationRule" => dominationRule_ = Some(_decode_PrivilegePolicyIdentifier(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeDescriptorSyntax {
        identifier: identifier_.unwrap(),
        attributeSyntax: attributeSyntax_.unwrap(),
        name: name_,
        description: description_,
        dominationRule: dominationRule_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttributeDescriptorSyntax(
    value_: &AttributeDescriptorSyntax,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(_encode_AttributeIdentifier(&value_.identifier)?);
    components_.push(BER.encode_octet_string(&value_.attributeSyntax)?);
    if let Some(v_) = &value_.name {
        components_.push(|v_1: &AttributeName| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeName(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.description {
        components_.push(|v_1: &AttributeDescription| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttributeDescription(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_PrivilegePolicyIdentifier(&value_.dominationRule)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeDescriptorSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeDescriptorSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeDescriptorSyntax,
        _eal_components_for_AttributeDescriptorSyntax,
        _rctl2_components_for_AttributeDescriptorSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "identifier" => _validate_AttributeIdentifier(_el)?,
            "attributeSyntax" => BER.validate_octet_string(_el)?,
            "name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "name"));
                }
                Ok(_validate_AttributeName(&el)?)
            }(_el)?,
            "description" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "description")
                    );
                }
                Ok(_validate_AttributeDescription(&el)?)
            }(_el)?,
            "dominationRule" => _validate_PrivilegePolicyIdentifier(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeIdentifier  ::=  ATTRIBUTE.&id({AttributeIDs})
/// ```
pub type AttributeIdentifier = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_AttributeIdentifier(el: &X690Element) -> ASN1Result<AttributeIdentifier> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_AttributeIdentifier(value_: &AttributeIdentifier) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_AttributeIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeIDs ATTRIBUTE ::= {...}
/// ```
///
///
pub fn AttributeIDs() -> Vec<ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeName  ::=  UTF8String(SIZE (1..MAX))
/// ```
pub type AttributeName = UTF8String; // UTF8String

pub fn _decode_AttributeName(el: &X690Element) -> ASN1Result<AttributeName> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_AttributeName(value_: &AttributeName) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_AttributeName(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeDescription  ::=  UTF8String(SIZE (1..MAX))
/// ```
pub type AttributeDescription = UTF8String; // UTF8String

pub fn _decode_AttributeDescription(el: &X690Element) -> ASN1Result<AttributeDescription> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_AttributeDescription(value_: &AttributeDescription) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_AttributeDescription(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
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
impl TryFrom<&X690Element> for PrivilegePolicyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PrivilegePolicyIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PrivilegePolicyIdentifier,
        _eal_components_for_PrivilegePolicyIdentifier,
        _rctl2_components_for_PrivilegePolicyIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut privilegePolicy_: OPTIONAL<PrivilegePolicy> = None;
    let mut privPolSyntax_: OPTIONAL<InfoSyntax> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "privilegePolicy" => privilegePolicy_ = Some(_decode_PrivilegePolicy(_el)?),
            "privPolSyntax" => privPolSyntax_ = Some(_decode_InfoSyntax(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PrivilegePolicyIdentifier {
        privilegePolicy: privilegePolicy_.unwrap(),
        privPolSyntax: privPolSyntax_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_PrivilegePolicyIdentifier(
    value_: &PrivilegePolicyIdentifier,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_PrivilegePolicy(&value_.privilegePolicy)?);
    components_.push(_encode_InfoSyntax(&value_.privPolSyntax)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PrivilegePolicyIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PrivilegePolicyIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PrivilegePolicyIdentifier,
        _eal_components_for_PrivilegePolicyIdentifier,
        _rctl2_components_for_PrivilegePolicyIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "privilegePolicy" => _validate_PrivilegePolicy(_el)?,
            "privPolSyntax" => _validate_InfoSyntax(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod attDescriptor {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = AttributeDescriptorSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_AttributeDescriptorSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_AttributeDescriptorSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeDescriptorSyntax(el)
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

pub mod roleSpecCertIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = RoleSpecCertIdentifierSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_RoleSpecCertIdentifierSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_RoleSpecCertIdentifierSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_RoleSpecCertIdentifierSyntax(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RoleSpecCertIdentifierSyntax",
            ))
        }
    };
    let mut items: SEQUENCE_OF<RoleSpecCertIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RoleSpecCertIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_RoleSpecCertIdentifierSyntax(
    value_: &RoleSpecCertIdentifierSyntax,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RoleSpecCertIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RoleSpecCertIdentifierSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RoleSpecCertIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "RoleSpecCertIdentifierSyntax",
        )),
    }
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
impl TryFrom<&X690Element> for RoleSpecCertIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RoleSpecCertIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RoleSpecCertIdentifier,
        _eal_components_for_RoleSpecCertIdentifier,
        _rctl2_components_for_RoleSpecCertIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut roleName_: OPTIONAL<GeneralName> = None;
    let mut roleCertIssuer_: OPTIONAL<GeneralName> = None;
    let mut roleCertSerialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut roleCertLocator_: OPTIONAL<GeneralNames> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "roleName" => {
                roleName_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "roleCertIssuer" => {
                roleCertIssuer_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "roleCertSerialNumber" => {
                roleCertSerialNumber_ = Some(_decode_CertificateSerialNumber(_el)?)
            }
            "roleCertLocator" => roleCertLocator_ = Some(_decode_GeneralNames(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RoleSpecCertIdentifier {
        roleName: roleName_.unwrap(),
        roleCertIssuer: roleCertIssuer_.unwrap(),
        roleCertSerialNumber: roleCertSerialNumber_,
        roleCertLocator: roleCertLocator_,
        _unrecognized,
    })
}

pub fn _encode_RoleSpecCertIdentifier(value_: &RoleSpecCertIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
        ))
    }(&value_.roleName)?);
    components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
        ))
    }(&value_.roleCertIssuer)?);
    if let Some(v_) = &value_.roleCertSerialNumber {
        components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.roleCertLocator {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RoleSpecCertIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RoleSpecCertIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RoleSpecCertIdentifier,
        _eal_components_for_RoleSpecCertIdentifier,
        _rctl2_components_for_RoleSpecCertIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "roleName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "roleName")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "roleCertIssuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "roleCertIssuer")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "roleCertSerialNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "roleCertSerialNumber",
                    ));
                }
                Ok(_validate_CertificateSerialNumber(&el)?)
            }(_el)?,
            "roleCertLocator" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "roleCertLocator",
                    ));
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod roleSpecCertIdMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = RoleSpecCertIdentifierSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_RoleSpecCertIdentifierSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_RoleSpecCertIdentifierSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_RoleSpecCertIdentifierSyntax(el)
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

pub mod basicAttConstraints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = BasicAttConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_BasicAttConstraintsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_BasicAttConstraintsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_BasicAttConstraintsSyntax(el)
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
impl TryFrom<&X690Element> for BasicAttConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "BasicAttConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BasicAttConstraintsSyntax,
        _eal_components_for_BasicAttConstraintsSyntax,
        _rctl2_components_for_BasicAttConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut authority_: OPTIONAL<BOOLEAN> = None;
    let mut pathLenConstraint_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "authority" => authority_ = Some(BER.decode_boolean(_el)?),
            "pathLenConstraint" => pathLenConstraint_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(BasicAttConstraintsSyntax {
        authority: authority_,
        pathLenConstraint: pathLenConstraint_,
        _unrecognized,
    })
}

pub fn _encode_BasicAttConstraintsSyntax(
    value_: &BasicAttConstraintsSyntax,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.authority {
        if *v_ != BasicAttConstraintsSyntax::_default_value_for_authority() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    if let Some(v_) = &value_.pathLenConstraint {
        components_.push(BER.encode_integer(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_BasicAttConstraintsSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "BasicAttConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BasicAttConstraintsSyntax,
        _eal_components_for_BasicAttConstraintsSyntax,
        _rctl2_components_for_BasicAttConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "authority" => BER.validate_boolean(_el)?,
            "pathLenConstraint" => BER.validate_integer(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod basicAttConstraintsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = BasicAttConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_BasicAttConstraintsSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_BasicAttConstraintsSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_BasicAttConstraintsSyntax(el)
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

pub mod delegatedNameConstraints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NameConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_NameConstraintsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_NameConstraintsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_NameConstraintsSyntax(el)
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

pub mod delegatedNameConstraintsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = NameConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_NameConstraintsSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_NameConstraintsSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_NameConstraintsSyntax(el)
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

pub mod acceptableCertPolicies {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AcceptableCertPoliciesSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AcceptableCertPoliciesSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AcceptableCertPoliciesSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AcceptableCertPoliciesSyntax(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AcceptableCertPoliciesSyntax",
            ))
        }
    };
    let mut items: SEQUENCE_OF<CertPolicyId> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertPolicyId(el)?);
    }
    Ok(items)
}

pub fn _encode_AcceptableCertPoliciesSyntax(
    value_: &AcceptableCertPoliciesSyntax,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertPolicyId(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AcceptableCertPoliciesSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertPolicyId(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "AcceptableCertPoliciesSyntax",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertPolicyId  ::=  OBJECT IDENTIFIER
/// ```
pub type CertPolicyId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CertPolicyId(el: &X690Element) -> ASN1Result<CertPolicyId> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_CertPolicyId(value_: &CertPolicyId) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_CertPolicyId(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
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

pub mod acceptableCertPoliciesMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = AcceptableCertPoliciesSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_AcceptableCertPoliciesSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_AcceptableCertPoliciesSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_AcceptableCertPoliciesSyntax(el)
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

pub mod authorityAttributeIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AuthorityAttributeIdentifierSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AuthorityAttributeIdentifierSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AuthorityAttributeIdentifierSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AuthorityAttributeIdentifierSyntax(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthorityAttributeIdentifierSyntax",
            ))
        }
    };
    let mut items: SEQUENCE_OF<AuthAttId> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AuthAttId(el)?);
    }
    Ok(items)
}

pub fn _encode_AuthorityAttributeIdentifierSyntax(
    value_: &AuthorityAttributeIdentifierSyntax,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AuthAttId(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AuthorityAttributeIdentifierSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AuthAttId(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "AuthorityAttributeIdentifierSyntax",
        )),
    }
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

pub fn _validate_AuthAttId(el: &X690Element) -> ASN1Result<()> {
    _validate_IssuerSerial(&el)
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

pub mod authAttIdMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = AuthorityAttributeIdentifierSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_AuthorityAttributeIdentifierSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_AuthorityAttributeIdentifierSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_AuthorityAttributeIdentifierSyntax(el)
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

pub mod indirectIssuer {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_null(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod issuedOnBehalfOf {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = GeneralName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_GeneralName(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_GeneralName(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_GeneralName(el)
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

pub mod noAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_null(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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

pub mod allowedAttributeAssignments {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AllowedAttributeAssignments; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AllowedAttributeAssignments(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AllowedAttributeAssignments(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AllowedAttributeAssignments(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AllowedAttributeAssignments",
            ))
        }
    };
    let mut items: SET_OF<AllowedAttributeAssignments_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AllowedAttributeAssignments_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_AllowedAttributeAssignments(
    value_: &AllowedAttributeAssignments,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AllowedAttributeAssignments_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AllowedAttributeAssignments(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AllowedAttributeAssignments_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "AllowedAttributeAssignments",
        )),
    }
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

pub mod attributeMappings {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AttributeMappings; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AttributeMappings(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AttributeMappings(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeMappings(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeMappings")
            )
        }
    };
    let mut items: SET_OF<AttributeMappings_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AttributeMappings_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_AttributeMappings(value_: &AttributeMappings) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AttributeMappings_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AttributeMappings(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AttributeMappings_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeMappings")),
    }
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

pub mod holderNameConstraints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = HolderNameConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_HolderNameConstraintsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_HolderNameConstraintsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_HolderNameConstraintsSyntax(el)
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
impl TryFrom<&X690Element> for HolderNameConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "HolderNameConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HolderNameConstraintsSyntax,
        _eal_components_for_HolderNameConstraintsSyntax,
        _rctl2_components_for_HolderNameConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut permittedSubtrees_: OPTIONAL<GeneralSubtrees> = None;
    let mut excludedSubtrees_: OPTIONAL<GeneralSubtrees> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "permittedSubtrees" => permittedSubtrees_ = Some(_decode_GeneralSubtrees(_el)?),
            "excludedSubtrees" => excludedSubtrees_ = Some(_decode_GeneralSubtrees(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(HolderNameConstraintsSyntax {
        permittedSubtrees: permittedSubtrees_.unwrap(),
        excludedSubtrees: excludedSubtrees_,
        _unrecognized,
    })
}

pub fn _encode_HolderNameConstraintsSyntax(
    value_: &HolderNameConstraintsSyntax,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.permittedSubtrees)?);
    if let Some(v_) = &value_.excludedSubtrees {
        components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_HolderNameConstraintsSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "HolderNameConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HolderNameConstraintsSyntax,
        _eal_components_for_HolderNameConstraintsSyntax,
        _rctl2_components_for_HolderNameConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "permittedSubtrees" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "permittedSubtrees",
                    ));
                }
                Ok(_validate_GeneralSubtrees(&el)?)
            }(_el)?,
            "excludedSubtrees" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "excludedSubtrees",
                    ));
                }
                Ok(_validate_GeneralSubtrees(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralSubtrees  ::=  SEQUENCE SIZE (1..MAX) OF GeneralSubtree
/// ```
pub type GeneralSubtrees = Vec<GeneralSubtree>; // SequenceOfType

pub fn _decode_GeneralSubtrees(el: &X690Element) -> ASN1Result<GeneralSubtrees> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtrees"))
        }
    };
    let mut items: SEQUENCE_OF<GeneralSubtree> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_GeneralSubtree(el)?);
    }
    Ok(items)
}

pub fn _encode_GeneralSubtrees(value_: &GeneralSubtrees) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_GeneralSubtree(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_GeneralSubtrees(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_GeneralSubtree(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtrees")),
    }
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
        Vec::from([ 0 ]) // TODO: Convert this to an i64
    }
}
impl TryFrom<&X690Element> for GeneralSubtree {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtree"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_GeneralSubtree,
        _eal_components_for_GeneralSubtree,
        _rctl2_components_for_GeneralSubtree,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut base_: OPTIONAL<GeneralName> = None;
    let mut minimum_: OPTIONAL<BaseDistance> = None;
    let mut maximum_: OPTIONAL<BaseDistance> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "base" => base_ = Some(_decode_GeneralName(_el)?),
            "minimum" => minimum_ = Some(_decode_BaseDistance(_el)?),
            "maximum" => maximum_ = Some(_decode_BaseDistance(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(GeneralSubtree {
        base: base_.unwrap(),
        minimum: minimum_,
        maximum: maximum_,
        _unrecognized,
    })
}

pub fn _encode_GeneralSubtree(value_: &GeneralSubtree) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_GeneralName(&value_.base)?);
    if let Some(v_) = &value_.minimum {
        if *v_ != GeneralSubtree::_default_value_for_minimum() {
            components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_BaseDistance(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.maximum {
        components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_BaseDistance(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_GeneralSubtree(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtree"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_GeneralSubtree,
        _eal_components_for_GeneralSubtree,
        _rctl2_components_for_GeneralSubtree,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "base" => _validate_GeneralName(_el)?,
            "minimum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minimum")
                    );
                }
                Ok(_validate_BaseDistance(&el)?)
            }(_el)?,
            "maximum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maximum")
                    );
                }
                Ok(_validate_BaseDistance(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BaseDistance  ::=  INTEGER(0..MAX)
/// ```
pub type BaseDistance = INTEGER;

pub fn _decode_BaseDistance(el: &X690Element) -> ASN1Result<BaseDistance> {
    BER.decode_integer(&el)
}

pub fn _encode_BaseDistance(value_: &BaseDistance) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_BaseDistance(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([attributeCertificateAttribute()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiUser(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod pmiUser {
    /* OBJECT_TYPES */
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            aACertificate(),
            attributeCertificateRevocationList(),
            eeAttrCertificateRevocationList(),
            attributeAuthorityRevocationList(),
        ])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiAA(),                      /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod pmiAA {
    /* OBJECT_TYPES */
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            attributeCertificateRevocationList(),
            eeAttrCertificateRevocationList(),
            attributeAuthorityRevocationList(),
            attributeDescriptorCertificate(),
        ])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiSOA(),                     /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod pmiSOA {
    /* OBJECT_TYPES */
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            attributeCertificateRevocationList(),
            eeAttrCertificateRevocationList(),
            attributeAuthorityRevocationList(),
        ])), /* OBJECT_FIELD_SETTING */
        id: id_oc_attCertCRLDistributionPts(),  /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod attCertCRLDistributionPt {
    /* OBJECT_TYPES */
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([delegationPath()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_pmiDelegationPath(),          /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod pmiDelegationPath {
    /* OBJECT_TYPES */
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([privPolicy()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_privilegePolicy(),            /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod privilegePolicy {
    /* OBJECT_TYPES */
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
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([protPrivPolicy()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_protectedPrivilegePolicy(),   /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod protectedPrivilegePolicy {
    /* OBJECT_TYPES */
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
        id: id_at_attributeCertificate(),                       /* OBJECT_FIELD_SETTING */
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

pub mod attributeCertificateAttribute {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeCertificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeCertificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeCertificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCertificate(el)
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
        id: id_at_aACertificate(),                              /* OBJECT_FIELD_SETTING */
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

pub mod aACertificate {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeCertificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeCertificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeCertificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCertificate(el)
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
        id: id_at_attributeDescriptorCertificate(),             /* OBJECT_FIELD_SETTING */
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

pub mod attributeDescriptorCertificate {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeCertificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeCertificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeCertificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCertificate(el)
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
        ldapSyntax: Some(x509CertificateList().id),        /* OBJECT_FIELD_SETTING */
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

pub mod attributeCertificateRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
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
        ldapSyntax: Some(x509CertificateList().id),        /* OBJECT_FIELD_SETTING */
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

pub mod eeAttrCertificateRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
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
        ldapSyntax: Some(x509CertificateList().id),        /* OBJECT_FIELD_SETTING */
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

pub mod attributeAuthorityRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
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

pub mod delegationPath {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttCertPath; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttCertPath(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttCertPath(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttCertPath(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertPath  ::=  SEQUENCE OF AttributeCertificate
/// ```
pub type AttCertPath = Vec<AttributeCertificate>; // SequenceOfType

pub fn _decode_AttCertPath(el: &X690Element) -> ASN1Result<AttCertPath> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertPath")),
    };
    let mut items: SEQUENCE_OF<AttributeCertificate> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AttributeCertificate(el)?);
    }
    Ok(items)
}

pub fn _encode_AttCertPath(value_: &AttCertPath) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AttributeCertificate(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AttCertPath(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AttributeCertificate(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertPath")),
    }
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

pub mod privPolicy {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PolicySyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PolicySyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PolicySyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PolicySyntax(el)
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
        id: id_at_protPrivPolicy(),                             /* OBJECT_FIELD_SETTING */
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

pub mod protPrivPolicy {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeCertificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeCertificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeCertificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCertificate(el)
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

pub mod xmlPrivPolicy {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UTF8String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_utf8_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_utf8_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utf8_string(el)
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

pub mod attributeCertificateExactMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = AttributeCertificateExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_AttributeCertificateExactAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_AttributeCertificateExactAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCertificateExactAssertion(el)
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
impl TryFrom<&X690Element> for AttributeCertificateExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeCertificateExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeCertificateExactAssertion,
        _eal_components_for_AttributeCertificateExactAssertion,
        _rctl2_components_for_AttributeCertificateExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut issuer_: OPTIONAL<AttCertIssuer> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "issuer" => issuer_ = Some(_decode_AttCertIssuer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeCertificateExactAssertion {
        serialNumber: serialNumber_.unwrap(),
        issuer: issuer_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttributeCertificateExactAssertion(
    value_: &AttributeCertificateExactAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    components_.push(_encode_AttCertIssuer(&value_.issuer)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeCertificateExactAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeCertificateExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeCertificateExactAssertion,
        _eal_components_for_AttributeCertificateExactAssertion,
        _rctl2_components_for_AttributeCertificateExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "issuer" => _validate_AttCertIssuer(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod attributeCertificateMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = AttributeCertificateAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_AttributeCertificateAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_AttributeCertificateAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCertificateAssertion(el)
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
impl TryFrom<&X690Element> for AttributeCertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeCertificateAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeCertificateAssertion,
        _eal_components_for_AttributeCertificateAssertion,
        _rctl2_components_for_AttributeCertificateAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut holder_: OPTIONAL<AttributeCertificateAssertion_holder> = None;
    let mut issuer_: OPTIONAL<GeneralNames> = None;
    let mut attCertValidity_: OPTIONAL<GeneralizedTime> = None;
    let mut attType_: OPTIONAL<Vec<AttributeType>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "holder" => {
                holder_ = Some(
                    |el: &X690Element| -> ASN1Result<AttributeCertificateAssertion_holder> {
                        Ok(_decode_AttributeCertificateAssertion_holder(&el.inner()?)?)
                    }(_el)?,
                )
            }
            "issuer" => issuer_ = Some(_decode_GeneralNames(_el)?),
            "attCertValidity" => attCertValidity_ = Some(BER.decode_generalized_time(_el)?),
            "attType" => {
                attType_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attType"))
                        }
                    };
                    let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_AttributeType(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeCertificateAssertion {
        holder: holder_,
        issuer: issuer_,
        attCertValidity: attCertValidity_,
        attType: attType_,
        _unrecognized,
    })
}

pub fn _encode_AttributeCertificateAssertion(
    value_: &AttributeCertificateAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.holder {
        components_.push(
            |v_1: &AttributeCertificateAssertion_holder| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_AttributeCertificateAssertion_holder(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.issuer {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.attCertValidity {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
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
                    Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeCertificateAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeCertificateAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeCertificateAssertion,
        _eal_components_for_AttributeCertificateAssertion,
        _rctl2_components_for_AttributeCertificateAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "holder" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "holder"));
                }
                Ok(_validate_AttributeCertificateAssertion_holder(
                    &el.inner()?,
                )?)
            }(_el)?,
            "issuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuer"));
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            "attCertValidity" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "attCertValidity",
                    ));
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            "attType" => {
                |el: &X690Element| -> ASN1Result<()> {
                    if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                        return Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attType")
                        );
                    }
                    Ok(|el: &X690Element| -> ASN1Result<()> {
                        match &el.value {
                            X690Value::Constructed(subs) => {
                                for sub in subs.iter() {
                                    _validate_AttributeType(&sub)?;
                                }
                                Ok(())
                            }
                            _ => Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attType")),
                        }
                    }(&el)?)
                }(_el)?
            }
            _ => (),
        }
    }
    Ok(())
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

pub mod holderIssuerMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = HolderIssuerAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_HolderIssuerAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_HolderIssuerAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_HolderIssuerAssertion(el)
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
impl TryFrom<&X690Element> for HolderIssuerAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HolderIssuerAssertion")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HolderIssuerAssertion,
        _eal_components_for_HolderIssuerAssertion,
        _rctl2_components_for_HolderIssuerAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut holder_: OPTIONAL<Holder> = None;
    let mut issuer_: OPTIONAL<AttCertIssuer> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "holder" => holder_ = Some(_decode_Holder(_el)?),
            "issuer" => issuer_ = Some(_decode_AttCertIssuer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(HolderIssuerAssertion {
        holder: holder_,
        issuer: issuer_,
        _unrecognized,
    })
}

pub fn _encode_HolderIssuerAssertion(value_: &HolderIssuerAssertion) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.holder {
        components_.push(|v_1: &Holder| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Holder(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.issuer {
        components_.push(|v_1: &AttCertIssuer| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AttCertIssuer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_HolderIssuerAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HolderIssuerAssertion")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HolderIssuerAssertion,
        _eal_components_for_HolderIssuerAssertion,
        _rctl2_components_for_HolderIssuerAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "holder" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "holder"));
                }
                Ok(_validate_Holder(&el)?)
            }(_el)?,
            "issuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuer"));
                }
                Ok(_validate_AttCertIssuer(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod delegationPathMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = DelMatchSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_DelMatchSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_DelMatchSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_DelMatchSyntax(el)
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
impl TryFrom<&X690Element> for DelMatchSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelMatchSyntax"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DelMatchSyntax,
        _eal_components_for_DelMatchSyntax,
        _rctl2_components_for_DelMatchSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut firstIssuer_: OPTIONAL<AttCertIssuer> = None;
    let mut lastHolder_: OPTIONAL<Holder> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "firstIssuer" => firstIssuer_ = Some(_decode_AttCertIssuer(_el)?),
            "lastHolder" => lastHolder_ = Some(_decode_Holder(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DelMatchSyntax {
        firstIssuer: firstIssuer_.unwrap(),
        lastHolder: lastHolder_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DelMatchSyntax(value_: &DelMatchSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AttCertIssuer(&value_.firstIssuer)?);
    components_.push(_encode_Holder(&value_.lastHolder)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DelMatchSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelMatchSyntax"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DelMatchSyntax,
        _eal_components_for_DelMatchSyntax,
        _rctl2_components_for_DelMatchSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "firstIssuer" => _validate_AttCertIssuer(_el)?,
            "lastHolder" => _validate_Holder(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod extensionPresenceMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = OBJECT_IDENTIFIER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_object_identifier(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_object_identifier(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_object_identifier(el)
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
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([24])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pmiAA                              OBJECT IDENTIFIER ::= {id-oc 25}
/// ```
///
///
pub fn id_oc_pmiAA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pmiSOA                             OBJECT IDENTIFIER ::= {id-oc 26}
/// ```
///
///
pub fn id_oc_pmiSOA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-attCertCRLDistributionPts          OBJECT IDENTIFIER ::= {id-oc 27}
/// ```
///
///
pub fn id_oc_attCertCRLDistributionPts() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-privilegePolicy                    OBJECT IDENTIFIER ::= {id-oc 32}
/// ```
///
///
pub fn id_oc_privilegePolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([32])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pmiDelegationPath                  OBJECT IDENTIFIER ::= {id-oc 33}
/// ```
///
///
pub fn id_oc_pmiDelegationPath() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([33])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-protectedPrivilegePolicy           OBJECT IDENTIFIER ::= {id-oc 34}
/// ```
///
///
pub fn id_oc_protectedPrivilegePolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([34])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeCertificate               OBJECT IDENTIFIER ::= {id-at 58}
/// ```
///
///
pub fn id_at_attributeCertificate() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([58])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeCertificateRevocationList OBJECT IDENTIFIER ::= {id-at 59}
/// ```
///
///
pub fn id_at_attributeCertificateRevocationList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([59])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-aACertificate                      OBJECT IDENTIFIER ::= {id-at 61}
/// ```
///
///
pub fn id_at_aACertificate() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([61])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeDescriptorCertificate     OBJECT IDENTIFIER ::= {id-at 62}
/// ```
///
///
pub fn id_at_attributeDescriptorCertificate() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([62])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeAuthorityRevocationList   OBJECT IDENTIFIER ::= {id-at 63}
/// ```
///
///
pub fn id_at_attributeAuthorityRevocationList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([63])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-privPolicy                         OBJECT IDENTIFIER ::= {id-at 71}
/// ```
///
///
pub fn id_at_privPolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([71])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-role                               OBJECT IDENTIFIER ::= {id-at 72}
/// ```
///
///
pub fn id_at_role() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([72])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-delegationPath                     OBJECT IDENTIFIER ::= {id-at 73}
/// ```
///
///
pub fn id_at_delegationPath() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([73])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-protPrivPolicy                     OBJECT IDENTIFIER ::= {id-at 74}
/// ```
///
///
pub fn id_at_protPrivPolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([74])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-xMLPrivilegeInfo                   OBJECT IDENTIFIER ::= {id-at 75}
/// ```
///
///
pub fn id_at_xMLPrivilegeInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([75])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-xmlPrivPolicy                      OBJECT IDENTIFIER ::= {id-at 76}
/// ```
///
///
pub fn id_at_xmlPrivPolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([76])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-permission                         OBJECT IDENTIFIER ::= {id-at 82}
/// ```
///
///
pub fn id_at_permission() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([82])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-eeAttrCertificateRevocationList    OBJECT IDENTIFIER ::= {id-at 102}
/// ```
///
///
pub fn id_at_eeAttrCertificateRevocationList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([102])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-authorityAttributeIdentifier       OBJECT IDENTIFIER ::= {id-ce 38}
/// ```
///
///
pub fn id_ce_authorityAttributeIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([38])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-roleSpecCertIdentifier             OBJECT IDENTIFIER ::= {id-ce 39}
/// ```
///
///
pub fn id_ce_roleSpecCertIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([39])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-basicAttConstraints                OBJECT IDENTIFIER ::= {id-ce 41}
/// ```
///
///
pub fn id_ce_basicAttConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([41])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-delegatedNameConstraints           OBJECT IDENTIFIER ::= {id-ce 42}
/// ```
///
///
pub fn id_ce_delegatedNameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([42])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-timeSpecification                  OBJECT IDENTIFIER ::= {id-ce 43}
/// ```
///
///
pub fn id_ce_timeSpecification() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([43])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-attributeDescriptor                OBJECT IDENTIFIER ::= {id-ce 48}
/// ```
///
///
pub fn id_ce_attributeDescriptor() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([48])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-userNotice                         OBJECT IDENTIFIER ::= {id-ce 49}
/// ```
///
///
pub fn id_ce_userNotice() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([49])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-sOAIdentifier                      OBJECT IDENTIFIER ::= {id-ce 50}
/// ```
///
///
pub fn id_ce_sOAIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([50])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-acceptableCertPolicies             OBJECT IDENTIFIER ::= {id-ce 52}
/// ```
///
///
pub fn id_ce_acceptableCertPolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([52])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-targetingInformation               OBJECT IDENTIFIER ::= {id-ce 55}
/// ```
///
///
pub fn id_ce_targetingInformation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([55])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-noRevAvail                         OBJECT IDENTIFIER ::= {id-ce 56}
/// ```
///
///
pub fn id_ce_noRevAvail() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([56])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-acceptablePrivilegePolicies        OBJECT IDENTIFIER ::= {id-ce 57}
/// ```
///
///
pub fn id_ce_acceptablePrivilegePolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([57])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-indirectIssuer                     OBJECT IDENTIFIER ::= {id-ce 61}
/// ```
///
///
pub fn id_ce_indirectIssuer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([61])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-noAssertion                        OBJECT IDENTIFIER ::= {id-ce 62}
/// ```
///
///
pub fn id_ce_noAssertion() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([62])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-issuedOnBehalfOf                   OBJECT IDENTIFIER ::= {id-ce 64}
/// ```
///
///
pub fn id_ce_issuedOnBehalfOf() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([64])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-singleUse                          OBJECT IDENTIFIER ::= {id-ce 65}
/// ```
///
///
pub fn id_ce_singleUse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([65])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-groupAC                            OBJECT IDENTIFIER ::= {id-ce 66}
/// ```
///
///
pub fn id_ce_groupAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([66])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-allowedAttributeAssignments        OBJECT IDENTIFIER ::= {id-ce 67}
/// ```
///
///
pub fn id_ce_allowedAttributeAssignments() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([67])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-attributeMappings                  OBJECT IDENTIFIER ::= {id-ce 68}
/// ```
///
///
pub fn id_ce_attributeMappings() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([68])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-holderNameConstraints              OBJECT IDENTIFIER ::= {id-ce 69}
/// ```
///
///
pub fn id_ce_holderNameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([69])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-attributeCertificateMatch          OBJECT IDENTIFIER ::= {id-mr 42}
/// ```
///
///
pub fn id_mr_attributeCertificateMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([42])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-attributeCertificateExactMatch     OBJECT IDENTIFIER ::= {id-mr 45}
/// ```
///
///
pub fn id_mr_attributeCertificateExactMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([45])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-holderIssuerMatch                  OBJECT IDENTIFIER ::= {id-mr 46}
/// ```
///
///
pub fn id_mr_holderIssuerMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([46])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-authAttIdMatch                     OBJECT IDENTIFIER ::= {id-mr 53}
/// ```
///
///
pub fn id_mr_authAttIdMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([53])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-roleSpecCertIdMatch                OBJECT IDENTIFIER ::= {id-mr 54}
/// ```
///
///
pub fn id_mr_roleSpecCertIdMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([54])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-basicAttConstraintsMatch           OBJECT IDENTIFIER ::= {id-mr 55}
/// ```
///
///
pub fn id_mr_basicAttConstraintsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([55])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-delegatedNameConstraintsMatch      OBJECT IDENTIFIER ::= {id-mr 56}
/// ```
///
///
pub fn id_mr_delegatedNameConstraintsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([56])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-timeSpecMatch                      OBJECT IDENTIFIER ::= {id-mr 57}
/// ```
///
///
pub fn id_mr_timeSpecMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([57])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-attDescriptorMatch                 OBJECT IDENTIFIER ::= {id-mr 58}
/// ```
///
///
pub fn id_mr_attDescriptorMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([58])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-acceptableCertPoliciesMatch        OBJECT IDENTIFIER ::= {id-mr 59}
/// ```
///
///
pub fn id_mr_acceptableCertPoliciesMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([59])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-delegationPathMatch                OBJECT IDENTIFIER ::= {id-mr 61}
/// ```
///
///
pub fn id_mr_delegationPathMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([61])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-sOAIdentifierMatch                 OBJECT IDENTIFIER ::= {id-mr 66}
/// ```
///
///
pub fn id_mr_sOAIdentifierMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([66])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-extensionPresenceMatch             OBJECT IDENTIFIER ::= {id-mr 67}
/// ```
///
///
pub fn id_mr_extensionPresenceMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([67])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-dualStringMatch                    OBJECT IDENTIFIER ::= {id-mr 69}
/// ```
///
///
pub fn id_mr_dualStringMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([69])].concat()) // OID_GETTER
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
    BER.decode_enumerated(&el)
}

pub fn _encode_ObjectDigestInfo_digestedObjectType(
    value_: &ObjectDigestInfo_digestedObjectType,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_ObjectDigestInfo_digestedObjectType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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

impl TryFrom<&X690Element> for AllowedAttributeAssignments_Item_attributes_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AllowedAttributeAssignments_Item_attributes_Item(el)
    }
}

pub fn _decode_AllowedAttributeAssignments_Item_attributes_Item(
    el: &X690Element,
) -> ASN1Result<AllowedAttributeAssignments_Item_attributes_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(
            AllowedAttributeAssignments_Item_attributes_Item::attributeType(_decode_AttributeType(
                &el,
            )?),
        ),
        (TagClass::CONTEXT, 1) => Ok(
            AllowedAttributeAssignments_Item_attributes_Item::attributeTypeandValues(
                _decode_Attribute(&el)?,
            ),
        ),
        _ => Ok(AllowedAttributeAssignments_Item_attributes_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_AllowedAttributeAssignments_Item_attributes_Item(
    value_: &AllowedAttributeAssignments_Item_attributes_Item,
) -> ASN1Result<X690Element> {
    match value_ {
        AllowedAttributeAssignments_Item_attributes_Item::attributeType(v) => {
            |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeType(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        AllowedAttributeAssignments_Item_attributes_Item::attributeTypeandValues(v) => {
            |v_1: &Attribute| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Attribute(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        AllowedAttributeAssignments_Item_attributes_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_AllowedAttributeAssignments_Item_attributes_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeType")
                );
            }
            Ok(_validate_AttributeType(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "attributeTypeandValues",
                ));
            }
            Ok(_validate_Attribute(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AllowedAttributeAssignments-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for AllowedAttributeAssignments_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AllowedAttributeAssignments-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AllowedAttributeAssignments_Item,
        _eal_components_for_AllowedAttributeAssignments_Item,
        _rctl2_components_for_AllowedAttributeAssignments_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attributes_: OPTIONAL<Vec<AllowedAttributeAssignments_Item_attributes_Item>> = None;
    let mut holderDomain_: OPTIONAL<GeneralName> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributes" => {
                attributes_ = Some(|el: &X690Element| -> ASN1Result<
                    SET_OF<AllowedAttributeAssignments_Item_attributes_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "attributes",
                            ))
                        }
                    };
                    let mut items: SET_OF<AllowedAttributeAssignments_Item_attributes_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_AllowedAttributeAssignments_Item_attributes_Item(
                            el,
                        )?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "holderDomain" => {
                holderDomain_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AllowedAttributeAssignments_Item {
        attributes: attributes_.unwrap(),
        holderDomain: holderDomain_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AllowedAttributeAssignments_Item(
    value_: &AllowedAttributeAssignments_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(|v_1: &Vec<AllowedAttributeAssignments_Item_attributes_Item>| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SET_OF<AllowedAttributeAssignments_Item_attributes_Item>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AllowedAttributeAssignments_Item_attributes_Item(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&value_.attributes)?);
    components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
        ))
    }(&value_.holderDomain)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AllowedAttributeAssignments_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AllowedAttributeAssignments-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AllowedAttributeAssignments_Item,
        _eal_components_for_AllowedAttributeAssignments_Item,
        _rctl2_components_for_AllowedAttributeAssignments_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributes")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_AllowedAttributeAssignments_Item_attributes_Item(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributes")
                        ),
                    }
                }(&el)?)
            }(_el)?,
            "holderDomain" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "holderDomain")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeMappings-Item-typeMappings ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for AttributeMappings_Item_typeMappings {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeMappings-Item-typeMappings",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeMappings_Item_typeMappings,
        _eal_components_for_AttributeMappings_Item_typeMappings,
        _rctl2_components_for_AttributeMappings_Item_typeMappings,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut local_: OPTIONAL<AttributeType> = None;
    let mut remote_: OPTIONAL<AttributeType> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "local" => local_ = Some(_decode_AttributeType(_el)?),
            "remote" => remote_ = Some(_decode_AttributeType(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeMappings_Item_typeMappings {
        local: local_.unwrap(),
        remote: remote_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttributeMappings_Item_typeMappings(
    value_: &AttributeMappings_Item_typeMappings,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_AttributeType(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.local)?);
    components_.push(|v_1: &AttributeType| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_AttributeType(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.remote)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeMappings_Item_typeMappings(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeMappings-Item-typeMappings",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeMappings_Item_typeMappings,
        _eal_components_for_AttributeMappings_Item_typeMappings,
        _rctl2_components_for_AttributeMappings_Item_typeMappings,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "local" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "local"));
                }
                Ok(_validate_AttributeType(&el)?)
            }(_el)?,
            "remote" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "remote"));
                }
                Ok(_validate_AttributeType(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeMappings-Item-typeValueMappings ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for AttributeMappings_Item_typeValueMappings {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeMappings-Item-typeValueMappings",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeMappings_Item_typeValueMappings,
        _eal_components_for_AttributeMappings_Item_typeValueMappings,
        _rctl2_components_for_AttributeMappings_Item_typeValueMappings,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut local_: OPTIONAL<AttributeTypeAndValue> = None;
    let mut remote_: OPTIONAL<AttributeTypeAndValue> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "local" => local_ = Some(_decode_AttributeTypeAndValue(_el)?),
            "remote" => remote_ = Some(_decode_AttributeTypeAndValue(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeMappings_Item_typeValueMappings {
        local: local_.unwrap(),
        remote: remote_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttributeMappings_Item_typeValueMappings(
    value_: &AttributeMappings_Item_typeValueMappings,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(|v_1: &AttributeTypeAndValue| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_AttributeTypeAndValue(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.local)?);
    components_.push(|v_1: &AttributeTypeAndValue| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_AttributeTypeAndValue(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.remote)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeMappings_Item_typeValueMappings(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeMappings-Item-typeValueMappings",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeMappings_Item_typeValueMappings,
        _eal_components_for_AttributeMappings_Item_typeValueMappings,
        _rctl2_components_for_AttributeMappings_Item_typeValueMappings,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "local" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "local"));
                }
                Ok(_validate_AttributeTypeAndValue(&el)?)
            }(_el)?,
            "remote" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "remote"));
                }
                Ok(_validate_AttributeTypeAndValue(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for AttributeMappings_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeMappings_Item(el)
    }
}

pub fn _decode_AttributeMappings_Item(el: &X690Element) -> ASN1Result<AttributeMappings_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(AttributeMappings_Item::typeMappings(
            _decode_AttributeMappings_Item_typeMappings(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(AttributeMappings_Item::typeValueMappings(
            _decode_AttributeMappings_Item_typeValueMappings(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "AttributeMappings-Item",
            ))
        }
    }
}

pub fn _encode_AttributeMappings_Item(value_: &AttributeMappings_Item) -> ASN1Result<X690Element> {
    match value_ {
        AttributeMappings_Item::typeMappings(v) => {
            |v_1: &AttributeMappings_Item_typeMappings| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeMappings_Item_typeMappings(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        AttributeMappings_Item::typeValueMappings(v) => {
            |v_1: &AttributeMappings_Item_typeValueMappings| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeMappings_Item_typeValueMappings(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_AttributeMappings_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "typeMappings")
                );
            }
            Ok(_validate_AttributeMappings_Item_typeMappings(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "typeValueMappings")
                );
            }
            Ok(_validate_AttributeMappings_Item_typeValueMappings(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "AttributeMappings-Item",
            ))
        }
    }
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

impl TryFrom<&X690Element> for AttributeCertificateAssertion_holder {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCertificateAssertion_holder(el)
    }
}

pub fn _decode_AttributeCertificateAssertion_holder(
    el: &X690Element,
) -> ASN1Result<AttributeCertificateAssertion_holder> {
    match (el.tag.tag_class, el.tag.tag_number) {
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
}

pub fn _encode_AttributeCertificateAssertion_holder(
    value_: &AttributeCertificateAssertion_holder,
) -> ASN1Result<X690Element> {
    match value_ {
        AttributeCertificateAssertion_holder::baseCertificateID(v) => {
            |v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_IssuerSerial(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        AttributeCertificateAssertion_holder::holderName(v) => {
            |v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        AttributeCertificateAssertion_holder::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_AttributeCertificateAssertion_holder(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "baseCertificateID")
                );
            }
            Ok(_validate_IssuerSerial(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "holderName"));
            }
            Ok(_validate_GeneralNames(&el)?)
        }(&el),
        _ => Ok(()),
    }
}
