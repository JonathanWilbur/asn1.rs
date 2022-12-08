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
use std::borrow::Borrow;
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
impl TryFrom<X690Element> for SPKM_REQ {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_REQ(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_REQ {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_REQ> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_REQ,
            _eal_components_for_SPKM_REQ,
            _rctl2_components_for_SPKM_REQ,
        )?;
        let requestToken = _decode_REQ_TOKEN(_components.get("requestToken").unwrap())?;
        let certif_data: OPTIONAL<CertificationData> = match _components.get("certif-data") {
            Some(c_) => Some(_decode_CertificationData(c_)?),
            _ => None,
        };
        let auth_data: OPTIONAL<AuthorizationData> = match _components.get("auth-data") {
            Some(c_) => Some(_decode_AuthorizationData(c_)?),
            _ => None,
        };
        Ok(SPKM_REQ {
            requestToken,
            certif_data,
            auth_data,
        })
    }(&el)
}

pub fn _encode_SPKM_REQ(value_: &SPKM_REQ) -> ASN1Result<X690Element> {
    |value_: &SPKM_REQ| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_REQ_TOKEN(&value_.requestToken)?);
        if let Some(v_) = &value_.certif_data {
            components_.push(|v_1: &CertificationData| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificationData(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.auth_data {
            components_.push(|v_1: &AuthorizationData| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AuthorizationData(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for CertificationData {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificationData(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificationData {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<CertificationData> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificationData,
            _eal_components_for_CertificationData,
            _rctl2_components_for_CertificationData,
        )?;
        let certificationPath: OPTIONAL<CertificationPath> =
            match _components.get("certificationPath") {
                Some(c_) => Some(_decode_CertificationPath(c_)?),
                _ => None,
            };
        let certificateRevocationList: OPTIONAL<CertificateList> =
            match _components.get("certificateRevocationList") {
                Some(c_) => Some(_decode_CertificateList(c_)?),
                _ => None,
            };
        Ok(CertificationData {
            certificationPath,
            certificateRevocationList,
        })
    }(&el)
}

pub fn _encode_CertificationData(value_: &CertificationData) -> ASN1Result<X690Element> {
    |value_: &CertificationData| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.certificationPath {
            components_.push(|v_1: &CertificationPath| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificationPath(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.certificateRevocationList {
            components_.push(|v_1: &CertificateList| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateList(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for CertificationPath {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificationPath(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificationPath {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<CertificationPath> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificationPath,
            _eal_components_for_CertificationPath,
            _rctl2_components_for_CertificationPath,
        )?;
        let userKeyId: OPTIONAL<OCTET_STRING> = match _components.get("userKeyId") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        let userCertif: OPTIONAL<Certificate> = match _components.get("userCertif") {
            Some(c_) => Some(_decode_Certificate(c_)?),
            _ => None,
        };
        let verifKeyId: OPTIONAL<OCTET_STRING> = match _components.get("verifKeyId") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        let userVerifCertif: OPTIONAL<Certificate> = match _components.get("userVerifCertif") {
            Some(c_) => Some(_decode_Certificate(c_)?),
            _ => None,
        };
        let theCACertificates: OPTIONAL<Vec<CertificatePair>> =
            match _components.get("theCACertificates") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertificatePair>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<CertificatePair> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_CertificatePair(el)?);
                        }
                        Ok(items)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(CertificationPath {
            userKeyId,
            userCertif,
            verifKeyId,
            userVerifCertif,
            theCACertificates,
        })
    }(&el)
}

pub fn _encode_CertificationPath(value_: &CertificationPath) -> ASN1Result<X690Element> {
    |value_: &CertificationPath| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(10);
        if let Some(v_) = &value_.userKeyId {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_octet_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.userCertif {
            components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Certificate(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.verifKeyId {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_octet_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.userVerifCertif {
            components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Certificate(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
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
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for REQ_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_REQ_TOKEN(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for REQ_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<REQ_TOKEN> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_REQ_TOKEN,
            _eal_components_for_REQ_TOKEN,
            _rctl2_components_for_REQ_TOKEN,
        )?;
        let req_contents = _decode_Req_contents(_components.get("req-contents").unwrap())?;
        let algId = _decode_AlgorithmIdentifier(_components.get("algId").unwrap())?;
        let req_integrity = _decode_Integrity(_components.get("req-integrity").unwrap())?;
        Ok(REQ_TOKEN {
            req_contents,
            algId,
            req_integrity,
        })
    }(&el)
}

pub fn _encode_REQ_TOKEN(value_: &REQ_TOKEN) -> ASN1Result<X690Element> {
    |value_: &REQ_TOKEN| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_Req_contents(&value_.req_contents)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
        components_.push(_encode_Integrity(&value_.req_integrity)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Integrity  ::=  BIT STRING
/// ```
pub type Integrity = BIT_STRING;

pub fn _decode_Integrity(el: &X690Element) -> ASN1Result<Integrity> {
    ber_decode_bit_string(&el)
}

pub fn _encode_Integrity(value_: &Integrity) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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
impl TryFrom<X690Element> for Req_contents {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Req_contents(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Req_contents {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Req_contents> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Req_contents,
            _eal_components_for_Req_contents,
            _rctl2_components_for_Req_contents,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        let pvno = ber_decode_bit_string(_components.get("pvno").unwrap())?;
        let timestamp: OPTIONAL<UTCTime> = match _components.get("timestamp") {
            Some(c_) => Some(ber_decode_utc_time(c_)?),
            _ => None,
        };
        let randSrc = _decode_Random_Integer(_components.get("randSrc").unwrap())?;
        let targ_name = _decode_Name(_components.get("targ-name").unwrap())?;
        let src_name: OPTIONAL<Name> = match _components.get("src-name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let req_data = _decode_Context_Data(_components.get("req-data").unwrap())?;
        let validity: OPTIONAL<Validity> = match _components.get("validity") {
            Some(c_) => Some(_decode_Validity(c_)?),
            _ => None,
        };
        let key_estb_set = _decode_Key_Estb_Algs(_components.get("key-estb-set").unwrap())?;
        let key_estb_req: OPTIONAL<BIT_STRING> = match _components.get("key-estb-req") {
            Some(c_) => Some(ber_decode_bit_string(c_)?),
            _ => None,
        };
        let key_src_bind: OPTIONAL<OCTET_STRING> = match _components.get("key-src-bind") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(Req_contents {
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
        })
    }(&el)
}

pub fn _encode_Req_contents(value_: &Req_contents) -> ASN1Result<X690Element> {
    |value_: &Req_contents| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        components_.push(ber_encode_bit_string(&value_.pvno)?);
        if let Some(v_) = &value_.timestamp {
            components_.push(ber_encode_utc_time(&v_)?);
        }
        components_.push(_encode_Random_Integer(&value_.randSrc)?);
        components_.push(_encode_Name(&value_.targ_name)?);
        if let Some(v_) = &value_.src_name {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(_encode_Context_Data(&value_.req_data)?);
        if let Some(v_) = &value_.validity {
            components_.push(|v_1: &Validity| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Validity(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_Key_Estb_Algs(&value_.key_estb_set)?);
        if let Some(v_) = &value_.key_estb_req {
            components_.push(ber_encode_bit_string(&v_)?);
        }
        if let Some(v_) = &value_.key_src_bind {
            components_.push(ber_encode_octet_string(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Random-Integer  ::=  BIT STRING
/// ```
pub type Random_Integer = BIT_STRING;

pub fn _decode_Random_Integer(el: &X690Element) -> ASN1Result<Random_Integer> {
    ber_decode_bit_string(&el)
}

pub fn _encode_Random_Integer(value_: &Random_Integer) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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
impl TryFrom<X690Element> for Context_Data {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Context_Data(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Context_Data {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Context_Data> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Context_Data,
            _eal_components_for_Context_Data,
            _rctl2_components_for_Context_Data,
        )?;
        let channelId: OPTIONAL<ChannelId> = match _components.get("channelId") {
            Some(c_) => Some(_decode_ChannelId(c_)?),
            _ => None,
        };
        let seq_number: OPTIONAL<INTEGER> = match _components.get("seq-number") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let options = _decode_Options(_components.get("options").unwrap())?;
        let conf_alg = _decode_Conf_Algs(_components.get("conf-alg").unwrap())?;
        let intg_alg = _decode_Intg_Algs(_components.get("intg-alg").unwrap())?;
        let owf_alg = _decode_OWF_Algs(_components.get("owf-alg").unwrap())?;
        Ok(Context_Data {
            channelId,
            seq_number,
            options,
            conf_alg,
            intg_alg,
            owf_alg,
        })
    }(&el)
}

pub fn _encode_Context_Data(value_: &Context_Data) -> ASN1Result<X690Element> {
    |value_: &Context_Data| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        if let Some(v_) = &value_.channelId {
            components_.push(_encode_ChannelId(&v_)?);
        }
        if let Some(v_) = &value_.seq_number {
            components_.push(ber_encode_integer(&v_)?);
        }
        components_.push(_encode_Options(&value_.options)?);
        components_.push(_encode_Conf_Algs(&value_.conf_alg)?);
        components_.push(_encode_Intg_Algs(&value_.intg_alg)?);
        components_.push(_encode_OWF_Algs(&value_.owf_alg)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChannelId  ::=  OCTET STRING
/// ```
pub type ChannelId = OCTET_STRING; // OctetStringType

pub fn _decode_ChannelId(el: &X690Element) -> ASN1Result<ChannelId> {
    ber_decode_octet_string(&el)
}

pub fn _encode_ChannelId(value_: &ChannelId) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_Options(value_: &Options) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

impl TryFrom<X690Element> for Conf_Algs {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Conf_Algs(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Conf_Algs {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Conf_Algs(el)
    }
}

pub fn _decode_Conf_Algs(el: &X690Element) -> ASN1Result<Conf_Algs> {
    |el: &X690Element| -> ASN1Result<Conf_Algs> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Conf_Algs::algs(|el: &X690Element| -> ASN1Result<
                SEQUENCE_OF<AlgorithmIdentifier>,
            > {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<AlgorithmIdentifier> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_AlgorithmIdentifier(el)?);
                }
                Ok(items)
            }(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(Conf_Algs::null(ber_decode_null(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_Conf_Algs(value_: &Conf_Algs) -> ASN1Result<X690Element> {
    |value: &Conf_Algs| -> ASN1Result<X690Element> {
        match value {
            Conf_Algs::algs(v) => |v_1: &Vec<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
                let mut el_1 =
                    |value_: &SEQUENCE_OF<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AlgorithmIdentifier(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            Conf_Algs::null(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_null(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Intg-Algs  ::=  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type Intg_Algs = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_Intg_Algs(el: &X690Element) -> ASN1Result<Intg_Algs> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AlgorithmIdentifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Intg_Algs(value_: &Intg_Algs) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AlgorithmIdentifier(&v)?);
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
/// OWF-Algs  ::=  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type OWF_Algs = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_OWF_Algs(el: &X690Element) -> ASN1Result<OWF_Algs> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AlgorithmIdentifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_OWF_Algs(value_: &OWF_Algs) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AlgorithmIdentifier(&v)?);
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
/// Key-Estb-Algs  ::=  SEQUENCE OF AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type Key_Estb_Algs = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_Key_Estb_Algs(el: &X690Element) -> ASN1Result<Key_Estb_Algs> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AlgorithmIdentifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Key_Estb_Algs(value_: &Key_Estb_Algs) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AlgorithmIdentifier(&v)?);
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
/// SPKM-REP-TI ::= SEQUENCE {
///   responseToken  REP-TI-TOKEN,
///   certif-data    CertificationData OPTIONAL
///   -- present if target-certif-data-required option was
/// }
/// ```
///
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
impl TryFrom<X690Element> for SPKM_REP_TI {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_REP_TI(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_REP_TI {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_REP_TI> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_REP_TI,
            _eal_components_for_SPKM_REP_TI,
            _rctl2_components_for_SPKM_REP_TI,
        )?;
        let responseToken = _decode_REP_TI_TOKEN(_components.get("responseToken").unwrap())?;
        let certif_data: OPTIONAL<CertificationData> = match _components.get("certif-data") {
            Some(c_) => Some(_decode_CertificationData(c_)?),
            _ => None,
        };
        Ok(SPKM_REP_TI {
            responseToken,
            certif_data,
        })
    }(&el)
}

pub fn _encode_SPKM_REP_TI(value_: &SPKM_REP_TI) -> ASN1Result<X690Element> {
    |value_: &SPKM_REP_TI| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_REP_TI_TOKEN(&value_.responseToken)?);
        if let Some(v_) = &value_.certif_data {
            components_.push(_encode_CertificationData(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for REP_TI_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_REP_TI_TOKEN(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for REP_TI_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<REP_TI_TOKEN> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_REP_TI_TOKEN,
            _eal_components_for_REP_TI_TOKEN,
            _rctl2_components_for_REP_TI_TOKEN,
        )?;
        let rep_ti_contents = _decode_Rep_ti_contents(_components.get("rep-ti-contents").unwrap())?;
        let algId = _decode_AlgorithmIdentifier(_components.get("algId").unwrap())?;
        let rep_ti_integ = _decode_Integrity(_components.get("rep-ti-integ").unwrap())?;
        Ok(REP_TI_TOKEN {
            rep_ti_contents,
            algId,
            rep_ti_integ,
        })
    }(&el)
}

pub fn _encode_REP_TI_TOKEN(value_: &REP_TI_TOKEN) -> ASN1Result<X690Element> {
    |value_: &REP_TI_TOKEN| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_Rep_ti_contents(&value_.rep_ti_contents)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
        components_.push(_encode_Integrity(&value_.rep_ti_integ)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for Rep_ti_contents {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Rep_ti_contents(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Rep_ti_contents {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Rep_ti_contents> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Rep_ti_contents,
            _eal_components_for_Rep_ti_contents,
            _rctl2_components_for_Rep_ti_contents,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        let pvno: OPTIONAL<BIT_STRING> = match _components.get("pvno") {
            Some(c_) => Some(ber_decode_bit_string(c_)?),
            _ => None,
        };
        let timestamp: OPTIONAL<UTCTime> = match _components.get("timestamp") {
            Some(c_) => Some(ber_decode_utc_time(c_)?),
            _ => None,
        };
        let randTarg = _decode_Random_Integer(_components.get("randTarg").unwrap())?;
        let src_name: OPTIONAL<Name> = match _components.get("src-name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let targ_name = _decode_Name(_components.get("targ-name").unwrap())?;
        let randSrc = _decode_Random_Integer(_components.get("randSrc").unwrap())?;
        let rep_data = _decode_Context_Data(_components.get("rep-data").unwrap())?;
        let validity: OPTIONAL<Validity> = match _components.get("validity") {
            Some(c_) => Some(_decode_Validity(c_)?),
            _ => None,
        };
        let key_estb_id: OPTIONAL<AlgorithmIdentifier> = match _components.get("key-estb-id") {
            Some(c_) => Some(_decode_AlgorithmIdentifier(c_)?),
            _ => None,
        };
        let key_estb_str: OPTIONAL<BIT_STRING> = match _components.get("key-estb-str") {
            Some(c_) => Some(ber_decode_bit_string(c_)?),
            _ => None,
        };
        Ok(Rep_ti_contents {
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
        })
    }(&el)
}

pub fn _encode_Rep_ti_contents(value_: &Rep_ti_contents) -> ASN1Result<X690Element> {
    |value_: &Rep_ti_contents| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        if let Some(v_) = &value_.pvno {
            components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_bit_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.timestamp {
            components_.push(ber_encode_utc_time(&v_)?);
        }
        components_.push(_encode_Random_Integer(&value_.randTarg)?);
        if let Some(v_) = &value_.src_name {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(_encode_Name(&value_.targ_name)?);
        components_.push(_encode_Random_Integer(&value_.randSrc)?);
        components_.push(_encode_Context_Data(&value_.rep_data)?);
        if let Some(v_) = &value_.validity {
            components_.push(|v_1: &Validity| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Validity(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.key_estb_id {
            components_.push(_encode_AlgorithmIdentifier(&v_)?);
        }
        if let Some(v_) = &value_.key_estb_str {
            components_.push(ber_encode_bit_string(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for SPKM_REP_IT {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_REP_IT(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_REP_IT {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_REP_IT> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_REP_IT,
            _eal_components_for_SPKM_REP_IT,
            _rctl2_components_for_SPKM_REP_IT,
        )?;
        let responseToken = _decode_REP_IT_TOKEN(_components.get("responseToken").unwrap())?;
        let algId = _decode_AlgorithmIdentifier(_components.get("algId").unwrap())?;
        let rep_it_integ = _decode_Integrity(_components.get("rep-it-integ").unwrap())?;
        Ok(SPKM_REP_IT {
            responseToken,
            algId,
            rep_it_integ,
        })
    }(&el)
}

pub fn _encode_SPKM_REP_IT(value_: &SPKM_REP_IT) -> ASN1Result<X690Element> {
    |value_: &SPKM_REP_IT| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_REP_IT_TOKEN(&value_.responseToken)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
        components_.push(_encode_Integrity(&value_.rep_it_integ)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for REP_IT_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_REP_IT_TOKEN(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for REP_IT_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<REP_IT_TOKEN> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_REP_IT_TOKEN,
            _eal_components_for_REP_IT_TOKEN,
            _rctl2_components_for_REP_IT_TOKEN,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        let randSrc = _decode_Random_Integer(_components.get("randSrc").unwrap())?;
        let randTarg = _decode_Random_Integer(_components.get("randTarg").unwrap())?;
        let targ_name = _decode_Name(_components.get("targ-name").unwrap())?;
        let src_name: OPTIONAL<Name> = match _components.get("src-name") {
            Some(c_) => Some(_decode_Name(c_)?),
            _ => None,
        };
        let key_estb_rep: OPTIONAL<BIT_STRING> = match _components.get("key-estb-rep") {
            Some(c_) => Some(ber_decode_bit_string(c_)?),
            _ => None,
        };
        Ok(REP_IT_TOKEN {
            tok_id,
            context_id,
            randSrc,
            randTarg,
            targ_name,
            src_name,
            key_estb_rep,
        })
    }(&el)
}

pub fn _encode_REP_IT_TOKEN(value_: &REP_IT_TOKEN) -> ASN1Result<X690Element> {
    |value_: &REP_IT_TOKEN| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        components_.push(_encode_Random_Integer(&value_.randSrc)?);
        components_.push(_encode_Random_Integer(&value_.randTarg)?);
        components_.push(_encode_Name(&value_.targ_name)?);
        if let Some(v_) = &value_.src_name {
            components_.push(_encode_Name(&v_)?);
        }
        if let Some(v_) = &value_.key_estb_rep {
            components_.push(ber_encode_bit_string(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for SPKM_ERROR {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_ERROR(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_ERROR {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_ERROR> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_ERROR,
            _eal_components_for_SPKM_ERROR,
            _rctl2_components_for_SPKM_ERROR,
        )?;
        let errorToken = _decode_ERROR_TOKEN(_components.get("errorToken").unwrap())?;
        let algId = _decode_AlgorithmIdentifier(_components.get("algId").unwrap())?;
        let integrity = _decode_Integrity(_components.get("integrity").unwrap())?;
        Ok(SPKM_ERROR {
            errorToken,
            algId,
            integrity,
        })
    }(&el)
}

pub fn _encode_SPKM_ERROR(value_: &SPKM_ERROR) -> ASN1Result<X690Element> {
    |value_: &SPKM_ERROR| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_ERROR_TOKEN(&value_.errorToken)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.algId)?);
        components_.push(_encode_Integrity(&value_.integrity)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for ERROR_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ERROR_TOKEN(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ERROR_TOKEN {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ERROR_TOKEN> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ERROR_TOKEN,
            _eal_components_for_ERROR_TOKEN,
            _rctl2_components_for_ERROR_TOKEN,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        Ok(ERROR_TOKEN { tok_id, context_id })
    }(&el)
}

pub fn _encode_ERROR_TOKEN(value_: &ERROR_TOKEN) -> ASN1Result<X690Element> {
    |value_: &ERROR_TOKEN| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-MIC ::= SEQUENCE {mic-header  Mic-Header,
///                        int-cksum   BIT STRING
/// }
/// ```
///
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
impl TryFrom<X690Element> for SPKM_MIC {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_MIC(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_MIC {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_MIC> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_MIC,
            _eal_components_for_SPKM_MIC,
            _rctl2_components_for_SPKM_MIC,
        )?;
        let mic_header = _decode_Mic_Header(_components.get("mic-header").unwrap())?;
        let int_cksum = ber_decode_bit_string(_components.get("int-cksum").unwrap())?;
        Ok(SPKM_MIC {
            mic_header,
            int_cksum,
        })
    }(&el)
}

pub fn _encode_SPKM_MIC(value_: &SPKM_MIC) -> ASN1Result<X690Element> {
    |value_: &SPKM_MIC| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_Mic_Header(&value_.mic_header)?);
        components_.push(ber_encode_bit_string(&value_.int_cksum)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for Mic_Header {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Mic_Header(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Mic_Header {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Mic_Header> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Mic_Header,
            _eal_components_for_Mic_Header,
            _rctl2_components_for_Mic_Header,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        let int_alg: OPTIONAL<AlgorithmIdentifier> = match _components.get("int-alg") {
            Some(c_) => Some(_decode_AlgorithmIdentifier(c_)?),
            _ => None,
        };
        let snd_seq: OPTIONAL<SeqNum> = match _components.get("snd-seq") {
            Some(c_) => Some(_decode_SeqNum(c_)?),
            _ => None,
        };
        Ok(Mic_Header {
            tok_id,
            context_id,
            int_alg,
            snd_seq,
        })
    }(&el)
}

pub fn _encode_Mic_Header(value_: &Mic_Header) -> ASN1Result<X690Element> {
    |value_: &Mic_Header| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        if let Some(v_) = &value_.int_alg {
            components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.snd_seq {
            components_.push(|v_1: &SeqNum| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SeqNum(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SeqNum ::= SEQUENCE {num      INTEGER,
///                      dir-ind  BOOLEAN
/// }
/// ```
///
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
impl TryFrom<X690Element> for SeqNum {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SeqNum(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SeqNum {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SeqNum> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SeqNum,
            _eal_components_for_SeqNum,
            _rctl2_components_for_SeqNum,
        )?;
        let num = ber_decode_integer(_components.get("num").unwrap())?;
        let dir_ind = ber_decode_boolean(_components.get("dir-ind").unwrap())?;
        Ok(SeqNum { num, dir_ind })
    }(&el)
}

pub fn _encode_SeqNum(value_: &SeqNum) -> ASN1Result<X690Element> {
    |value_: &SeqNum| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_integer(&value_.num)?);
        components_.push(ber_encode_boolean(&value_.dir_ind)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-WRAP ::= SEQUENCE {wrap-header  Wrap-Header,
///                         wrap-body    Wrap-Body
/// }
/// ```
///
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
impl TryFrom<X690Element> for SPKM_WRAP {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_WRAP(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_WRAP {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_WRAP> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_WRAP,
            _eal_components_for_SPKM_WRAP,
            _rctl2_components_for_SPKM_WRAP,
        )?;
        let wrap_header = _decode_Wrap_Header(_components.get("wrap-header").unwrap())?;
        let wrap_body = _decode_Wrap_Body(_components.get("wrap-body").unwrap())?;
        Ok(SPKM_WRAP {
            wrap_header,
            wrap_body,
        })
    }(&el)
}

pub fn _encode_SPKM_WRAP(value_: &SPKM_WRAP) -> ASN1Result<X690Element> {
    |value_: &SPKM_WRAP| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_Wrap_Header(&value_.wrap_header)?);
        components_.push(_encode_Wrap_Body(&value_.wrap_body)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for Wrap_Header {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Wrap_Header(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Wrap_Header {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Wrap_Header> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Wrap_Header,
            _eal_components_for_Wrap_Header,
            _rctl2_components_for_Wrap_Header,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        let int_alg: OPTIONAL<AlgorithmIdentifier> = match _components.get("int-alg") {
            Some(c_) => Some(_decode_AlgorithmIdentifier(c_)?),
            _ => None,
        };
        let conf_alg: OPTIONAL<Conf_Alg> = match _components.get("conf-alg") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Conf_Alg> {
                Ok(_decode_Conf_Alg(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let snd_seq: OPTIONAL<SeqNum> = match _components.get("snd-seq") {
            Some(c_) => Some(_decode_SeqNum(c_)?),
            _ => None,
        };
        Ok(Wrap_Header {
            tok_id,
            context_id,
            int_alg,
            conf_alg,
            snd_seq,
        })
    }(&el)
}

pub fn _encode_Wrap_Header(value_: &Wrap_Header) -> ASN1Result<X690Element> {
    |value_: &Wrap_Header| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(10);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        if let Some(v_) = &value_.int_alg {
            components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.conf_alg {
            components_.push(|v_1: &Conf_Alg| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Conf_Alg(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.snd_seq {
            components_.push(|v_1: &SeqNum| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SeqNum(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Wrap-Body ::= SEQUENCE {int-cksum  BIT STRING,
///                         data       BIT STRING
/// }
/// ```
///
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
impl TryFrom<X690Element> for Wrap_Body {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Wrap_Body(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Wrap_Body {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Wrap_Body> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Wrap_Body,
            _eal_components_for_Wrap_Body,
            _rctl2_components_for_Wrap_Body,
        )?;
        let int_cksum = ber_decode_bit_string(_components.get("int-cksum").unwrap())?;
        let data = ber_decode_bit_string(_components.get("data").unwrap())?;
        Ok(Wrap_Body { int_cksum, data })
    }(&el)
}

pub fn _encode_Wrap_Body(value_: &Wrap_Body) -> ASN1Result<X690Element> {
    |value_: &Wrap_Body| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_bit_string(&value_.int_cksum)?);
        components_.push(ber_encode_bit_string(&value_.data)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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

impl TryFrom<X690Element> for Conf_Alg {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Conf_Alg(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Conf_Alg {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Conf_Alg(el)
    }
}

pub fn _decode_Conf_Alg(el: &X690Element) -> ASN1Result<Conf_Alg> {
    |el: &X690Element| -> ASN1Result<Conf_Alg> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Conf_Alg::algId(_decode_AlgorithmIdentifier(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(Conf_Alg::null(ber_decode_null(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_Conf_Alg(value_: &Conf_Alg) -> ASN1Result<X690Element> {
    |value: &Conf_Alg| -> ASN1Result<X690Element> {
        match value {
            Conf_Alg::algId(v) => |v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            Conf_Alg::null(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_null(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SPKM-DEL ::= SEQUENCE {del-header  Del-Header,
///                        int-cksum   BIT STRING
/// }
/// ```
///
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
impl TryFrom<X690Element> for SPKM_DEL {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKM_DEL(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKM_DEL {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SPKM_DEL> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SPKM_DEL,
            _eal_components_for_SPKM_DEL,
            _rctl2_components_for_SPKM_DEL,
        )?;
        let del_header = _decode_Del_Header(_components.get("del-header").unwrap())?;
        let int_cksum = ber_decode_bit_string(_components.get("int-cksum").unwrap())?;
        Ok(SPKM_DEL {
            del_header,
            int_cksum,
        })
    }(&el)
}

pub fn _encode_SPKM_DEL(value_: &SPKM_DEL) -> ASN1Result<X690Element> {
    |value_: &SPKM_DEL| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_Del_Header(&value_.del_header)?);
        components_.push(ber_encode_bit_string(&value_.int_cksum)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for Del_Header {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Del_Header(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Del_Header {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Del_Header> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Del_Header,
            _eal_components_for_Del_Header,
            _rctl2_components_for_Del_Header,
        )?;
        let tok_id = ber_decode_integer(_components.get("tok-id").unwrap())?;
        let context_id = _decode_Random_Integer(_components.get("context-id").unwrap())?;
        let int_alg: OPTIONAL<AlgorithmIdentifier> = match _components.get("int-alg") {
            Some(c_) => Some(_decode_AlgorithmIdentifier(c_)?),
            _ => None,
        };
        let snd_seq: OPTIONAL<SeqNum> = match _components.get("snd-seq") {
            Some(c_) => Some(_decode_SeqNum(c_)?),
            _ => None,
        };
        Ok(Del_Header {
            tok_id,
            context_id,
            int_alg,
            snd_seq,
        })
    }(&el)
}

pub fn _encode_Del_Header(value_: &Del_Header) -> ASN1Result<X690Element> {
    |value_: &Del_Header| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(ber_encode_integer(&value_.tok_id)?);
        components_.push(_encode_Random_Integer(&value_.context_id)?);
        if let Some(v_) = &value_.int_alg {
            components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.snd_seq {
            components_.push(|v_1: &SeqNum| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SeqNum(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MechType  ::=  OBJECT IDENTIFIER
/// ```
pub type MechType = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_MechType(el: &X690Element) -> ASN1Result<MechType> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_MechType(value_: &MechType) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
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
impl TryFrom<X690Element> for InitialContextToken {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InitialContextToken(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InitialContextToken {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<InitialContextToken> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_InitialContextToken,
            _eal_components_for_InitialContextToken,
            _rctl2_components_for_InitialContextToken,
        )?;
        let thisMech = _decode_MechType(_components.get("thisMech").unwrap())?;
        let innerContextToken =
            _decode_SPKMInnerContextToken(_components.get("innerContextToken").unwrap())?;
        Ok(InitialContextToken {
            thisMech,
            innerContextToken,
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
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(components_)),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
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

impl TryFrom<X690Element> for SPKMInnerContextToken {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SPKMInnerContextToken(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SPKMInnerContextToken {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SPKMInnerContextToken(el)
    }
}

pub fn _decode_SPKMInnerContextToken(el: &X690Element) -> ASN1Result<SPKMInnerContextToken> {
    |el: &X690Element| -> ASN1Result<SPKMInnerContextToken> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(SPKMInnerContextToken::req(_decode_SPKM_REQ(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(SPKMInnerContextToken::rep_ti(_decode_SPKM_REP_TI(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(SPKMInnerContextToken::rep_it(_decode_SPKM_REP_IT(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(SPKMInnerContextToken::error(_decode_SPKM_ERROR(&el)?)),
            (TagClass::CONTEXT, 4) => Ok(SPKMInnerContextToken::mic(_decode_SPKM_MIC(&el)?)),
            (TagClass::CONTEXT, 5) => Ok(SPKMInnerContextToken::wrap(_decode_SPKM_WRAP(&el)?)),
            (TagClass::CONTEXT, 6) => Ok(SPKMInnerContextToken::del(_decode_SPKM_DEL(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_SPKMInnerContextToken(value_: &SPKMInnerContextToken) -> ASN1Result<X690Element> {
    |value: &SPKMInnerContextToken| -> ASN1Result<X690Element> {
        match value {
            SPKMInnerContextToken::req(v) => |v_1: &SPKM_REQ| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_REQ(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            SPKMInnerContextToken::rep_ti(v) => |v_1: &SPKM_REP_TI| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_REP_TI(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            SPKMInnerContextToken::rep_it(v) => |v_1: &SPKM_REP_IT| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_REP_IT(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            SPKMInnerContextToken::error(v) => |v_1: &SPKM_ERROR| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_ERROR(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            SPKMInnerContextToken::mic(v) => |v_1: &SPKM_MIC| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_MIC(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v),
            SPKMInnerContextToken::wrap(v) => |v_1: &SPKM_WRAP| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_WRAP(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v),
            SPKMInnerContextToken::del(v) => |v_1: &SPKM_DEL| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SPKM_DEL(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v),
        }
    }(&value_)
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
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AuthorizationData_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AuthorizationData_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AuthorizationData_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AuthorizationData(value_: &AuthorizationData) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AuthorizationData_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AuthorizationData_Item(&v)?);
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
/// md5-DES-CBC OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    integrity(3) md5-DES-CBC(1)}
/// ```
///
///
pub fn md5_DES_CBC() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* security */ 5, /* integrity */ 3,
        /* md5-DES-CBC */ 1,
    ]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sum64-DES-CBC OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    integrity(3) sum64-DES-CBC(2)}
/// ```
///
///
pub fn sum64_DES_CBC() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* security */ 5, /* integrity */ 3,
        /* sum64-DES-CBC */ 2,
    ]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// spkm-1 OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    mechanisms(5) spkm(1) spkm-1(1)}
/// ```
///
///
pub fn spkm_1() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* security */ 5, /* mechanisms */ 5, /* spkm */ 1,
        /* spkm-1 */ 1,
    ]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// spkm-2 OBJECT IDENTIFIER ::= {iso(1) identified-organization(3) dod(6) internet(1) security(5)
///    mechanisms(5) spkm(1) spkm-2(2)}
/// ```
///
///
pub fn spkm_2() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* security */ 5, /* mechanisms */ 5, /* spkm */ 1,
        /* spkm-2 */ 2,
    ]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthorizationData-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for AuthorizationData_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AuthorizationData_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AuthorizationData_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<AuthorizationData_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AuthorizationData_Item,
            _eal_components_for_AuthorizationData_Item,
            _rctl2_components_for_AuthorizationData_Item,
        )?;
        let ad_type = ber_decode_integer(_components.get("ad-type").unwrap())?;
        let ad_data = ber_decode_octet_string(_components.get("ad-data").unwrap())?;
        Ok(AuthorizationData_Item { ad_type, ad_data })
    }(&el)
}

pub fn _encode_AuthorizationData_Item(value_: &AuthorizationData_Item) -> ASN1Result<X690Element> {
    |value_: &AuthorizationData_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_integer(&value_.ad_type)?);
        components_.push(ber_encode_octet_string(&value_.ad_data)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}
