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
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x500::AttributeCertificateDefinitions::{
    AttributeCertificate, _decode_AttributeCertificate, _encode_AttributeCertificate,
};
use x500::AuthenticationFramework::*;
use x500::InformationFramework::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentInfo ::= SEQUENCE {
///   content-type   CMS-CONTENT-TYPE.&id({CMSContentTable}),
///   pkcs7-content  [0]  CMS-CONTENT-TYPE.&Type({CMSContentTable})
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ContentInfo {
    pub content_type: OBJECT_IDENTIFIER,
    pub pkcs7_content: X690Element,
}
impl ContentInfo {
    pub fn new(content_type: OBJECT_IDENTIFIER, pkcs7_content: X690Element) -> Self {
        ContentInfo {
            content_type,
            pkcs7_content,
        }
    }
}
impl TryFrom<X690Element> for ContentInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContentInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContentInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContentInfo(el)
    }
}

pub const _rctl1_components_for_ContentInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "content-type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkcs7-content",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContentInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContentInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ContentInfo(el: &X690Element) -> ASN1Result<ContentInfo> {
    |el_: &X690Element| -> ASN1Result<ContentInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ContentInfo,
            _eal_components_for_ContentInfo,
            _rctl2_components_for_ContentInfo,
        )?;
        let content_type = ber_decode_object_identifier(_components.get("content-type").unwrap())?;
        let pkcs7_content = x690_identity(_components.get("pkcs7-content").unwrap())?;
        Ok(ContentInfo {
            content_type,
            pkcs7_content,
        })
    }(&el)
}

pub fn _encode_ContentInfo(value_: &ContentInfo) -> ASN1Result<X690Element> {
    |value_: &ContentInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_object_identifier(&value_.content_type)?);
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            let mut el_1 = x690_identity(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.pkcs7_content)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

pub type CMS_CONTENT_TYPE = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// CMSContentTable CMS-CONTENT-TYPE ::= {...}
/// ```
///
///
pub fn CMSContentTable() -> Vec<CMS_CONTENT_TYPE> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentType  ::=  OBJECT IDENTIFIER
/// ```
pub type ContentType = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_ContentType(el: &X690Element) -> ASN1Result<ContentType> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_ContentType(value_: &ContentType) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedData ::= SEQUENCE {
///   version           CMSVersion,
///   digestAlgorithms  DigestAlgorithmIdentifiers,
///   encapContentInfo  EncapsulatedContentInfo,
///   certificates      [0] IMPLICIT CertificateSet OPTIONAL,
///   crls              [1] IMPLICIT CertificateRevocationLists OPTIONAL,
///   signerInfos       SignerInfos
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SignedData {
    pub version: CMSVersion,
    pub digestAlgorithms: DigestAlgorithmIdentifiers,
    pub encapContentInfo: EncapsulatedContentInfo,
    pub certificates: OPTIONAL<CertificateSet>,
    pub crls: OPTIONAL<CertificateRevocationLists>,
    pub signerInfos: SignerInfos,
}
impl SignedData {
    pub fn new(
        version: CMSVersion,
        digestAlgorithms: DigestAlgorithmIdentifiers,
        encapContentInfo: EncapsulatedContentInfo,
        certificates: OPTIONAL<CertificateSet>,
        crls: OPTIONAL<CertificateRevocationLists>,
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
impl TryFrom<X690Element> for SignedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SignedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SignedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SignedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SignedData,
            _eal_components_for_SignedData,
            _rctl2_components_for_SignedData,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let digestAlgorithms =
            _decode_DigestAlgorithmIdentifiers(_components.get("digestAlgorithms").unwrap())?;
        let encapContentInfo =
            _decode_EncapsulatedContentInfo(_components.get("encapContentInfo").unwrap())?;
        let certificates: OPTIONAL<CertificateSet> = match _components.get("certificates") {
            Some(c_) => Some(_decode_CertificateSet(c_)?),
            _ => None,
        };
        let crls: OPTIONAL<CertificateRevocationLists> = match _components.get("crls") {
            Some(c_) => Some(_decode_CertificateRevocationLists(c_)?),
            _ => None,
        };
        let signerInfos = _decode_SignerInfos(_components.get("signerInfos").unwrap())?;
        Ok(SignedData {
            version,
            digestAlgorithms,
            encapContentInfo,
            certificates,
            crls,
            signerInfos,
        })
    }(&el)
}

pub fn _encode_SignedData(value_: &SignedData) -> ASN1Result<X690Element> {
    |value_: &SignedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_DigestAlgorithmIdentifiers(
            &value_.digestAlgorithms,
        )?);
        components_.push(_encode_EncapsulatedContentInfo(&value_.encapContentInfo)?);
        if let Some(v_) = &value_.certificates {
            components_.push(|v_1: &CertificateSet| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSet(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.crls {
            components_.push(
                |v_1: &CertificateRevocationLists| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertificateRevocationLists(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?,
            );
        }
        components_.push(_encode_SignerInfos(&value_.signerInfos)?);
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
/// DigestAlgorithmIdentifiers  ::=  SET OF DigestAlgorithmIdentifier
/// ```
pub type DigestAlgorithmIdentifiers = Vec<DigestAlgorithmIdentifier>; // SetOfType

pub fn _decode_DigestAlgorithmIdentifiers(
    el: &X690Element,
) -> ASN1Result<DigestAlgorithmIdentifiers> {
    |el: &X690Element| -> ASN1Result<SET_OF<DigestAlgorithmIdentifier>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<DigestAlgorithmIdentifier> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_DigestAlgorithmIdentifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_DigestAlgorithmIdentifiers(
    value_: &DigestAlgorithmIdentifiers,
) -> ASN1Result<X690Element> {
    |value_: &SET_OF<DigestAlgorithmIdentifier>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_DigestAlgorithmIdentifier(&v)?);
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
/// SignerInfos  ::=  SET OF SignerInfo
/// ```
pub type SignerInfos = Vec<SignerInfo>; // SetOfType

pub fn _decode_SignerInfos(el: &X690Element) -> ASN1Result<SignerInfos> {
    |el: &X690Element| -> ASN1Result<SET_OF<SignerInfo>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<SignerInfo> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_SignerInfo(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SignerInfos(value_: &SignerInfos) -> ASN1Result<X690Element> {
    |value_: &SET_OF<SignerInfo>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_SignerInfo(&v)?);
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
/// EncapsulatedContentInfo ::= SEQUENCE {
///   eContentType  ContentType,
///   eContent      [0] EXPLICIT OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncapsulatedContentInfo {
    pub eContentType: ContentType,
    pub eContent: OPTIONAL<OCTET_STRING>,
}
impl EncapsulatedContentInfo {
    pub fn new(eContentType: ContentType, eContent: OPTIONAL<OCTET_STRING>) -> Self {
        EncapsulatedContentInfo {
            eContentType,
            eContent,
        }
    }
}
impl TryFrom<X690Element> for EncapsulatedContentInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncapsulatedContentInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncapsulatedContentInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<EncapsulatedContentInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncapsulatedContentInfo,
            _eal_components_for_EncapsulatedContentInfo,
            _rctl2_components_for_EncapsulatedContentInfo,
        )?;
        let eContentType = _decode_ContentType(_components.get("eContentType").unwrap())?;
        let eContent: OPTIONAL<OCTET_STRING> = match _components.get("eContent") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(ber_decode_octet_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(EncapsulatedContentInfo {
            eContentType,
            eContent,
        })
    }(&el)
}

pub fn _encode_EncapsulatedContentInfo(
    value_: &EncapsulatedContentInfo,
) -> ASN1Result<X690Element> {
    |value_: &EncapsulatedContentInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_ContentType(&value_.eContentType)?);
        if let Some(v_) = &value_.eContent {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_octet_string(
                        &v_1,
                    )?))),
                ))
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
/// SignerInfo ::= SEQUENCE {
///   version             CMSVersion,
///   sid                 SignerIdentifier,
///   digestAlgorithm     DigestAlgorithmIdentifier,
///   signedAttrs         [0] IMPLICIT SignedAttributes OPTIONAL,
///   signatureAlgorithm  SignatureAlgorithmIdentifier,
///   signature           SignatureValue,
///   unsignedAttrs       [1] IMPLICIT UnsignedAttributes OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SignerInfo {
    pub version: CMSVersion,
    pub sid: SignerIdentifier,
    pub digestAlgorithm: DigestAlgorithmIdentifier,
    pub signedAttrs: OPTIONAL<SignedAttributes>,
    pub signatureAlgorithm: SignatureAlgorithmIdentifier,
    pub signature: SignatureValue,
    pub unsignedAttrs: OPTIONAL<UnsignedAttributes>,
}
impl SignerInfo {
    pub fn new(
        version: CMSVersion,
        sid: SignerIdentifier,
        digestAlgorithm: DigestAlgorithmIdentifier,
        signedAttrs: OPTIONAL<SignedAttributes>,
        signatureAlgorithm: SignatureAlgorithmIdentifier,
        signature: SignatureValue,
        unsignedAttrs: OPTIONAL<UnsignedAttributes>,
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
impl TryFrom<X690Element> for SignerInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SignerInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SignerInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SignerInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SignerInfo,
            _eal_components_for_SignerInfo,
            _rctl2_components_for_SignerInfo,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let sid = _decode_SignerIdentifier(_components.get("sid").unwrap())?;
        let digestAlgorithm =
            _decode_DigestAlgorithmIdentifier(_components.get("digestAlgorithm").unwrap())?;
        let signedAttrs: OPTIONAL<SignedAttributes> = match _components.get("signedAttrs") {
            Some(c_) => Some(_decode_SignedAttributes(c_)?),
            _ => None,
        };
        let signatureAlgorithm =
            _decode_SignatureAlgorithmIdentifier(_components.get("signatureAlgorithm").unwrap())?;
        let signature = _decode_SignatureValue(_components.get("signature").unwrap())?;
        let unsignedAttrs: OPTIONAL<UnsignedAttributes> = match _components.get("unsignedAttrs") {
            Some(c_) => Some(_decode_UnsignedAttributes(c_)?),
            _ => None,
        };
        Ok(SignerInfo {
            version,
            sid,
            digestAlgorithm,
            signedAttrs,
            signatureAlgorithm,
            signature,
            unsignedAttrs,
        })
    }(&el)
}

pub fn _encode_SignerInfo(value_: &SignerInfo) -> ASN1Result<X690Element> {
    |value_: &SignerInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_SignerIdentifier(&value_.sid)?);
        components_.push(_encode_DigestAlgorithmIdentifier(&value_.digestAlgorithm)?);
        if let Some(v_) = &value_.signedAttrs {
            components_.push(|v_1: &SignedAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SignedAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_SignatureAlgorithmIdentifier(
            &value_.signatureAlgorithm,
        )?);
        components_.push(_encode_SignatureValue(&value_.signature)?);
        if let Some(v_) = &value_.unsignedAttrs {
            components_.push(|v_1: &UnsignedAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_UnsignedAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
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
/// SignerIdentifier  ::=  CHOICE {
///   issuerAndSerialNumber  IssuerAndSerialNumber,
///   subjectKeyIdentifier   [0]  SubjectKeyIdentifier
/// }
/// ```
#[derive(Debug, Clone)]
pub enum SignerIdentifier {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    subjectKeyIdentifier(SubjectKeyIdentifier),
}

impl TryFrom<X690Element> for SignerIdentifier {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SignerIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SignerIdentifier {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SignerIdentifier(el)
    }
}

pub fn _decode_SignerIdentifier(el: &X690Element) -> ASN1Result<SignerIdentifier> {
    |el: &X690Element| -> ASN1Result<SignerIdentifier> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(SignerIdentifier::issuerAndSerialNumber(
                _decode_IssuerAndSerialNumber(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(SignerIdentifier::subjectKeyIdentifier(
                _decode_SubjectKeyIdentifier(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_SignerIdentifier(value_: &SignerIdentifier) -> ASN1Result<X690Element> {
    |value: &SignerIdentifier| -> ASN1Result<X690Element> {
        match value {
            SignerIdentifier::issuerAndSerialNumber(v) => _encode_IssuerAndSerialNumber(&v),
            SignerIdentifier::subjectKeyIdentifier(v) => {
                |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedAttributes  ::=  SET SIZE (1..MAX) OF Attribute
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
/// UnsignedAttributes  ::=  SET SIZE (1..MAX) OF Attribute
/// ```
pub type UnsignedAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_UnsignedAttributes(el: &X690Element) -> ASN1Result<UnsignedAttributes> {
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

pub fn _encode_UnsignedAttributes(value_: &UnsignedAttributes) -> ASN1Result<X690Element> {
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
/// Attribute ::= SEQUENCE {
///   attrType    OBJECT IDENTIFIER,
///   attrValues  SET OF AttributeValue
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Attribute {
    pub attrType: OBJECT_IDENTIFIER,
    pub attrValues: Vec<AttributeValue>,
}
impl Attribute {
    pub fn new(attrType: OBJECT_IDENTIFIER, attrValues: Vec<AttributeValue>) -> Self {
        Attribute {
            attrType,
            attrValues,
        }
    }
}
impl TryFrom<X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Attribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Attribute,
            _eal_components_for_Attribute,
            _rctl2_components_for_Attribute,
        )?;
        let attrType = ber_decode_object_identifier(_components.get("attrType").unwrap())?;
        let attrValues = |el: &X690Element| -> ASN1Result<SET_OF<AttributeValue>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<AttributeValue> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_AttributeValue(el)?);
            }
            Ok(items)
        }(_components.get("attrValues").unwrap())?;
        Ok(Attribute {
            attrType,
            attrValues,
        })
    }(&el)
}

pub fn _encode_Attribute(value_: &Attribute) -> ASN1Result<X690Element> {
    |value_: &Attribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_object_identifier(&value_.attrType)?);
        components_.push(
            |value_: &SET_OF<AttributeValue>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_AttributeValue(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.attrValues)?,
        );
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
/// OPEN ::= CLASS {&Type
/// }WITH SYNTAX {TYPE &Type
/// }
/// ```
///
#[derive(Debug)]
pub struct OPEN {}
impl OPEN {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValue  ::=  OPEN.&Type
/// ```
pub type AttributeValue = X690Element; // ObjectClassFieldType

pub fn _decode_AttributeValue(el: &X690Element) -> ASN1Result<AttributeValue> {
    x690_identity(&el)
}

pub fn _encode_AttributeValue(value_: &AttributeValue) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureValue  ::=  OCTET STRING
/// ```
pub type SignatureValue = OCTET_STRING; // OctetStringType

pub fn _decode_SignatureValue(el: &X690Element) -> ASN1Result<SignatureValue> {
    ber_decode_octet_string(&el)
}

pub fn _encode_SignatureValue(value_: &SignatureValue) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnvelopedData ::= SEQUENCE {
///   version               CMSVersion,
///   originatorInfo        [0] IMPLICIT OriginatorInfo OPTIONAL,
///   recipientInfos        RecipientInfos,
///   encryptedContentInfo  EncryptedContentInfo,
///   unprotectedAttrs      [1] IMPLICIT UnprotectedAttributes OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EnvelopedData {
    pub version: CMSVersion,
    pub originatorInfo: OPTIONAL<OriginatorInfo>,
    pub recipientInfos: RecipientInfos,
    pub encryptedContentInfo: EncryptedContentInfo,
    pub unprotectedAttrs: OPTIONAL<UnprotectedAttributes>,
}
impl EnvelopedData {
    pub fn new(
        version: CMSVersion,
        originatorInfo: OPTIONAL<OriginatorInfo>,
        recipientInfos: RecipientInfos,
        encryptedContentInfo: EncryptedContentInfo,
        unprotectedAttrs: OPTIONAL<UnprotectedAttributes>,
    ) -> Self {
        EnvelopedData {
            version,
            originatorInfo,
            recipientInfos,
            encryptedContentInfo,
            unprotectedAttrs,
        }
    }
}
impl TryFrom<X690Element> for EnvelopedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EnvelopedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EnvelopedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EnvelopedData(el)
    }
}

pub const _rctl1_components_for_EnvelopedData: &[ComponentSpec; 5] = &[
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
    ComponentSpec::new(
        "unprotectedAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EnvelopedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EnvelopedData: &[ComponentSpec; 0] = &[];

pub fn _decode_EnvelopedData(el: &X690Element) -> ASN1Result<EnvelopedData> {
    |el_: &X690Element| -> ASN1Result<EnvelopedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EnvelopedData,
            _eal_components_for_EnvelopedData,
            _rctl2_components_for_EnvelopedData,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let originatorInfo: OPTIONAL<OriginatorInfo> = match _components.get("originatorInfo") {
            Some(c_) => Some(_decode_OriginatorInfo(c_)?),
            _ => None,
        };
        let recipientInfos = _decode_RecipientInfos(_components.get("recipientInfos").unwrap())?;
        let encryptedContentInfo =
            _decode_EncryptedContentInfo(_components.get("encryptedContentInfo").unwrap())?;
        let unprotectedAttrs: OPTIONAL<UnprotectedAttributes> =
            match _components.get("unprotectedAttrs") {
                Some(c_) => Some(_decode_UnprotectedAttributes(c_)?),
                _ => None,
            };
        Ok(EnvelopedData {
            version,
            originatorInfo,
            recipientInfos,
            encryptedContentInfo,
            unprotectedAttrs,
        })
    }(&el)
}

pub fn _encode_EnvelopedData(value_: &EnvelopedData) -> ASN1Result<X690Element> {
    |value_: &EnvelopedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(10);
        components_.push(_encode_CMSVersion(&value_.version)?);
        if let Some(v_) = &value_.originatorInfo {
            components_.push(|v_1: &OriginatorInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_OriginatorInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_RecipientInfos(&value_.recipientInfos)?);
        components_.push(_encode_EncryptedContentInfo(&value_.encryptedContentInfo)?);
        if let Some(v_) = &value_.unprotectedAttrs {
            components_.push(|v_1: &UnprotectedAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_UnprotectedAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
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
/// OriginatorInfo ::= SEQUENCE {
///   certs  [0] IMPLICIT CertificateSet OPTIONAL,
///   crls   [1] IMPLICIT CertificateRevocationLists OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct OriginatorInfo {
    pub certs: OPTIONAL<CertificateSet>,
    pub crls: OPTIONAL<CertificateRevocationLists>,
}
impl OriginatorInfo {
    pub fn new(
        certs: OPTIONAL<CertificateSet>,
        crls: OPTIONAL<CertificateRevocationLists>,
    ) -> Self {
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
impl TryFrom<X690Element> for OriginatorInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OriginatorInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<OriginatorInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_OriginatorInfo,
            _eal_components_for_OriginatorInfo,
            _rctl2_components_for_OriginatorInfo,
        )?;
        let certs: OPTIONAL<CertificateSet> = match _components.get("certs") {
            Some(c_) => Some(_decode_CertificateSet(c_)?),
            _ => None,
        };
        let crls: OPTIONAL<CertificateRevocationLists> = match _components.get("crls") {
            Some(c_) => Some(_decode_CertificateRevocationLists(c_)?),
            _ => None,
        };
        Ok(OriginatorInfo { certs, crls })
    }(&el)
}

pub fn _encode_OriginatorInfo(value_: &OriginatorInfo) -> ASN1Result<X690Element> {
    |value_: &OriginatorInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.certs {
            components_.push(|v_1: &CertificateSet| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSet(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.crls {
            components_.push(
                |v_1: &CertificateRevocationLists| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertificateRevocationLists(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
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
/// RecipientInfos  ::=  SET OF RecipientInfo
/// ```
pub type RecipientInfos = Vec<RecipientInfo>; // SetOfType

pub fn _decode_RecipientInfos(el: &X690Element) -> ASN1Result<RecipientInfos> {
    |el: &X690Element| -> ASN1Result<SET_OF<RecipientInfo>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<RecipientInfo> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_RecipientInfo(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RecipientInfos(value_: &RecipientInfos) -> ASN1Result<X690Element> {
    |value_: &SET_OF<RecipientInfo>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_RecipientInfo(&v)?);
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
/// EncryptedContentInfo ::= SEQUENCE {
///   contentType                 ContentType,
///   contentEncryptionAlgorithm  ContentEncryptionAlgorithmIdentifier,
///   encryptedContent            [0] IMPLICIT EncryptedContent OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncryptedContentInfo {
    pub contentType: ContentType,
    pub contentEncryptionAlgorithm: ContentEncryptionAlgorithmIdentifier,
    pub encryptedContent: OPTIONAL<EncryptedContent>,
}
impl EncryptedContentInfo {
    pub fn new(
        contentType: ContentType,
        contentEncryptionAlgorithm: ContentEncryptionAlgorithmIdentifier,
        encryptedContent: OPTIONAL<EncryptedContent>,
    ) -> Self {
        EncryptedContentInfo {
            contentType,
            contentEncryptionAlgorithm,
            encryptedContent,
        }
    }
}
impl TryFrom<X690Element> for EncryptedContentInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedContentInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncryptedContentInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedContentInfo(el)
    }
}

pub const _rctl1_components_for_EncryptedContentInfo: &[ComponentSpec; 3] = &[
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
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
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

pub const _rctl2_components_for_EncryptedContentInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedContentInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedContentInfo(el: &X690Element) -> ASN1Result<EncryptedContentInfo> {
    |el_: &X690Element| -> ASN1Result<EncryptedContentInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncryptedContentInfo,
            _eal_components_for_EncryptedContentInfo,
            _rctl2_components_for_EncryptedContentInfo,
        )?;
        let contentType = _decode_ContentType(_components.get("contentType").unwrap())?;
        let contentEncryptionAlgorithm = _decode_ContentEncryptionAlgorithmIdentifier(
            _components.get("contentEncryptionAlgorithm").unwrap(),
        )?;
        let encryptedContent: OPTIONAL<EncryptedContent> = match _components.get("encryptedContent")
        {
            Some(c_) => Some(_decode_EncryptedContent(c_)?),
            _ => None,
        };
        Ok(EncryptedContentInfo {
            contentType,
            contentEncryptionAlgorithm,
            encryptedContent,
        })
    }(&el)
}

pub fn _encode_EncryptedContentInfo(value_: &EncryptedContentInfo) -> ASN1Result<X690Element> {
    |value_: &EncryptedContentInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_ContentType(&value_.contentType)?);
        components_.push(_encode_ContentEncryptionAlgorithmIdentifier(
            &value_.contentEncryptionAlgorithm,
        )?);
        if let Some(v_) = &value_.encryptedContent {
            components_.push(|v_1: &EncryptedContent| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_EncryptedContent(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
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
/// EncryptedContent  ::=  OCTET STRING
/// ```
pub type EncryptedContent = OCTET_STRING; // OctetStringType

pub fn _decode_EncryptedContent(el: &X690Element) -> ASN1Result<EncryptedContent> {
    ber_decode_octet_string(&el)
}

pub fn _encode_EncryptedContent(value_: &EncryptedContent) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnprotectedAttributes  ::=  SET SIZE (1..MAX) OF Attribute
/// ```
pub type UnprotectedAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_UnprotectedAttributes(el: &X690Element) -> ASN1Result<UnprotectedAttributes> {
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

pub fn _encode_UnprotectedAttributes(value_: &UnprotectedAttributes) -> ASN1Result<X690Element> {
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
/// RecipientInfo  ::=  CHOICE {
///   ktri   KeyTransRecipientInfo,
///   kari   [1]  KeyAgreeRecipientInfo,
///   kekri  [2]  KEKRecipientInfo
/// }
/// ```
#[derive(Debug, Clone)]
pub enum RecipientInfo {
    ktri(KeyTransRecipientInfo),
    kari(KeyAgreeRecipientInfo),
    kekri(KEKRecipientInfo),
}

impl TryFrom<X690Element> for RecipientInfo {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RecipientInfo {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientInfo(el)
    }
}

pub fn _decode_RecipientInfo(el: &X690Element) -> ASN1Result<RecipientInfo> {
    |el: &X690Element| -> ASN1Result<RecipientInfo> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => {
                Ok(RecipientInfo::ktri(_decode_KeyTransRecipientInfo(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(RecipientInfo::kari(_decode_KeyAgreeRecipientInfo(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(RecipientInfo::kekri(_decode_KEKRecipientInfo(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_RecipientInfo(value_: &RecipientInfo) -> ASN1Result<X690Element> {
    |value: &RecipientInfo| -> ASN1Result<X690Element> {
        match value {
            RecipientInfo::ktri(v) => _encode_KeyTransRecipientInfo(&v),
            RecipientInfo::kari(v) => |v_1: &KeyAgreeRecipientInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_KeyAgreeRecipientInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            RecipientInfo::kekri(v) => |v_1: &KEKRecipientInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_KEKRecipientInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
        }
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
/// KeyTransRecipientInfo ::= SEQUENCE {
///   version                 CMSVersion, -- always set to 0 or 2
///   rid                     RecipientIdentifier,
///   keyEncryptionAlgorithm  KeyEncryptionAlgorithmIdentifier,
///   encryptedKey            EncryptedKey
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct KeyTransRecipientInfo {
    pub version: CMSVersion,
    pub rid: RecipientIdentifier,
    pub keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
    pub encryptedKey: EncryptedKey,
}
impl KeyTransRecipientInfo {
    pub fn new(
        version: CMSVersion,
        rid: RecipientIdentifier,
        keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
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
impl TryFrom<X690Element> for KeyTransRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KeyTransRecipientInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KeyTransRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<KeyTransRecipientInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_KeyTransRecipientInfo,
            _eal_components_for_KeyTransRecipientInfo,
            _rctl2_components_for_KeyTransRecipientInfo,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let rid = _decode_RecipientIdentifier(_components.get("rid").unwrap())?;
        let keyEncryptionAlgorithm = _decode_KeyEncryptionAlgorithmIdentifier(
            _components.get("keyEncryptionAlgorithm").unwrap(),
        )?;
        let encryptedKey = _decode_EncryptedKey(_components.get("encryptedKey").unwrap())?;
        Ok(KeyTransRecipientInfo {
            version,
            rid,
            keyEncryptionAlgorithm,
            encryptedKey,
        })
    }(&el)
}

pub fn _encode_KeyTransRecipientInfo(value_: &KeyTransRecipientInfo) -> ASN1Result<X690Element> {
    |value_: &KeyTransRecipientInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_RecipientIdentifier(&value_.rid)?);
        components_.push(_encode_KeyEncryptionAlgorithmIdentifier(
            &value_.keyEncryptionAlgorithm,
        )?);
        components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
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
/// RecipientIdentifier  ::=  CHOICE {
///   issuerAndSerialNumber  IssuerAndSerialNumber,
///   subjectKeyIdentifier   [0]  SubjectKeyIdentifier
/// }
/// ```
#[derive(Debug, Clone)]
pub enum RecipientIdentifier {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    subjectKeyIdentifier(SubjectKeyIdentifier),
}

impl TryFrom<X690Element> for RecipientIdentifier {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RecipientIdentifier {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientIdentifier(el)
    }
}

pub fn _decode_RecipientIdentifier(el: &X690Element) -> ASN1Result<RecipientIdentifier> {
    |el: &X690Element| -> ASN1Result<RecipientIdentifier> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(RecipientIdentifier::issuerAndSerialNumber(
                _decode_IssuerAndSerialNumber(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(RecipientIdentifier::subjectKeyIdentifier(
                _decode_SubjectKeyIdentifier(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_RecipientIdentifier(value_: &RecipientIdentifier) -> ASN1Result<X690Element> {
    |value: &RecipientIdentifier| -> ASN1Result<X690Element> {
        match value {
            RecipientIdentifier::issuerAndSerialNumber(v) => _encode_IssuerAndSerialNumber(&v),
            RecipientIdentifier::subjectKeyIdentifier(v) => {
                |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreeRecipientInfo ::= SEQUENCE {
///   version                 CMSVersion, -- always set to 3
///   originator              [0] EXPLICIT OriginatorIdentifierOrKey,
///   ukm                     [1] EXPLICIT UserKeyingMaterial OPTIONAL,
///   keyEncryptionAlgorithm  KeyEncryptionAlgorithmIdentifier,
///   recipientEncryptedKeys  RecipientEncryptedKeys
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct KeyAgreeRecipientInfo {
    pub version: CMSVersion,
    pub originator: OriginatorIdentifierOrKey,
    pub ukm: OPTIONAL<UserKeyingMaterial>,
    pub keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
    pub recipientEncryptedKeys: RecipientEncryptedKeys,
}
impl KeyAgreeRecipientInfo {
    pub fn new(
        version: CMSVersion,
        originator: OriginatorIdentifierOrKey,
        ukm: OPTIONAL<UserKeyingMaterial>,
        keyEncryptionAlgorithm: KeyEncryptionAlgorithmIdentifier,
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
impl TryFrom<X690Element> for KeyAgreeRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreeRecipientInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KeyAgreeRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<KeyAgreeRecipientInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_KeyAgreeRecipientInfo,
            _eal_components_for_KeyAgreeRecipientInfo,
            _rctl2_components_for_KeyAgreeRecipientInfo,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let originator = |el: &X690Element| -> ASN1Result<OriginatorIdentifierOrKey> {
            Ok(_decode_OriginatorIdentifierOrKey(&el.inner()?)?)
        }(_components.get("originator").unwrap())?;
        let ukm: OPTIONAL<UserKeyingMaterial> = match _components.get("ukm") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<UserKeyingMaterial> {
                Ok(_decode_UserKeyingMaterial(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let keyEncryptionAlgorithm = _decode_KeyEncryptionAlgorithmIdentifier(
            _components.get("keyEncryptionAlgorithm").unwrap(),
        )?;
        let recipientEncryptedKeys =
            _decode_RecipientEncryptedKeys(_components.get("recipientEncryptedKeys").unwrap())?;
        Ok(KeyAgreeRecipientInfo {
            version,
            originator,
            ukm,
            keyEncryptionAlgorithm,
            recipientEncryptedKeys,
        })
    }(&el)
}

pub fn _encode_KeyAgreeRecipientInfo(value_: &KeyAgreeRecipientInfo) -> ASN1Result<X690Element> {
    |value_: &KeyAgreeRecipientInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(10);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(
            |v_1: &OriginatorIdentifierOrKey| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_OriginatorIdentifierOrKey(&v_1)?,
                    ))),
                ))
            }(&value_.originator)?,
        );
        if let Some(v_) = &value_.ukm {
            components_.push(|v_1: &UserKeyingMaterial| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UserKeyingMaterial(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        components_.push(_encode_KeyEncryptionAlgorithmIdentifier(
            &value_.keyEncryptionAlgorithm,
        )?);
        components_.push(_encode_RecipientEncryptedKeys(
            &value_.recipientEncryptedKeys,
        )?);
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
/// OriginatorIdentifierOrKey  ::=  CHOICE {
///   issuerAndSerialNumber  IssuerAndSerialNumber,
///   subjectKeyIdentifier   [0]  SubjectKeyIdentifier,
///   originatorKey          [1]  OriginatorPublicKey
/// }
/// ```
#[derive(Debug, Clone)]
pub enum OriginatorIdentifierOrKey {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    subjectKeyIdentifier(SubjectKeyIdentifier),
    originatorKey(OriginatorPublicKey),
}

impl TryFrom<X690Element> for OriginatorIdentifierOrKey {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorIdentifierOrKey(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OriginatorIdentifierOrKey {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorIdentifierOrKey(el)
    }
}

pub fn _decode_OriginatorIdentifierOrKey(
    el: &X690Element,
) -> ASN1Result<OriginatorIdentifierOrKey> {
    |el: &X690Element| -> ASN1Result<OriginatorIdentifierOrKey> {
        match (el.tag_class, el.tag_number) {
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
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_OriginatorIdentifierOrKey(
    value_: &OriginatorIdentifierOrKey,
) -> ASN1Result<X690Element> {
    |value: &OriginatorIdentifierOrKey| -> ASN1Result<X690Element> {
        match value {
            OriginatorIdentifierOrKey::issuerAndSerialNumber(v) => {
                _encode_IssuerAndSerialNumber(&v)
            }
            OriginatorIdentifierOrKey::subjectKeyIdentifier(v) => {
                |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            OriginatorIdentifierOrKey::originatorKey(v) => {
                |v_1: &OriginatorPublicKey| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_OriginatorPublicKey(&v_1)?;
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
/// OriginatorPublicKey ::= SEQUENCE {
///   algorithm  AlgorithmIdentifier,
///   publicKey  BIT STRING
/// }
/// ```
///
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
impl TryFrom<X690Element> for OriginatorPublicKey {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OriginatorPublicKey(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OriginatorPublicKey {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<OriginatorPublicKey> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_OriginatorPublicKey,
            _eal_components_for_OriginatorPublicKey,
            _rctl2_components_for_OriginatorPublicKey,
        )?;
        let algorithm = _decode_AlgorithmIdentifier(_components.get("algorithm").unwrap())?;
        let publicKey = ber_decode_bit_string(_components.get("publicKey").unwrap())?;
        Ok(OriginatorPublicKey {
            algorithm,
            publicKey,
        })
    }(&el)
}

pub fn _encode_OriginatorPublicKey(value_: &OriginatorPublicKey) -> ASN1Result<X690Element> {
    |value_: &OriginatorPublicKey| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
        components_.push(ber_encode_bit_string(&value_.publicKey)?);
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
/// RecipientEncryptedKeys  ::=  SEQUENCE OF RecipientEncryptedKey
/// ```
pub type RecipientEncryptedKeys = Vec<RecipientEncryptedKey>; // SequenceOfType

pub fn _decode_RecipientEncryptedKeys(el: &X690Element) -> ASN1Result<RecipientEncryptedKeys> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<RecipientEncryptedKey>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<RecipientEncryptedKey> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_RecipientEncryptedKey(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RecipientEncryptedKeys(value_: &RecipientEncryptedKeys) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<RecipientEncryptedKey>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_RecipientEncryptedKey(&v)?);
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
/// RecipientEncryptedKey ::= SEQUENCE {
///   rid           KeyAgreeRecipientIdentifier,
///   encryptedKey  EncryptedKey
/// }
/// ```
///
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
impl TryFrom<X690Element> for RecipientEncryptedKey {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientEncryptedKey(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RecipientEncryptedKey {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<RecipientEncryptedKey> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RecipientEncryptedKey,
            _eal_components_for_RecipientEncryptedKey,
            _rctl2_components_for_RecipientEncryptedKey,
        )?;
        let rid = _decode_KeyAgreeRecipientIdentifier(_components.get("rid").unwrap())?;
        let encryptedKey = _decode_EncryptedKey(_components.get("encryptedKey").unwrap())?;
        Ok(RecipientEncryptedKey { rid, encryptedKey })
    }(&el)
}

pub fn _encode_RecipientEncryptedKey(value_: &RecipientEncryptedKey) -> ASN1Result<X690Element> {
    |value_: &RecipientEncryptedKey| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_KeyAgreeRecipientIdentifier(&value_.rid)?);
        components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
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
/// KeyAgreeRecipientIdentifier  ::=  CHOICE {
///   issuerAndSerialNumber  IssuerAndSerialNumber,
///   rKeyId                 [0] IMPLICIT RecipientKeyIdentifier
/// }
/// ```
#[derive(Debug, Clone)]
pub enum KeyAgreeRecipientIdentifier {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    rKeyId(RecipientKeyIdentifier),
}

impl TryFrom<X690Element> for KeyAgreeRecipientIdentifier {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreeRecipientIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KeyAgreeRecipientIdentifier {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_KeyAgreeRecipientIdentifier(el)
    }
}

pub fn _decode_KeyAgreeRecipientIdentifier(
    el: &X690Element,
) -> ASN1Result<KeyAgreeRecipientIdentifier> {
    |el: &X690Element| -> ASN1Result<KeyAgreeRecipientIdentifier> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(KeyAgreeRecipientIdentifier::issuerAndSerialNumber(
                _decode_IssuerAndSerialNumber(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(KeyAgreeRecipientIdentifier::rKeyId(
                _decode_RecipientKeyIdentifier(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_KeyAgreeRecipientIdentifier(
    value_: &KeyAgreeRecipientIdentifier,
) -> ASN1Result<X690Element> {
    |value: &KeyAgreeRecipientIdentifier| -> ASN1Result<X690Element> {
        match value {
            KeyAgreeRecipientIdentifier::issuerAndSerialNumber(v) => {
                _encode_IssuerAndSerialNumber(&v)
            }
            KeyAgreeRecipientIdentifier::rKeyId(v) => {
                |v_1: &RecipientKeyIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_RecipientKeyIdentifier(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RecipientKeyIdentifier ::= SEQUENCE {
///   subjectKeyIdentifier  SubjectKeyIdentifier,
///   date                  GeneralizedTime OPTIONAL,
///   other                 OtherKeyAttribute OPTIONAL
/// }
/// ```
///
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
impl TryFrom<X690Element> for RecipientKeyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RecipientKeyIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RecipientKeyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<RecipientKeyIdentifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RecipientKeyIdentifier,
            _eal_components_for_RecipientKeyIdentifier,
            _rctl2_components_for_RecipientKeyIdentifier,
        )?;
        let subjectKeyIdentifier =
            _decode_SubjectKeyIdentifier(_components.get("subjectKeyIdentifier").unwrap())?;
        let date: OPTIONAL<GeneralizedTime> = match _components.get("date") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let other: OPTIONAL<OtherKeyAttribute> = match _components.get("other") {
            Some(c_) => Some(_decode_OtherKeyAttribute(c_)?),
            _ => None,
        };
        Ok(RecipientKeyIdentifier {
            subjectKeyIdentifier,
            date,
            other,
        })
    }(&el)
}

pub fn _encode_RecipientKeyIdentifier(value_: &RecipientKeyIdentifier) -> ASN1Result<X690Element> {
    |value_: &RecipientKeyIdentifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_SubjectKeyIdentifier(&value_.subjectKeyIdentifier)?);
        if let Some(v_) = &value_.date {
            components_.push(ber_encode_generalized_time(&v_)?);
        }
        if let Some(v_) = &value_.other {
            components_.push(_encode_OtherKeyAttribute(&v_)?);
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
/// SubjectKeyIdentifier  ::=  OCTET STRING
/// ```
pub type SubjectKeyIdentifier = OCTET_STRING; // OctetStringType

pub fn _decode_SubjectKeyIdentifier(el: &X690Element) -> ASN1Result<SubjectKeyIdentifier> {
    ber_decode_octet_string(&el)
}

pub fn _encode_SubjectKeyIdentifier(value_: &SubjectKeyIdentifier) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEKRecipientInfo ::= SEQUENCE {
///   version                 CMSVersion, -- always set to 4
///   kekid                   KEKIdentifier,
///   keyEncryptionAlgorithm  KeyEncryptionAlgorithmIdentifier,
///   encryptedKey            EncryptedKey
/// }
/// ```
///
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
impl TryFrom<X690Element> for KEKRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KEKRecipientInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KEKRecipientInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<KEKRecipientInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_KEKRecipientInfo,
            _eal_components_for_KEKRecipientInfo,
            _rctl2_components_for_KEKRecipientInfo,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let kekid = _decode_KEKIdentifier(_components.get("kekid").unwrap())?;
        let keyEncryptionAlgorithm = _decode_KeyEncryptionAlgorithmIdentifier(
            _components.get("keyEncryptionAlgorithm").unwrap(),
        )?;
        let encryptedKey = _decode_EncryptedKey(_components.get("encryptedKey").unwrap())?;
        Ok(KEKRecipientInfo {
            version,
            kekid,
            keyEncryptionAlgorithm,
            encryptedKey,
        })
    }(&el)
}

pub fn _encode_KEKRecipientInfo(value_: &KEKRecipientInfo) -> ASN1Result<X690Element> {
    |value_: &KEKRecipientInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_KEKIdentifier(&value_.kekid)?);
        components_.push(_encode_KeyEncryptionAlgorithmIdentifier(
            &value_.keyEncryptionAlgorithm,
        )?);
        components_.push(_encode_EncryptedKey(&value_.encryptedKey)?);
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
/// KEKIdentifier ::= SEQUENCE {
///   keyIdentifier  OCTET STRING,
///   date           GeneralizedTime OPTIONAL,
///   other          OtherKeyAttribute OPTIONAL
/// }
/// ```
///
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
impl TryFrom<X690Element> for KEKIdentifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_KEKIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for KEKIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<KEKIdentifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_KEKIdentifier,
            _eal_components_for_KEKIdentifier,
            _rctl2_components_for_KEKIdentifier,
        )?;
        let keyIdentifier = ber_decode_octet_string(_components.get("keyIdentifier").unwrap())?;
        let date: OPTIONAL<GeneralizedTime> = match _components.get("date") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let other: OPTIONAL<OtherKeyAttribute> = match _components.get("other") {
            Some(c_) => Some(_decode_OtherKeyAttribute(c_)?),
            _ => None,
        };
        Ok(KEKIdentifier {
            keyIdentifier,
            date,
            other,
        })
    }(&el)
}

pub fn _encode_KEKIdentifier(value_: &KEKIdentifier) -> ASN1Result<X690Element> {
    |value_: &KEKIdentifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(ber_encode_octet_string(&value_.keyIdentifier)?);
        if let Some(v_) = &value_.date {
            components_.push(ber_encode_generalized_time(&v_)?);
        }
        if let Some(v_) = &value_.other {
            components_.push(_encode_OtherKeyAttribute(&v_)?);
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
/// DigestedData ::= SEQUENCE {
///   version           CMSVersion,
///   digestAlgorithm   DigestAlgorithmIdentifier,
///   encapContentInfo  EncapsulatedContentInfo,
///   digest            Digest
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DigestedData {
    pub version: CMSVersion,
    pub digestAlgorithm: DigestAlgorithmIdentifier,
    pub encapContentInfo: EncapsulatedContentInfo,
    pub digest: Digest,
}
impl DigestedData {
    pub fn new(
        version: CMSVersion,
        digestAlgorithm: DigestAlgorithmIdentifier,
        encapContentInfo: EncapsulatedContentInfo,
        digest: Digest,
    ) -> Self {
        DigestedData {
            version,
            digestAlgorithm,
            encapContentInfo,
            digest,
        }
    }
}
impl TryFrom<X690Element> for DigestedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DigestedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DigestedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<DigestedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DigestedData,
            _eal_components_for_DigestedData,
            _rctl2_components_for_DigestedData,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let digestAlgorithm =
            _decode_DigestAlgorithmIdentifier(_components.get("digestAlgorithm").unwrap())?;
        let encapContentInfo =
            _decode_EncapsulatedContentInfo(_components.get("encapContentInfo").unwrap())?;
        let digest = _decode_Digest(_components.get("digest").unwrap())?;
        Ok(DigestedData {
            version,
            digestAlgorithm,
            encapContentInfo,
            digest,
        })
    }(&el)
}

pub fn _encode_DigestedData(value_: &DigestedData) -> ASN1Result<X690Element> {
    |value_: &DigestedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_DigestAlgorithmIdentifier(&value_.digestAlgorithm)?);
        components_.push(_encode_EncapsulatedContentInfo(&value_.encapContentInfo)?);
        components_.push(_encode_Digest(&value_.digest)?);
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
/// Digest  ::=  OCTET STRING
/// ```
pub type Digest = OCTET_STRING; // OctetStringType

pub fn _decode_Digest(el: &X690Element) -> ASN1Result<Digest> {
    ber_decode_octet_string(&el)
}

pub fn _encode_Digest(value_: &Digest) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncryptedData ::= SEQUENCE {
///   version               CMSVersion,
///   encryptedContentInfo  EncryptedContentInfo,
///   unprotectedAttrs      [1] IMPLICIT UnprotectedAttributes OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub version: CMSVersion,
    pub encryptedContentInfo: EncryptedContentInfo,
    pub unprotectedAttrs: OPTIONAL<UnprotectedAttributes>,
}
impl EncryptedData {
    pub fn new(
        version: CMSVersion,
        encryptedContentInfo: EncryptedContentInfo,
        unprotectedAttrs: OPTIONAL<UnprotectedAttributes>,
    ) -> Self {
        EncryptedData {
            version,
            encryptedContentInfo,
            unprotectedAttrs,
        }
    }
}
impl TryFrom<X690Element> for EncryptedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EncryptedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EncryptedData(el)
    }
}

pub const _rctl1_components_for_EncryptedData: &[ComponentSpec; 3] = &[
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
    ComponentSpec::new(
        "unprotectedAttrs",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncryptedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncryptedData: &[ComponentSpec; 0] = &[];

pub fn _decode_EncryptedData(el: &X690Element) -> ASN1Result<EncryptedData> {
    |el_: &X690Element| -> ASN1Result<EncryptedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EncryptedData,
            _eal_components_for_EncryptedData,
            _rctl2_components_for_EncryptedData,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let encryptedContentInfo =
            _decode_EncryptedContentInfo(_components.get("encryptedContentInfo").unwrap())?;
        let unprotectedAttrs: OPTIONAL<UnprotectedAttributes> =
            match _components.get("unprotectedAttrs") {
                Some(c_) => Some(_decode_UnprotectedAttributes(c_)?),
                _ => None,
            };
        Ok(EncryptedData {
            version,
            encryptedContentInfo,
            unprotectedAttrs,
        })
    }(&el)
}

pub fn _encode_EncryptedData(value_: &EncryptedData) -> ASN1Result<X690Element> {
    |value_: &EncryptedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_EncryptedContentInfo(&value_.encryptedContentInfo)?);
        if let Some(v_) = &value_.unprotectedAttrs {
            components_.push(|v_1: &UnprotectedAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_UnprotectedAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
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
/// AuthenticatedData ::= SEQUENCE {
///   version                    CMSVersion,
///   originatorInfo             [0] IMPLICIT OriginatorInfo OPTIONAL,
///   recipientInfos             RecipientInfos,
///   macAlgorithm               MessageAuthenticationCodeAlgorithm,
///   digestAlgorithm            [1]  DigestAlgorithmIdentifier OPTIONAL,
///   encapContentInfo           EncapsulatedContentInfo,
///   authenticatedAttributes    [2] IMPLICIT AuthAttributes OPTIONAL,
///   mac                        MessageAuthenticationCode,
///   unauthenticatedAttributes  [3] IMPLICIT UnauthAttributes OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AuthenticatedData {
    pub version: CMSVersion,
    pub originatorInfo: OPTIONAL<OriginatorInfo>,
    pub recipientInfos: RecipientInfos,
    pub macAlgorithm: MessageAuthenticationCodeAlgorithm,
    pub digestAlgorithm: OPTIONAL<DigestAlgorithmIdentifier>,
    pub encapContentInfo: EncapsulatedContentInfo,
    pub authenticatedAttributes: OPTIONAL<AuthAttributes>,
    pub mac: MessageAuthenticationCode,
    pub unauthenticatedAttributes: OPTIONAL<UnauthAttributes>,
}
impl AuthenticatedData {
    pub fn new(
        version: CMSVersion,
        originatorInfo: OPTIONAL<OriginatorInfo>,
        recipientInfos: RecipientInfos,
        macAlgorithm: MessageAuthenticationCodeAlgorithm,
        digestAlgorithm: OPTIONAL<DigestAlgorithmIdentifier>,
        encapContentInfo: EncapsulatedContentInfo,
        authenticatedAttributes: OPTIONAL<AuthAttributes>,
        mac: MessageAuthenticationCode,
        unauthenticatedAttributes: OPTIONAL<UnauthAttributes>,
    ) -> Self {
        AuthenticatedData {
            version,
            originatorInfo,
            recipientInfos,
            macAlgorithm,
            digestAlgorithm,
            encapContentInfo,
            authenticatedAttributes,
            mac,
            unauthenticatedAttributes,
        }
    }
}
impl TryFrom<X690Element> for AuthenticatedData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticatedData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AuthenticatedData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
        "authenticatedAttributes",
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
        "unauthenticatedAttributes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AuthenticatedData: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AuthenticatedData: &[ComponentSpec; 0] = &[];

pub fn _decode_AuthenticatedData(el: &X690Element) -> ASN1Result<AuthenticatedData> {
    |el_: &X690Element| -> ASN1Result<AuthenticatedData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AuthenticatedData,
            _eal_components_for_AuthenticatedData,
            _rctl2_components_for_AuthenticatedData,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let originatorInfo: OPTIONAL<OriginatorInfo> = match _components.get("originatorInfo") {
            Some(c_) => Some(_decode_OriginatorInfo(c_)?),
            _ => None,
        };
        let recipientInfos = _decode_RecipientInfos(_components.get("recipientInfos").unwrap())?;
        let macAlgorithm =
            _decode_MessageAuthenticationCodeAlgorithm(_components.get("macAlgorithm").unwrap())?;
        let digestAlgorithm: OPTIONAL<DigestAlgorithmIdentifier> =
            match _components.get("digestAlgorithm") {
                Some(c_) => Some(_decode_DigestAlgorithmIdentifier(c_)?),
                _ => None,
            };
        let encapContentInfo =
            _decode_EncapsulatedContentInfo(_components.get("encapContentInfo").unwrap())?;
        let authenticatedAttributes: OPTIONAL<AuthAttributes> =
            match _components.get("authenticatedAttributes") {
                Some(c_) => Some(_decode_AuthAttributes(c_)?),
                _ => None,
            };
        let mac = _decode_MessageAuthenticationCode(_components.get("mac").unwrap())?;
        let unauthenticatedAttributes: OPTIONAL<UnauthAttributes> =
            match _components.get("unauthenticatedAttributes") {
                Some(c_) => Some(_decode_UnauthAttributes(c_)?),
                _ => None,
            };
        Ok(AuthenticatedData {
            version,
            originatorInfo,
            recipientInfos,
            macAlgorithm,
            digestAlgorithm,
            encapContentInfo,
            authenticatedAttributes,
            mac,
            unauthenticatedAttributes,
        })
    }(&el)
}

pub fn _encode_AuthenticatedData(value_: &AuthenticatedData) -> ASN1Result<X690Element> {
    |value_: &AuthenticatedData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_CMSVersion(&value_.version)?);
        if let Some(v_) = &value_.originatorInfo {
            components_.push(|v_1: &OriginatorInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_OriginatorInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
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
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?,
            );
        }
        components_.push(_encode_EncapsulatedContentInfo(&value_.encapContentInfo)?);
        if let Some(v_) = &value_.authenticatedAttributes {
            components_.push(|v_1: &AuthAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AuthAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_MessageAuthenticationCode(&value_.mac)?);
        if let Some(v_) = &value_.unauthenticatedAttributes {
            components_.push(|v_1: &UnauthAttributes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_UnauthAttributes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
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
/// AuthAttributes  ::=  SET SIZE (1..MAX) OF Attribute
/// ```
pub type AuthAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_AuthAttributes(el: &X690Element) -> ASN1Result<AuthAttributes> {
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

pub fn _encode_AuthAttributes(value_: &AuthAttributes) -> ASN1Result<X690Element> {
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
/// UnauthAttributes  ::=  SET SIZE (1..MAX) OF Attribute
/// ```
pub type UnauthAttributes = Vec<Attribute>; // SetOfType

pub fn _decode_UnauthAttributes(el: &X690Element) -> ASN1Result<UnauthAttributes> {
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

pub fn _encode_UnauthAttributes(value_: &UnauthAttributes) -> ASN1Result<X690Element> {
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
/// MessageAuthenticationCode  ::=  OCTET STRING
/// ```
pub type MessageAuthenticationCode = OCTET_STRING; // OctetStringType

pub fn _decode_MessageAuthenticationCode(
    el: &X690Element,
) -> ASN1Result<MessageAuthenticationCode> {
    ber_decode_octet_string(&el)
}

pub fn _encode_MessageAuthenticationCode(
    value_: &MessageAuthenticationCode,
) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DigestAlgorithmIdentifier  ::=  AlgorithmIdentifier
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureAlgorithmIdentifier  ::=  AlgorithmIdentifier
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyEncryptionAlgorithmIdentifier  ::=  AlgorithmIdentifier
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentEncryptionAlgorithmIdentifier  ::=  AlgorithmIdentifier
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageAuthenticationCodeAlgorithm  ::=  AlgorithmIdentifier
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateRevocationLists  ::=  SET OF CertificateList
/// ```
pub type CertificateRevocationLists = Vec<CertificateList>; // SetOfType

pub fn _decode_CertificateRevocationLists(
    el: &X690Element,
) -> ASN1Result<CertificateRevocationLists> {
    |el: &X690Element| -> ASN1Result<SET_OF<CertificateList>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<CertificateList> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertificateList(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertificateRevocationLists(
    value_: &CertificateRevocationLists,
) -> ASN1Result<X690Element> {
    |value_: &SET_OF<CertificateList>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertificateList(&v)?);
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
/// CertificateChoices  ::=  CHOICE {
///   certificate          Certificate, -- See X.509
///   extendedCertificate  [0] IMPLICIT ExtendedCertificate, -- Obsolete
///   attrCert             [1] IMPLICIT AttributeCertificate
/// }
/// ```
#[derive(Debug, Clone)]
pub enum CertificateChoices {
    certificate(Certificate),
    extendedCertificate(ExtendedCertificate),
    attrCert(AttributeCertificate),
}

impl TryFrom<X690Element> for CertificateChoices {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateChoices(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateChoices {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateChoices(el)
    }
}

pub fn _decode_CertificateChoices(el: &X690Element) -> ASN1Result<CertificateChoices> {
    |el: &X690Element| -> ASN1Result<CertificateChoices> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertificateChoices::extendedCertificate(
                _decode_ExtendedCertificate(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(CertificateChoices::attrCert(
                _decode_AttributeCertificate(&el)?,
            )),
            _ => Ok(CertificateChoices::certificate(_decode_Certificate(&el)?)),
        }
    }(&el)
}

pub fn _encode_CertificateChoices(value_: &CertificateChoices) -> ASN1Result<X690Element> {
    |value: &CertificateChoices| -> ASN1Result<X690Element> {
        match value {
            CertificateChoices::certificate(v) => _encode_Certificate(&v),
            CertificateChoices::extendedCertificate(v) => {
                |v_1: &ExtendedCertificate| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ExtendedCertificate(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            CertificateChoices::attrCert(v) => {
                |v_1: &AttributeCertificate| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeCertificate(&v_1)?;
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
/// CertificateSet  ::=  SET OF CertificateChoices
/// ```
pub type CertificateSet = Vec<CertificateChoices>; // SetOfType

pub fn _decode_CertificateSet(el: &X690Element) -> ASN1Result<CertificateSet> {
    |el: &X690Element| -> ASN1Result<SET_OF<CertificateChoices>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<CertificateChoices> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertificateChoices(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertificateSet(value_: &CertificateSet) -> ASN1Result<X690Element> {
    |value_: &SET_OF<CertificateChoices>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertificateChoices(&v)?);
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
/// IssuerAndSerialNumber ::= SEQUENCE {
///   issuer        Name,
///   serialNumber  CertificateSerialNumber
/// }
/// ```
///
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
impl TryFrom<X690Element> for IssuerAndSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerAndSerialNumber(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IssuerAndSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<IssuerAndSerialNumber> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IssuerAndSerialNumber,
            _eal_components_for_IssuerAndSerialNumber,
            _rctl2_components_for_IssuerAndSerialNumber,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(IssuerAndSerialNumber {
            issuer,
            serialNumber,
        })
    }(&el)
}

pub fn _encode_IssuerAndSerialNumber(value_: &IssuerAndSerialNumber) -> ASN1Result<X690Element> {
    |value_: &IssuerAndSerialNumber| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_Name(&value_.issuer)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
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
/// CMSVersion  ::=  INTEGER {v0(0), v1(1), v2(2), v3(3), v4(4)}
/// ```
pub type CMSVersion = INTEGER;

pub const CMSVersion_v0: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v1: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v2: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v3: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const CMSVersion_v4: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_CMSVersion(el: &X690Element) -> ASN1Result<CMSVersion> {
    ber_decode_integer(&el)
}

pub fn _encode_CMSVersion(value_: &CMSVersion) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserKeyingMaterial  ::=  OCTET STRING
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
/// OtherKeyAttribute ::= SEQUENCE {
///   keyAttributeIdentifier  OTHER-KEY-ATTRIBUTE.&id({OtherKeyAttributeTable}),
///   keyAttribute
///     OTHER-KEY-ATTRIBUTE.&Type
///       ({OtherKeyAttributeTable}{@keyAttributeIdentifier}) OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct OtherKeyAttribute {
    pub keyAttributeIdentifier: OBJECT_IDENTIFIER,
    pub keyAttribute: OPTIONAL<X690Element>,
}
impl OtherKeyAttribute {
    pub fn new(
        keyAttributeIdentifier: OBJECT_IDENTIFIER,
        keyAttribute: OPTIONAL<X690Element>,
    ) -> Self {
        OtherKeyAttribute {
            keyAttributeIdentifier,
            keyAttribute,
        }
    }
}
impl TryFrom<X690Element> for OtherKeyAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OtherKeyAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OtherKeyAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OtherKeyAttribute(el)
    }
}

pub const _rctl1_components_for_OtherKeyAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "keyAttributeIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("keyAttribute", true, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OtherKeyAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OtherKeyAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_OtherKeyAttribute(el: &X690Element) -> ASN1Result<OtherKeyAttribute> {
    |el_: &X690Element| -> ASN1Result<OtherKeyAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_OtherKeyAttribute,
            _eal_components_for_OtherKeyAttribute,
            _rctl2_components_for_OtherKeyAttribute,
        )?;
        let keyAttributeIdentifier =
            ber_decode_object_identifier(_components.get("keyAttributeIdentifier").unwrap())?;
        let keyAttribute: OPTIONAL<X690Element> = match _components.get("keyAttribute") {
            Some(c_) => Some(x690_identity(c_)?),
            _ => None,
        };
        Ok(OtherKeyAttribute {
            keyAttributeIdentifier,
            keyAttribute,
        })
    }(&el)
}

pub fn _encode_OtherKeyAttribute(value_: &OtherKeyAttribute) -> ASN1Result<X690Element> {
    |value_: &OtherKeyAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_object_identifier(
            &value_.keyAttributeIdentifier,
        )?);
        if let Some(v_) = &value_.keyAttribute {
            components_.push(x690_identity(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

pub type OTHER_KEY_ATTRIBUTE = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OtherKeyAttributeTable OTHER-KEY-ATTRIBUTE ::= {...}
/// ```
///
///
pub fn OtherKeyAttributeTable() -> Vec<OTHER_KEY_ATTRIBUTE> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageDigest  ::=  OCTET STRING
/// ```
pub type MessageDigest = OCTET_STRING; // OctetStringType

pub fn _decode_MessageDigest(el: &X690Element) -> ASN1Result<MessageDigest> {
    ber_decode_octet_string(&el)
}

pub fn _encode_MessageDigest(value_: &MessageDigest) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SigningTime  ::=  Time
/// ```
pub type SigningTime = Time; // DefinedType

pub fn _decode_SigningTime(el: &X690Element) -> ASN1Result<SigningTime> {
    _decode_Time(&el)
}

pub fn _encode_SigningTime(value_: &SigningTime) -> ASN1Result<X690Element> {
    _encode_Time(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Time  ::=  CHOICE {utcTime      UTCTime,
///                  generalTime  GeneralizedTime
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Time {
    utcTime(UTCTime),
    generalTime(GeneralizedTime),
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
            (TagClass::UNIVERSAL, 24) => Ok(Time::generalTime(ber_decode_generalized_time(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_Time(value_: &Time) -> ASN1Result<X690Element> {
    |value: &Time| -> ASN1Result<X690Element> {
        match value {
            Time::utcTime(v) => ber_encode_utc_time(&v),
            Time::generalTime(v) => ber_encode_generalized_time(&v),
        }
    }(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha-1 OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) oiw(14) secsig(3) algorithm(2) 26}
/// ```
///
///
pub fn sha_1() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* oiw */ 14,
        /* secsig */ 3, /* algorithm */ 2, 26,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// md5 OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) digestAlgorithm(2) 5}
/// ```
///
///
pub fn md5() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* digestAlgorithm */ 2, 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha1 OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) x9-57(10040) x9cm(4) 3}
/// ```
///
///
pub fn id_dsa_with_sha1() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* x9-57 */ 10040,
        /* x9cm */ 4, 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rsaEncryption OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-1(1) 1}
/// ```
///
///
pub fn rsaEncryption() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-1 */ 1, 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dh-public-number OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) ansi-x942(10046) number-type(2) 1}
/// ```
///
///
pub fn dh_public_number() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-x942 */ 10046,
        /* number-type */ 2, 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-ESDH OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16)
///    alg(3) 5}
/// ```
///
///
pub fn id_alg_ESDH() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-CMS3DESwrap OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16)
///    alg(3) 6}
/// ```
///
///
pub fn id_alg_CMS3DESwrap() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 6,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-CMSRC2wrap OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16)
///    alg(3) 7}
/// ```
///
///
pub fn id_alg_CMSRC2wrap() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 7,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// des-ede3-cbc OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) encryptionAlgorithm(3) 7}
/// ```
///
///
pub fn des_ede3_cbc() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* encryptionAlgorithm */ 3, 7,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rc2-cbc OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) encryptionAlgorithm(3) 2}
/// ```
///
///
pub fn rc2_cbc() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* encryptionAlgorithm */ 3, 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hMAC-SHA1 OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    mechanisms(5) 8 1 2}
/// ```
///
///
pub fn hMAC_SHA1() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* security */ 5, /* mechanisms */ 5, 8, 1, 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyWrapAlgorithm  ::=  AlgorithmIdentifier
/// ```
pub type KeyWrapAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_KeyWrapAlgorithm(el: &X690Element) -> ASN1Result<KeyWrapAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_KeyWrapAlgorithm(value_: &KeyWrapAlgorithm) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RC2wrapParameter  ::=  RC2ParameterVersion
/// ```
pub type RC2wrapParameter = RC2ParameterVersion; // DefinedType

pub fn _decode_RC2wrapParameter(el: &X690Element) -> ASN1Result<RC2wrapParameter> {
    _decode_RC2ParameterVersion(&el)
}

pub fn _encode_RC2wrapParameter(value_: &RC2wrapParameter) -> ASN1Result<X690Element> {
    _encode_RC2ParameterVersion(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RC2ParameterVersion  ::=  INTEGER
/// ```
pub type RC2ParameterVersion = INTEGER;

pub fn _decode_RC2ParameterVersion(el: &X690Element) -> ASN1Result<RC2ParameterVersion> {
    ber_decode_integer(&el)
}

pub fn _encode_RC2ParameterVersion(value_: &RC2ParameterVersion) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CBCParameter  ::=  IV
/// ```
pub type CBCParameter = IV; // DefinedType

pub fn _decode_CBCParameter(el: &X690Element) -> ASN1Result<CBCParameter> {
    _decode_IV(&el)
}

pub fn _encode_CBCParameter(value_: &CBCParameter) -> ASN1Result<X690Element> {
    _encode_IV(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IV  ::=  OCTET STRING
/// ```
pub type IV = OCTET_STRING; // OctetStringType

pub fn _decode_IV(el: &X690Element) -> ASN1Result<IV> {
    ber_decode_octet_string(&el)
}

pub fn _encode_IV(value_: &IV) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RC2CBCParameter ::= SEQUENCE {
///   rc2ParameterVersion  INTEGER,
///   iv                   OCTET STRING
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RC2CBCParameter {
    pub rc2ParameterVersion: INTEGER,
    pub iv: OCTET_STRING,
}
impl RC2CBCParameter {
    pub fn new(rc2ParameterVersion: INTEGER, iv: OCTET_STRING) -> Self {
        RC2CBCParameter {
            rc2ParameterVersion,
            iv,
        }
    }
}
impl TryFrom<X690Element> for RC2CBCParameter {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RC2CBCParameter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RC2CBCParameter {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RC2CBCParameter(el)
    }
}

pub const _rctl1_components_for_RC2CBCParameter: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "rc2ParameterVersion",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "iv",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RC2CBCParameter: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RC2CBCParameter: &[ComponentSpec; 0] = &[];

pub fn _decode_RC2CBCParameter(el: &X690Element) -> ASN1Result<RC2CBCParameter> {
    |el_: &X690Element| -> ASN1Result<RC2CBCParameter> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RC2CBCParameter,
            _eal_components_for_RC2CBCParameter,
            _rctl2_components_for_RC2CBCParameter,
        )?;
        let rc2ParameterVersion =
            ber_decode_integer(_components.get("rc2ParameterVersion").unwrap())?;
        let iv = ber_decode_octet_string(_components.get("iv").unwrap())?;
        Ok(RC2CBCParameter {
            rc2ParameterVersion,
            iv,
        })
    }(&el)
}

pub fn _encode_RC2CBCParameter(value_: &RC2CBCParameter) -> ASN1Result<X690Element> {
    |value_: &RC2CBCParameter| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_integer(&value_.rc2ParameterVersion)?);
        components_.push(ber_encode_octet_string(&value_.iv)?);
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
/// id-ct-contentInfo OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16)
///    ct(1) 6}
/// ```
///
///
pub fn id_ct_contentInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1, 6,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-data OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 1}
/// ```
///
///
pub fn id_data() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signedData OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 2}
/// ```
///
///
pub fn id_signedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-envelopedData OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 3}
/// ```
///
///
pub fn id_envelopedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-digestedData OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 5}
/// ```
///
///
pub fn id_digestedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-encryptedData OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 6}
/// ```
///
///
pub fn id_encryptedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 6,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-authData OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16)
///    ct(1) 2}
/// ```
///
///
pub fn id_ct_authData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1, 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contentType OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 3}
/// ```
///
///
pub fn id_contentType() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-messageDigest OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 4}
/// ```
///
///
pub fn id_messageDigest() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 4,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signingTime OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 5}
/// ```
///
///
pub fn id_signingTime() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-countersignature OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) 6}
/// ```
///
///
pub fn id_countersignature() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 6,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedCertificate ::= SEQUENCE {
///   extendedCertificateInfo  ExtendedCertificateInfo,
///   signatureAlgorithm       SignatureAlgorithmIdentifier,
///   signature                Signature
/// }
/// ```
///
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
impl TryFrom<X690Element> for ExtendedCertificate {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedCertificate(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtendedCertificate {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ExtendedCertificate> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtendedCertificate,
            _eal_components_for_ExtendedCertificate,
            _rctl2_components_for_ExtendedCertificate,
        )?;
        let extendedCertificateInfo =
            _decode_ExtendedCertificateInfo(_components.get("extendedCertificateInfo").unwrap())?;
        let signatureAlgorithm =
            _decode_SignatureAlgorithmIdentifier(_components.get("signatureAlgorithm").unwrap())?;
        let signature = _decode_Signature(_components.get("signature").unwrap())?;
        Ok(ExtendedCertificate {
            extendedCertificateInfo,
            signatureAlgorithm,
            signature,
        })
    }(&el)
}

pub fn _encode_ExtendedCertificate(value_: &ExtendedCertificate) -> ASN1Result<X690Element> {
    |value_: &ExtendedCertificate| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_ExtendedCertificateInfo(
            &value_.extendedCertificateInfo,
        )?);
        components_.push(_encode_SignatureAlgorithmIdentifier(
            &value_.signatureAlgorithm,
        )?);
        components_.push(_encode_Signature(&value_.signature)?);
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
/// ExtendedCertificateInfo ::= SEQUENCE {
///   version      CMSVersion,
///   certificate  Certificate,
///   attributes   UnauthAttributes
/// }
/// ```
///
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
impl TryFrom<X690Element> for ExtendedCertificateInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedCertificateInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtendedCertificateInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ExtendedCertificateInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtendedCertificateInfo,
            _eal_components_for_ExtendedCertificateInfo,
            _rctl2_components_for_ExtendedCertificateInfo,
        )?;
        let version = _decode_CMSVersion(_components.get("version").unwrap())?;
        let certificate = _decode_Certificate(_components.get("certificate").unwrap())?;
        let attributes = _decode_UnauthAttributes(_components.get("attributes").unwrap())?;
        Ok(ExtendedCertificateInfo {
            version,
            certificate,
            attributes,
        })
    }(&el)
}

pub fn _encode_ExtendedCertificateInfo(
    value_: &ExtendedCertificateInfo,
) -> ASN1Result<X690Element> {
    |value_: &ExtendedCertificateInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_CMSVersion(&value_.version)?);
        components_.push(_encode_Certificate(&value_.certificate)?);
        components_.push(_encode_UnauthAttributes(&value_.attributes)?);
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
/// Signature  ::=  BIT STRING
/// ```
pub type Signature = BIT_STRING;

pub fn _decode_Signature(el: &X690Element) -> ASN1Result<Signature> {
    ber_decode_bit_string(&el)
}

pub fn _encode_Signature(value_: &Signature) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}
