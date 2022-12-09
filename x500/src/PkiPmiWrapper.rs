#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PkiPmiWrapper
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PkiPmiWrapper`.
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
use crate::InformationFramework::*;
use crate::SelectedAttributeTypes::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

pub type WRAPPED_PDU = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// PDU-wrapper  ::=  SIGNED{TBSPDU-wrapper}
/// ```
pub type PDU_wrapper = SIGNED<TBSPDU_wrapper>; // DefinedType

pub fn _decode_PDU_wrapper(el: &X690Element) -> ASN1Result<PDU_wrapper> {
    _decode_SIGNED::<TBSPDU_wrapper>(_decode_TBSPDU_wrapper, &el)
}

pub fn _encode_PDU_wrapper(value_: &PDU_wrapper) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSPDU_wrapper>(_encode_TBSPDU_wrapper, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSPDU-wrapper ::= SEQUENCE  {
///   version               Version DEFAULT v1,
///   signatureAlgorithm    AlgorithmIdentifier {{SupportedSignatureAlgorithms}},
///   certPath         [0]  IMPLICIT PkiPath,
///   signedAttrs      [1]  IMPLICIT SignedAttributes OPTIONAL,
///   conf                  CHOICE {
///     clear            [2]  WrappedPDUInfo,
///     protected        [3]  EncryptedInfo,
///    ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TBSPDU_wrapper {
    pub version: OPTIONAL<Version>,
    pub signatureAlgorithm: AlgorithmIdentifier,
    pub certPath: PkiPath,
    pub signedAttrs: OPTIONAL<SignedAttributes>,
    pub conf: TBSPDU_wrapper_conf,
    pub _unrecognized: Vec<X690Element>,
}
impl TBSPDU_wrapper {
    pub fn new(
        version: OPTIONAL<Version>,
        signatureAlgorithm: AlgorithmIdentifier,
        certPath: PkiPath,
        signedAttrs: OPTIONAL<SignedAttributes>,
        conf: TBSPDU_wrapper_conf,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TBSPDU_wrapper {
            version,
            signatureAlgorithm,
            certPath,
            signedAttrs,
            conf,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        Version_v1
    }
}
impl TryFrom<X690Element> for TBSPDU_wrapper {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSPDU_wrapper(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSPDU_wrapper {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TBSPDU_wrapper(el)
    }
}

pub const _rctl1_components_for_TBSPDU_wrapper: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signatureAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certPath",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signedAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new("conf", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_TBSPDU_wrapper: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TBSPDU_wrapper: &[ComponentSpec; 0] = &[];

pub fn _decode_TBSPDU_wrapper(el: &X690Element) -> ASN1Result<TBSPDU_wrapper> {
    |el_: &X690Element| -> ASN1Result<TBSPDU_wrapper> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TBSPDU_wrapper,
            _eal_components_for_TBSPDU_wrapper,
            _rctl2_components_for_TBSPDU_wrapper,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let signatureAlgorithm =
            _decode_AlgorithmIdentifier(_components.get("signatureAlgorithm").unwrap())?;
        let certPath = _decode_PkiPath(_components.get("certPath").unwrap())?;
        let signedAttrs: OPTIONAL<SignedAttributes> = match _components.get("signedAttrs") {
            Some(c_) => Some(_decode_SignedAttributes(c_)?),
            _ => None,
        };
        let conf = _decode_TBSPDU_wrapper_conf(_components.get("conf").unwrap())?;
        Ok(TBSPDU_wrapper {
            version,
            signatureAlgorithm,
            certPath,
            signedAttrs,
            conf,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TBSPDU_wrapper(value_: &TBSPDU_wrapper) -> ASN1Result<X690Element> {
    |value_: &TBSPDU_wrapper| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.version {
            if *v_ != TBSPDU_wrapper::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.signatureAlgorithm)?);
        components_.push(|v_1: &PkiPath| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_PkiPath(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.certPath)?);
        if let Some(v_) = &value_.signedAttrs {
            components_.push(|v_1: &SignedAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SignedAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_TBSPDU_wrapper_conf(&value_.conf)?);
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
/// SupportedSignatureAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedSignatureAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedAttributes  ::=  SET SIZE (1..MAX) OF Attribute{{SupportedSignedAttributes}}
/// ```
pub type SignedAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_SignedAttributes(el: &X690Element) -> ASN1Result<SignedAttributes> {
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
    }(&el)
}

pub fn _encode_SignedAttributes(value_: &SignedAttributes) -> ASN1Result<X690Element> {
    |value_: &SET_OF<Attribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Attribute(&v)?);
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
/// SupportedSignedAttributes ATTRIBUTE ::= { contentType | messageDigest }
/// ```
///
///
pub fn SupportedSignedAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([contentType(), messageDigest()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// WrappedPDUInfo ::= SEQUENCE {
///   pduType      WRAPPED-PDU.&id ({SupportedPduSet}),
///   pduInfo      WRAPPED-PDU.&Type ({SupportedPduSet}{@pduType}),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct WrappedPDUInfo {
    pub pduType: OBJECT_IDENTIFIER,
    pub pduInfo: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl WrappedPDUInfo {
    pub fn new(
        pduType: OBJECT_IDENTIFIER,
        pduInfo: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        WrappedPDUInfo {
            pduType,
            pduInfo,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for WrappedPDUInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_WrappedPDUInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for WrappedPDUInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_WrappedPDUInfo(el)
    }
}

pub const _rctl1_components_for_WrappedPDUInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "pduType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("pduInfo", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_WrappedPDUInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_WrappedPDUInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_WrappedPDUInfo(el: &X690Element) -> ASN1Result<WrappedPDUInfo> {
    |el_: &X690Element| -> ASN1Result<WrappedPDUInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_WrappedPDUInfo,
            _eal_components_for_WrappedPDUInfo,
            _rctl2_components_for_WrappedPDUInfo,
        )?;
        let pduType = ber_decode_object_identifier(_components.get("pduType").unwrap())?;
        let pduInfo = x690_identity(_components.get("pduInfo").unwrap())?;
        Ok(WrappedPDUInfo {
            pduType,
            pduInfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_WrappedPDUInfo(value_: &WrappedPDUInfo) -> ASN1Result<X690Element> {
    |value_: &WrappedPDUInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.pduType)?);
        components_.push(x690_identity(&value_.pduInfo)?);
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
/// SupportedPduSet WRAPPED-PDU ::= {...}
/// ```
///
///
pub fn SupportedPduSet() -> Vec<WRAPPED_PDU> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedInfo ::= SEQUENCE {
///   keyAgreement      KeyAgreement,
///   encryptedPduInfo  EncryptedPduInfo,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncryptedInfo {
    pub keyAgreement: KeyAgreement,
    pub encryptedPduInfo: EncryptedPduInfo,
    pub _unrecognized: Vec<X690Element>,
}
impl EncryptedInfo {
    pub fn new(
        keyAgreement: KeyAgreement,
        encryptedPduInfo: EncryptedPduInfo,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EncryptedInfo {
            keyAgreement,
            encryptedPduInfo,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for EncryptedInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncryptedInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedInfo(el)
    }
}

pub const _rctl1_components_for_EncryptedInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "keyAgreement",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedPduInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncryptedInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedInfo(el: &X690Element) -> ASN1Result<EncryptedInfo> {
    |el_: &X690Element| -> ASN1Result<EncryptedInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncryptedInfo,
            _eal_components_for_EncryptedInfo,
            _rctl2_components_for_EncryptedInfo,
        )?;
        let keyAgreement = _decode_KeyAgreement(_components.get("keyAgreement").unwrap())?;
        let encryptedPduInfo =
            _decode_EncryptedPduInfo(_components.get("encryptedPduInfo").unwrap())?;
        Ok(EncryptedInfo {
            keyAgreement,
            encryptedPduInfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EncryptedInfo(value_: &EncryptedInfo) -> ASN1Result<X690Element> {
    |value_: &EncryptedInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_KeyAgreement(&value_.keyAgreement)?);
        components_.push(_encode_EncryptedPduInfo(&value_.encryptedPduInfo)?);
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
/// KeyAgreement ::= SEQUENCE {
///   senderDhInfo       [0] SenderDhInfo,
///   keyEncryptionAlgorithm SEQUENCE {
///     algorithm    ALGORITHM.&id ({SupportedKeyEncryptionAlgorithm}),
///     parameters   ALGORITHM.&Type({SupportedKeyEncryptionAlgorithm}{@.algorithm}),
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct KeyAgreement {
    pub senderDhInfo: SenderDhInfo,
    pub keyEncryptionAlgorithm: KeyAgreement_keyEncryptionAlgorithm,
    pub _unrecognized: Vec<X690Element>,
}
impl KeyAgreement {
    pub fn new(
        senderDhInfo: SenderDhInfo,
        keyEncryptionAlgorithm: KeyAgreement_keyEncryptionAlgorithm,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        KeyAgreement {
            senderDhInfo,
            keyEncryptionAlgorithm,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for KeyAgreement {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreement(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KeyAgreement {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreement(el)
    }
}

pub const _rctl1_components_for_KeyAgreement: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "senderDhInfo",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyEncryptionAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_KeyAgreement: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_KeyAgreement: &[ComponentSpec; 0] = &[];

pub fn _decode_KeyAgreement(el: &X690Element) -> ASN1Result<KeyAgreement> {
    |el_: &X690Element| -> ASN1Result<KeyAgreement> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_KeyAgreement,
            _eal_components_for_KeyAgreement,
            _rctl2_components_for_KeyAgreement,
        )?;
        let senderDhInfo = |el: &X690Element| -> ASN1Result<SenderDhInfo> {
            Ok(_decode_SenderDhInfo(&el.inner()?)?)
        }(_components.get("senderDhInfo").unwrap())?;
        let keyEncryptionAlgorithm = _decode_KeyAgreement_keyEncryptionAlgorithm(
            _components.get("keyEncryptionAlgorithm").unwrap(),
        )?;
        Ok(KeyAgreement {
            senderDhInfo,
            keyEncryptionAlgorithm,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_KeyAgreement(value_: &KeyAgreement) -> ASN1Result<X690Element> {
    |value_: &KeyAgreement| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &SenderDhInfo| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_SenderDhInfo(
                    &v_1,
                )?))),
            ))
        }(&value_.senderDhInfo)?);
        components_.push(_encode_KeyAgreement_keyEncryptionAlgorithm(
            &value_.keyEncryptionAlgorithm,
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
/// SupportedKeyEncryptionAlgorithm ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedKeyEncryptionAlgorithm() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SenderDhInfo  ::=  CHOICE {
///   senderStaticInfo   [0] SenderStaticInfo,
///   senderDhPublicKey  [1] SenderDhPublicKey,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum SenderDhInfo {
    senderStaticInfo(SenderStaticInfo),
    senderDhPublicKey(SenderDhPublicKey),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for SenderDhInfo {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SenderDhInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SenderDhInfo {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SenderDhInfo(el)
    }
}

pub fn _decode_SenderDhInfo(el: &X690Element) -> ASN1Result<SenderDhInfo> {
    |el: &X690Element| -> ASN1Result<SenderDhInfo> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(SenderDhInfo::senderStaticInfo(_decode_SenderStaticInfo(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 1) => Ok(SenderDhInfo::senderDhPublicKey(
                _decode_SenderDhPublicKey(&el.inner()?)?,
            )),
            _ => Ok(SenderDhInfo::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_SenderDhInfo(value_: &SenderDhInfo) -> ASN1Result<X690Element> {
    |value: &SenderDhInfo| -> ASN1Result<X690Element> {
        match value {
            SenderDhInfo::senderStaticInfo(v) => {
                |v_1: &SenderStaticInfo| -> ASN1Result<X690Element> {
                    let el_1 = _encode_SenderStaticInfo(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            SenderDhInfo::senderDhPublicKey(v) => {
                |v_1: &SenderDhPublicKey| -> ASN1Result<X690Element> {
                    let el_1 = _encode_SenderDhPublicKey(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            SenderDhInfo::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SenderStaticInfo ::= SEQUENCE {
///   issuer       Name,
///   serialNumber CertificateSerialNumber,
///   partyAinfo   UserKeyingMaterial,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SenderStaticInfo {
    pub issuer: Name,
    pub serialNumber: CertificateSerialNumber,
    pub partyAinfo: UserKeyingMaterial,
    pub _unrecognized: Vec<X690Element>,
}
impl SenderStaticInfo {
    pub fn new(
        issuer: Name,
        serialNumber: CertificateSerialNumber,
        partyAinfo: UserKeyingMaterial,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SenderStaticInfo {
            issuer,
            serialNumber,
            partyAinfo,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SenderStaticInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SenderStaticInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SenderStaticInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SenderStaticInfo(el)
    }
}

pub const _rctl1_components_for_SenderStaticInfo: &[ComponentSpec; 3] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "partyAinfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SenderStaticInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SenderStaticInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_SenderStaticInfo(el: &X690Element) -> ASN1Result<SenderStaticInfo> {
    |el_: &X690Element| -> ASN1Result<SenderStaticInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SenderStaticInfo,
            _eal_components_for_SenderStaticInfo,
            _rctl2_components_for_SenderStaticInfo,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let partyAinfo = _decode_UserKeyingMaterial(_components.get("partyAinfo").unwrap())?;
        Ok(SenderStaticInfo {
            issuer,
            serialNumber,
            partyAinfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SenderStaticInfo(value_: &SenderStaticInfo) -> ASN1Result<X690Element> {
    |value_: &SenderStaticInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_Name(&value_.issuer)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_UserKeyingMaterial(&value_.partyAinfo)?);
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
/// SenderDhPublicKey ::= SEQUENCE {
///   algorithm   AlgorithmIdentifier {{SupportedDHPublicKeyAlgorithms}},
///   publicKey   BIT STRING,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SenderDhPublicKey {
    pub algorithm: AlgorithmIdentifier,
    pub publicKey: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl SenderDhPublicKey {
    pub fn new(
        algorithm: AlgorithmIdentifier,
        publicKey: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SenderDhPublicKey {
            algorithm,
            publicKey,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SenderDhPublicKey {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SenderDhPublicKey(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SenderDhPublicKey {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SenderDhPublicKey(el)
    }
}

pub const _rctl1_components_for_SenderDhPublicKey: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "publicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SenderDhPublicKey: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SenderDhPublicKey: &[ComponentSpec; 0] = &[];

pub fn _decode_SenderDhPublicKey(el: &X690Element) -> ASN1Result<SenderDhPublicKey> {
    |el_: &X690Element| -> ASN1Result<SenderDhPublicKey> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SenderDhPublicKey,
            _eal_components_for_SenderDhPublicKey,
            _rctl2_components_for_SenderDhPublicKey,
        )?;
        let algorithm = _decode_AlgorithmIdentifier(_components.get("algorithm").unwrap())?;
        let publicKey = ber_decode_bit_string(_components.get("publicKey").unwrap())?;
        Ok(SenderDhPublicKey {
            algorithm,
            publicKey,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SenderDhPublicKey(value_: &SenderDhPublicKey) -> ASN1Result<X690Element> {
    |value_: &SenderDhPublicKey| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
        components_.push(ber_encode_bit_string(&value_.publicKey)?);
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
/// SupportedDHPublicKeyAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedDHPublicKeyAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserKeyingMaterial  ::=  OCTET STRING (SIZE (64))
/// ```
pub type UserKeyingMaterial = OCTET_STRING; // OctetStringType

pub fn _decode_UserKeyingMaterial(el: &X690Element) -> ASN1Result<UserKeyingMaterial> {
    ber_decode_octet_string(&el)
}

pub fn _encode_UserKeyingMaterial(value_: &UserKeyingMaterial) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedPduInfo ::= SEQUENCE {
///   pduType                 WRAPPED-PDU.&id ({SupportedPduSet}),
///   encryptedKey            EncryptedKey OPTIONAL,
///   pduEncryptionAlgorithm  SEQUENCE {
///     algorithm               ALGORITHM.&id ({SymmetricEncryptionAlgorithms}),
///     parameter               ALGORITHM.&Type
///                   ({SymmetricEncryptionAlgorithms}{@.algorithm})} OPTIONAL,
///   encryptedPdu        [0] EncryptedPdu,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncryptedPduInfo {
    pub pduType: OBJECT_IDENTIFIER,
    pub encryptedKey: OPTIONAL<EncryptedKey>,
    pub pduEncryptionAlgorithm: OPTIONAL<EncryptedPduInfo_pduEncryptionAlgorithm>,
    pub encryptedPdu: EncryptedPdu,
    pub _unrecognized: Vec<X690Element>,
}
impl EncryptedPduInfo {
    pub fn new(
        pduType: OBJECT_IDENTIFIER,
        encryptedKey: OPTIONAL<EncryptedKey>,
        pduEncryptionAlgorithm: OPTIONAL<EncryptedPduInfo_pduEncryptionAlgorithm>,
        encryptedPdu: EncryptedPdu,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EncryptedPduInfo {
            pduType,
            encryptedKey,
            pduEncryptionAlgorithm,
            encryptedPdu,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for EncryptedPduInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedPduInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncryptedPduInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedPduInfo(el)
    }
}

pub const _rctl1_components_for_EncryptedPduInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "pduType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedKey",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pduEncryptionAlgorithm",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedPdu",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncryptedPduInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedPduInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedPduInfo(el: &X690Element) -> ASN1Result<EncryptedPduInfo> {
    |el_: &X690Element| -> ASN1Result<EncryptedPduInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncryptedPduInfo,
            _eal_components_for_EncryptedPduInfo,
            _rctl2_components_for_EncryptedPduInfo,
        )?;
        let pduType = ber_decode_object_identifier(_components.get("pduType").unwrap())?;
        let encryptedKey: OPTIONAL<EncryptedKey> = match _components.get("encryptedKey") {
            Some(c_) => Some(_decode_EncryptedKey(c_)?),
            _ => None,
        };
        let pduEncryptionAlgorithm: OPTIONAL<EncryptedPduInfo_pduEncryptionAlgorithm> =
            match _components.get("pduEncryptionAlgorithm") {
                Some(c_) => Some(_decode_EncryptedPduInfo_pduEncryptionAlgorithm(c_)?),
                _ => None,
            };
        let encryptedPdu = |el: &X690Element| -> ASN1Result<EncryptedPdu> {
            Ok(_decode_EncryptedPdu(&el.inner()?)?)
        }(_components.get("encryptedPdu").unwrap())?;
        Ok(EncryptedPduInfo {
            pduType,
            encryptedKey,
            pduEncryptionAlgorithm,
            encryptedPdu,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EncryptedPduInfo(value_: &EncryptedPduInfo) -> ASN1Result<X690Element> {
    |value_: &EncryptedPduInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(ber_encode_object_identifier(&value_.pduType)?);
        if let Some(v_) = &value_.encryptedKey {
            components_.push(_encode_EncryptedKey(&v_)?);
        }
        if let Some(v_) = &value_.pduEncryptionAlgorithm {
            components_.push(_encode_EncryptedPduInfo_pduEncryptionAlgorithm(&v_)?);
        }
        components_.push(|v_1: &EncryptedPdu| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_EncryptedPdu(
                    &v_1,
                )?))),
            ))
        }(&value_.encryptedPdu)?);
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
/// EncryptedKey  ::=  OCTET STRING
/// ```
pub type EncryptedKey = OCTET_STRING; // OctetStringType

pub fn _decode_EncryptedKey(el: &X690Element) -> ASN1Result<EncryptedKey> {
    ber_decode_octet_string(&el)
}

pub fn _encode_EncryptedKey(value_: &EncryptedKey) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SymmetricEncryptionAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SymmetricEncryptionAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedPdu  ::=  OCTET STRING
/// ```
pub type EncryptedPdu = OCTET_STRING; // OctetStringType

pub fn _decode_EncryptedPdu(el: &X690Element) -> ASN1Result<EncryptedPdu> {
    ber_decode_octet_string(&el)
}

pub fn _encode_EncryptedPdu(value_: &EncryptedPdu) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAttributes ATTRIBUTE ::= {...}
/// ```
///
///
pub fn SupportedAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificateV2  ::=  AttributeCertificate
/// ```
pub type AttributeCertificateV2 = AttributeCertificate; // DefinedType

pub fn _decode_AttributeCertificateV2(el: &X690Element) -> ASN1Result<AttributeCertificateV2> {
    _decode_AttributeCertificate(&el)
}

pub fn _encode_AttributeCertificateV2(value_: &AttributeCertificateV2) -> ASN1Result<X690Element> {
    _encode_AttributeCertificate(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contentType ATTRIBUTE ::= {
///   WITH SYNTAX            WRAPPED-PDU.&id({SupportedPduSet})
///   EQUALITY MATCHING RULE objectIdentifierMatch
///   SINGLE VALUE           TRUE
///   ID                     id-contentType }
/// ```
///
///
pub fn contentType() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        id: id_contentType(),                                    /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
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
/// id-contentType OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 3 }
/// ```
///
///
pub fn id_contentType() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 3,
    ]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// messageDigest ATTRIBUTE ::= {
///   WITH SYNTAX            OCTET STRING
///   EQUALITY MATCHING RULE octetStringMatch
///   SINGLE VALUE           TRUE
///   ID                     id-messageDigest }
/// ```
///
///
pub fn messageDigest() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(octetStringMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                          /* OBJECT_FIELD_SETTING */
        id: id_messageDigest(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
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
/// id-messageDigest OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 4 }
/// ```
///
///
pub fn id_messageDigest() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 4,
    ]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PkiWaError  ::=  ENUMERATED {
///   unsupportedWrapperVersion           (0),
///   unsupportedSignatureAlgorithm       (1),
///   incompleteCertPath                  (2),
///   certificationPathFailure            (3),
///   invalidSignature                    (4),
///   missingMandatoryAttributes          (5),
///   unwantedAttribute                   (6),
///   unsupportedPduType                  (7),
///   unexpectedPduType                   (8),
///   invalidPduSyntax                    (9),
///   unknownDHpkCetificate               (10),
///   invalidKeyingMaterial               (11),
///   dhAlgorithmMismatch                 (12),
///   invalideDhPublickey                 (13),
///   unsupportedKeyWrappingAlgorithm     (14),
///   keyEncAlgorithmParametersMissing    (15),
///   keyEncAlgorithmParametersNotAllowed (16),
///   invalidParmsForSymEncryptAlgorithms (17),
///   decryptionFailed                    (18),
///   ... }
/// ```
pub type PkiWaError = ENUMERATED;

pub const PkiWaError_unsupportedWrapperVersion: PkiWaError = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_unsupportedSignatureAlgorithm: PkiWaError = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_incompleteCertPath: PkiWaError = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_certificationPathFailure: PkiWaError = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_invalidSignature: PkiWaError = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_missingMandatoryAttributes: PkiWaError = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_unwantedAttribute: PkiWaError = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_unsupportedPduType: PkiWaError = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_unexpectedPduType: PkiWaError = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_invalidPduSyntax: PkiWaError = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_unknownDHpkCetificate: PkiWaError = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_invalidKeyingMaterial: PkiWaError = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_dhAlgorithmMismatch: PkiWaError = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_invalideDhPublickey: PkiWaError = 13; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_unsupportedKeyWrappingAlgorithm: PkiWaError = 14; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_keyEncAlgorithmParametersMissing: PkiWaError = 15; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_keyEncAlgorithmParametersNotAllowed: PkiWaError = 16; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_invalidParmsForSymEncryptAlgorithms: PkiWaError = 17; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PkiWaError_decryptionFailed: PkiWaError = 18; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_PkiWaError(el: &X690Element) -> ASN1Result<PkiWaError> {
    ber_decode_enumerated(&el)
}

pub fn _encode_PkiWaError(value_: &PkiWaError) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSPDU-wrapper-conf ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TBSPDU_wrapper_conf {
    clear(WrappedPDUInfo),
    protected(EncryptedInfo),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TBSPDU_wrapper_conf {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSPDU_wrapper_conf(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSPDU_wrapper_conf {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TBSPDU_wrapper_conf(el)
    }
}

pub fn _decode_TBSPDU_wrapper_conf(el: &X690Element) -> ASN1Result<TBSPDU_wrapper_conf> {
    |el: &X690Element| -> ASN1Result<TBSPDU_wrapper_conf> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 2) => Ok(TBSPDU_wrapper_conf::clear(_decode_WrappedPDUInfo(
                &el.inner()?,
            )?)),
            (TagClass::CONTEXT, 3) => Ok(TBSPDU_wrapper_conf::protected(_decode_EncryptedInfo(
                &el.inner()?,
            )?)),
            _ => Ok(TBSPDU_wrapper_conf::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TBSPDU_wrapper_conf(value_: &TBSPDU_wrapper_conf) -> ASN1Result<X690Element> {
    |value: &TBSPDU_wrapper_conf| -> ASN1Result<X690Element> {
        match value {
            TBSPDU_wrapper_conf::clear(v) => |v_1: &WrappedPDUInfo| -> ASN1Result<X690Element> {
                let el_1 = _encode_WrappedPDUInfo(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            TBSPDU_wrapper_conf::protected(v) => |v_1: &EncryptedInfo| -> ASN1Result<X690Element> {
                let el_1 = _encode_EncryptedInfo(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            TBSPDU_wrapper_conf::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreement-keyEncryptionAlgorithm ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct KeyAgreement_keyEncryptionAlgorithm {
    pub algorithm: OBJECT_IDENTIFIER,
    pub parameters: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl KeyAgreement_keyEncryptionAlgorithm {
    pub fn new(
        algorithm: OBJECT_IDENTIFIER,
        parameters: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        KeyAgreement_keyEncryptionAlgorithm {
            algorithm,
            parameters,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for KeyAgreement_keyEncryptionAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreement_keyEncryptionAlgorithm(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KeyAgreement_keyEncryptionAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreement_keyEncryptionAlgorithm(el)
    }
}

pub const _rctl1_components_for_KeyAgreement_keyEncryptionAlgorithm: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("parameters", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_KeyAgreement_keyEncryptionAlgorithm: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_KeyAgreement_keyEncryptionAlgorithm: &[ComponentSpec; 0] = &[];

pub fn _decode_KeyAgreement_keyEncryptionAlgorithm(
    el: &X690Element,
) -> ASN1Result<KeyAgreement_keyEncryptionAlgorithm> {
    |el_: &X690Element| -> ASN1Result<KeyAgreement_keyEncryptionAlgorithm> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_KeyAgreement_keyEncryptionAlgorithm,
            _eal_components_for_KeyAgreement_keyEncryptionAlgorithm,
            _rctl2_components_for_KeyAgreement_keyEncryptionAlgorithm,
        )?;
        let algorithm = ber_decode_object_identifier(_components.get("algorithm").unwrap())?;
        let parameters = x690_identity(_components.get("parameters").unwrap())?;
        Ok(KeyAgreement_keyEncryptionAlgorithm {
            algorithm,
            parameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_KeyAgreement_keyEncryptionAlgorithm(
    value_: &KeyAgreement_keyEncryptionAlgorithm,
) -> ASN1Result<X690Element> {
    |value_: &KeyAgreement_keyEncryptionAlgorithm| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.algorithm)?);
        components_.push(x690_identity(&value_.parameters)?);
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
/// EncryptedPduInfo-pduEncryptionAlgorithm ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncryptedPduInfo_pduEncryptionAlgorithm {
    pub algorithm: OBJECT_IDENTIFIER,
    pub parameter: X690Element,
}
impl EncryptedPduInfo_pduEncryptionAlgorithm {
    pub fn new(algorithm: OBJECT_IDENTIFIER, parameter: X690Element) -> Self {
        EncryptedPduInfo_pduEncryptionAlgorithm {
            algorithm,
            parameter,
        }
    }
}
impl TryFrom<X690Element> for EncryptedPduInfo_pduEncryptionAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedPduInfo_pduEncryptionAlgorithm(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncryptedPduInfo_pduEncryptionAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedPduInfo_pduEncryptionAlgorithm(el)
    }
}

pub const _rctl1_components_for_EncryptedPduInfo_pduEncryptionAlgorithm: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("parameter", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_EncryptedPduInfo_pduEncryptionAlgorithm: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedPduInfo_pduEncryptionAlgorithm: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedPduInfo_pduEncryptionAlgorithm(
    el: &X690Element,
) -> ASN1Result<EncryptedPduInfo_pduEncryptionAlgorithm> {
    |el_: &X690Element| -> ASN1Result<EncryptedPduInfo_pduEncryptionAlgorithm> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
            _eal_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
            _rctl2_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
        )?;
        let algorithm = ber_decode_object_identifier(_components.get("algorithm").unwrap())?;
        let parameter = x690_identity(_components.get("parameter").unwrap())?;
        Ok(EncryptedPduInfo_pduEncryptionAlgorithm {
            algorithm,
            parameter,
        })
    }(&el)
}

pub fn _encode_EncryptedPduInfo_pduEncryptionAlgorithm(
    value_: &EncryptedPduInfo_pduEncryptionAlgorithm,
) -> ASN1Result<X690Element> {
    |value_: &EncryptedPduInfo_pduEncryptionAlgorithm| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_object_identifier(&value_.algorithm)?);
        components_.push(x690_identity(&value_.parameter)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}
