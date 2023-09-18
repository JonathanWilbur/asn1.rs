#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # AlgorithmObjectIdentifiers
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `AlgorithmObjectIdentifiers`.
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
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ID  ::=  OBJECT IDENTIFIER
/// ```
pub type ID = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_ID(el: &X690Element) -> ASN1Result<ID> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_ID(value_: &ID) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_ID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nullAlgorithm           ID ::= {algorithm 0}
/// ```
///
///
pub fn nullAlgorithm() -> ID {
    OBJECT_IDENTIFIER([algorithm().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// encryptionAlgorithm     ID ::= {algorithm 1}
/// ```
///
///
pub fn encryptionAlgorithm() -> ID {
    OBJECT_IDENTIFIER([algorithm().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hashAlgorithm           ID ::= {algorithm 2}
/// ```
///
///
pub fn hashAlgorithm() -> ID {
    OBJECT_IDENTIFIER([algorithm().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signatureAlgorithm      ID ::= {algorithm 3}
/// ```
///
///
pub fn signatureAlgorithm() -> ID {
    OBJECT_IDENTIFIER([algorithm().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ea                   ID ::= encryptionAlgorithm
/// ```
///
///
pub fn id_ea() -> ID {
    encryptionAlgorithm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ha                   ID ::= hashAlgorithm
/// ```
///
///
pub fn id_ha() -> ID {
    hashAlgorithm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sa                   ID ::= signatureAlgorithm
/// ```
///
///
pub fn id_sa() -> ID {
    signatureAlgorithm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ea-rsa               ID ::= {id-ea 1}
/// ```
///
///
pub fn id_ea_rsa() -> ID {
    OBJECT_IDENTIFIER([id_ea().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ha-sqMod-n           ID ::= {id-ha 1}
/// ```
///
///
pub fn id_ha_sqMod_n() -> ID {
    OBJECT_IDENTIFIER([id_ha().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sa-sqMod-nWithRSA    ID ::= {id-sa 1}
/// ```
///
///
pub fn id_sa_sqMod_nWithRSA() -> ID {
    OBJECT_IDENTIFIER([id_sa().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// us-iso                  ID ::= { iso(1) member-body(2) us(840) }
/// ```
///
///
pub fn us_iso() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ansi-x9-57              ID ::= { us-iso ansi-x9-57(10040) }
/// ```
///
///
pub fn ansi_x9_57() -> ID {
    OBJECT_IDENTIFIER([us_iso().0, Vec::<u32>::from([/* ansi-x9-57 */ 10040])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ansi-x9-62              ID ::= { us-iso ansi-x962(10045) }
/// ```
///
///
pub fn ansi_x9_62() -> ID {
    OBJECT_IDENTIFIER([us_iso().0, Vec::<u32>::from([/* ansi-x962 */ 10045])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ansi-x9-42              ID ::= { us-iso ansi-x942(10046) }
/// ```
///
///
pub fn ansi_x9_42() -> ID {
    OBJECT_IDENTIFIER([us_iso().0, Vec::<u32>::from([/* ansi-x942 */ 10046])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iso-standard            ID ::= { iso(1) standard(0) }
/// ```
///
///
pub fn iso_standard() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([/* iso */ 1, /* standard */ 0])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iso9797                 ID ::= { iso-standard message-authentication-codes(9797) }
/// ```
///
///
pub fn iso9797() -> ID {
    OBJECT_IDENTIFIER(
        [
            iso_standard().0,
            Vec::<u32>::from([/* message-authentication-codes */ 9797]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iso-organization        ID ::= { iso(1) identified-organization(3) }
/// ```
///
///
pub fn iso_organization() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certicom                ID ::= { iso-organization certicom(132) }
/// ```
///
///
pub fn certicom() -> ID {
    OBJECT_IDENTIFIER([iso_organization().0, Vec::<u32>::from([/* certicom */ 132])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certicom-curve          ID ::= { certicom curve(0) }
/// ```
///
///
pub fn certicom_curve() -> ID {
    OBJECT_IDENTIFIER([certicom().0, Vec::<u32>::from([/* curve */ 0])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// teletrust               ID ::= { iso-organization teletrust(36) }
/// ```
///
///
pub fn teletrust() -> ID {
    OBJECT_IDENTIFIER([iso_organization().0, Vec::<u32>::from([/* teletrust */ 36])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecStdCurvesAndGen       ID ::= { teletrust algorithm(3) signature-algorithm(3) ecSign(2) 8}
/// ```
///
///
pub fn ecStdCurvesAndGen() -> ID {
    OBJECT_IDENTIFIER(
        [
            teletrust().0,
            Vec::<u32>::from([
                /* algorithm */ 3, /* signature-algorithm */ 3, /* ecSign */ 2, 8,
            ]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// versionOne              ID ::= { ecStdCurvesAndGen ellipticCurve(1) versionOne(1) }
/// ```
///
///
pub fn versionOne() -> ID {
    OBJECT_IDENTIFIER(
        [
            ecStdCurvesAndGen().0,
            Vec::<u32>::from([/* ellipticCurve */ 1, /* versionOne */ 1]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// us-joint                ID ::= { joint-iso-itu-t(2) country(16) us(840) }
/// ```
///
///
pub fn us_joint() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* joint-iso-itu-t */ 2, /* country */ 16, /* us */ 840,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// usgov                   ID ::= { us-joint organization(1) gov(101) }
/// ```
///
///
pub fn usgov() -> ID {
    OBJECT_IDENTIFIER(
        [
            us_joint().0,
            Vec::<u32>::from([/* organization */ 1, /* gov */ 101]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dodAlgorithms           ID ::= { usgov dod(2) infosec(1) algorithms(1) }
/// ```
///
///
pub fn dodAlgorithms() -> ID {
    OBJECT_IDENTIFIER(
        [
            usgov().0,
            Vec::<u32>::from([
                /* dod */ 2, /* infosec */ 1, /* algorithms */ 1,
            ]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// csor                    ID ::= { usgov csor(3) }
/// ```
///
///
pub fn csor() -> ID {
    OBJECT_IDENTIFIER([usgov().0, Vec::<u32>::from([/* csor */ 3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nistAlgorithms          ID ::= { csor nistAlgorithm(4) }
/// ```
///
///
pub fn nistAlgorithms() -> ID {
    OBJECT_IDENTIFIER([csor().0, Vec::<u32>::from([/* nistAlgorithm */ 4])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aes                     ID ::= { nistAlgorithms 1 }
/// ```
///
///
pub fn aes() -> ID {
    OBJECT_IDENTIFIER([nistAlgorithms().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hashAlgs                ID ::= { nistAlgorithms hashalgs(2) }
/// ```
///
///
pub fn hashAlgs() -> ID {
    OBJECT_IDENTIFIER([nistAlgorithms().0, Vec::<u32>::from([/* hashalgs */ 2])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sigAlgs                 ID ::= { nistAlgorithms 3 }
/// ```
///
///
pub fn sigAlgs() -> ID {
    OBJECT_IDENTIFIER([nistAlgorithms().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rsadsi                  ID ::= { iso(1) member-body(2) us(840) rsadsi(113549) }
/// ```
///
///
pub fn rsadsi() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkcs-1                  ID ::= { rsadsi pkcs(1) pkcs-1(1) }
/// ```
///
///
pub fn pkcs_1() -> ID {
    OBJECT_IDENTIFIER([rsadsi().0, Vec::<u32>::from([/* pkcs */ 1, /* pkcs-1 */ 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// digestAlgorithm         ID ::= { rsadsi digestAlgorithm(2) }
/// ```
///
///
pub fn digestAlgorithm() -> ID {
    OBJECT_IDENTIFIER([rsadsi().0, Vec::<u32>::from([/* digestAlgorithm */ 2])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes128-CBC           ID ::= { aes 2 }
/// ```
///
///
pub fn id_aes128_CBC() -> ID {
    OBJECT_IDENTIFIER([aes().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes192-CBC           ID ::= { aes 22 }
/// ```
///
///
pub fn id_aes192_CBC() -> ID {
    OBJECT_IDENTIFIER([aes().0, Vec::<u32>::from([22])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes256-CBC           ID ::= { aes 42 }
/// ```
///
///
pub fn id_aes256_CBC() -> ID {
    OBJECT_IDENTIFIER([aes().0, Vec::<u32>::from([42])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes128-wrap          ID ::= { aes 5 }
/// ```
///
///
pub fn id_aes128_wrap() -> ID {
    OBJECT_IDENTIFIER([aes().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes192-wrap          ID ::= { aes 25 }
/// ```
///
///
pub fn id_aes192_wrap() -> ID {
    OBJECT_IDENTIFIER([aes().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes256-wrap          ID ::= { aes 45 }
/// ```
///
///
pub fn id_aes256_wrap() -> ID {
    OBJECT_IDENTIFIER([aes().0, Vec::<u32>::from([45])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rsaEncryption           ID ::= { pkcs-1 rsaEncryption(1)}
/// ```
///
///
pub fn rsaEncryption() -> ID {
    OBJECT_IDENTIFIER([pkcs_1().0, Vec::<u32>::from([/* rsaEncryption */ 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-keyExchangeAlgorithm ID ::= { dodAlgorithms id-keyExchangeAlgorithm(22)}
/// ```
///
///
pub fn id_keyExchangeAlgorithm() -> ID {
    OBJECT_IDENTIFIER(
        [
            dodAlgorithms().0,
            Vec::<u32>::from([/* id-keyExchangeAlgorithm */ 22]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa                  ID ::= { ansi-x9-57 x9algorithm(4) 1 }
/// ```
///
///
pub fn id_dsa() -> ID {
    OBJECT_IDENTIFIER([ansi_x9_57().0, Vec::<u32>::from([/* x9algorithm */ 4, 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecPublicKey          ID ::= { ansi-x9-62 keyType(2) 1 }
/// ```
///
///
pub fn id_ecPublicKey() -> ID {
    OBJECT_IDENTIFIER([ansi_x9_62().0, Vec::<u32>::from([/* keyType */ 2, 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecDH                 ID ::= { certicom schemes(1) ecdh(12) }
/// ```
///
///
pub fn id_ecDH() -> ID {
    OBJECT_IDENTIFIER(
        [
            certicom().0,
            Vec::<u32>::from([/* schemes */ 1, /* ecdh */ 12]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecMQV                ID ::= { certicom schemes(1) ecmqv(13) }
/// ```
///
///
pub fn id_ecMQV() -> ID {
    OBJECT_IDENTIFIER(
        [
            certicom().0,
            Vec::<u32>::from([/* schemes */ 1, /* ecmqv */ 13]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dh-public-number        ID ::= { ansi-x9-42 number-type(2) dh-public-number(1) }
/// ```
///
///
pub fn dh_public_number() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_42().0,
            Vec::<u32>::from([/* number-type */ 2, /* dh-public-number */ 1]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha1                 ID ::= {iso(1) identified-organization(3) oiw(14) secsig(3)
///                                 algorithms(2) 26}
/// ```
///
///
pub fn id_sha1() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* oiw */ 14,
        /* secsig */ 3, /* algorithms */ 2, 26,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha256               ID ::= { hashAlgs 1 }
/// ```
///
///
pub fn id_sha256() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha384               ID ::= { hashAlgs 2 }
/// ```
///
///
pub fn id_sha384() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha512               ID ::= { hashAlgs 3 }
/// ```
///
///
pub fn id_sha512() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha224               ID ::= { hashAlgs 4 }
/// ```
///
///
pub fn id_sha224() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha512-224           ID ::= { hashAlgs 5 }
/// ```
///
///
pub fn id_sha512_224() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha512-256           ID ::= { hashAlgs 6 }
/// ```
///
///
pub fn id_sha512_256() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-224             ID ::= { hashAlgs 7 }
/// ```
///
///
pub fn id_sha3_224() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-256             ID ::= { hashAlgs 8 }
/// ```
///
///
pub fn id_sha3_256() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-384             ID ::= { hashAlgs 9 }
/// ```
///
///
pub fn id_sha3_384() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-512             ID ::= { hashAlgs 10 }
/// ```
///
///
pub fn id_sha3_512() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake128             ID ::= { hashAlgs 11 }
/// ```
///
///
pub fn id_shake128() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake256             ID ::= { hashAlgs 12 }
/// ```
///
///
pub fn id_shake256() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake128-len         ID ::= { hashAlgs 17 }
/// ```
///
///
pub fn id_shake128_len() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake256-len         ID ::= { hashAlgs 18 }
/// ```
///
///
pub fn id_shake256_len() -> ID {
    OBJECT_IDENTIFIER([hashAlgs().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hashAlg                 ID ::= {  iso(1) identified-organization(3) dod(6) internet(1)
///                                   private(4) enterprise(1) kudelski(1722)
///                                   cryptography(12) 2 }
/// ```
///
///
pub fn hashAlg() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1, /* private */ 4, /* enterprise */ 1,
        /* kudelski */ 1722, /* cryptography */ 12, 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha1WithRSAEncryption   ID ::= { pkcs-1 sha1WithRSAEncryption(5) }
/// ```
///
///
pub fn sha1WithRSAEncryption() -> ID {
    OBJECT_IDENTIFIER(
        [
            pkcs_1().0,
            Vec::<u32>::from([/* sha1WithRSAEncryption */ 5]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha256WithRSAEncryption ID ::= { pkcs-1 sha256WithRSAEncryption(11) }
/// ```
///
///
pub fn sha256WithRSAEncryption() -> ID {
    OBJECT_IDENTIFIER(
        [
            pkcs_1().0,
            Vec::<u32>::from([/* sha256WithRSAEncryption */ 11]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha384WithRSAEncryption ID ::= { pkcs-1 sha384WithRSAEncryption(12) }
/// ```
///
///
pub fn sha384WithRSAEncryption() -> ID {
    OBJECT_IDENTIFIER(
        [
            pkcs_1().0,
            Vec::<u32>::from([/* sha384WithRSAEncryption */ 12]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512WithRSAEncryption ID ::= { pkcs-1 sha512WithRSAEncryption(13) }
/// ```
///
///
pub fn sha512WithRSAEncryption() -> ID {
    OBJECT_IDENTIFIER(
        [
            pkcs_1().0,
            Vec::<u32>::from([/* sha512WithRSAEncryption */ 13]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha224WithRSAEncryption ID ::= { pkcs-1 sha224WithRSAEncryption(14) }
/// ```
///
///
pub fn sha224WithRSAEncryption() -> ID {
    OBJECT_IDENTIFIER(
        [
            pkcs_1().0,
            Vec::<u32>::from([/* sha224WithRSAEncryption */ 14]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-RSASSA-PSS           ID ::= { pkcs-1 10 }
/// ```
///
///
pub fn id_RSASSA_PSS() -> ID {
    OBJECT_IDENTIFIER([pkcs_1().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mgf1                 ID ::= { pkcs-1 8 }
/// ```
///
///
pub fn id_mgf1() -> ID {
    OBJECT_IDENTIFIER([pkcs_1().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha1        ID ::= {iso(1) member-body(2) us(840) x9-57(10040) x9algorithm(4)
///                                 dsa-with-sha1(3)}
/// ```
///
///
pub fn id_dsa_with_sha1() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* x9-57 */ 10040,
        /* x9algorithm */ 4, /* dsa-with-sha1 */ 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha224      ID ::= { sigAlgs 1 }
/// ```
///
///
pub fn id_dsa_with_sha224() -> ID {
    OBJECT_IDENTIFIER([sigAlgs().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha256      ID ::= { sigAlgs 2 }
/// ```
///
///
pub fn id_dsa_with_sha256() -> ID {
    OBJECT_IDENTIFIER([sigAlgs().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA224       ID ::= { ansi-x9-62 signatures(4)
///                                                 ecdsa-with-SHA2(3) 1 }
/// ```
///
///
pub fn ecdsa_with_SHA224() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_62().0,
            Vec::<u32>::from([/* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 1]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA256       ID ::= { ansi-x9-62 signatures(4)
///                                                 ecdsa-with-SHA2(3) 2 }
/// ```
///
///
pub fn ecdsa_with_SHA256() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_62().0,
            Vec::<u32>::from([/* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 2]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA384       ID ::= { ansi-x9-62 signatures(4)
///                                                 ecdsa-with-SHA2(3) 3 }
/// ```
///
///
pub fn ecdsa_with_SHA384() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_62().0,
            Vec::<u32>::from([/* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 3]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA512       ID ::= { ansi-x9-62 signatures(4) ecdsa-with-SHA2(3) 4 }
/// ```
///
///
pub fn ecdsa_with_SHA512() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_62().0,
            Vec::<u32>::from([/* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 4]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp192r1       ID ::= { ansi-x9-62 curves(3) prime(1) 1 }
/// ```
///
///
pub fn secp192r1() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_62().0,
            Vec::<u32>::from([/* curves */ 3, /* prime */ 1, 1]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect163k1       ID ::= { certicom-curve 1 }
/// ```
///
///
pub fn sect163k1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect163r2       ID ::= { certicom-curve 15 }
/// ```
///
///
pub fn sect163r2() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp224r1       ID ::= { certicom-curve 33 }
/// ```
///
///
pub fn secp224r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([33])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect233k1       ID ::= { certicom-curve 26 }
/// ```
///
///
pub fn sect233k1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect233r1       ID ::= { certicom-curve 27 }
/// ```
///
///
pub fn sect233r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp256r1       ID ::= { ansi-x9-62 curves(3) prime(1) 7 }
/// ```
///
///
pub fn secp256r1() -> ID {
    OBJECT_IDENTIFIER(
        [
            ansi_x9_62().0,
            Vec::<u32>::from([/* curves */ 3, /* prime */ 1, 7]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect283k1       ID ::= { certicom-curve 16 }
/// ```
///
///
pub fn sect283k1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect283r1       ID ::= { certicom-curve 17 }
/// ```
///
///
pub fn sect283r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp384r1       ID ::= { certicom-curve 34 }
/// ```
///
///
pub fn secp384r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([34])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect409k1       ID ::= { certicom-curve 36 }
/// ```
///
///
pub fn sect409k1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([36])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect409r1       ID ::= { certicom-curve 37 }
/// ```
///
///
pub fn sect409r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([37])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp521r1       ID ::= { certicom-curve 35 }
/// ```
///
///
pub fn secp521r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([35])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect571k1       ID ::= { certicom-curve 38 }
/// ```
///
///
pub fn sect571k1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([38])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect571r1       ID ::= { certicom-curve 39 }
/// ```
///
///
pub fn sect571r1() -> ID {
    OBJECT_IDENTIFIER([certicom_curve().0, Vec::<u32>::from([39])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP160r1 ID ::= { versionOne 1 }
/// ```
///
///
pub fn brainpoolP160r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP160t1 ID ::= { versionOne 2 }
/// ```
///
///
pub fn brainpoolP160t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP192r1 ID ::= { versionOne 3 }
/// ```
///
///
pub fn brainpoolP192r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP192t1 ID ::= { versionOne 4 }
/// ```
///
///
pub fn brainpoolP192t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP224r1 ID ::= { versionOne 5 }
/// ```
///
///
pub fn brainpoolP224r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP224t1 ID ::= { versionOne 6 }
/// ```
///
///
pub fn brainpoolP224t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP256r1 ID ::= { versionOne 7 }
/// ```
///
///
pub fn brainpoolP256r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP256t1 ID ::= { versionOne 8 }
/// ```
///
///
pub fn brainpoolP256t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP320r1 ID ::= { versionOne 9 }
/// ```
///
///
pub fn brainpoolP320r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP320t1 ID ::= { versionOne 10 }
/// ```
///
///
pub fn brainpoolP320t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP384r1 ID ::= { versionOne 11 }
/// ```
///
///
pub fn brainpoolP384r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP384t1 ID ::= { versionOne 12 }
/// ```
///
///
pub fn brainpoolP384t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP512r1 ID ::= { versionOne 13 }
/// ```
///
///
pub fn brainpoolP512r1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP512t1 ID ::= { versionOne 14 }
/// ```
///
///
pub fn brainpoolP512t1() -> ID {
    OBJECT_IDENTIFIER([versionOne().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// X509Curves OBJECT IDENTIFIER ::= { secp192r1 | sect163k1 | sect163r2 | secp224r1 | sect233k1 |
///                                    sect233r1 | secp256r1 | sect283k1 | sect283r1 | secp384r1 |
///                                    sect409k1 | sect409r1 | secp521r1 | sect571k1 | sect571r1 }
/// ```
///
///
pub type X509Curves = OBJECT_IDENTIFIER; // VALUE_SET_TYPE

pub fn _decode_X509Curves(el: &X690Element) -> ASN1Result<X509Curves> {
    BER.decode_object_identifier(el)
}

pub fn _encode_X509Curves(value_: &X509Curves) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(value_)
}

pub fn _validate_X509Curves(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA224       ID ::= { digestAlgorithm 8 }
/// ```
///
///
pub fn id_hmacWithSHA224() -> ID {
    OBJECT_IDENTIFIER([digestAlgorithm().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA256       ID ::= { digestAlgorithm 9 }
/// ```
///
///
pub fn id_hmacWithSHA256() -> ID {
    OBJECT_IDENTIFIER([digestAlgorithm().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA384       ID ::= { digestAlgorithm 10 }
/// ```
///
///
pub fn id_hmacWithSHA384() -> ID {
    OBJECT_IDENTIFIER([digestAlgorithm().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA512       ID ::= { digestAlgorithm 11 }
/// ```
///
///
pub fn id_hmacWithSHA512() -> ID {
    OBJECT_IDENTIFIER([digestAlgorithm().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-gmac                 ID ::= { iso9797 part3(3) gmac(4) }
/// ```
///
///
pub fn id_gmac() -> ID {
    OBJECT_IDENTIFIER([iso9797().0, Vec::<u32>::from([/* part3 */ 3, /* gmac */ 4])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mD5Algorithm ALGORITHM ::= {
///   PARMS          NULL
///   IDENTIFIED BY {iso(1) member-body(2) us(840) rsadsi(113549) digestAlgorithm(2) md5(5)}}
/// ```
///
///
pub fn mD5Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: OBJECT_IDENTIFIER(Vec::<u32>::from([
            /* iso */ 1, /* member-body */ 2, /* us */ 840,
            /* rsadsi */ 113549, /* digestAlgorithm */ 2, /* md5 */ 5,
        ])), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mD5Algorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha1Algorithm ALGORITHM ::= {
///   PARMS          NULL
///   IDENTIFIED BY id-sha1 }
/// ```
///
///
pub fn sha1Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: id_sha1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha1Algorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha256 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha256 }
/// ```
///
///
pub fn sha256() -> ALGORITHM {
    ALGORITHM {
        id: id_sha256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha256 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha384 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha384 }
/// ```
///
///
pub fn sha384() -> ALGORITHM {
    ALGORITHM {
        id: id_sha384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha384 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha512 }
/// ```
///
///
pub fn sha512() -> ALGORITHM {
    ALGORITHM {
        id: id_sha512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha224 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha224 }
/// ```
///
///
pub fn sha224() -> ALGORITHM {
    ALGORITHM {
        id: id_sha224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha224 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512-224 ALGORITHM ::= {
///   IDENTIFIED BY id-sha512-224 }
/// ```
///
///
pub fn sha512_224() -> ALGORITHM {
    ALGORITHM {
        id: id_sha512_224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512_224 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512-256 ALGORITHM ::= {
///   IDENTIFIED BY id-sha512-256 }
/// ```
///
///
pub fn sha512_256() -> ALGORITHM {
    ALGORITHM {
        id: id_sha512_256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512_256 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-224 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-224 }
/// ```
///
///
pub fn sha3_224() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_224 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-256 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-256 }
/// ```
///
///
pub fn sha3_256() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_256 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-384 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-384 }
/// ```
///
///
pub fn sha3_384() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_384 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-512 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-512 }
/// ```
///
///
pub fn sha3_512() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_512 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake128 ALGORITHM ::= {
///   IDENTIFIED BY id-shake128 }
/// ```
///
///
pub fn shake128() -> ALGORITHM {
    ALGORITHM {
        id: id_shake128(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake128 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake256 ALGORITHM ::= {
///   IDENTIFIED BY id-shake256 }
/// ```
///
///
pub fn shake256() -> ALGORITHM {
    ALGORITHM {
        id: id_shake256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake256 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake128-len ALGORITHM ::= {
///   PARMS         ShakeOutputLen
///   IDENTIFIED BY id-shake128-len }
/// ```
///
///
pub fn shake128_len() -> ALGORITHM {
    ALGORITHM {
        id: id_shake128_len(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake128_len {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ShakeOutputLen; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ShakeOutputLen(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ShakeOutputLen(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ShakeOutputLen(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake256-len ALGORITHM ::= {
///   PARMS         ShakeOutputLen
///   IDENTIFIED BY id-shake256-len }
/// ```
///
///
pub fn shake256_len() -> ALGORITHM {
    ALGORITHM {
        id: id_shake256_len(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake256_len {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ShakeOutputLen; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ShakeOutputLen(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ShakeOutputLen(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ShakeOutputLen(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShakeOutputLen  ::=  INTEGER
/// ```
pub type ShakeOutputLen = INTEGER;

pub fn _decode_ShakeOutputLen(el: &X690Element) -> ASN1Result<ShakeOutputLen> {
    BER.decode_integer(&el)
}

pub fn _encode_ShakeOutputLen(value_: &ShakeOutputLen) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_ShakeOutputLen(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashAlgorithms ALGORITHM ::= {sha1Algorithm |
///                               sha224 |
///                               sha256 |
///                               sha384 |
///                               sha512 }
/// ```
///
///
pub fn HashAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([sha1Algorithm(), sha224(), sha256(), sha384(), sha512()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aes128-CBC ALGORITHM ::= {  -- CSOR
///   PARMS         AES-InitializationVector
///   IDENTIFIED BY id-aes128-CBC }
/// ```
///
///
pub fn aes128_CBC() -> ALGORITHM {
    ALGORITHM {
        id: id_aes128_CBC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod aes128_CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AES_InitializationVector; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AES_InitializationVector(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AES_InitializationVector(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AES_InitializationVector(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aes192-CBC ALGORITHM ::= { -- CSOR
///   PARMS         AES-InitializationVector
///   IDENTIFIED BY id-aes192-CBC }
/// ```
///
///
pub fn aes192_CBC() -> ALGORITHM {
    ALGORITHM {
        id: id_aes192_CBC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod aes192_CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AES_InitializationVector; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AES_InitializationVector(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AES_InitializationVector(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AES_InitializationVector(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aes256-CBC ALGORITHM ::= { -- CSOR
///   PARMS         AES-InitializationVector
///   IDENTIFIED BY id-aes256-CBC }
/// ```
///
///
pub fn aes256_CBC() -> ALGORITHM {
    ALGORITHM {
        id: id_aes256_CBC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod aes256_CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AES_InitializationVector; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AES_InitializationVector(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AES_InitializationVector(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AES_InitializationVector(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AES-InitializationVector  ::=  OCTET STRING (SIZE (16))
/// ```
pub type AES_InitializationVector = OCTET_STRING; // OctetStringType

pub fn _decode_AES_InitializationVector(el: &X690Element) -> ASN1Result<AES_InitializationVector> {
    BER.decode_octet_string(&el)
}

pub fn _encode_AES_InitializationVector(
    value_: &AES_InitializationVector,
) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_AES_InitializationVector(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rsaEncryptionAlgorithm ALGORITHM ::= { -- IETF RFC 4055
///   PARMS         NULL
///   IDENTIFIED BY rsaEncryption }
/// ```
///
///
pub fn rsaEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: rsaEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod rsaEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// keyExchangeAlgorithm ALGORITHM ::= { -- IETF RFC 3279
///   PARMS         KEA-Parms-Id
///   IDENTIFIED BY id-keyExchangeAlgorithm }
/// ```
///
///
pub fn keyExchangeAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: id_keyExchangeAlgorithm(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod keyExchangeAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = KEA_Parms_Id; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_KEA_Parms_Id(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_KEA_Parms_Id(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_KEA_Parms_Id(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEA-Parms-Id  ::=  OCTET STRING (SIZE (10))
/// ```
pub type KEA_Parms_Id = OCTET_STRING; // OctetStringType

pub fn _decode_KEA_Parms_Id(el: &X690Element) -> ASN1Result<KEA_Parms_Id> {
    BER.decode_octet_string(&el)
}

pub fn _encode_KEA_Parms_Id(value_: &KEA_Parms_Id) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_KEA_Parms_Id(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsa ALGORITHM ::= { -- IETF RFC 5480
///   PARMS         DSS-Parms
///   IDENTIFIED BY id-dsa }
/// ```
///
///
pub fn dsa() -> ALGORITHM {
    ALGORITHM {
        id: id_dsa(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsa {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DSS_Parms; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DSS_Parms(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DSS_Parms(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DSS_Parms(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSS-Parms ::= SEQUENCE {
///   p   INTEGER,
///   q   INTEGER,
///   g   INTEGER,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DSS_Parms {
    pub p: INTEGER,
    pub q: INTEGER,
    pub g: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl DSS_Parms {
    pub fn new(p: INTEGER, q: INTEGER, g: INTEGER, _unrecognized: Vec<X690Element>) -> Self {
        DSS_Parms {
            p,
            q,
            g,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for DSS_Parms {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DSS_Parms(el)
    }
}

pub const _rctl1_components_for_DSS_Parms: &[ComponentSpec; 3] = &[
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

pub const _rctl2_components_for_DSS_Parms: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DSS_Parms: &[ComponentSpec; 0] = &[];

pub fn _decode_DSS_Parms(el: &X690Element) -> ASN1Result<DSS_Parms> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSS-Parms")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DSS_Parms,
        _eal_components_for_DSS_Parms,
        _rctl2_components_for_DSS_Parms,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut p_: OPTIONAL<INTEGER> = None;
    let mut q_: OPTIONAL<INTEGER> = None;
    let mut g_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "p" => p_ = Some(BER.decode_integer(_el)?),
            "q" => q_ = Some(BER.decode_integer(_el)?),
            "g" => g_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DSS_Parms {
        p: p_.unwrap(),
        q: q_.unwrap(),
        g: g_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DSS_Parms(value_: &DSS_Parms) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_integer(&value_.p)?);
    components_.push(BER.encode_integer(&value_.q)?);
    components_.push(BER.encode_integer(&value_.g)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DSS_Parms(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DSS-Parms")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DSS_Parms,
        _eal_components_for_DSS_Parms,
        _rctl2_components_for_DSS_Parms,
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
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecPublicKey ALGORITHM ::= { -- IETF RFC 5480
///   PARMS         X509Curves
///   IDENTIFIED BY id-ecPublicKey }
/// ```
///
///
pub fn ecPublicKey() -> ALGORITHM {
    ALGORITHM {
        id: id_ecPublicKey(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecPublicKey {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X509Curves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X509Curves(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X509Curves(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_X509Curves(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecDH ALGORITHM ::= { -- IETF RFC 5480
///   PARMS         X509Curves
///   IDENTIFIED BY id-ecDH }
/// ```
///
///
pub fn ecDH() -> ALGORITHM {
    ALGORITHM {
        id: id_ecDH(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecDH {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X509Curves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X509Curves(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X509Curves(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_X509Curves(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecMQV ALGORITHM ::= { -- IETF RFC 5480
///   PARMS         X509Curves
///   IDENTIFIED BY id-ecMQV }
/// ```
///
///
pub fn ecMQV() -> ALGORITHM {
    ALGORITHM {
        id: id_ecMQV(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecMQV {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X509Curves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X509Curves(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X509Curves(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_X509Curves(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dh-public-numberAlgorithm ALGORITHM ::= {
///   PARMS         DomainParameters
///   IDENTIFIED BY dh-public-number }
/// ```
///
///
pub fn dh_public_numberAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: dh_public_number(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dh_public_numberAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DomainParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DomainParameters(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DomainParameters(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DomainParameters(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DomainParameters ::= SEQUENCE {
///   p               INTEGER, -- odd prime, p=jq+1
///   g               INTEGER, -- generator, g
///   q               INTEGER, -- factor of p-1
///   j               INTEGER  OPTIONAL, -- subgroup factor
///   validationParms ValidationParms OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DomainParameters {
    pub p: INTEGER,
    pub g: INTEGER,
    pub q: INTEGER,
    pub j: OPTIONAL<INTEGER>,
    pub validationParms: OPTIONAL<ValidationParms>,
    pub _unrecognized: Vec<X690Element>,
}
impl DomainParameters {
    pub fn new(
        p: INTEGER,
        g: INTEGER,
        q: INTEGER,
        j: OPTIONAL<INTEGER>,
        validationParms: OPTIONAL<ValidationParms>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DomainParameters {
            p,
            g,
            q,
            j,
            validationParms,
            _unrecognized,
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
        "validationParms",
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
    let mut validationParms_: OPTIONAL<ValidationParms> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
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
            "validationParms" => validationParms_ = Some(_decode_ValidationParms(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DomainParameters {
        p: p_.unwrap(),
        g: g_.unwrap(),
        q: q_.unwrap(),
        j: j_,
        validationParms: validationParms_,
        _unrecognized,
    })
}

pub fn _encode_DomainParameters(value_: &DomainParameters) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(BER.encode_integer(&value_.p)?);
    components_.push(BER.encode_integer(&value_.g)?);
    components_.push(BER.encode_integer(&value_.q)?);
    if let Some(v_) = &value_.j {
        components_.push(BER.encode_integer(&v_)?);
    }
    if let Some(v_) = &value_.validationParms {
        components_.push(_encode_ValidationParms(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
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
            "validationParms" => _validate_ValidationParms(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ValidationParms ::= SEQUENCE {
///   seed         BIT STRING,
///   pgenCounter  INTEGER,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ValidationParms {
    pub seed: BIT_STRING,
    pub pgenCounter: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl ValidationParms {
    pub fn new(seed: BIT_STRING, pgenCounter: INTEGER, _unrecognized: Vec<X690Element>) -> Self {
        ValidationParms {
            seed,
            pgenCounter,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ValidationParms {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ValidationParms(el)
    }
}

pub const _rctl1_components_for_ValidationParms: &[ComponentSpec; 2] = &[
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

pub const _rctl2_components_for_ValidationParms: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ValidationParms: &[ComponentSpec; 0] = &[];

pub fn _decode_ValidationParms(el: &X690Element) -> ASN1Result<ValidationParms> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ValidationParms"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ValidationParms,
        _eal_components_for_ValidationParms,
        _rctl2_components_for_ValidationParms,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut seed_: OPTIONAL<BIT_STRING> = None;
    let mut pgenCounter_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "seed" => seed_ = Some(BER.decode_bit_string(_el)?),
            "pgenCounter" => pgenCounter_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ValidationParms {
        seed: seed_.unwrap(),
        pgenCounter: pgenCounter_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ValidationParms(value_: &ValidationParms) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_bit_string(&value_.seed)?);
    components_.push(BER.encode_integer(&value_.pgenCounter)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ValidationParms(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ValidationParms"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ValidationParms,
        _eal_components_for_ValidationParms,
        _rctl2_components_for_ValidationParms,
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
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha1WithRSAEncryptionAlgorithm ALGORITHM ::= { -- IETF 7427
///   PARMS         NULL
///   IDENTIFIED BY sha1WithRSAEncryption }
/// ```
///
///
pub fn sha1WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha1WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha1WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha224WithRSAEncryptionAlgorithm ALGORITHM ::= { -- IETF RFC 5754
///   PARMS         NULL
///   IDENTIFIED BY sha224WithRSAEncryption }
/// ```
///
///
pub fn sha224WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha224WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha224WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha256WithRSAEncryptionAlgorithm ALGORITHM ::= { -- IETF RFC 7427
///   PARMS         NULL
///   IDENTIFIED BY sha256WithRSAEncryption }
/// ```
///
///
pub fn sha256WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha256WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha256WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha384WithRSAEncryptionAlgorithm ALGORITHM ::= { -- IETF RFC 7427
///   PARMS         NULL
///   IDENTIFIED BY sha384WithRSAEncryption }
/// ```
///
///
pub fn sha384WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha384WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha384WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512WithRSAEncryptionAlgorithm ALGORITHM ::= { -- IETF RFC 7427
///   PARMS         NULL
///   IDENTIFIED BY sha512WithRSAEncryption }
/// ```
///
///
pub fn sha512WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha512WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rSASSA-PSS ALGORITHM ::= {
///   PARMS                 SEQUENCE {
///     hashAlgorithm    [0]  AlgorithmIdentifier {{HashAlgorithms}},
///  -- maskGenAlgorithm [1]  AlgorithmIdentifier {{MaskGenAlgorithms}},
///     saltLength       [2]  INTEGER DEFAULT 20,
///     trailerField     [3]  INTEGER DEFAULT 1 }
///   IDENTIFIED BY         id-RSASSA-PSS }
/// ```
///
///
pub fn rSASSA_PSS() -> ALGORITHM {
    ALGORITHM {
        id: id_RSASSA_PSS(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod rSASSA_PSS {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = rSASSA_PSS_Type; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_rSASSA_PSS_Type(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_rSASSA_PSS_Type(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_rSASSA_PSS_Type(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsa-with-sha224 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-dsa-with-sha224 }
/// ```
///
///
pub fn dsa_with_sha224() -> ALGORITHM {
    ALGORITHM {
        id: id_dsa_with_sha224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsa_with_sha224 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsa-with-sha256 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-dsa-with-sha256 }
/// ```
///
///
pub fn dsa_with_sha256() -> ALGORITHM {
    ALGORITHM {
        id: id_dsa_with_sha256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsa_with_sha256 {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA224-Algorithm ALGORITHM ::= { -- IETF RFC
///   IDENTIFIED BY ecdsa-with-SHA224 }
/// ```
///
///
pub fn ecdsa_with_SHA224_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA224_Algorithm {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA256-Algorithm ALGORITHM ::= { -- IETF RFC 5758
///   IDENTIFIED BY ecdsa-with-SHA256 }
/// ```
///
///
pub fn ecdsa_with_SHA256_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA256_Algorithm {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA384-Algorithm ALGORITHM ::= { -- IETF RFC 5758
///   IDENTIFIED BY ecdsa-with-SHA384 }
/// ```
///
///
pub fn ecdsa_with_SHA384_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA384_Algorithm {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA512-Algorithm ALGORITHM ::= { -- IETF RFC 5758
///   IDENTIFIED BY ecdsa-with-SHA512 }
/// ```
///
///
pub fn ecdsa_with_SHA512_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA512_Algorithm {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hmacWithSHA224 ALGORITHM ::= {  -- IETF RFC 4231
///   PARMS         NULL
///   IDENTIFIED BY id-hmacWithSHA224 }
/// ```
///
///
pub fn hmacWithSHA224() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA224 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hmacWithSHA256 ALGORITHM ::= {  -- IETF RFC 4231
///   PARMS         NULL
///   IDENTIFIED BY id-hmacWithSHA256 }
/// ```
///
///
pub fn hmacWithSHA256() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA256 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hmacWithSHA384 ALGORITHM ::= {  -- IETF RFC 4231
///   PARMS         NULL
///   IDENTIFIED BY id-hmacWithSHA384 }
/// ```
///
///
pub fn hmacWithSHA384() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA384 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hmacWithSHA512 ALGORITHM ::= {  -- IETF RFC 4231
///   PARMS         NULL
///   IDENTIFIED BY id-hmacWithSHA512 }
/// ```
///
///
pub fn hmacWithSHA512() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA512 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rSASSA-PSS-Type ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct rSASSA_PSS_Type {
    pub hashAlgorithm: AlgorithmIdentifier,
    pub saltLength: OPTIONAL<INTEGER>,
    pub trailerField: OPTIONAL<INTEGER>,
}
impl rSASSA_PSS_Type {
    pub fn new(
        hashAlgorithm: AlgorithmIdentifier,
        saltLength: OPTIONAL<INTEGER>,
        trailerField: OPTIONAL<INTEGER>,
    ) -> Self {
        rSASSA_PSS_Type {
            hashAlgorithm,
            saltLength,
            trailerField,
        }
    }
    pub fn _default_value_for_saltLength() -> INTEGER {
        Vec::from([ 20 ])
    }
    pub fn _default_value_for_trailerField() -> INTEGER {
        Vec::from([ 1 ])
    }
}
impl TryFrom<&X690Element> for rSASSA_PSS_Type {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_rSASSA_PSS_Type(el)
    }
}

pub const _rctl1_components_for_rSASSA_PSS_Type: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "hashAlgorithm",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
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

pub const _rctl2_components_for_rSASSA_PSS_Type: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_rSASSA_PSS_Type: &[ComponentSpec; 0] = &[];

pub fn _decode_rSASSA_PSS_Type(el: &X690Element) -> ASN1Result<rSASSA_PSS_Type> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rSASSA-PSS-Type"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_rSASSA_PSS_Type,
        _eal_components_for_rSASSA_PSS_Type,
        _rctl2_components_for_rSASSA_PSS_Type,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut hashAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut saltLength_: OPTIONAL<INTEGER> = None;
    let mut trailerField_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashAlgorithm" => {
                hashAlgorithm_ = Some(|el: &X690Element| -> ASN1Result<AlgorithmIdentifier> {
                    Ok(_decode_AlgorithmIdentifier(&el.inner()?)?)
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
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rSASSA-PSS-Type")
                )
            }
        }
    }
    Ok(rSASSA_PSS_Type {
        hashAlgorithm: hashAlgorithm_.unwrap(),
        saltLength: saltLength_,
        trailerField: trailerField_,
    })
}

pub fn _encode_rSASSA_PSS_Type(value_: &rSASSA_PSS_Type) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(|v_1: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&_encode_AlgorithmIdentifier(&v_1)?),
        ))
    }(&value_.hashAlgorithm)?);
    if let Some(v_) = &value_.saltLength {
        if *v_ != rSASSA_PSS_Type::_default_value_for_saltLength() {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&BER.encode_integer(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.trailerField {
        if *v_ != rSASSA_PSS_Type::_default_value_for_trailerField() {
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

pub fn _validate_rSASSA_PSS_Type(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rSASSA-PSS-Type"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_rSASSA_PSS_Type,
        _eal_components_for_rSASSA_PSS_Type,
        _rctl2_components_for_rSASSA_PSS_Type,
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
                Ok(_validate_AlgorithmIdentifier(&el.inner()?)?)
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
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rSASSA-PSS-Type")
                )
            }
        }
    }
    Ok(())
}
