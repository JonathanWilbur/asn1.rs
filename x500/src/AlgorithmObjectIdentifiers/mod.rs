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
use wildboar_asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ID  ::=  OBJECT IDENTIFIER
/// ```
pub type ID = OBJECT_IDENTIFIER; // ObjectIdentifierType

#[inline]
pub fn _decode_ID(el: &X690Element) -> ASN1Result<ID> {
    BER.decode_object_identifier(&el)
}

#[inline]
pub fn _encode_ID(value_: &ID) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

#[inline]
pub fn _validate_ID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nullAlgorithm           ID ::= {algorithm 0}
/// ```
///
#[inline]
pub fn nullAlgorithm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(algorithm(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// encryptionAlgorithm     ID ::= {algorithm 1}
/// ```
///
#[inline]
pub fn encryptionAlgorithm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(algorithm(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hashAlgorithm           ID ::= {algorithm 2}
/// ```
///
#[inline]
pub fn hashAlgorithm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(algorithm(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signatureAlgorithm      ID ::= {algorithm 3}
/// ```
///
#[inline]
pub fn signatureAlgorithm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(algorithm(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ea                   ID ::= encryptionAlgorithm
/// ```
///
#[inline]
pub fn id_ea () -> OBJECT_IDENTIFIER {
	encryptionAlgorithm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ha                   ID ::= hashAlgorithm
/// ```
///
#[inline]
pub fn id_ha () -> OBJECT_IDENTIFIER {
	hashAlgorithm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sa                   ID ::= signatureAlgorithm
/// ```
///
#[inline]
pub fn id_sa () -> OBJECT_IDENTIFIER {
	signatureAlgorithm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ea-rsa               ID ::= {id-ea 1}
/// ```
///
#[inline]
pub fn id_ea_rsa () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ea(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ha-sqMod-n           ID ::= {id-ha 1}
/// ```
///
#[inline]
pub fn id_ha_sqMod_n () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ha(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sa-sqMod-nWithRSA    ID ::= {id-sa 1}
/// ```
///
#[inline]
pub fn id_sa_sqMod_nWithRSA () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sa(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// us-iso                  ID ::= { iso(1) member-body(2) us(840) }
/// ```
///
#[inline]
pub fn us_iso () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ansi-x9-57              ID ::= { us-iso ansi-x9-57(10040) }
/// ```
///
#[inline]
pub fn ansi_x9_57 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, /* ansi-x9-57 */ 0xce, 56 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ansi-x9-62              ID ::= { us-iso ansi-x962(10045) }
/// ```
///
#[inline]
pub fn ansi_x9_62 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, /* ansi-x962 */ 0xce, 61 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ansi-x9-42              ID ::= { us-iso ansi-x942(10046) }
/// ```
///
#[inline]
pub fn ansi_x9_42 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, /* ansi-x942 */ 0xce, 62 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iso-standard            ID ::= { iso(1) standard(0) }
/// ```
///
#[inline]
pub fn iso_standard () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* standard */ 0) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iso9797                 ID ::= { iso-standard message-authentication-codes(9797) }
/// ```
///
#[inline]
pub fn iso9797 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 40, /* message-authentication-codes */ 0xcc, 69 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iso-organization        ID ::= { iso(1) identified-organization(3) }
/// ```
///
#[inline]
pub fn iso_organization () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certicom                ID ::= { iso-organization certicom(132) }
/// ```
///
#[inline]
pub fn certicom () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, /* certicom */ 0x81, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certicom-curve          ID ::= { certicom curve(0) }
/// ```
///
#[inline]
pub fn certicom_curve () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, /* curve */ 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// teletrust               ID ::= { iso-organization teletrust(36) }
/// ```
///
#[inline]
pub fn teletrust () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, /* teletrust */ 36 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecStdCurvesAndGen       ID ::= { teletrust algorithm(3) signature-algorithm(3) ecSign(2) 8}
/// ```
///
#[inline]
pub fn ecStdCurvesAndGen () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, /* algorithm */ 3, /* signature-algorithm */ 3, /* ecSign */ 2, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// versionOne              ID ::= { ecStdCurvesAndGen ellipticCurve(1) versionOne(1) }
/// ```
///
#[inline]
pub fn versionOne () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, /* ellipticCurve */ 1, /* versionOne */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// us-joint                ID ::= { joint-iso-itu-t(2) country(16) us(840) }
/// ```
///
#[inline]
pub fn us_joint () -> OBJECT_IDENTIFIER {
	oid!(/* joint-iso-itu-t */ 2,/* country */ 16,/* us */ 840) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// usgov                   ID ::= { us-joint organization(1) gov(101) }
/// ```
///
#[inline]
pub fn usgov () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, /* organization */ 1, /* gov */ 101 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dodAlgorithms           ID ::= { usgov dod(2) infosec(1) algorithms(1) }
/// ```
///
#[inline]
pub fn dodAlgorithms () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, /* dod */ 2, /* infosec */ 1, /* algorithms */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// csor                    ID ::= { usgov csor(3) }
/// ```
///
#[inline]
pub fn csor () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, /* csor */ 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nistAlgorithms          ID ::= { csor nistAlgorithm(4) }
/// ```
///
#[inline]
pub fn nistAlgorithms () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, /* nistAlgorithm */ 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aes                     ID ::= { nistAlgorithms 1 }
/// ```
///
#[inline]
pub fn aes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hashAlgs                ID ::= { nistAlgorithms hashalgs(2) }
/// ```
///
#[inline]
pub fn hashAlgs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, /* hashalgs */ 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sigAlgs                 ID ::= { nistAlgorithms 3 }
/// ```
///
#[inline]
pub fn sigAlgs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rsadsi                  ID ::= { iso(1) member-body(2) us(840) rsadsi(113549) }
/// ```
///
#[inline]
pub fn rsadsi () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkcs-1                  ID ::= { rsadsi pkcs(1) pkcs-1(1) }
/// ```
///
#[inline]
pub fn pkcs_1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, /* pkcs */ 1, /* pkcs-1 */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// digestAlgorithm         ID ::= { rsadsi digestAlgorithm(2) }
/// ```
///
#[inline]
pub fn digestAlgorithm () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, /* digestAlgorithm */ 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes128-CBC           ID ::= { aes 2 }
/// ```
///
#[inline]
pub fn id_aes128_CBC () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes192-CBC           ID ::= { aes 22 }
/// ```
///
#[inline]
pub fn id_aes192_CBC () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1, 22 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes256-CBC           ID ::= { aes 42 }
/// ```
///
#[inline]
pub fn id_aes256_CBC () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1, 42 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes128-wrap          ID ::= { aes 5 }
/// ```
///
#[inline]
pub fn id_aes128_wrap () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes192-wrap          ID ::= { aes 25 }
/// ```
///
#[inline]
pub fn id_aes192_wrap () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1, 25 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aes256-wrap          ID ::= { aes 45 }
/// ```
///
#[inline]
pub fn id_aes256_wrap () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 1, 45 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rsaEncryption           ID ::= { pkcs-1 rsaEncryption(1)}
/// ```
///
#[inline]
pub fn rsaEncryption () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, /* rsaEncryption */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-keyExchangeAlgorithm ID ::= { dodAlgorithms id-keyExchangeAlgorithm(22)}
/// ```
///
#[inline]
pub fn id_keyExchangeAlgorithm () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 2, 1, 1, /* id-keyExchangeAlgorithm */ 22 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa                  ID ::= { ansi-x9-57 x9algorithm(4) 1 }
/// ```
///
#[inline]
pub fn id_dsa () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 56, /* x9algorithm */ 4, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecPublicKey          ID ::= { ansi-x9-62 keyType(2) 1 }
/// ```
///
#[inline]
pub fn id_ecPublicKey () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* keyType */ 2, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecDH                 ID ::= { certicom schemes(1) ecdh(12) }
/// ```
///
#[inline]
pub fn id_ecDH () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, /* schemes */ 1, /* ecdh */ 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecMQV                ID ::= { certicom schemes(1) ecmqv(13) }
/// ```
///
#[inline]
pub fn id_ecMQV () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, /* schemes */ 1, /* ecmqv */ 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dh-public-number        ID ::= { ansi-x9-42 number-type(2) dh-public-number(1) }
/// ```
///
#[inline]
pub fn dh_public_number () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 62, /* number-type */ 2, /* dh-public-number */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha1                 ID ::= {iso(1) identified-organization(3) oiw(14) secsig(3)
///                                 algorithms(2) 26}
/// ```
///
#[inline]
pub fn id_sha1 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* oiw */ 14,/* secsig */ 3,/* algorithms */ 2,26) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha256               ID ::= { hashAlgs 1 }
/// ```
///
#[inline]
pub fn id_sha256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha384               ID ::= { hashAlgs 2 }
/// ```
///
#[inline]
pub fn id_sha384 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha512               ID ::= { hashAlgs 3 }
/// ```
///
#[inline]
pub fn id_sha512 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha224               ID ::= { hashAlgs 4 }
/// ```
///
#[inline]
pub fn id_sha224 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha512-224           ID ::= { hashAlgs 5 }
/// ```
///
#[inline]
pub fn id_sha512_224 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha512-256           ID ::= { hashAlgs 6 }
/// ```
///
#[inline]
pub fn id_sha512_256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-224             ID ::= { hashAlgs 7 }
/// ```
///
#[inline]
pub fn id_sha3_224 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-256             ID ::= { hashAlgs 8 }
/// ```
///
#[inline]
pub fn id_sha3_256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-384             ID ::= { hashAlgs 9 }
/// ```
///
#[inline]
pub fn id_sha3_384 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sha3-512             ID ::= { hashAlgs 10 }
/// ```
///
#[inline]
pub fn id_sha3_512 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake128             ID ::= { hashAlgs 11 }
/// ```
///
#[inline]
pub fn id_shake128 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake256             ID ::= { hashAlgs 12 }
/// ```
///
#[inline]
pub fn id_shake256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake128-len         ID ::= { hashAlgs 17 }
/// ```
///
#[inline]
pub fn id_shake128_len () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-shake256-len         ID ::= { hashAlgs 18 }
/// ```
///
#[inline]
pub fn id_shake256_len () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 2, 18 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hashAlg                 ID ::= {  iso(1) identified-organization(3) dod(6) internet(1)
///                                   private(4) enterprise(1) kudelski(1722)
///                                   cryptography(12) 2 }
/// ```
///
#[inline]
pub fn hashAlg () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* dod */ 6,/* internet */ 1,/* private */ 4,/* enterprise */ 1,/* kudelski */ 1722,/* cryptography */ 12,2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha1WithRSAEncryption   ID ::= { pkcs-1 sha1WithRSAEncryption(5) }
/// ```
///
#[inline]
pub fn sha1WithRSAEncryption () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, /* sha1WithRSAEncryption */ 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha256WithRSAEncryption ID ::= { pkcs-1 sha256WithRSAEncryption(11) }
/// ```
///
#[inline]
pub fn sha256WithRSAEncryption () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, /* sha256WithRSAEncryption */ 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha384WithRSAEncryption ID ::= { pkcs-1 sha384WithRSAEncryption(12) }
/// ```
///
#[inline]
pub fn sha384WithRSAEncryption () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, /* sha384WithRSAEncryption */ 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512WithRSAEncryption ID ::= { pkcs-1 sha512WithRSAEncryption(13) }
/// ```
///
#[inline]
pub fn sha512WithRSAEncryption () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, /* sha512WithRSAEncryption */ 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha224WithRSAEncryption ID ::= { pkcs-1 sha224WithRSAEncryption(14) }
/// ```
///
#[inline]
pub fn sha224WithRSAEncryption () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, /* sha224WithRSAEncryption */ 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-RSASSA-PSS           ID ::= { pkcs-1 10 }
/// ```
///
#[inline]
pub fn id_RSASSA_PSS () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mgf1                 ID ::= { pkcs-1 8 }
/// ```
///
#[inline]
pub fn id_mgf1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 1, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha1        ID ::= {iso(1) member-body(2) us(840) x9-57(10040) x9algorithm(4)
///                                 dsa-with-sha1(3)}
/// ```
///
#[inline]
pub fn id_dsa_with_sha1 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* x9-57 */ 10040,/* x9algorithm */ 4,/* dsa-with-sha1 */ 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha224      ID ::= { sigAlgs 1 }
/// ```
///
#[inline]
pub fn id_dsa_with_sha224 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 3, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dsa-with-sha256      ID ::= { sigAlgs 2 }
/// ```
///
#[inline]
pub fn id_dsa_with_sha256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 101, 3, 4, 3, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA224       ID ::= { ansi-x9-62 signatures(4)
///                                                 ecdsa-with-SHA2(3) 1 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA224 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA256       ID ::= { ansi-x9-62 signatures(4)
///                                                 ecdsa-with-SHA2(3) 2 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA384       ID ::= { ansi-x9-62 signatures(4)
///                                                 ecdsa-with-SHA2(3) 3 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA384 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA512       ID ::= { ansi-x9-62 signatures(4) ecdsa-with-SHA2(3) 4 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA512 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* signatures */ 4, /* ecdsa-with-SHA2 */ 3, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp192r1       ID ::= { ansi-x9-62 curves(3) prime(1) 1 }
/// ```
///
#[inline]
pub fn secp192r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* curves */ 3, /* prime */ 1, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect163k1       ID ::= { certicom-curve 1 }
/// ```
///
#[inline]
pub fn sect163k1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect163r2       ID ::= { certicom-curve 15 }
/// ```
///
#[inline]
pub fn sect163r2 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp224r1       ID ::= { certicom-curve 33 }
/// ```
///
#[inline]
pub fn secp224r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 33 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect233k1       ID ::= { certicom-curve 26 }
/// ```
///
#[inline]
pub fn sect233k1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 26 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect233r1       ID ::= { certicom-curve 27 }
/// ```
///
#[inline]
pub fn sect233r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 27 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp256r1       ID ::= { ansi-x9-62 curves(3) prime(1) 7 }
/// ```
///
#[inline]
pub fn secp256r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 61, /* curves */ 3, /* prime */ 1, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect283k1       ID ::= { certicom-curve 16 }
/// ```
///
#[inline]
pub fn sect283k1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect283r1       ID ::= { certicom-curve 17 }
/// ```
///
#[inline]
pub fn sect283r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp384r1       ID ::= { certicom-curve 34 }
/// ```
///
#[inline]
pub fn secp384r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 34 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect409k1       ID ::= { certicom-curve 36 }
/// ```
///
#[inline]
pub fn sect409k1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 36 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect409r1       ID ::= { certicom-curve 37 }
/// ```
///
#[inline]
pub fn sect409r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 37 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// secp521r1       ID ::= { certicom-curve 35 }
/// ```
///
#[inline]
pub fn secp521r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 35 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect571k1       ID ::= { certicom-curve 38 }
/// ```
///
#[inline]
pub fn sect571k1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 38 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sect571r1       ID ::= { certicom-curve 39 }
/// ```
///
#[inline]
pub fn sect571r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 4, 0, 39 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP160r1 ID ::= { versionOne 1 }
/// ```
///
#[inline]
pub fn brainpoolP160r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP160t1 ID ::= { versionOne 2 }
/// ```
///
#[inline]
pub fn brainpoolP160t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP192r1 ID ::= { versionOne 3 }
/// ```
///
#[inline]
pub fn brainpoolP192r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP192t1 ID ::= { versionOne 4 }
/// ```
///
#[inline]
pub fn brainpoolP192t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP224r1 ID ::= { versionOne 5 }
/// ```
///
#[inline]
pub fn brainpoolP224r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP224t1 ID ::= { versionOne 6 }
/// ```
///
#[inline]
pub fn brainpoolP224t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP256r1 ID ::= { versionOne 7 }
/// ```
///
#[inline]
pub fn brainpoolP256r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP256t1 ID ::= { versionOne 8 }
/// ```
///
#[inline]
pub fn brainpoolP256t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP320r1 ID ::= { versionOne 9 }
/// ```
///
#[inline]
pub fn brainpoolP320r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP320t1 ID ::= { versionOne 10 }
/// ```
///
#[inline]
pub fn brainpoolP320t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP384r1 ID ::= { versionOne 11 }
/// ```
///
#[inline]
pub fn brainpoolP384r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP384t1 ID ::= { versionOne 12 }
/// ```
///
#[inline]
pub fn brainpoolP384t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP512r1 ID ::= { versionOne 13 }
/// ```
///
#[inline]
pub fn brainpoolP512r1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// brainpoolP512t1 ID ::= { versionOne 14 }
/// ```
///
#[inline]
pub fn brainpoolP512t1 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 36, 3, 3, 2, 8, 1, 1, 14 ].as_slice()) } // OID_GETTER
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

#[inline]
pub fn _decode_X509Curves(el: &X690Element) -> ASN1Result<X509Curves> {
    BER.decode_object_identifier(el)
}

#[inline]
pub fn _encode_X509Curves(value_: &X509Curves) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(value_)
}

#[inline]
pub fn _validate_X509Curves(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA224       ID ::= { digestAlgorithm 8 }
/// ```
///
#[inline]
pub fn id_hmacWithSHA224 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 2, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA256       ID ::= { digestAlgorithm 9 }
/// ```
///
#[inline]
pub fn id_hmacWithSHA256 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 2, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA384       ID ::= { digestAlgorithm 10 }
/// ```
///
#[inline]
pub fn id_hmacWithSHA384 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 2, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-hmacWithSHA512       ID ::= { digestAlgorithm 11 }
/// ```
///
#[inline]
pub fn id_hmacWithSHA512 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 2, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-gmac                 ID ::= { iso9797 part3(3) gmac(4) }
/// ```
///
#[inline]
pub fn id_gmac () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 40, 0xcc, 69, /* part3 */ 3, /* gmac */ 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// mD5Algorithm ALGORITHM ::= {
///   PARMS          NULL
///   IDENTIFIED BY {iso(1) member-body(2) us(840) rsadsi(113549) digestAlgorithm(2) md5(5)}}
/// ```
///
#[inline]
pub fn mD5Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: oid!(
            /* iso */ 1, /* member-body */ 2, /* us */ 840,
            /* rsadsi */ 113549, /* digestAlgorithm */ 2, /* md5 */ 5), /* OBJECT_FIELD_SETTING */
    }
}

pub mod mD5Algorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn sha1Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: id_sha1(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha1Algorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn sha256() -> ALGORITHM {
    ALGORITHM {
        id: id_sha256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha256 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha384 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha384 }
/// ```
///
#[inline]
pub fn sha384() -> ALGORITHM {
    ALGORITHM {
        id: id_sha384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha384 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha512 }
/// ```
///
#[inline]
pub fn sha512() -> ALGORITHM {
    ALGORITHM {
        id: id_sha512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha224 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-sha224 }
/// ```
///
#[inline]
pub fn sha224() -> ALGORITHM {
    ALGORITHM {
        id: id_sha224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha224 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512-224 ALGORITHM ::= {
///   IDENTIFIED BY id-sha512-224 }
/// ```
///
#[inline]
pub fn sha512_224() -> ALGORITHM {
    ALGORITHM {
        id: id_sha512_224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512_224 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha512-256 ALGORITHM ::= {
///   IDENTIFIED BY id-sha512-256 }
/// ```
///
#[inline]
pub fn sha512_256() -> ALGORITHM {
    ALGORITHM {
        id: id_sha512_256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512_256 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-224 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-224 }
/// ```
///
#[inline]
pub fn sha3_224() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_224 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-256 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-256 }
/// ```
///
#[inline]
pub fn sha3_256() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_256 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-384 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-384 }
/// ```
///
#[inline]
pub fn sha3_384() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_384 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha3-512 ALGORITHM ::= {
///   IDENTIFIED BY id-sha3-512 }
/// ```
///
#[inline]
pub fn sha3_512() -> ALGORITHM {
    ALGORITHM {
        id: id_sha3_512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha3_512 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake128 ALGORITHM ::= {
///   IDENTIFIED BY id-shake128 }
/// ```
///
#[inline]
pub fn shake128() -> ALGORITHM {
    ALGORITHM {
        id: id_shake128(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake128 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake256 ALGORITHM ::= {
///   IDENTIFIED BY id-shake256 }
/// ```
///
#[inline]
pub fn shake256() -> ALGORITHM {
    ALGORITHM {
        id: id_shake256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake256 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shake128-len ALGORITHM ::= {
///   PARMS         ShakeOutputLen
///   IDENTIFIED BY id-shake128-len }
/// ```
///
#[inline]
pub fn shake128_len() -> ALGORITHM {
    ALGORITHM {
        id: id_shake128_len(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake128_len {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ShakeOutputLen; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ShakeOutputLen(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ShakeOutputLen(value_)
    }

    #[inline]
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
#[inline]
pub fn shake256_len() -> ALGORITHM {
    ALGORITHM {
        id: id_shake256_len(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shake256_len {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ShakeOutputLen; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ShakeOutputLen(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ShakeOutputLen(value_)
    }

    #[inline]
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

#[inline]
pub fn _decode_ShakeOutputLen(el: &X690Element) -> ASN1Result<ShakeOutputLen> {
    BER.decode_integer(&el)
}

#[inline]
pub fn _encode_ShakeOutputLen(value_: &ShakeOutputLen) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

#[inline]
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
#[inline]
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
#[inline]
pub fn aes128_CBC() -> ALGORITHM {
    ALGORITHM {
        id: id_aes128_CBC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod aes128_CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AES_InitializationVector; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AES_InitializationVector(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AES_InitializationVector(value_)
    }

    #[inline]
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
#[inline]
pub fn aes192_CBC() -> ALGORITHM {
    ALGORITHM {
        id: id_aes192_CBC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod aes192_CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AES_InitializationVector; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AES_InitializationVector(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AES_InitializationVector(value_)
    }

    #[inline]
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
#[inline]
pub fn aes256_CBC() -> ALGORITHM {
    ALGORITHM {
        id: id_aes256_CBC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod aes256_CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AES_InitializationVector; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AES_InitializationVector(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AES_InitializationVector(value_)
    }

    #[inline]
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

#[inline]
pub fn _decode_AES_InitializationVector(el: &X690Element) -> ASN1Result<AES_InitializationVector> {
    BER.decode_octet_string(&el)
}

#[inline]
pub fn _encode_AES_InitializationVector(
    value_: &AES_InitializationVector,
) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

#[inline]
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
#[inline]
pub fn rsaEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: rsaEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod rsaEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn keyExchangeAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: id_keyExchangeAlgorithm(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod keyExchangeAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = KEA_Parms_Id; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_KEA_Parms_Id(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_KEA_Parms_Id(value_)
    }

    #[inline]
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

#[inline]
pub fn _decode_KEA_Parms_Id(el: &X690Element) -> ASN1Result<KEA_Parms_Id> {
    BER.decode_octet_string(&el)
}

#[inline]
pub fn _encode_KEA_Parms_Id(value_: &KEA_Parms_Id) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

#[inline]
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
#[inline]
pub fn dsa() -> ALGORITHM {
    ALGORITHM {
        id: id_dsa(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsa {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DSS_Parms; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DSS_Parms(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DSS_Parms(value_)
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
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
#[inline]
pub fn ecPublicKey() -> ALGORITHM {
    ALGORITHM {
        id: id_ecPublicKey(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecPublicKey {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X509Curves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X509Curves(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X509Curves(value_)
    }

    #[inline]
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
#[inline]
pub fn ecDH() -> ALGORITHM {
    ALGORITHM {
        id: id_ecDH(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecDH {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X509Curves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X509Curves(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X509Curves(value_)
    }

    #[inline]
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
#[inline]
pub fn ecMQV() -> ALGORITHM {
    ALGORITHM {
        id: id_ecMQV(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecMQV {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X509Curves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X509Curves(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X509Curves(value_)
    }

    #[inline]
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
#[inline]
pub fn dh_public_numberAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: dh_public_number(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dh_public_numberAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DomainParameters; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DomainParameters(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DomainParameters(value_)
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
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

    #[inline]
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

    #[inline]
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
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
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
#[inline]
pub fn sha1WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha1WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha1WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn sha224WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha224WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha224WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn sha256WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha256WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha256WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn sha384WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha384WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha384WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn sha512WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: sha512WithRSAEncryption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha512WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn rSASSA_PSS() -> ALGORITHM {
    ALGORITHM {
        id: id_RSASSA_PSS(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod rSASSA_PSS {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = rSASSA_PSS_Type; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_rSASSA_PSS_Type(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_rSASSA_PSS_Type(value_)
    }

    #[inline]
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
#[inline]
pub fn dsa_with_sha224() -> ALGORITHM {
    ALGORITHM {
        id: id_dsa_with_sha224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsa_with_sha224 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsa-with-sha256 ALGORITHM ::= { -- IETF RFC 5754
///   IDENTIFIED BY id-dsa-with-sha256 }
/// ```
///
#[inline]
pub fn dsa_with_sha256() -> ALGORITHM {
    ALGORITHM {
        id: id_dsa_with_sha256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsa_with_sha256 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA224-Algorithm ALGORITHM ::= { -- IETF RFC
///   IDENTIFIED BY ecdsa-with-SHA224 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA224_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA224_Algorithm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA256-Algorithm ALGORITHM ::= { -- IETF RFC 5758
///   IDENTIFIED BY ecdsa-with-SHA256 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA256_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA256_Algorithm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA384-Algorithm ALGORITHM ::= { -- IETF RFC 5758
///   IDENTIFIED BY ecdsa-with-SHA384 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA384_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA384_Algorithm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecdsa-with-SHA512-Algorithm ALGORITHM ::= { -- IETF RFC 5758
///   IDENTIFIED BY ecdsa-with-SHA512 }
/// ```
///
#[inline]
pub fn ecdsa_with_SHA512_Algorithm() -> ALGORITHM {
    ALGORITHM {
        id: ecdsa_with_SHA512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecdsa_with_SHA512_Algorithm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hmacWithSHA224 ALGORITHM ::= {  -- IETF RFC 4231
///   PARMS         NULL
///   IDENTIFIED BY id-hmacWithSHA224 }
/// ```
///
#[inline]
pub fn hmacWithSHA224() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA224(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA224 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn hmacWithSHA256() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA256(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA256 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn hmacWithSHA384() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA384(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA384 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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
#[inline]
pub fn hmacWithSHA512() -> ALGORITHM {
    ALGORITHM {
        id: id_hmacWithSHA512(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod hmacWithSHA512 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */

    #[inline]
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }

    #[inline]
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    pub fn _default_value_for_saltLength() -> INTEGER {
        Vec::from([20])
    }

    #[inline]
    pub fn _default_value_for_trailerField() -> INTEGER {
        Vec::from([1])
    }
}
impl TryFrom<&X690Element> for rSASSA_PSS_Type {
    type Error = ASN1Error;

    #[inline]
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
            X690Value::from_explicit(_encode_AlgorithmIdentifier(&v_1)?),
        ))
    }(&value_.hashAlgorithm)?);
    if let Some(v_) = &value_.saltLength {
        if *v_ != rSASSA_PSS_Type::_default_value_for_saltLength() {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(BER.encode_integer(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.trailerField {
        if *v_ != rSASSA_PSS_Type::_default_value_for_trailerField() {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 3),
                    X690Value::from_explicit(BER.encode_integer(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
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
