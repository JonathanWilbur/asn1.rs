#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKIX1-PSS-OAEP-Algorithms-2009
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKIX1-PSS-OAEP-Algorithms-2009`.
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
use asn1::*;
use std::sync::Arc;
use x500::AuthenticationFramework::{
    AlgorithmIdentifier, _decode_AlgorithmIdentifier, _encode_AlgorithmIdentifier,
    _validate_AlgorithmIdentifier,
};
use x500::CertificateExtensions::{
    KeyUsage_cRLSign, KeyUsage_dataEncipherment, KeyUsage_digitalSignature, KeyUsage_keyCertSign,
    KeyUsage_keyEncipherment, KeyUsage_nonRepudiation,
};
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// PublicKeys PUBLIC-KEY ::= { pk-rsaSSA-PSS | pk-rsaES-OAEP, ... }
/// ```
///
///
pub fn PublicKeys() -> Vec<PUBLIC_KEY> {
    Vec::from([pk_rsaSSA_PSS(), pk_rsaES_OAEP()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignatureAlgs SIGNATURE-ALGORITHM ::= { sa-rsaSSA-PSS, ...}
/// ```
///
///
pub fn SignatureAlgs() -> Vec<SIGNATURE_ALGORITHM> {
    Vec::from([sa_rsaSSA_PSS()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyTransportAlgs KEY-TRANSPORT ::= { kta-rsaES-OAEP, ... }
/// ```
///
///
pub fn KeyTransportAlgs() -> Vec<KEY_TRANSPORT> {
    Vec::from([kta_rsaES_OAEP()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashAlgs DIGEST-ALGORITHM ::= { mda-sha224 | mda-sha256 | mda-sha384 | mda-sha512,
/// ...
/// }
/// ```
///
///
pub fn HashAlgs() -> Vec<DIGEST_ALGORITHM> {
    Vec::from([mda_sha224(), mda_sha256(), mda_sha384(), mda_sha512()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMimeCaps SMIME-CAPS ::= {
/// sa-rsaSSA-PSS.&smimeCaps |
/// kta-rsaES-OAEP.&smimeCaps,
/// ...
/// }
/// ```
///
///
pub fn SMimeCaps() -> Vec<SMIME_CAPS> {
    Vec::from([
        sa_rsaSSA_PSS().smimeCaps.unwrap(),
        kta_rsaES_OAEP().smimeCaps.unwrap(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-rsaSSA-PSS PUBLIC-KEY ::= {
/// IDENTIFIER id-RSASSA-PSS
/// KEY RSAPublicKey
/// PARAMS TYPE RSASSA-PSS-params ARE optional
/// -- Private key format not in this module --
/// CERT-KEY-USAGE { nonRepudiation, digitalSignature,
/// keyCertSign, cRLSign }
/// }
/// ```
///
///
pub fn pk_rsaSSA_PSS() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_RSASSA_PSS(),                        /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_optional), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_nonRepudiation,
            KeyUsage_digitalSignature,
            KeyUsage_keyCertSign,
            KeyUsage_cRLSign,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_rsaSSA_PSS {
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
    pub type Params = RSASSA_PSS_params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_RSASSA_PSS_params(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_RSASSA_PSS_params(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_RSASSA_PSS_params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaSSA-PSS SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER id-RSASSA-PSS
/// PARAMS TYPE RSASSA-PSS-params ARE required
/// HASHES { mda-sha1 | mda-sha224 | mda-sha256 | mda-sha384
/// | mda-sha512 }
/// PUBLIC-KEYS { pk-rsa | pk-rsaSSA-PSS }
/// SMIME-CAPS { IDENTIFIED BY id-RSASSA-PSS }
/// }
/// ```
///
///
pub fn sa_rsaSSA_PSS() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: id_RSASSA_PSS(),                        /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([
            mda_sha1(),
            mda_sha224(),
            mda_sha256(),
            mda_sha384(),
            mda_sha512(),
        ])), /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa(), pk_rsaSSA_PSS()])), /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_rsaSSA_PSS_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaSSA_PSS {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = RSASSA_PSS_params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_RSASSA_PSS_params(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_RSASSA_PSS_params(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_RSASSA_PSS_params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha224WithRSAEncryption SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER sha224WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-sha224 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS { IDENTIFIED BY sha224WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_sha224WithRSAEncryption() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: sha224WithRSAEncryption(),              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha224()])),   /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_sha224WithRSAEncryption_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha224WithRSAEncryption {
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
/// sha224WithRSAEncryption  OBJECT IDENTIFIER ::= { pkcs-1 14 }
/// ```
///
#[inline]
pub fn sha224WithRSAEncryption () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(pkcs_1(), 14).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha256WithRSAEncryption SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER sha256WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-sha256 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS { IDENTIFIED BY sha256WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_sha256WithRSAEncryption() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: sha256WithRSAEncryption(),              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha256()])),   /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_sha256WithRSAEncryption_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha256WithRSAEncryption {
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
/// sha256WithRSAEncryption  OBJECT IDENTIFIER ::= { pkcs-1 11 }
/// ```
///
#[inline]
pub fn sha256WithRSAEncryption () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(pkcs_1(), 11).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha384WithRSAEncryption SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER sha384WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-sha384 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS { IDENTIFIED BY sha384WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_sha384WithRSAEncryption() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: sha384WithRSAEncryption(),              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha384()])),   /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_sha384WithRSAEncryption_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha384WithRSAEncryption {
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
/// sha384WithRSAEncryption  OBJECT IDENTIFIER ::= { pkcs-1 12 }
/// ```
///
#[inline]
pub fn sha384WithRSAEncryption () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(pkcs_1(), 12).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha512WithRSAEncryption SIGNATURE-ALGORITHM ::= {
/// IDENTIFIER sha512WithRSAEncryption
/// PARAMS TYPE NULL ARE required
/// HASHES { mda-sha512 }
/// PUBLIC-KEYS { pk-rsa }
/// SMIME-CAPS { IDENTIFIED BY sha512WithRSAEncryption }
/// }
/// ```
///
///
pub fn sa_sha512WithRSAEncryption() -> SIGNATURE_ALGORITHM {
    SIGNATURE_ALGORITHM {
        id: sha512WithRSAEncryption(),              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        HashSet: Some(Vec::from([mda_sha512()])),   /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa()])),  /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(sa_sha512WithRSAEncryption_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha512WithRSAEncryption {
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
/// sha512WithRSAEncryption  OBJECT IDENTIFIER ::= { pkcs-1 13 }
/// ```
///
#[inline]
pub fn sha512WithRSAEncryption () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(pkcs_1(), 13).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pk-rsaES-OAEP PUBLIC-KEY ::= {
/// IDENTIFIER id-RSAES-OAEP
/// KEY RSAPublicKey
/// PARAMS TYPE RSAES-OAEP-params ARE optional
/// -- Private key format not in this module --
/// CERT-KEY-USAGE {keyEncipherment, dataEncipherment}
/// }
/// ```
///
///
pub fn pk_rsaES_OAEP() -> PUBLIC_KEY {
    PUBLIC_KEY {
        id: id_RSAES_OAEP(),                        /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_optional), /* OBJECT_FIELD_SETTING */
        keyUsage: Some(BIT_STRING::with_bits_set(&[
            KeyUsage_keyEncipherment,
            KeyUsage_dataEncipherment,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod pk_rsaES_OAEP {
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
    pub type Params = RSAES_OAEP_params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_RSAES_OAEP_params(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_RSAES_OAEP_params(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_RSAES_OAEP_params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kta-rsaES-OAEP KEY-TRANSPORT ::= {
/// IDENTIFIER id-RSAES-OAEP
/// PARAMS TYPE RSAES-OAEP-params ARE required
/// PUBLIC-KEYS { pk-rsa | pk-rsaES-OAEP }
/// SMIME-CAPS { TYPE RSAES-OAEP-params IDENTIFIED BY id-RSAES-OAEP}
/// }
/// ```
///
///
pub fn kta_rsaES_OAEP() -> KEY_TRANSPORT {
    KEY_TRANSPORT {
        id: id_RSAES_OAEP(),                        /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        PublicKeySet: Some(Vec::from([pk_rsa(), pk_rsaES_OAEP()])), /* OBJECT_FIELD_SETTING */
        smimeCaps: Some(kta_rsaES_OAEP_smimeCaps()), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kta_rsaES_OAEP {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = RSAES_OAEP_params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_RSAES_OAEP_params(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_RSAES_OAEP_params(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_RSAES_OAEP_params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkcs-1  OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) 1 }
/// ```
///
///
#[inline]
pub fn pkcs_1 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549,/* pkcs */ 1,1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-RSAES-OAEP  OBJECT IDENTIFIER ::= { pkcs-1 7 }
/// ```
///
///
#[inline]
pub fn id_RSAES_OAEP () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mgf1  OBJECT IDENTIFIER ::= { pkcs-1 8 }
/// ```
///
///
#[inline]
pub fn id_mgf1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pSpecified  OBJECT IDENTIFIER ::= { pkcs-1 9 }
/// ```
///
///
#[inline]
pub fn id_pSpecified () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-RSASSA-PSS  OBJECT IDENTIFIER ::= { pkcs-1 10 }
/// ```
///
///
#[inline]
pub fn id_RSASSA_PSS () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha224  OBJECT IDENTIFIER ::= { joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101)
/// csor(3) nistAlgorithms(4) hashalgs(2) 4 }
/// ```
///
///
#[inline]
pub fn id_sha224 () -> OBJECT_IDENTIFIER {
	oid!(/* joint-iso-itu-t */ 2,/* country */ 16,/* us */ 840,/* organization */ 1,/* gov */ 101,/* csor */ 3,/* nistAlgorithms */ 4,/* hashalgs */ 2,4) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-sha224 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-sha224
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_sha224() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha224(),                                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_sha224 {
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
/// id-sha256  OBJECT IDENTIFIER ::= { joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101)
/// csor(3) nistAlgorithms(4) hashalgs(2) 1 }
/// ```
///
///
pub fn id_sha256() -> OBJECT_IDENTIFIER {
    oid!(
        /* joint-iso-itu-t */ 2, /* country */ 16, /* us */ 840,
        /* organization */ 1, /* gov */ 101, /* csor */ 3,
        /* nistAlgorithms */ 4, /* hashalgs */ 2, 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-sha256 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-sha256
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_sha256() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha256(),                                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_sha256 {
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
/// id-sha384  OBJECT IDENTIFIER ::= { joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101)
/// csor(3) nistAlgorithms(4) hashalgs(2) 2 }
/// ```
///
///
pub fn id_sha384() -> OBJECT_IDENTIFIER {
    oid!(
        /* joint-iso-itu-t */ 2, /* country */ 16, /* us */ 840,
        /* organization */ 1, /* gov */ 101, /* csor */ 3,
        /* nistAlgorithms */ 4, /* hashalgs */ 2, 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-sha384 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-sha384
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_sha384() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha384(),                                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_sha384 {
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
/// id-sha512  OBJECT IDENTIFIER ::= { joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101)
/// csor(3) nistAlgorithms(4) hashalgs(2) 3 }
/// ```
///
///
pub fn id_sha512() -> OBJECT_IDENTIFIER {
    oid!(
        /* joint-iso-itu-t */ 2, /* country */ 16, /* us */ 840,
        /* organization */ 1, /* gov */ 101, /* csor */ 3,
        /* nistAlgorithms */ 4, /* hashalgs */ 2, 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mda-sha512 DIGEST-ALGORITHM ::= {
/// IDENTIFIER id-sha512
/// PARAMS TYPE NULL ARE preferredAbsent
/// }
/// ```
///
///
pub fn mda_sha512() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha512(),                                   /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredAbsent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mda_sha512 {
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
/// EncodingParameters  ::=  OCTET STRING(SIZE(0..MAX))
/// ```
pub type EncodingParameters = OCTET_STRING; // OctetStringType

pub fn _decode_EncodingParameters(el: &X690Element) -> ASN1Result<EncodingParameters> {
    BER.decode_octet_string(&el)
}

pub fn _encode_EncodingParameters(value_: &EncodingParameters) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_EncodingParameters(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nullOctetString  EncodingParameters ::= ''H
/// ```
///
///
pub const nullOctetString: EncodingParameters = Vec::<u8>::new();

// %FIXME%: COULD_NOT_COMPILE_ASSIGNMENT nullParameters COULD_NOT_COMPILE_VALUE

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashAlgorithm   ::=   AlgorithmIdentifier{DIGEST-ALGORITHM,{HashAlgorithms}}
/// ```
pub type HashAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_HashAlgorithm(el: &X690Element) -> ASN1Result<HashAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_HashAlgorithm(value_: &HashAlgorithm) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_HashAlgorithm(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashAlgorithms DIGEST-ALGORITHM ::= {
/// { IDENTIFIER id-sha1 PARAMS TYPE NULL ARE preferredPresent } |
/// { IDENTIFIER id-sha224 PARAMS TYPE NULL ARE preferredPresent } |
/// { IDENTIFIER id-sha256 PARAMS TYPE NULL ARE preferredPresent } |
/// { IDENTIFIER id-sha384 PARAMS TYPE NULL ARE preferredPresent } |
/// { IDENTIFIER id-sha512 PARAMS TYPE NULL ARE preferredPresent }
/// }
/// ```
///
///
pub fn HashAlgorithms() -> Vec<DIGEST_ALGORITHM> {
    [
        HashAlgorithms_Union0_Intersection0_Element(),
        HashAlgorithms_Union1_Intersection0_Element(),
        HashAlgorithms_Union2_Intersection0_Element(),
        HashAlgorithms_Union3_Intersection0_Element(),
        HashAlgorithms_Union4_Intersection0_Element(),
    ]
    .into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha1Identifier HashAlgorithm ::= {
/// algorithm id-sha1,
/// parameters NULL : NULL
/// }
/// ```
///
///
pub fn sha1Identifier() -> HashAlgorithm {
    HashAlgorithm {
        algorithm: id_sha1(),
        parameters: Some(BER.encode_null(&()).unwrap()),
        _unrecognized: vec![],
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MaskGenAlgorithm  ::=  AlgorithmIdentifier{ALGORITHM,{PKCS1MGFAlgorithms}}
/// ```
pub type MaskGenAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_MaskGenAlgorithm(el: &X690Element) -> ASN1Result<MaskGenAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_MaskGenAlgorithm(value_: &MaskGenAlgorithm) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_MaskGenAlgorithm(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mgf1SHA1 MaskGenAlgorithm ::= {
/// algorithm id-mgf1,
/// parameters HashAlgorithm : sha1Identifier
/// }
/// ```
///
///
// pub const mgf1SHA1: MaskGenAlgorithm = MaskGenAlgorithm {
//     algorithm: id_mgf1(),
//     parameters: Some(_encode_AlgorithmIdentifier(&sha1Identifier()).unwrap()),
//     _unrecognized: Vec::new(),
// };
pub fn mgf1SHA1() -> MaskGenAlgorithm {
    AlgorithmIdentifier {
        algorithm: id_mgf1(),
        parameters: Some(_encode_AlgorithmIdentifier(&sha1Identifier()).unwrap()),
        _unrecognized: vec![],
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKCS1MGFAlgorithms ALGORITHM ::= {
/// { IDENTIFIER id-mgf1 PARAMS TYPE HashAlgorithm ARE required },
/// ...
/// }
/// ```
///
///
pub fn PKCS1MGFAlgorithms() -> Vec<ALGORITHM> {
    [PKCS1MGFAlgorithms_Union0_Intersection0_Element()].into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PSourceAlgorithm  ::=  AlgorithmIdentifier{ALGORITHM,{PSS-SourceAlgorithms}}
/// ```
pub type PSourceAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_PSourceAlgorithm(el: &X690Element) -> ASN1Result<PSourceAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_PSourceAlgorithm(value_: &PSourceAlgorithm) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_PSourceAlgorithm(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PSS-SourceAlgorithms ALGORITHM ::= {
/// { IDENTIFIER id-pSpecified PARAMS TYPE EncodingParameters
/// ARE required },
/// ...
/// }
/// ```
///
///
pub fn PSS_SourceAlgorithms() -> Vec<ALGORITHM> {
    [PSS_SourceAlgorithms_Union0_Intersection0_Element()].into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pSpecifiedEmpty PSourceAlgorithm ::= {
/// algorithm	id-pSpecified,
/// parameters	EncodingParameters : nullOctetString
/// }
/// ```
///
///
pub fn pSpecifiedEmpty() -> PSourceAlgorithm {
    AlgorithmIdentifier {
        algorithm: id_pSpecified(),
        parameters: Some(BER.encode_octet_string(&nullOctetString).unwrap()),
        _unrecognized: vec![],
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RSASSA-PSS-params ::= SEQUENCE  {
/// hashAlgorithm	[0] HashAlgorithm DEFAULT sha1Identifier,
/// maskGenAlgorithm	[1] MaskGenAlgorithm DEFAULT mgf1SHA1,
/// saltLength		[2] INTEGER DEFAULT 20,
/// trailerField	[3] INTEGER DEFAULT 1
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct RSASSA_PSS_params {
    pub hashAlgorithm: OPTIONAL<HashAlgorithm>,
    pub maskGenAlgorithm: OPTIONAL<MaskGenAlgorithm>,
    pub saltLength: OPTIONAL<INTEGER>,
    pub trailerField: OPTIONAL<INTEGER>,
}
impl RSASSA_PSS_params {
    pub fn new(
        hashAlgorithm: OPTIONAL<HashAlgorithm>,
        maskGenAlgorithm: OPTIONAL<MaskGenAlgorithm>,
        saltLength: OPTIONAL<INTEGER>,
        trailerField: OPTIONAL<INTEGER>,
    ) -> Self {
        RSASSA_PSS_params {
            hashAlgorithm,
            maskGenAlgorithm,
            saltLength,
            trailerField,
        }
    }
    pub fn _default_value_for_hashAlgorithm() -> HashAlgorithm {
        sha1Identifier()
    }
    pub fn _default_value_for_maskGenAlgorithm() -> MaskGenAlgorithm {
        mgf1SHA1()
    }
    pub fn _default_value_for_saltLength() -> INTEGER {
        Vec::from([20])
    }
    pub fn _default_value_for_trailerField() -> INTEGER {
        Vec::from([1])
    }
}
impl Default for RSASSA_PSS_params {
    fn default() -> Self {
        RSASSA_PSS_params {
            hashAlgorithm: None,
            maskGenAlgorithm: None,
            saltLength: None,
            trailerField: None,
        }
    }
}
impl TryFrom<&X690Element> for RSASSA_PSS_params {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RSASSA_PSS_params(el)
    }
}

pub const _rctl1_components_for_RSASSA_PSS_params: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "hashAlgorithm",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maskGenAlgorithm",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "saltLength",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "trailerField",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RSASSA_PSS_params: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RSASSA_PSS_params: &[ComponentSpec; 0] = &[];

pub fn _decode_RSASSA_PSS_params(el: &X690Element) -> ASN1Result<RSASSA_PSS_params> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSASSA-PSS-params")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RSASSA_PSS_params,
        _eal_components_for_RSASSA_PSS_params,
        _rctl2_components_for_RSASSA_PSS_params,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut hashAlgorithm_: OPTIONAL<HashAlgorithm> = None;
    let mut maskGenAlgorithm_: OPTIONAL<MaskGenAlgorithm> = None;
    let mut saltLength_: OPTIONAL<INTEGER> = None;
    let mut trailerField_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashAlgorithm" => {
                hashAlgorithm_ = Some(|el: &X690Element| -> ASN1Result<HashAlgorithm> {
                    Ok(_decode_HashAlgorithm(&el.inner()?)?)
                }(_el)?)
            }
            "maskGenAlgorithm" => {
                maskGenAlgorithm_ = Some(|el: &X690Element| -> ASN1Result<MaskGenAlgorithm> {
                    Ok(_decode_MaskGenAlgorithm(&el.inner()?)?)
                }(_el)?)
            }
            "saltLength" => {
                saltLength_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "trailerField" => {
                trailerField_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSASSA-PSS-params")
                )
            }
        }
    }
    Ok(RSASSA_PSS_params {
        hashAlgorithm: hashAlgorithm_,
        maskGenAlgorithm: maskGenAlgorithm_,
        saltLength: saltLength_,
        trailerField: trailerField_,
    })
}

pub fn _encode_RSASSA_PSS_params(value_: &RSASSA_PSS_params) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    if let Some(v_) = &value_.hashAlgorithm {
        if v_.algorithm != RSASSA_PSS_params::_default_value_for_hashAlgorithm().algorithm {
            components_.push(|v_1: &HashAlgorithm| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_HashAlgorithm(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.maskGenAlgorithm {
        if v_.algorithm != RSASSA_PSS_params::_default_value_for_maskGenAlgorithm().algorithm {
            components_.push(|v_1: &MaskGenAlgorithm| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_MaskGenAlgorithm(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.saltLength {
        if *v_ != RSASSA_PSS_params::_default_value_for_saltLength() {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&BER.encode_integer(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.trailerField {
        if *v_ != RSASSA_PSS_params::_default_value_for_trailerField() {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 3),
                    X690Value::from_explicit(&BER.encode_integer(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_RSASSA_PSS_params(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSASSA-PSS-params")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RSASSA_PSS_params,
        _eal_components_for_RSASSA_PSS_params,
        _rctl2_components_for_RSASSA_PSS_params,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashAlgorithm" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "hashAlgorithm")
                    );
                }
                Ok(_validate_HashAlgorithm(&el.inner()?)?)
            }(_el)?,
            "maskGenAlgorithm" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "maskGenAlgorithm",
                    ));
                }
                Ok(_validate_MaskGenAlgorithm(&el.inner()?)?)
            }(_el)?,
            "saltLength" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "saltLength")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "trailerField" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "trailerField")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSASSA-PSS-params")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RSAES-OAEP-params ::= SEQUENCE  {
/// hashFunc		[0] HashAlgorithm DEFAULT sha1Identifier,
/// maskGenFunc		[1] MaskGenAlgorithm DEFAULT mgf1SHA1,
/// pSourceFunc		[2] PSourceAlgorithm DEFAULT
/// pSpecifiedEmpty
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct RSAES_OAEP_params {
    pub hashFunc: OPTIONAL<HashAlgorithm>,
    pub maskGenFunc: OPTIONAL<MaskGenAlgorithm>,
    pub pSourceFunc: OPTIONAL<PSourceAlgorithm>,
}
impl RSAES_OAEP_params {
    pub fn new(
        hashFunc: OPTIONAL<HashAlgorithm>,
        maskGenFunc: OPTIONAL<MaskGenAlgorithm>,
        pSourceFunc: OPTIONAL<PSourceAlgorithm>,
    ) -> Self {
        RSAES_OAEP_params {
            hashFunc,
            maskGenFunc,
            pSourceFunc,
        }
    }
    pub fn _default_value_for_hashFunc() -> HashAlgorithm {
        sha1Identifier()
    }
    pub fn _default_value_for_maskGenFunc() -> MaskGenAlgorithm {
        mgf1SHA1()
    }
    pub fn _default_value_for_pSourceFunc() -> PSourceAlgorithm {
        pSpecifiedEmpty()
    }
}
impl Default for RSAES_OAEP_params {
    fn default() -> Self {
        RSAES_OAEP_params {
            hashFunc: None,
            maskGenFunc: None,
            pSourceFunc: None,
        }
    }
}
impl TryFrom<&X690Element> for RSAES_OAEP_params {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RSAES_OAEP_params(el)
    }
}

pub const _rctl1_components_for_RSAES_OAEP_params: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "hashFunc",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maskGenFunc",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pSourceFunc",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RSAES_OAEP_params: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RSAES_OAEP_params: &[ComponentSpec; 0] = &[];

pub fn _decode_RSAES_OAEP_params(el: &X690Element) -> ASN1Result<RSAES_OAEP_params> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAES-OAEP-params")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RSAES_OAEP_params,
        _eal_components_for_RSAES_OAEP_params,
        _rctl2_components_for_RSAES_OAEP_params,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut hashFunc_: OPTIONAL<HashAlgorithm> = None;
    let mut maskGenFunc_: OPTIONAL<MaskGenAlgorithm> = None;
    let mut pSourceFunc_: OPTIONAL<PSourceAlgorithm> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashFunc" => {
                hashFunc_ = Some(|el: &X690Element| -> ASN1Result<HashAlgorithm> {
                    Ok(_decode_HashAlgorithm(&el.inner()?)?)
                }(_el)?)
            }
            "maskGenFunc" => {
                maskGenFunc_ = Some(|el: &X690Element| -> ASN1Result<MaskGenAlgorithm> {
                    Ok(_decode_MaskGenAlgorithm(&el.inner()?)?)
                }(_el)?)
            }
            "pSourceFunc" => {
                pSourceFunc_ = Some(|el: &X690Element| -> ASN1Result<PSourceAlgorithm> {
                    Ok(_decode_PSourceAlgorithm(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAES-OAEP-params")
                )
            }
        }
    }
    Ok(RSAES_OAEP_params {
        hashFunc: hashFunc_,
        maskGenFunc: maskGenFunc_,
        pSourceFunc: pSourceFunc_,
    })
}

pub fn _encode_RSAES_OAEP_params(value_: &RSAES_OAEP_params) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.hashFunc {
        if v_.algorithm != RSAES_OAEP_params::_default_value_for_hashFunc().algorithm {
            components_.push(|v_1: &HashAlgorithm| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_HashAlgorithm(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.maskGenFunc {
        if v_.algorithm != RSAES_OAEP_params::_default_value_for_maskGenFunc().algorithm {
            components_.push(|v_1: &MaskGenAlgorithm| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_MaskGenAlgorithm(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.pSourceFunc {
        if v_.algorithm != RSAES_OAEP_params::_default_value_for_pSourceFunc().algorithm {
            components_.push(|v_1: &PSourceAlgorithm| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&_encode_PSourceAlgorithm(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_RSAES_OAEP_params(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAES-OAEP-params")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RSAES_OAEP_params,
        _eal_components_for_RSAES_OAEP_params,
        _rctl2_components_for_RSAES_OAEP_params,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashFunc" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "hashFunc")
                    );
                }
                Ok(_validate_HashAlgorithm(&el.inner()?)?)
            }(_el)?,
            "maskGenFunc" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maskGenFunc")
                    );
                }
                Ok(_validate_MaskGenAlgorithm(&el.inner()?)?)
            }(_el)?,
            "pSourceFunc" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pSourceFunc")
                    );
                }
                Ok(_validate_PSourceAlgorithm(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RSAES-OAEP-params")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-rsaSSA-PSS-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_rsaSSA_PSS_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_RSASSA_PSS(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_rsaSSA_PSS_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha224WithRSAEncryption-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_sha224WithRSAEncryption_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: sha224WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha224WithRSAEncryption_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha256WithRSAEncryption-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_sha256WithRSAEncryption_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: sha256WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha256WithRSAEncryption_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha384WithRSAEncryption-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_sha384WithRSAEncryption_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: sha384WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha384WithRSAEncryption_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sa-sha512WithRSAEncryption-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn sa_sha512WithRSAEncryption_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: sha512WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sa_sha512WithRSAEncryption_smimeCaps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// kta-rsaES-OAEP-smimeCaps ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn kta_rsaES_OAEP_smimeCaps() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_RSAES_OAEP(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod kta_rsaES_OAEP_smimeCaps {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = RSAES_OAEP_params; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_RSAES_OAEP_params(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_RSAES_OAEP_params(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_RSAES_OAEP_params(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashAlgorithms-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn HashAlgorithms_Union0_Intersection0_Element() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha1(),                                      /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredPresent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod HashAlgorithms_Union0_Intersection0_Element {
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
/// HashAlgorithms-Union1-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn HashAlgorithms_Union1_Intersection0_Element() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha224(),                                    /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredPresent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod HashAlgorithms_Union1_Intersection0_Element {
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
/// HashAlgorithms-Union2-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn HashAlgorithms_Union2_Intersection0_Element() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha256(),                                    /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredPresent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod HashAlgorithms_Union2_Intersection0_Element {
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
/// HashAlgorithms-Union3-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn HashAlgorithms_Union3_Intersection0_Element() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha384(),                                    /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredPresent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod HashAlgorithms_Union3_Intersection0_Element {
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
/// HashAlgorithms-Union4-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn HashAlgorithms_Union4_Intersection0_Element() -> DIGEST_ALGORITHM {
    DIGEST_ALGORITHM {
        id: id_sha512(),                                    /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_preferredPresent), /* OBJECT_FIELD_SETTING */
    }
}

pub mod HashAlgorithms_Union4_Intersection0_Element {
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
/// PKCS1MGFAlgorithms-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn PKCS1MGFAlgorithms_Union0_Intersection0_Element() -> ALGORITHM {
    ALGORITHM {
        id: id_mgf1(),                              /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: None,
    }
}

pub mod PKCS1MGFAlgorithms_Union0_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = HashAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_HashAlgorithm(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_HashAlgorithm(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_HashAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PSS-SourceAlgorithms-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn PSS_SourceAlgorithms_Union0_Intersection0_Element() -> ALGORITHM {
    ALGORITHM {
        id: id_pSpecified(),                        /* OBJECT_FIELD_SETTING */
        paramPresence: Some(ParamOptions_required), /* OBJECT_FIELD_SETTING */
        smimeCaps: None,
    }
}

pub mod PSS_SourceAlgorithms_Union0_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Params = EncodingParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Params(el: &X690Element) -> ASN1Result<Params> {
        _decode_EncodingParameters(el)
    }
    pub fn _encode_Params(value_: &Params) -> ASN1Result<X690Element> {
        _encode_EncodingParameters(value_)
    }
    pub fn _validate_Params(el: &X690Element) -> ASN1Result<()> {
        _validate_EncodingParameters(el)
    }
}
