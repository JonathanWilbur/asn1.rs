#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKIXAlgs-2009
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKIXAlgs-2009`.
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
use crate::PKIX1_PSS_OAEP_Algorithms_2009::*;
use asn1::*;
use std::sync::Arc;
use x500::CertificateExtensions::{
    KeyUsage_cRLSign, KeyUsage_dataEncipherment, KeyUsage_decipherOnly, KeyUsage_digitalSignature,
    KeyUsage_encipherOnly, KeyUsage_keyAgreement, KeyUsage_keyCertSign, KeyUsage_keyEncipherment,
    KeyUsage_nonRepudiation,
};
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// PublicKeys PUBLIC-KEY ::= {
/// pk-rsa	|
/// pk-dsa	|
/// pk-dh		|
/// pk-kea,
/// ...,
/// pk-ec		|
/// pk-ecDH	|
/// pk-ecMQV
/// }
/// ```
///
///
pub fn PublicKeys() -> Vec<PUBLIC_KEY> {
    Vec::from([
        pk_rsa(),
        pk_dsa(),
        pk_dh(),
        pk_kea(),
        pk_ec(),
        pk_ecDH(),
        pk_ecMQV(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureAlgs SIGNATURE-ALGORITHM ::= {
/// sa-rsaWithMD2	|
/// sa-rsaWithMD5	|
/// sa-rsaWithSHA1	|
/// sa-dsaWithSHA1	|
/// sa-ecdsaWithSHA1,
/// ..., -- Extensible
/// sa-dsaWithSHA224	|
/// sa-dsaWithSHA256	|
/// sa-ecdsaWithSHA224	|
/// sa-ecdsaWithSHA256	|
/// sa-ecdsaWithSHA384	|
/// sa-ecdsaWithSHA512
/// }
/// ```
///
///
pub fn SignatureAlgs() -> Vec<SIGNATURE_ALGORITHM> {
    Vec::from([
        sa_rsaWithMD2(),
        sa_rsaWithMD5(),
        sa_rsaWithSHA1(),
        sa_dsaWithSHA1(),
        sa_ecdsaWithSHA1(),
        sa_dsaWithSHA224(),
        sa_dsaWithSHA256(),
        sa_ecdsaWithSHA224(),
        sa_ecdsaWithSHA256(),
        sa_ecdsaWithSHA384(),
        sa_ecdsaWithSHA512(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMimeCaps SMIME-CAPS ::= {
/// sa-rsaWithMD2.&smimeCaps		|
/// sa-rsaWithMD5.&smimeCaps		|
/// sa-rsaWithSHA1.&smimeCaps		|
/// sa-dsaWithSHA1.&smimeCaps		|
/// sa-dsaWithSHA224.&smimeCaps	|
/// sa-dsaWithSHA256.&smimeCaps	|
/// sa-ecdsaWithSHA1.&smimeCaps	|
/// sa-ecdsaWithSHA224.&smimeCaps	|
/// sa-ecdsaWithSHA256.&smimeCaps	|
/// sa-ecdsaWithSHA384.&smimeCaps	|
/// sa-ecdsaWithSHA512.&smimeCaps,
/// ... }
/// ```
///
///
pub fn SMimeCaps() -> Vec<SMIME_CAPS> {
    Vec::from([
        sa_rsaWithMD2().smimeCaps.unwrap(),
        sa_rsaWithMD5().smimeCaps.unwrap(),
        sa_rsaWithSHA1().smimeCaps.unwrap(),
        sa_dsaWithSHA1().smimeCaps.unwrap(),
        sa_dsaWithSHA224().smimeCaps.unwrap(),
        sa_dsaWithSHA256().smimeCaps.unwrap(),
        sa_ecdsaWithSHA1().smimeCaps.unwrap(),
        sa_ecdsaWithSHA224().smimeCaps.unwrap(),
        sa_ecdsaWithSHA256().smimeCaps.unwrap(),
        sa_ecdsaWithSHA384().smimeCaps.unwrap(),
        sa_ecdsaWithSHA512().smimeCaps.unwrap(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-rsa PUBLIC-KEY ::= {
/// IDENTIFIER rsaEncryption
/// KEY RSAPublicKey
/// PARAMS TYPE NULL ARE absent
/// -- Private key format not in this module --
/// CERT-KEY-USAGE {digitalSignature, nonRepudiation,
/// 	keyEncipherment, dataEncipherment, keyCertSign, cRLSign}
/// }
/// ```
///
///
pub fn pk_rsa() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: rsaEncryption(),                      /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_digitalSignature,
            KeyUsage_nonRepudiation,
            KeyUsage_keyEncipherment,
            KeyUsage_dataEncipherment,
            KeyUsage_keyCertSign,
            KeyUsage_cRLSign,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_rsa {
    /* OBJECT_TYPES */
    use super::*;
    pub type KeyValue = RSAPublicKey; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_KeyValue(el: &X690Element) -> ASN1Result<KeyValue> {
        _decode_RSAPublicKey(el)
    }
    pub fn _encode_KeyValue(value_: &KeyValue) -> ASN1Result<X690Element> {
        _encode_RSAPublicKey(value_)
    }
    pub fn _validate_KeyValue(el: &X690Element) -> ASN1Result<()> {
        _validate_RSAPublicKey(el)
    }
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
/// rsaEncryption OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-1(1) 1 }
/// ```
///
///
pub fn rsaEncryption() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-1 */ 1, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RSAPublicKey ::= SEQUENCE {
/// modulus		INTEGER, -- n
/// publicExponent	INTEGER  -- e
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct RSAPublicKey {
    pub modulus: INTEGER,
    pub publicExponent: INTEGER,
}
impl RSAPublicKey {
    pub fn new(modulus: INTEGER, publicExponent: INTEGER) -> Self {
        RSAPublicKey {
            modulus,
            publicExponent,
        }
    }
}
impl TryFrom<&X690Element> for RSAPublicKey {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RSAPublicKey(el)
    }
}

pub const _rctl1_components_for_RSAPublicKey: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "modulus",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "publicExponent",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RSAPublicKey: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RSAPublicKey: &[ComponentSpec; 0] = &[];

pub fn _decode_RSAPublicKey(el: &X690Element) -> ASN1Result<RSAPublicKey> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAPublicKey")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RSAPublicKey,
        _eal_components_for_RSAPublicKey,
        _rctl2_components_for_RSAPublicKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut modulus_: OPTIONAL<INTEGER> = None;
    let mut publicExponent_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "modulus" => modulus_ = Some(BER.decode_integer(_el)?),
            "publicExponent" => publicExponent_ = Some(BER.decode_integer(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAPublicKey")
                )
            }
        }
    }
    Ok(RSAPublicKey {
        modulus: modulus_.unwrap(),
        publicExponent: publicExponent_.unwrap(),
    })
}

pub fn _encode_RSAPublicKey(value_: &RSAPublicKey) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.modulus)?);
    components_.push(BER.encode_integer(&value_.publicExponent)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_RSAPublicKey(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAPublicKey")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RSAPublicKey,
        _eal_components_for_RSAPublicKey,
        _rctl2_components_for_RSAPublicKey,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "modulus" => BER.validate_integer(_el)?,
            "publicExponent" => BER.validate_integer(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAPublicKey")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-dsa PUBLIC-KEY ::= {
/// IDENTIFIER id-dsa
/// KEY DSAPublicKey
/// PARAMS TYPE DSA-Params ARE inheritable
/// -- Private key format not in this module --
/// CERT-KEY-USAGE { digitalSignature, nonRepudiation, keyCertSign,
/// cRLSign }
/// }
/// ```
///
///
pub fn pk_dsa() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_dsa(),                                  /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_inheritable), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_digitalSignature,
            KeyUsage_nonRepudiation,
            KeyUsage_keyCertSign,
            KeyUsage_cRLSign,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_dsa {
    /* OBJECT_TYPES */
    use super::*;
    pub type KeyValue = DSAPublicKey; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_KeyValue(el: &X690Element) -> ASN1Result<KeyValue> {
        _decode_DSAPublicKey(el)
    }
    pub fn _encode_KeyValue(value_: &KeyValue) -> ASN1Result<X690Element> {
        _encode_DSAPublicKey(value_)
    }
    pub fn _validate_KeyValue(el: &X690Element) -> ASN1Result<()> {
        _validate_DSAPublicKey(el)
    }
    pub type Params = DSA_Params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_DSA_Params(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_DSA_Params(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_DSA_Params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) x9-57(10040) x9algorithm(4) 1 }
/// ```
///
///
pub fn id_dsa() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* x9-57 */ 10040,
        /* x9algorithm */ 4, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSA-Params ::= SEQUENCE {
/// p  INTEGER,
/// q  INTEGER,
/// g  INTEGER
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct DSA_Params {
    pub p: INTEGER,
    pub q: INTEGER,
    pub g: INTEGER,
}
impl DSA_Params {
    pub fn new(p: INTEGER, q: INTEGER, g: INTEGER) -> Self {
        DSA_Params { p, q, g }
    }
}
impl TryFrom<&X690Element> for DSA_Params {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DSA_Params(el)
    }
}

pub const _rctl1_components_for_DSA_Params: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "p",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "q",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "g",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DSA_Params: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DSA_Params: &[ComponentSpec; 0] = &[];

pub fn _decode_DSA_Params(el: &X690Element) -> ASN1Result<DSA_Params> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Params")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DSA_Params,
        _eal_components_for_DSA_Params,
        _rctl2_components_for_DSA_Params,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut p_: OPTIONAL<INTEGER> = None;
    let mut q_: OPTIONAL<INTEGER> = None;
    let mut g_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "p" => p_ = Some(BER.decode_integer(_el)?),
            "q" => q_ = Some(BER.decode_integer(_el)?),
            "g" => g_ = Some(BER.decode_integer(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Params"))
            }
        }
    }
    Ok(DSA_Params {
        p: p_.unwrap(),
        q: q_.unwrap(),
        g: g_.unwrap(),
    })
}

pub fn _encode_DSA_Params(value_: &DSA_Params) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(BER.encode_integer(&value_.p)?);
    components_.push(BER.encode_integer(&value_.q)?);
    components_.push(BER.encode_integer(&value_.g)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_DSA_Params(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Params")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DSA_Params,
        _eal_components_for_DSA_Params,
        _rctl2_components_for_DSA_Params,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "p" => BER.validate_integer(_el)?,
            "q" => BER.validate_integer(_el)?,
            "g" => BER.validate_integer(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Params"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSAPublicKey  ::=  INTEGER
/// ```
pub type DSAPublicKey = INTEGER;

pub fn _decode_DSAPublicKey(el: &X690Element) -> ASN1Result<DSAPublicKey> {
    BER.decode_integer(&el)
}

pub fn _encode_DSAPublicKey(value_: &DSAPublicKey) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_DSAPublicKey(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-dh PUBLIC-KEY ::= {
/// IDENTIFIER dhpublicnumber
/// KEY DHPublicKey
/// PARAMS TYPE DomainParameters ARE inheritable
/// -- Private key format not in this module --
/// CERT-KEY-USAGE {keyAgreement, encipherOnly, decipherOnly }
/// }
/// ```
///
///
pub fn pk_dh() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: dhpublicnumber(),                          /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_inheritable), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_keyAgreement,
            KeyUsage_encipherOnly,
            KeyUsage_decipherOnly,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_dh {
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
    pub type Params = DomainParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_DomainParameters(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_DomainParameters(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_DomainParameters(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dhpublicnumber OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-x942(10046) number-type(2) 1 }
/// ```
///
///
pub fn dhpublicnumber() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-x942 */ 10046,
        /* number-type */ 2, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DomainParameters ::= SEQUENCE {
/// p				INTEGER,           -- odd prime, p=jq +1
/// g				INTEGER,           -- generator, g
/// q				INTEGER,           -- factor of p-1
/// j				INTEGER OPTIONAL,  -- subgroup factor, j>= 2
/// validationParams	ValidationParams OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct DomainParameters {
    pub p: INTEGER,
    pub g: INTEGER,
    pub q: INTEGER,
    pub j: OPTIONAL<INTEGER>,
    pub validationParams: OPTIONAL<ValidationParams>,
}
impl DomainParameters {
    pub fn new(
        p: INTEGER,
        g: INTEGER,
        q: INTEGER,
        j: OPTIONAL<INTEGER>,
        validationParams: OPTIONAL<ValidationParams>,
    ) -> Self {
        DomainParameters {
            p,
            g,
            q,
            j,
            validationParams,
        }
    }
}
impl TryFrom<&X690Element> for DomainParameters {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DomainParameters(el)
    }
}

pub const _rctl1_components_for_DomainParameters: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "p",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "g",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "q",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "j",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "validationParams",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DomainParameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DomainParameters: &[ComponentSpec; 0] = &[];

pub fn _decode_DomainParameters(el: &X690Element) -> ASN1Result<DomainParameters> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DomainParameters")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DomainParameters,
        _eal_components_for_DomainParameters,
        _rctl2_components_for_DomainParameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut p_: OPTIONAL<INTEGER> = None;
    let mut g_: OPTIONAL<INTEGER> = None;
    let mut q_: OPTIONAL<INTEGER> = None;
    let mut j_: OPTIONAL<INTEGER> = None;
    let mut validationParams_: OPTIONAL<ValidationParams> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "p" => p_ = Some(BER.decode_integer(_el)?),
            "g" => g_ = Some(BER.decode_integer(_el)?),
            "q" => q_ = Some(BER.decode_integer(_el)?),
            "j" => j_ = Some(BER.decode_integer(_el)?),
            "validationParams" => validationParams_ = Some(_decode_ValidationParams(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DomainParameters")
                )
            }
        }
    }
    Ok(DomainParameters {
        p: p_.unwrap(),
        g: g_.unwrap(),
        q: q_.unwrap(),
        j: j_,
        validationParams: validationParams_,
    })
}

pub fn _encode_DomainParameters(value_: &DomainParameters) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    components_.push(BER.encode_integer(&value_.p)?);
    components_.push(BER.encode_integer(&value_.g)?);
    components_.push(BER.encode_integer(&value_.q)?);
    if let Some(v_) = &value_.j {
        components_.push(BER.encode_integer(&v_)?);
    }
    if let Some(v_) = &value_.validationParams {
        components_.push(_encode_ValidationParams(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_DomainParameters(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DomainParameters")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DomainParameters,
        _eal_components_for_DomainParameters,
        _rctl2_components_for_DomainParameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "p" => BER.validate_integer(_el)?,
            "g" => BER.validate_integer(_el)?,
            "q" => BER.validate_integer(_el)?,
            "j" => BER.validate_integer(_el)?,
            "validationParams" => _validate_ValidationParams(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DomainParameters")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ValidationParams ::= SEQUENCE {
/// seed		BIT STRING,
/// pgenCounter	INTEGER
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ValidationParams {
    pub seed: BIT_STRING,
    pub pgenCounter: INTEGER,
}
impl ValidationParams {
    pub fn new(seed: BIT_STRING, pgenCounter: INTEGER) -> Self {
        ValidationParams { seed, pgenCounter }
    }
}
impl TryFrom<&X690Element> for ValidationParams {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ValidationParams(el)
    }
}

pub const _rctl1_components_for_ValidationParams: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "seed",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pgenCounter",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ValidationParams: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ValidationParams: &[ComponentSpec; 0] = &[];

pub fn _decode_ValidationParams(el: &X690Element) -> ASN1Result<ValidationParams> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ValidationParams")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ValidationParams,
        _eal_components_for_ValidationParams,
        _rctl2_components_for_ValidationParams,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut seed_: OPTIONAL<BIT_STRING> = None;
    let mut pgenCounter_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "seed" => seed_ = Some(BER.decode_bit_string(_el)?),
            "pgenCounter" => pgenCounter_ = Some(BER.decode_integer(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ValidationParams")
                )
            }
        }
    }
    Ok(ValidationParams {
        seed: seed_.unwrap(),
        pgenCounter: pgenCounter_.unwrap(),
    })
}

pub fn _encode_ValidationParams(value_: &ValidationParams) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_bit_string(&value_.seed)?);
    components_.push(BER.encode_integer(&value_.pgenCounter)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ValidationParams(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ValidationParams")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ValidationParams,
        _eal_components_for_ValidationParams,
        _rctl2_components_for_ValidationParams,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "seed" => BER.validate_bit_string(_el)?,
            "pgenCounter" => BER.validate_integer(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ValidationParams")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DHPublicKey  ::=  INTEGER
/// ```
pub type DHPublicKey = INTEGER;

pub fn _decode_DHPublicKey(el: &X690Element) -> ASN1Result<DHPublicKey> {
    BER.decode_integer(&el)
}

pub fn _encode_DHPublicKey(value_: &DHPublicKey) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_DHPublicKey(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-kea PUBLIC-KEY ::= {
/// IDENTIFIER id-keyExchangeAlgorithm
/// -- key is not encoded --
/// PARAMS TYPE KEA-Params-Id ARE required
/// -- Private key format not in this module --
/// CERT-KEY-USAGE {keyAgreement, encipherOnly, decipherOnly }
/// }
/// ```
///
///
pub fn pk_kea() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_keyExchangeAlgorithm(),              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_keyAgreement,
            KeyUsage_encipherOnly,
            KeyUsage_decipherOnly,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_kea {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = KEA_Params_Id; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_KEA_Params_Id(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_KEA_Params_Id(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_KEA_Params_Id(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-keyExchangeAlgorithm OBJECT IDENTIFIER ::= {
/// joint-iso-itu-t(2) country(16) us(840) organization(1)
/// gov(101) dod(2) infosec(1) algorithms(1) 22 }
/// ```
///
///
pub fn id_keyExchangeAlgorithm() -> OBJECT_IDENTIFIER {
    oid!(
        /* joint-iso-itu-t */ 2, /* country */ 16, /* us */ 840,
        /* organization */ 1, /* gov */ 101, /* dod */ 2, /* infosec */ 1,
        /* algorithms */ 1, 22) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEA-Params-Id  ::=  OCTET STRING
/// ```
pub type KEA_Params_Id = OCTET_STRING; // OctetStringType

pub fn _decode_KEA_Params_Id(el: &X690Element) -> ASN1Result<KEA_Params_Id> {
    BER.decode_octet_string(&el)
}

pub fn _encode_KEA_Params_Id(value_: &KEA_Params_Id) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_KEA_Params_Id(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-ec PUBLIC-KEY ::= {
/// IDENTIFIER id-ecPublicKey
/// KEY ECPoint
/// PARAMS TYPE ECParameters ARE required
/// -- Private key format not in this module --
/// CERT-KEY-USAGE { digitalSignature, nonRepudiation, keyAgreement,
///                 keyCertSign, cRLSign }
/// }
/// ```
///
///
pub fn pk_ec() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_ecPublicKey(),                       /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_digitalSignature,
            KeyUsage_nonRepudiation,
            KeyUsage_keyAgreement,
            KeyUsage_keyCertSign,
            KeyUsage_cRLSign,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_ec {
    /* OBJECT_TYPES */
    use super::*;
    pub type KeyValue = ECPoint; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_KeyValue(el: &X690Element) -> ASN1Result<KeyValue> {
        _decode_ECPoint(el)
    }
    pub fn _encode_KeyValue(value_: &KeyValue) -> ASN1Result<X690Element> {
        _encode_ECPoint(value_)
    }
    pub fn _validate_KeyValue(el: &X690Element) -> ASN1Result<()> {
        _validate_ECPoint(el)
    }
    pub type Params = ECParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_ECParameters(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_ECParameters(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_ECParameters(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ECPoint  ::=  OCTET STRING
/// ```
pub type ECPoint = OCTET_STRING; // OctetStringType

pub fn _decode_ECPoint(el: &X690Element) -> ASN1Result<ECPoint> {
    BER.decode_octet_string(&el)
}

pub fn _encode_ECPoint(value_: &ECPoint) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_ECPoint(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecPublicKey OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) keyType(2) 1 }
/// ```
///
///
pub fn id_ecPublicKey() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* keyType */ 2, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-ecDH PUBLIC-KEY ::= {
/// IDENTIFIER id-ecDH
/// KEY ECPoint
/// PARAMS TYPE ECParameters ARE required
/// -- Private key format not in this module --
/// CERT-KEY-USAGE { keyAgreement, encipherOnly, decipherOnly }
/// }
/// ```
///
///
pub fn pk_ecDH() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_ecDH(),                              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_keyAgreement,
            KeyUsage_encipherOnly,
            KeyUsage_decipherOnly,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_ecDH {
    /* OBJECT_TYPES */
    use super::*;
    pub type KeyValue = ECPoint; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_KeyValue(el: &X690Element) -> ASN1Result<KeyValue> {
        _decode_ECPoint(el)
    }
    pub fn _encode_KeyValue(value_: &KeyValue) -> ASN1Result<X690Element> {
        _encode_ECPoint(value_)
    }
    pub fn _validate_KeyValue(el: &X690Element) -> ASN1Result<()> {
        _validate_ECPoint(el)
    }
    pub type Params = ECParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_ECParameters(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_ECParameters(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_ECParameters(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecDH OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) schemes(1) ecdh(12) }
/// ```
///
///
pub fn id_ecDH() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* schemes */ 1, /* ecdh */ 12) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-ecMQV PUBLIC-KEY ::= {
/// IDENTIFIER id-ecMQV
/// KEY ECPoint
/// PARAMS TYPE ECParameters ARE required
/// -- Private key format not in this module --
/// CERT-KEY-USAGE { keyAgreement, encipherOnly, decipherOnly }
/// }
/// ```
///
///
pub fn pk_ecMQV() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_ecMQV(),                             /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_keyAgreement,
            KeyUsage_encipherOnly,
            KeyUsage_decipherOnly,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_ecMQV {
    /* OBJECT_TYPES */
    use super::*;
    pub type KeyValue = ECPoint; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_KeyValue(el: &X690Element) -> ASN1Result<KeyValue> {
        _decode_ECPoint(el)
    }
    pub fn _encode_KeyValue(value_: &KeyValue) -> ASN1Result<X690Element> {
        _encode_ECPoint(value_)
    }
    pub fn _validate_KeyValue(el: &X690Element) -> ASN1Result<()> {
        _validate_ECPoint(el)
    }
    pub type Params = ECParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_ECParameters(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_ECParameters(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_ECParameters(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecMQV OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) schemes(1) ecmqv(13) }
/// ```
///
///
pub fn id_ecMQV() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* schemes */ 1, /* ecmqv */ 13) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ECParameters  ::=  CHOICE {
/// namedCurve      CURVE.&id({NamedCurve})
/// -- implicitCurve   NULL
/// -- implicitCurve MUST NOT be used in PKIX
/// -- specifiedCurve  SpecifiedCurve
/// -- specifiedCurve MUST NOT be used in PKIX
/// -- Details for specifiedCurve can be found in [X9.62]
/// -- Any future additions to this CHOICE should be coordinated
/// -- with ANSI X.9.
/// }
/// ```
#[derive(Debug, Clone)]
pub enum ECParameters {
    namedCurve(OBJECT_IDENTIFIER),
}

impl TryFrom<&X690Element> for ECParameters {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ECParameters(el)
    }
}

pub fn _decode_ECParameters(el: &X690Element) -> ASN1Result<ECParameters> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 6) => {
            Ok(ECParameters::namedCurve(BER.decode_object_identifier(&el)?))
        }
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ECParameters",
            ))
        }
    }
}

pub fn _encode_ECParameters(value_: &ECParameters) -> ASN1Result<X690Element> {
    match value_ {
        ECParameters::namedCurve(v) => BER.encode_object_identifier(&v),
    }
}

pub fn _validate_ECParameters(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ECParameters",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CURVE ::= CLASS { &id OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX { ID &id }
/// ```
///
#[derive(Debug)]
pub struct CURVE {
    pub id: OBJECT_IDENTIFIER,
}
impl CURVE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve CURVE ::= {
/// { ID secp192r1 } | { ID sect163k1 } | { ID sect163r2 } |
/// { ID secp224r1 } | { ID sect233k1 } | { ID sect233r1 } |
/// { ID secp256r1 } | { ID sect283k1 } | { ID sect283r1 } |
/// { ID secp384r1 } | { ID sect409k1 } | { ID sect409r1 } |
/// { ID secp521r1 } | { ID sect571k1 } | { ID sect571r1 },
/// ... -- Extensible
/// }
/// ```
///
///
pub fn NamedCurve() -> Vec<CURVE> {
    [
        NamedCurve_Union0_Intersection0_Element(),
        NamedCurve_Union1_Intersection0_Element(),
        NamedCurve_Union2_Intersection0_Element(),
        NamedCurve_Union3_Intersection0_Element(),
        NamedCurve_Union4_Intersection0_Element(),
        NamedCurve_Union5_Intersection0_Element(),
        NamedCurve_Union6_Intersection0_Element(),
        NamedCurve_Union7_Intersection0_Element(),
        NamedCurve_Union8_Intersection0_Element(),
        NamedCurve_Union9_Intersection0_Element(),
        NamedCurve_Union10_Intersection0_Element(),
        NamedCurve_Union11_Intersection0_Element(),
        NamedCurve_Union12_Intersection0_Element(),
        NamedCurve_Union13_Intersection0_Element(),
        NamedCurve_Union14_Intersection0_Element(),
    ]
    .into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp192r1 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) curves(3)
/// prime(1) 1 }
/// ```
///
///
pub fn secp192r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* curves */ 3, /* prime */ 1, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect163k1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 1 }
/// ```
///
///
pub fn sect163k1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect163r2 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 15 }
/// ```
///
///
pub fn sect163r2() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 15) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp224r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 33 }
/// ```
///
///
pub fn secp224r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 33) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect233k1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 26 }
/// ```
///
///
pub fn sect233k1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 26) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect233r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 27 }
/// ```
///
///
pub fn sect233r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 27) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp256r1 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) curves(3)
/// prime(1) 7 }
/// ```
///
///
pub fn secp256r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* curves */ 3, /* prime */ 1, 7) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect283k1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 16 }
/// ```
///
///
pub fn sect283k1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 16) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect283r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 17 }
/// ```
///
///
pub fn sect283r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 17) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp384r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 34 }
/// ```
///
///
pub fn secp384r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 34) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect409k1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 36 }
/// ```
///
///
pub fn sect409k1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 36) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect409r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 37 }
/// ```
///
///
pub fn sect409r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 37) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp521r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 35 }
/// ```
///
///
pub fn secp521r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 35) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect571k1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 38 }
/// ```
///
///
pub fn sect571k1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 38) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect571r1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) certicom(132) curve(0) 39 }
/// ```
///
///
pub fn sect571r1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* certicom */ 132,
        /* curve */ 0, 39) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaWithMD2 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER md2WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-md2 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS { IDENTIFIED BY md2WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_rsaWithMD2() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: md2WithRSAEncryption(),                 /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_md2()])),      /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_rsaWithMD2_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaWithMD2 {
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
/// md2WithRSAEncryption OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1)
/// pkcs-1(1) 2 }
/// ```
///
///
pub fn md2WithRSAEncryption() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-1 */ 1, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaWithMD5 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER md5WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-md5 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS { IDENTIFIED BY md5WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_rsaWithMD5() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: md5WithRSAEncryption(),                 /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_md5()])),      /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_rsaWithMD5_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaWithMD5 {
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
/// md5WithRSAEncryption OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-1(1) 4 }
/// ```
///
///
pub fn md5WithRSAEncryption() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-1 */ 1, 4) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaWithSHA1 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER sha1WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-sha1 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS {IDENTIFIED BY sha1WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_rsaWithSHA1() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: sha1WithRSAEncryption(),                 /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required),  /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha1()])),      /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),   /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_rsaWithSHA1_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaWithSHA1 {
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
/// sha1WithRSAEncryption OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-1(1) 5 }
/// ```
///
///
pub fn sha1WithRSAEncryption() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-1 */ 1, 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-dsaWithSHA1 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER dsa-with-sha1
/// VALUE DSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha1 }
/// PUBLIC-KEYS { pk-dsa }
/// SMIME-CAPS { IDENTIFIED BY dsa-with-sha1 }
/// }
/// ```
///
///
pub fn sa_dsaWithSHA1() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: dsa_with_sha1(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),    /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha1()])),      /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_dsa()])),   /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_dsaWithSHA1_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_dsaWithSHA1 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = DSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_DSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_DSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_DSA_Sig_Value(el)
    }
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
/// dsa-with-sha1 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) x9-57(10040) x9algorithm(4) 3 }
/// ```
///
///
pub fn dsa_with_sha1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* x9-57 */ 10040,
        /* x9algorithm */ 4, 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-dsaWithSHA224 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER dsa-with-sha224
/// VALUE DSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha224 }
/// PUBLIC-KEYS { pk-dsa }
/// SMIME-CAPS { IDENTIFIED BY dsa-with-sha224 }
/// }
/// ```
///
///
pub fn sa_dsaWithSHA224() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: dsa_with_sha224(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),      /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha224()])),      /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_dsa()])),     /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_dsaWithSHA224_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_dsaWithSHA224 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = DSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_DSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_DSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_DSA_Sig_Value(el)
    }
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
/// dsa-with-sha224 OBJECT IDENTIFIER ::= {
/// joint-iso-ccitt(2) country(16) us(840) organization(1) gov(101)
/// csor(3) algorithms(4) id-dsa-with-sha2(3) 1 }
/// ```
///
///
pub fn dsa_with_sha224() -> OBJECT_IDENTIFIER {
    oid!(
        /* joint-iso-ccitt */ 2, /* country */ 16, /* us */ 840,
        /* organization */ 1, /* gov */ 101, /* csor */ 3, /* algorithms */ 4,
        /* id-dsa-with-sha2 */ 3, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-dsaWithSHA256 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER dsa-with-sha256
/// VALUE DSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha256 }
/// PUBLIC-KEYS { pk-dsa }
/// SMIME-CAPS { IDENTIFIED BY dsa-with-sha256 }
/// }
/// ```
///
///
pub fn sa_dsaWithSHA256() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: dsa_with_sha256(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),      /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha256()])),      /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_dsa()])),     /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_dsaWithSHA256_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_dsaWithSHA256 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = DSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_DSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_DSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_DSA_Sig_Value(el)
    }
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
/// dsa-with-sha256 OBJECT IDENTIFIER ::= {
/// joint-iso-ccitt(2) country(16) us(840) organization(1) gov(101)
/// csor(3) algorithms(4) id-dsa-with-sha2(3) 2 }
/// ```
///
///
pub fn dsa_with_sha256() -> OBJECT_IDENTIFIER {
    oid!(
        /* joint-iso-ccitt */ 2, /* country */ 16, /* us */ 840,
        /* organization */ 1, /* gov */ 101, /* csor */ 3, /* algorithms */ 4,
        /* id-dsa-with-sha2 */ 3, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA1 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER ecdsa-with-SHA1
/// VALUE ECDSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha1 }
/// PUBLIC-KEYS { pk-ec }
/// SMIME-CAPS {IDENTIFIED BY ecdsa-with-SHA1 }
/// }
/// ```
///
///
pub fn sa_ecdsaWithSHA1() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: ecdsa_with_SHA1(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),      /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha1()])),        /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_ec()])),      /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_ecdsaWithSHA1_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA1 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = ECDSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_ECDSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_ECDSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_ECDSA_Sig_Value(el)
    }
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
/// ecdsa-with-SHA1 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045)
/// signatures(4) 1 }
/// ```
///
///
pub fn ecdsa_with_SHA1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* signatures */ 4, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA224 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER ecdsa-with-SHA224
/// VALUE ECDSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha224 }
/// PUBLIC-KEYS { pk-ec }
/// SMIME-CAPS { IDENTIFIED BY ecdsa-with-SHA224 }
/// }
/// ```
///
///
pub fn sa_ecdsaWithSHA224() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: ecdsa_with_SHA224(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),        /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha224()])),        /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_ec()])),        /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_ecdsaWithSHA224_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA224 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = ECDSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_ECDSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_ECDSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_ECDSA_Sig_Value(el)
    }
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
/// ecdsa-with-SHA224 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) signatures(4)
/// ecdsa-with-SHA2(3) 1 }
/// ```
///
///
pub fn ecdsa_with_SHA224() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA256 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER ecdsa-with-SHA256
/// VALUE ECDSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha256 }
/// PUBLIC-KEYS { pk-ec }
/// SMIME-CAPS { IDENTIFIED BY ecdsa-with-SHA256 }
/// }
/// ```
///
///
pub fn sa_ecdsaWithSHA256() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: ecdsa_with_SHA256(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),        /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha256()])),        /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_ec()])),        /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_ecdsaWithSHA256_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA256 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = ECDSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_ECDSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_ECDSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_ECDSA_Sig_Value(el)
    }
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
/// ecdsa-with-SHA256 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) signatures(4)
/// ecdsa-with-SHA2(3) 2 }
/// ```
///
///
pub fn ecdsa_with_SHA256() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA384 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER ecdsa-with-SHA384
/// VALUE ECDSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha384 }
/// PUBLIC-KEYS { pk-ec }
/// SMIME-CAPS { IDENTIFIED BY ecdsa-with-SHA384 }
/// }
/// ```
///
///
pub fn sa_ecdsaWithSHA384() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: ecdsa_with_SHA384(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),        /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha384()])),        /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_ec()])),        /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_ecdsaWithSHA384_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA384 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = ECDSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_ECDSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_ECDSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_ECDSA_Sig_Value(el)
    }
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
/// ecdsa-with-SHA384 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) signatures(4)
/// ecdsa-with-SHA2(3) 3 }
/// ```
///
///
pub fn ecdsa_with_SHA384() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA512 SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER ecdsa-with-SHA512
/// VALUE ECDSA-Sig-Value
/// PARAMS TYPE NULL ARE absent
/// HASHES { mda-sha512 }
/// PUBLIC-KEYS { pk-ec }
/// SMIME-CAPS { IDENTIFIED BY ecdsa-with-SHA512 }
/// }
/// ```
///
///
pub fn sa_ecdsaWithSHA512() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: ecdsa_with_SHA512(),                         /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_absent),        /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha512()])),        /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_ec()])),        /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_ecdsaWithSHA512_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA512 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Value = ECDSA_Sig_Value; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Value(el: &X690Element) -> ASN1Result<Value> {
        _decode_ECDSA_Sig_Value(el)
    }
    pub fn _encode_Value(value_: &Value) -> ASN1Result<X690Element> {
        _encode_ECDSA_Sig_Value(value_)
    }
    pub fn _validate_Value(el: &X690Element) -> ASN1Result<()> {
        _validate_ECDSA_Sig_Value(el)
    }
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
/// ecdsa-with-SHA512 OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) ansi-X9-62(10045) signatures(4)
/// ecdsa-with-SHA2(3) 4 }
/// ```
///
///
pub fn ecdsa_with_SHA512() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 4) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSA-Sig-Value ::= SEQUENCE {
/// r	INTEGER,
/// s	INTEGER
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct DSA_Sig_Value {
    pub r: INTEGER,
    pub s: INTEGER,
}
impl DSA_Sig_Value {
    pub fn new(r: INTEGER, s: INTEGER) -> Self {
        DSA_Sig_Value { r, s }
    }
}
impl TryFrom<&X690Element> for DSA_Sig_Value {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DSA_Sig_Value(el)
    }
}

pub const _rctl1_components_for_DSA_Sig_Value: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "r",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "s",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DSA_Sig_Value: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DSA_Sig_Value: &[ComponentSpec; 0] = &[];

pub fn _decode_DSA_Sig_Value(el: &X690Element) -> ASN1Result<DSA_Sig_Value> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Sig-Value")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DSA_Sig_Value,
        _eal_components_for_DSA_Sig_Value,
        _rctl2_components_for_DSA_Sig_Value,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut r_: OPTIONAL<INTEGER> = None;
    let mut s_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "r" => r_ = Some(BER.decode_integer(_el)?),
            "s" => s_ = Some(BER.decode_integer(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Sig-Value")
                )
            }
        }
    }
    Ok(DSA_Sig_Value {
        r: r_.unwrap(),
        s: s_.unwrap(),
    })
}

pub fn _encode_DSA_Sig_Value(value_: &DSA_Sig_Value) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.r)?);
    components_.push(BER.encode_integer(&value_.s)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_DSA_Sig_Value(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Sig-Value")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DSA_Sig_Value,
        _eal_components_for_DSA_Sig_Value,
        _rctl2_components_for_DSA_Sig_Value,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "r" => BER.validate_integer(_el)?,
            "s" => BER.validate_integer(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSA-Sig-Value")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ECDSA-Sig-Value ::= SEQUENCE {
/// r	INTEGER,
/// s	INTEGER
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ECDSA_Sig_Value {
    pub r: INTEGER,
    pub s: INTEGER,
}
impl ECDSA_Sig_Value {
    pub fn new(r: INTEGER, s: INTEGER) -> Self {
        ECDSA_Sig_Value { r, s }
    }
}
impl TryFrom<&X690Element> for ECDSA_Sig_Value {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ECDSA_Sig_Value(el)
    }
}

pub const _rctl1_components_for_ECDSA_Sig_Value: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "r",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "s",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ECDSA_Sig_Value: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ECDSA_Sig_Value: &[ComponentSpec; 0] = &[];

pub fn _decode_ECDSA_Sig_Value(el: &X690Element) -> ASN1Result<ECDSA_Sig_Value> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ECDSA-Sig-Value"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ECDSA_Sig_Value,
        _eal_components_for_ECDSA_Sig_Value,
        _rctl2_components_for_ECDSA_Sig_Value,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut r_: OPTIONAL<INTEGER> = None;
    let mut s_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "r" => r_ = Some(BER.decode_integer(_el)?),
            "s" => s_ = Some(BER.decode_integer(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ECDSA-Sig-Value")
                )
            }
        }
    }
    Ok(ECDSA_Sig_Value {
        r: r_.unwrap(),
        s: s_.unwrap(),
    })
}

pub fn _encode_ECDSA_Sig_Value(value_: &ECDSA_Sig_Value) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.r)?);
    components_.push(BER.encode_integer(&value_.s)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ECDSA_Sig_Value(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ECDSA-Sig-Value"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ECDSA_Sig_Value,
        _eal_components_for_ECDSA_Sig_Value,
        _rctl2_components_for_ECDSA_Sig_Value,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "r" => BER.validate_integer(_el)?,
            "s" => BER.validate_integer(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ECDSA-Sig-Value")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashAlgs DIGEST-ALGORITHM ::= {
/// mda-md2	|
/// mda-md5	|
/// mda-sha1,
/// ... -- Extensible
/// }
/// ```
///
///
pub fn HashAlgs() -> Vec<DIGEST_ALGORITHM> {
    Vec::from([mda_md2(), mda_md5(), mda_sha1()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-md2 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-md2
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_md2() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_md2(),                                      /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_md2 {
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
/// id-md2  OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549)
/// digestAlgorithm(2) 2 }
/// ```
///
///
pub fn id_md2() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* digestAlgorithm */ 2, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-md5 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-md5
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_md5() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_md5(),                                      /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_md5 {
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
/// id-md5  OBJECT IDENTIFIER ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549)
/// digestAlgorithm(2) 5 }
/// ```
///
///
pub fn id_md5() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* digestAlgorithm */ 2, 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-sha1 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-sha1
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_sha1() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha1(),                                     /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_sha1 {
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
/// id-sha1 OBJECT IDENTIFIER ::= {
/// iso(1) identified-organization(3) oiw(14) secsig(3)
/// algorithm(2) 26 }
/// ```
///
///
pub fn id_sha1() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* identified-organization */ 3, /* oiw */ 14,
        /* secsig */ 3, /* algorithm */ 2, 26) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union0_Intersection0_Element() -> CURVE {
    CURVE {
        id: secp192r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union0_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union1-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union1_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect163k1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union1_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union2-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union2_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect163r2(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union2_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union3-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union3_Intersection0_Element() -> CURVE {
    CURVE {
        id: secp224r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union3_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union4-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union4_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect233k1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union4_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union5-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union5_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect233r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union5_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union6-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union6_Intersection0_Element() -> CURVE {
    CURVE {
        id: secp256r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union6_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union7-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union7_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect283k1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union7_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union8-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union8_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect283r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union8_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union9-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union9_Intersection0_Element() -> CURVE {
    CURVE {
        id: secp384r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union9_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union10-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union10_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect409k1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union10_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union11-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union11_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect409r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union11_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union12-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union12_Intersection0_Element() -> CURVE {
    CURVE {
        id: secp521r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union12_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union13-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union13_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect571k1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union13_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedCurve-Union14-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn NamedCurve_Union14_Intersection0_Element() -> CURVE {
    CURVE {
        id: sect571r1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod NamedCurve_Union14_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaWithMD2-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_rsaWithMD2_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: md2WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaWithMD2_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaWithMD5-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_rsaWithMD5_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: md5WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaWithMD5_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaWithSHA1-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_rsaWithSHA1_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: sha1WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaWithSHA1_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-dsaWithSHA1-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_dsaWithSHA1_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: dsa_with_sha1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_dsaWithSHA1_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-dsaWithSHA224-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_dsaWithSHA224_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: dsa_with_sha224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_dsaWithSHA224_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-dsaWithSHA256-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_dsaWithSHA256_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: dsa_with_sha256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_dsaWithSHA256_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA1-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_ecdsaWithSHA1_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: ecdsa_with_SHA1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA1_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA224-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_ecdsaWithSHA224_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: ecdsa_with_SHA224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA224_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA256-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_ecdsaWithSHA256_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: ecdsa_with_SHA256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA256_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA384-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_ecdsaWithSHA384_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: ecdsa_with_SHA384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA384_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-ecdsaWithSHA512-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_ecdsaWithSHA512_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: ecdsa_with_SHA512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_ecdsaWithSHA512_smimeCaps {
    /* OBJECT_TYPES */
}
