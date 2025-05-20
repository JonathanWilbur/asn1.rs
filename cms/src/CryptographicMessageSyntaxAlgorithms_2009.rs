#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CryptographicMessageSyntaxAlgorithms-2009
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CryptographicMessageSyntaxAlgorithms-2009`.
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
use crate::PKIXAlgs_2009::*;
use crate::SecureMimeMessageV3dot1_2009::cap_RC2CBC;
use asn1::*;
use std::sync::Arc;
use x500::AuthenticationFramework::{
    AlgorithmIdentifier, _decode_AlgorithmIdentifier, _encode_AlgorithmIdentifier,
    _validate_AlgorithmIdentifier,
};
use x500::CertificateExtensions::{
    KeyUsage_decipherOnly, KeyUsage_encipherOnly, KeyUsage_keyAgreement,
};
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageDigestAlgs DIGEST-ALGORITHM ::= {
/// --   mda-md5 | mda-sha1,
/// ... }
/// ```
///
///
pub fn MessageDigestAlgs() -> Vec<DIGEST_ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureAlgs SIGNATURE-ALGORITHM ::= {
/// --  See RFC 3279
/// --  sa-dsaWithSHA1 |  sa-rsaWithMD5 | sa-rsaWithSHA1,
/// ... }
/// ```
///
///
pub fn SignatureAlgs() -> Vec<SIGNATURE_ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreementAlgs KEY-AGREE ::= { kaa-esdh | kaa-ssdh, ...}
/// ```
///
///
pub fn KeyAgreementAlgs() -> Vec<KEY_AGREE> {
    Vec::from([kaa_esdh(), kaa_ssdh()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyAgreePublicKeys PUBLIC-KEY ::= { pk-dh, ...}
/// ```
///
///
pub fn KeyAgreePublicKeys() -> Vec<PUBLIC_KEY> {
    Vec::from([pk_dh()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyTransportAlgs KEY-TRANSPORT ::= { kt-rsa, ... }
/// ```
///
///
pub fn KeyTransportAlgs() -> Vec<KEY_TRANSPORT> {
    Vec::from([kt_rsa()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyWrapAlgs KEY-WRAP ::= { kwa-3DESWrap | kwa-RC2Wrap, ... }
/// ```
///
///
pub fn KeyWrapAlgs() -> Vec<KEY_WRAP> {
    Vec::from([kwa_3DESWrap(), kwa_RC2Wrap()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyDerivationAlgs KEY-DERIVATION ::= { kda-PBKDF2, ... }
/// ```
///
///
pub fn KeyDerivationAlgs() -> Vec<KEY_DERIVATION> {
    Vec::from([kda_PBKDF2()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContentEncryptionAlgs CONTENT-ENCRYPTION ::= { cea-3DES-cbc | cea-RC2-cbc, ... }
/// ```
///
///
pub fn ContentEncryptionAlgs() -> Vec<CONTENT_ENCRYPTION> {
    Vec::from([cea_3DES_cbc(), cea_RC2_cbc()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageAuthAlgs MAC-ALGORITHM ::= { maca-hMAC-SHA1, ... }
/// ```
///
///
pub fn MessageAuthAlgs() -> Vec<MAC_ALGORITHM> {
    Vec::from([maca_hMAC_SHA1()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMimeCaps SMIME-CAPS ::= {
/// kaa-esdh.&smimeCaps	|
/// kaa-ssdh.&smimeCaps	|
/// kt-rsa.&smimeCaps		|
/// kwa-3DESWrap.&smimeCaps	|
/// kwa-RC2Wrap.&smimeCaps	|
/// cea-3DES-cbc.&smimeCaps	|
/// cea-RC2-cbc.&smimeCaps	|
/// maca-hMAC-SHA1.&smimeCaps,
/// ...}
/// ```
///
///
pub fn SMimeCaps() -> Vec<SMIME_CAPS> {
    Vec::from([
        kaa_esdh().smimeCaps.unwrap(),
        kaa_ssdh().smimeCaps.unwrap(),
        kt_rsa().smimeCaps.unwrap(),
        kwa_3DESWrap().smimeCaps.unwrap(),
        kwa_RC2Wrap().smimeCaps.unwrap(),
        cea_3DES_cbc().smimeCaps.unwrap(),
        cea_RC2_cbc().smimeCaps.unwrap(),
        maca_hMAC_SHA1().smimeCaps.unwrap(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-ESDH OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840)
/// rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) alg(3) 5 }
/// ```
///
///
pub fn id_alg_ESDH() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-SSDH OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840)
/// rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) alg(3) 10 }
/// ```
///
///
pub fn id_alg_SSDH() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 10) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-CMS3DESwrap OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) alg(3) 6 }
/// ```
///
///
pub fn id_alg_CMS3DESwrap() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 6) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-CMSRC2wrap OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) alg(3) 7 }
/// ```
///
///
pub fn id_alg_CMSRC2wrap() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* alg */ 3, 7) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// des-ede3-cbc OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) encryptionAlgorithm(3) 7 }
/// ```
///
///
pub fn des_ede3_cbc() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* encryptionAlgorithm */ 3, 7) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rc2-cbc OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840)
/// rsadsi(113549) encryptionAlgorithm(3) 2 }
/// ```
///
///
pub fn rc2_cbc() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* encryptionAlgorithm */ 3, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hMAC-SHA1 OBJECT IDENTIFIER ::= { iso(1) identified-organization(3)
/// dod(6) internet(1) security(5) mechanisms(5) 8 1 2 }
/// ```
///
///
pub fn hMAC_SHA1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* security */ 5, /* mechanisms */ 5, 8, 1, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-PBKDF2 OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840)
/// rsadsi(113549) pkcs(1) pkcs-5(5) 12 }
/// ```
///
///
pub fn id_PBKDF2() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-5 */ 5, 12) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyWrapAlgorithm  ::=  AlgorithmIdentifier {KEY-WRAP, {KeyWrapAlgs }}
/// ```
pub type KeyWrapAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_KeyWrapAlgorithm(el: &X690Element) -> ASN1Result<KeyWrapAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_KeyWrapAlgorithm(value_: &KeyWrapAlgorithm) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_KeyWrapAlgorithm(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
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

pub fn _validate_RC2wrapParameter(el: &X690Element) -> ASN1Result<()> {
    _validate_RC2ParameterVersion(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RC2ParameterVersion  ::=  INTEGER
/// ```
pub type RC2ParameterVersion = INTEGER;

pub fn _decode_RC2ParameterVersion(el: &X690Element) -> ASN1Result<RC2ParameterVersion> {
    BER.decode_integer(&el)
}

pub fn _encode_RC2ParameterVersion(value_: &RC2ParameterVersion) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_RC2ParameterVersion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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

pub fn _validate_CBCParameter(el: &X690Element) -> ASN1Result<()> {
    _validate_IV(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IV  ::=  OCTET STRING
/// ```
pub type IV = OCTET_STRING; // OctetStringType

pub fn _decode_IV(el: &X690Element) -> ASN1Result<IV> {
    BER.decode_octet_string(&el)
}

pub fn _encode_IV(value_: &IV) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_IV(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RC2CBCParameter ::= SEQUENCE {
/// rc2ParameterVersion INTEGER (1..256),
/// iv OCTET STRING  }
/// ```
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
impl TryFrom<&X690Element> for RC2CBCParameter {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RC2CBCParameter"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RC2CBCParameter,
        _eal_components_for_RC2CBCParameter,
        _rctl2_components_for_RC2CBCParameter,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut rc2ParameterVersion_: OPTIONAL<INTEGER> = None;
    let mut iv_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "rc2ParameterVersion" => rc2ParameterVersion_ = Some(BER.decode_integer(_el)?),
            "iv" => iv_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RC2CBCParameter")
                )
            }
        }
    }
    Ok(RC2CBCParameter {
        rc2ParameterVersion: rc2ParameterVersion_.unwrap(),
        iv: iv_.unwrap(),
    })
}

pub fn _encode_RC2CBCParameter(value_: &RC2CBCParameter) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.rc2ParameterVersion)?);
    components_.push(BER.encode_octet_string(&value_.iv)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_RC2CBCParameter(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RC2CBCParameter"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RC2CBCParameter,
        _eal_components_for_RC2CBCParameter,
        _rctl2_components_for_RC2CBCParameter,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "rc2ParameterVersion" => BER.validate_integer(_el)?,
            "iv" => BER.validate_octet_string(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RC2CBCParameter")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// maca-hMAC-SHA1 MAC-ALGORITHM ::= {
/// IDENTIFIER hMAC-SHA1
/// PARAMS TYPE NULL ARE preferredAbsent
/// IS-KEYED-MAC TRUE
/// SMIME-CAPS {IDENTIFIED BY hMAC-SHA1}
/// }
/// ```
///
///
pub fn maca_hMAC_SHA1() -> MAC_ALGORITHM {
    MAC_ALGORITHM {
        id: hMAC_SHA1(),                                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
        keyed: true,                                       /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(maca_hMAC_SHA1_smimeCaps()),       /* OBJECT_FIELD_SETTING */
    }
}

pub mod maca_hMAC_SHA1 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        BER.decode_null(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PBKDF2-PRFsAlgorithmIdentifier  ::=  AlgorithmIdentifier{ ALGORITHM,{PBKDF2-PRFs} }
/// ```
pub type PBKDF2_PRFsAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_PBKDF2_PRFsAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<PBKDF2_PRFsAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_PBKDF2_PRFsAlgorithmIdentifier(
    value_: &PBKDF2_PRFsAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_PBKDF2_PRFsAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// alg-hMAC-SHA1 ALGORITHM ::= { IDENTIFIER hMAC-SHA1 PARAMS TYPE NULL ARE required }
/// ```
///
///
pub fn alg_hMAC_SHA1() -> ALGORITHM {
    ALGORITHM {
        id: hMAC_SHA1(),                            /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: None,
    }
}

pub mod alg_hMAC_SHA1 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        BER.decode_null(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PBKDF2-PRFs ALGORITHM ::= { alg-hMAC-SHA1, ... }
/// ```
///
///
pub fn PBKDF2_PRFs() -> Vec<ALGORITHM> {
    Vec::from([alg_hMAC_SHA1()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PBKDF2-SaltSources ALGORITHM ::= { ... }
/// ```
///
///
pub fn PBKDF2_SaltSources() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PBKDF2-SaltSourcesAlgorithmIdentifier  ::=
/// AlgorithmIdentifier {ALGORITHM, {PBKDF2-SaltSources}}
/// ```
pub type PBKDF2_SaltSourcesAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_PBKDF2_SaltSourcesAlgorithmIdentifier(
    el: &X690Element,
) -> ASN1Result<PBKDF2_SaltSourcesAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_PBKDF2_SaltSourcesAlgorithmIdentifier(
    value_: &PBKDF2_SaltSourcesAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_PBKDF2_SaltSourcesAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// defaultPBKDF2 PBKDF2-PRFsAlgorithmIdentifier ::= { algorithm alg-hMAC-SHA1.&id, parameters NULL:NULL }
/// ```
///
pub fn defaultPBKDF2() -> PBKDF2_PRFsAlgorithmIdentifier {
    AlgorithmIdentifier {
        algorithm: alg_hMAC_SHA1().id,
        parameters: Some(BER.encode_null(&()).unwrap()),
        _unrecognized: vec![],
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PBKDF2-params ::= SEQUENCE {
/// salt CHOICE {
/// 	specified 	OCTET STRING,
/// 	otherSource 	PBKDF2-SaltSourcesAlgorithmIdentifier },
/// iterationCount 	INTEGER (1..MAX),
/// keyLength 		INTEGER (1..MAX) OPTIONAL,
/// prf 			PBKDF2-PRFsAlgorithmIdentifier DEFAULT defaultPBKDF2 }
/// ```
///
#[derive(Debug, Clone)]
pub struct PBKDF2_params {
    pub salt: PBKDF2_params_salt,
    pub iterationCount: INTEGER,
    pub keyLength: OPTIONAL<INTEGER>,
    pub prf: OPTIONAL<PBKDF2_PRFsAlgorithmIdentifier>,
}
impl PBKDF2_params {
    pub fn new(
        salt: PBKDF2_params_salt,
        iterationCount: INTEGER,
        keyLength: OPTIONAL<INTEGER>,
        prf: OPTIONAL<PBKDF2_PRFsAlgorithmIdentifier>,
    ) -> Self {
        PBKDF2_params {
            salt,
            iterationCount,
            keyLength,
            prf,
        }
    }
    pub fn _default_value_for_prf() -> PBKDF2_PRFsAlgorithmIdentifier {
        defaultPBKDF2()
    }
}
impl TryFrom<&X690Element> for PBKDF2_params {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PBKDF2_params(el)
    }
}

pub const _rctl1_components_for_PBKDF2_params: &[ComponentSpec; 4] = &[
    ComponentSpec::new("salt", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "iterationCount",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyLength",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "prf",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PBKDF2_params: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PBKDF2_params: &[ComponentSpec; 0] = &[];

pub fn _decode_PBKDF2_params(el: &X690Element) -> ASN1Result<PBKDF2_params> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PBKDF2-params")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PBKDF2_params,
        _eal_components_for_PBKDF2_params,
        _rctl2_components_for_PBKDF2_params,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut salt_: OPTIONAL<PBKDF2_params_salt> = None;
    let mut iterationCount_: OPTIONAL<INTEGER> = None;
    let mut keyLength_: OPTIONAL<INTEGER> = None;
    let mut prf_: OPTIONAL<PBKDF2_PRFsAlgorithmIdentifier> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "salt" => salt_ = Some(_decode_PBKDF2_params_salt(_el)?),
            "iterationCount" => iterationCount_ = Some(BER.decode_integer(_el)?),
            "keyLength" => keyLength_ = Some(BER.decode_integer(_el)?),
            "prf" => prf_ = Some(_decode_PBKDF2_PRFsAlgorithmIdentifier(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PBKDF2-params")
                )
            }
        }
    }
    Ok(PBKDF2_params {
        salt: salt_.unwrap(),
        iterationCount: iterationCount_.unwrap(),
        keyLength: keyLength_,
        prf: prf_,
    })
}

pub fn _encode_PBKDF2_params(value_: &PBKDF2_params) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_PBKDF2_params_salt(&value_.salt)?);
    components_.push(BER.encode_integer(&value_.iterationCount)?);
    if let Some(v_) = &value_.keyLength {
        components_.push(BER.encode_integer(&v_)?);
    }
    if let Some(v_) = &value_.prf {
        if v_.algorithm != PBKDF2_params::_default_value_for_prf().algorithm {
            components_.push(_encode_PBKDF2_PRFsAlgorithmIdentifier(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_PBKDF2_params(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PBKDF2-params")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PBKDF2_params,
        _eal_components_for_PBKDF2_params,
        _rctl2_components_for_PBKDF2_params,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "salt" => _validate_PBKDF2_params_salt(_el)?,
            "iterationCount" => BER.validate_integer(_el)?,
            "keyLength" => BER.validate_integer(_el)?,
            "prf" => _validate_PBKDF2_PRFsAlgorithmIdentifier(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PBKDF2-params")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kaa-esdh KEY-AGREE ::= {
/// IDENTIFIER	id-alg-ESDH
/// PARAMS TYPE	KeyWrapAlgorithm ARE required
/// PUBLIC-KEYS	{ pk-dh }
/// -- UKM is not ASN.1 encoded
/// UKM ARE optional
/// SMIME-CAPS {TYPE KeyWrapAlgorithm IDENTIFIED BY id-alg-ESDH}
/// }
/// ```
///
///
pub fn kaa_esdh() -> KEY_AGREE {
    KEY_AGREE {
        id: id_alg_ESDH(),                          /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_dh()])),   /* OBJECT_FIELD_SETTING */
        ukmPresence: Some(ParamOptions_optional),   /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(kaa_esdh_smimeCaps()),      /* OBJECT_FIELD_SETTING */
    }
}

pub mod kaa_esdh {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = KeyWrapAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_KeyWrapAlgorithm(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_KeyWrapAlgorithm(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_KeyWrapAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kaa-ssdh KEY-AGREE ::= {
/// IDENTIFIER id-alg-SSDH
/// PARAMS TYPE KeyWrapAlgorithm ARE required
/// PUBLIC-KEYS {pk-dh}
/// -- UKM is not ASN.1 encoded
/// UKM ARE optional
/// SMIME-CAPS {TYPE KeyWrapAlgorithm IDENTIFIED BY id-alg-SSDH}
/// }
/// ```
///
///
pub fn kaa_ssdh() -> KEY_AGREE {
    KEY_AGREE {
        id: id_alg_SSDH(),                          /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_dh()])),   /* OBJECT_FIELD_SETTING */
        ukmPresence: Some(ParamOptions_optional),   /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(kaa_ssdh_smimeCaps()),      /* OBJECT_FIELD_SETTING */
    }
}

pub mod kaa_ssdh {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = KeyWrapAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_KeyWrapAlgorithm(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_KeyWrapAlgorithm(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_KeyWrapAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dh-public-number OBJECT IDENTIFIER ::= dhpublicnumber
/// ```
///
///
pub fn dh_public_number() -> OBJECT_IDENTIFIER {
    dhpublicnumber() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-originator-dh PUBLIC-KEY ::= {
/// IDENTIFIER dh-public-number
/// KEY DHPublicKey
/// PARAMS ARE absent
/// CERT-KEY-USAGE {keyAgreement, encipherOnly, decipherOnly}
/// }
/// ```
///
///
pub fn pk_originator_dh() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: dh_public_number(),                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_keyAgreement,
            KeyUsage_encipherOnly,
            KeyUsage_decipherOnly,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_originator_dh {
    /* OBJECT_TYPES */
    use super::*;
    pub type KeyValue = DHPublicKey; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_KeyValue(el: &X690Element) -> ASN1Result<KeyValue> {
        _decode_DHPublicKey(el)
    }
    pub fn _encode_KeyValue(value_: &KeyValue) -> ASN1Result<X690Element> {
        _encode_DHPublicKey(value_)
    }
    pub fn _validate_KeyValue(el: &X690Element) -> ASN1Result<()> {
        _validate_DHPublicKey(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kwa-3DESWrap KEY-WRAP ::= {
/// IDENTIFIER id-alg-CMS3DESwrap
/// PARAMS TYPE NULL ARE required
/// SMIME-CAPS {IDENTIFIED BY id-alg-CMS3DESwrap}
/// }
/// ```
///
///
pub fn kwa_3DESWrap() -> KEY_WRAP {
    KEY_WRAP {
        id: id_alg_CMS3DESwrap(),                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(kwa_3DESWrap_smimeCaps()),  /* OBJECT_FIELD_SETTING */
    }
}

pub mod kwa_3DESWrap {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        BER.decode_null(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kwa-RC2Wrap KEY-WRAP ::= {
/// IDENTIFIER id-alg-CMSRC2wrap
/// PARAMS TYPE RC2wrapParameter ARE required
/// SMIME-CAPS { IDENTIFIED BY id-alg-CMSRC2wrap }
/// }
/// ```
///
///
pub fn kwa_RC2Wrap() -> KEY_WRAP {
    KEY_WRAP {
        id: id_alg_CMSRC2wrap(),                    /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(kwa_RC2Wrap_smimeCaps()),   /* OBJECT_FIELD_SETTING */
    }
}

pub mod kwa_RC2Wrap {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = RC2wrapParameter; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_RC2wrapParameter(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_RC2wrapParameter(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_RC2wrapParameter(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kda-PBKDF2 KEY-DERIVATION ::= {
/// IDENTIFIER id-PBKDF2
/// PARAMS TYPE PBKDF2-params ARE required
/// -- No S/MIME caps defined
/// }
/// ```
///
///
pub fn kda_PBKDF2() -> KEY_DERIVATION {
    KEY_DERIVATION {
        id: id_PBKDF2(),                            /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: None,
    }
}

pub mod kda_PBKDF2 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = PBKDF2_params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_PBKDF2_params(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_PBKDF2_params(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_PBKDF2_params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cea-3DES-cbc CONTENT-ENCRYPTION ::= {
/// IDENTIFIER des-ede3-cbc
/// PARAMS TYPE IV ARE required
/// SMIME-CAPS { IDENTIFIED BY des-ede3-cbc }
/// }
/// ```
///
///
pub fn cea_3DES_cbc() -> CONTENT_ENCRYPTION {
    CONTENT_ENCRYPTION {
        id: des_ede3_cbc(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(cea_3DES_cbc_smimeCaps()),  /* OBJECT_FIELD_SETTING */
    }
}

pub mod cea_3DES_cbc {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = IV; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_IV(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_IV(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_IV(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cea-RC2-cbc CONTENT-ENCRYPTION ::= {
/// IDENTIFIER rc2-cbc
/// PARAMS TYPE RC2CBCParameter ARE required
/// SMIME-CAPS cap-RC2CBC
/// }
/// ```
///
///
pub fn cea_RC2_cbc() -> CONTENT_ENCRYPTION {
    CONTENT_ENCRYPTION {
        id: rc2_cbc(),                              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(cap_RC2CBC()),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod cea_RC2_cbc {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = RC2CBCParameter; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_RC2CBCParameter(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_RC2CBCParameter(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_RC2CBCParameter(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kt-rsa KEY-TRANSPORT ::= {
/// IDENTIFIER rsaEncryption
/// PARAMS TYPE NULL ARE required
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS {IDENTIFIED BY rsaEncryption}
/// }
/// ```
///
///
pub fn kt_rsa() -> KEY_TRANSPORT {
    KEY_TRANSPORT {
        id: rsaEncryption(),                        /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(kt_rsa_smimeCaps()),        /* OBJECT_FIELD_SETTING */
    }
}

pub mod kt_rsa {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        BER.decode_null(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cap-3DESwrap SMIME-CAPS ::= { IDENTIFIED BY id-alg-CMS3DESwrap }
/// ```
///
///
pub fn cap_3DESwrap() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_alg_CMS3DESwrap(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod cap_3DESwrap {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// maca-hMAC-SHA1-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn maca_hMAC_SHA1_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: hMAC_SHA1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod maca_hMAC_SHA1_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PBKDF2-params-salt ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum PBKDF2_params_salt {
    specified(OCTET_STRING),
    otherSource(PBKDF2_SaltSourcesAlgorithmIdentifier),
}

impl TryFrom<&X690Element> for PBKDF2_params_salt {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PBKDF2_params_salt(el)
    }
}

pub fn _decode_PBKDF2_params_salt(el: &X690Element) -> ASN1Result<PBKDF2_params_salt> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 4) => {
            Ok(PBKDF2_params_salt::specified(BER.decode_octet_string(&el)?))
        }
        (TagClass::UNIVERSAL, 16) => Ok(PBKDF2_params_salt::otherSource(
            _decode_PBKDF2_SaltSourcesAlgorithmIdentifier(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "PBKDF2-params-salt",
            ))
        }
    }
}

pub fn _encode_PBKDF2_params_salt(value_: &PBKDF2_params_salt) -> ASN1Result<X690Element> {
    match value_ {
        PBKDF2_params_salt::specified(v) => BER.encode_octet_string(&v),
        PBKDF2_params_salt::otherSource(v) => _encode_PBKDF2_SaltSourcesAlgorithmIdentifier(&v),
    }
}

pub fn _validate_PBKDF2_params_salt(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 4) => BER.validate_octet_string(&el),
        (TagClass::UNIVERSAL, 16) => _validate_PBKDF2_SaltSourcesAlgorithmIdentifier(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "PBKDF2-params-salt",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kaa-esdh-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn kaa_esdh_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_alg_ESDH(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kaa_esdh_smimeCaps {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = KeyWrapAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_KeyWrapAlgorithm(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_KeyWrapAlgorithm(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_KeyWrapAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kaa-ssdh-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn kaa_ssdh_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_alg_SSDH(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kaa_ssdh_smimeCaps {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = KeyWrapAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_KeyWrapAlgorithm(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_KeyWrapAlgorithm(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_KeyWrapAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kwa-3DESWrap-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn kwa_3DESWrap_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_alg_CMS3DESwrap(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kwa_3DESWrap_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kwa-RC2Wrap-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn kwa_RC2Wrap_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_alg_CMSRC2wrap(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kwa_RC2Wrap_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cea-3DES-cbc-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn cea_3DES_cbc_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: des_ede3_cbc(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod cea_3DES_cbc_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kt-rsa-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn kt_rsa_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: rsaEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kt_rsa_smimeCaps {
    /* OBJECT_TYPES */
}
