#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # Wrapper
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `Wrapper`.
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
use crate::CryptoTools::*;
use crate::PKI_Stub::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

pub type WRAPPED_PROT = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// WrappedProt {WRAPPED-PROT:SupportedProtSet} ::= SEQUENCE {
///   id    WRAPPED-PROT.&id({SupportedProtSet}),
///   prot  WRAPPED-PROT.&Type({SupportedProtSet}{@id}),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct WrappedProt {
    pub id: OBJECT_IDENTIFIER,
    pub prot: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl WrappedProt {
    pub fn new(id: OBJECT_IDENTIFIER, prot: X690Element, _unrecognized: Vec<X690Element>) -> Self {
        WrappedProt {
            id,
            prot,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for WrappedProt {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_WrappedProt(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for WrappedProt {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_WrappedProt(el)
    }
}

pub const _rctl1_components_for_WrappedProt: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("prot", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_WrappedProt: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_WrappedProt: &[ComponentSpec; 0] = &[];

pub fn _decode_WrappedProt(el: &X690Element) -> ASN1Result<WrappedProt> {
    |el_: &X690Element| -> ASN1Result<WrappedProt> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_WrappedProt,
            _eal_components_for_WrappedProt,
            _rctl2_components_for_WrappedProt,
        )?;
        let id = ber_decode_object_identifier(_components.get("id").unwrap())?;
        let prot = x690_identity(_components.get("prot").unwrap())?;
        Ok(WrappedProt {
            id,
            prot,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_WrappedProt(value_: &WrappedProt) -> ASN1Result<X690Element> {
    |value_: &WrappedProt| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.id)?);
        components_.push(x690_identity(&value_.prot)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// WrapperPDU  ::=  CHOICE {
///   handshakeReq       [0]  HandshakeReq,
///   handshakeAcc       [1]  HandshakeAcc,
///   handshakeWrpRej    [2]  HandshakeWrpRej,
///   handshakeProRej    [3]  HandshakeProRej,
///   handshakeSecAbort  [4]  HandshakeSecAbort,
///   handshakeProAbort  [5]  HandshakeProAbort,
///   dtSecAbort         [6]  DtSecAbort,
///   applAbort          [7]  ApplAbort,
///   releaseReq         [8]  ReleaseReq,
///   releaseRsp         [9]  ReleaseRsp,
///   dataTransferClient [10] DataTransferClient,
///   dataTransferServer [11] DataTransferServer,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum WrapperPDU {
    handshakeReq(HandshakeReq),
    handshakeAcc(HandshakeAcc),
    handshakeWrpRej(HandshakeWrpRej),
    handshakeProRej(HandshakeProRej),
    handshakeSecAbort(HandshakeSecAbort),
    handshakeProAbort(HandshakeProAbort),
    dtSecAbort(DtSecAbort),
    applAbort(ApplAbort),
    releaseReq(ReleaseReq),
    releaseRsp(ReleaseRsp),
    dataTransferClient(DataTransferClient),
    dataTransferServer(DataTransferServer),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for WrapperPDU {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_WrapperPDU(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for WrapperPDU {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_WrapperPDU(el)
    }
}

pub fn _decode_WrapperPDU(el: &X690Element) -> ASN1Result<WrapperPDU> {
    |el: &X690Element| -> ASN1Result<WrapperPDU> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(WrapperPDU::handshakeReq(_decode_HandshakeReq(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(WrapperPDU::handshakeAcc(_decode_HandshakeAcc(&el)?)),
            (TagClass::CONTEXT, 2) => {
                Ok(WrapperPDU::handshakeWrpRej(_decode_HandshakeWrpRej(&el)?))
            }
            (TagClass::CONTEXT, 3) => {
                Ok(WrapperPDU::handshakeProRej(_decode_HandshakeProRej(&el)?))
            }
            (TagClass::CONTEXT, 4) => Ok(WrapperPDU::handshakeSecAbort(_decode_HandshakeSecAbort(
                &el,
            )?)),
            (TagClass::CONTEXT, 5) => Ok(WrapperPDU::handshakeProAbort(_decode_HandshakeProAbort(
                &el,
            )?)),
            (TagClass::CONTEXT, 6) => Ok(WrapperPDU::dtSecAbort(_decode_DtSecAbort(&el)?)),
            (TagClass::CONTEXT, 7) => Ok(WrapperPDU::applAbort(_decode_ApplAbort(&el)?)),
            (TagClass::CONTEXT, 8) => Ok(WrapperPDU::releaseReq(_decode_ReleaseReq(&el)?)),
            (TagClass::CONTEXT, 9) => Ok(WrapperPDU::releaseRsp(_decode_ReleaseRsp(&el)?)),
            (TagClass::CONTEXT, 10) => Ok(WrapperPDU::dataTransferClient(
                |el: &X690Element| -> ASN1Result<DataTransferClient> {
                    Ok(_decode_DataTransferClient(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 11) => Ok(WrapperPDU::dataTransferServer(
                |el: &X690Element| -> ASN1Result<DataTransferServer> {
                    Ok(_decode_DataTransferServer(&el.inner()?)?)
                }(&el)?,
            )),
            _ => Ok(WrapperPDU::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_WrapperPDU(value_: &WrapperPDU) -> ASN1Result<X690Element> {
    |value: &WrapperPDU| -> ASN1Result<X690Element> {
        match value {
            WrapperPDU::handshakeReq(v) => |v_1: &HandshakeReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_HandshakeReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            WrapperPDU::handshakeAcc(v) => |v_1: &HandshakeAcc| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_HandshakeAcc(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            WrapperPDU::handshakeWrpRej(v) => |v_1: &HandshakeWrpRej| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_HandshakeWrpRej(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            WrapperPDU::handshakeProRej(v) => |v_1: &HandshakeProRej| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_HandshakeProRej(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            WrapperPDU::handshakeSecAbort(v) => {
                |v_1: &HandshakeSecAbort| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_HandshakeSecAbort(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 4;
                    Ok(el_1)
                }(&v)
            }
            WrapperPDU::handshakeProAbort(v) => {
                |v_1: &HandshakeProAbort| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_HandshakeProAbort(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 5;
                    Ok(el_1)
                }(&v)
            }
            WrapperPDU::dtSecAbort(v) => |v_1: &DtSecAbort| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DtSecAbort(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v),
            WrapperPDU::applAbort(v) => |v_1: &ApplAbort| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ApplAbort(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v),
            WrapperPDU::releaseReq(v) => |v_1: &ReleaseReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReleaseReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 8;
                Ok(el_1)
            }(&v),
            WrapperPDU::releaseRsp(v) => |v_1: &ReleaseRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReleaseRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 9;
                Ok(el_1)
            }(&v),
            WrapperPDU::dataTransferClient(v) => {
                |v_1: &DataTransferClient| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        10,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_DataTransferClient(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            WrapperPDU::dataTransferServer(v) => {
                |v_1: &DataTransferServer| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        11,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_DataTransferServer(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            WrapperPDU::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HandshakeReq  ::=  Signed{TbsHandshakeReq}
/// ```
pub type HandshakeReq = Signed<TbsHandshakeReq>; // DefinedType

pub fn _decode_HandshakeReq(el: &X690Element) -> ASN1Result<HandshakeReq> {
    _decode_Signed::<TbsHandshakeReq>(_decode_TbsHandshakeReq, &el)
}

pub fn _encode_HandshakeReq(value_: &HandshakeReq) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsHandshakeReq>(_encode_TbsHandshakeReq, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeReq ::= SEQUENCE {
///   version        Version DEFAULT {v1},
///   prProt         WRAPPED-PROT.&id ({SupportedProtSet}),
///   sigAlg         AlgorithmIdentifier {{SupportedSignatureAlgorithms}},
///   altSigAlg  [0] AlgorithmIdentifier {{SupportedAltSignatureAlgorithms}} OPTIONAL,
///   pkiPath        DER-PkiPath,
///   assoID         AssoID,
///   time           TimeStamp,
///   keyEst         AlgorithmWithInvoke{{SupportedKeyEstablishmentAlgos}},
///   altKeyEst  [1] AlgorithmWithInvoke{{SupportedAltKeyEstablishmentAlgos}} OPTIONAL,
///   encr-mode      CHOICE {
///     aead       [2] SEQUENCE SIZE (1..MAX) OF
///       algo           AlgorithmIdentifier{{SupportedAeadAlgorithms}},
///     non-aead   [3] SEQUENCE {
///       encr       [0] SEQUENCE SIZE (1..MAX) OF
///         algo           AlgorithmIdentifier{{SupportedSymmetricKeyAlgorithms}}
///                          OPTIONAL,
///       icvAlgID   [1] SEQUENCE SIZE (1..MAX) OF
///         algo           AlgorithmIdentifier{{SupportedIcvAlgorithms}} },
///     ... },
///   attCert        DER-AttributeCertificate OPTIONAL,
///   applData   [4] WrappedProt{{SupportedProtSet}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeReq {
    pub version: OPTIONAL<Version>,
    pub prProt: OBJECT_IDENTIFIER,
    pub sigAlg: AlgorithmIdentifier,
    pub altSigAlg: OPTIONAL<AlgorithmIdentifier>,
    pub pkiPath: DER_PkiPath,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub keyEst: AlgorithmWithInvoke,
    pub altKeyEst: OPTIONAL<AlgorithmWithInvoke>,
    pub encr_mode: TbsHandshakeReq_encr_mode,
    pub attCert: OPTIONAL<DER_AttributeCertificate>,
    pub applData: OPTIONAL<WrappedProt>,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsHandshakeReq {
    pub fn new(
        version: OPTIONAL<Version>,
        prProt: OBJECT_IDENTIFIER,
        sigAlg: AlgorithmIdentifier,
        altSigAlg: OPTIONAL<AlgorithmIdentifier>,
        pkiPath: DER_PkiPath,
        assoID: AssoID,
        time: TimeStamp,
        keyEst: AlgorithmWithInvoke,
        altKeyEst: OPTIONAL<AlgorithmWithInvoke>,
        encr_mode: TbsHandshakeReq_encr_mode,
        attCert: OPTIONAL<DER_AttributeCertificate>,
        applData: OPTIONAL<WrappedProt>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsHandshakeReq {
            version,
            prProt,
            sigAlg,
            altSigAlg,
            pkiPath,
            assoID,
            time,
            keyEst,
            altKeyEst,
            encr_mode,
            attCert,
            applData,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        BIT_STRING::with_bits_set(&[Version_v1])
    }
}
impl TryFrom<X690Element> for TbsHandshakeReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeReq(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeReq: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "prProt",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "altSigAlg",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyEst",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "altKeyEst",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new("encr-mode", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "attCert",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "applData",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeReq: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeReq(el: &X690Element) -> ASN1Result<TbsHandshakeReq> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeReq,
            _eal_components_for_TbsHandshakeReq,
            _rctl2_components_for_TbsHandshakeReq,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let prProt = ber_decode_object_identifier(_components.get("prProt").unwrap())?;
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let altSigAlg: OPTIONAL<AlgorithmIdentifier> = match _components.get("altSigAlg") {
            Some(c_) => Some(_decode_AlgorithmIdentifier(c_)?),
            _ => None,
        };
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let keyEst = _decode_AlgorithmWithInvoke(_components.get("keyEst").unwrap())?;
        let altKeyEst: OPTIONAL<AlgorithmWithInvoke> = match _components.get("altKeyEst") {
            Some(c_) => Some(_decode_AlgorithmWithInvoke(c_)?),
            _ => None,
        };
        let encr_mode = _decode_TbsHandshakeReq_encr_mode(_components.get("encr-mode").unwrap())?;
        let attCert: OPTIONAL<DER_AttributeCertificate> = match _components.get("attCert") {
            Some(c_) => Some(_decode_DER_AttributeCertificate(c_)?),
            _ => None,
        };
        let applData: OPTIONAL<WrappedProt> = match _components.get("applData") {
            Some(c_) => Some(_decode_WrappedProt(c_)?),
            _ => None,
        };
        Ok(TbsHandshakeReq {
            version,
            prProt,
            sigAlg,
            altSigAlg,
            pkiPath,
            assoID,
            time,
            keyEst,
            altKeyEst,
            encr_mode,
            attCert,
            applData,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsHandshakeReq(value_: &TbsHandshakeReq) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(22);
        if let Some(v_) = &value_.version {
            if *v_ != TbsHandshakeReq::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(ber_encode_object_identifier(&value_.prProt)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        if let Some(v_) = &value_.altSigAlg {
            components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_AlgorithmWithInvoke(&value_.keyEst)?);
        if let Some(v_) = &value_.altKeyEst {
            components_.push(|v_1: &AlgorithmWithInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmWithInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_TbsHandshakeReq_encr_mode(&value_.encr_mode)?);
        if let Some(v_) = &value_.attCert {
            components_.push(_encode_DER_AttributeCertificate(&v_)?);
        }
        if let Some(v_) = &value_.applData {
            components_.push(|v_1: &WrappedProt| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_WrappedProt(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Version  ::=  BIT STRING {
///   v1 (0)  -- version 1
///   }
/// ```
pub type Version = BIT_STRING;

pub const Version_v1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_Version(el: &X690Element) -> ASN1Result<Version> {
    ber_decode_bit_string(&el)
}

pub fn _encode_Version(value_: &Version) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DER-PkiPath  ::=  OCTET STRING
///   (CONTAINING PkiPath ENCODED BY der)
/// ```
pub type DER_PkiPath = OCTET_STRING; // OctetStringType

pub fn _decode_DER_PkiPath(el: &X690Element) -> ASN1Result<DER_PkiPath> {
    ber_decode_octet_string(&el)
}

pub fn _encode_DER_PkiPath(value_: &DER_PkiPath) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DER-AttributeCertificate  ::=  OCTET STRING
///   (CONTAINING AttributeCertificate ENCODED BY der)
/// ```
pub type DER_AttributeCertificate = OCTET_STRING; // OctetStringType

pub fn _decode_DER_AttributeCertificate(el: &X690Element) -> ASN1Result<DER_AttributeCertificate> {
    ber_decode_octet_string(&el)
}

pub fn _encode_DER_AttributeCertificate(
    value_: &DER_AttributeCertificate,
) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// der OBJECT IDENTIFIER ::= {joint-iso-itu-t asn1(1) ber-derived(2) distinguished-encoding(1)}
/// ```
///
///
pub fn der() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        joint_iso_itu_t,
        /* asn1 */ 1,
        /* ber-derived */ 2,
        /* distinguished-encoding */ 1,
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AssoID  ::=  INTEGER (0..32767)
/// ```
pub type AssoID = INTEGER;

pub fn _decode_AssoID(el: &X690Element) -> ASN1Result<AssoID> {
    ber_decode_integer(&el)
}

pub fn _encode_AssoID(value_: &AssoID) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStamp  ::=  GeneralizedTime
/// ```
pub type TimeStamp = GeneralizedTime; // GeneralizedTime

pub fn _decode_TimeStamp(el: &X690Element) -> ASN1Result<TimeStamp> {
    ber_decode_generalized_time(&el)
}

pub fn _encode_TimeStamp(value_: &TimeStamp) -> ASN1Result<X690Element> {
    ber_encode_generalized_time(&value_)
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
/// SupportedAltSignatureAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAltSignatureAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedKeyEstablishmentAlgos ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedKeyEstablishmentAlgos() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAltKeyEstablishmentAlgos ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAltKeyEstablishmentAlgos() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAeadAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAeadAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedSymmetricKeyAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedSymmetricKeyAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedIcvAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedIcvAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HandshakeAcc  ::=  Signed{TbsHandshakeAcc}
/// ```
pub type HandshakeAcc = Signed<TbsHandshakeAcc>; // DefinedType

pub fn _decode_HandshakeAcc(el: &X690Element) -> ASN1Result<HandshakeAcc> {
    _decode_Signed::<TbsHandshakeAcc>(_decode_TbsHandshakeAcc, &el)
}

pub fn _encode_HandshakeAcc(value_: &HandshakeAcc) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsHandshakeAcc>(_encode_TbsHandshakeAcc, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeAcc ::= SEQUENCE {
///   version        Version DEFAULT {v1},
///   sigSel         CHOICE {
///     sigAlg         AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///     altSigAlg  [0] AlgorithmIdentifier{{SupportedAltSignatureAlgorithms}} },
///   pkiPath        DER-PkiPath,
///   assoID         AssoID,
///   time           TimeStamp,
///   keyEstSel      CHOICE {
///     keyEst         AlgorithmWithInvoke{{SupportedKeyEstablishmentAlgos}},
///     altKeyEst  [1] AlgorithmWithInvoke{{SupportedAltKeyEstablishmentAlgos}} },
///   encr-mode      CHOICE {
///     aead       [2] AlgorithmIdentifier{{SupportedAeadAlgorithms}},
///     non-aead   [3] SEQUENCE {
///       encr       [0] AlgorithmIdentifier{{SupportedSymmetricKeyAlgorithms}} OPTIONAL,
///       icvAlgID   [1] AlgorithmIdentifier{{SupportedIcvAlgorithms}} },
///     ... },
///   attCert        DER-AttributeCertificate OPTIONAL,
///   applData   [4] WrappedProt{{SupportedProtSet}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeAcc {
    pub version: OPTIONAL<Version>,
    pub sigSel: TbsHandshakeAcc_sigSel,
    pub pkiPath: DER_PkiPath,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub keyEstSel: TbsHandshakeAcc_keyEstSel,
    pub encr_mode: TbsHandshakeAcc_encr_mode,
    pub attCert: OPTIONAL<DER_AttributeCertificate>,
    pub applData: OPTIONAL<WrappedProt>,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsHandshakeAcc {
    pub fn new(
        version: OPTIONAL<Version>,
        sigSel: TbsHandshakeAcc_sigSel,
        pkiPath: DER_PkiPath,
        assoID: AssoID,
        time: TimeStamp,
        keyEstSel: TbsHandshakeAcc_keyEstSel,
        encr_mode: TbsHandshakeAcc_encr_mode,
        attCert: OPTIONAL<DER_AttributeCertificate>,
        applData: OPTIONAL<WrappedProt>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsHandshakeAcc {
            version,
            sigSel,
            pkiPath,
            assoID,
            time,
            keyEstSel,
            encr_mode,
            attCert,
            applData,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        BIT_STRING::with_bits_set(&[Version_v1])
    }
}
impl TryFrom<X690Element> for TbsHandshakeAcc {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeAcc {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeAcc: &[ComponentSpec; 9] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new("sigSel", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new("keyEstSel", false, TagSelector::any, None, None),
    ComponentSpec::new("encr-mode", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "attCert",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "applData",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeAcc: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeAcc: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeAcc(el: &X690Element) -> ASN1Result<TbsHandshakeAcc> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeAcc> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeAcc,
            _eal_components_for_TbsHandshakeAcc,
            _rctl2_components_for_TbsHandshakeAcc,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let sigSel = _decode_TbsHandshakeAcc_sigSel(_components.get("sigSel").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let keyEstSel = _decode_TbsHandshakeAcc_keyEstSel(_components.get("keyEstSel").unwrap())?;
        let encr_mode = _decode_TbsHandshakeAcc_encr_mode(_components.get("encr-mode").unwrap())?;
        let attCert: OPTIONAL<DER_AttributeCertificate> = match _components.get("attCert") {
            Some(c_) => Some(_decode_DER_AttributeCertificate(c_)?),
            _ => None,
        };
        let applData: OPTIONAL<WrappedProt> = match _components.get("applData") {
            Some(c_) => Some(_decode_WrappedProt(c_)?),
            _ => None,
        };
        Ok(TbsHandshakeAcc {
            version,
            sigSel,
            pkiPath,
            assoID,
            time,
            keyEstSel,
            encr_mode,
            attCert,
            applData,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsHandshakeAcc(value_: &TbsHandshakeAcc) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeAcc| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(19);
        if let Some(v_) = &value_.version {
            if *v_ != TbsHandshakeAcc::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(_encode_TbsHandshakeAcc_sigSel(&value_.sigSel)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_TbsHandshakeAcc_keyEstSel(&value_.keyEstSel)?);
        components_.push(_encode_TbsHandshakeAcc_encr_mode(&value_.encr_mode)?);
        if let Some(v_) = &value_.attCert {
            components_.push(_encode_DER_AttributeCertificate(&v_)?);
        }
        if let Some(v_) = &value_.applData {
            components_.push(|v_1: &WrappedProt| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_WrappedProt(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HandshakeWrpRej  ::=  Signed{TbsHandshakeWrpRej}
/// ```
pub type HandshakeWrpRej = Signed<TbsHandshakeWrpRej>; // DefinedType

pub fn _decode_HandshakeWrpRej(el: &X690Element) -> ASN1Result<HandshakeWrpRej> {
    _decode_Signed::<TbsHandshakeWrpRej>(_decode_TbsHandshakeWrpRej, &el)
}

pub fn _encode_HandshakeWrpRej(value_: &HandshakeWrpRej) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsHandshakeWrpRej>(_encode_TbsHandshakeWrpRej, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeWrpRej ::= SEQUENCE {
///   version        Version DEFAULT {v1},
///   sigSel         CHOICE {
///     sigAlg         AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///     altSigAlg  [0] AlgorithmIdentifier{{SupportedAltSignatureAlgorithms}} },
///   assoID         AssoID,
///   time           TimeStamp,
///   pkiPath        DER-PkiPath,
///   diag           WrpError OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeWrpRej {
    pub version: OPTIONAL<Version>,
    pub sigSel: TbsHandshakeWrpRej_sigSel,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub diag: OPTIONAL<WrpError>,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsHandshakeWrpRej {
    pub fn new(
        version: OPTIONAL<Version>,
        sigSel: TbsHandshakeWrpRej_sigSel,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        diag: OPTIONAL<WrpError>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsHandshakeWrpRej {
            version,
            sigSel,
            assoID,
            time,
            pkiPath,
            diag,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        BIT_STRING::with_bits_set(&[Version_v1])
    }
}
impl TryFrom<X690Element> for TbsHandshakeWrpRej {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeWrpRej(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeWrpRej {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeWrpRej(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeWrpRej: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new("sigSel", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diag",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeWrpRej: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeWrpRej: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeWrpRej(el: &X690Element) -> ASN1Result<TbsHandshakeWrpRej> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeWrpRej> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeWrpRej,
            _eal_components_for_TbsHandshakeWrpRej,
            _rctl2_components_for_TbsHandshakeWrpRej,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let sigSel = _decode_TbsHandshakeWrpRej_sigSel(_components.get("sigSel").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let diag: OPTIONAL<WrpError> = match _components.get("diag") {
            Some(c_) => Some(_decode_WrpError(c_)?),
            _ => None,
        };
        Ok(TbsHandshakeWrpRej {
            version,
            sigSel,
            assoID,
            time,
            pkiPath,
            diag,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsHandshakeWrpRej(value_: &TbsHandshakeWrpRej) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeWrpRej| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.version {
            if *v_ != TbsHandshakeWrpRej::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(_encode_TbsHandshakeWrpRej_sigSel(&value_.sigSel)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        if let Some(v_) = &value_.diag {
            components_.push(_encode_WrpError(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HandshakeProRej  ::=  Signed{TbsHandshakeProRej}
/// ```
pub type HandshakeProRej = Signed<TbsHandshakeProRej>; // DefinedType

pub fn _decode_HandshakeProRej(el: &X690Element) -> ASN1Result<HandshakeProRej> {
    _decode_Signed::<TbsHandshakeProRej>(_decode_TbsHandshakeProRej, &el)
}

pub fn _encode_HandshakeProRej(value_: &HandshakeProRej) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsHandshakeProRej>(_encode_TbsHandshakeProRej, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeProRej ::= SEQUENCE {
///   sigSel         CHOICE {
///     sigAlg         AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///     altSigAlg  [0] AlgorithmIdentifier{{SupportedAltSignatureAlgorithms}} },
///   assoID         AssoID,
///   time           TimeStamp,
///   pkiPath        DER-PkiPath,
///   applData       WrappedProt{{SupportedProtSet}},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeProRej {
    pub sigSel: TbsHandshakeProRej_sigSel,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub applData: WrappedProt,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsHandshakeProRej {
    pub fn new(
        sigSel: TbsHandshakeProRej_sigSel,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        applData: WrappedProt,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsHandshakeProRej {
            sigSel,
            assoID,
            time,
            pkiPath,
            applData,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TbsHandshakeProRej {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeProRej(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeProRej {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeProRej(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeProRej: &[ComponentSpec; 5] = &[
    ComponentSpec::new("sigSel", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "applData",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeProRej: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeProRej: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeProRej(el: &X690Element) -> ASN1Result<TbsHandshakeProRej> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeProRej> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeProRej,
            _eal_components_for_TbsHandshakeProRej,
            _rctl2_components_for_TbsHandshakeProRej,
        )?;
        let sigSel = _decode_TbsHandshakeProRej_sigSel(_components.get("sigSel").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let applData = _decode_WrappedProt(_components.get("applData").unwrap())?;
        Ok(TbsHandshakeProRej {
            sigSel,
            assoID,
            time,
            pkiPath,
            applData,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsHandshakeProRej(value_: &TbsHandshakeProRej) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeProRej| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_TbsHandshakeProRej_sigSel(&value_.sigSel)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        components_.push(_encode_WrappedProt(&value_.applData)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HandshakeSecAbort  ::=  Signed{TbsHandshakeSecAbort}
/// ```
pub type HandshakeSecAbort = Signed<TbsHandshakeSecAbort>; // DefinedType

pub fn _decode_HandshakeSecAbort(el: &X690Element) -> ASN1Result<HandshakeSecAbort> {
    _decode_Signed::<TbsHandshakeSecAbort>(_decode_TbsHandshakeSecAbort, &el)
}

pub fn _encode_HandshakeSecAbort(value_: &HandshakeSecAbort) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsHandshakeSecAbort>(_encode_TbsHandshakeSecAbort, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeSecAbort ::= SEQUENCE {
///   version      Version DEFAULT {v1},
///   sigAlg       AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///   assoID       AssoID,
///   time         TimeStamp,
///   pkiPath      DER-PkiPath,
///   diag         WrpError OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeSecAbort {
    pub version: OPTIONAL<Version>,
    pub sigAlg: AlgorithmIdentifier,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub diag: OPTIONAL<WrpError>,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsHandshakeSecAbort {
    pub fn new(
        version: OPTIONAL<Version>,
        sigAlg: AlgorithmIdentifier,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        diag: OPTIONAL<WrpError>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsHandshakeSecAbort {
            version,
            sigAlg,
            assoID,
            time,
            pkiPath,
            diag,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        BIT_STRING::with_bits_set(&[Version_v1])
    }
}
impl TryFrom<X690Element> for TbsHandshakeSecAbort {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeSecAbort(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeSecAbort {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeSecAbort(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeSecAbort: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diag",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeSecAbort: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeSecAbort: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeSecAbort(el: &X690Element) -> ASN1Result<TbsHandshakeSecAbort> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeSecAbort> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeSecAbort,
            _eal_components_for_TbsHandshakeSecAbort,
            _rctl2_components_for_TbsHandshakeSecAbort,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let diag: OPTIONAL<WrpError> = match _components.get("diag") {
            Some(c_) => Some(_decode_WrpError(c_)?),
            _ => None,
        };
        Ok(TbsHandshakeSecAbort {
            version,
            sigAlg,
            assoID,
            time,
            pkiPath,
            diag,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsHandshakeSecAbort(value_: &TbsHandshakeSecAbort) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeSecAbort| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.version {
            if *v_ != TbsHandshakeSecAbort::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        if let Some(v_) = &value_.diag {
            components_.push(_encode_WrpError(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HandshakeProAbort  ::=  Signed{TbsHandshakeProAbort}
/// ```
pub type HandshakeProAbort = Signed<TbsHandshakeProAbort>; // DefinedType

pub fn _decode_HandshakeProAbort(el: &X690Element) -> ASN1Result<HandshakeProAbort> {
    _decode_Signed::<TbsHandshakeProAbort>(_decode_TbsHandshakeProAbort, &el)
}

pub fn _encode_HandshakeProAbort(value_: &HandshakeProAbort) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsHandshakeProAbort>(_encode_TbsHandshakeProAbort, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeProAbort ::= SEQUENCE {
///   sigAlg       AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///   assoID       AssoID,
///   time         TimeStamp,
///   pkiPath      DER-PkiPath,
///   applData     WrappedProt{{SupportedProtSet}},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeProAbort {
    pub sigAlg: AlgorithmIdentifier,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub applData: WrappedProt,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsHandshakeProAbort {
    pub fn new(
        sigAlg: AlgorithmIdentifier,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        applData: WrappedProt,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsHandshakeProAbort {
            sigAlg,
            assoID,
            time,
            pkiPath,
            applData,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TbsHandshakeProAbort {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeProAbort(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeProAbort {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeProAbort(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeProAbort: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "applData",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeProAbort: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeProAbort: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeProAbort(el: &X690Element) -> ASN1Result<TbsHandshakeProAbort> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeProAbort> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeProAbort,
            _eal_components_for_TbsHandshakeProAbort,
            _rctl2_components_for_TbsHandshakeProAbort,
        )?;
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let applData = _decode_WrappedProt(_components.get("applData").unwrap())?;
        Ok(TbsHandshakeProAbort {
            sigAlg,
            assoID,
            time,
            pkiPath,
            applData,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsHandshakeProAbort(value_: &TbsHandshakeProAbort) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeProAbort| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        components_.push(_encode_WrappedProt(&value_.applData)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DtSecAbort  ::=  Signed{TbsDtSecAbort}
/// ```
pub type DtSecAbort = Signed<TbsDtSecAbort>; // DefinedType

pub fn _decode_DtSecAbort(el: &X690Element) -> ASN1Result<DtSecAbort> {
    _decode_Signed::<TbsDtSecAbort>(_decode_TbsDtSecAbort, &el)
}

pub fn _encode_DtSecAbort(value_: &DtSecAbort) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsDtSecAbort>(_encode_TbsDtSecAbort, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsDtSecAbort ::= SEQUENCE {
///   sigAlg       AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///   assoID       AssoID,
///   time         TimeStamp,
///   pkiPath      DER-PkiPath,
///   seq          SequenceNumber,
///   diag         WrpError OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsDtSecAbort {
    pub sigAlg: AlgorithmIdentifier,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub seq: SequenceNumber,
    pub diag: OPTIONAL<WrpError>,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsDtSecAbort {
    pub fn new(
        sigAlg: AlgorithmIdentifier,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        seq: SequenceNumber,
        diag: OPTIONAL<WrpError>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsDtSecAbort {
            sigAlg,
            assoID,
            time,
            pkiPath,
            seq,
            diag,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TbsDtSecAbort {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsDtSecAbort(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsDtSecAbort {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsDtSecAbort(el)
    }
}

pub const _rctl1_components_for_TbsDtSecAbort: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diag",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsDtSecAbort: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsDtSecAbort: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsDtSecAbort(el: &X690Element) -> ASN1Result<TbsDtSecAbort> {
    |el_: &X690Element| -> ASN1Result<TbsDtSecAbort> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsDtSecAbort,
            _eal_components_for_TbsDtSecAbort,
            _rctl2_components_for_TbsDtSecAbort,
        )?;
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let diag: OPTIONAL<WrpError> = match _components.get("diag") {
            Some(c_) => Some(_decode_WrpError(c_)?),
            _ => None,
        };
        Ok(TbsDtSecAbort {
            sigAlg,
            assoID,
            time,
            pkiPath,
            seq,
            diag,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsDtSecAbort(value_: &TbsDtSecAbort) -> ASN1Result<X690Element> {
    |value_: &TbsDtSecAbort| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.diag {
            components_.push(_encode_WrpError(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ApplAbort  ::=  Signed{TbsApplAbort}
/// ```
pub type ApplAbort = Signed<TbsApplAbort>; // DefinedType

pub fn _decode_ApplAbort(el: &X690Element) -> ASN1Result<ApplAbort> {
    _decode_Signed::<TbsApplAbort>(_decode_TbsApplAbort, &el)
}

pub fn _encode_ApplAbort(value_: &ApplAbort) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsApplAbort>(_encode_TbsApplAbort, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsApplAbort ::= SEQUENCE {
///   sigAlg       AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///   assoID       AssoID,
///   time         TimeStamp,
///   pkiPath      DER-PkiPath,
///   seq          SequenceNumber,
///   applData     WrappedProt{{SupportedProtSet}},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsApplAbort {
    pub sigAlg: AlgorithmIdentifier,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub seq: SequenceNumber,
    pub applData: WrappedProt,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsApplAbort {
    pub fn new(
        sigAlg: AlgorithmIdentifier,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        seq: SequenceNumber,
        applData: WrappedProt,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsApplAbort {
            sigAlg,
            assoID,
            time,
            pkiPath,
            seq,
            applData,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TbsApplAbort {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsApplAbort(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsApplAbort {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsApplAbort(el)
    }
}

pub const _rctl1_components_for_TbsApplAbort: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "applData",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsApplAbort: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsApplAbort: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsApplAbort(el: &X690Element) -> ASN1Result<TbsApplAbort> {
    |el_: &X690Element| -> ASN1Result<TbsApplAbort> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsApplAbort,
            _eal_components_for_TbsApplAbort,
            _rctl2_components_for_TbsApplAbort,
        )?;
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let applData = _decode_WrappedProt(_components.get("applData").unwrap())?;
        Ok(TbsApplAbort {
            sigAlg,
            assoID,
            time,
            pkiPath,
            seq,
            applData,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsApplAbort(value_: &TbsApplAbort) -> ASN1Result<X690Element> {
    |value_: &TbsApplAbort| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        components_.push(_encode_WrappedProt(&value_.applData)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReleaseReq  ::=  Signed{TbsReleaseReq}
/// ```
pub type ReleaseReq = Signed<TbsReleaseReq>; // DefinedType

pub fn _decode_ReleaseReq(el: &X690Element) -> ASN1Result<ReleaseReq> {
    _decode_Signed::<TbsReleaseReq>(_decode_TbsReleaseReq, &el)
}

pub fn _encode_ReleaseReq(value_: &ReleaseReq) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsReleaseReq>(_encode_TbsReleaseReq, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsReleaseReq ::= SEQUENCE {
///   version      Version DEFAULT {v1},
///   sigAlg       AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///   assoID       AssoID,
///   time         TimeStamp,
///   pkiPath      DER-PkiPath,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsReleaseReq {
    pub version: OPTIONAL<Version>,
    pub sigAlg: AlgorithmIdentifier,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsReleaseReq {
    pub fn new(
        version: OPTIONAL<Version>,
        sigAlg: AlgorithmIdentifier,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsReleaseReq {
            version,
            sigAlg,
            assoID,
            time,
            pkiPath,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        BIT_STRING::with_bits_set(&[Version_v1])
    }
}
impl TryFrom<X690Element> for TbsReleaseReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsReleaseReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsReleaseReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsReleaseReq(el)
    }
}

pub const _rctl1_components_for_TbsReleaseReq: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsReleaseReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsReleaseReq: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsReleaseReq(el: &X690Element) -> ASN1Result<TbsReleaseReq> {
    |el_: &X690Element| -> ASN1Result<TbsReleaseReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsReleaseReq,
            _eal_components_for_TbsReleaseReq,
            _rctl2_components_for_TbsReleaseReq,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        Ok(TbsReleaseReq {
            version,
            sigAlg,
            assoID,
            time,
            pkiPath,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsReleaseReq(value_: &TbsReleaseReq) -> ASN1Result<X690Element> {
    |value_: &TbsReleaseReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.version {
            if *v_ != TbsReleaseReq::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReleaseRsp  ::=  Signed{TbsReleaseRsp}
/// ```
pub type ReleaseRsp = Signed<TbsReleaseRsp>; // DefinedType

pub fn _decode_ReleaseRsp(el: &X690Element) -> ASN1Result<ReleaseRsp> {
    _decode_Signed::<TbsReleaseRsp>(_decode_TbsReleaseRsp, &el)
}

pub fn _encode_ReleaseRsp(value_: &ReleaseRsp) -> ASN1Result<X690Element> {
    _encode_Signed::<TbsReleaseRsp>(_encode_TbsReleaseRsp, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsReleaseRsp ::= SEQUENCE {
///   version      Version DEFAULT {v1},
///   sigAlg       AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///   assoID       AssoID,
///   time         TimeStamp,
///   pkiPath      DER-PkiPath,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsReleaseRsp {
    pub version: OPTIONAL<Version>,
    pub sigAlg: AlgorithmIdentifier,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub pkiPath: DER_PkiPath,
    pub _unrecognized: Vec<X690Element>,
}
impl TbsReleaseRsp {
    pub fn new(
        version: OPTIONAL<Version>,
        sigAlg: AlgorithmIdentifier,
        assoID: AssoID,
        time: TimeStamp,
        pkiPath: DER_PkiPath,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbsReleaseRsp {
            version,
            sigAlg,
            assoID,
            time,
            pkiPath,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        BIT_STRING::with_bits_set(&[Version_v1])
    }
}
impl TryFrom<X690Element> for TbsReleaseRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsReleaseRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsReleaseRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsReleaseRsp(el)
    }
}

pub const _rctl1_components_for_TbsReleaseRsp: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sigAlg",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pkiPath",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsReleaseRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsReleaseRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsReleaseRsp(el: &X690Element) -> ASN1Result<TbsReleaseRsp> {
    |el_: &X690Element| -> ASN1Result<TbsReleaseRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsReleaseRsp,
            _eal_components_for_TbsReleaseRsp,
            _rctl2_components_for_TbsReleaseRsp,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let sigAlg = _decode_AlgorithmIdentifier(_components.get("sigAlg").unwrap())?;
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let pkiPath = _decode_DER_PkiPath(_components.get("pkiPath").unwrap())?;
        Ok(TbsReleaseRsp {
            version,
            sigAlg,
            assoID,
            time,
            pkiPath,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbsReleaseRsp(value_: &TbsReleaseRsp) -> ASN1Result<X690Element> {
    |value_: &TbsReleaseRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.version {
            if *v_ != TbsReleaseRsp::_default_value_for_version() {
                components_.push(_encode_Version(&v_)?);
            }
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.sigAlg)?);
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_DER_PkiPath(&value_.pkiPath)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DataTransferClient  ::=  CHOICE {
///   aead     [0] DataTransferClientAE,
///   non-aead [1] DataTransferClientNEA,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum DataTransferClient {
    aead(DataTransferClientAE),
    non_aead(DataTransferClientNEA),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for DataTransferClient {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DataTransferClient(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DataTransferClient {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DataTransferClient(el)
    }
}

pub fn _decode_DataTransferClient(el: &X690Element) -> ASN1Result<DataTransferClient> {
    |el: &X690Element| -> ASN1Result<DataTransferClient> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(DataTransferClient::aead(_decode_DataTransferClientAE(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(DataTransferClient::non_aead(
                _decode_DataTransferClientNEA(&el)?,
            )),
            _ => Ok(DataTransferClient::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_DataTransferClient(value_: &DataTransferClient) -> ASN1Result<X690Element> {
    |value: &DataTransferClient| -> ASN1Result<X690Element> {
        match value {
            DataTransferClient::aead(v) => {
                |v_1: &DataTransferClientAE| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_DataTransferClientAE(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            DataTransferClient::non_aead(v) => {
                |v_1: &DataTransferClientNEA| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_DataTransferClientNEA(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            DataTransferClient::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DataTransferClientAE  ::=  AUTHEN-ENCRYPT{AadClientAE, WRAPPED-PROT.&Type}
/// ```
pub type DataTransferClientAE = AUTHEN_ENCRYPT<AadClientAE, X690Element>; // DefinedType

pub fn _decode_DataTransferClientAE(el: &X690Element) -> ASN1Result<DataTransferClientAE> {
    _decode_AUTHEN_ENCRYPT::<AadClientAE, X690Element>(_decode_AadClientAE, x690_identity, &el)
}

pub fn _encode_DataTransferClientAE(value_: &DataTransferClientAE) -> ASN1Result<X690Element> {
    _encode_AUTHEN_ENCRYPT::<AadClientAE, X690Element>(_encode_AadClientAE, x690_identity, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AadClientAE ::= SEQUENCE {
///   COMPONENTS OF    AadClient,
///   encInvoke    [3] AlgoInvoke{{SupportedAeadAlgorithms}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AadClientAE {
    pub invokeID: OPTIONAL<InvokeID>, /* REPLICATED_COMPONENT */
    pub assoID: AssoID,               /* REPLICATED_COMPONENT */
    pub time: TimeStamp,              /* REPLICATED_COMPONENT */
    pub seq: SequenceNumber,          /* REPLICATED_COMPONENT */
    pub keyEst: OPTIONAL<AlgoInvoke>, /* REPLICATED_COMPONENT */
    pub encInvoke: OPTIONAL<AlgoInvoke>,
    pub _unrecognized: Vec<X690Element>,
}
impl AadClientAE {
    pub fn new(
        invokeID: OPTIONAL<InvokeID>, /* REPLICATED_COMPONENT */
        assoID: AssoID,               /* REPLICATED_COMPONENT */
        time: TimeStamp,              /* REPLICATED_COMPONENT */
        seq: SequenceNumber,          /* REPLICATED_COMPONENT */
        keyEst: OPTIONAL<AlgoInvoke>, /* REPLICATED_COMPONENT */
        encInvoke: OPTIONAL<AlgoInvoke>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AadClientAE {
            invokeID,
            assoID,
            time,
            seq,
            keyEst,
            encInvoke,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AadClientAE {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AadClientAE(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AadClientAE {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AadClientAE(el)
    }
}

pub const _rctl1_components_for_AadClientAE: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "invokeID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyEst",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encInvoke",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AadClientAE: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AadClientAE: &[ComponentSpec; 0] = &[];

pub fn _decode_AadClientAE(el: &X690Element) -> ASN1Result<AadClientAE> {
    |el_: &X690Element| -> ASN1Result<AadClientAE> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AadClientAE,
            _eal_components_for_AadClientAE,
            _rctl2_components_for_AadClientAE,
        )?;
        let invokeID: OPTIONAL<InvokeID> = match _components.get("invokeID") {
            Some(c_) => Some(_decode_InvokeID(c_)?),
            _ => None,
        };
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let keyEst: OPTIONAL<AlgoInvoke> = match _components.get("keyEst") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        let encInvoke: OPTIONAL<AlgoInvoke> = match _components.get("encInvoke") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        Ok(AadClientAE {
            invokeID,
            assoID,
            time,
            seq,
            keyEst,
            encInvoke,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AadClientAE(value_: &AadClientAE) -> ASN1Result<X690Element> {
    |value_: &AadClientAE| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.invokeID {
            components_.push(|v_1: &InvokeID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InvokeID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.keyEst {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.encInvoke {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DataTransferClientNEA  ::=  ICV-Invoke{TbpDataTransferClient}
/// ```
pub type DataTransferClientNEA = ICV_Invoke<TbpDataTransferClient>; // DefinedType

pub fn _decode_DataTransferClientNEA(el: &X690Element) -> ASN1Result<DataTransferClientNEA> {
    _decode_ICV_Invoke::<TbpDataTransferClient>(_decode_TbpDataTransferClient, &el)
}

pub fn _encode_DataTransferClientNEA(value_: &DataTransferClientNEA) -> ASN1Result<X690Element> {
    _encode_ICV_Invoke::<TbpDataTransferClient>(_encode_TbpDataTransferClient, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbpDataTransferClient ::= SEQUENCE {
///   COMPONENTS OF    AadClient,
///   encEnvoke    [3] AlgoInvoke{{SupportedSymmetricKeyAlgorithms}} OPTIONAL,
///   conf             CHOICE {
///     clear        [4] WrappedProt{{SupportedProtSet}},
///     protected    [5] ENCIPHERED{WRAPPED-PROT.&Type},
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbpDataTransferClient {
    pub invokeID: OPTIONAL<InvokeID>, /* REPLICATED_COMPONENT */
    pub assoID: AssoID,               /* REPLICATED_COMPONENT */
    pub time: TimeStamp,              /* REPLICATED_COMPONENT */
    pub seq: SequenceNumber,          /* REPLICATED_COMPONENT */
    pub keyEst: OPTIONAL<AlgoInvoke>, /* REPLICATED_COMPONENT */
    pub encEnvoke: OPTIONAL<AlgoInvoke>,
    pub conf: TbpDataTransferClient_conf,
    pub _unrecognized: Vec<X690Element>,
}
impl TbpDataTransferClient {
    pub fn new(
        invokeID: OPTIONAL<InvokeID>, /* REPLICATED_COMPONENT */
        assoID: AssoID,               /* REPLICATED_COMPONENT */
        time: TimeStamp,              /* REPLICATED_COMPONENT */
        seq: SequenceNumber,          /* REPLICATED_COMPONENT */
        keyEst: OPTIONAL<AlgoInvoke>, /* REPLICATED_COMPONENT */
        encEnvoke: OPTIONAL<AlgoInvoke>,
        conf: TbpDataTransferClient_conf,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbpDataTransferClient {
            invokeID,
            assoID,
            time,
            seq,
            keyEst,
            encEnvoke,
            conf,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TbpDataTransferClient {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferClient(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbpDataTransferClient {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferClient(el)
    }
}

pub const _rctl1_components_for_TbpDataTransferClient: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "invokeID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyEst",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encEnvoke",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new("conf", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_TbpDataTransferClient: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbpDataTransferClient: &[ComponentSpec; 0] = &[];

pub fn _decode_TbpDataTransferClient(el: &X690Element) -> ASN1Result<TbpDataTransferClient> {
    |el_: &X690Element| -> ASN1Result<TbpDataTransferClient> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbpDataTransferClient,
            _eal_components_for_TbpDataTransferClient,
            _rctl2_components_for_TbpDataTransferClient,
        )?;
        let invokeID: OPTIONAL<InvokeID> = match _components.get("invokeID") {
            Some(c_) => Some(_decode_InvokeID(c_)?),
            _ => None,
        };
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let keyEst: OPTIONAL<AlgoInvoke> = match _components.get("keyEst") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        let encEnvoke: OPTIONAL<AlgoInvoke> = match _components.get("encEnvoke") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        let conf = _decode_TbpDataTransferClient_conf(_components.get("conf").unwrap())?;
        Ok(TbpDataTransferClient {
            invokeID,
            assoID,
            time,
            seq,
            keyEst,
            encEnvoke,
            conf,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbpDataTransferClient(value_: &TbpDataTransferClient) -> ASN1Result<X690Element> {
    |value_: &TbpDataTransferClient| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        if let Some(v_) = &value_.invokeID {
            components_.push(|v_1: &InvokeID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InvokeID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.keyEst {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.encEnvoke {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_TbpDataTransferClient_conf(&value_.conf)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AadClient ::= SEQUENCE {
///   invokeID [0] InvokeID OPTIONAL,
///   assoID       AssoID,
///   time         TimeStamp,
///   seq          SequenceNumber,
///   keyEst   [2] AlgoInvoke{{SupportedKeyEstablishmentAlgos}} OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AadClient {
    pub invokeID: OPTIONAL<InvokeID>,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub seq: SequenceNumber,
    pub keyEst: OPTIONAL<AlgoInvoke>,
}
impl AadClient {
    pub fn new(
        invokeID: OPTIONAL<InvokeID>,
        assoID: AssoID,
        time: TimeStamp,
        seq: SequenceNumber,
        keyEst: OPTIONAL<AlgoInvoke>,
    ) -> Self {
        AadClient {
            invokeID,
            assoID,
            time,
            seq,
            keyEst,
        }
    }
}
impl TryFrom<X690Element> for AadClient {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AadClient(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AadClient {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AadClient(el)
    }
}

pub const _rctl1_components_for_AadClient: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "invokeID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyEst",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AadClient: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AadClient: &[ComponentSpec; 0] = &[];

pub fn _decode_AadClient(el: &X690Element) -> ASN1Result<AadClient> {
    |el_: &X690Element| -> ASN1Result<AadClient> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AadClient,
            _eal_components_for_AadClient,
            _rctl2_components_for_AadClient,
        )?;
        let invokeID: OPTIONAL<InvokeID> = match _components.get("invokeID") {
            Some(c_) => Some(_decode_InvokeID(c_)?),
            _ => None,
        };
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let keyEst: OPTIONAL<AlgoInvoke> = match _components.get("keyEst") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        Ok(AadClient {
            invokeID,
            assoID,
            time,
            seq,
            keyEst,
        })
    }(&el)
}

pub fn _encode_AadClient(value_: &AadClient) -> ASN1Result<X690Element> {
    |value_: &AadClient| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(10);
        if let Some(v_) = &value_.invokeID {
            components_.push(|v_1: &InvokeID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InvokeID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.keyEst {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
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
/// InvokeID  ::=  OCTET STRING (SIZE (6))
/// ```
pub type InvokeID = OCTET_STRING; // OctetStringType

pub fn _decode_InvokeID(el: &X690Element) -> ASN1Result<InvokeID> {
    ber_decode_octet_string(&el)
}

pub fn _encode_InvokeID(value_: &InvokeID) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SequenceNumber  ::=  INTEGER (0..2147483647)
/// ```
pub type SequenceNumber = INTEGER;

pub fn _decode_SequenceNumber(el: &X690Element) -> ASN1Result<SequenceNumber> {
    ber_decode_integer(&el)
}

pub fn _encode_SequenceNumber(value_: &SequenceNumber) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DataTransferServer  ::=  CHOICE {
///   aead     [0] DataTransferServerAE,
///   non-aead [1] DataTransferServerNEA,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum DataTransferServer {
    aead(DataTransferServerAE),
    non_aead(DataTransferServerNEA),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for DataTransferServer {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DataTransferServer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DataTransferServer {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DataTransferServer(el)
    }
}

pub fn _decode_DataTransferServer(el: &X690Element) -> ASN1Result<DataTransferServer> {
    |el: &X690Element| -> ASN1Result<DataTransferServer> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(DataTransferServer::aead(_decode_DataTransferServerAE(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(DataTransferServer::non_aead(
                _decode_DataTransferServerNEA(&el)?,
            )),
            _ => Ok(DataTransferServer::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_DataTransferServer(value_: &DataTransferServer) -> ASN1Result<X690Element> {
    |value: &DataTransferServer| -> ASN1Result<X690Element> {
        match value {
            DataTransferServer::aead(v) => {
                |v_1: &DataTransferServerAE| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_DataTransferServerAE(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            DataTransferServer::non_aead(v) => {
                |v_1: &DataTransferServerNEA| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_DataTransferServerNEA(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            DataTransferServer::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DataTransferServerAE  ::=  AUTHEN-ENCRYPT{AadServerAE, WRAPPED-PROT.&Type}
/// ```
pub type DataTransferServerAE = AUTHEN_ENCRYPT<AadServerAE, X690Element>; // DefinedType

pub fn _decode_DataTransferServerAE(el: &X690Element) -> ASN1Result<DataTransferServerAE> {
    _decode_AUTHEN_ENCRYPT::<AadServerAE, X690Element>(_decode_AadServerAE, x690_identity, &el)
}

pub fn _encode_DataTransferServerAE(value_: &DataTransferServerAE) -> ASN1Result<X690Element> {
    _encode_AUTHEN_ENCRYPT::<AadServerAE, X690Element>(_encode_AadServerAE, x690_identity, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AadServerAE ::= SEQUENCE {
///   COMPONENTS OF AadServer,
///   encInvoke [3] AlgoInvoke{{SupportedAeadAlgorithms}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AadServerAE {
    pub invokeID: OPTIONAL<InvokeID>,  /* REPLICATED_COMPONENT */
    pub assoID: AssoID,                /* REPLICATED_COMPONENT */
    pub time: TimeStamp,               /* REPLICATED_COMPONENT */
    pub seq: SequenceNumber,           /* REPLICATED_COMPONENT */
    pub reqRekey: OPTIONAL<BOOLEAN>,   /* REPLICATED_COMPONENT */
    pub changedKey: OPTIONAL<BOOLEAN>, /* REPLICATED_COMPONENT */
    pub encInvoke: OPTIONAL<AlgoInvoke>,
    pub _unrecognized: Vec<X690Element>,
}
impl AadServerAE {
    pub fn new(
        invokeID: OPTIONAL<InvokeID>,  /* REPLICATED_COMPONENT */
        assoID: AssoID,                /* REPLICATED_COMPONENT */
        time: TimeStamp,               /* REPLICATED_COMPONENT */
        seq: SequenceNumber,           /* REPLICATED_COMPONENT */
        reqRekey: OPTIONAL<BOOLEAN>,   /* REPLICATED_COMPONENT */
        changedKey: OPTIONAL<BOOLEAN>, /* REPLICATED_COMPONENT */
        encInvoke: OPTIONAL<AlgoInvoke>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AadServerAE {
            invokeID,
            assoID,
            time,
            seq,
            reqRekey,
            changedKey,
            encInvoke,
            _unrecognized,
        }
    }
    pub fn _default_value_for_reqRekey() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_changedKey() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for AadServerAE {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AadServerAE(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AadServerAE {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AadServerAE(el)
    }
}

pub const _rctl1_components_for_AadServerAE: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "invokeID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reqRekey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "changedKey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encInvoke",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AadServerAE: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AadServerAE: &[ComponentSpec; 0] = &[];

pub fn _decode_AadServerAE(el: &X690Element) -> ASN1Result<AadServerAE> {
    |el_: &X690Element| -> ASN1Result<AadServerAE> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AadServerAE,
            _eal_components_for_AadServerAE,
            _rctl2_components_for_AadServerAE,
        )?;
        let invokeID: OPTIONAL<InvokeID> = match _components.get("invokeID") {
            Some(c_) => Some(_decode_InvokeID(c_)?),
            _ => None,
        };
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let reqRekey: OPTIONAL<BOOLEAN> = match _components.get("reqRekey") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let changedKey: OPTIONAL<BOOLEAN> = match _components.get("changedKey") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let encInvoke: OPTIONAL<AlgoInvoke> = match _components.get("encInvoke") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        Ok(AadServerAE {
            invokeID,
            assoID,
            time,
            seq,
            reqRekey,
            changedKey,
            encInvoke,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AadServerAE(value_: &AadServerAE) -> ASN1Result<X690Element> {
    |value_: &AadServerAE| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        if let Some(v_) = &value_.invokeID {
            components_.push(|v_1: &InvokeID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InvokeID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.reqRekey {
            if *v_ != AadServerAE::_default_value_for_reqRekey() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.changedKey {
            if *v_ != AadServerAE::_default_value_for_changedKey() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.encInvoke {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DataTransferServerNEA  ::=  ICV-Invoke{TbpDataTransferServer}
/// ```
pub type DataTransferServerNEA = ICV_Invoke<TbpDataTransferServer>; // DefinedType

pub fn _decode_DataTransferServerNEA(el: &X690Element) -> ASN1Result<DataTransferServerNEA> {
    _decode_ICV_Invoke::<TbpDataTransferServer>(_decode_TbpDataTransferServer, &el)
}

pub fn _encode_DataTransferServerNEA(value_: &DataTransferServerNEA) -> ASN1Result<X690Element> {
    _encode_ICV_Invoke::<TbpDataTransferServer>(_encode_TbpDataTransferServer, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbpDataTransferServer ::= SEQUENCE {
///   COMPONENTS OF     AadServer,
///   encInvoke     [3] AlgoInvoke{{SupportedSymmetricKeyAlgorithms}} OPTIONAL,
///   conf              CHOICE {
///     clear         [4] WrappedProt{{SupportedProtSet}},
///     protected     [5] ENCIPHERED{WRAPPED-PROT.&Type},
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbpDataTransferServer {
    pub invokeID: OPTIONAL<InvokeID>,  /* REPLICATED_COMPONENT */
    pub assoID: AssoID,                /* REPLICATED_COMPONENT */
    pub time: TimeStamp,               /* REPLICATED_COMPONENT */
    pub seq: SequenceNumber,           /* REPLICATED_COMPONENT */
    pub reqRekey: OPTIONAL<BOOLEAN>,   /* REPLICATED_COMPONENT */
    pub changedKey: OPTIONAL<BOOLEAN>, /* REPLICATED_COMPONENT */
    pub encInvoke: OPTIONAL<AlgoInvoke>,
    pub conf: TbpDataTransferServer_conf,
    pub _unrecognized: Vec<X690Element>,
}
impl TbpDataTransferServer {
    pub fn new(
        invokeID: OPTIONAL<InvokeID>,  /* REPLICATED_COMPONENT */
        assoID: AssoID,                /* REPLICATED_COMPONENT */
        time: TimeStamp,               /* REPLICATED_COMPONENT */
        seq: SequenceNumber,           /* REPLICATED_COMPONENT */
        reqRekey: OPTIONAL<BOOLEAN>,   /* REPLICATED_COMPONENT */
        changedKey: OPTIONAL<BOOLEAN>, /* REPLICATED_COMPONENT */
        encInvoke: OPTIONAL<AlgoInvoke>,
        conf: TbpDataTransferServer_conf,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TbpDataTransferServer {
            invokeID,
            assoID,
            time,
            seq,
            reqRekey,
            changedKey,
            encInvoke,
            conf,
            _unrecognized,
        }
    }
    pub fn _default_value_for_reqRekey() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_changedKey() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for TbpDataTransferServer {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferServer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbpDataTransferServer {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferServer(el)
    }
}

pub const _rctl1_components_for_TbpDataTransferServer: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "invokeID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reqRekey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "changedKey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encInvoke",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new("conf", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_TbpDataTransferServer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbpDataTransferServer: &[ComponentSpec; 0] = &[];

pub fn _decode_TbpDataTransferServer(el: &X690Element) -> ASN1Result<TbpDataTransferServer> {
    |el_: &X690Element| -> ASN1Result<TbpDataTransferServer> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbpDataTransferServer,
            _eal_components_for_TbpDataTransferServer,
            _rctl2_components_for_TbpDataTransferServer,
        )?;
        let invokeID: OPTIONAL<InvokeID> = match _components.get("invokeID") {
            Some(c_) => Some(_decode_InvokeID(c_)?),
            _ => None,
        };
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let reqRekey: OPTIONAL<BOOLEAN> = match _components.get("reqRekey") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let changedKey: OPTIONAL<BOOLEAN> = match _components.get("changedKey") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let encInvoke: OPTIONAL<AlgoInvoke> = match _components.get("encInvoke") {
            Some(c_) => Some(_decode_AlgoInvoke(c_)?),
            _ => None,
        };
        let conf = _decode_TbpDataTransferServer_conf(_components.get("conf").unwrap())?;
        Ok(TbpDataTransferServer {
            invokeID,
            assoID,
            time,
            seq,
            reqRekey,
            changedKey,
            encInvoke,
            conf,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TbpDataTransferServer(value_: &TbpDataTransferServer) -> ASN1Result<X690Element> {
    |value_: &TbpDataTransferServer| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        if let Some(v_) = &value_.invokeID {
            components_.push(|v_1: &InvokeID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InvokeID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.reqRekey {
            if *v_ != TbpDataTransferServer::_default_value_for_reqRekey() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.changedKey {
            if *v_ != TbpDataTransferServer::_default_value_for_changedKey() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.encInvoke {
            components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgoInvoke(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_TbpDataTransferServer_conf(&value_.conf)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AadServer ::= SEQUENCE {
///   invokeID   [0] InvokeID OPTIONAL,
///   assoID         AssoID,
///   time           TimeStamp,
///   seq            SequenceNumber,
///   reqRekey   [1] BOOLEAN DEFAULT FALSE,
///   changedKey [2] BOOLEAN DEFAULT FALSE }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AadServer {
    pub invokeID: OPTIONAL<InvokeID>,
    pub assoID: AssoID,
    pub time: TimeStamp,
    pub seq: SequenceNumber,
    pub reqRekey: OPTIONAL<BOOLEAN>,
    pub changedKey: OPTIONAL<BOOLEAN>,
}
impl AadServer {
    pub fn new(
        invokeID: OPTIONAL<InvokeID>,
        assoID: AssoID,
        time: TimeStamp,
        seq: SequenceNumber,
        reqRekey: OPTIONAL<BOOLEAN>,
        changedKey: OPTIONAL<BOOLEAN>,
    ) -> Self {
        AadServer {
            invokeID,
            assoID,
            time,
            seq,
            reqRekey,
            changedKey,
        }
    }
    pub fn _default_value_for_reqRekey() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_changedKey() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for AadServer {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AadServer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AadServer {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AadServer(el)
    }
}

pub const _rctl1_components_for_AadServer: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "invokeID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assoID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "seq",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reqRekey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "changedKey",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AadServer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AadServer: &[ComponentSpec; 0] = &[];

pub fn _decode_AadServer(el: &X690Element) -> ASN1Result<AadServer> {
    |el_: &X690Element| -> ASN1Result<AadServer> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AadServer,
            _eal_components_for_AadServer,
            _rctl2_components_for_AadServer,
        )?;
        let invokeID: OPTIONAL<InvokeID> = match _components.get("invokeID") {
            Some(c_) => Some(_decode_InvokeID(c_)?),
            _ => None,
        };
        let assoID = _decode_AssoID(_components.get("assoID").unwrap())?;
        let time = _decode_TimeStamp(_components.get("time").unwrap())?;
        let seq = _decode_SequenceNumber(_components.get("seq").unwrap())?;
        let reqRekey: OPTIONAL<BOOLEAN> = match _components.get("reqRekey") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let changedKey: OPTIONAL<BOOLEAN> = match _components.get("changedKey") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(AadServer {
            invokeID,
            assoID,
            time,
            seq,
            reqRekey,
            changedKey,
        })
    }(&el)
}

pub fn _encode_AadServer(value_: &AadServer) -> ASN1Result<X690Element> {
    |value_: &AadServer| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        if let Some(v_) = &value_.invokeID {
            components_.push(|v_1: &InvokeID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InvokeID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_AssoID(&value_.assoID)?);
        components_.push(_encode_TimeStamp(&value_.time)?);
        components_.push(_encode_SequenceNumber(&value_.seq)?);
        if let Some(v_) = &value_.reqRekey {
            if *v_ != AadServer::_default_value_for_reqRekey() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.changedKey {
            if *v_ != AadServer::_default_value_for_changedKey() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
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
/// WrpError  ::=  ENUMERATED {
///   protocol-error                         (0),
///   invalid-signatureAlgorithm             (1),
///   unexpected-version                     (2),
///   protected-protocol-not-supported       (3),
///   duplicate-assoID                       (4),
///   invalid-time-value                     (5),
///   key-estab-algorithm-not-supported      (6),
///   encr-mode-aead-not-supported           (7),
///   encryption-not-supported               (8),
///   encryption-required                    (9),
///   aead-algorithms-not-supported          (10),
///   aead-is-required                       (11),
///   symmetricKey-algorithms-not-supported  (12),
///   icv-algorithms-not-supported           (13),
///   invalid-attribute-certificate          (14),
///   alt-signature-not-allowed              (15),
///   only-one-version                       (16),
///   invalid-key-estab-algorithm            (17),
///   invalid-alt-key-estab-algorithm        (18),
///   invalid-aead-algorithm                 (19),
///   aead-not-allowed                       (20),
///   invalid-symmetricKey-algorithm         (21),
///   invalid-icv-algorithm                  (22),
///   dynamic-aead-algo-parms-required       (23),
///   invalid-dynamic-aead-algo-parms        (24),
///   dynamic-aead-algo-parms-not-required   (25),
///   dynamic-symKey-algo-parms-required     (26),
///   invalid-dynamic-symKey-algo-parms      (27),
///   dynamic-symKey-algo-parms-not-required (28),
///   dynamic-icv-algo-parms-required        (29),
///   invalid-dynamic-icv-algo-parms         (30),
///   dynamic-icv-algo-parms-not-required    (31),
///   unexpected-invokeID-received           (32),
///   rekey-out-of-sequence                  (33),
///   invalid-dynamic-keyEst-algo-parms      (34),
///   changedKey-out-of-sequence             (35),
///   ... }
/// ```
pub type WrpError = ENUMERATED;

pub const WrpError_protocol_error: WrpError = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_signatureAlgorithm: WrpError = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_unexpected_version: WrpError = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_protected_protocol_not_supported: WrpError = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_duplicate_assoID: WrpError = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_time_value: WrpError = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_key_estab_algorithm_not_supported: WrpError = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_encr_mode_aead_not_supported: WrpError = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_encryption_not_supported: WrpError = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_encryption_required: WrpError = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_aead_algorithms_not_supported: WrpError = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_aead_is_required: WrpError = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_symmetricKey_algorithms_not_supported: WrpError = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_icv_algorithms_not_supported: WrpError = 13; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_attribute_certificate: WrpError = 14; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_alt_signature_not_allowed: WrpError = 15; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_only_one_version: WrpError = 16; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_key_estab_algorithm: WrpError = 17; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_alt_key_estab_algorithm: WrpError = 18; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_aead_algorithm: WrpError = 19; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_aead_not_allowed: WrpError = 20; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_symmetricKey_algorithm: WrpError = 21; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_icv_algorithm: WrpError = 22; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_dynamic_aead_algo_parms_required: WrpError = 23; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_dynamic_aead_algo_parms: WrpError = 24; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_dynamic_aead_algo_parms_not_required: WrpError = 25; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_dynamic_symKey_algo_parms_required: WrpError = 26; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_dynamic_symKey_algo_parms: WrpError = 27; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_dynamic_symKey_algo_parms_not_required: WrpError = 28; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_dynamic_icv_algo_parms_required: WrpError = 29; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_dynamic_icv_algo_parms: WrpError = 30; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_dynamic_icv_algo_parms_not_required: WrpError = 31; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_unexpected_invokeID_received: WrpError = 32; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_rekey_out_of_sequence: WrpError = 33; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_invalid_dynamic_keyEst_algo_parms: WrpError = 34; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WrpError_changedKey_out_of_sequence: WrpError = 35; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_WrpError(el: &X690Element) -> ASN1Result<WrpError> {
    ber_decode_enumerated(&el)
}

pub fn _encode_WrpError(value_: &WrpError) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeReq-encr-mode-non-aead ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeReq_encr_mode_non_aead {
    pub encr: OPTIONAL<Vec<AlgorithmIdentifier>>,
    pub icvAlgID: Vec<AlgorithmIdentifier>,
}
impl TbsHandshakeReq_encr_mode_non_aead {
    pub fn new(encr: OPTIONAL<Vec<AlgorithmIdentifier>>, icvAlgID: Vec<AlgorithmIdentifier>) -> Self {
        TbsHandshakeReq_encr_mode_non_aead { encr, icvAlgID }
    }
}
impl TryFrom<X690Element> for TbsHandshakeReq_encr_mode_non_aead {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeReq_encr_mode_non_aead(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeReq_encr_mode_non_aead {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeReq_encr_mode_non_aead(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeReq_encr_mode_non_aead: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "encr",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "icvAlgID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeReq_encr_mode_non_aead: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeReq_encr_mode_non_aead: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeReq_encr_mode_non_aead(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeReq_encr_mode_non_aead> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeReq_encr_mode_non_aead> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeReq_encr_mode_non_aead,
            _eal_components_for_TbsHandshakeReq_encr_mode_non_aead,
            _rctl2_components_for_TbsHandshakeReq_encr_mode_non_aead,
        )?;
        let encr: OPTIONAL<Vec<AlgorithmIdentifier>> = match _components.get("encr") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
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
                }(c_)?,
            ),
            _ => None,
        };
        let icvAlgID = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_AlgorithmIdentifier(el)?);
            }
            Ok(items)
        }(_components.get("icvAlgID").unwrap())?;
        Ok(TbsHandshakeReq_encr_mode_non_aead { encr, icvAlgID })
    }(&el)
}

pub fn _encode_TbsHandshakeReq_encr_mode_non_aead(
    value_: &TbsHandshakeReq_encr_mode_non_aead,
) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeReq_encr_mode_non_aead| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.encr {
            components_.push(
                |v_1: &Vec<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
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
                }(&v_)?,
            );
        }
        components_.push(
            |v_1: &Vec<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
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
                el_1.tag_number = 1;
                Ok(el_1)
            }(&value_.icvAlgID)?,
        );
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
/// TbsHandshakeReq-encr-mode ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbsHandshakeReq_encr_mode {
    aead(Vec<AlgorithmIdentifier>),
    non_aead(TbsHandshakeReq_encr_mode_non_aead),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TbsHandshakeReq_encr_mode {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeReq_encr_mode(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeReq_encr_mode {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeReq_encr_mode(el)
    }
}

pub fn _decode_TbsHandshakeReq_encr_mode(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeReq_encr_mode> {
    |el: &X690Element| -> ASN1Result<TbsHandshakeReq_encr_mode> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 2) => Ok(TbsHandshakeReq_encr_mode::aead(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AlgorithmIdentifier>> {
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
                }(&el)?,
            )),
            (TagClass::CONTEXT, 3) => Ok(TbsHandshakeReq_encr_mode::non_aead(
                _decode_TbsHandshakeReq_encr_mode_non_aead(&el)?,
            )),
            _ => Ok(TbsHandshakeReq_encr_mode::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TbsHandshakeReq_encr_mode(
    value_: &TbsHandshakeReq_encr_mode,
) -> ASN1Result<X690Element> {
    |value: &TbsHandshakeReq_encr_mode| -> ASN1Result<X690Element> {
        match value {
            TbsHandshakeReq_encr_mode::aead(v) => {
                |v_1: &Vec<AlgorithmIdentifier>| -> ASN1Result<X690Element> {
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
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v)
            }
            TbsHandshakeReq_encr_mode::non_aead(v) => {
                |v_1: &TbsHandshakeReq_encr_mode_non_aead| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_TbsHandshakeReq_encr_mode_non_aead(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 3;
                    Ok(el_1)
                }(&v)
            }
            TbsHandshakeReq_encr_mode::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeAcc-sigSel ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbsHandshakeAcc_sigSel {
    sigAlg(AlgorithmIdentifier),
    altSigAlg(AlgorithmIdentifier),
}

impl TryFrom<X690Element> for TbsHandshakeAcc_sigSel {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_sigSel(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeAcc_sigSel {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_sigSel(el)
    }
}

pub fn _decode_TbsHandshakeAcc_sigSel(el: &X690Element) -> ASN1Result<TbsHandshakeAcc_sigSel> {
    |el: &X690Element| -> ASN1Result<TbsHandshakeAcc_sigSel> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(TbsHandshakeAcc_sigSel::sigAlg(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(TbsHandshakeAcc_sigSel::altSigAlg(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_TbsHandshakeAcc_sigSel(value_: &TbsHandshakeAcc_sigSel) -> ASN1Result<X690Element> {
    |value: &TbsHandshakeAcc_sigSel| -> ASN1Result<X690Element> {
        match value {
            TbsHandshakeAcc_sigSel::sigAlg(v) => _encode_AlgorithmIdentifier(&v),
            TbsHandshakeAcc_sigSel::altSigAlg(v) => {
                |v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
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
/// TbsHandshakeAcc-keyEstSel ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbsHandshakeAcc_keyEstSel {
    keyEst(AlgorithmWithInvoke),
    altKeyEst(AlgorithmWithInvoke),
}

impl TryFrom<X690Element> for TbsHandshakeAcc_keyEstSel {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_keyEstSel(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeAcc_keyEstSel {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_keyEstSel(el)
    }
}

pub fn _decode_TbsHandshakeAcc_keyEstSel(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeAcc_keyEstSel> {
    |el: &X690Element| -> ASN1Result<TbsHandshakeAcc_keyEstSel> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(TbsHandshakeAcc_keyEstSel::keyEst(
                _decode_AlgorithmWithInvoke(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(TbsHandshakeAcc_keyEstSel::altKeyEst(
                _decode_AlgorithmWithInvoke(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_TbsHandshakeAcc_keyEstSel(
    value_: &TbsHandshakeAcc_keyEstSel,
) -> ASN1Result<X690Element> {
    |value: &TbsHandshakeAcc_keyEstSel| -> ASN1Result<X690Element> {
        match value {
            TbsHandshakeAcc_keyEstSel::keyEst(v) => _encode_AlgorithmWithInvoke(&v),
            TbsHandshakeAcc_keyEstSel::altKeyEst(v) => {
                |v_1: &AlgorithmWithInvoke| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AlgorithmWithInvoke(&v_1)?;
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
/// TbsHandshakeAcc-encr-mode-non-aead ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TbsHandshakeAcc_encr_mode_non_aead {
    pub encr: OPTIONAL<AlgorithmIdentifier>,
    pub icvAlgID: AlgorithmIdentifier,
}
impl TbsHandshakeAcc_encr_mode_non_aead {
    pub fn new(encr: OPTIONAL<AlgorithmIdentifier>, icvAlgID: AlgorithmIdentifier) -> Self {
        TbsHandshakeAcc_encr_mode_non_aead { encr, icvAlgID }
    }
}
impl TryFrom<X690Element> for TbsHandshakeAcc_encr_mode_non_aead {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_encr_mode_non_aead(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeAcc_encr_mode_non_aead {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_encr_mode_non_aead(el)
    }
}

pub const _rctl1_components_for_TbsHandshakeAcc_encr_mode_non_aead: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "encr",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "icvAlgID",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TbsHandshakeAcc_encr_mode_non_aead: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TbsHandshakeAcc_encr_mode_non_aead: &[ComponentSpec; 0] = &[];

pub fn _decode_TbsHandshakeAcc_encr_mode_non_aead(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeAcc_encr_mode_non_aead> {
    |el_: &X690Element| -> ASN1Result<TbsHandshakeAcc_encr_mode_non_aead> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TbsHandshakeAcc_encr_mode_non_aead,
            _eal_components_for_TbsHandshakeAcc_encr_mode_non_aead,
            _rctl2_components_for_TbsHandshakeAcc_encr_mode_non_aead,
        )?;
        let encr: OPTIONAL<AlgorithmIdentifier> = match _components.get("encr") {
            Some(c_) => Some(_decode_AlgorithmIdentifier(c_)?),
            _ => None,
        };
        let icvAlgID = _decode_AlgorithmIdentifier(_components.get("icvAlgID").unwrap())?;
        Ok(TbsHandshakeAcc_encr_mode_non_aead { encr, icvAlgID })
    }(&el)
}

pub fn _encode_TbsHandshakeAcc_encr_mode_non_aead(
    value_: &TbsHandshakeAcc_encr_mode_non_aead,
) -> ASN1Result<X690Element> {
    |value_: &TbsHandshakeAcc_encr_mode_non_aead| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.encr {
            components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 1;
            Ok(el_1)
        }(&value_.icvAlgID)?);
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
/// TbsHandshakeAcc-encr-mode ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbsHandshakeAcc_encr_mode {
    aead(AlgorithmIdentifier),
    non_aead(TbsHandshakeAcc_encr_mode_non_aead),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TbsHandshakeAcc_encr_mode {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_encr_mode(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeAcc_encr_mode {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeAcc_encr_mode(el)
    }
}

pub fn _decode_TbsHandshakeAcc_encr_mode(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeAcc_encr_mode> {
    |el: &X690Element| -> ASN1Result<TbsHandshakeAcc_encr_mode> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 2) => Ok(TbsHandshakeAcc_encr_mode::aead(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            (TagClass::CONTEXT, 3) => Ok(TbsHandshakeAcc_encr_mode::non_aead(
                _decode_TbsHandshakeAcc_encr_mode_non_aead(&el)?,
            )),
            _ => Ok(TbsHandshakeAcc_encr_mode::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TbsHandshakeAcc_encr_mode(
    value_: &TbsHandshakeAcc_encr_mode,
) -> ASN1Result<X690Element> {
    |value: &TbsHandshakeAcc_encr_mode| -> ASN1Result<X690Element> {
        match value {
            TbsHandshakeAcc_encr_mode::aead(v) => {
                |v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v)
            }
            TbsHandshakeAcc_encr_mode::non_aead(v) => {
                |v_1: &TbsHandshakeAcc_encr_mode_non_aead| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_TbsHandshakeAcc_encr_mode_non_aead(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 3;
                    Ok(el_1)
                }(&v)
            }
            TbsHandshakeAcc_encr_mode::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbsHandshakeWrpRej-sigSel ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbsHandshakeWrpRej_sigSel {
    sigAlg(AlgorithmIdentifier),
    altSigAlg(AlgorithmIdentifier),
}

impl TryFrom<X690Element> for TbsHandshakeWrpRej_sigSel {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeWrpRej_sigSel(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeWrpRej_sigSel {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeWrpRej_sigSel(el)
    }
}

pub fn _decode_TbsHandshakeWrpRej_sigSel(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeWrpRej_sigSel> {
    |el: &X690Element| -> ASN1Result<TbsHandshakeWrpRej_sigSel> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(TbsHandshakeWrpRej_sigSel::sigAlg(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(TbsHandshakeWrpRej_sigSel::altSigAlg(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_TbsHandshakeWrpRej_sigSel(
    value_: &TbsHandshakeWrpRej_sigSel,
) -> ASN1Result<X690Element> {
    |value: &TbsHandshakeWrpRej_sigSel| -> ASN1Result<X690Element> {
        match value {
            TbsHandshakeWrpRej_sigSel::sigAlg(v) => _encode_AlgorithmIdentifier(&v),
            TbsHandshakeWrpRej_sigSel::altSigAlg(v) => {
                |v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
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
/// TbsHandshakeProRej-sigSel ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbsHandshakeProRej_sigSel {
    sigAlg(AlgorithmIdentifier),
    altSigAlg(AlgorithmIdentifier),
}

impl TryFrom<X690Element> for TbsHandshakeProRej_sigSel {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeProRej_sigSel(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbsHandshakeProRej_sigSel {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbsHandshakeProRej_sigSel(el)
    }
}

pub fn _decode_TbsHandshakeProRej_sigSel(
    el: &X690Element,
) -> ASN1Result<TbsHandshakeProRej_sigSel> {
    |el: &X690Element| -> ASN1Result<TbsHandshakeProRej_sigSel> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(TbsHandshakeProRej_sigSel::sigAlg(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(TbsHandshakeProRej_sigSel::altSigAlg(
                _decode_AlgorithmIdentifier(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_TbsHandshakeProRej_sigSel(
    value_: &TbsHandshakeProRej_sigSel,
) -> ASN1Result<X690Element> {
    |value: &TbsHandshakeProRej_sigSel| -> ASN1Result<X690Element> {
        match value {
            TbsHandshakeProRej_sigSel::sigAlg(v) => _encode_AlgorithmIdentifier(&v),
            TbsHandshakeProRej_sigSel::altSigAlg(v) => {
                |v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AlgorithmIdentifier(&v_1)?;
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
/// TbpDataTransferClient-conf ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbpDataTransferClient_conf {
    clear(WrappedProt),
    protected(ENCIPHERED),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TbpDataTransferClient_conf {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferClient_conf(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbpDataTransferClient_conf {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferClient_conf(el)
    }
}

pub fn _decode_TbpDataTransferClient_conf(
    el: &X690Element,
) -> ASN1Result<TbpDataTransferClient_conf> {
    |el: &X690Element| -> ASN1Result<TbpDataTransferClient_conf> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 4) => {
                Ok(TbpDataTransferClient_conf::clear(_decode_WrappedProt(&el)?))
            }
            (TagClass::CONTEXT, 5) => Ok(TbpDataTransferClient_conf::protected(
                _decode_ENCIPHERED(&el)?,
            )),
            _ => Ok(TbpDataTransferClient_conf::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TbpDataTransferClient_conf(
    value_: &TbpDataTransferClient_conf,
) -> ASN1Result<X690Element> {
    |value: &TbpDataTransferClient_conf| -> ASN1Result<X690Element> {
        match value {
            TbpDataTransferClient_conf::clear(v) => {
                |v_1: &WrappedProt| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_WrappedProt(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 4;
                    Ok(el_1)
                }(&v)
            }
            TbpDataTransferClient_conf::protected(v) => {
                |v_1: &ENCIPHERED| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ENCIPHERED(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 5;
                    Ok(el_1)
                }(&v)
            }
            TbpDataTransferClient_conf::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TbpDataTransferServer-conf ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TbpDataTransferServer_conf {
    clear(WrappedProt),
    protected(ENCIPHERED),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for TbpDataTransferServer_conf {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferServer_conf(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TbpDataTransferServer_conf {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TbpDataTransferServer_conf(el)
    }
}

pub fn _decode_TbpDataTransferServer_conf(
    el: &X690Element,
) -> ASN1Result<TbpDataTransferServer_conf> {
    |el: &X690Element| -> ASN1Result<TbpDataTransferServer_conf> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 4) => {
                Ok(TbpDataTransferServer_conf::clear(_decode_WrappedProt(&el)?))
            }
            (TagClass::CONTEXT, 5) => Ok(TbpDataTransferServer_conf::protected(
                _decode_ENCIPHERED(&el)?,
            )),
            _ => Ok(TbpDataTransferServer_conf::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TbpDataTransferServer_conf(
    value_: &TbpDataTransferServer_conf,
) -> ASN1Result<X690Element> {
    |value: &TbpDataTransferServer_conf| -> ASN1Result<X690Element> {
        match value {
            TbpDataTransferServer_conf::clear(v) => {
                |v_1: &WrappedProt| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_WrappedProt(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 4;
                    Ok(el_1)
                }(&v)
            }
            TbpDataTransferServer_conf::protected(v) => {
                |v_1: &ENCIPHERED| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_ENCIPHERED(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 5;
                    Ok(el_1)
                }(&v)
            }
            TbpDataTransferServer_conf::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}
