#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CMSObjectIdentifiers
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CMSObjectIdentifiers`.
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
use wildboar_asn1::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OID  ::=  OBJECT IDENTIFIER
/// ```
pub type OID = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_OID (el: &X690Element) -> ASN1Result<OID> {
	BER.decode_object_identifier(&el)
}

pub fn _encode_OID (value_: &OID) -> ASN1Result<X690Element> {
	BER.encode_object_identifier(&value_)
}

pub fn _validate_OID (el: &X690Element) -> ASN1Result<()> {
	BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkcs7 OID ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) }
/// ```
///
///
#[inline]
pub fn pkcs7 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549,/* pkcs */ 1,/* pkcs7 */ 7) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-data OID ::= { pkcs7 data(1) }
/// ```
///
///
#[inline]
pub fn id_data () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 7, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signedData OID ::= { pkcs7 signedData(2) }
/// ```
///
///
#[inline]
pub fn id_signedData () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 7, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-envelopedData OID ::= { pkcs7 envelopedData (3) }
/// ```
///
///
#[inline]
pub fn id_envelopedData () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 7, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-digestedData OID ::= { pkcs7 digestedData(5) }
/// ```
///
///
#[inline]
pub fn id_digestedData () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 7, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-encryptedData OID ::= { pkcs7 encryptedData (6) }
/// ```
///
///
#[inline]
pub fn id_encryptedData () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 7, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-namedkeyencryptedData OID ::= { iso(1) member-body(2) us(840)
/// x973(10060) types(1) namedKeyEncryptedData(2) }
/// ```
///
///
#[inline]
pub fn id_namedkeyencryptedData () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* types */ 1,/* namedKeyEncryptedData */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signcryptedData OID ::= { iso(1) member-body(2) us(840)
/// x973(10060) types(1) signcryptedData(3)}
/// ```
///
///
#[inline]
pub fn id_signcryptedData () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* types */ 1,/* signcryptedData */ 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcryption-manifest OID ::= { id-signcryptedData manifest(1) }
/// ```
///
///
#[inline]
pub fn signcryption_manifest () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 1, 3, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// xPath OID ::= { signcryption-manifest xPath(0) }
/// ```
///
///
#[inline]
pub fn xPath () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 1, 3, 1, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkcs9 OID ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs9(9) }
/// ```
///
///
#[inline]
pub fn pkcs9 () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549,/* pkcs */ 1,/* pkcs9 */ 9) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// smime OID ::= { pkcs9 smime(16) }
/// ```
///
///
#[inline]
pub fn smime () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-authData OID ::= { smime ct(1) 2 }
/// ```
///
///
#[inline]
pub fn id_ct_authData () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 16, 1, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contentType OID ::= { pkcs9 contentType(3) }
/// ```
///
///
#[inline]
pub fn id_contentType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-messageDigest OID ::= { pkcs9 messageDigest(4) }
/// ```
///
///
#[inline]
pub fn id_messageDigest () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-recip-info OID ::= { iso member-body(2) us(840) x973(10060) km(2) 1 }
/// ```
///
///
#[inline]
pub fn id_ckm_recip_info () -> OBJECT_IDENTIFIER {
	oid!(iso,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* km */ 2,1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-recip-info2 OID ::= { iso member-body(2) us(840) x973(10060) km(2) 2}
/// ```
///
///
#[inline]
pub fn id_ckm_recip_info2 () -> OBJECT_IDENTIFIER {
	oid!(iso,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* km */ 2,2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-algorithms OID ::= { iso member-body(2) us(840) x973(10060) algorithms(3) }
/// ```
///
///
#[inline]
pub fn id_ckm_algorithms () -> OBJECT_IDENTIFIER {
	oid!(iso,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* algorithms */ 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-symmetric OID ::= { id-ckm-algorithms symmetric(1) }
/// ```
///
///
#[inline]
pub fn id_ckm_symmetric () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 3, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-key-transport OID ::= { id-ckm-algorithms key-transport(2) }
/// ```
///
///
#[inline]
pub fn id_ckm_key_transport () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 3, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-key-agree-multiple-encrypt OID ::= {
/// id-ckm-algorithms key-agree-multiple-encrypt(3) }
/// ```
///
///
#[inline]
pub fn id_ckm_key_agree_multiple_encrypt () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 3, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-key-agree-hash OID ::= { id-ckm-algorithms key-agree-hash(4) }
/// ```
///
///
#[inline]
pub fn id_ckm_key_agree_hash () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 3, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ckm-header OID ::= { iso member-body(2) us(840) x973(10060) header(4) }
/// ```
///
///
#[inline]
pub fn id_ckm_header () -> OBJECT_IDENTIFIER {
	oid!(iso,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* header */ 4) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ckm-CMS OID ::= {
/// joint-iso-itu-t(2) international-organizations(23) set(42) vendors(9) griffin(10) business(3) tecsec(0) cms(2) header(2) }
/// ```
///
///
#[inline]
pub fn ckm_CMS () -> OBJECT_IDENTIFIER {
	oid!(/* joint-iso-itu-t */ 2,/* international-organizations */ 23,/* set */ 42,/* vendors */ 9,/* griffin */ 10,/* business */ 3,/* tecsec */ 0,/* cms */ 2,/* header */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Ivec OID ::= { ckm-CMS 1 }
/// ```
///
///
#[inline]
pub fn id_Ivec () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Secryptm OID ::= { ckm-CMS 2 }
/// ```
///
///
#[inline]
pub fn id_Secryptm () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Filelength OID ::= { ckm-CMS 3 }
/// ```
///
///
#[inline]
pub fn id_Filelength () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Filehash OID ::= { ckm-CMS 4 }
/// ```
///
///
#[inline]
pub fn id_Filehash () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Filename OID ::= { ckm-CMS 5 }
/// ```
///
///
#[inline]
pub fn id_Filename () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Domainlist OID ::= { ckm-CMS 6 }
/// ```
///
///
#[inline]
pub fn id_Domainlist () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Accessgrouplist OID ::= { ckm-CMS 7 }
/// ```
///
///
#[inline]
pub fn id_Accessgrouplist () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Issuer OID ::= { ckm-CMS 8 }
/// ```
///
///
#[inline]
pub fn id_Issuer () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-Credentiallist OID ::= { ckm-CMS 9 }
/// ```
///
///
#[inline]
pub fn id_Credentiallist () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-SignKey OID ::= { ckm-CMS 10 }
/// ```
///
///
#[inline]
pub fn id_SignKey () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-KeyUsage OID ::= { ckm-CMS 11 }
/// ```
///
///
#[inline]
pub fn id_KeyUsage () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-BitSpray OID ::= { ckm-CMS 12 }
/// ```
///
///
#[inline]
pub fn id_BitSpray () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-BitSprayMeta OID ::= { ckm-CMS 12 1 }
/// ```
///
///
#[inline]
pub fn id_BitSprayMeta () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 12, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-BitSprayShares OID ::= { ckm-CMS 12 2 }
/// ```
///
///
#[inline]
pub fn id_BitSprayShares () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 12, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-FavoriteName OID ::= { ckm-CMS 13 }
/// ```
///
///
#[inline]
pub fn id_FavoriteName () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-DataSignature OID ::= { ckm-CMS 14 }
/// ```
///
///
#[inline]
pub fn id_DataSignature () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-BlockSize OID ::= { ckm-CMS 15 }
/// ```
///
///
#[inline]
pub fn id_BlockSize () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-DataFormat OID ::= { ckm-CMS 16 }
/// ```
///
///
#[inline]
pub fn id_DataFormat () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 103, 42, 9, 10, 3, 0, 2, 2, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-tokenization-manifest OID ::= { iso(1) identified-organization(3) tc68(133)
/// country(16) x9(840) x9Standards(9) x9-73(73) tokenization(3) }
/// ```
///
///
#[inline]
pub fn id_tokenization_manifest () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* tc68 */ 133,/* country */ 16,/* x9 */ 840,/* x9Standards */ 9,/* x9-73 */ 73,/* tokenization */ 3) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-tokenizedParts OID ::= { id-tokenization-manifest tokenizedParts(0) }
/// ```
///
///
#[inline]
pub fn id_tokenizedParts () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 3, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-XPathTokensSet OID ::= { id-tokenization-manifest xPathTokensSet(1) }
/// ```
///
///
#[inline]
pub fn id_XPathTokensSet () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 3, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcryption OID ::= { iso(1) identified-organization(3) tc68(133)
/// country(16) x9(840) x9Standards(9) x9-73(73) signcryption(4) }
/// ```
///
///
#[inline]
pub fn signcryption () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* tc68 */ 133,/* country */ 16,/* x9 */ 840,/* x9Standards */ 9,/* x9-73 */ 73,/* signcryption */ 4) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcryption-mode OID ::= { signcryption modes(1) }
/// ```
///
///
#[inline]
pub fn signcryption_mode () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 4, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcrypted-content OID ::= { signcryption-mode content(1) }
/// ```
///
///
#[inline]
pub fn signcrypted_content () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 4, 1, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcrypted-attributes OID ::= { signcryption-mode attributes(2) }
/// ```
///
///
#[inline]
pub fn signcrypted_attributes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 4, 1, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcrypted-components OID ::= { signcryption-mode components(3) }
/// ```
///
///
#[inline]
pub fn signcrypted_components () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 4, 1, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// signcrypted-envelope OID ::= { signcryption-mode enveloped(4) }
/// ```
///
///
#[inline]
pub fn signcrypted_envelope () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 4, 1, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signcryptedParts OID ::= { signcryption-manifest signcryptedParts(1) }
/// ```
///
///
#[inline]
pub fn id_signcryptedParts () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 1, 3, 1, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-XPathSigncryptionSet OID ::= { signcryption-manifest xPathSigncryptionSet(2) }
/// ```
///
///
#[inline]
pub fn id_XPathSigncryptionSet () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0xce, 76, 1, 3, 1, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cms-attributes OID ::= { iso(1) identified-organization(3) tc68(133)
/// country(16) x9(840) x9Standards(9) x9-73(73) attributes(1) }
/// ```
///
///
#[inline]
pub fn id_cms_attributes () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* tc68 */ 133,/* country */ 16,/* x9 */ 840,/* x9Standards */ 9,/* x9-73 */ 73,/* attributes */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// xmlMarkup OID ::= { id-cms-attributes xml(0) }
/// ```
///
///
#[inline]
pub fn xmlMarkup () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 1, 0 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cms-SAML OID ::= { xmlMarkup saml(1) }
/// ```
///
///
#[inline]
pub fn id_cms_SAML () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 1, 0, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cms-XKMS OID ::= { xmlMarkup xkms(2) }
/// ```
///
///
#[inline]
pub fn id_cms_XKMS () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 1, 0, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-messageComponents OID ::= { xmlMarkup mc(3) }
/// ```
///
///
#[inline]
pub fn id_messageComponents () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 43, 0x81, 5, 16, 0x86, 72, 9, 73, 1, 0, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cms-namespaces OID ::= { iso(1) identified-organization(3) tc68(133)
/// country(16) x9(840) x9Standards(9) x9-73(73) namespaces(2) }
/// ```
///
///
#[inline]
pub fn id_cms_namespaces () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* tc68 */ 133,/* country */ 16,/* x9 */ 840,/* x9Standards */ 9,/* x9-73 */ 73,/* namespaces */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cms OID ::= { iso(1) identified-organization(3) tc68(133) country(16) x9(840)
/// x9Standards(9) x9-73(73) namespaces(2) cms(0) }
/// ```
///
///
#[inline]
pub fn cms () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* tc68 */ 133,/* country */ 16,/* x9 */ 840,/* x9Standards */ 9,/* x9-73 */ 73,/* namespaces */ 2,/* cms */ 0) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-alg-PWRI-KEK OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) alg(3) 9 }
/// ```
///
///
#[inline]
pub fn id_alg_PWRI_KEK () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* rsadsi */ 113549,/* pkcs */ 1,/* pkcs-9 */ 9,/* smime */ 16,/* alg */ 3,9) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dbEKM OID ::= {joint-iso-itu-t(2) country(16) us(840) organization(1) wfbna(114171)
/// lobs(4) eisArchitecture(1) techniques(2) dbEKM(0)}
/// ```
///
///
#[inline]
pub fn dbEKM () -> OBJECT_IDENTIFIER {
	oid!(/* joint-iso-itu-t */ 2,/* country */ 16,/* us */ 840,/* organization */ 1,/* wfbna */ 114171,/* lobs */ 4,/* eisArchitecture */ 1,/* techniques */ 2,/* dbEKM */ 0) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-SimpleString OID ::= { dbEKM ss(1) }
/// ```
///
///
#[inline]
pub fn id_SimpleString () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 0x86, 0xfb, 123, 4, 1, 2, 0, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-UniqueIdentifier OID ::= { dbEKM uid(2) }
/// ```
///
///
#[inline]
pub fn id_UniqueIdentifier () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 96, 0x86, 72, 1, 0x86, 0xfb, 123, 4, 1, 2, 0, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-dbekm-recip-info OID ::= { iso member-body(2) us(840) x973(10060) km(2) 3 }
/// ```
///
///
#[inline]
pub fn id_dbekm_recip_info () -> OBJECT_IDENTIFIER {
	oid!(iso,/* member-body */ 2,/* us */ 840,/* x973 */ 10060,/* km */ 2,3) // OID_GETTER
}
