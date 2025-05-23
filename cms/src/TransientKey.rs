#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # TransientKey
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `TransientKey`.
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
use crate::CryptographicMessageSyntax::*;
use crate::TrustedTimeStamp::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// transientKeySignedTST OID ::= {
/// iso(1) identified-organization(3) tc68(133) country(16) x9(840)
/// x9Standards(9) x9-95(95) module(0) tk(2) contentType(1)}
/// ```
///
///
pub fn transientKeySignedTST() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* tc68 */ 133,
        /* country */ 16, /* x9 */ 840, /* x9Standards */ 9, /* x9-95 */ 95,
        /* module */ 0, /* tk */ 2, /* contentType */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-req-tk OID ::= {
/// iso(1) identified-organization(3) tc68(133) country(16) x9(840)
/// x9Standards(9) x9-95(95) module(0) tk(2) method(2)}
/// ```
///
///
pub fn tsp_req_tk() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* tc68 */ 133,
        /* country */ 16, /* x9 */ 840, /* x9Standards */ 9, /* x9-95 */ 95,
        /* module */ 0, /* tk */ 2, /* method */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TransientKeySignedTST ::= SEQUENCE {
/// version         TKSVersion,
/// tstAndInterval  TSTAndInterval,
/// digest          Digest,
/// previousDigest  [0] Digest  OPTIONAL,
/// signature       Signature
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TransientKeySignedTST {
    pub version: TKSVersion,
    pub tstAndInterval: TSTAndInterval,
    pub digest: Digest,
    pub previousDigest: OPTIONAL<Digest>,
    pub signature: Signature,
}
impl TransientKeySignedTST {
    pub fn new(
        version: TKSVersion,
        tstAndInterval: TSTAndInterval,
        digest: Digest,
        previousDigest: OPTIONAL<Digest>,
        signature: Signature,
    ) -> Self {
        TransientKeySignedTST {
            version,
            tstAndInterval,
            digest,
            previousDigest,
            signature,
        }
    }
}
impl TryFrom<&X690Element> for TransientKeySignedTST {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TransientKeySignedTST(el)
    }
}

pub const _rctl1_components_for_TransientKeySignedTST: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tstAndInterval",
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
    ComponentSpec::new(
        "previousDigest",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
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

pub const _rctl2_components_for_TransientKeySignedTST: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TransientKeySignedTST: &[ComponentSpec; 0] = &[];

pub fn _decode_TransientKeySignedTST(el: &X690Element) -> ASN1Result<TransientKeySignedTST> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TransientKeySignedTST")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TransientKeySignedTST,
        _eal_components_for_TransientKeySignedTST,
        _rctl2_components_for_TransientKeySignedTST,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<TKSVersion> = None;
    let mut tstAndInterval_: OPTIONAL<TSTAndInterval> = None;
    let mut digest_: OPTIONAL<Digest> = None;
    let mut previousDigest_: OPTIONAL<Digest> = None;
    let mut signature_: OPTIONAL<Signature> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_TKSVersion(_el)?),
            "tstAndInterval" => tstAndInterval_ = Some(_decode_TSTAndInterval(_el)?),
            "digest" => digest_ = Some(_decode_Digest(_el)?),
            "previousDigest" => previousDigest_ = Some(_decode_Digest(_el)?),
            "signature" => signature_ = Some(_decode_Signature(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "TransientKeySignedTST",
                ))
            }
        }
    }
    Ok(TransientKeySignedTST {
        version: version_.unwrap(),
        tstAndInterval: tstAndInterval_.unwrap(),
        digest: digest_.unwrap(),
        previousDigest: previousDigest_,
        signature: signature_.unwrap(),
    })
}

pub fn _encode_TransientKeySignedTST(value_: &TransientKeySignedTST) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    components_.push(_encode_TKSVersion(&value_.version)?);
    components_.push(_encode_TSTAndInterval(&value_.tstAndInterval)?);
    components_.push(_encode_Digest(&value_.digest)?);
    if let Some(v_) = &value_.previousDigest {
        components_.push(|v_1: &Digest| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Digest(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_Signature(&value_.signature)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TransientKeySignedTST(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TransientKeySignedTST")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TransientKeySignedTST,
        _eal_components_for_TransientKeySignedTST,
        _rctl2_components_for_TransientKeySignedTST,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_TKSVersion(_el)?,
            "tstAndInterval" => _validate_TSTAndInterval(_el)?,
            "digest" => _validate_Digest(_el)?,
            "previousDigest" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "previousDigest")
                    );
                }
                Ok(_validate_Digest(&el)?)
            }(_el)?,
            "signature" => _validate_Signature(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "TransientKeySignedTST",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TKSVersion  ::=  INTEGER { version1(1) } (version1, ...)
/// ```
pub type TKSVersion = i8;

pub const TKSVersion_version1: TKSVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_TKSVersion(el: &X690Element) -> ASN1Result<TKSVersion> {
    BER.decode_i8(el)
}

pub fn _encode_TKSVersion(value_: &TKSVersion) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_TKSVersion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSTAndInterval ::= SEQUENCE {
/// tstInfo       TSTInfo,
/// intervalInfo  IntervalInfo
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TSTAndInterval {
    pub tstInfo: TSTInfo,
    pub intervalInfo: IntervalInfo,
}
impl TSTAndInterval {
    pub fn new(tstInfo: TSTInfo, intervalInfo: IntervalInfo) -> Self {
        TSTAndInterval {
            tstInfo,
            intervalInfo,
        }
    }
}
impl TryFrom<&X690Element> for TSTAndInterval {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TSTAndInterval(el)
    }
}

pub const _rctl1_components_for_TSTAndInterval: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "tstInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "intervalInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TSTAndInterval: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TSTAndInterval: &[ComponentSpec; 0] = &[];

pub fn _decode_TSTAndInterval(el: &X690Element) -> ASN1Result<TSTAndInterval> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTAndInterval"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TSTAndInterval,
        _eal_components_for_TSTAndInterval,
        _rctl2_components_for_TSTAndInterval,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut tstInfo_: OPTIONAL<TSTInfo> = None;
    let mut intervalInfo_: OPTIONAL<IntervalInfo> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tstInfo" => tstInfo_ = Some(_decode_TSTInfo(_el)?),
            "intervalInfo" => intervalInfo_ = Some(_decode_IntervalInfo(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTAndInterval")
                )
            }
        }
    }
    Ok(TSTAndInterval {
        tstInfo: tstInfo_.unwrap(),
        intervalInfo: intervalInfo_.unwrap(),
    })
}

pub fn _encode_TSTAndInterval(value_: &TSTAndInterval) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_TSTInfo(&value_.tstInfo)?);
    components_.push(_encode_IntervalInfo(&value_.intervalInfo)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TSTAndInterval(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTAndInterval"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TSTAndInterval,
        _eal_components_for_TSTAndInterval,
        _rctl2_components_for_TSTAndInterval,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "tstInfo" => _validate_TSTInfo(_el)?,
            "intervalInfo" => _validate_IntervalInfo(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTAndInterval")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IntervalInfo ::= SEQUENCE {
/// version             IIVersion,
/// signedIntervalSpec  SignedIntervalSpec,
/// archiveTree         ArchiveTree,
/// certifierList       UriList  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct IntervalInfo {
    pub version: IIVersion,
    pub signedIntervalSpec: SignedIntervalSpec,
    pub archiveTree: ArchiveTree,
    pub certifierList: OPTIONAL<UriList>,
}
impl IntervalInfo {
    pub fn new(
        version: IIVersion,
        signedIntervalSpec: SignedIntervalSpec,
        archiveTree: ArchiveTree,
        certifierList: OPTIONAL<UriList>,
    ) -> Self {
        IntervalInfo {
            version,
            signedIntervalSpec,
            archiveTree,
            certifierList,
        }
    }
}
impl TryFrom<&X690Element> for IntervalInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IntervalInfo(el)
    }
}

pub const _rctl1_components_for_IntervalInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signedIntervalSpec",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "archiveTree",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certifierList",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IntervalInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IntervalInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_IntervalInfo(el: &X690Element) -> ASN1Result<IntervalInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IntervalInfo,
        _eal_components_for_IntervalInfo,
        _rctl2_components_for_IntervalInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<IIVersion> = None;
    let mut signedIntervalSpec_: OPTIONAL<SignedIntervalSpec> = None;
    let mut archiveTree_: OPTIONAL<ArchiveTree> = None;
    let mut certifierList_: OPTIONAL<UriList> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_IIVersion(_el)?),
            "signedIntervalSpec" => signedIntervalSpec_ = Some(_decode_SignedIntervalSpec(_el)?),
            "archiveTree" => archiveTree_ = Some(_decode_ArchiveTree(_el)?),
            "certifierList" => certifierList_ = Some(_decode_UriList(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalInfo")
                )
            }
        }
    }
    Ok(IntervalInfo {
        version: version_.unwrap(),
        signedIntervalSpec: signedIntervalSpec_.unwrap(),
        archiveTree: archiveTree_.unwrap(),
        certifierList: certifierList_,
    })
}

pub fn _encode_IntervalInfo(value_: &IntervalInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_IIVersion(&value_.version)?);
    components_.push(_encode_SignedIntervalSpec(&value_.signedIntervalSpec)?);
    components_.push(_encode_ArchiveTree(&value_.archiveTree)?);
    if let Some(v_) = &value_.certifierList {
        components_.push(_encode_UriList(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_IntervalInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IntervalInfo,
        _eal_components_for_IntervalInfo,
        _rctl2_components_for_IntervalInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_IIVersion(_el)?,
            "signedIntervalSpec" => _validate_SignedIntervalSpec(_el)?,
            "archiveTree" => _validate_ArchiveTree(_el)?,
            "certifierList" => _validate_UriList(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IIVersion  ::=  INTEGER { version1(1) } (version1, ...)
/// ```
pub type IIVersion = i8;

pub const IIVersion_version1: IIVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_IIVersion(el: &X690Element) -> ASN1Result<IIVersion> {
    BER.decode_i8(el)
}

pub fn _encode_IIVersion(value_: &IIVersion) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_IIVersion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedIntervalSpec ::= SEQUENCE {
/// intervalSpec       IntervalSpec,
/// signature          Signature,
/// identitySignature  IdentitySignature
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SignedIntervalSpec {
    pub intervalSpec: IntervalSpec,
    pub signature: Signature,
    pub identitySignature: IdentitySignature,
}
impl SignedIntervalSpec {
    pub fn new(
        intervalSpec: IntervalSpec,
        signature: Signature,
        identitySignature: IdentitySignature,
    ) -> Self {
        SignedIntervalSpec {
            intervalSpec,
            signature,
            identitySignature,
        }
    }
}
impl TryFrom<&X690Element> for SignedIntervalSpec {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SignedIntervalSpec(el)
    }
}

pub const _rctl1_components_for_SignedIntervalSpec: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "intervalSpec",
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
    ComponentSpec::new(
        "identitySignature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SignedIntervalSpec: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SignedIntervalSpec: &[ComponentSpec; 0] = &[];

pub fn _decode_SignedIntervalSpec(el: &X690Element) -> ASN1Result<SignedIntervalSpec> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedIntervalSpec")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SignedIntervalSpec,
        _eal_components_for_SignedIntervalSpec,
        _rctl2_components_for_SignedIntervalSpec,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut intervalSpec_: OPTIONAL<IntervalSpec> = None;
    let mut signature_: OPTIONAL<Signature> = None;
    let mut identitySignature_: OPTIONAL<IdentitySignature> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "intervalSpec" => intervalSpec_ = Some(_decode_IntervalSpec(_el)?),
            "signature" => signature_ = Some(_decode_Signature(_el)?),
            "identitySignature" => identitySignature_ = Some(_decode_IdentitySignature(_el)?),
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedIntervalSpec"))
            }
        }
    }
    Ok(SignedIntervalSpec {
        intervalSpec: intervalSpec_.unwrap(),
        signature: signature_.unwrap(),
        identitySignature: identitySignature_.unwrap(),
    })
}

pub fn _encode_SignedIntervalSpec(value_: &SignedIntervalSpec) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_IntervalSpec(&value_.intervalSpec)?);
    components_.push(_encode_Signature(&value_.signature)?);
    components_.push(_encode_IdentitySignature(&value_.identitySignature)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SignedIntervalSpec(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedIntervalSpec")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SignedIntervalSpec,
        _eal_components_for_SignedIntervalSpec,
        _rctl2_components_for_SignedIntervalSpec,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "intervalSpec" => _validate_IntervalSpec(_el)?,
            "signature" => _validate_Signature(_el)?,
            "identitySignature" => _validate_IdentitySignature(_el)?,
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SignedIntervalSpec"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IntervalSpec ::= SEQUENCE {
/// chainSpec           ChainSpec,
/// intervalStart       GeneralizedTime,
/// intervalStop        GeneralizedTime,
/// publicKey           PublicKey,
/// previousPublicKey   [0] PublicKey  OPTIONAL,
/// previousMetaDigest  [1] OCTET STRING  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct IntervalSpec {
    pub chainSpec: ChainSpec,
    pub intervalStart: GeneralizedTime,
    pub intervalStop: GeneralizedTime,
    pub publicKey: PublicKey,
    pub previousPublicKey: OPTIONAL<PublicKey>,
    pub previousMetaDigest: OPTIONAL<OCTET_STRING>,
}
impl IntervalSpec {
    pub fn new(
        chainSpec: ChainSpec,
        intervalStart: GeneralizedTime,
        intervalStop: GeneralizedTime,
        publicKey: PublicKey,
        previousPublicKey: OPTIONAL<PublicKey>,
        previousMetaDigest: OPTIONAL<OCTET_STRING>,
    ) -> Self {
        IntervalSpec {
            chainSpec,
            intervalStart,
            intervalStop,
            publicKey,
            previousPublicKey,
            previousMetaDigest,
        }
    }
}
impl TryFrom<&X690Element> for IntervalSpec {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IntervalSpec(el)
    }
}

pub const _rctl1_components_for_IntervalSpec: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "chainSpec",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "intervalStart",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "intervalStop",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "publicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "previousPublicKey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "previousMetaDigest",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IntervalSpec: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IntervalSpec: &[ComponentSpec; 0] = &[];

pub fn _decode_IntervalSpec(el: &X690Element) -> ASN1Result<IntervalSpec> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalSpec")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IntervalSpec,
        _eal_components_for_IntervalSpec,
        _rctl2_components_for_IntervalSpec,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut chainSpec_: OPTIONAL<ChainSpec> = None;
    let mut intervalStart_: OPTIONAL<GeneralizedTime> = None;
    let mut intervalStop_: OPTIONAL<GeneralizedTime> = None;
    let mut publicKey_: OPTIONAL<PublicKey> = None;
    let mut previousPublicKey_: OPTIONAL<PublicKey> = None;
    let mut previousMetaDigest_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "chainSpec" => chainSpec_ = Some(_decode_ChainSpec(_el)?),
            "intervalStart" => intervalStart_ = Some(BER.decode_generalized_time(_el)?),
            "intervalStop" => intervalStop_ = Some(BER.decode_generalized_time(_el)?),
            "publicKey" => publicKey_ = Some(_decode_PublicKey(_el)?),
            "previousPublicKey" => previousPublicKey_ = Some(_decode_PublicKey(_el)?),
            "previousMetaDigest" => previousMetaDigest_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalSpec")
                )
            }
        }
    }
    Ok(IntervalSpec {
        chainSpec: chainSpec_.unwrap(),
        intervalStart: intervalStart_.unwrap(),
        intervalStop: intervalStop_.unwrap(),
        publicKey: publicKey_.unwrap(),
        previousPublicKey: previousPublicKey_,
        previousMetaDigest: previousMetaDigest_,
    })
}

pub fn _encode_IntervalSpec(value_: &IntervalSpec) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_ChainSpec(&value_.chainSpec)?);
    components_.push(BER.encode_generalized_time(&value_.intervalStart)?);
    components_.push(BER.encode_generalized_time(&value_.intervalStop)?);
    components_.push(_encode_PublicKey(&value_.publicKey)?);
    if let Some(v_) = &value_.previousPublicKey {
        components_.push(|v_1: &PublicKey| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_PublicKey(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.previousMetaDigest {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
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

pub fn _validate_IntervalSpec(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalSpec")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IntervalSpec,
        _eal_components_for_IntervalSpec,
        _rctl2_components_for_IntervalSpec,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "chainSpec" => _validate_ChainSpec(_el)?,
            "intervalStart" => BER.validate_generalized_time(_el)?,
            "intervalStop" => BER.validate_generalized_time(_el)?,
            "publicKey" => _validate_PublicKey(_el)?,
            "previousPublicKey" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "previousPublicKey",
                    ));
                }
                Ok(_validate_PublicKey(&el)?)
            }(_el)?,
            "previousMetaDigest" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "previousMetaDigest",
                    ));
                }
                Ok(BER.validate_octet_string(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntervalSpec")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChainSpec ::= SEQUENCE {
/// serverId                  Uri,
/// chainStart                GeneralizedTime,
/// digestAlgorithm           DigestAlgorithmIdentifier,
/// signatureAlgorithm        SignatureAlgorithmIdentifier,
/// publicVerificationServer  Uri  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ChainSpec {
    pub serverId: Uri,
    pub chainStart: GeneralizedTime,
    pub digestAlgorithm: DigestAlgorithmIdentifier,
    pub signatureAlgorithm: SignatureAlgorithmIdentifier,
    pub publicVerificationServer: OPTIONAL<Uri>,
}
impl ChainSpec {
    pub fn new(
        serverId: Uri,
        chainStart: GeneralizedTime,
        digestAlgorithm: DigestAlgorithmIdentifier,
        signatureAlgorithm: SignatureAlgorithmIdentifier,
        publicVerificationServer: OPTIONAL<Uri>,
    ) -> Self {
        ChainSpec {
            serverId,
            chainStart,
            digestAlgorithm,
            signatureAlgorithm,
            publicVerificationServer,
        }
    }
}
impl TryFrom<&X690Element> for ChainSpec {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ChainSpec(el)
    }
}

pub const _rctl1_components_for_ChainSpec: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "serverId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "chainStart",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
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
        "signatureAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "publicVerificationServer",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 22)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ChainSpec: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ChainSpec: &[ComponentSpec; 0] = &[];

pub fn _decode_ChainSpec(el: &X690Element) -> ASN1Result<ChainSpec> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ChainSpec")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ChainSpec,
        _eal_components_for_ChainSpec,
        _rctl2_components_for_ChainSpec,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serverId_: OPTIONAL<Uri> = None;
    let mut chainStart_: OPTIONAL<GeneralizedTime> = None;
    let mut digestAlgorithm_: OPTIONAL<DigestAlgorithmIdentifier> = None;
    let mut signatureAlgorithm_: OPTIONAL<SignatureAlgorithmIdentifier> = None;
    let mut publicVerificationServer_: OPTIONAL<Uri> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serverId" => serverId_ = Some(_decode_Uri(_el)?),
            "chainStart" => chainStart_ = Some(BER.decode_generalized_time(_el)?),
            "digestAlgorithm" => digestAlgorithm_ = Some(_decode_DigestAlgorithmIdentifier(_el)?),
            "signatureAlgorithm" => {
                signatureAlgorithm_ = Some(_decode_SignatureAlgorithmIdentifier(_el)?)
            }
            "publicVerificationServer" => publicVerificationServer_ = Some(_decode_Uri(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ChainSpec"))
            }
        }
    }
    Ok(ChainSpec {
        serverId: serverId_.unwrap(),
        chainStart: chainStart_.unwrap(),
        digestAlgorithm: digestAlgorithm_.unwrap(),
        signatureAlgorithm: signatureAlgorithm_.unwrap(),
        publicVerificationServer: publicVerificationServer_,
    })
}

pub fn _encode_ChainSpec(value_: &ChainSpec) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    components_.push(_encode_Uri(&value_.serverId)?);
    components_.push(BER.encode_generalized_time(&value_.chainStart)?);
    components_.push(_encode_DigestAlgorithmIdentifier(&value_.digestAlgorithm)?);
    components_.push(_encode_SignatureAlgorithmIdentifier(
        &value_.signatureAlgorithm,
    )?);
    if let Some(v_) = &value_.publicVerificationServer {
        components_.push(_encode_Uri(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ChainSpec(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ChainSpec")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ChainSpec,
        _eal_components_for_ChainSpec,
        _rctl2_components_for_ChainSpec,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serverId" => _validate_Uri(_el)?,
            "chainStart" => BER.validate_generalized_time(_el)?,
            "digestAlgorithm" => _validate_DigestAlgorithmIdentifier(_el)?,
            "signatureAlgorithm" => _validate_SignatureAlgorithmIdentifier(_el)?,
            "publicVerificationServer" => _validate_Uri(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ChainSpec"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Uri  ::=  IA5String
/// ```
pub type Uri = IA5String; // IA5String

pub fn _decode_Uri(el: &X690Element) -> ASN1Result<Uri> {
    BER.decode_ia5_string(&el)
}

pub fn _encode_Uri(value_: &Uri) -> ASN1Result<X690Element> {
    BER.encode_ia5_string(&value_)
}

pub fn _validate_Uri(el: &X690Element) -> ASN1Result<()> {
    BER.validate_ia5_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PublicKey  ::=  OCTET STRING
/// ```
pub type PublicKey = OCTET_STRING; // OctetStringType

pub fn _decode_PublicKey(el: &X690Element) -> ASN1Result<PublicKey> {
    BER.decode_octet_string(&el)
}

pub fn _encode_PublicKey(value_: &PublicKey) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_PublicKey(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
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
/// IdentitySignature ::= SEQUENCE {
/// signatureAlgorithm  SignatureAlgorithmIdentifier,
/// signature           Signature,
/// certificate         EncodedCertificate  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct IdentitySignature {
    pub signatureAlgorithm: SignatureAlgorithmIdentifier,
    pub signature: Signature,
    pub certificate: OPTIONAL<EncodedCertificate>,
}
impl IdentitySignature {
    pub fn new(
        signatureAlgorithm: SignatureAlgorithmIdentifier,
        signature: Signature,
        certificate: OPTIONAL<EncodedCertificate>,
    ) -> Self {
        IdentitySignature {
            signatureAlgorithm,
            signature,
            certificate,
        }
    }
}
impl TryFrom<&X690Element> for IdentitySignature {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IdentitySignature(el)
    }
}

pub const _rctl1_components_for_IdentitySignature: &[ComponentSpec; 3] = &[
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
    ComponentSpec::new("certificate", true, TagSelector::any, None, None),
];

pub const _rctl2_components_for_IdentitySignature: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IdentitySignature: &[ComponentSpec; 0] = &[];

pub fn _decode_IdentitySignature(el: &X690Element) -> ASN1Result<IdentitySignature> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdentitySignature")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdentitySignature,
        _eal_components_for_IdentitySignature,
        _rctl2_components_for_IdentitySignature,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut signatureAlgorithm_: OPTIONAL<SignatureAlgorithmIdentifier> = None;
    let mut signature_: OPTIONAL<Signature> = None;
    let mut certificate_: OPTIONAL<EncodedCertificate> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "signatureAlgorithm" => {
                signatureAlgorithm_ = Some(_decode_SignatureAlgorithmIdentifier(_el)?)
            }
            "signature" => signature_ = Some(_decode_Signature(_el)?),
            "certificate" => certificate_ = Some(_decode_EncodedCertificate(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdentitySignature")
                )
            }
        }
    }
    Ok(IdentitySignature {
        signatureAlgorithm: signatureAlgorithm_.unwrap(),
        signature: signature_.unwrap(),
        certificate: certificate_,
    })
}

pub fn _encode_IdentitySignature(value_: &IdentitySignature) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_SignatureAlgorithmIdentifier(
        &value_.signatureAlgorithm,
    )?);
    components_.push(_encode_Signature(&value_.signature)?);
    if let Some(v_) = &value_.certificate {
        components_.push(_encode_EncodedCertificate(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_IdentitySignature(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdentitySignature")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdentitySignature,
        _eal_components_for_IdentitySignature,
        _rctl2_components_for_IdentitySignature,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "signatureAlgorithm" => _validate_SignatureAlgorithmIdentifier(_el)?,
            "signature" => _validate_Signature(_el)?,
            "certificate" => _validate_EncodedCertificate(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdentitySignature")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncodedCertificate  ::=  TYPE-IDENTIFIER.&Type( Certificate )
/// ```
pub type EncodedCertificate = X690Element; // ObjectClassFieldType

pub fn _decode_EncodedCertificate(el: &X690Element) -> ASN1Result<EncodedCertificate> {
    x690_identity(&el)
}

pub fn _encode_EncodedCertificate(value_: &EncodedCertificate) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

pub fn _validate_EncodedCertificate(el: &X690Element) -> ASN1Result<()> {
    BER.validate_any(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ArchiveTree ::= SEQUENCE {
/// archive   Uri,
/// children  ArchiveTreeList  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ArchiveTree {
    pub archive: Uri,
    pub children: OPTIONAL<ArchiveTreeList>,
}
impl ArchiveTree {
    pub fn new(archive: Uri, children: OPTIONAL<ArchiveTreeList>) -> Self {
        ArchiveTree { archive, children }
    }
}
impl TryFrom<&X690Element> for ArchiveTree {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ArchiveTree(el)
    }
}

pub const _rctl1_components_for_ArchiveTree: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "archive",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 22)),
        None,
        None,
    ),
    ComponentSpec::new(
        "children",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ArchiveTree: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ArchiveTree: &[ComponentSpec; 0] = &[];

pub fn _decode_ArchiveTree(el: &X690Element) -> ASN1Result<ArchiveTree> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ArchiveTree")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ArchiveTree,
        _eal_components_for_ArchiveTree,
        _rctl2_components_for_ArchiveTree,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut archive_: OPTIONAL<Uri> = None;
    let mut children_: OPTIONAL<ArchiveTreeList> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "archive" => archive_ = Some(_decode_Uri(_el)?),
            "children" => children_ = Some(_decode_ArchiveTreeList(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ArchiveTree")
                )
            }
        }
    }
    Ok(ArchiveTree {
        archive: archive_.unwrap(),
        children: children_,
    })
}

pub fn _encode_ArchiveTree(value_: &ArchiveTree) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Uri(&value_.archive)?);
    if let Some(v_) = &value_.children {
        components_.push(_encode_ArchiveTreeList(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ArchiveTree(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ArchiveTree")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ArchiveTree,
        _eal_components_for_ArchiveTree,
        _rctl2_components_for_ArchiveTree,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "archive" => _validate_Uri(_el)?,
            "children" => _validate_ArchiveTreeList(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ArchiveTree")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ArchiveTreeList  ::=  SEQUENCE SIZE(1..MAX) OF ArchiveTree
/// ```
pub type ArchiveTreeList = Vec<ArchiveTree>; // SequenceOfType

pub fn _decode_ArchiveTreeList(el: &X690Element) -> ASN1Result<ArchiveTreeList> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ArchiveTreeList"))
        }
    };
    let mut items: SEQUENCE_OF<ArchiveTree> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_ArchiveTree(el)?);
    }
    Ok(items)
}

pub fn _encode_ArchiveTreeList(value_: &ArchiveTreeList) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_ArchiveTree(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ArchiveTreeList(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_ArchiveTree(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ArchiveTreeList")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UriList  ::=  SEQUENCE SIZE(1..MAX) OF Uri
/// ```
pub type UriList = Vec<Uri>; // SequenceOfType

pub fn _decode_UriList(el: &X690Element) -> ASN1Result<UriList> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UriList")),
    };
    let mut items: SEQUENCE_OF<Uri> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Uri(el)?);
    }
    Ok(items)
}

pub fn _encode_UriList(value_: &UriList) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Uri(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_UriList(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Uri(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UriList")),
    }
}
