#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CryptographicMessageSyntax
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CryptographicMessageSyntax`.
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
use crate::AlgorithmInformation_2009::*;
use crate::CryptographicMessageSyntaxAlgorithms_2009::{
    KeyTransportAlgs,
    KeyAgreePublicKeys,
    KeyAgreementAlgs,
    SignatureAlgs,
    MessageDigestAlgs,
    KeyWrapAlgs,
    ContentEncryptionAlgs,
    MessageAuthAlgs,
};
use crate::CMSProfileAttributes::CMSProfileAttributes;
use crate::TokenizationManifest::tokenizedParts;
use crate::AttributeCertificateVersion1_2009::{
    AttributeCertificateV1,
    _decode_AttributeCertificateV1,
    _encode_AttributeCertificateV1,
    _validate_AttributeCertificateV1,
};
use wildboar_asn1::*;
use std::sync::Arc;
use x500::AttributeCertificateDefinitions::{
    AttributeCertificate, _decode_AttributeCertificate, _encode_AttributeCertificate,
    _validate_AttributeCertificate,
};
use x500::AuthenticationFramework::*;
use x500::InformationFramework::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// CONTENT-TYPE ::= CLASS {
/// &id        OBJECT IDENTIFIER UNIQUE,
/// &Type      OPTIONAL
/// } WITH SYNTAX {
/// [TYPE &Type] IDENTIFIED BY &id
/// }
/// ```
///
#[derive(Debug)]
pub struct CONTENT_TYPE {
    pub id: OBJECT_IDENTIFIER,
}
impl CONTENT_TYPE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentType  ::=  CONTENT-TYPE.&id
/// ```
pub type ContentType = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_ContentType(el: &X690Element) -> ASN1Result<ContentType> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_ContentType(value_: &ContentType) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_ContentType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentInfo ::= SEQUENCE {
/// contentType	CONTENT-TYPE.&id({ContentSet}),
/// content	[0] EXPLICIT CONTENT-TYPE.&Type({ContentSet}{@contentType})}
/// ```
///
#[derive(Debug, Clone)]
pub struct ContentInfo {
    pub contentType: OBJECT_IDENTIFIER,
    pub content: X690Element,
}
impl ContentInfo {
    pub fn new(contentType: OBJECT_IDENTIFIER, content: X690Element) -> Self {
        ContentInfo {
            contentType,
            content,
        }
    }
}
impl TryFrom<&X690Element> for ContentInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ContentInfo(el)
    }
}

pub const _rctl1_components_for_ContentInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "contentType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "content",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContentInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContentInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ContentInfo(el: &X690Element) -> ASN1Result<ContentInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContentInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ContentInfo,
        _eal_components_for_ContentInfo,
        _rctl2_components_for_ContentInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut contentType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut content_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contentType" => contentType_ = Some(BER.decode_object_identifier(_el)?),
            "content" => {
                content_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContentInfo")
                )
            }
        }
    }
    Ok(ContentInfo {
        contentType: contentType_.unwrap(),
        content: content_.unwrap(),
    })
}

pub fn _encode_ContentInfo(value_: &ContentInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.contentType)?);
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(x690_identity(&v_1)?),
        ))
    }(&value_.content)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ContentInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContentInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ContentInfo,
        _eal_components_for_ContentInfo,
        _rctl2_components_for_ContentInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contentType" => BER.validate_object_identifier(_el)?,
            "content" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "content")
                    );
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContentInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentSet CONTENT-TYPE ::= {
/// --  Define the set of content types to be recognized.
/// ct-Data | ct-SignedData | ct-EncryptedData | ct-EnvelopedData |
/// ct-AuthenticatedData | ct-DigestedData, ... }
/// ```
///
///
pub fn ContentSet() -> Vec<CONTENT_TYPE> {
    Vec::from([
        ct_Data(),
        ct_SignedData(),
        ct_EncryptedData(),
        ct_EnvelopedData(),
        ct_AuthenticatedData(),
        ct_DigestedData(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedData ::= SEQUENCE {
/// version		CMSVersion,
/// digestAlgorithms	SET OF DigestAlgorithmIdentifier,
/// encapContentInfo	EncapsulatedContentInfo,
/// certificates 	[0] IMPLICIT CertificateSet OPTIONAL,
/// crls 			[1] IMPLICIT RevocationInfoChoices OPTIONAL,
/// signerInfos 	SignerInfos }
/// ```
///
#[derive(Debug, Clone)]
pub struct SignedData {
    pub version: CMSVersion,
    pub digestAlgorithms: Vec<DigestAlgorithmIdentifier>,
    pub encapContentInfo: EncapsulatedContentInfo,
    pub certificates: OPTIONAL<CertificateSet>,
    pub crls: OPTIONAL<RevocationInfoChoices>,
    pub signerInfos: SignerInfos,
}
impl SignedData {
    pub fn new(
        version: CMSVersion,
        digestAlgorithms: Vec<DigestAlgorithmIdentifier>,
        encapContentInfo: EncapsulatedContentInfo,
        certificates: OPTIONAL<CertificateSet>,
        crls: OPTIONAL<RevocationInfoChoices>,
        signerInfos: SignerInfos,
    ) -> Self {
        SignedData {
            version,
            digestAlgorithms,
            encapContentInfo,
            certificates,
            crls,
            signerInfos,
        }
    }
}
impl TryFrom<&X690Element> for SignedData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SignedData(el)
    }
}

pub const _rctl1_components_for_SignedData: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "digestAlgorithms",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encapContentInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certificates",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "crls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signerInfos",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SignedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SignedData: &[ComponentSpec; 0] = &[];

pub fn _decode_SignedData(el: &X690Element) -> ASN1Result<SignedData> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SignedData,
        _eal_components_for_SignedData,
        _rctl2_components_for_SignedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut digestAlgorithms_: OPTIONAL<Vec<DigestAlgorithmIdentifier>> = None;
    let mut encapContentInfo_: OPTIONAL<EncapsulatedContentInfo> = None;
    let mut certificates_: OPTIONAL<CertificateSet> = None;
    let mut crls_: OPTIONAL<RevocationInfoChoices> = None;
    let mut signerInfos_: OPTIONAL<SignerInfos> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "digestAlgorithms" => {
                digestAlgorithms_ = Some(
                    |el: &X690Element| -> ASN1Result<SET_OF<DigestAlgorithmIdentifier>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "digestAlgorithms",
                                ))
                            }
                        };
                        let mut items: SET_OF<DigestAlgorithmIdentifier> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_DigestAlgorithmIdentifier(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            "encapContentInfo" => encapContentInfo_ = Some(_decode_EncapsulatedContentInfo(_el)?),
            "certificates" => certificates_ = Some(_decode_CertificateSet(_el)?),
            "crls" => crls_ = Some(_decode_RevocationInfoChoices(_el)?),
            "signerInfos" => signerInfos_ = Some(_decode_SignerInfos(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedData"))
            }
        }
    }
    Ok(SignedData {
        version: version_.unwrap(),
        digestAlgorithms: digestAlgorithms_.unwrap(),
        encapContentInfo: encapContentInfo_.unwrap(),
        certificates: certificates_,
        crls: crls_,
        signerInfos: signerInfos_.unwrap(),
    })
}

pub fn _encode_SignedData(value_: &SignedData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(
        |value_: &SET_OF<DigestAlgorithmIdentifier>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_DigestAlgorithmIdentifier(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.digestAlgorithms)?,
    );
    components_.push(_encode_EncapsulatedContentInfo(&value_.encapContentInfo)?);
    if let Some(v_) = &value_.certificates {
        components_.push(|v_1: &CertificateSet| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateSet(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.crls {
        components_.push(|v_1: &RevocationInfoChoices| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_RevocationInfoChoices(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_SignerInfos(&value_.signerInfos)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SignedData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SignedData,
        _eal_components_for_SignedData,
        _rctl2_components_for_SignedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "digestAlgorithms" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_DigestAlgorithmIdentifier(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "digestAlgorithms",
                    )),
                }
            }(_el)?,
            "encapContentInfo" => _validate_EncapsulatedContentInfo(_el)?,
            "certificates" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certificates")
                    );
                }
                Ok(_validate_CertificateSet(&el)?)
            }(_el)?,
            "crls" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "crls"));
                }
                Ok(_validate_RevocationInfoChoices(&el)?)
            }(_el)?,
            "signerInfos" => _validate_SignerInfos(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedData"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignerInfos  ::=  SET OF SignerInfo
/// ```
pub type SignerInfos = Vec<SignerInfo>; // SetOfType

pub fn _decode_SignerInfos(el: &X690Element) -> ASN1Result<SignerInfos> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignerInfos")),
    };
    let mut items: SET_OF<SignerInfo> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_SignerInfo(el)?);
    }
    Ok(items)
}

pub fn _encode_SignerInfos(value_: &SignerInfos) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_SignerInfo(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SignerInfos(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_SignerInfo(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignerInfos")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncapsulatedContentInfo ::= SEQUENCE {
/// eContentType       CONTENT-TYPE.&id({ContentSet}),
/// eContent           [0] EXPLICIT OCTET STRING
/// (CONTAINING CONTENT-TYPE.&Type({ContentSet}{@eContentType})) OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct EncapsulatedContentInfo {
    pub eContentType: OBJECT_IDENTIFIER,
    pub eContent: OPTIONAL<OCTET_STRING>,
}
impl EncapsulatedContentInfo {
    pub fn new(eContentType: OBJECT_IDENTIFIER, eContent: OPTIONAL<OCTET_STRING>) -> Self {
        EncapsulatedContentInfo {
            eContentType,
            eContent,
        }
    }
}
impl TryFrom<&X690Element> for EncapsulatedContentInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EncapsulatedContentInfo(el)
    }
}

pub const _rctl1_components_for_EncapsulatedContentInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "eContentType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "eContent",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncapsulatedContentInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncapsulatedContentInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EncapsulatedContentInfo(el: &X690Element) -> ASN1Result<EncapsulatedContentInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncapsulatedContentInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncapsulatedContentInfo,
        _eal_components_for_EncapsulatedContentInfo,
        _rctl2_components_for_EncapsulatedContentInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut eContentType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut eContent_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "eContentType" => eContentType_ = Some(BER.decode_object_identifier(_el)?),
            "eContent" => {
                eContent_ = Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                    Ok(BER.decode_octet_string(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncapsulatedContentInfo",
                ))
            }
        }
    }
    Ok(EncapsulatedContentInfo {
        eContentType: eContentType_.unwrap(),
        eContent: eContent_,
    })
}

pub fn _encode_EncapsulatedContentInfo(
    value_: &EncapsulatedContentInfo,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.eContentType)?);
    if let Some(v_) = &value_.eContent {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(BER.encode_octet_string(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EncapsulatedContentInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncapsulatedContentInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncapsulatedContentInfo,
        _eal_components_for_EncapsulatedContentInfo,
        _rctl2_components_for_EncapsulatedContentInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "eContentType" => BER.validate_object_identifier(_el)?,
            "eContent" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "eContent")
                    );
                }
                Ok(BER.validate_octet_string(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncapsulatedContentInfo",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignerInfo ::= SEQUENCE {
/// version 		CMSVersion,
/// sid 			SignerIdentifier,
/// digestAlgorithm 	DigestAlgorithmIdentifier,
/// signedAttrs 	[0] IMPLICIT SignedAttributes OPTIONAL,
/// signatureAlgorithm SignatureAlgorithmIdentifier,
/// signature 		SignatureValue,
/// unsignedAttrs 	[1] IMPLICIT Attributes{{UnsignedAttributes}} OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct SignerInfo {
    pub version: CMSVersion,
    pub sid: SignerIdentifier,
    pub digestAlgorithm: DigestAlgorithmIdentifier,
    pub signedAttrs: OPTIONAL<SignedAttributes>,
    pub signatureAlgorithm: SignatureAlgorithmIdentifier,
    pub signature: SignatureValue,
    pub unsignedAttrs: OPTIONAL<Attributes>,
}
impl SignerInfo {
    pub fn new(
        version: CMSVersion,
        sid: SignerIdentifier,
        digestAlgorithm: DigestAlgorithmIdentifier,
        signedAttrs: OPTIONAL<SignedAttributes>,
        signatureAlgorithm: SignatureAlgorithmIdentifier,
        signature: SignatureValue,
        unsignedAttrs: OPTIONAL<Attributes>,
    ) -> Self {
        SignerInfo {
            version,
            sid,
            digestAlgorithm,
            signedAttrs,
            signatureAlgorithm,
            signature,
            unsignedAttrs,
        }
    }
}
impl TryFrom<&X690Element> for SignerInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SignerInfo(el)
    }
}

pub const _rctl1_components_for_SignerInfo: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("sid", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "digestAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signedAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
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
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "unsignedAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SignerInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SignerInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_SignerInfo(el: &X690Element) -> ASN1Result<SignerInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignerInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SignerInfo,
        _eal_components_for_SignerInfo,
        _rctl2_components_for_SignerInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut sid_: OPTIONAL<SignerIdentifier> = None;
    let mut digestAlgorithm_: OPTIONAL<DigestAlgorithmIdentifier> = None;
    let mut signedAttrs_: OPTIONAL<SignedAttributes> = None;
    let mut signatureAlgorithm_: OPTIONAL<SignatureAlgorithmIdentifier> = None;
    let mut signature_: OPTIONAL<SignatureValue> = None;
    let mut unsignedAttrs_: OPTIONAL<Attributes> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "sid" => sid_ = Some(_decode_SignerIdentifier(_el)?),
            "digestAlgorithm" => digestAlgorithm_ = Some(_decode_DigestAlgorithmIdentifier(_el)?),
            "signedAttrs" => signedAttrs_ = Some(_decode_SignedAttributes(_el)?),
            "signatureAlgorithm" => {
                signatureAlgorithm_ = Some(_decode_SignatureAlgorithmIdentifier(_el)?)
            }
            "signature" => signature_ = Some(_decode_SignatureValue(_el)?),
            "unsignedAttrs" => unsignedAttrs_ = Some(_decode_Attributes(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignerInfo"))
            }
        }
    }
    Ok(SignerInfo {
        version: version_.unwrap(),
        sid: sid_.unwrap(),
        digestAlgorithm: digestAlgorithm_.unwrap(),
        signedAttrs: signedAttrs_,
        signatureAlgorithm: signatureAlgorithm_.unwrap(),
        signature: signature_.unwrap(),
        unsignedAttrs: unsignedAttrs_,
    })
}

pub fn _encode_SignerInfo(value_: &SignerInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(_encode_SignerIdentifier(&value_.sid)?);
    components_.push(_encode_DigestAlgorithmIdentifier(&value_.digestAlgorithm)?);
    if let Some(v_) = &value_.signedAttrs {
        components_.push(|v_1: &SignedAttributes| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SignedAttributes(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_SignatureAlgorithmIdentifier(
        &value_.signatureAlgorithm,
    )?);
    components_.push(_encode_SignatureValue(&value_.signature)?);
    if let Some(v_) = &value_.unsignedAttrs {
        components_.push(|v_1: &Attributes| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Attributes(&v_1)?;
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

pub fn _validate_SignerInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignerInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SignerInfo,
        _eal_components_for_SignerInfo,
        _rctl2_components_for_SignerInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "sid" => _validate_SignerIdentifier(_el)?,
            "digestAlgorithm" => _validate_DigestAlgorithmIdentifier(_el)?,
            "signedAttrs" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "signedAttrs")
                    );
                }
                Ok(_validate_SignedAttributes(&el)?)
            }(_el)?,
            "signatureAlgorithm" => _validate_SignatureAlgorithmIdentifier(_el)?,
            "signature" => _validate_SignatureValue(_el)?,
            "unsignedAttrs" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "unsignedAttrs")
                    );
                }
                Ok(_validate_Attributes(&el)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignerInfo"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedAttributes  ::=  Attributes {{ SignedAttributesSet }}
/// ```
pub type SignedAttributes = Attributes; // DefinedType

pub fn _decode_SignedAttributes(el: &X690Element) -> ASN1Result<SignedAttributes> {
    _decode_Attributes(&el)
}

pub fn _encode_SignedAttributes(value_: &SignedAttributes) -> ASN1Result<X690Element> {
    _encode_Attributes(&value_)
}

pub fn _validate_SignedAttributes(el: &X690Element) -> ASN1Result<()> {
    _validate_Attributes(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignerIdentifier  ::=  CHOICE {
/// issuerAndSerialNumber 	IssuerAndSerialNumber,
/// ...,
/// [[3: subjectKeyIdentifier [0] SubjectKeyIdentifier ]] }
/// ```
#[derive(Debug, Clone)]
pub enum SignerIdentifier {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    subjectKeyIdentifier(SubjectKeyIdentifier),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for SignerIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SignerIdentifier(el)
    }
}

pub fn _decode_SignerIdentifier(el: &X690Element) -> ASN1Result<SignerIdentifier> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(SignerIdentifier::issuerAndSerialNumber(
            _decode_IssuerAndSerialNumber(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(SignerIdentifier::subjectKeyIdentifier(
            _decode_SubjectKeyIdentifier(&el)?,
        )),
        _ => Ok(SignerIdentifier::_unrecognized(el.clone())),
    }
}

pub fn _encode_SignerIdentifier(value_: &SignerIdentifier) -> ASN1Result<X690Element> {
    match value_ {
        SignerIdentifier::issuerAndSerialNumber(v) => _encode_IssuerAndSerialNumber(&v),
        SignerIdentifier::subjectKeyIdentifier(v) => {
            |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        SignerIdentifier::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_SignerIdentifier(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_IssuerAndSerialNumber(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "subjectKeyIdentifier",
                ));
            }
            Ok(_validate_SubjectKeyIdentifier(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedAttributesSet ATTRIBUTE ::= { aa-signingTime | aa-messageDigest | aa-contentType |
///   CMSProfileAttributes | tokenizedParts, ... }
/// ```
///
///
pub fn SignedAttributesSet() -> Vec<ATTRIBUTE> {
    [
        CMSProfileAttributes().as_slice(),
        Vec::from([
            aa_signingTime(),
            aa_messageDigest(),
            aa_contentType(),
            tokenizedParts(),
        ])
        .as_slice(),
    ]
    .concat()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnsignedAttributes ATTRIBUTE ::= { aa-countersignature, ... }
/// ```
///
///
pub fn UnsignedAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([aa_countersignature()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureValue  ::=  OCTET STRING
/// ```
pub type SignatureValue = OCTET_STRING; // OctetStringType

pub fn _decode_SignatureValue(el: &X690Element) -> ASN1Result<SignatureValue> {
    BER.decode_octet_string(&el)
}

pub fn _encode_SignatureValue(value_: &SignatureValue) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_SignatureValue(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnvelopedData ::= SEQUENCE {
/// version 			CMSVersion,
/// originatorInfo 		[0] IMPLICIT OriginatorInfo OPTIONAL,
/// recipientInfos 		RecipientInfos,
/// encryptedContentInfo 	EncryptedContentInfo,
/// ...,
/// [[2: unprotectedAttrs 	[1] IMPLICIT Attributes
/// 			{{ UnprotectedEnvAttributes }} OPTIONAL ]] }
/// ```
///
#[derive(Debug, Clone)]
pub struct EnvelopedData {
    pub version: CMSVersion,
    pub originatorInfo: OPTIONAL<OriginatorInfo>,
    pub recipientInfos: RecipientInfos,
    pub encryptedContentInfo: EncryptedContentInfo,
    pub _unrecognized: Vec<X690Element>,
}
impl EnvelopedData {
    pub fn new(
        version: CMSVersion,
        originatorInfo: OPTIONAL<OriginatorInfo>,
        recipientInfos: RecipientInfos,
        encryptedContentInfo: EncryptedContentInfo,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EnvelopedData {
            version,
            originatorInfo,
            recipientInfos,
            encryptedContentInfo,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for EnvelopedData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EnvelopedData(el)
    }
}

pub const _rctl1_components_for_EnvelopedData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "originatorInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "recipientInfos",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedContentInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EnvelopedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EnvelopedData: &[ComponentSpec; 0] = &[];

pub fn _decode_EnvelopedData(el: &X690Element) -> ASN1Result<EnvelopedData> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EnvelopedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EnvelopedData,
        _eal_components_for_EnvelopedData,
        _rctl2_components_for_EnvelopedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut originatorInfo_: OPTIONAL<OriginatorInfo> = None;
    let mut recipientInfos_: OPTIONAL<RecipientInfos> = None;
    let mut encryptedContentInfo_: OPTIONAL<EncryptedContentInfo> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "originatorInfo" => originatorInfo_ = Some(_decode_OriginatorInfo(_el)?),
            "recipientInfos" => recipientInfos_ = Some(_decode_RecipientInfos(_el)?),
            "encryptedContentInfo" => {
                encryptedContentInfo_ = Some(_decode_EncryptedContentInfo(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EnvelopedData {
        version: version_.unwrap(),
        originatorInfo: originatorInfo_,
        recipientInfos: recipientInfos_.unwrap(),
        encryptedContentInfo: encryptedContentInfo_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_EnvelopedData(value_: &EnvelopedData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(_encode_CMSVersion(&value_.version)?);
    if let Some(v_) = &value_.originatorInfo {
        components_.push(|v_1: &OriginatorInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OriginatorInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_RecipientInfos(&value_.recipientInfos)?);
    components_.push(_encode_EncryptedContentInfo(&value_.encryptedContentInfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EnvelopedData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EnvelopedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EnvelopedData,
        _eal_components_for_EnvelopedData,
        _rctl2_components_for_EnvelopedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "originatorInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "originatorInfo")
                    );
                }
                Ok(_validate_OriginatorInfo(&el)?)
            }(_el)?,
            "recipientInfos" => _validate_RecipientInfos(_el)?,
            "encryptedContentInfo" => _validate_EncryptedContentInfo(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OriginatorInfo ::= SEQUENCE {
/// certs 	[0] IMPLICIT CertificateSet OPTIONAL,
/// crls 		[1] IMPLICIT RevocationInfoChoices OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct OriginatorInfo {
    pub certs: OPTIONAL<CertificateSet>,
    pub crls: OPTIONAL<RevocationInfoChoices>,
}
impl OriginatorInfo {
    pub fn new(certs: OPTIONAL<CertificateSet>, crls: OPTIONAL<RevocationInfoChoices>) -> Self {
        OriginatorInfo { certs, crls }
    }
}
impl Default for OriginatorInfo {
    fn default() -> Self {
        OriginatorInfo {
            certs: None,
            crls: None,
        }
    }
}
impl TryFrom<&X690Element> for OriginatorInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorInfo(el)
    }
}

pub const _rctl1_components_for_OriginatorInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "certs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "crls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OriginatorInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OriginatorInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_OriginatorInfo(el: &X690Element) -> ASN1Result<OriginatorInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorInfo"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OriginatorInfo,
        _eal_components_for_OriginatorInfo,
        _rctl2_components_for_OriginatorInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut certs_: OPTIONAL<CertificateSet> = None;
    let mut crls_: OPTIONAL<RevocationInfoChoices> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certs" => certs_ = Some(_decode_CertificateSet(_el)?),
            "crls" => crls_ = Some(_decode_RevocationInfoChoices(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorInfo")
                )
            }
        }
    }
    Ok(OriginatorInfo {
        certs: certs_,
        crls: crls_,
    })
}

pub fn _encode_OriginatorInfo(value_: &OriginatorInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.certs {
        components_.push(|v_1: &CertificateSet| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateSet(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.crls {
        components_.push(|v_1: &RevocationInfoChoices| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_RevocationInfoChoices(&v_1)?;
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

pub fn _validate_OriginatorInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorInfo"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OriginatorInfo,
        _eal_components_for_OriginatorInfo,
        _rctl2_components_for_OriginatorInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certs" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs"));
                }
                Ok(_validate_CertificateSet(&el)?)
            }(_el)?,
            "crls" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "crls"));
                }
                Ok(_validate_RevocationInfoChoices(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientInfos  ::=  SET SIZE (1..MAX) OF RecipientInfo
/// ```
pub type RecipientInfos = Vec<RecipientInfo>; // SetOfType

pub fn _decode_RecipientInfos(el: &X690Element) -> ASN1Result<RecipientInfos> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RecipientInfos"))
        }
    };
    let mut items: SET_OF<RecipientInfo> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RecipientInfo(el)?);
    }
    Ok(items)
}

pub fn _encode_RecipientInfos(value_: &RecipientInfos) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RecipientInfo(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RecipientInfos(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RecipientInfo(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RecipientInfos")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedContentInfo  ::=
/// EncryptedContentInfoType { ContentEncryptionAlgorithmIdentifier }
/// ```
pub type EncryptedContentInfo = EncryptedContentInfoType<ContentEncryptionAlgorithmIdentifier>; // DefinedType

pub fn _decode_EncryptedContentInfo(el: &X690Element) -> ASN1Result<EncryptedContentInfo> {
    _decode_EncryptedContentInfoType::<ContentEncryptionAlgorithmIdentifier>(
        _decode_ContentEncryptionAlgorithmIdentifier,
        el,
    )
}

pub fn _encode_EncryptedContentInfo(value_: &EncryptedContentInfo) -> ASN1Result<X690Element> {
    _encode_EncryptedContentInfoType::<ContentEncryptionAlgorithmIdentifier>(
        _encode_ContentEncryptionAlgorithmIdentifier,
        value_,
    )
}

pub fn _validate_EncryptedContentInfo(el: &X690Element) -> ASN1Result<()> {
    _validate_EncryptedContentInfoType::<ContentEncryptionAlgorithmIdentifier>(
        _validate_ContentEncryptionAlgorithmIdentifier,
        el,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedContentInfoType { AlgorithmIdentifierType } ::= SEQUENCE {
/// contentType        		CONTENT-TYPE.&id({ContentSet}),
/// contentEncryptionAlgorithm	AlgorithmIdentifierType,
/// encryptedContent   		[0] IMPLICIT OCTET STRING OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct EncryptedContentInfoType<AlgorithmIdentifierType> {
    pub contentType: OBJECT_IDENTIFIER,
    pub contentEncryptionAlgorithm: AlgorithmIdentifierType,
    pub encryptedContent: OPTIONAL<OCTET_STRING>,
}
impl<AlgorithmIdentifierType> EncryptedContentInfoType<AlgorithmIdentifierType> {
    pub fn new(
        contentType: OBJECT_IDENTIFIER,
        contentEncryptionAlgorithm: AlgorithmIdentifierType,
        encryptedContent: OPTIONAL<OCTET_STRING>,
    ) -> Self {
        EncryptedContentInfoType {
            contentType,
            contentEncryptionAlgorithm,
            encryptedContent,
        }
    }
}

pub const _rctl1_components_for_EncryptedContentInfoType: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "contentType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contentEncryptionAlgorithm",
        false,
        TagSelector::any,
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedContent",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncryptedContentInfoType: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedContentInfoType: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedContentInfoType<AlgorithmIdentifierType>(
    _decode_AlgorithmIdentifierType: fn(&X690Element) -> ASN1Result<AlgorithmIdentifierType>,
    el: &X690Element,
) -> ASN1Result<EncryptedContentInfoType<AlgorithmIdentifierType>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncryptedContentInfoType",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedContentInfoType,
        _eal_components_for_EncryptedContentInfoType,
        _rctl2_components_for_EncryptedContentInfoType,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut contentType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut contentEncryptionAlgorithm_: OPTIONAL<AlgorithmIdentifierType> = None;
    let mut encryptedContent_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contentType" => contentType_ = Some(BER.decode_object_identifier(_el)?),
            "contentEncryptionAlgorithm" => {
                contentEncryptionAlgorithm_ = Some(_decode_AlgorithmIdentifierType(_el)?)
            }
            "encryptedContent" => encryptedContent_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncryptedContentInfoType",
                ))
            }
        }
    }
    Ok(EncryptedContentInfoType {
        contentType: contentType_.unwrap(),
        contentEncryptionAlgorithm: contentEncryptionAlgorithm_.unwrap(),
        encryptedContent: encryptedContent_,
    })
}

pub fn _encode_EncryptedContentInfoType<AlgorithmIdentifierType>(
    _encode_AlgorithmIdentifierType: fn(&AlgorithmIdentifierType) -> ASN1Result<X690Element>,
    value_: &EncryptedContentInfoType<AlgorithmIdentifierType>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(BER.encode_object_identifier(&value_.contentType)?);
    components_.push(_encode_AlgorithmIdentifierType(
        &value_.contentEncryptionAlgorithm,
    )?);
    if let Some(v_) = &value_.encryptedContent {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EncryptedContentInfoType<AlgorithmIdentifierType>(
    _validate_AlgorithmIdentifierType: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncryptedContentInfoType",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedContentInfoType,
        _eal_components_for_EncryptedContentInfoType,
        _rctl2_components_for_EncryptedContentInfoType,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contentType" => BER.validate_object_identifier(_el)?,
            "contentEncryptionAlgorithm" => _validate_AlgorithmIdentifierType(_el)?,
            "encryptedContent" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "encryptedContent",
                    ));
                }
                Ok(BER.validate_octet_string(&el)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncryptedContentInfoType",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnprotectedEnvAttributes ATTRIBUTE ::= { ... }
/// ```
///
///
pub fn UnprotectedEnvAttributes() -> Vec<ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnprotectedEncAttributes ATTRIBUTE ::= { ... }
/// ```
///
///
pub fn UnprotectedEncAttributes() -> Vec<ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientInfo  ::=  CHOICE {
/// ktri      		KeyTransRecipientInfo,
/// ...,
/// [[3: kari  		[1] KeyAgreeRecipientInfo ]],
/// [[4: kekri 		[2] KEKRecipientInfo]],
/// [[5: pwri  		[3] PasswordRecipientInfo,
/// ori   		[4] OtherRecipientInfo ]] }
/// ```
#[derive(Debug, Clone)]
pub enum RecipientInfo {
    ktri(KeyTransRecipientInfo),
    kari(KeyAgreeRecipientInfo),
    kekri(KEKRecipientInfo),
    pwri(PasswordRecipientInfo),
    ori(OtherRecipientInfo),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for RecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientInfo(el)
    }
}

pub fn _decode_RecipientInfo(el: &X690Element) -> ASN1Result<RecipientInfo> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(RecipientInfo::ktri(_decode_KeyTransRecipientInfo(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(RecipientInfo::kari(_decode_KeyAgreeRecipientInfo(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(RecipientInfo::kekri(_decode_KEKRecipientInfo(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(RecipientInfo::pwri(_decode_PasswordRecipientInfo(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(RecipientInfo::ori(_decode_OtherRecipientInfo(&el)?)),
        _ => Ok(RecipientInfo::_unrecognized(el.clone())),
    }
}

pub fn _encode_RecipientInfo(value_: &RecipientInfo) -> ASN1Result<X690Element> {
    match value_ {
        RecipientInfo::ktri(v) => _encode_KeyTransRecipientInfo(&v),
        RecipientInfo::kari(v) => |v_1: &KeyAgreeRecipientInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_KeyAgreeRecipientInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        RecipientInfo::kekri(v) => |v_1: &KEKRecipientInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_KEKRecipientInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        RecipientInfo::pwri(v) => |v_1: &PasswordRecipientInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_PasswordRecipientInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        RecipientInfo::ori(v) => |v_1: &OtherRecipientInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OtherRecipientInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 4;
            Ok(el_1)
        }(&v),
        RecipientInfo::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_RecipientInfo(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_KeyTransRecipientInfo(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "kari"));
            }
            Ok(_validate_KeyAgreeRecipientInfo(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "kekri"));
            }
            Ok(_validate_KEKRecipientInfo(&el)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pwri"));
            }
            Ok(_validate_PasswordRecipientInfo(&el)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ori"));
            }
            Ok(_validate_OtherRecipientInfo(&el)?)
        }(&el),
        _ => Ok(()),
    }
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
/// KeyTransRecipientInfo ::= SEQUENCE {
/// version 			CMSVersion,  -- always set to 0 or 2
/// rid 				RecipientIdentifier,
/// keyEncryptionAlgorithm 	AlgorithmIdentifier
/// 		{KEY-TRANSPORT, {KeyTransportAlgorithmSet}},
/// encryptedKey 		EncryptedKey }
/// ```
///
#[derive(Debug, Clone)]
pub struct KeyTransRecipientInfo {
    pub version: CMSVersion,
    pub rid: RecipientIdentifier,
    pub keyEncryptionAlgorithm: AlgorithmIdentifier,
    pub encryptedKey: EncryptedKey,
}
impl KeyTransRecipientInfo {
    pub fn new(
        version: CMSVersion,
        rid: RecipientIdentifier,
        keyEncryptionAlgorithm: AlgorithmIdentifier,
        encryptedKey: EncryptedKey,
    ) -> Self {
        KeyTransRecipientInfo {
            version,
            rid,
            keyEncryptionAlgorithm,
            encryptedKey,
        }
    }
}
impl TryFrom<&X690Element> for KeyTransRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_KeyTransRecipientInfo(el)
    }
}

pub const _rctl1_components_for_KeyTransRecipientInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("rid", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "keyEncryptionAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_KeyTransRecipientInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_KeyTransRecipientInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_KeyTransRecipientInfo(el: &X690Element) -> ASN1Result<KeyTransRecipientInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KeyTransRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyTransRecipientInfo,
        _eal_components_for_KeyTransRecipientInfo,
        _rctl2_components_for_KeyTransRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut rid_: OPTIONAL<RecipientIdentifier> = None;
    let mut keyEncryptionAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut encryptedKey_: OPTIONAL<EncryptedKey> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "rid" => rid_ = Some(_decode_RecipientIdentifier(_el)?),
            "keyEncryptionAlgorithm" => {
                keyEncryptionAlgorithm_ = Some(_decode_AlgorithmIdentifier(_el)?)
            }
            "encryptedKey" => encryptedKey_ = Some(_decode_EncryptedKey(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "KeyTransRecipientInfo",
                ))
            }
        }
    }
    Ok(KeyTransRecipientInfo {
        version: version_.unwrap(),
        rid: rid_.unwrap(),
        keyEncryptionAlgorithm: keyEncryptionAlgorithm_.unwrap(),
        encryptedKey: encryptedKey_.unwrap(),
    })
}

pub fn _encode_KeyTransRecipientInfo(value_: &KeyTransRecipientInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(_encode_RecipientIdentifier(&value_.rid)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.keyEncryptionAlgorithm)?);
    components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_KeyTransRecipientInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KeyTransRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyTransRecipientInfo,
        _eal_components_for_KeyTransRecipientInfo,
        _rctl2_components_for_KeyTransRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "rid" => _validate_RecipientIdentifier(_el)?,
            "keyEncryptionAlgorithm" => _validate_AlgorithmIdentifier(_el)?,
            "encryptedKey" => _validate_EncryptedKey(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "KeyTransRecipientInfo",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyTransportAlgorithmSet KEY-TRANSPORT ::= { KeyTransportAlgs, ... }
/// ```
///
///
pub fn KeyTransportAlgorithmSet() -> Vec<KEY_TRANSPORT> {
    KeyTransportAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientIdentifier  ::=  CHOICE {
/// issuerAndSerialNumber 	IssuerAndSerialNumber,
/// ...,
/// [[2: subjectKeyIdentifier	[0] SubjectKeyIdentifier ]] }
/// ```
#[derive(Debug, Clone)]
pub enum RecipientIdentifier {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    subjectKeyIdentifier(SubjectKeyIdentifier),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for RecipientIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientIdentifier(el)
    }
}

pub fn _decode_RecipientIdentifier(el: &X690Element) -> ASN1Result<RecipientIdentifier> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(RecipientIdentifier::issuerAndSerialNumber(
            _decode_IssuerAndSerialNumber(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(RecipientIdentifier::subjectKeyIdentifier(
            _decode_SubjectKeyIdentifier(&el)?,
        )),
        _ => Ok(RecipientIdentifier::_unrecognized(el.clone())),
    }
}

pub fn _encode_RecipientIdentifier(value_: &RecipientIdentifier) -> ASN1Result<X690Element> {
    match value_ {
        RecipientIdentifier::issuerAndSerialNumber(v) => _encode_IssuerAndSerialNumber(&v),
        RecipientIdentifier::subjectKeyIdentifier(v) => {
            |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        RecipientIdentifier::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_RecipientIdentifier(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_IssuerAndSerialNumber(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "subjectKeyIdentifier",
                ));
            }
            Ok(_validate_SubjectKeyIdentifier(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreeRecipientInfo ::= SEQUENCE {
/// version 			CMSVersion,  -- always set to 3
/// originator 			[0] EXPLICIT OriginatorIdentifierOrKey,
/// ukm 				[1] EXPLICIT UserKeyingMaterial OPTIONAL,
/// keyEncryptionAlgorithm 	AlgorithmIdentifier
/// 		{KEY-AGREE, {KeyAgreementAlgorithmSet}},
/// recipientEncryptedKeys 	RecipientEncryptedKeys }
/// ```
///
#[derive(Debug, Clone)]
pub struct KeyAgreeRecipientInfo {
    pub version: CMSVersion,
    pub originator: OriginatorIdentifierOrKey,
    pub ukm: OPTIONAL<UserKeyingMaterial>,
    pub keyEncryptionAlgorithm: AlgorithmIdentifier,
    pub recipientEncryptedKeys: RecipientEncryptedKeys,
}
impl KeyAgreeRecipientInfo {
    pub fn new(
        version: CMSVersion,
        originator: OriginatorIdentifierOrKey,
        ukm: OPTIONAL<UserKeyingMaterial>,
        keyEncryptionAlgorithm: AlgorithmIdentifier,
        recipientEncryptedKeys: RecipientEncryptedKeys,
    ) -> Self {
        KeyAgreeRecipientInfo {
            version,
            originator,
            ukm,
            keyEncryptionAlgorithm,
            recipientEncryptedKeys,
        }
    }
}
impl TryFrom<&X690Element> for KeyAgreeRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreeRecipientInfo(el)
    }
}

pub const _rctl1_components_for_KeyAgreeRecipientInfo: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "originator",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ukm",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
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
    ComponentSpec::new(
        "recipientEncryptedKeys",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_KeyAgreeRecipientInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_KeyAgreeRecipientInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_KeyAgreeRecipientInfo(el: &X690Element) -> ASN1Result<KeyAgreeRecipientInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KeyAgreeRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyAgreeRecipientInfo,
        _eal_components_for_KeyAgreeRecipientInfo,
        _rctl2_components_for_KeyAgreeRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut originator_: OPTIONAL<OriginatorIdentifierOrKey> = None;
    let mut ukm_: OPTIONAL<UserKeyingMaterial> = None;
    let mut keyEncryptionAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut recipientEncryptedKeys_: OPTIONAL<RecipientEncryptedKeys> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "originator" => {
                originator_ = Some(
                    |el: &X690Element| -> ASN1Result<OriginatorIdentifierOrKey> {
                        Ok(_decode_OriginatorIdentifierOrKey(&el.inner()?)?)
                    }(_el)?,
                )
            }
            "ukm" => {
                ukm_ = Some(|el: &X690Element| -> ASN1Result<UserKeyingMaterial> {
                    Ok(_decode_UserKeyingMaterial(&el.inner()?)?)
                }(_el)?)
            }
            "keyEncryptionAlgorithm" => {
                keyEncryptionAlgorithm_ = Some(_decode_AlgorithmIdentifier(_el)?)
            }
            "recipientEncryptedKeys" => {
                recipientEncryptedKeys_ = Some(_decode_RecipientEncryptedKeys(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "KeyAgreeRecipientInfo",
                ))
            }
        }
    }
    Ok(KeyAgreeRecipientInfo {
        version: version_.unwrap(),
        originator: originator_.unwrap(),
        ukm: ukm_,
        keyEncryptionAlgorithm: keyEncryptionAlgorithm_.unwrap(),
        recipientEncryptedKeys: recipientEncryptedKeys_.unwrap(),
    })
}

pub fn _encode_KeyAgreeRecipientInfo(value_: &KeyAgreeRecipientInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(
        |v_1: &OriginatorIdentifierOrKey| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(_encode_OriginatorIdentifierOrKey(&v_1)?),
            ))
        }(&value_.originator)?,
    );
    if let Some(v_) = &value_.ukm {
        components_.push(|v_1: &UserKeyingMaterial| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(_encode_UserKeyingMaterial(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(_encode_AlgorithmIdentifier(&value_.keyEncryptionAlgorithm)?);
    components_.push(_encode_RecipientEncryptedKeys(
        &value_.recipientEncryptedKeys,
    )?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_KeyAgreeRecipientInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KeyAgreeRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KeyAgreeRecipientInfo,
        _eal_components_for_KeyAgreeRecipientInfo,
        _rctl2_components_for_KeyAgreeRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "originator" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "originator")
                    );
                }
                Ok(_validate_OriginatorIdentifierOrKey(&el.inner()?)?)
            }(_el)?,
            "ukm" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ukm"));
                }
                Ok(_validate_UserKeyingMaterial(&el.inner()?)?)
            }(_el)?,
            "keyEncryptionAlgorithm" => _validate_AlgorithmIdentifier(_el)?,
            "recipientEncryptedKeys" => _validate_RecipientEncryptedKeys(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "KeyAgreeRecipientInfo",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreementAlgorithmSet KEY-AGREE ::= { KeyAgreementAlgs, ... }
/// ```
///
///
pub fn KeyAgreementAlgorithmSet() -> Vec<KEY_AGREE> {
    KeyAgreementAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OriginatorIdentifierOrKey  ::=  CHOICE {
/// issuerAndSerialNumber 	IssuerAndSerialNumber,
/// subjectKeyIdentifier 	[0] SubjectKeyIdentifier,
/// originatorKey 		[1] OriginatorPublicKey }
/// ```
#[derive(Debug, Clone)]
pub enum OriginatorIdentifierOrKey {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    subjectKeyIdentifier(SubjectKeyIdentifier),
    originatorKey(OriginatorPublicKey),
}

impl TryFrom<&X690Element> for OriginatorIdentifierOrKey {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorIdentifierOrKey(el)
    }
}

pub fn _decode_OriginatorIdentifierOrKey(
    el: &X690Element,
) -> ASN1Result<OriginatorIdentifierOrKey> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(OriginatorIdentifierOrKey::issuerAndSerialNumber(
            _decode_IssuerAndSerialNumber(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(OriginatorIdentifierOrKey::subjectKeyIdentifier(
            _decode_SubjectKeyIdentifier(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(OriginatorIdentifierOrKey::originatorKey(
            _decode_OriginatorPublicKey(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OriginatorIdentifierOrKey",
            ))
        }
    }
}

pub fn _encode_OriginatorIdentifierOrKey(
    value_: &OriginatorIdentifierOrKey,
) -> ASN1Result<X690Element> {
    match value_ {
        OriginatorIdentifierOrKey::issuerAndSerialNumber(v) => _encode_IssuerAndSerialNumber(&v),
        OriginatorIdentifierOrKey::subjectKeyIdentifier(v) => {
            |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        OriginatorIdentifierOrKey::originatorKey(v) => {
            |v_1: &OriginatorPublicKey| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_OriginatorPublicKey(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_OriginatorIdentifierOrKey(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_IssuerAndSerialNumber(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "subjectKeyIdentifier",
                ));
            }
            Ok(_validate_SubjectKeyIdentifier(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "originatorKey")
                );
            }
            Ok(_validate_OriginatorPublicKey(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OriginatorIdentifierOrKey",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OriginatorPublicKey ::= SEQUENCE {
/// algorithm 	AlgorithmIdentifier {PUBLIC-KEY, {OriginatorKeySet}},
/// publicKey 	BIT STRING }
/// ```
///
#[derive(Debug, Clone)]
pub struct OriginatorPublicKey {
    pub algorithm: AlgorithmIdentifier,
    pub publicKey: BIT_STRING,
}
impl OriginatorPublicKey {
    pub fn new(algorithm: AlgorithmIdentifier, publicKey: BIT_STRING) -> Self {
        OriginatorPublicKey {
            algorithm,
            publicKey,
        }
    }
}
impl TryFrom<&X690Element> for OriginatorPublicKey {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorPublicKey(el)
    }
}

pub const _rctl1_components_for_OriginatorPublicKey: &[ComponentSpec; 2] = &[
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

pub const _rctl2_components_for_OriginatorPublicKey: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OriginatorPublicKey: &[ComponentSpec; 0] = &[];

pub fn _decode_OriginatorPublicKey(el: &X690Element) -> ASN1Result<OriginatorPublicKey> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorPublicKey")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OriginatorPublicKey,
        _eal_components_for_OriginatorPublicKey,
        _rctl2_components_for_OriginatorPublicKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut publicKey_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "publicKey" => publicKey_ = Some(BER.decode_bit_string(_el)?),
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorPublicKey"))
            }
        }
    }
    Ok(OriginatorPublicKey {
        algorithm: algorithm_.unwrap(),
        publicKey: publicKey_.unwrap(),
    })
}

pub fn _encode_OriginatorPublicKey(value_: &OriginatorPublicKey) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
    components_.push(BER.encode_bit_string(&value_.publicKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OriginatorPublicKey(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorPublicKey")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OriginatorPublicKey,
        _eal_components_for_OriginatorPublicKey,
        _rctl2_components_for_OriginatorPublicKey,
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
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OriginatorPublicKey"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OriginatorKeySet PUBLIC-KEY ::= { KeyAgreePublicKeys, ... }
/// ```
///
///
pub fn OriginatorKeySet() -> Vec<PUBLIC_KEY> {
    KeyAgreePublicKeys()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientEncryptedKeys  ::=  SEQUENCE OF RecipientEncryptedKey
/// ```
pub type RecipientEncryptedKeys = Vec<RecipientEncryptedKey>; // SequenceOfType

pub fn _decode_RecipientEncryptedKeys(el: &X690Element) -> ASN1Result<RecipientEncryptedKeys> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RecipientEncryptedKeys",
            ))
        }
    };
    let mut items: SEQUENCE_OF<RecipientEncryptedKey> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RecipientEncryptedKey(el)?);
    }
    Ok(items)
}

pub fn _encode_RecipientEncryptedKeys(value_: &RecipientEncryptedKeys) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RecipientEncryptedKey(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RecipientEncryptedKeys(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RecipientEncryptedKey(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "RecipientEncryptedKeys",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientEncryptedKey ::= SEQUENCE {
/// rid 			KeyAgreeRecipientIdentifier,
/// encryptedKey 	EncryptedKey }
/// ```
///
#[derive(Debug, Clone)]
pub struct RecipientEncryptedKey {
    pub rid: KeyAgreeRecipientIdentifier,
    pub encryptedKey: EncryptedKey,
}
impl RecipientEncryptedKey {
    pub fn new(rid: KeyAgreeRecipientIdentifier, encryptedKey: EncryptedKey) -> Self {
        RecipientEncryptedKey { rid, encryptedKey }
    }
}
impl TryFrom<&X690Element> for RecipientEncryptedKey {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientEncryptedKey(el)
    }
}

pub const _rctl1_components_for_RecipientEncryptedKey: &[ComponentSpec; 2] = &[
    ComponentSpec::new("rid", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "encryptedKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RecipientEncryptedKey: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RecipientEncryptedKey: &[ComponentSpec; 0] = &[];

pub fn _decode_RecipientEncryptedKey(el: &X690Element) -> ASN1Result<RecipientEncryptedKey> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RecipientEncryptedKey")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RecipientEncryptedKey,
        _eal_components_for_RecipientEncryptedKey,
        _rctl2_components_for_RecipientEncryptedKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut rid_: OPTIONAL<KeyAgreeRecipientIdentifier> = None;
    let mut encryptedKey_: OPTIONAL<EncryptedKey> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "rid" => rid_ = Some(_decode_KeyAgreeRecipientIdentifier(_el)?),
            "encryptedKey" => encryptedKey_ = Some(_decode_EncryptedKey(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "RecipientEncryptedKey",
                ))
            }
        }
    }
    Ok(RecipientEncryptedKey {
        rid: rid_.unwrap(),
        encryptedKey: encryptedKey_.unwrap(),
    })
}

pub fn _encode_RecipientEncryptedKey(value_: &RecipientEncryptedKey) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_KeyAgreeRecipientIdentifier(&value_.rid)?);
    components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_RecipientEncryptedKey(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RecipientEncryptedKey")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RecipientEncryptedKey,
        _eal_components_for_RecipientEncryptedKey,
        _rctl2_components_for_RecipientEncryptedKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "rid" => _validate_KeyAgreeRecipientIdentifier(_el)?,
            "encryptedKey" => _validate_EncryptedKey(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "RecipientEncryptedKey",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreeRecipientIdentifier  ::=  CHOICE {
/// issuerAndSerialNumber	IssuerAndSerialNumber,
/// rKeyId 			[0] IMPLICIT RecipientKeyIdentifier }
/// ```
#[derive(Debug, Clone)]
pub enum KeyAgreeRecipientIdentifier {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    rKeyId(RecipientKeyIdentifier),
}

impl TryFrom<&X690Element> for KeyAgreeRecipientIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreeRecipientIdentifier(el)
    }
}

pub fn _decode_KeyAgreeRecipientIdentifier(
    el: &X690Element,
) -> ASN1Result<KeyAgreeRecipientIdentifier> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(KeyAgreeRecipientIdentifier::issuerAndSerialNumber(
            _decode_IssuerAndSerialNumber(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(KeyAgreeRecipientIdentifier::rKeyId(
            _decode_RecipientKeyIdentifier(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "KeyAgreeRecipientIdentifier",
            ))
        }
    }
}

pub fn _encode_KeyAgreeRecipientIdentifier(
    value_: &KeyAgreeRecipientIdentifier,
) -> ASN1Result<X690Element> {
    match value_ {
        KeyAgreeRecipientIdentifier::issuerAndSerialNumber(v) => _encode_IssuerAndSerialNumber(&v),
        KeyAgreeRecipientIdentifier::rKeyId(v) => {
            |v_1: &RecipientKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_RecipientKeyIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_KeyAgreeRecipientIdentifier(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_IssuerAndSerialNumber(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rKeyId"));
            }
            Ok(_validate_RecipientKeyIdentifier(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "KeyAgreeRecipientIdentifier",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientKeyIdentifier ::= SEQUENCE {
/// subjectKeyIdentifier 	SubjectKeyIdentifier,
/// date 				GeneralizedTime OPTIONAL,
/// other 			OtherKeyAttribute OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct RecipientKeyIdentifier {
    pub subjectKeyIdentifier: SubjectKeyIdentifier,
    pub date: OPTIONAL<GeneralizedTime>,
    pub other: OPTIONAL<OtherKeyAttribute>,
}
impl RecipientKeyIdentifier {
    pub fn new(
        subjectKeyIdentifier: SubjectKeyIdentifier,
        date: OPTIONAL<GeneralizedTime>,
        other: OPTIONAL<OtherKeyAttribute>,
    ) -> Self {
        RecipientKeyIdentifier {
            subjectKeyIdentifier,
            date,
            other,
        }
    }
}
impl TryFrom<&X690Element> for RecipientKeyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientKeyIdentifier(el)
    }
}

pub const _rctl1_components_for_RecipientKeyIdentifier: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "subjectKeyIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "date",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "other",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RecipientKeyIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RecipientKeyIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_RecipientKeyIdentifier(el: &X690Element) -> ASN1Result<RecipientKeyIdentifier> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RecipientKeyIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RecipientKeyIdentifier,
        _eal_components_for_RecipientKeyIdentifier,
        _rctl2_components_for_RecipientKeyIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut subjectKeyIdentifier_: OPTIONAL<SubjectKeyIdentifier> = None;
    let mut date_: OPTIONAL<GeneralizedTime> = None;
    let mut other_: OPTIONAL<OtherKeyAttribute> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subjectKeyIdentifier" => {
                subjectKeyIdentifier_ = Some(_decode_SubjectKeyIdentifier(_el)?)
            }
            "date" => date_ = Some(BER.decode_generalized_time(_el)?),
            "other" => other_ = Some(_decode_OtherKeyAttribute(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "RecipientKeyIdentifier",
                ))
            }
        }
    }
    Ok(RecipientKeyIdentifier {
        subjectKeyIdentifier: subjectKeyIdentifier_.unwrap(),
        date: date_,
        other: other_,
    })
}

pub fn _encode_RecipientKeyIdentifier(value_: &RecipientKeyIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_SubjectKeyIdentifier(&value_.subjectKeyIdentifier)?);
    if let Some(v_) = &value_.date {
        components_.push(BER.encode_generalized_time(&v_)?);
    }
    if let Some(v_) = &value_.other {
        components_.push(_encode_OtherKeyAttribute(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_RecipientKeyIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RecipientKeyIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RecipientKeyIdentifier,
        _eal_components_for_RecipientKeyIdentifier,
        _rctl2_components_for_RecipientKeyIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subjectKeyIdentifier" => _validate_SubjectKeyIdentifier(_el)?,
            "date" => BER.validate_generalized_time(_el)?,
            "other" => _validate_OtherKeyAttribute(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "RecipientKeyIdentifier",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubjectKeyIdentifier  ::=  OCTET STRING
/// ```
pub type SubjectKeyIdentifier = OCTET_STRING; // OctetStringType

pub fn _decode_SubjectKeyIdentifier(el: &X690Element) -> ASN1Result<SubjectKeyIdentifier> {
    BER.decode_octet_string(&el)
}

pub fn _encode_SubjectKeyIdentifier(value_: &SubjectKeyIdentifier) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_SubjectKeyIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEKRecipientInfo ::= SEQUENCE {
/// version 			CMSVersion,  -- always set to 4
/// kekid 			KEKIdentifier,
/// keyEncryptionAlgorithm 	KeyEncryptionAlgorithmIdentifier,
/// encryptedKey 		EncryptedKey }
/// ```
///
#[derive(Debug, Clone)]
pub struct KEKRecipientInfo {
    pub version: CMSVersion,
    pub kekid: KEKIdentifier,
    pub keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
    pub encryptedKey: EncryptedKey,
}
impl KEKRecipientInfo {
    pub fn new(
        version: CMSVersion,
        kekid: KEKIdentifier,
        keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
        encryptedKey: EncryptedKey,
    ) -> Self {
        KEKRecipientInfo {
            version,
            kekid,
            keyEncryptionAlgorithm,
            encryptedKey,
        }
    }
}
impl TryFrom<&X690Element> for KEKRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_KEKRecipientInfo(el)
    }
}

pub const _rctl1_components_for_KEKRecipientInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "kekid",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
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
    ComponentSpec::new(
        "encryptedKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_KEKRecipientInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_KEKRecipientInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_KEKRecipientInfo(el: &X690Element) -> ASN1Result<KEKRecipientInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KEKRecipientInfo,
        _eal_components_for_KEKRecipientInfo,
        _rctl2_components_for_KEKRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut kekid_: OPTIONAL<KEKIdentifier> = None;
    let mut keyEncryptionAlgorithm_: OPTIONAL<KeyEncryptionAlgorithmIdentifier> = None;
    let mut encryptedKey_: OPTIONAL<EncryptedKey> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "kekid" => kekid_ = Some(_decode_KEKIdentifier(_el)?),
            "keyEncryptionAlgorithm" => {
                keyEncryptionAlgorithm_ = Some(_decode_KeyEncryptionAlgorithmIdentifier(_el)?)
            }
            "encryptedKey" => encryptedKey_ = Some(_decode_EncryptedKey(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKRecipientInfo")
                )
            }
        }
    }
    Ok(KEKRecipientInfo {
        version: version_.unwrap(),
        kekid: kekid_.unwrap(),
        keyEncryptionAlgorithm: keyEncryptionAlgorithm_.unwrap(),
        encryptedKey: encryptedKey_.unwrap(),
    })
}

pub fn _encode_KEKRecipientInfo(value_: &KEKRecipientInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(_encode_KEKIdentifier(&value_.kekid)?);
    components_.push(_encode_KeyEncryptionAlgorithmIdentifier(
        &value_.keyEncryptionAlgorithm,
    )?);
    components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_KEKRecipientInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KEKRecipientInfo,
        _eal_components_for_KEKRecipientInfo,
        _rctl2_components_for_KEKRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "kekid" => _validate_KEKIdentifier(_el)?,
            "keyEncryptionAlgorithm" => _validate_KeyEncryptionAlgorithmIdentifier(_el)?,
            "encryptedKey" => _validate_EncryptedKey(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKRecipientInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEKIdentifier ::= SEQUENCE {
/// keyIdentifier 	OCTET STRING,
/// date 			GeneralizedTime OPTIONAL,
/// other 		OtherKeyAttribute OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct KEKIdentifier {
    pub keyIdentifier: OCTET_STRING,
    pub date: OPTIONAL<GeneralizedTime>,
    pub other: OPTIONAL<OtherKeyAttribute>,
}
impl KEKIdentifier {
    pub fn new(
        keyIdentifier: OCTET_STRING,
        date: OPTIONAL<GeneralizedTime>,
        other: OPTIONAL<OtherKeyAttribute>,
    ) -> Self {
        KEKIdentifier {
            keyIdentifier,
            date,
            other,
        }
    }
}
impl TryFrom<&X690Element> for KEKIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_KEKIdentifier(el)
    }
}

pub const _rctl1_components_for_KEKIdentifier: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "keyIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "date",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "other",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_KEKIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_KEKIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_KEKIdentifier(el: &X690Element) -> ASN1Result<KEKIdentifier> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKIdentifier")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KEKIdentifier,
        _eal_components_for_KEKIdentifier,
        _rctl2_components_for_KEKIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut keyIdentifier_: OPTIONAL<OCTET_STRING> = None;
    let mut date_: OPTIONAL<GeneralizedTime> = None;
    let mut other_: OPTIONAL<OtherKeyAttribute> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyIdentifier" => keyIdentifier_ = Some(BER.decode_octet_string(_el)?),
            "date" => date_ = Some(BER.decode_generalized_time(_el)?),
            "other" => other_ = Some(_decode_OtherKeyAttribute(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKIdentifier")
                )
            }
        }
    }
    Ok(KEKIdentifier {
        keyIdentifier: keyIdentifier_.unwrap(),
        date: date_,
        other: other_,
    })
}

pub fn _encode_KEKIdentifier(value_: &KEKIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(BER.encode_octet_string(&value_.keyIdentifier)?);
    if let Some(v_) = &value_.date {
        components_.push(BER.encode_generalized_time(&v_)?);
    }
    if let Some(v_) = &value_.other {
        components_.push(_encode_OtherKeyAttribute(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_KEKIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKIdentifier")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_KEKIdentifier,
        _eal_components_for_KEKIdentifier,
        _rctl2_components_for_KEKIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyIdentifier" => BER.validate_octet_string(_el)?,
            "date" => BER.validate_generalized_time(_el)?,
            "other" => _validate_OtherKeyAttribute(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "KEKIdentifier")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PasswordRecipientInfo ::= SEQUENCE {
/// version 			CMSVersion,   -- always set to 0
/// keyDerivationAlgorithm 	[0] KeyDerivationAlgorithmIdentifier
/// 		OPTIONAL,
/// keyEncryptionAlgorithm 	KeyEncryptionAlgorithmIdentifier,
/// encryptedKey 		EncryptedKey }
/// ```
///
#[derive(Debug, Clone)]
pub struct PasswordRecipientInfo {
    pub version: CMSVersion,
    pub keyDerivationAlgorithm: OPTIONAL<KeyDerivationAlgorithmIdentifier>,
    pub keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
    pub encryptedKey: EncryptedKey,
}
impl PasswordRecipientInfo {
    pub fn new(
        version: CMSVersion,
        keyDerivationAlgorithm: OPTIONAL<KeyDerivationAlgorithmIdentifier>,
        keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
        encryptedKey: EncryptedKey,
    ) -> Self {
        PasswordRecipientInfo {
            version,
            keyDerivationAlgorithm,
            keyEncryptionAlgorithm,
            encryptedKey,
        }
    }
}
impl TryFrom<&X690Element> for PasswordRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PasswordRecipientInfo(el)
    }
}

pub const _rctl1_components_for_PasswordRecipientInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyDerivationAlgorithm",
        true,
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
    ComponentSpec::new(
        "encryptedKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PasswordRecipientInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PasswordRecipientInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_PasswordRecipientInfo(el: &X690Element) -> ASN1Result<PasswordRecipientInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PasswordRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PasswordRecipientInfo,
        _eal_components_for_PasswordRecipientInfo,
        _rctl2_components_for_PasswordRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut keyDerivationAlgorithm_: OPTIONAL<KeyDerivationAlgorithmIdentifier> = None;
    let mut keyEncryptionAlgorithm_: OPTIONAL<KeyEncryptionAlgorithmIdentifier> = None;
    let mut encryptedKey_: OPTIONAL<EncryptedKey> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "keyDerivationAlgorithm" => {
                keyDerivationAlgorithm_ = Some(_decode_KeyDerivationAlgorithmIdentifier(_el)?)
            }
            "keyEncryptionAlgorithm" => {
                keyEncryptionAlgorithm_ = Some(_decode_KeyEncryptionAlgorithmIdentifier(_el)?)
            }
            "encryptedKey" => encryptedKey_ = Some(_decode_EncryptedKey(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "PasswordRecipientInfo",
                ))
            }
        }
    }
    Ok(PasswordRecipientInfo {
        version: version_.unwrap(),
        keyDerivationAlgorithm: keyDerivationAlgorithm_,
        keyEncryptionAlgorithm: keyEncryptionAlgorithm_.unwrap(),
        encryptedKey: encryptedKey_.unwrap(),
    })
}

pub fn _encode_PasswordRecipientInfo(value_: &PasswordRecipientInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_CMSVersion(&value_.version)?);
    if let Some(v_) = &value_.keyDerivationAlgorithm {
        components_.push(
            |v_1: &KeyDerivationAlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_KeyDerivationAlgorithmIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v_)?,
        );
    }
    components_.push(_encode_KeyEncryptionAlgorithmIdentifier(
        &value_.keyEncryptionAlgorithm,
    )?);
    components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_PasswordRecipientInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PasswordRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PasswordRecipientInfo,
        _eal_components_for_PasswordRecipientInfo,
        _rctl2_components_for_PasswordRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "keyDerivationAlgorithm" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "keyDerivationAlgorithm",
                    ));
                }
                Ok(_validate_KeyDerivationAlgorithmIdentifier(&el)?)
            }(_el)?,
            "keyEncryptionAlgorithm" => _validate_KeyEncryptionAlgorithmIdentifier(_el)?,
            "encryptedKey" => _validate_EncryptedKey(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "PasswordRecipientInfo",
                ))
            }
        }
    }
    Ok(())
}

pub type OTHER_RECIPIENT = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OtherRecipientInfo ::= SEQUENCE {
/// oriType    OTHER-RECIPIENT.&id({SupportedOtherRecipInfo}),
/// oriValue   OTHER-RECIPIENT.&Type({SupportedOtherRecipInfo}{@oriType})}
/// ```
///
#[derive(Debug, Clone)]
pub struct OtherRecipientInfo {
    pub oriType: OBJECT_IDENTIFIER,
    pub oriValue: X690Element,
}
impl OtherRecipientInfo {
    pub fn new(oriType: OBJECT_IDENTIFIER, oriValue: X690Element) -> Self {
        OtherRecipientInfo { oriType, oriValue }
    }
}
impl TryFrom<&X690Element> for OtherRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OtherRecipientInfo(el)
    }
}

pub const _rctl1_components_for_OtherRecipientInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "oriType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("oriValue", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OtherRecipientInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OtherRecipientInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_OtherRecipientInfo(el: &X690Element) -> ASN1Result<OtherRecipientInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherRecipientInfo,
        _eal_components_for_OtherRecipientInfo,
        _rctl2_components_for_OtherRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut oriType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut oriValue_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "oriType" => oriType_ = Some(BER.decode_object_identifier(_el)?),
            "oriValue" => oriValue_ = Some(x690_identity(_el)?),
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherRecipientInfo"))
            }
        }
    }
    Ok(OtherRecipientInfo {
        oriType: oriType_.unwrap(),
        oriValue: oriValue_.unwrap(),
    })
}

pub fn _encode_OtherRecipientInfo(value_: &OtherRecipientInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.oriType)?);
    components_.push(x690_identity(&value_.oriValue)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OtherRecipientInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherRecipientInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherRecipientInfo,
        _eal_components_for_OtherRecipientInfo,
        _rctl2_components_for_OtherRecipientInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "oriType" => BER.validate_object_identifier(_el)?,
            "oriValue" => BER.validate_any(_el)?,
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherRecipientInfo"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedOtherRecipInfo OTHER-RECIPIENT ::= { ... }
/// ```
///
///
pub fn SupportedOtherRecipInfo() -> Vec<OTHER_RECIPIENT> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DigestedData ::= SEQUENCE {
/// version 			CMSVersion,
/// digestAlgorithm 		DigestAlgorithmIdentifier,
/// encapContentInfo 		EncapsulatedContentInfo,
/// digest 			Digest, ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DigestedData {
    pub version: CMSVersion,
    pub digestAlgorithm: DigestAlgorithmIdentifier,
    pub encapContentInfo: EncapsulatedContentInfo,
    pub digest: Digest,
    pub _unrecognized: Vec<X690Element>,
}
impl DigestedData {
    pub fn new(
        version: CMSVersion,
        digestAlgorithm: DigestAlgorithmIdentifier,
        encapContentInfo: EncapsulatedContentInfo,
        digest: Digest,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DigestedData {
            version,
            digestAlgorithm,
            encapContentInfo,
            digest,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for DigestedData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DigestedData(el)
    }
}

pub const _rctl1_components_for_DigestedData: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
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
        "encapContentInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "digest",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DigestedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DigestedData: &[ComponentSpec; 0] = &[];

pub fn _decode_DigestedData(el: &X690Element) -> ASN1Result<DigestedData> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DigestedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DigestedData,
        _eal_components_for_DigestedData,
        _rctl2_components_for_DigestedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut digestAlgorithm_: OPTIONAL<DigestAlgorithmIdentifier> = None;
    let mut encapContentInfo_: OPTIONAL<EncapsulatedContentInfo> = None;
    let mut digest_: OPTIONAL<Digest> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "digestAlgorithm" => digestAlgorithm_ = Some(_decode_DigestAlgorithmIdentifier(_el)?),
            "encapContentInfo" => encapContentInfo_ = Some(_decode_EncapsulatedContentInfo(_el)?),
            "digest" => digest_ = Some(_decode_Digest(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DigestedData {
        version: version_.unwrap(),
        digestAlgorithm: digestAlgorithm_.unwrap(),
        encapContentInfo: encapContentInfo_.unwrap(),
        digest: digest_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DigestedData(value_: &DigestedData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(_encode_DigestAlgorithmIdentifier(&value_.digestAlgorithm)?);
    components_.push(_encode_EncapsulatedContentInfo(&value_.encapContentInfo)?);
    components_.push(_encode_Digest(&value_.digest)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DigestedData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DigestedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DigestedData,
        _eal_components_for_DigestedData,
        _rctl2_components_for_DigestedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "digestAlgorithm" => _validate_DigestAlgorithmIdentifier(_el)?,
            "encapContentInfo" => _validate_EncapsulatedContentInfo(_el)?,
            "digest" => _validate_Digest(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Digest  ::=  OCTET STRING
/// ```
pub type Digest = OCTET_STRING; // OctetStringType

pub fn _decode_Digest(el: &X690Element) -> ASN1Result<Digest> {
    BER.decode_octet_string(&el)
}

pub fn _encode_Digest(value_: &Digest) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_Digest(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedData ::= SEQUENCE {
/// version 			CMSVersion,
/// encryptedContentInfo 	EncryptedContentInfo,
/// ...,
/// [[2: unprotectedAttrs 	[1] IMPLICIT Attributes
/// 		{{UnprotectedEncAttributes}} OPTIONAL ]] }
/// ```
///
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub version: CMSVersion,
    pub encryptedContentInfo: EncryptedContentInfo,
    pub _unrecognized: Vec<X690Element>,
}
impl EncryptedData {
    pub fn new(
        version: CMSVersion,
        encryptedContentInfo: EncryptedContentInfo,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EncryptedData {
            version,
            encryptedContentInfo,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for EncryptedData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedData(el)
    }
}

pub const _rctl1_components_for_EncryptedData: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedContentInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncryptedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedData: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedData(el: &X690Element) -> ASN1Result<EncryptedData> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EncryptedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedData,
        _eal_components_for_EncryptedData,
        _rctl2_components_for_EncryptedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut encryptedContentInfo_: OPTIONAL<EncryptedContentInfo> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "encryptedContentInfo" => {
                encryptedContentInfo_ = Some(_decode_EncryptedContentInfo(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EncryptedData {
        version: version_.unwrap(),
        encryptedContentInfo: encryptedContentInfo_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_EncryptedData(value_: &EncryptedData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(_encode_EncryptedContentInfo(&value_.encryptedContentInfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EncryptedData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EncryptedData")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncryptedData,
        _eal_components_for_EncryptedData,
        _rctl2_components_for_EncryptedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "encryptedContentInfo" => _validate_EncryptedContentInfo(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticatedData ::= SEQUENCE {
/// version 		CMSVersion,
/// originatorInfo 	[0] IMPLICIT OriginatorInfo OPTIONAL,
/// recipientInfos 	RecipientInfos,
/// macAlgorithm 	MessageAuthenticationCodeAlgorithm,
/// digestAlgorithm 	[1] DigestAlgorithmIdentifier OPTIONAL,
/// encapContentInfo 	EncapsulatedContentInfo,
/// authAttrs 		[2] IMPLICIT AuthAttributes OPTIONAL,
/// mac 			MessageAuthenticationCode,
/// unauthAttrs 	[3] IMPLICIT UnauthAttributes OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct AuthenticatedData {
    pub version: CMSVersion,
    pub originatorInfo: OPTIONAL<OriginatorInfo>,
    pub recipientInfos: RecipientInfos,
    pub macAlgorithm: MessageAuthenticationCodeAlgorithm,
    pub digestAlgorithm: OPTIONAL<DigestAlgorithmIdentifier>,
    pub encapContentInfo: EncapsulatedContentInfo,
    pub authAttrs: OPTIONAL<AuthAttributes>,
    pub mac: MessageAuthenticationCode,
    pub unauthAttrs: OPTIONAL<UnauthAttributes>,
}
impl AuthenticatedData {
    pub fn new(
        version: CMSVersion,
        originatorInfo: OPTIONAL<OriginatorInfo>,
        recipientInfos: RecipientInfos,
        macAlgorithm: MessageAuthenticationCodeAlgorithm,
        digestAlgorithm: OPTIONAL<DigestAlgorithmIdentifier>,
        encapContentInfo: EncapsulatedContentInfo,
        authAttrs: OPTIONAL<AuthAttributes>,
        mac: MessageAuthenticationCode,
        unauthAttrs: OPTIONAL<UnauthAttributes>,
    ) -> Self {
        AuthenticatedData {
            version,
            originatorInfo,
            recipientInfos,
            macAlgorithm,
            digestAlgorithm,
            encapContentInfo,
            authAttrs,
            mac,
            unauthAttrs,
        }
    }
}
impl TryFrom<&X690Element> for AuthenticatedData {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticatedData(el)
    }
}

pub const _rctl1_components_for_AuthenticatedData: &[ComponentSpec; 9] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "originatorInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "recipientInfos",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "macAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "digestAlgorithm",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encapContentInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "mac",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "unauthAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AuthenticatedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AuthenticatedData: &[ComponentSpec; 0] = &[];

pub fn _decode_AuthenticatedData(el: &X690Element) -> ASN1Result<AuthenticatedData> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthenticatedData")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthenticatedData,
        _eal_components_for_AuthenticatedData,
        _rctl2_components_for_AuthenticatedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut originatorInfo_: OPTIONAL<OriginatorInfo> = None;
    let mut recipientInfos_: OPTIONAL<RecipientInfos> = None;
    let mut macAlgorithm_: OPTIONAL<MessageAuthenticationCodeAlgorithm> = None;
    let mut digestAlgorithm_: OPTIONAL<DigestAlgorithmIdentifier> = None;
    let mut encapContentInfo_: OPTIONAL<EncapsulatedContentInfo> = None;
    let mut authAttrs_: OPTIONAL<AuthAttributes> = None;
    let mut mac_: OPTIONAL<MessageAuthenticationCode> = None;
    let mut unauthAttrs_: OPTIONAL<UnauthAttributes> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "originatorInfo" => originatorInfo_ = Some(_decode_OriginatorInfo(_el)?),
            "recipientInfos" => recipientInfos_ = Some(_decode_RecipientInfos(_el)?),
            "macAlgorithm" => {
                macAlgorithm_ = Some(_decode_MessageAuthenticationCodeAlgorithm(_el)?)
            }
            "digestAlgorithm" => digestAlgorithm_ = Some(_decode_DigestAlgorithmIdentifier(_el)?),
            "encapContentInfo" => encapContentInfo_ = Some(_decode_EncapsulatedContentInfo(_el)?),
            "authAttrs" => authAttrs_ = Some(_decode_AuthAttributes(_el)?),
            "mac" => mac_ = Some(_decode_MessageAuthenticationCode(_el)?),
            "unauthAttrs" => unauthAttrs_ = Some(_decode_UnauthAttributes(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthenticatedData")
                )
            }
        }
    }
    Ok(AuthenticatedData {
        version: version_.unwrap(),
        originatorInfo: originatorInfo_,
        recipientInfos: recipientInfos_.unwrap(),
        macAlgorithm: macAlgorithm_.unwrap(),
        digestAlgorithm: digestAlgorithm_,
        encapContentInfo: encapContentInfo_.unwrap(),
        authAttrs: authAttrs_,
        mac: mac_.unwrap(),
        unauthAttrs: unauthAttrs_,
    })
}

pub fn _encode_AuthenticatedData(value_: &AuthenticatedData) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(_encode_CMSVersion(&value_.version)?);
    if let Some(v_) = &value_.originatorInfo {
        components_.push(|v_1: &OriginatorInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OriginatorInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_RecipientInfos(&value_.recipientInfos)?);
    components_.push(_encode_MessageAuthenticationCodeAlgorithm(
        &value_.macAlgorithm,
    )?);
    if let Some(v_) = &value_.digestAlgorithm {
        components_.push(
            |v_1: &DigestAlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DigestAlgorithmIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v_)?,
        );
    }
    components_.push(_encode_EncapsulatedContentInfo(&value_.encapContentInfo)?);
    if let Some(v_) = &value_.authAttrs {
        components_.push(|v_1: &AuthAttributes| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AuthAttributes(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_MessageAuthenticationCode(&value_.mac)?);
    if let Some(v_) = &value_.unauthAttrs {
        components_.push(|v_1: &UnauthAttributes| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_UnauthAttributes(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_AuthenticatedData(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthenticatedData")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthenticatedData,
        _eal_components_for_AuthenticatedData,
        _rctl2_components_for_AuthenticatedData,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "originatorInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "originatorInfo")
                    );
                }
                Ok(_validate_OriginatorInfo(&el)?)
            }(_el)?,
            "recipientInfos" => _validate_RecipientInfos(_el)?,
            "macAlgorithm" => _validate_MessageAuthenticationCodeAlgorithm(_el)?,
            "digestAlgorithm" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "digestAlgorithm",
                    ));
                }
                Ok(_validate_DigestAlgorithmIdentifier(&el)?)
            }(_el)?,
            "encapContentInfo" => _validate_EncapsulatedContentInfo(_el)?,
            "authAttrs" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "authAttrs")
                    );
                }
                Ok(_validate_AuthAttributes(&el)?)
            }(_el)?,
            "mac" => _validate_MessageAuthenticationCode(_el)?,
            "unauthAttrs" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "unauthAttrs")
                    );
                }
                Ok(_validate_UnauthAttributes(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthenticatedData")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthAttributes  ::=  SET SIZE (1..MAX) OF Attribute {{AuthAttributeSet}}
/// ```
pub type AuthAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_AuthAttributes(el: &X690Element) -> ASN1Result<AuthAttributes> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthAttributes"))
        }
    };
    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Attribute(el)?);
    }
    Ok(items)
}

pub fn _encode_AuthAttributes(value_: &AuthAttributes) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Attribute(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AuthAttributes(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Attribute(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AuthAttributes")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthAttributeSet ATTRIBUTE ::= { aa-contentType | aa-messageDigest | aa-signingTime, ...}
/// ```
///
///
pub fn AuthAttributeSet() -> Vec<ATTRIBUTE> {
    Vec::from([aa_contentType(), aa_messageDigest(), aa_signingTime()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageAuthenticationCode  ::=  OCTET STRING
/// ```
pub type MessageAuthenticationCode = OCTET_STRING; // OctetStringType

pub fn _decode_MessageAuthenticationCode(
    el: &X690Element,
) -> ASN1Result<MessageAuthenticationCode> {
    BER.decode_octet_string(&el)
}

pub fn _encode_MessageAuthenticationCode(
    value_: &MessageAuthenticationCode,
) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_MessageAuthenticationCode(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnauthAttributes  ::=  SET SIZE (1..MAX) OF Attribute {{UnauthAttributeSet}}
/// ```
pub type UnauthAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_UnauthAttributes(el: &X690Element) -> ASN1Result<UnauthAttributes> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UnauthAttributes")
            )
        }
    };
    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Attribute(el)?);
    }
    Ok(items)
}

pub fn _encode_UnauthAttributes(value_: &UnauthAttributes) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Attribute(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_UnauthAttributes(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Attribute(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UnauthAttributes")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnauthAttributeSet ATTRIBUTE ::= {...}
/// ```
///
///
pub fn UnauthAttributeSet() -> Vec<ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DigestAlgorithmIdentifier  ::=  AlgorithmIdentifier
/// {DIGEST-ALGORITHM, {DigestAlgorithmSet}}
/// ```
pub type DigestAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_DigestAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<DigestAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_DigestAlgorithmIdentifier(
    value_: &DigestAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_DigestAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DigestAlgorithmSet DIGEST-ALGORITHM ::= {
/// CryptographicMessageSyntaxAlgorithms-2009.MessageDigestAlgs, ... }
/// ```
///
///
pub fn DigestAlgorithmSet() -> Vec<DIGEST_ALGORITHM> {
    MessageDigestAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureAlgorithmIdentifier  ::=  AlgorithmIdentifier
/// {SIGNATURE-ALGORITHM, {SignatureAlgorithmSet}}
/// ```
pub type SignatureAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_SignatureAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<SignatureAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_SignatureAlgorithmIdentifier(
    value_: &SignatureAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_SignatureAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureAlgorithmSet SIGNATURE-ALGORITHM ::= { SignatureAlgs, ... }
/// ```
///
///
pub fn SignatureAlgorithmSet() -> Vec<SIGNATURE_ALGORITHM> {
    SignatureAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyEncryptionAlgorithmIdentifier  ::=  AlgorithmIdentifier
/// {KEY-WRAP, {KeyEncryptionAlgorithmSet}}
/// ```
pub type KeyEncryptionAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_KeyEncryptionAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<KeyEncryptionAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_KeyEncryptionAlgorithmIdentifier(
    value_: &KeyEncryptionAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_KeyEncryptionAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyEncryptionAlgorithmSet KEY-WRAP ::= { KeyWrapAlgs, ... }
/// ```
///
///
pub fn KeyEncryptionAlgorithmSet() -> Vec<KEY_WRAP> {
    KeyWrapAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentEncryptionAlgorithmIdentifier  ::=  AlgorithmIdentifier
/// {CONTENT-ENCRYPTION, {ContentEncryptionAlgorithmSet}}
/// ```
pub type ContentEncryptionAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_ContentEncryptionAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<ContentEncryptionAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_ContentEncryptionAlgorithmIdentifier(
    value_: &ContentEncryptionAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_ContentEncryptionAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentEncryptionAlgorithmSet CONTENT-ENCRYPTION ::= { ContentEncryptionAlgs, ... }
/// ```
///
///
pub fn ContentEncryptionAlgorithmSet() -> Vec<CONTENT_ENCRYPTION> {
    ContentEncryptionAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageAuthenticationCodeAlgorithm  ::=  AlgorithmIdentifier
/// {MAC-ALGORITHM, {MessageAuthenticationCodeAlgorithmSet}}
/// ```
pub type MessageAuthenticationCodeAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_MessageAuthenticationCodeAlgorithm(
    el: &X690Element,
) -> ASN1Result<MessageAuthenticationCodeAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_MessageAuthenticationCodeAlgorithm(
    value_: &MessageAuthenticationCodeAlgorithm,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_MessageAuthenticationCodeAlgorithm(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageAuthenticationCodeAlgorithmSet MAC-ALGORITHM ::= { MessageAuthAlgs, ... }
/// ```
///
///
pub fn MessageAuthenticationCodeAlgorithmSet() -> Vec<MAC_ALGORITHM> {
    MessageAuthAlgs()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyDerivationAlgorithmIdentifier  ::=  AlgorithmIdentifier
/// {KEY-DERIVATION, {KeyDerivationAlgs, ...}}
/// ```
pub type KeyDerivationAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_KeyDerivationAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<KeyDerivationAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_KeyDerivationAlgorithmIdentifier(
    value_: &KeyDerivationAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_KeyDerivationAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RevocationInfoChoices  ::=  SET OF RevocationInfoChoice
/// ```
pub type RevocationInfoChoices = Vec<RevocationInfoChoice>; // SetOfType

pub fn _decode_RevocationInfoChoices(el: &X690Element) -> ASN1Result<RevocationInfoChoices> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RevocationInfoChoices")
            )
        }
    };
    let mut items: SET_OF<RevocationInfoChoice> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RevocationInfoChoice(el)?);
    }
    Ok(items)
}

pub fn _encode_RevocationInfoChoices(value_: &RevocationInfoChoices) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RevocationInfoChoice(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RevocationInfoChoices(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RevocationInfoChoice(&sub)?;
            }
            Ok(())
        }
        _ => {
            Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RevocationInfoChoices"))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RevocationInfoChoice  ::=  CHOICE {
/// crl CertificateList,
/// ...,
/// [[5: other [1] IMPLICIT OtherRevocationInfoFormat ]] }
/// ```
#[derive(Debug, Clone)]
pub enum RevocationInfoChoice {
    crl(CertificateList),
    other(OtherRevocationInfoFormat),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for RevocationInfoChoice {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RevocationInfoChoice(el)
    }
}

pub fn _decode_RevocationInfoChoice(el: &X690Element) -> ASN1Result<RevocationInfoChoice> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(RevocationInfoChoice::crl(_decode_CertificateList(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(RevocationInfoChoice::other(
            _decode_OtherRevocationInfoFormat(&el)?,
        )),
        _ => Ok(RevocationInfoChoice::_unrecognized(el.clone())),
    }
}

pub fn _encode_RevocationInfoChoice(value_: &RevocationInfoChoice) -> ASN1Result<X690Element> {
    match value_ {
        RevocationInfoChoice::crl(v) => _encode_CertificateList(&v),
        RevocationInfoChoice::other(v) => {
            |v_1: &OtherRevocationInfoFormat| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_OtherRevocationInfoFormat(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        RevocationInfoChoice::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_RevocationInfoChoice(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_CertificateList(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "other"));
            }
            Ok(_validate_OtherRevocationInfoFormat(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

pub type OTHER_REVOK_INFO = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OtherRevocationInfoFormat ::= SEQUENCE {
/// otherRevInfoFormat    OTHER-REVOK-INFO.&id({SupportedOtherRevokInfo}),
/// otherRevInfo          OTHER-REVOK-INFO.&Type
/// 	({SupportedOtherRevokInfo}{@otherRevInfoFormat})}
/// ```
///
#[derive(Debug, Clone)]
pub struct OtherRevocationInfoFormat {
    pub otherRevInfoFormat: OBJECT_IDENTIFIER,
    pub otherRevInfo: X690Element,
}
impl OtherRevocationInfoFormat {
    pub fn new(otherRevInfoFormat: OBJECT_IDENTIFIER, otherRevInfo: X690Element) -> Self {
        OtherRevocationInfoFormat {
            otherRevInfoFormat,
            otherRevInfo,
        }
    }
}
impl TryFrom<&X690Element> for OtherRevocationInfoFormat {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OtherRevocationInfoFormat(el)
    }
}

pub const _rctl1_components_for_OtherRevocationInfoFormat: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "otherRevInfoFormat",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("otherRevInfo", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OtherRevocationInfoFormat: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OtherRevocationInfoFormat: &[ComponentSpec; 0] = &[];

pub fn _decode_OtherRevocationInfoFormat(
    el: &X690Element,
) -> ASN1Result<OtherRevocationInfoFormat> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OtherRevocationInfoFormat",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherRevocationInfoFormat,
        _eal_components_for_OtherRevocationInfoFormat,
        _rctl2_components_for_OtherRevocationInfoFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut otherRevInfoFormat_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut otherRevInfo_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "otherRevInfoFormat" => otherRevInfoFormat_ = Some(BER.decode_object_identifier(_el)?),
            "otherRevInfo" => otherRevInfo_ = Some(x690_identity(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OtherRevocationInfoFormat",
                ))
            }
        }
    }
    Ok(OtherRevocationInfoFormat {
        otherRevInfoFormat: otherRevInfoFormat_.unwrap(),
        otherRevInfo: otherRevInfo_.unwrap(),
    })
}

pub fn _encode_OtherRevocationInfoFormat(
    value_: &OtherRevocationInfoFormat,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.otherRevInfoFormat)?);
    components_.push(x690_identity(&value_.otherRevInfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OtherRevocationInfoFormat(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OtherRevocationInfoFormat",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherRevocationInfoFormat,
        _eal_components_for_OtherRevocationInfoFormat,
        _rctl2_components_for_OtherRevocationInfoFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "otherRevInfoFormat" => BER.validate_object_identifier(_el)?,
            "otherRevInfo" => BER.validate_any(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OtherRevocationInfoFormat",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedOtherRevokInfo OTHER-REVOK-INFO ::= { ... }
/// ```
///
///
pub fn SupportedOtherRevokInfo() -> Vec<OTHER_REVOK_INFO> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateChoices  ::=  CHOICE {
/// certificate 		Certificate,
/// extendedCertificate 	[0] IMPLICIT ExtendedCertificate,
/// -- Obsolete
/// ...,
/// [[3: v1AttrCert 		[1] IMPLICIT AttributeCertificateV1]],
/// -- Obsolete
/// [[4: v2AttrCert 		[2] IMPLICIT AttributeCertificateV2]],
/// [[5: other      		[3] IMPLICIT OtherCertificateFormat]] }
/// ```
#[derive(Debug, Clone)]
pub enum CertificateChoices {
    certificate(Certificate),
    extendedCertificate(ExtendedCertificate),
    v1AttrCert(AttributeCertificateV1),
    v2AttrCert(AttributeCertificateV2),
    other(OtherCertificateFormat),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for CertificateChoices {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateChoices(el)
    }
}

pub fn _decode_CertificateChoices(el: &X690Element) -> ASN1Result<CertificateChoices> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(CertificateChoices::certificate(_decode_Certificate(&el)?)),
        (TagClass::CONTEXT, 0) => Ok(CertificateChoices::extendedCertificate(
            _decode_ExtendedCertificate(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertificateChoices::v1AttrCert(
            _decode_AttributeCertificateV1(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(CertificateChoices::v2AttrCert(
            _decode_AttributeCertificateV2(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(CertificateChoices::other(_decode_OtherCertificateFormat(
            &el,
        )?)),
        _ => Ok(CertificateChoices::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertificateChoices(value_: &CertificateChoices) -> ASN1Result<X690Element> {
    match value_ {
        CertificateChoices::certificate(v) => _encode_Certificate(&v),
        CertificateChoices::extendedCertificate(v) => {
            |v_1: &ExtendedCertificate| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ExtendedCertificate(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        CertificateChoices::v1AttrCert(v) => {
            |v_1: &AttributeCertificateV1| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeCertificateV1(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertificateChoices::v2AttrCert(v) => {
            |v_1: &AttributeCertificateV2| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeCertificateV2(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 2;
                Ok(el_1)
            }(&v)
        }
        CertificateChoices::other(v) => |v_1: &OtherCertificateFormat| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OtherCertificateFormat(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        CertificateChoices::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertificateChoices(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_Certificate(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "extendedCertificate",
                ));
            }
            Ok(_validate_ExtendedCertificate(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "v1AttrCert"));
            }
            Ok(_validate_AttributeCertificateV1(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "v2AttrCert"));
            }
            Ok(_validate_AttributeCertificateV2(&el)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "other"));
            }
            Ok(_validate_OtherCertificateFormat(&el)?)
        }(&el),
        _ => Ok(()),
    }
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

pub type OTHER_CERT_FMT = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OtherCertificateFormat ::= SEQUENCE {
/// otherCertFormat 	OTHER-CERT-FMT.&id({SupportedCertFormats}),
/// otherCert       	OTHER-CERT-FMT.&Type
/// 	({SupportedCertFormats}{@otherCertFormat})}
/// ```
///
#[derive(Debug, Clone)]
pub struct OtherCertificateFormat {
    pub otherCertFormat: OBJECT_IDENTIFIER,
    pub otherCert: X690Element,
}
impl OtherCertificateFormat {
    pub fn new(otherCertFormat: OBJECT_IDENTIFIER, otherCert: X690Element) -> Self {
        OtherCertificateFormat {
            otherCertFormat,
            otherCert,
        }
    }
}
impl TryFrom<&X690Element> for OtherCertificateFormat {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OtherCertificateFormat(el)
    }
}

pub const _rctl1_components_for_OtherCertificateFormat: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "otherCertFormat",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("otherCert", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OtherCertificateFormat: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OtherCertificateFormat: &[ComponentSpec; 0] = &[];

pub fn _decode_OtherCertificateFormat(el: &X690Element) -> ASN1Result<OtherCertificateFormat> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OtherCertificateFormat",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherCertificateFormat,
        _eal_components_for_OtherCertificateFormat,
        _rctl2_components_for_OtherCertificateFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut otherCertFormat_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut otherCert_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "otherCertFormat" => otherCertFormat_ = Some(BER.decode_object_identifier(_el)?),
            "otherCert" => otherCert_ = Some(x690_identity(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OtherCertificateFormat",
                ))
            }
        }
    }
    Ok(OtherCertificateFormat {
        otherCertFormat: otherCertFormat_.unwrap(),
        otherCert: otherCert_.unwrap(),
    })
}

pub fn _encode_OtherCertificateFormat(value_: &OtherCertificateFormat) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.otherCertFormat)?);
    components_.push(x690_identity(&value_.otherCert)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OtherCertificateFormat(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OtherCertificateFormat",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherCertificateFormat,
        _eal_components_for_OtherCertificateFormat,
        _rctl2_components_for_OtherCertificateFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "otherCertFormat" => BER.validate_object_identifier(_el)?,
            "otherCert" => BER.validate_any(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OtherCertificateFormat",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedCertFormats OTHER-CERT-FMT ::= { ... }
/// ```
///
///
pub fn SupportedCertFormats() -> Vec<OTHER_CERT_FMT> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateSet  ::=  SET OF CertificateChoices
/// ```
pub type CertificateSet = Vec<CertificateChoices>; // SetOfType

pub fn _decode_CertificateSet(el: &X690Element) -> ASN1Result<CertificateSet> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificateSet"))
        }
    };
    let mut items: SET_OF<CertificateChoices> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertificateChoices(el)?);
    }
    Ok(items)
}

pub fn _encode_CertificateSet(value_: &CertificateSet) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertificateChoices(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertificateSet(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertificateChoices(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificateSet")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IssuerAndSerialNumber ::= SEQUENCE {
/// issuer 		Name,
/// serialNumber 	CertificateSerialNumber }
/// ```
///
#[derive(Debug, Clone)]
pub struct IssuerAndSerialNumber {
    pub issuer: Name,
    pub serialNumber: CertificateSerialNumber,
}
impl IssuerAndSerialNumber {
    pub fn new(issuer: Name, serialNumber: CertificateSerialNumber) -> Self {
        IssuerAndSerialNumber {
            issuer,
            serialNumber,
        }
    }
}
impl TryFrom<&X690Element> for IssuerAndSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerAndSerialNumber(el)
    }
}

pub const _rctl1_components_for_IssuerAndSerialNumber: &[ComponentSpec; 2] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IssuerAndSerialNumber: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IssuerAndSerialNumber: &[ComponentSpec; 0] = &[];

pub fn _decode_IssuerAndSerialNumber(el: &X690Element) -> ASN1Result<IssuerAndSerialNumber> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerAndSerialNumber")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerAndSerialNumber,
        _eal_components_for_IssuerAndSerialNumber,
        _rctl2_components_for_IssuerAndSerialNumber,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "IssuerAndSerialNumber",
                ))
            }
        }
    }
    Ok(IssuerAndSerialNumber {
        issuer: issuer_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
    })
}

pub fn _encode_IssuerAndSerialNumber(value_: &IssuerAndSerialNumber) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_IssuerAndSerialNumber(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerAndSerialNumber")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerAndSerialNumber,
        _eal_components_for_IssuerAndSerialNumber,
        _rctl2_components_for_IssuerAndSerialNumber,
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
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "IssuerAndSerialNumber",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CMSVersion  ::=  INTEGER  { v0(0), v1(1), v2(2), v3(3), v4(4), v5(5) }
/// ```
pub type CMSVersion = i8;

pub const CMSVersion_v0: CMSVersion = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v1: CMSVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v2: CMSVersion = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v3: CMSVersion = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v4: CMSVersion = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v5: CMSVersion = 5; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_CMSVersion(el: &X690Element) -> ASN1Result<CMSVersion> {
    BER.decode_i8(el)
}

pub fn _encode_CMSVersion(value_: &CMSVersion) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_CMSVersion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserKeyingMaterial  ::=  OCTET STRING
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

pub type KEY_ATTRIBUTE = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OtherKeyAttribute ::= SEQUENCE {
/// keyAttrId  KEY-ATTRIBUTE.&id({SupportedKeyAttributes}),
/// keyAttr    KEY-ATTRIBUTE.&Type({SupportedKeyAttributes}{@keyAttrId})}
/// ```
///
#[derive(Debug, Clone)]
pub struct OtherKeyAttribute {
    pub keyAttrId: OBJECT_IDENTIFIER,
    pub keyAttr: X690Element,
}
impl OtherKeyAttribute {
    pub fn new(keyAttrId: OBJECT_IDENTIFIER, keyAttr: X690Element) -> Self {
        OtherKeyAttribute { keyAttrId, keyAttr }
    }
}
impl TryFrom<&X690Element> for OtherKeyAttribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OtherKeyAttribute(el)
    }
}

pub const _rctl1_components_for_OtherKeyAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "keyAttrId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("keyAttr", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OtherKeyAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OtherKeyAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_OtherKeyAttribute(el: &X690Element) -> ASN1Result<OtherKeyAttribute> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherKeyAttribute")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherKeyAttribute,
        _eal_components_for_OtherKeyAttribute,
        _rctl2_components_for_OtherKeyAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut keyAttrId_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut keyAttr_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyAttrId" => keyAttrId_ = Some(BER.decode_object_identifier(_el)?),
            "keyAttr" => keyAttr_ = Some(x690_identity(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherKeyAttribute")
                )
            }
        }
    }
    Ok(OtherKeyAttribute {
        keyAttrId: keyAttrId_.unwrap(),
        keyAttr: keyAttr_.unwrap(),
    })
}

pub fn _encode_OtherKeyAttribute(value_: &OtherKeyAttribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.keyAttrId)?);
    components_.push(x690_identity(&value_.keyAttr)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OtherKeyAttribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherKeyAttribute")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OtherKeyAttribute,
        _eal_components_for_OtherKeyAttribute,
        _rctl2_components_for_OtherKeyAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyAttrId" => BER.validate_object_identifier(_el)?,
            "keyAttr" => BER.validate_any(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OtherKeyAttribute")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedKeyAttributes KEY-ATTRIBUTE ::= { ... }
/// ```
///
///
pub fn SupportedKeyAttributes() -> Vec<KEY_ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-contentInfo OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs9(9) smime(16) ct(1) 6 }
/// ```
///
///
pub fn id_ct_contentInfo() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, /* smime */ 16, /* ct */ 1, 6) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ct-Data CONTENT-TYPE ::= { IDENTIFIED BY id-data }
/// ```
///
///
pub fn ct_Data() -> CONTENT_TYPE {
    CONTENT_TYPE {
        id: id_data(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ct_Data {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-data OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs7(7) 1 }
/// ```
///
///
pub fn id_data() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ct-SignedData CONTENT-TYPE ::= { TYPE SignedData IDENTIFIED BY id-signedData}
/// ```
///
///
pub fn ct_SignedData() -> CONTENT_TYPE {
    CONTENT_TYPE {
        id: id_signedData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ct_SignedData {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SignedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SignedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SignedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SignedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signedData OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs7(7) 2 }
/// ```
///
///
pub fn id_signedData() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ct-EnvelopedData CONTENT-TYPE ::= { TYPE EnvelopedData IDENTIFIED BY id-envelopedData}
/// ```
///
///
pub fn ct_EnvelopedData() -> CONTENT_TYPE {
    CONTENT_TYPE {
        id: id_envelopedData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ct_EnvelopedData {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = EnvelopedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_EnvelopedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_EnvelopedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_EnvelopedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-envelopedData OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs7(7) 3 }
/// ```
///
///
pub fn id_envelopedData() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ct-DigestedData CONTENT-TYPE ::= { TYPE DigestedData IDENTIFIED BY id-digestedData}
/// ```
///
///
pub fn ct_DigestedData() -> CONTENT_TYPE {
    CONTENT_TYPE {
        id: id_digestedData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ct_DigestedData {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DigestedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DigestedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DigestedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DigestedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-digestedData OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs7(7) 5 }
/// ```
///
///
pub fn id_digestedData() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ct-EncryptedData CONTENT-TYPE ::= { TYPE EncryptedData IDENTIFIED BY id-encryptedData}
/// ```
///
///
pub fn ct_EncryptedData() -> CONTENT_TYPE {
    CONTENT_TYPE {
        id: id_encryptedData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ct_EncryptedData {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = EncryptedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_EncryptedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_EncryptedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_EncryptedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-encryptedData OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs7(7) 6 }
/// ```
///
///
pub fn id_encryptedData() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 6) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ct-AuthenticatedData CONTENT-TYPE ::= { TYPE AuthenticatedData IDENTIFIED BY id-ct-authData}
/// ```
///
///
pub fn ct_AuthenticatedData() -> CONTENT_TYPE {
    CONTENT_TYPE {
        id: id_ct_authData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ct_AuthenticatedData {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AuthenticatedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AuthenticatedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AuthenticatedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AuthenticatedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-authData OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) ct(1) 2 }
/// ```
///
///
pub fn id_ct_authData() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageDigest  ::=  OCTET STRING
/// ```
pub type MessageDigest = OCTET_STRING; // OctetStringType

pub fn _decode_MessageDigest(el: &X690Element) -> ASN1Result<MessageDigest> {
    BER.decode_octet_string(&el)
}

pub fn _encode_MessageDigest(value_: &MessageDigest) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_MessageDigest(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SigningTime   ::=  Time
/// ```
pub type SigningTime = Time; // DefinedType

pub fn _decode_SigningTime(el: &X690Element) -> ASN1Result<SigningTime> {
    _decode_Time(&el)
}

pub fn _encode_SigningTime(value_: &SigningTime) -> ASN1Result<X690Element> {
    _encode_Time(&value_)
}

pub fn _validate_SigningTime(el: &X690Element) -> ASN1Result<()> {
    _validate_Time(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Time  ::=  CHOICE {
/// utcTime UTCTime,
/// generalTime GeneralizedTime }
/// ```
#[derive(Debug, Clone)]
pub enum Time {
    utcTime(UTCTime),
    generalTime(GeneralizedTime),
}

impl TryFrom<&X690Element> for Time {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Time(el)
    }
}

pub fn _decode_Time(el: &X690Element) -> ASN1Result<Time> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 23) => Ok(Time::utcTime(BER.decode_utc_time(&el)?)),
        (TagClass::UNIVERSAL, 24) => Ok(Time::generalTime(BER.decode_generalized_time(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Time",
            ))
        }
    }
}

pub fn _encode_Time(value_: &Time) -> ASN1Result<X690Element> {
    match value_ {
        Time::utcTime(v) => BER.encode_utc_time(&v),
        Time::generalTime(v) => BER.encode_generalized_time(&v),
    }
}

pub fn _validate_Time(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 23) => BER.validate_utc_time(&el),
        (TagClass::UNIVERSAL, 24) => BER.validate_generalized_time(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Time",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Countersignature  ::=  SignerInfo
/// ```
pub type Countersignature = SignerInfo; // DefinedType

pub fn _decode_Countersignature(el: &X690Element) -> ASN1Result<Countersignature> {
    _decode_SignerInfo(&el)
}

pub fn _encode_Countersignature(value_: &Countersignature) -> ASN1Result<X690Element> {
    _encode_SignerInfo(&value_)
}

pub fn _validate_Countersignature(el: &X690Element) -> ASN1Result<()> {
    _validate_SignerInfo(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-contentType ATTRIBUTE ::= { TYPE ContentType IDENTIFIED BY id-contentType }
/// ```
///
///
pub fn aa_contentType() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_contentType(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod aa_contentType {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ContentType; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ContentType(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ContentType(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ContentType(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contentType OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs9(9) 3 }
/// ```
///
///
pub fn id_contentType() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-messageDigest ATTRIBUTE ::= { TYPE MessageDigest IDENTIFIED BY id-messageDigest}
/// ```
///
///
pub fn aa_messageDigest() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_messageDigest(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod aa_messageDigest {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MessageDigest; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MessageDigest(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MessageDigest(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MessageDigest(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-messageDigest OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs9(9) 4 }
/// ```
///
///
pub fn id_messageDigest() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 4) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-signingTime ATTRIBUTE ::= { TYPE SigningTime IDENTIFIED BY id-signingTime }
/// ```
///
///
pub fn aa_signingTime() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_signingTime(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod aa_signingTime {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SigningTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SigningTime(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SigningTime(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SigningTime(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signingTime OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs9(9) 5 }
/// ```
///
///
pub fn id_signingTime() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-countersignature ATTRIBUTE ::= { TYPE Countersignature IDENTIFIED BY id-countersignature }
/// ```
///
///
pub fn aa_countersignature() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_countersignature(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod aa_countersignature {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Countersignature; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Countersignature(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Countersignature(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Countersignature(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-countersignature OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs9(9) 6 }
/// ```
///
///
pub fn id_countersignature() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 6) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedCertificateOrCertificate  ::=  CHOICE {
/// certificate 		Certificate,
/// extendedCertificate 	[0] IMPLICIT ExtendedCertificate }
/// ```
#[derive(Debug, Clone)]
pub enum ExtendedCertificateOrCertificate {
    certificate(Certificate),
    extendedCertificate(ExtendedCertificate),
}

impl TryFrom<&X690Element> for ExtendedCertificateOrCertificate {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedCertificateOrCertificate(el)
    }
}

pub fn _decode_ExtendedCertificateOrCertificate(
    el: &X690Element,
) -> ASN1Result<ExtendedCertificateOrCertificate> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(ExtendedCertificateOrCertificate::certificate(
            _decode_Certificate(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(ExtendedCertificateOrCertificate::extendedCertificate(
            _decode_ExtendedCertificate(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ExtendedCertificateOrCertificate",
            ))
        }
    }
}

pub fn _encode_ExtendedCertificateOrCertificate(
    value_: &ExtendedCertificateOrCertificate,
) -> ASN1Result<X690Element> {
    match value_ {
        ExtendedCertificateOrCertificate::certificate(v) => _encode_Certificate(&v),
        ExtendedCertificateOrCertificate::extendedCertificate(v) => {
            |v_1: &ExtendedCertificate| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ExtendedCertificate(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_ExtendedCertificateOrCertificate(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_Certificate(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "extendedCertificate",
                ));
            }
            Ok(_validate_ExtendedCertificate(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ExtendedCertificateOrCertificate",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedCertificate ::= SEQUENCE {
/// extendedCertificateInfo 	ExtendedCertificateInfo,
/// signatureAlgorithm 	SignatureAlgorithmIdentifier,
/// signature 			Signature }
/// ```
///
#[derive(Debug, Clone)]
pub struct ExtendedCertificate {
    pub extendedCertificateInfo: ExtendedCertificateInfo,
    pub signatureAlgorithm: SignatureAlgorithmIdentifier,
    pub signature: Signature,
}
impl ExtendedCertificate {
    pub fn new(
        extendedCertificateInfo: ExtendedCertificateInfo,
        signatureAlgorithm: SignatureAlgorithmIdentifier,
        signature: Signature,
    ) -> Self {
        ExtendedCertificate {
            extendedCertificateInfo,
            signatureAlgorithm,
            signature,
        }
    }
}
impl TryFrom<&X690Element> for ExtendedCertificate {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedCertificate(el)
    }
}

pub const _rctl1_components_for_ExtendedCertificate: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "extendedCertificateInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
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
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendedCertificate: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendedCertificate: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendedCertificate(el: &X690Element) -> ASN1Result<ExtendedCertificate> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedCertificate")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendedCertificate,
        _eal_components_for_ExtendedCertificate,
        _rctl2_components_for_ExtendedCertificate,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut extendedCertificateInfo_: OPTIONAL<ExtendedCertificateInfo> = None;
    let mut signatureAlgorithm_: OPTIONAL<SignatureAlgorithmIdentifier> = None;
    let mut signature_: OPTIONAL<Signature> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "extendedCertificateInfo" => {
                extendedCertificateInfo_ = Some(_decode_ExtendedCertificateInfo(_el)?)
            }
            "signatureAlgorithm" => {
                signatureAlgorithm_ = Some(_decode_SignatureAlgorithmIdentifier(_el)?)
            }
            "signature" => signature_ = Some(_decode_Signature(_el)?),
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedCertificate"))
            }
        }
    }
    Ok(ExtendedCertificate {
        extendedCertificateInfo: extendedCertificateInfo_.unwrap(),
        signatureAlgorithm: signatureAlgorithm_.unwrap(),
        signature: signature_.unwrap(),
    })
}

pub fn _encode_ExtendedCertificate(value_: &ExtendedCertificate) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_ExtendedCertificateInfo(
        &value_.extendedCertificateInfo,
    )?);
    components_.push(_encode_SignatureAlgorithmIdentifier(
        &value_.signatureAlgorithm,
    )?);
    components_.push(_encode_Signature(&value_.signature)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ExtendedCertificate(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedCertificate")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendedCertificate,
        _eal_components_for_ExtendedCertificate,
        _rctl2_components_for_ExtendedCertificate,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "extendedCertificateInfo" => _validate_ExtendedCertificateInfo(_el)?,
            "signatureAlgorithm" => _validate_SignatureAlgorithmIdentifier(_el)?,
            "signature" => _validate_Signature(_el)?,
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedCertificate"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedCertificateInfo ::= SEQUENCE {
/// version 	CMSVersion,
/// certificate	Certificate,
/// attributes 	UnauthAttributes }
/// ```
///
#[derive(Debug, Clone)]
pub struct ExtendedCertificateInfo {
    pub version: CMSVersion,
    pub certificate: Certificate,
    pub attributes: UnauthAttributes,
}
impl ExtendedCertificateInfo {
    pub fn new(
        version: CMSVersion,
        certificate: Certificate,
        attributes: UnauthAttributes,
    ) -> Self {
        ExtendedCertificateInfo {
            version,
            certificate,
            attributes,
        }
    }
}
impl TryFrom<&X690Element> for ExtendedCertificateInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedCertificateInfo(el)
    }
}

pub const _rctl1_components_for_ExtendedCertificateInfo: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certificate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendedCertificateInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendedCertificateInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendedCertificateInfo(el: &X690Element) -> ASN1Result<ExtendedCertificateInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ExtendedCertificateInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendedCertificateInfo,
        _eal_components_for_ExtendedCertificateInfo,
        _rctl2_components_for_ExtendedCertificateInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CMSVersion> = None;
    let mut certificate_: OPTIONAL<Certificate> = None;
    let mut attributes_: OPTIONAL<UnauthAttributes> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CMSVersion(_el)?),
            "certificate" => certificate_ = Some(_decode_Certificate(_el)?),
            "attributes" => attributes_ = Some(_decode_UnauthAttributes(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "ExtendedCertificateInfo",
                ))
            }
        }
    }
    Ok(ExtendedCertificateInfo {
        version: version_.unwrap(),
        certificate: certificate_.unwrap(),
        attributes: attributes_.unwrap(),
    })
}

pub fn _encode_ExtendedCertificateInfo(
    value_: &ExtendedCertificateInfo,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_CMSVersion(&value_.version)?);
    components_.push(_encode_Certificate(&value_.certificate)?);
    components_.push(_encode_UnauthAttributes(&value_.attributes)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ExtendedCertificateInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ExtendedCertificateInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendedCertificateInfo,
        _eal_components_for_ExtendedCertificateInfo,
        _rctl2_components_for_ExtendedCertificateInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CMSVersion(_el)?,
            "certificate" => _validate_Certificate(_el)?,
            "attributes" => _validate_UnauthAttributes(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "ExtendedCertificateInfo",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Signature  ::=  BIT STRING
/// ```
pub type Signature = BIT_STRING;

pub fn _decode_Signature(el: &X690Element) -> ASN1Result<Signature> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Signature(value_: &Signature) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Signature(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute{ ATTRIBUTE:AttrList } ::= SEQUENCE {
/// attrType	ATTRIBUTE.&id({AttrList}),
/// attrValues 	SET OF ATTRIBUTE.&Type({AttrList}{@attrType})  }
/// ```
///
#[derive(Debug, Clone)]
pub struct Attribute {
    pub attrType: OBJECT_IDENTIFIER,
    pub attrValues: Vec<X690Element>,
}
impl Attribute {
    pub fn new(attrType: OBJECT_IDENTIFIER, attrValues: Vec<X690Element>) -> Self {
        Attribute {
            attrType,
            attrValues,
        }
    }
}
impl TryFrom<&X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute(el)
    }
}

pub const _rctl1_components_for_Attribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "attrType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attrValues",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub fn _decode_Attribute(el: &X690Element) -> ASN1Result<Attribute> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute,
        _eal_components_for_Attribute,
        _rctl2_components_for_Attribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attrType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut attrValues_: OPTIONAL<Vec<X690Element>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attrType" => attrType_ = Some(BER.decode_object_identifier(_el)?),
            "attrValues" => {
                attrValues_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "attrValues",
                            ))
                        }
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute"))
            }
        }
    }
    Ok(Attribute {
        attrType: attrType_.unwrap(),
        attrValues: attrValues_.unwrap(),
    })
}

pub fn _encode_Attribute(value_: &Attribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.attrType)?);
    components_.push(|value_: &SET_OF<X690Element>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(x690_identity(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.attrValues)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Attribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute,
        _eal_components_for_Attribute,
        _rctl2_components_for_Attribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attrType" => BER.validate_object_identifier(_el)?,
            "attrValues" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => {
                        Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attrValues"))
                    }
                }
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attributes { ATTRIBUTE:AttrList }  ::=  SET SIZE (1..MAX) OF Attribute {{ AttrList }}
/// ```
pub type Attributes = Vec<Attribute>; // SetOfType

pub fn _decode_Attributes(el: &X690Element) -> ASN1Result<Attributes> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attributes")),
    };
    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Attribute(el)?);
    }
    Ok(items)
}

pub fn _encode_Attributes(value_: &Attributes) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Attribute(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Attributes(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Attribute(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attributes")),
    }
}
