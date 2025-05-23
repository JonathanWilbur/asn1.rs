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
    _decode_SIGNED::<TBSPDU_wrapper>(_decode_TBSPDU_wrapper, el)
}

pub fn _encode_PDU_wrapper(value_: &PDU_wrapper) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSPDU_wrapper>(_encode_TBSPDU_wrapper, value_)
}

pub fn _validate_PDU_wrapper(el: &X690Element) -> ASN1Result<()> {
    _validate_SIGNED::<TBSPDU_wrapper>(_validate_TBSPDU_wrapper, el)
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
impl TryFrom<&X690Element> for TBSPDU_wrapper {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBSPDU-wrapper"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSPDU_wrapper,
        _eal_components_for_TBSPDU_wrapper,
        _rctl2_components_for_TBSPDU_wrapper,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut signatureAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut certPath_: OPTIONAL<PkiPath> = None;
    let mut signedAttrs_: OPTIONAL<SignedAttributes> = None;
    let mut conf_: OPTIONAL<TBSPDU_wrapper_conf> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "signatureAlgorithm" => signatureAlgorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "certPath" => certPath_ = Some(_decode_PkiPath(_el)?),
            "signedAttrs" => signedAttrs_ = Some(_decode_SignedAttributes(_el)?),
            "conf" => conf_ = Some(_decode_TBSPDU_wrapper_conf(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBSPDU_wrapper {
        version: version_,
        signatureAlgorithm: signatureAlgorithm_.unwrap(),
        certPath: certPath_.unwrap(),
        signedAttrs: signedAttrs_,
        conf: conf_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_TBSPDU_wrapper(value_: &TBSPDU_wrapper) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    if let Some(v_) = &value_.version {
        if *v_ != TBSPDU_wrapper::_default_value_for_version() {
            components_.push(_encode_Version(&v_)?);
        }
    }
    components_.push(_encode_AlgorithmIdentifier(&value_.signatureAlgorithm)?);
    components_.push(|v_1: &PkiPath| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_PkiPath(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.certPath)?);
    if let Some(v_) = &value_.signedAttrs {
        components_.push(|v_1: &SignedAttributes| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SignedAttributes(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_TBSPDU_wrapper_conf(&value_.conf)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TBSPDU_wrapper(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBSPDU-wrapper"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSPDU_wrapper,
        _eal_components_for_TBSPDU_wrapper,
        _rctl2_components_for_TBSPDU_wrapper,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_Version(_el)?,
            "signatureAlgorithm" => _validate_AlgorithmIdentifier(_el)?,
            "certPath" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certPath")
                    );
                }
                Ok(_validate_PkiPath(&el)?)
            }(_el)?,
            "signedAttrs" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "signedAttrs")
                    );
                }
                Ok(_validate_SignedAttributes(&el)?)
            }(_el)?,
            "conf" => _validate_TBSPDU_wrapper_conf(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedSignatureAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedSignatureAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedAttributes  ::=  SET SIZE (1..MAX) OF Attribute{{SupportedSignedAttributes}}
/// ```
pub type SignedAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_SignedAttributes(el: &X690Element) -> ASN1Result<SignedAttributes> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedAttributes")
            )
        }
    };
    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Attribute(el)?);
    }
    Ok(items)
}

pub fn _encode_SignedAttributes(value_: &SignedAttributes) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Attribute(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SignedAttributes(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Attribute(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedAttributes")),
    }
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
impl TryFrom<&X690Element> for WrappedPDUInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "WrappedPDUInfo"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_WrappedPDUInfo,
        _eal_components_for_WrappedPDUInfo,
        _rctl2_components_for_WrappedPDUInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut pduType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut pduInfo_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pduType" => pduType_ = Some(BER.decode_object_identifier(_el)?),
            "pduInfo" => pduInfo_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(WrappedPDUInfo {
        pduType: pduType_.unwrap(),
        pduInfo: pduInfo_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_WrappedPDUInfo(value_: &WrappedPDUInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.pduType)?);
    components_.push(x690_identity(&value_.pduInfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_WrappedPDUInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "WrappedPDUInfo"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_WrappedPDUInfo,
        _eal_components_for_WrappedPDUInfo,
        _rctl2_components_for_WrappedPDUInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pduType" => BER.validate_object_identifier(_el)?,
            "pduInfo" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedPduSet WRAPPED-PDU ::= {...}
/// ```
///
///
pub fn SupportedPduSet() -> Vec<WRAPPED_PDU> {
    Vec::new()
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
impl TryFrom<&X690Element> for EncryptedInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EncryptedInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedInfo,
        _eal_components_for_EncryptedInfo,
        _rctl2_components_for_EncryptedInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut keyAgreement_: OPTIONAL<KeyAgreement> = None;
    let mut encryptedPduInfo_: OPTIONAL<EncryptedPduInfo> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyAgreement" => keyAgreement_ = Some(_decode_KeyAgreement(_el)?),
            "encryptedPduInfo" => encryptedPduInfo_ = Some(_decode_EncryptedPduInfo(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EncryptedInfo {
        keyAgreement: keyAgreement_.unwrap(),
        encryptedPduInfo: encryptedPduInfo_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_EncryptedInfo(value_: &EncryptedInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_KeyAgreement(&value_.keyAgreement)?);
    components_.push(_encode_EncryptedPduInfo(&value_.encryptedPduInfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EncryptedInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EncryptedInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedInfo,
        _eal_components_for_EncryptedInfo,
        _rctl2_components_for_EncryptedInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyAgreement" => _validate_KeyAgreement(_el)?,
            "encryptedPduInfo" => _validate_EncryptedPduInfo(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for KeyAgreement {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KeyAgreement")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyAgreement,
        _eal_components_for_KeyAgreement,
        _rctl2_components_for_KeyAgreement,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut senderDhInfo_: OPTIONAL<SenderDhInfo> = None;
    let mut keyEncryptionAlgorithm_: OPTIONAL<KeyAgreement_keyEncryptionAlgorithm> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "senderDhInfo" => {
                senderDhInfo_ = Some(|el: &X690Element| -> ASN1Result<SenderDhInfo> {
                    Ok(_decode_SenderDhInfo(&el.inner()?)?)
                }(_el)?)
            }
            "keyEncryptionAlgorithm" => {
                keyEncryptionAlgorithm_ = Some(_decode_KeyAgreement_keyEncryptionAlgorithm(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(KeyAgreement {
        senderDhInfo: senderDhInfo_.unwrap(),
        keyEncryptionAlgorithm: keyEncryptionAlgorithm_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_KeyAgreement(value_: &KeyAgreement) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(|v_1: &SenderDhInfo| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_SenderDhInfo(&v_1)?),
        ))
    }(&value_.senderDhInfo)?);
    components_.push(_encode_KeyAgreement_keyEncryptionAlgorithm(
        &value_.keyEncryptionAlgorithm,
    )?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_KeyAgreement(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KeyAgreement")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyAgreement,
        _eal_components_for_KeyAgreement,
        _rctl2_components_for_KeyAgreement,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "senderDhInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "senderDhInfo")
                    );
                }
                Ok(_validate_SenderDhInfo(&el.inner()?)?)
            }(_el)?,
            "keyEncryptionAlgorithm" => _validate_KeyAgreement_keyEncryptionAlgorithm(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedKeyEncryptionAlgorithm ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedKeyEncryptionAlgorithm() -> Vec<ALGORITHM> {
    Vec::new()
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

impl TryFrom<&X690Element> for SenderDhInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SenderDhInfo(el)
    }
}

pub fn _decode_SenderDhInfo(el: &X690Element) -> ASN1Result<SenderDhInfo> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(SenderDhInfo::senderStaticInfo(
            |el: &X690Element| -> ASN1Result<SenderStaticInfo> {
                Ok(_decode_SenderStaticInfo(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(SenderDhInfo::senderDhPublicKey(
            |el: &X690Element| -> ASN1Result<SenderDhPublicKey> {
                Ok(_decode_SenderDhPublicKey(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(SenderDhInfo::_unrecognized(el.clone())),
    }
}

pub fn _encode_SenderDhInfo(value_: &SenderDhInfo) -> ASN1Result<X690Element> {
    match value_ {
        SenderDhInfo::senderStaticInfo(v) => |v_1: &SenderStaticInfo| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_SenderStaticInfo(&v_1)?),
            ))
        }(&v),
        SenderDhInfo::senderDhPublicKey(v) => {
            |v_1: &SenderDhPublicKey| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_SenderDhPublicKey(&v_1)?),
                ))
            }(&v)
        }
        SenderDhInfo::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_SenderDhInfo(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "senderStaticInfo")
                );
            }
            Ok(_validate_SenderStaticInfo(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "senderDhPublicKey")
                );
            }
            Ok(_validate_SenderDhPublicKey(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for SenderStaticInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SenderStaticInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SenderStaticInfo,
        _eal_components_for_SenderStaticInfo,
        _rctl2_components_for_SenderStaticInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut partyAinfo_: OPTIONAL<UserKeyingMaterial> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "partyAinfo" => partyAinfo_ = Some(_decode_UserKeyingMaterial(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SenderStaticInfo {
        issuer: issuer_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        partyAinfo: partyAinfo_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_SenderStaticInfo(value_: &SenderStaticInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    components_.push(_encode_UserKeyingMaterial(&value_.partyAinfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SenderStaticInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SenderStaticInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SenderStaticInfo,
        _eal_components_for_SenderStaticInfo,
        _rctl2_components_for_SenderStaticInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "partyAinfo" => _validate_UserKeyingMaterial(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for SenderDhPublicKey {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SenderDhPublicKey")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SenderDhPublicKey,
        _eal_components_for_SenderDhPublicKey,
        _rctl2_components_for_SenderDhPublicKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut publicKey_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "publicKey" => publicKey_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SenderDhPublicKey {
        algorithm: algorithm_.unwrap(),
        publicKey: publicKey_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_SenderDhPublicKey(value_: &SenderDhPublicKey) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
    components_.push(BER.encode_bit_string(&value_.publicKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SenderDhPublicKey(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SenderDhPublicKey")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SenderDhPublicKey,
        _eal_components_for_SenderDhPublicKey,
        _rctl2_components_for_SenderDhPublicKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => _validate_AlgorithmIdentifier(_el)?,
            "publicKey" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedDHPublicKeyAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedDHPublicKeyAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserKeyingMaterial  ::=  OCTET STRING (SIZE (64))
/// ```
pub type UserKeyingMaterial = OCTET_STRING; // OctetStringType

pub fn _decode_UserKeyingMaterial(el: &X690Element) -> ASN1Result<UserKeyingMaterial> {
    BER.decode_octet_string(&el)
}

pub fn _encode_UserKeyingMaterial(value_: &UserKeyingMaterial) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_UserKeyingMaterial(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
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
impl TryFrom<&X690Element> for EncryptedPduInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EncryptedPduInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedPduInfo,
        _eal_components_for_EncryptedPduInfo,
        _rctl2_components_for_EncryptedPduInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut pduType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut encryptedKey_: OPTIONAL<EncryptedKey> = None;
    let mut pduEncryptionAlgorithm_: OPTIONAL<EncryptedPduInfo_pduEncryptionAlgorithm> = None;
    let mut encryptedPdu_: OPTIONAL<EncryptedPdu> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pduType" => pduType_ = Some(BER.decode_object_identifier(_el)?),
            "encryptedKey" => encryptedKey_ = Some(_decode_EncryptedKey(_el)?),
            "pduEncryptionAlgorithm" => {
                pduEncryptionAlgorithm_ =
                    Some(_decode_EncryptedPduInfo_pduEncryptionAlgorithm(_el)?)
            }
            "encryptedPdu" => {
                encryptedPdu_ = Some(|el: &X690Element| -> ASN1Result<EncryptedPdu> {
                    Ok(_decode_EncryptedPdu(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EncryptedPduInfo {
        pduType: pduType_.unwrap(),
        encryptedKey: encryptedKey_,
        pduEncryptionAlgorithm: pduEncryptionAlgorithm_,
        encryptedPdu: encryptedPdu_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_EncryptedPduInfo(value_: &EncryptedPduInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(BER.encode_object_identifier(&value_.pduType)?);
    if let Some(v_) = &value_.encryptedKey {
        components_.push(_encode_EncryptedKey(&v_)?);
    }
    if let Some(v_) = &value_.pduEncryptionAlgorithm {
        components_.push(_encode_EncryptedPduInfo_pduEncryptionAlgorithm(&v_)?);
    }
    components_.push(|v_1: &EncryptedPdu| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_EncryptedPdu(&v_1)?),
        ))
    }(&value_.encryptedPdu)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EncryptedPduInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EncryptedPduInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedPduInfo,
        _eal_components_for_EncryptedPduInfo,
        _rctl2_components_for_EncryptedPduInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pduType" => BER.validate_object_identifier(_el)?,
            "encryptedKey" => _validate_EncryptedKey(_el)?,
            "pduEncryptionAlgorithm" => _validate_EncryptedPduInfo_pduEncryptionAlgorithm(_el)?,
            "encryptedPdu" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "encryptedPdu")
                    );
                }
                Ok(_validate_EncryptedPdu(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedKey  ::=  OCTET STRING
/// ```
pub type EncryptedKey = OCTET_STRING; // OctetStringType

pub fn _decode_EncryptedKey(el: &X690Element) -> ASN1Result<EncryptedKey> {
    BER.decode_octet_string(&el)
}

pub fn _encode_EncryptedKey(value_: &EncryptedKey) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_EncryptedKey(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SymmetricEncryptionAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SymmetricEncryptionAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedPdu  ::=  OCTET STRING
/// ```
pub type EncryptedPdu = OCTET_STRING; // OctetStringType

pub fn _decode_EncryptedPdu(el: &X690Element) -> ASN1Result<EncryptedPdu> {
    BER.decode_octet_string(&el)
}

pub fn _encode_EncryptedPdu(value_: &EncryptedPdu) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_EncryptedPdu(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAttributes ATTRIBUTE ::= {...}
/// ```
///
///
pub fn SupportedAttributes() -> Vec<ATTRIBUTE> {
    Vec::new()
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

pub fn _validate_AttributeCertificateV2(el: &X690Element) -> ASN1Result<()> {
    _validate_AttributeCertificate(&el)
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

pub mod contentType {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = OBJECT_IDENTIFIER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_object_identifier(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_object_identifier(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_object_identifier(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contentType OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 3 }
/// ```
///
///
#[inline]
pub fn id_contentType () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549,/* pkcs */ 1,/* pkcs9 */ 9,3) // OID_GETTER
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

pub mod messageDigest {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-messageDigest OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 4 }
/// ```
///
///
#[inline]
pub fn id_messageDigest () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549,/* pkcs */ 1,/* pkcs9 */ 9,4) // OID_GETTER
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
    BER.decode_enumerated(&el)
}

pub fn _encode_PkiWaError(value_: &PkiWaError) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_PkiWaError(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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

impl TryFrom<&X690Element> for TBSPDU_wrapper_conf {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBSPDU_wrapper_conf(el)
    }
}

pub fn _decode_TBSPDU_wrapper_conf(el: &X690Element) -> ASN1Result<TBSPDU_wrapper_conf> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 2) => Ok(TBSPDU_wrapper_conf::clear(
            |el: &X690Element| -> ASN1Result<WrappedPDUInfo> {
                Ok(_decode_WrappedPDUInfo(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(TBSPDU_wrapper_conf::protected(
            |el: &X690Element| -> ASN1Result<EncryptedInfo> {
                Ok(_decode_EncryptedInfo(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(TBSPDU_wrapper_conf::_unrecognized(el.clone())),
    }
}

pub fn _encode_TBSPDU_wrapper_conf(value_: &TBSPDU_wrapper_conf) -> ASN1Result<X690Element> {
    match value_ {
        TBSPDU_wrapper_conf::clear(v) => |v_1: &WrappedPDUInfo| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_WrappedPDUInfo(&v_1)?),
            ))
        }(&v),
        TBSPDU_wrapper_conf::protected(v) => |v_1: &EncryptedInfo| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_EncryptedInfo(&v_1)?),
            ))
        }(&v),
        TBSPDU_wrapper_conf::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TBSPDU_wrapper_conf(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "clear"));
            }
            Ok(_validate_WrappedPDUInfo(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "protected"));
            }
            Ok(_validate_EncryptedInfo(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreement-keyEncryptionAlgorithm ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for KeyAgreement_keyEncryptionAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "KeyAgreement-keyEncryptionAlgorithm",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyAgreement_keyEncryptionAlgorithm,
        _eal_components_for_KeyAgreement_keyEncryptionAlgorithm,
        _rctl2_components_for_KeyAgreement_keyEncryptionAlgorithm,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parameters_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(BER.decode_object_identifier(_el)?),
            "parameters" => parameters_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(KeyAgreement_keyEncryptionAlgorithm {
        algorithm: algorithm_.unwrap(),
        parameters: parameters_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_KeyAgreement_keyEncryptionAlgorithm(
    value_: &KeyAgreement_keyEncryptionAlgorithm,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.algorithm)?);
    components_.push(x690_identity(&value_.parameters)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_KeyAgreement_keyEncryptionAlgorithm(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "KeyAgreement-keyEncryptionAlgorithm",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyAgreement_keyEncryptionAlgorithm,
        _eal_components_for_KeyAgreement_keyEncryptionAlgorithm,
        _rctl2_components_for_KeyAgreement_keyEncryptionAlgorithm,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => BER.validate_object_identifier(_el)?,
            "parameters" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedPduInfo-pduEncryptionAlgorithm ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for EncryptedPduInfo_pduEncryptionAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncryptedPduInfo-pduEncryptionAlgorithm",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
        _eal_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
        _rctl2_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parameter_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(BER.decode_object_identifier(_el)?),
            "parameter" => parameter_ = Some(x690_identity(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncryptedPduInfo-pduEncryptionAlgorithm",
                ))
            }
        }
    }
    Ok(EncryptedPduInfo_pduEncryptionAlgorithm {
        algorithm: algorithm_.unwrap(),
        parameter: parameter_.unwrap(),
    })
}

pub fn _encode_EncryptedPduInfo_pduEncryptionAlgorithm(
    value_: &EncryptedPduInfo_pduEncryptionAlgorithm,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.algorithm)?);
    components_.push(x690_identity(&value_.parameter)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EncryptedPduInfo_pduEncryptionAlgorithm(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncryptedPduInfo-pduEncryptionAlgorithm",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
        _eal_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
        _rctl2_components_for_EncryptedPduInfo_pduEncryptionAlgorithm,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => BER.validate_object_identifier(_el)?,
            "parameter" => BER.validate_any(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncryptedPduInfo-pduEncryptionAlgorithm",
                ))
            }
        }
    }
    Ok(())
}
