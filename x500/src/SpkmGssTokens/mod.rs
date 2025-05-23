#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # SpkmGssTokens
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `SpkmGssTokens`.
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
use crate::InformationFramework::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-REQ ::= SEQUENCE {
///   requestToken  REQ-TOKEN,
///   certif-data   [0]  CertificationData OPTIONAL,
///   auth-data     [1]  AuthorizationData OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_REQ {
    pub requestToken: REQ_TOKEN,
    pub certif_data: OPTIONAL<CertificationData>,
    pub auth_data: OPTIONAL<AuthorizationData>,
}
impl SPKM_REQ {
    pub fn new(
        requestToken: REQ_TOKEN,
        certif_data: OPTIONAL<CertificationData>,
        auth_data: OPTIONAL<AuthorizationData>,
    ) -> Self {
        SPKM_REQ {
            requestToken,
            certif_data,
            auth_data,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_REQ {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_REQ(el)
    }
}

pub const _rctl1_components_for_SPKM_REQ: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "requestToken",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certif-data",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "auth-data",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_REQ: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_REQ: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_REQ(el: &X690Element) -> ASN1Result<SPKM_REQ> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REQ")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_REQ,
        _eal_components_for_SPKM_REQ,
        _rctl2_components_for_SPKM_REQ,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut requestToken_: OPTIONAL<REQ_TOKEN> = None;
    let mut certif_data_: OPTIONAL<CertificationData> = None;
    let mut auth_data_: OPTIONAL<AuthorizationData> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "requestToken" => requestToken_ = Some(_decode_REQ_TOKEN(_el)?),
            "certif-data" => certif_data_ = Some(_decode_CertificationData(_el)?),
            "auth-data" => auth_data_ = Some(_decode_AuthorizationData(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REQ")),
        }
    }
    Ok(SPKM_REQ {
        requestToken: requestToken_.unwrap(),
        certif_data: certif_data_,
        auth_data: auth_data_,
    })
}

pub fn _encode_SPKM_REQ(value_: &SPKM_REQ) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_REQ_TOKEN(&value_.requestToken)?);
    if let Some(v_) = &value_.certif_data {
        components_.push(|v_1: &CertificationData| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificationData(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.auth_data {
        components_.push(|v_1: &AuthorizationData| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AuthorizationData(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_REQ(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REQ")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_REQ,
        _eal_components_for_SPKM_REQ,
        _rctl2_components_for_SPKM_REQ,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "requestToken" => _validate_REQ_TOKEN(_el)?,
            "certif-data" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certif-data")
                    );
                }
                Ok(_validate_CertificationData(&el)?)
            }(_el)?,
            "auth-data" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "auth-data")
                    );
                }
                Ok(_validate_AuthorizationData(&el)?)
            }(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REQ")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificationData ::= SEQUENCE {
///   certificationPath          [0]  CertificationPath OPTIONAL,
///   certificateRevocationList  [1]  CertificateList OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertificationData {
    pub certificationPath: OPTIONAL<CertificationPath>,
    pub certificateRevocationList: OPTIONAL<CertificateList>,
}
impl CertificationData {
    pub fn new(
        certificationPath: OPTIONAL<CertificationPath>,
        certificateRevocationList: OPTIONAL<CertificateList>,
    ) -> Self {
        CertificationData {
            certificationPath,
            certificateRevocationList,
        }
    }
}
impl Default for CertificationData {
    fn default() -> Self {
        CertificationData {
            certificationPath: None,
            certificateRevocationList: None,
        }
    }
}
impl TryFrom<&X690Element> for CertificationData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificationData(el)
    }
}

pub const _rctl1_components_for_CertificationData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "certificationPath",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certificateRevocationList",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificationData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificationData: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificationData(el: &X690Element) -> ASN1Result<CertificationData> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationData")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificationData,
        _eal_components_for_CertificationData,
        _rctl2_components_for_CertificationData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut certificationPath_: OPTIONAL<CertificationPath> = None;
    let mut certificateRevocationList_: OPTIONAL<CertificateList> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificationPath" => certificationPath_ = Some(_decode_CertificationPath(_el)?),
            "certificateRevocationList" => {
                certificateRevocationList_ = Some(_decode_CertificateList(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationData")
                )
            }
        }
    }
    Ok(CertificationData {
        certificationPath: certificationPath_,
        certificateRevocationList: certificateRevocationList_,
    })
}

pub fn _encode_CertificationData(value_: &CertificationData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.certificationPath {
        components_.push(|v_1: &CertificationPath| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificationPath(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.certificateRevocationList {
        components_.push(|v_1: &CertificateList| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateList(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_CertificationData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationData")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificationData,
        _eal_components_for_CertificationData,
        _rctl2_components_for_CertificationData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificationPath" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "certificationPath",
                    ));
                }
                Ok(_validate_CertificationPath(&el)?)
            }(_el)?,
            "certificateRevocationList" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "certificateRevocationList",
                    ));
                }
                Ok(_validate_CertificateList(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationData")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificationPath ::= SEQUENCE {
///   userKeyId          [0]  OCTET STRING OPTIONAL,
///   userCertif         [1]  Certificate OPTIONAL,
///   verifKeyId         [2]  OCTET STRING OPTIONAL,
///   userVerifCertif    [3]  Certificate OPTIONAL,
///   theCACertificates  [4]  SEQUENCE OF CertificatePair OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertificationPath {
    pub userKeyId: OPTIONAL<OCTET_STRING>,
    pub userCertif: OPTIONAL<Certificate>,
    pub verifKeyId: OPTIONAL<OCTET_STRING>,
    pub userVerifCertif: OPTIONAL<Certificate>,
    pub theCACertificates: OPTIONAL<Vec<CertificatePair>>,
}
impl CertificationPath {
    pub fn new(
        userKeyId: OPTIONAL<OCTET_STRING>,
        userCertif: OPTIONAL<Certificate>,
        verifKeyId: OPTIONAL<OCTET_STRING>,
        userVerifCertif: OPTIONAL<Certificate>,
        theCACertificates: OPTIONAL<Vec<CertificatePair>>,
    ) -> Self {
        CertificationPath {
            userKeyId,
            userCertif,
            verifKeyId,
            userVerifCertif,
            theCACertificates,
        }
    }
}
impl Default for CertificationPath {
    fn default() -> Self {
        CertificationPath {
            userKeyId: None,
            userCertif: None,
            verifKeyId: None,
            userVerifCertif: None,
            theCACertificates: None,
        }
    }
}
impl TryFrom<&X690Element> for CertificationPath {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificationPath(el)
    }
}

pub const _rctl1_components_for_CertificationPath: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "userKeyId",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userCertif",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "verifKeyId",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userVerifCertif",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "theCACertificates",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificationPath: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificationPath: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificationPath(el: &X690Element) -> ASN1Result<CertificationPath> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationPath")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificationPath,
        _eal_components_for_CertificationPath,
        _rctl2_components_for_CertificationPath,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut userKeyId_: OPTIONAL<OCTET_STRING> = None;
    let mut userCertif_: OPTIONAL<Certificate> = None;
    let mut verifKeyId_: OPTIONAL<OCTET_STRING> = None;
    let mut userVerifCertif_: OPTIONAL<Certificate> = None;
    let mut theCACertificates_: OPTIONAL<Vec<CertificatePair>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userKeyId" => userKeyId_ = Some(BER.decode_octet_string(_el)?),
            "userCertif" => userCertif_ = Some(_decode_Certificate(_el)?),
            "verifKeyId" => verifKeyId_ = Some(BER.decode_octet_string(_el)?),
            "userVerifCertif" => userVerifCertif_ = Some(_decode_Certificate(_el)?),
            "theCACertificates" => {
                theCACertificates_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertificatePair>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "theCACertificates",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<CertificatePair> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_CertificatePair(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationPath")
                )
            }
        }
    }
    Ok(CertificationPath {
        userKeyId: userKeyId_,
        userCertif: userCertif_,
        verifKeyId: verifKeyId_,
        userVerifCertif: userVerifCertif_,
        theCACertificates: theCACertificates_,
    })
}

pub fn _encode_CertificationPath(value_: &CertificationPath) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    if let Some(v_) = &value_.userKeyId {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.userCertif {
        components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Certificate(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.verifKeyId {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.userVerifCertif {
        components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Certificate(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.theCACertificates {
        components_.push(|v_1: &Vec<CertificatePair>| -> ASN1Result<X690Element> {
            let mut el_1 = |value_: &SEQUENCE_OF<CertificatePair>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_CertificatePair(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 4;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_CertificationPath(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationPath")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificationPath,
        _eal_components_for_CertificationPath,
        _rctl2_components_for_CertificationPath,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userKeyId" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userKeyId")
                    );
                }
                Ok(BER.validate_octet_string(&el)?)
            }(_el)?,
            "userCertif" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userCertif")
                    );
                }
                Ok(_validate_Certificate(&el)?)
            }(_el)?,
            "verifKeyId" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "verifKeyId")
                    );
                }
                Ok(BER.validate_octet_string(&el)?)
            }(_el)?,
            "userVerifCertif" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "userVerifCertif",
                    ));
                }
                Ok(_validate_Certificate(&el)?)
            }(_el)?,
            "theCACertificates" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "theCACertificates",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_CertificatePair(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "theCACertificates",
                        )),
                    }
                }(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationPath")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// REQ-TOKEN ::= SEQUENCE {
///   req-contents   Req-contents,
///   algId          AlgorithmIdentifier{{SupportedAlgorithms}},
///   req-integrity  Integrity -- "token" is Req-contents
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct REQ_TOKEN {
    pub req_contents: Req_contents,
    pub algId: AlgorithmIdentifier,
    pub req_integrity: Integrity,
}
impl REQ_TOKEN {
    pub fn new(
        req_contents: Req_contents,
        algId: AlgorithmIdentifier,
        req_integrity: Integrity,
    ) -> Self {
        REQ_TOKEN {
            req_contents,
            algId,
            req_integrity,
        }
    }
}
impl TryFrom<&X690Element> for REQ_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_REQ_TOKEN(el)
    }
}

pub const _rctl1_components_for_REQ_TOKEN: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "req-contents",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "algId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "req-integrity",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_REQ_TOKEN: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_REQ_TOKEN: &[ComponentSpec; 0] = &[];

pub fn _decode_REQ_TOKEN(el: &X690Element) -> ASN1Result<REQ_TOKEN> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REQ-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_REQ_TOKEN,
        _eal_components_for_REQ_TOKEN,
        _rctl2_components_for_REQ_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut req_contents_: OPTIONAL<Req_contents> = None;
    let mut algId_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut req_integrity_: OPTIONAL<Integrity> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "req-contents" => req_contents_ = Some(_decode_Req_contents(_el)?),
            "algId" => algId_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "req-integrity" => req_integrity_ = Some(_decode_Integrity(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REQ-TOKEN"))
            }
        }
    }
    Ok(REQ_TOKEN {
        req_contents: req_contents_.unwrap(),
        algId: algId_.unwrap(),
        req_integrity: req_integrity_.unwrap(),
    })
}

pub fn _encode_REQ_TOKEN(value_: &REQ_TOKEN) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_Req_contents(&value_.req_contents)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
    components_.push(_encode_Integrity(&value_.req_integrity)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_REQ_TOKEN(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REQ-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_REQ_TOKEN,
        _eal_components_for_REQ_TOKEN,
        _rctl2_components_for_REQ_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "req-contents" => _validate_Req_contents(_el)?,
            "algId" => _validate_AlgorithmIdentifier(_el)?,
            "req-integrity" => _validate_Integrity(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REQ-TOKEN"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Integrity  ::=  BIT STRING
/// ```
pub type Integrity = BIT_STRING;

pub fn _decode_Integrity(el: &X690Element) -> ASN1Result<Integrity> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Integrity(value_: &Integrity) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Integrity(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Req-contents ::= SEQUENCE {
///   tok-id        INTEGER(256), -- shall contain 0100 (hex)
///   context-id    Random-Integer,
///   pvno          BIT STRING,
///   timestamp     UTCTime OPTIONAL, -- mandatory for SPKM-2
///   randSrc       Random-Integer,
///   targ-name     Name,
///   src-name      [0]  Name OPTIONAL,
///   req-data      Context-Data,
///   validity      [1]  Validity OPTIONAL,
///   key-estb-set  Key-Estb-Algs,
///   key-estb-req  BIT STRING OPTIONAL,
///   key-src-bind  OCTET STRING OPTIONAL
///   -- This field must be present for the case of SPKM-2
///   -- unilateral authen. if the K-ALG in use does not provide
///   -- such a binding (but is optional for all other cases).
///   -- The octet string holds the result of applying the
///   -- mandatory hashing procedure (in MANDATORY I-ALG;
///   -- see Section 2.1) as follows:  MD5(src || context_key),
///   -- where "src" is the DER-encoded octets of src-name,
///   -- "context-key" is the symmetric key (i.e., the
///   -- unprotected version of what is transmitted in
///   -- key-estb-req), and "||" is the concatenation operation.
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Req_contents {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
    pub pvno: BIT_STRING,
    pub timestamp: OPTIONAL<UTCTime>,
    pub randSrc: Random_Integer,
    pub targ_name: Name,
    pub src_name: OPTIONAL<Name>,
    pub req_data: Context_Data,
    pub validity: OPTIONAL<Validity>,
    pub key_estb_set: Key_Estb_Algs,
    pub key_estb_req: OPTIONAL<BIT_STRING>,
    pub key_src_bind: OPTIONAL<OCTET_STRING>,
}
impl Req_contents {
    pub fn new(
        tok_id: INTEGER,
        context_id: Random_Integer,
        pvno: BIT_STRING,
        timestamp: OPTIONAL<UTCTime>,
        randSrc: Random_Integer,
        targ_name: Name,
        src_name: OPTIONAL<Name>,
        req_data: Context_Data,
        validity: OPTIONAL<Validity>,
        key_estb_set: Key_Estb_Algs,
        key_estb_req: OPTIONAL<BIT_STRING>,
        key_src_bind: OPTIONAL<OCTET_STRING>,
    ) -> Self {
        Req_contents {
            tok_id,
            context_id,
            pvno,
            timestamp,
            randSrc,
            targ_name,
            src_name,
            req_data,
            validity,
            key_estb_set,
            key_estb_req,
            key_src_bind,
        }
    }
}
impl TryFrom<&X690Element> for Req_contents {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Req_contents(el)
    }
}

pub const _rctl1_components_for_Req_contents: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pvno",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timestamp",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "randSrc",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new("targ-name", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "src-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "req-data",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
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
        "key-estb-set",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "key-estb-req",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "key-src-bind",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Req_contents: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Req_contents: &[ComponentSpec; 0] = &[];

pub fn _decode_Req_contents(el: &X690Element) -> ASN1Result<Req_contents> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Req-contents")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Req_contents,
        _eal_components_for_Req_contents,
        _rctl2_components_for_Req_contents,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    let mut pvno_: OPTIONAL<BIT_STRING> = None;
    let mut timestamp_: OPTIONAL<UTCTime> = None;
    let mut randSrc_: OPTIONAL<Random_Integer> = None;
    let mut targ_name_: OPTIONAL<Name> = None;
    let mut src_name_: OPTIONAL<Name> = None;
    let mut req_data_: OPTIONAL<Context_Data> = None;
    let mut validity_: OPTIONAL<Validity> = None;
    let mut key_estb_set_: OPTIONAL<Key_Estb_Algs> = None;
    let mut key_estb_req_: OPTIONAL<BIT_STRING> = None;
    let mut key_src_bind_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            "pvno" => pvno_ = Some(BER.decode_bit_string(_el)?),
            "timestamp" => timestamp_ = Some(BER.decode_utc_time(_el)?),
            "randSrc" => randSrc_ = Some(_decode_Random_Integer(_el)?),
            "targ-name" => targ_name_ = Some(_decode_Name(_el)?),
            "src-name" => {
                src_name_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "req-data" => req_data_ = Some(_decode_Context_Data(_el)?),
            "validity" => validity_ = Some(_decode_Validity(_el)?),
            "key-estb-set" => key_estb_set_ = Some(_decode_Key_Estb_Algs(_el)?),
            "key-estb-req" => key_estb_req_ = Some(BER.decode_bit_string(_el)?),
            "key-src-bind" => key_src_bind_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Req-contents")
                )
            }
        }
    }
    Ok(Req_contents {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
        pvno: pvno_.unwrap(),
        timestamp: timestamp_,
        randSrc: randSrc_.unwrap(),
        targ_name: targ_name_.unwrap(),
        src_name: src_name_,
        req_data: req_data_.unwrap(),
        validity: validity_,
        key_estb_set: key_estb_set_.unwrap(),
        key_estb_req: key_estb_req_,
        key_src_bind: key_src_bind_,
    })
}

pub fn _encode_Req_contents(value_: &Req_contents) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(17);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    components_.push(BER.encode_bit_string(&value_.pvno)?);
    if let Some(v_) = &value_.timestamp {
        components_.push(BER.encode_utc_time(&v_)?);
    }
    components_.push(_encode_Random_Integer(&value_.randSrc)?);
    components_.push(_encode_Name(&value_.targ_name)?);
    if let Some(v_) = &value_.src_name {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(_encode_Context_Data(&value_.req_data)?);
    if let Some(v_) = &value_.validity {
        components_.push(|v_1: &Validity| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Validity(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_Key_Estb_Algs(&value_.key_estb_set)?);
    if let Some(v_) = &value_.key_estb_req {
        components_.push(BER.encode_bit_string(&v_)?);
    }
    if let Some(v_) = &value_.key_src_bind {
        components_.push(BER.encode_octet_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Req_contents(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Req-contents")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Req_contents,
        _eal_components_for_Req_contents,
        _rctl2_components_for_Req_contents,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            "pvno" => BER.validate_bit_string(_el)?,
            "timestamp" => BER.validate_utc_time(_el)?,
            "randSrc" => _validate_Random_Integer(_el)?,
            "targ-name" => _validate_Name(_el)?,
            "src-name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "src-name")
                    );
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "req-data" => _validate_Context_Data(_el)?,
            "validity" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "validity")
                    );
                }
                Ok(_validate_Validity(&el)?)
            }(_el)?,
            "key-estb-set" => _validate_Key_Estb_Algs(_el)?,
            "key-estb-req" => BER.validate_bit_string(_el)?,
            "key-src-bind" => BER.validate_octet_string(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Req-contents")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Random-Integer  ::=  BIT STRING
/// ```
pub type Random_Integer = BIT_STRING;

pub fn _decode_Random_Integer(el: &X690Element) -> ASN1Result<Random_Integer> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Random_Integer(value_: &Random_Integer) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Random_Integer(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Context-Data ::= SEQUENCE {
///   channelId   ChannelId OPTIONAL,
///   seq-number  INTEGER OPTIONAL,
///   options     Options,
///   conf-alg    Conf-Algs,
///   intg-alg    Intg-Algs,
///   owf-alg     OWF-Algs
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Context_Data {
    pub channelId: OPTIONAL<ChannelId>,
    pub seq_number: OPTIONAL<INTEGER>,
    pub options: Options,
    pub conf_alg: Conf_Algs,
    pub intg_alg: Intg_Algs,
    pub owf_alg: OWF_Algs,
}
impl Context_Data {
    pub fn new(
        channelId: OPTIONAL<ChannelId>,
        seq_number: OPTIONAL<INTEGER>,
        options: Options,
        conf_alg: Conf_Algs,
        intg_alg: Intg_Algs,
        owf_alg: OWF_Algs,
    ) -> Self {
        Context_Data {
            channelId,
            seq_number,
            options,
            conf_alg,
            intg_alg,
            owf_alg,
        }
    }
}
impl TryFrom<&X690Element> for Context_Data {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Context_Data(el)
    }
}

pub const _rctl1_components_for_Context_Data: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "channelId",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq-number",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "options",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new("conf-alg", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "intg-alg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "owf-alg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Context_Data: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Context_Data: &[ComponentSpec; 0] = &[];

pub fn _decode_Context_Data(el: &X690Element) -> ASN1Result<Context_Data> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-Data")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context_Data,
        _eal_components_for_Context_Data,
        _rctl2_components_for_Context_Data,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut channelId_: OPTIONAL<ChannelId> = None;
    let mut seq_number_: OPTIONAL<INTEGER> = None;
    let mut options_: OPTIONAL<Options> = None;
    let mut conf_alg_: OPTIONAL<Conf_Algs> = None;
    let mut intg_alg_: OPTIONAL<Intg_Algs> = None;
    let mut owf_alg_: OPTIONAL<OWF_Algs> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "channelId" => channelId_ = Some(_decode_ChannelId(_el)?),
            "seq-number" => seq_number_ = Some(BER.decode_integer(_el)?),
            "options" => options_ = Some(_decode_Options(_el)?),
            "conf-alg" => conf_alg_ = Some(_decode_Conf_Algs(_el)?),
            "intg-alg" => intg_alg_ = Some(_decode_Intg_Algs(_el)?),
            "owf-alg" => owf_alg_ = Some(_decode_OWF_Algs(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-Data")
                )
            }
        }
    }
    Ok(Context_Data {
        channelId: channelId_,
        seq_number: seq_number_,
        options: options_.unwrap(),
        conf_alg: conf_alg_.unwrap(),
        intg_alg: intg_alg_.unwrap(),
        owf_alg: owf_alg_.unwrap(),
    })
}

pub fn _encode_Context_Data(value_: &Context_Data) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    if let Some(v_) = &value_.channelId {
        components_.push(_encode_ChannelId(&v_)?);
    }
    if let Some(v_) = &value_.seq_number {
        components_.push(BER.encode_integer(&v_)?);
    }
    components_.push(_encode_Options(&value_.options)?);
    components_.push(_encode_Conf_Algs(&value_.conf_alg)?);
    components_.push(_encode_Intg_Algs(&value_.intg_alg)?);
    components_.push(_encode_OWF_Algs(&value_.owf_alg)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Context_Data(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-Data")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context_Data,
        _eal_components_for_Context_Data,
        _rctl2_components_for_Context_Data,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "channelId" => _validate_ChannelId(_el)?,
            "seq-number" => BER.validate_integer(_el)?,
            "options" => _validate_Options(_el)?,
            "conf-alg" => _validate_Conf_Algs(_el)?,
            "intg-alg" => _validate_Intg_Algs(_el)?,
            "owf-alg" => _validate_OWF_Algs(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-Data")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChannelId  ::=  OCTET STRING
/// ```
pub type ChannelId = OCTET_STRING; // OctetStringType

pub fn _decode_ChannelId(el: &X690Element) -> ASN1Result<ChannelId> {
    BER.decode_octet_string(&el)
}

pub fn _encode_ChannelId(value_: &ChannelId) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_ChannelId(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Options  ::=  BIT STRING {
///   delegation-state(0), mutual-state(1), replay-det-state(2), sequence-state(3),
///   conf-avail(4), integ-avail(5), target-certif-data-required(6)}
/// ```
pub type Options = BIT_STRING;

pub const Options_delegation_state: BIT = 0; /* LONG_NAMED_BIT */

pub const Options_mutual_state: BIT = 1; /* LONG_NAMED_BIT */

pub const Options_replay_det_state: BIT = 2; /* LONG_NAMED_BIT */

pub const Options_sequence_state: BIT = 3; /* LONG_NAMED_BIT */

pub const Options_conf_avail: BIT = 4; /* LONG_NAMED_BIT */

pub const Options_integ_avail: BIT = 5; /* LONG_NAMED_BIT */

pub const Options_target_certif_data_required: BIT = 6; /* LONG_NAMED_BIT */

pub fn _decode_Options(el: &X690Element) -> ASN1Result<Options> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Options(value_: &Options) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Options(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Conf-Algs  ::=  CHOICE {
///   algs  [0]  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}},
///   null  [1]  NULL
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Conf_Algs {
    algs(Vec<AlgorithmIdentifier>),
    null(NULL),
}

impl TryFrom<&X690Element> for Conf_Algs {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Conf_Algs(el)
    }
}

pub fn _decode_Conf_Algs(el: &X690Element) -> ASN1Result<Conf_Algs> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(Conf_Algs::algs(|el: &X690Element| -> ASN1Result<
            SEQUENCE_OF<AlgorithmIdentifier>,
        > {
            let elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "algs")),
            };
            let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
            for el in elements.iter() {
                items.push(_decode_AlgorithmIdentifier(el)?);
            }
            Ok(items)
        }(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(Conf_Algs::null(BER.decode_null(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Conf-Algs",
            ))
        }
    }
}

pub fn _encode_Conf_Algs(value_: &Conf_Algs) -> ASN1Result<X690Element> {
    match value_ {
        Conf_Algs::algs(v) => |v_1: &Vec<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
            let mut el_1 = |value_: &SEQUENCE_OF<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_AlgorithmIdentifier(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        Conf_Algs::null(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_null(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_Conf_Algs(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "algs"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_AlgorithmIdentifier(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "algs")),
                }
            }(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "null"));
            }
            Ok(BER.validate_null(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Conf-Algs",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Intg-Algs  ::=  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type Intg_Algs = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_Intg_Algs(el: &X690Element) -> ASN1Result<Intg_Algs> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Intg-Algs")),
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_Intg_Algs(value_: &Intg_Algs) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Intg_Algs(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Intg-Algs")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OWF-Algs  ::=  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type OWF_Algs = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_OWF_Algs(el: &X690Element) -> ASN1Result<OWF_Algs> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OWF-Algs")),
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_OWF_Algs(value_: &OWF_Algs) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_OWF_Algs(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OWF-Algs")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Key-Estb-Algs  ::=  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type Key_Estb_Algs = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_Key_Estb_Algs(el: &X690Element) -> ASN1Result<Key_Estb_Algs> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Key-Estb-Algs")),
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_Key_Estb_Algs(value_: &Key_Estb_Algs) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Key_Estb_Algs(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Key-Estb-Algs")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-REP-TI ::= SEQUENCE {
///   responseToken  REP-TI-TOKEN,
///   certif-data    CertificationData OPTIONAL
///   -- present if target-certif-data-required option was
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_REP_TI {
    pub responseToken: REP_TI_TOKEN,
    pub certif_data: OPTIONAL<CertificationData>,
}
impl SPKM_REP_TI {
    pub fn new(responseToken: REP_TI_TOKEN, certif_data: OPTIONAL<CertificationData>) -> Self {
        SPKM_REP_TI {
            responseToken,
            certif_data,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_REP_TI {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_REP_TI(el)
    }
}

pub const _rctl1_components_for_SPKM_REP_TI: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "responseToken",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certif-data",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_REP_TI: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_REP_TI: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_REP_TI(el: &X690Element) -> ASN1Result<SPKM_REP_TI> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-TI")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_REP_TI,
        _eal_components_for_SPKM_REP_TI,
        _rctl2_components_for_SPKM_REP_TI,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut responseToken_: OPTIONAL<REP_TI_TOKEN> = None;
    let mut certif_data_: OPTIONAL<CertificationData> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "responseToken" => responseToken_ = Some(_decode_REP_TI_TOKEN(_el)?),
            "certif-data" => certif_data_ = Some(_decode_CertificationData(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-TI")
                )
            }
        }
    }
    Ok(SPKM_REP_TI {
        responseToken: responseToken_.unwrap(),
        certif_data: certif_data_,
    })
}

pub fn _encode_SPKM_REP_TI(value_: &SPKM_REP_TI) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_REP_TI_TOKEN(&value_.responseToken)?);
    if let Some(v_) = &value_.certif_data {
        components_.push(_encode_CertificationData(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_REP_TI(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-TI")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_REP_TI,
        _eal_components_for_SPKM_REP_TI,
        _rctl2_components_for_SPKM_REP_TI,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "responseToken" => _validate_REP_TI_TOKEN(_el)?,
            "certif-data" => _validate_CertificationData(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-TI")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// REP-TI-TOKEN ::= SEQUENCE {
///   rep-ti-contents  Rep-ti-contents,
///   algId            AlgorithmIdentifier{{SupportedAlgorithms}},
///   rep-ti-integ     Integrity -- "token" is Rep-ti-contents
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct REP_TI_TOKEN {
    pub rep_ti_contents: Rep_ti_contents,
    pub algId: AlgorithmIdentifier,
    pub rep_ti_integ: Integrity,
}
impl REP_TI_TOKEN {
    pub fn new(
        rep_ti_contents: Rep_ti_contents,
        algId: AlgorithmIdentifier,
        rep_ti_integ: Integrity,
    ) -> Self {
        REP_TI_TOKEN {
            rep_ti_contents,
            algId,
            rep_ti_integ,
        }
    }
}
impl TryFrom<&X690Element> for REP_TI_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_REP_TI_TOKEN(el)
    }
}

pub const _rctl1_components_for_REP_TI_TOKEN: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "rep-ti-contents",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "algId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "rep-ti-integ",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_REP_TI_TOKEN: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_REP_TI_TOKEN: &[ComponentSpec; 0] = &[];

pub fn _decode_REP_TI_TOKEN(el: &X690Element) -> ASN1Result<REP_TI_TOKEN> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-TI-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_REP_TI_TOKEN,
        _eal_components_for_REP_TI_TOKEN,
        _rctl2_components_for_REP_TI_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut rep_ti_contents_: OPTIONAL<Rep_ti_contents> = None;
    let mut algId_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut rep_ti_integ_: OPTIONAL<Integrity> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "rep-ti-contents" => rep_ti_contents_ = Some(_decode_Rep_ti_contents(_el)?),
            "algId" => algId_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "rep-ti-integ" => rep_ti_integ_ = Some(_decode_Integrity(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-TI-TOKEN")
                )
            }
        }
    }
    Ok(REP_TI_TOKEN {
        rep_ti_contents: rep_ti_contents_.unwrap(),
        algId: algId_.unwrap(),
        rep_ti_integ: rep_ti_integ_.unwrap(),
    })
}

pub fn _encode_REP_TI_TOKEN(value_: &REP_TI_TOKEN) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_Rep_ti_contents(&value_.rep_ti_contents)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
    components_.push(_encode_Integrity(&value_.rep_ti_integ)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_REP_TI_TOKEN(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-TI-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_REP_TI_TOKEN,
        _eal_components_for_REP_TI_TOKEN,
        _rctl2_components_for_REP_TI_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "rep-ti-contents" => _validate_Rep_ti_contents(_el)?,
            "algId" => _validate_AlgorithmIdentifier(_el)?,
            "rep-ti-integ" => _validate_Integrity(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-TI-TOKEN")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Rep-ti-contents ::= SEQUENCE {
///   tok-id        INTEGER(512), -- shall contain 0200 (hex)
///   context-id    Random-Integer,
///   pvno          [0]  BIT STRING OPTIONAL,
///   timestamp     UTCTime OPTIONAL, -- mandatory for SPKM-2
///   randTarg      Random-Integer,
///   src-name      [1]  Name OPTIONAL,
///   targ-name     Name,
///   randSrc       Random-Integer,
///   rep-data      Context-Data,
///   validity      [2]  Validity OPTIONAL,
///   key-estb-id   AlgorithmIdentifier{{SupportedAlgorithms}} OPTIONAL,
///   key-estb-str  BIT STRING OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Rep_ti_contents {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
    pub pvno: OPTIONAL<BIT_STRING>,
    pub timestamp: OPTIONAL<UTCTime>,
    pub randTarg: Random_Integer,
    pub src_name: OPTIONAL<Name>,
    pub targ_name: Name,
    pub randSrc: Random_Integer,
    pub rep_data: Context_Data,
    pub validity: OPTIONAL<Validity>,
    pub key_estb_id: OPTIONAL<AlgorithmIdentifier>,
    pub key_estb_str: OPTIONAL<BIT_STRING>,
}
impl Rep_ti_contents {
    pub fn new(
        tok_id: INTEGER,
        context_id: Random_Integer,
        pvno: OPTIONAL<BIT_STRING>,
        timestamp: OPTIONAL<UTCTime>,
        randTarg: Random_Integer,
        src_name: OPTIONAL<Name>,
        targ_name: Name,
        randSrc: Random_Integer,
        rep_data: Context_Data,
        validity: OPTIONAL<Validity>,
        key_estb_id: OPTIONAL<AlgorithmIdentifier>,
        key_estb_str: OPTIONAL<BIT_STRING>,
    ) -> Self {
        Rep_ti_contents {
            tok_id,
            context_id,
            pvno,
            timestamp,
            randTarg,
            src_name,
            targ_name,
            randSrc,
            rep_data,
            validity,
            key_estb_id,
            key_estb_str,
        }
    }
}
impl TryFrom<&X690Element> for Rep_ti_contents {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Rep_ti_contents(el)
    }
}

pub const _rctl1_components_for_Rep_ti_contents: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pvno",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timestamp",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 23)),
        None,
        None,
    ),
    ComponentSpec::new(
        "randTarg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "src-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new("targ-name", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "randSrc",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "rep-data",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "validity",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "key-estb-id",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "key-estb-str",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Rep_ti_contents: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Rep_ti_contents: &[ComponentSpec; 0] = &[];

pub fn _decode_Rep_ti_contents(el: &X690Element) -> ASN1Result<Rep_ti_contents> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Rep-ti-contents"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Rep_ti_contents,
        _eal_components_for_Rep_ti_contents,
        _rctl2_components_for_Rep_ti_contents,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    let mut pvno_: OPTIONAL<BIT_STRING> = None;
    let mut timestamp_: OPTIONAL<UTCTime> = None;
    let mut randTarg_: OPTIONAL<Random_Integer> = None;
    let mut src_name_: OPTIONAL<Name> = None;
    let mut targ_name_: OPTIONAL<Name> = None;
    let mut randSrc_: OPTIONAL<Random_Integer> = None;
    let mut rep_data_: OPTIONAL<Context_Data> = None;
    let mut validity_: OPTIONAL<Validity> = None;
    let mut key_estb_id_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut key_estb_str_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            "pvno" => pvno_ = Some(BER.decode_bit_string(_el)?),
            "timestamp" => timestamp_ = Some(BER.decode_utc_time(_el)?),
            "randTarg" => randTarg_ = Some(_decode_Random_Integer(_el)?),
            "src-name" => {
                src_name_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "targ-name" => targ_name_ = Some(_decode_Name(_el)?),
            "randSrc" => randSrc_ = Some(_decode_Random_Integer(_el)?),
            "rep-data" => rep_data_ = Some(_decode_Context_Data(_el)?),
            "validity" => validity_ = Some(_decode_Validity(_el)?),
            "key-estb-id" => key_estb_id_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "key-estb-str" => key_estb_str_ = Some(BER.decode_bit_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Rep-ti-contents")
                )
            }
        }
    }
    Ok(Rep_ti_contents {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
        pvno: pvno_,
        timestamp: timestamp_,
        randTarg: randTarg_.unwrap(),
        src_name: src_name_,
        targ_name: targ_name_.unwrap(),
        randSrc: randSrc_.unwrap(),
        rep_data: rep_data_.unwrap(),
        validity: validity_,
        key_estb_id: key_estb_id_,
        key_estb_str: key_estb_str_,
    })
}

pub fn _encode_Rep_ti_contents(value_: &Rep_ti_contents) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(17);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    if let Some(v_) = &value_.pvno {
        components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_bit_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.timestamp {
        components_.push(BER.encode_utc_time(&v_)?);
    }
    components_.push(_encode_Random_Integer(&value_.randTarg)?);
    if let Some(v_) = &value_.src_name {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(_encode_Name(&value_.targ_name)?);
    components_.push(_encode_Random_Integer(&value_.randSrc)?);
    components_.push(_encode_Context_Data(&value_.rep_data)?);
    if let Some(v_) = &value_.validity {
        components_.push(|v_1: &Validity| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Validity(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.key_estb_id {
        components_.push(_encode_AlgorithmIdentifier(&v_)?);
    }
    if let Some(v_) = &value_.key_estb_str {
        components_.push(BER.encode_bit_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Rep_ti_contents(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Rep-ti-contents"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Rep_ti_contents,
        _eal_components_for_Rep_ti_contents,
        _rctl2_components_for_Rep_ti_contents,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            "pvno" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pvno"));
                }
                Ok(BER.validate_bit_string(&el)?)
            }(_el)?,
            "timestamp" => BER.validate_utc_time(_el)?,
            "randTarg" => _validate_Random_Integer(_el)?,
            "src-name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "src-name")
                    );
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "targ-name" => _validate_Name(_el)?,
            "randSrc" => _validate_Random_Integer(_el)?,
            "rep-data" => _validate_Context_Data(_el)?,
            "validity" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "validity")
                    );
                }
                Ok(_validate_Validity(&el)?)
            }(_el)?,
            "key-estb-id" => _validate_AlgorithmIdentifier(_el)?,
            "key-estb-str" => BER.validate_bit_string(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Rep-ti-contents")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-REP-IT ::= SEQUENCE {
///   responseToken  REP-IT-TOKEN,
///   algId          AlgorithmIdentifier{{SupportedAlgorithms}},
///   rep-it-integ   Integrity -- "token" is REP-IT-TOKEN
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_REP_IT {
    pub responseToken: REP_IT_TOKEN,
    pub algId: AlgorithmIdentifier,
    pub rep_it_integ: Integrity,
}
impl SPKM_REP_IT {
    pub fn new(
        responseToken: REP_IT_TOKEN,
        algId: AlgorithmIdentifier,
        rep_it_integ: Integrity,
    ) -> Self {
        SPKM_REP_IT {
            responseToken,
            algId,
            rep_it_integ,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_REP_IT {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_REP_IT(el)
    }
}

pub const _rctl1_components_for_SPKM_REP_IT: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "responseToken",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "algId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "rep-it-integ",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_REP_IT: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_REP_IT: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_REP_IT(el: &X690Element) -> ASN1Result<SPKM_REP_IT> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-IT")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_REP_IT,
        _eal_components_for_SPKM_REP_IT,
        _rctl2_components_for_SPKM_REP_IT,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut responseToken_: OPTIONAL<REP_IT_TOKEN> = None;
    let mut algId_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut rep_it_integ_: OPTIONAL<Integrity> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "responseToken" => responseToken_ = Some(_decode_REP_IT_TOKEN(_el)?),
            "algId" => algId_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "rep-it-integ" => rep_it_integ_ = Some(_decode_Integrity(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-IT")
                )
            }
        }
    }
    Ok(SPKM_REP_IT {
        responseToken: responseToken_.unwrap(),
        algId: algId_.unwrap(),
        rep_it_integ: rep_it_integ_.unwrap(),
    })
}

pub fn _encode_SPKM_REP_IT(value_: &SPKM_REP_IT) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_REP_IT_TOKEN(&value_.responseToken)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
    components_.push(_encode_Integrity(&value_.rep_it_integ)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_REP_IT(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-IT")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_REP_IT,
        _eal_components_for_SPKM_REP_IT,
        _rctl2_components_for_SPKM_REP_IT,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "responseToken" => _validate_REP_IT_TOKEN(_el)?,
            "algId" => _validate_AlgorithmIdentifier(_el)?,
            "rep-it-integ" => _validate_Integrity(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-REP-IT")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// REP-IT-TOKEN ::= SEQUENCE {
///   tok-id        INTEGER(768), -- shall contain 0300 (hex)
///   context-id    Random-Integer,
///   randSrc       Random-Integer,
///   randTarg      Random-Integer,
///   targ-name     Name,
///   src-name      Name OPTIONAL,
///   key-estb-rep  BIT STRING OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct REP_IT_TOKEN {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
    pub randSrc: Random_Integer,
    pub randTarg: Random_Integer,
    pub targ_name: Name,
    pub src_name: OPTIONAL<Name>,
    pub key_estb_rep: OPTIONAL<BIT_STRING>,
}
impl REP_IT_TOKEN {
    pub fn new(
        tok_id: INTEGER,
        context_id: Random_Integer,
        randSrc: Random_Integer,
        randTarg: Random_Integer,
        targ_name: Name,
        src_name: OPTIONAL<Name>,
        key_estb_rep: OPTIONAL<BIT_STRING>,
    ) -> Self {
        REP_IT_TOKEN {
            tok_id,
            context_id,
            randSrc,
            randTarg,
            targ_name,
            src_name,
            key_estb_rep,
        }
    }
}
impl TryFrom<&X690Element> for REP_IT_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_REP_IT_TOKEN(el)
    }
}

pub const _rctl1_components_for_REP_IT_TOKEN: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "randSrc",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "randTarg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new("targ-name", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "src-name",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::UNIVERSAL, 16))]),
        None,
        None,
    ),
    ComponentSpec::new(
        "key-estb-rep",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_REP_IT_TOKEN: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_REP_IT_TOKEN: &[ComponentSpec; 0] = &[];

pub fn _decode_REP_IT_TOKEN(el: &X690Element) -> ASN1Result<REP_IT_TOKEN> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-IT-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_REP_IT_TOKEN,
        _eal_components_for_REP_IT_TOKEN,
        _rctl2_components_for_REP_IT_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    let mut randSrc_: OPTIONAL<Random_Integer> = None;
    let mut randTarg_: OPTIONAL<Random_Integer> = None;
    let mut targ_name_: OPTIONAL<Name> = None;
    let mut src_name_: OPTIONAL<Name> = None;
    let mut key_estb_rep_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            "randSrc" => randSrc_ = Some(_decode_Random_Integer(_el)?),
            "randTarg" => randTarg_ = Some(_decode_Random_Integer(_el)?),
            "targ-name" => targ_name_ = Some(_decode_Name(_el)?),
            "src-name" => src_name_ = Some(_decode_Name(_el)?),
            "key-estb-rep" => key_estb_rep_ = Some(BER.decode_bit_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-IT-TOKEN")
                )
            }
        }
    }
    Ok(REP_IT_TOKEN {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
        randSrc: randSrc_.unwrap(),
        randTarg: randTarg_.unwrap(),
        targ_name: targ_name_.unwrap(),
        src_name: src_name_,
        key_estb_rep: key_estb_rep_,
    })
}

pub fn _encode_REP_IT_TOKEN(value_: &REP_IT_TOKEN) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    components_.push(_encode_Random_Integer(&value_.randSrc)?);
    components_.push(_encode_Random_Integer(&value_.randTarg)?);
    components_.push(_encode_Name(&value_.targ_name)?);
    if let Some(v_) = &value_.src_name {
        components_.push(_encode_Name(&v_)?);
    }
    if let Some(v_) = &value_.key_estb_rep {
        components_.push(BER.encode_bit_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_REP_IT_TOKEN(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-IT-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_REP_IT_TOKEN,
        _eal_components_for_REP_IT_TOKEN,
        _rctl2_components_for_REP_IT_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            "randSrc" => _validate_Random_Integer(_el)?,
            "randTarg" => _validate_Random_Integer(_el)?,
            "targ-name" => _validate_Name(_el)?,
            "src-name" => _validate_Name(_el)?,
            "key-estb-rep" => BER.validate_bit_string(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "REP-IT-TOKEN")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-ERROR ::= SEQUENCE {
///   errorToken  ERROR-TOKEN,
///   algId       AlgorithmIdentifier{{SupportedAlgorithms}},
///   integrity   Integrity -- "token" is ERROR-TOKEN
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_ERROR {
    pub errorToken: ERROR_TOKEN,
    pub algId: AlgorithmIdentifier,
    pub integrity: Integrity,
}
impl SPKM_ERROR {
    pub fn new(errorToken: ERROR_TOKEN, algId: AlgorithmIdentifier, integrity: Integrity) -> Self {
        SPKM_ERROR {
            errorToken,
            algId,
            integrity,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_ERROR {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_ERROR(el)
    }
}

pub const _rctl1_components_for_SPKM_ERROR: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "errorToken",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "algId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "integrity",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_ERROR: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_ERROR: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_ERROR(el: &X690Element) -> ASN1Result<SPKM_ERROR> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-ERROR")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_ERROR,
        _eal_components_for_SPKM_ERROR,
        _rctl2_components_for_SPKM_ERROR,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut errorToken_: OPTIONAL<ERROR_TOKEN> = None;
    let mut algId_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut integrity_: OPTIONAL<Integrity> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "errorToken" => errorToken_ = Some(_decode_ERROR_TOKEN(_el)?),
            "algId" => algId_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "integrity" => integrity_ = Some(_decode_Integrity(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-ERROR"))
            }
        }
    }
    Ok(SPKM_ERROR {
        errorToken: errorToken_.unwrap(),
        algId: algId_.unwrap(),
        integrity: integrity_.unwrap(),
    })
}

pub fn _encode_SPKM_ERROR(value_: &SPKM_ERROR) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_ERROR_TOKEN(&value_.errorToken)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
    components_.push(_encode_Integrity(&value_.integrity)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_ERROR(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-ERROR")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_ERROR,
        _eal_components_for_SPKM_ERROR,
        _rctl2_components_for_SPKM_ERROR,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "errorToken" => _validate_ERROR_TOKEN(_el)?,
            "algId" => _validate_AlgorithmIdentifier(_el)?,
            "integrity" => _validate_Integrity(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-ERROR"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ERROR-TOKEN ::= SEQUENCE {
///   tok-id      INTEGER(1024), -- shall contain 0400 (hex)
///   context-id  Random-Integer
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ERROR_TOKEN {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
}
impl ERROR_TOKEN {
    pub fn new(tok_id: INTEGER, context_id: Random_Integer) -> Self {
        ERROR_TOKEN { tok_id, context_id }
    }
}
impl TryFrom<&X690Element> for ERROR_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ERROR_TOKEN(el)
    }
}

pub const _rctl1_components_for_ERROR_TOKEN: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ERROR_TOKEN: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ERROR_TOKEN: &[ComponentSpec; 0] = &[];

pub fn _decode_ERROR_TOKEN(el: &X690Element) -> ASN1Result<ERROR_TOKEN> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ERROR-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ERROR_TOKEN,
        _eal_components_for_ERROR_TOKEN,
        _rctl2_components_for_ERROR_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ERROR-TOKEN")
                )
            }
        }
    }
    Ok(ERROR_TOKEN {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
    })
}

pub fn _encode_ERROR_TOKEN(value_: &ERROR_TOKEN) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ERROR_TOKEN(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ERROR-TOKEN")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ERROR_TOKEN,
        _eal_components_for_ERROR_TOKEN,
        _rctl2_components_for_ERROR_TOKEN,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ERROR-TOKEN")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-MIC ::= SEQUENCE {mic-header  Mic-Header,
///                        int-cksum   BIT STRING
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_MIC {
    pub mic_header: Mic_Header,
    pub int_cksum: BIT_STRING,
}
impl SPKM_MIC {
    pub fn new(mic_header: Mic_Header, int_cksum: BIT_STRING) -> Self {
        SPKM_MIC {
            mic_header,
            int_cksum,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_MIC {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_MIC(el)
    }
}

pub const _rctl1_components_for_SPKM_MIC: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "mic-header",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "int-cksum",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_MIC: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_MIC: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_MIC(el: &X690Element) -> ASN1Result<SPKM_MIC> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-MIC")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_MIC,
        _eal_components_for_SPKM_MIC,
        _rctl2_components_for_SPKM_MIC,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut mic_header_: OPTIONAL<Mic_Header> = None;
    let mut int_cksum_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mic-header" => mic_header_ = Some(_decode_Mic_Header(_el)?),
            "int-cksum" => int_cksum_ = Some(BER.decode_bit_string(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-MIC")),
        }
    }
    Ok(SPKM_MIC {
        mic_header: mic_header_.unwrap(),
        int_cksum: int_cksum_.unwrap(),
    })
}

pub fn _encode_SPKM_MIC(value_: &SPKM_MIC) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Mic_Header(&value_.mic_header)?);
    components_.push(BER.encode_bit_string(&value_.int_cksum)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_MIC(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-MIC")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_MIC,
        _eal_components_for_SPKM_MIC,
        _rctl2_components_for_SPKM_MIC,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mic-header" => _validate_Mic_Header(_el)?,
            "int-cksum" => BER.validate_bit_string(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-MIC")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Mic-Header ::= SEQUENCE {
///   tok-id      INTEGER(257), -- shall contain 0101 (hex)
///   context-id  Random-Integer,
///   int-alg     [0]  AlgorithmIdentifier{{SupportedAlgorithms}} OPTIONAL,
///   snd-seq     [1]  SeqNum OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Mic_Header {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
    pub int_alg: OPTIONAL<AlgorithmIdentifier>,
    pub snd_seq: OPTIONAL<SeqNum>,
}
impl Mic_Header {
    pub fn new(
        tok_id: INTEGER,
        context_id: Random_Integer,
        int_alg: OPTIONAL<AlgorithmIdentifier>,
        snd_seq: OPTIONAL<SeqNum>,
    ) -> Self {
        Mic_Header {
            tok_id,
            context_id,
            int_alg,
            snd_seq,
        }
    }
}
impl TryFrom<&X690Element> for Mic_Header {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Mic_Header(el)
    }
}

pub const _rctl1_components_for_Mic_Header: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "int-alg",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "snd-seq",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Mic_Header: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Mic_Header: &[ComponentSpec; 0] = &[];

pub fn _decode_Mic_Header(el: &X690Element) -> ASN1Result<Mic_Header> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Mic-Header")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Mic_Header,
        _eal_components_for_Mic_Header,
        _rctl2_components_for_Mic_Header,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    let mut int_alg_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut snd_seq_: OPTIONAL<SeqNum> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            "int-alg" => int_alg_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "snd-seq" => snd_seq_ = Some(_decode_SeqNum(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Mic-Header"))
            }
        }
    }
    Ok(Mic_Header {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
        int_alg: int_alg_,
        snd_seq: snd_seq_,
    })
}

pub fn _encode_Mic_Header(value_: &Mic_Header) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    if let Some(v_) = &value_.int_alg {
        components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.snd_seq {
        components_.push(|v_1: &SeqNum| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SeqNum(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Mic_Header(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Mic-Header")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Mic_Header,
        _eal_components_for_Mic_Header,
        _rctl2_components_for_Mic_Header,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            "int-alg" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "int-alg")
                    );
                }
                Ok(_validate_AlgorithmIdentifier(&el)?)
            }(_el)?,
            "snd-seq" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "snd-seq")
                    );
                }
                Ok(_validate_SeqNum(&el)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Mic-Header"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SeqNum ::= SEQUENCE {num      INTEGER,
///                      dir-ind  BOOLEAN
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SeqNum {
    pub num: INTEGER,
    pub dir_ind: BOOLEAN,
}
impl SeqNum {
    pub fn new(num: INTEGER, dir_ind: BOOLEAN) -> Self {
        SeqNum { num, dir_ind }
    }
}
impl TryFrom<&X690Element> for SeqNum {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SeqNum(el)
    }
}

pub const _rctl1_components_for_SeqNum: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "num",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dir-ind",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SeqNum: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SeqNum: &[ComponentSpec; 0] = &[];

pub fn _decode_SeqNum(el: &X690Element) -> ASN1Result<SeqNum> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SeqNum")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SeqNum,
        _eal_components_for_SeqNum,
        _rctl2_components_for_SeqNum,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut num_: OPTIONAL<INTEGER> = None;
    let mut dir_ind_: OPTIONAL<BOOLEAN> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "num" => num_ = Some(BER.decode_integer(_el)?),
            "dir-ind" => dir_ind_ = Some(BER.decode_boolean(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SeqNum")),
        }
    }
    Ok(SeqNum {
        num: num_.unwrap(),
        dir_ind: dir_ind_.unwrap(),
    })
}

pub fn _encode_SeqNum(value_: &SeqNum) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.num)?);
    components_.push(BER.encode_boolean(&value_.dir_ind)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SeqNum(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SeqNum")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SeqNum,
        _eal_components_for_SeqNum,
        _rctl2_components_for_SeqNum,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "num" => BER.validate_integer(_el)?,
            "dir-ind" => BER.validate_boolean(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SeqNum")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-WRAP ::= SEQUENCE {wrap-header  Wrap-Header,
///                         wrap-body    Wrap-Body
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_WRAP {
    pub wrap_header: Wrap_Header,
    pub wrap_body: Wrap_Body,
}
impl SPKM_WRAP {
    pub fn new(wrap_header: Wrap_Header, wrap_body: Wrap_Body) -> Self {
        SPKM_WRAP {
            wrap_header,
            wrap_body,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_WRAP {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_WRAP(el)
    }
}

pub const _rctl1_components_for_SPKM_WRAP: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "wrap-header",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "wrap-body",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_WRAP: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_WRAP: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_WRAP(el: &X690Element) -> ASN1Result<SPKM_WRAP> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-WRAP")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_WRAP,
        _eal_components_for_SPKM_WRAP,
        _rctl2_components_for_SPKM_WRAP,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut wrap_header_: OPTIONAL<Wrap_Header> = None;
    let mut wrap_body_: OPTIONAL<Wrap_Body> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "wrap-header" => wrap_header_ = Some(_decode_Wrap_Header(_el)?),
            "wrap-body" => wrap_body_ = Some(_decode_Wrap_Body(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-WRAP"))
            }
        }
    }
    Ok(SPKM_WRAP {
        wrap_header: wrap_header_.unwrap(),
        wrap_body: wrap_body_.unwrap(),
    })
}

pub fn _encode_SPKM_WRAP(value_: &SPKM_WRAP) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Wrap_Header(&value_.wrap_header)?);
    components_.push(_encode_Wrap_Body(&value_.wrap_body)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_WRAP(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-WRAP")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_WRAP,
        _eal_components_for_SPKM_WRAP,
        _rctl2_components_for_SPKM_WRAP,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "wrap-header" => _validate_Wrap_Header(_el)?,
            "wrap-body" => _validate_Wrap_Body(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-WRAP"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Wrap-Header ::= SEQUENCE {
///   tok-id      INTEGER(513), -- shall contain 0201 (hex)
///   context-id  Random-Integer,
///   int-alg     [0]  AlgorithmIdentifier{{SupportedAlgorithms}} OPTIONAL,
///   conf-alg    [1]  Conf-Alg OPTIONAL,
///   snd-seq     [2]  SeqNum OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Wrap_Header {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
    pub int_alg: OPTIONAL<AlgorithmIdentifier>,
    pub conf_alg: OPTIONAL<Conf_Alg>,
    pub snd_seq: OPTIONAL<SeqNum>,
}
impl Wrap_Header {
    pub fn new(
        tok_id: INTEGER,
        context_id: Random_Integer,
        int_alg: OPTIONAL<AlgorithmIdentifier>,
        conf_alg: OPTIONAL<Conf_Alg>,
        snd_seq: OPTIONAL<SeqNum>,
    ) -> Self {
        Wrap_Header {
            tok_id,
            context_id,
            int_alg,
            conf_alg,
            snd_seq,
        }
    }
}
impl TryFrom<&X690Element> for Wrap_Header {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Wrap_Header(el)
    }
}

pub const _rctl1_components_for_Wrap_Header: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "int-alg",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "conf-alg",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "snd-seq",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Wrap_Header: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Wrap_Header: &[ComponentSpec; 0] = &[];

pub fn _decode_Wrap_Header(el: &X690Element) -> ASN1Result<Wrap_Header> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Header")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Wrap_Header,
        _eal_components_for_Wrap_Header,
        _rctl2_components_for_Wrap_Header,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    let mut int_alg_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut conf_alg_: OPTIONAL<Conf_Alg> = None;
    let mut snd_seq_: OPTIONAL<SeqNum> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            "int-alg" => int_alg_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "conf-alg" => {
                conf_alg_ = Some(|el: &X690Element| -> ASN1Result<Conf_Alg> {
                    Ok(_decode_Conf_Alg(&el.inner()?)?)
                }(_el)?)
            }
            "snd-seq" => snd_seq_ = Some(_decode_SeqNum(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Header")
                )
            }
        }
    }
    Ok(Wrap_Header {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
        int_alg: int_alg_,
        conf_alg: conf_alg_,
        snd_seq: snd_seq_,
    })
}

pub fn _encode_Wrap_Header(value_: &Wrap_Header) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    if let Some(v_) = &value_.int_alg {
        components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.conf_alg {
        components_.push(|v_1: &Conf_Alg| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_Conf_Alg(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.snd_seq {
        components_.push(|v_1: &SeqNum| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SeqNum(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Wrap_Header(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Header")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Wrap_Header,
        _eal_components_for_Wrap_Header,
        _rctl2_components_for_Wrap_Header,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            "int-alg" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "int-alg")
                    );
                }
                Ok(_validate_AlgorithmIdentifier(&el)?)
            }(_el)?,
            "conf-alg" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "conf-alg")
                    );
                }
                Ok(_validate_Conf_Alg(&el.inner()?)?)
            }(_el)?,
            "snd-seq" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "snd-seq")
                    );
                }
                Ok(_validate_SeqNum(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Header")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Wrap-Body ::= SEQUENCE {int-cksum  BIT STRING,
///                         data       BIT STRING
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Wrap_Body {
    pub int_cksum: BIT_STRING,
    pub data: BIT_STRING,
}
impl Wrap_Body {
    pub fn new(int_cksum: BIT_STRING, data: BIT_STRING) -> Self {
        Wrap_Body { int_cksum, data }
    }
}
impl TryFrom<&X690Element> for Wrap_Body {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Wrap_Body(el)
    }
}

pub const _rctl1_components_for_Wrap_Body: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "int-cksum",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "data",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Wrap_Body: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Wrap_Body: &[ComponentSpec; 0] = &[];

pub fn _decode_Wrap_Body(el: &X690Element) -> ASN1Result<Wrap_Body> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Body")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Wrap_Body,
        _eal_components_for_Wrap_Body,
        _rctl2_components_for_Wrap_Body,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut int_cksum_: OPTIONAL<BIT_STRING> = None;
    let mut data_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "int-cksum" => int_cksum_ = Some(BER.decode_bit_string(_el)?),
            "data" => data_ = Some(BER.decode_bit_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Body"))
            }
        }
    }
    Ok(Wrap_Body {
        int_cksum: int_cksum_.unwrap(),
        data: data_.unwrap(),
    })
}

pub fn _encode_Wrap_Body(value_: &Wrap_Body) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_bit_string(&value_.int_cksum)?);
    components_.push(BER.encode_bit_string(&value_.data)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Wrap_Body(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Body")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Wrap_Body,
        _eal_components_for_Wrap_Body,
        _rctl2_components_for_Wrap_Body,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "int-cksum" => BER.validate_bit_string(_el)?,
            "data" => BER.validate_bit_string(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Wrap-Body"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Conf-Alg  ::=  CHOICE {
///   algId  [0]  AlgorithmIdentifier{{SupportedAlgorithms}},
///   null   [1]  NULL
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Conf_Alg {
    algId(AlgorithmIdentifier),
    null(NULL),
}

impl TryFrom<&X690Element> for Conf_Alg {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Conf_Alg(el)
    }
}

pub fn _decode_Conf_Alg(el: &X690Element) -> ASN1Result<Conf_Alg> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(Conf_Alg::algId(_decode_AlgorithmIdentifier(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(Conf_Alg::null(BER.decode_null(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Conf-Alg",
            ))
        }
    }
}

pub fn _encode_Conf_Alg(value_: &Conf_Alg) -> ASN1Result<X690Element> {
    match value_ {
        Conf_Alg::algId(v) => |v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        Conf_Alg::null(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_null(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_Conf_Alg(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "algId"));
            }
            Ok(_validate_AlgorithmIdentifier(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "null"));
            }
            Ok(BER.validate_null(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Conf-Alg",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-DEL ::= SEQUENCE {del-header  Del-Header,
///                        int-cksum   BIT STRING
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SPKM_DEL {
    pub del_header: Del_Header,
    pub int_cksum: BIT_STRING,
}
impl SPKM_DEL {
    pub fn new(del_header: Del_Header, int_cksum: BIT_STRING) -> Self {
        SPKM_DEL {
            del_header,
            int_cksum,
        }
    }
}
impl TryFrom<&X690Element> for SPKM_DEL {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_DEL(el)
    }
}

pub const _rctl1_components_for_SPKM_DEL: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "del-header",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "int-cksum",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SPKM_DEL: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SPKM_DEL: &[ComponentSpec; 0] = &[];

pub fn _decode_SPKM_DEL(el: &X690Element) -> ASN1Result<SPKM_DEL> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-DEL")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_DEL,
        _eal_components_for_SPKM_DEL,
        _rctl2_components_for_SPKM_DEL,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut del_header_: OPTIONAL<Del_Header> = None;
    let mut int_cksum_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "del-header" => del_header_ = Some(_decode_Del_Header(_el)?),
            "int-cksum" => int_cksum_ = Some(BER.decode_bit_string(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-DEL")),
        }
    }
    Ok(SPKM_DEL {
        del_header: del_header_.unwrap(),
        int_cksum: int_cksum_.unwrap(),
    })
}

pub fn _encode_SPKM_DEL(value_: &SPKM_DEL) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Del_Header(&value_.del_header)?);
    components_.push(BER.encode_bit_string(&value_.int_cksum)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SPKM_DEL(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-DEL")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SPKM_DEL,
        _eal_components_for_SPKM_DEL,
        _rctl2_components_for_SPKM_DEL,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "del-header" => _validate_Del_Header(_el)?,
            "int-cksum" => BER.validate_bit_string(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SPKM-DEL")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Del-Header ::= SEQUENCE {
///   tok-id      INTEGER(769), -- shall contain 0301 (hex)
///   context-id  Random-Integer,
///   int-alg     [0]  AlgorithmIdentifier{{SupportedAlgorithms}} OPTIONAL,
///   snd-seq     [1]  SeqNum OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Del_Header {
    pub tok_id: INTEGER,
    pub context_id: Random_Integer,
    pub int_alg: OPTIONAL<AlgorithmIdentifier>,
    pub snd_seq: OPTIONAL<SeqNum>,
}
impl Del_Header {
    pub fn new(
        tok_id: INTEGER,
        context_id: Random_Integer,
        int_alg: OPTIONAL<AlgorithmIdentifier>,
        snd_seq: OPTIONAL<SeqNum>,
    ) -> Self {
        Del_Header {
            tok_id,
            context_id,
            int_alg,
            snd_seq,
        }
    }
}
impl TryFrom<&X690Element> for Del_Header {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Del_Header(el)
    }
}

pub const _rctl1_components_for_Del_Header: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "tok-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "context-id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "int-alg",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "snd-seq",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Del_Header: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Del_Header: &[ComponentSpec; 0] = &[];

pub fn _decode_Del_Header(el: &X690Element) -> ASN1Result<Del_Header> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Del-Header")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Del_Header,
        _eal_components_for_Del_Header,
        _rctl2_components_for_Del_Header,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tok_id_: OPTIONAL<INTEGER> = None;
    let mut context_id_: OPTIONAL<Random_Integer> = None;
    let mut int_alg_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut snd_seq_: OPTIONAL<SeqNum> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => tok_id_ = Some(BER.decode_integer(_el)?),
            "context-id" => context_id_ = Some(_decode_Random_Integer(_el)?),
            "int-alg" => int_alg_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "snd-seq" => snd_seq_ = Some(_decode_SeqNum(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Del-Header"))
            }
        }
    }
    Ok(Del_Header {
        tok_id: tok_id_.unwrap(),
        context_id: context_id_.unwrap(),
        int_alg: int_alg_,
        snd_seq: snd_seq_,
    })
}

pub fn _encode_Del_Header(value_: &Del_Header) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(BER.encode_integer(&value_.tok_id)?);
    components_.push(_encode_Random_Integer(&value_.context_id)?);
    if let Some(v_) = &value_.int_alg {
        components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.snd_seq {
        components_.push(|v_1: &SeqNum| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SeqNum(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Del_Header(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Del-Header")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Del_Header,
        _eal_components_for_Del_Header,
        _rctl2_components_for_Del_Header,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tok-id" => BER.validate_integer(_el)?,
            "context-id" => _validate_Random_Integer(_el)?,
            "int-alg" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "int-alg")
                    );
                }
                Ok(_validate_AlgorithmIdentifier(&el)?)
            }(_el)?,
            "snd-seq" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "snd-seq")
                    );
                }
                Ok(_validate_SeqNum(&el)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Del-Header"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MechType  ::=  OBJECT IDENTIFIER
/// ```
pub type MechType = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_MechType(el: &X690Element) -> ASN1Result<MechType> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_MechType(value_: &MechType) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_MechType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitialContextToken ::= [APPLICATION 0] IMPLICIT SEQUENCE {
///   thisMech           MechType,
///   innerContextToken  SPKMInnerContextToken
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct InitialContextToken {
    pub thisMech: MechType,
    pub innerContextToken: SPKMInnerContextToken,
}
impl InitialContextToken {
    pub fn new(thisMech: MechType, innerContextToken: SPKMInnerContextToken) -> Self {
        InitialContextToken {
            thisMech,
            innerContextToken,
        }
    }
}
impl TryFrom<&X690Element> for InitialContextToken {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_InitialContextToken(el)
    }
}

pub const _rctl1_components_for_InitialContextToken: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "thisMech",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("innerContextToken", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_InitialContextToken: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InitialContextToken: &[ComponentSpec; 0] = &[];

pub fn _decode_InitialContextToken(el: &X690Element) -> ASN1Result<InitialContextToken> {
    |el: &X690Element| -> ASN1Result<InitialContextToken> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => {
                return Err(el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitialContextToken"))
            }
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_InitialContextToken,
            _eal_components_for_InitialContextToken,
            _rctl2_components_for_InitialContextToken,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut thisMech_: OPTIONAL<MechType> = None;
        let mut innerContextToken_: OPTIONAL<SPKMInnerContextToken> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "thisMech" => thisMech_ = Some(_decode_MechType(_el)?),
                "innerContextToken" => {
                    innerContextToken_ = Some(_decode_SPKMInnerContextToken(_el)?)
                }
                _ => {
                    return Err(_el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "InitialContextToken",
                    ))
                }
            }
        }
        Ok(InitialContextToken {
            thisMech: thisMech_.unwrap(),
            innerContextToken: innerContextToken_.unwrap(),
        })
    }(&el)
}

pub fn _encode_InitialContextToken(value_: &InitialContextToken) -> ASN1Result<X690Element> {
    |v_1: &InitialContextToken| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &InitialContextToken| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(7);
            components_.push(_encode_MechType(&value_.thisMech)?);
            components_.push(_encode_SPKMInnerContextToken(&value_.innerContextToken)?);
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_InitialContextToken(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 0 {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitialContextToken")
            );
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "InitialContextToken",
                    ))
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_InitialContextToken,
                _eal_components_for_InitialContextToken,
                _rctl2_components_for_InitialContextToken,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "thisMech" => _validate_MechType(_el)?,
                    "innerContextToken" => _validate_SPKMInnerContextToken(_el)?,
                    _ => {
                        return Err(_el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "InitialContextToken",
                        ))
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKMInnerContextToken  ::=  CHOICE {
///   req     [0]  SPKM-REQ,
///   rep-ti  [1]  SPKM-REP-TI,
///   rep-it  [2]  SPKM-REP-IT,
///   error   [3]  SPKM-ERROR,
///   mic     [4]  SPKM-MIC,
///   wrap    [5]  SPKM-WRAP,
///   del     [6]  SPKM-DEL
/// }
/// ```
#[derive(Debug, Clone)]
pub enum SPKMInnerContextToken {
    req(SPKM_REQ),
    rep_ti(SPKM_REP_TI),
    rep_it(SPKM_REP_IT),
    error(SPKM_ERROR),
    mic(SPKM_MIC),
    wrap(SPKM_WRAP),
    del(SPKM_DEL),
}

impl TryFrom<&X690Element> for SPKMInnerContextToken {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SPKMInnerContextToken(el)
    }
}

pub fn _decode_SPKMInnerContextToken(el: &X690Element) -> ASN1Result<SPKMInnerContextToken> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(SPKMInnerContextToken::req(_decode_SPKM_REQ(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(SPKMInnerContextToken::rep_ti(_decode_SPKM_REP_TI(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(SPKMInnerContextToken::rep_it(_decode_SPKM_REP_IT(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(SPKMInnerContextToken::error(_decode_SPKM_ERROR(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(SPKMInnerContextToken::mic(_decode_SPKM_MIC(&el)?)),
        (TagClass::CONTEXT, 5) => Ok(SPKMInnerContextToken::wrap(_decode_SPKM_WRAP(&el)?)),
        (TagClass::CONTEXT, 6) => Ok(SPKMInnerContextToken::del(_decode_SPKM_DEL(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "SPKMInnerContextToken",
            ))
        }
    }
}

pub fn _encode_SPKMInnerContextToken(value_: &SPKMInnerContextToken) -> ASN1Result<X690Element> {
    match value_ {
        SPKMInnerContextToken::req(v) => |v_1: &SPKM_REQ| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_REQ(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        SPKMInnerContextToken::rep_ti(v) => |v_1: &SPKM_REP_TI| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_REP_TI(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        SPKMInnerContextToken::rep_it(v) => |v_1: &SPKM_REP_IT| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_REP_IT(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        SPKMInnerContextToken::error(v) => |v_1: &SPKM_ERROR| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_ERROR(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        SPKMInnerContextToken::mic(v) => |v_1: &SPKM_MIC| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_MIC(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 4;
            Ok(el_1)
        }(&v),
        SPKMInnerContextToken::wrap(v) => |v_1: &SPKM_WRAP| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_WRAP(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v),
        SPKMInnerContextToken::del(v) => |v_1: &SPKM_DEL| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SPKM_DEL(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 6;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_SPKMInnerContextToken(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "req"));
            }
            Ok(_validate_SPKM_REQ(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rep-ti"));
            }
            Ok(_validate_SPKM_REP_TI(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rep-it"));
            }
            Ok(_validate_SPKM_REP_IT(&el)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "error"));
            }
            Ok(_validate_SPKM_ERROR(&el)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mic"));
            }
            Ok(_validate_SPKM_MIC(&el)?)
        }(&el),
        (TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "wrap"));
            }
            Ok(_validate_SPKM_WRAP(&el)?)
        }(&el),
        (TagClass::CONTEXT, 6) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "del"));
            }
            Ok(_validate_SPKM_DEL(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "SPKMInnerContextToken",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthorizationData  ::=
///   SEQUENCE OF SEQUENCE {ad-type  INTEGER,
///                         ad-data  OCTET STRING}
/// ```
pub type AuthorizationData = Vec<AuthorizationData_Item>; // SequenceOfType

pub fn _decode_AuthorizationData(el: &X690Element) -> ASN1Result<AuthorizationData> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthorizationData")
            )
        }
    };
    let mut items: SEQUENCE_OF<AuthorizationData_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AuthorizationData_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_AuthorizationData(value_: &AuthorizationData) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AuthorizationData_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AuthorizationData(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AuthorizationData_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthorizationData")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// md5-DES-CBC OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    integrity(3) md5-DES-CBC(1)}
/// ```
///
///
#[inline]
pub fn md5_DES_CBC () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* dod */ 6,/* internet */ 1,/* security */ 5,/* integrity */ 3,/* md5-DES-CBC */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sum64-DES-CBC OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    integrity(3) sum64-DES-CBC(2)}
/// ```
///
///
#[inline]
pub fn sum64_DES_CBC () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* dod */ 6,/* internet */ 1,/* security */ 5,/* integrity */ 3,/* sum64-DES-CBC */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// spkm-1 OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    mechanisms(5) spkm(1) spkm-1(1)}
/// ```
///
///
#[inline]
pub fn spkm_1 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* dod */ 6,/* internet */ 1,/* security */ 5,/* mechanisms */ 5,/* spkm */ 1,/* spkm-1 */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// spkm-2 OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    mechanisms(5) spkm(1) spkm-2(2)}
/// ```
///
///
#[inline]
pub fn spkm_2 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* dod */ 6,/* internet */ 1,/* security */ 5,/* mechanisms */ 5,/* spkm */ 1,/* spkm-2 */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthorizationData-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct AuthorizationData_Item {
    pub ad_type: INTEGER,
    pub ad_data: OCTET_STRING,
}
impl AuthorizationData_Item {
    pub fn new(ad_type: INTEGER, ad_data: OCTET_STRING) -> Self {
        AuthorizationData_Item { ad_type, ad_data }
    }
}
impl TryFrom<&X690Element> for AuthorizationData_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AuthorizationData_Item(el)
    }
}

pub const _rctl1_components_for_AuthorizationData_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "ad-type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ad-data",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AuthorizationData_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AuthorizationData_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_AuthorizationData_Item(el: &X690Element) -> ASN1Result<AuthorizationData_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthorizationData-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthorizationData_Item,
        _eal_components_for_AuthorizationData_Item,
        _rctl2_components_for_AuthorizationData_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut ad_type_: OPTIONAL<INTEGER> = None;
    let mut ad_data_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ad-type" => ad_type_ = Some(BER.decode_integer(_el)?),
            "ad-data" => ad_data_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "AuthorizationData-Item",
                ))
            }
        }
    }
    Ok(AuthorizationData_Item {
        ad_type: ad_type_.unwrap(),
        ad_data: ad_data_.unwrap(),
    })
}

pub fn _encode_AuthorizationData_Item(value_: &AuthorizationData_Item) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.ad_type)?);
    components_.push(BER.encode_octet_string(&value_.ad_data)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_AuthorizationData_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthorizationData-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthorizationData_Item,
        _eal_components_for_AuthorizationData_Item,
        _rctl2_components_for_AuthorizationData_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ad-type" => BER.validate_integer(_el)?,
            "ad-data" => BER.validate_octet_string(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "AuthorizationData-Item",
                ))
            }
        }
    }
    Ok(())
}
