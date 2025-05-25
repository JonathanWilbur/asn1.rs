#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # SecureMimeMessageV3dot1-2009
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `SecureMimeMessageV3dot1-2009`.
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
use crate::CryptographicMessageSyntax::*;
use crate::CryptographicMessageSyntaxAlgorithms_2009::*;
use wildboar_asn1::*;
use x500::InformationFramework::ATTRIBUTE;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMimeAttributeSet ATTRIBUTE ::= { aa-smimeCapabilities | aa-encrypKeyPref,
/// ...
/// }
/// ```
///
///
pub fn SMimeAttributeSet() -> Vec<ATTRIBUTE> {
    Vec::from([aa_smimeCapabilities(), aa_encrypKeyPref()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aa OBJECT IDENTIFIER ::= { iso(1) member-body(2) usa(840) rsadsi(113549) pkcs(1) pkcs-9(9)
/// smime(16) attributes(2)}
/// ```
///
///
pub fn id_aa() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* usa */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* attributes */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-smimeCapabilities ATTRIBUTE ::= {
/// TYPE SMIMECapabilities{{SMimeCapsSet}}
/// IDENTIFIED BY smimeCapabilities }
/// ```
///
///
pub fn aa_smimeCapabilities() -> ATTRIBUTE {
    ATTRIBUTE {
        id: smimeCapabilities(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod aa_smimeCapabilities {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SMIMECapabilities; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SMIMECapabilities(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SMIMECapabilities(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SMIMECapabilities(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// smimeCapabilities OBJECT IDENTIFIER ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9) 15 }
/// ```
///
///
pub fn smimeCapabilities() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, 15) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMimeCapsSet SMIME-CAPS ::= { cap-preferBinaryInside | cap-RC2CBC |
/// PKIXAlgs-2009.SMimeCaps |
/// CryptographicMessageSyntaxAlgorithms-2009.SMimeCaps |
/// PKIX1-PSS-OAEP-Algorithms-2009.SMimeCaps,
/// ...
/// }
/// ```
///
///
pub fn SMimeCapsSet() -> Vec<SMIME_CAPS> {
    [
        crate::PKIXAlgs_2009::SMimeCaps().as_slice(),
        crate::CryptographicMessageSyntaxAlgorithms_2009::SMimeCaps().as_slice(),
        crate::PKIX1_PSS_OAEP_Algorithms_2009::SMimeCaps().as_slice(),
        Vec::from([cap_preferBinaryInside(), cap_RC2CBC()]).as_slice(),
    ]
    .concat()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aa-encrypKeyPref ATTRIBUTE ::= {
/// TYPE SMIMEEncryptionKeyPreference
/// IDENTIFIED BY id-aa-encrypKeyPref }
/// ```
///
///
pub fn aa_encrypKeyPref() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_aa_encrypKeyPref(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
    }
}

pub mod aa_encrypKeyPref {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SMIMEEncryptionKeyPreference; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SMIMEEncryptionKeyPreference(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SMIMEEncryptionKeyPreference(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SMIMEEncryptionKeyPreference(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aa-encrypKeyPref OBJECT IDENTIFIER ::= {id-aa 11}
/// ```
///
#[inline]
pub fn id_aa_encrypKeyPref () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 16, 2, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMIMEEncryptionKeyPreference  ::=  CHOICE {
/// issuerAndSerialNumber	[0] IssuerAndSerialNumber,
/// receipentKeyId		[1] RecipientKeyIdentifier,
/// subjectAltKeyIdentifier	[2] SubjectKeyIdentifier
/// }
/// ```
#[derive(Debug, Clone)]
pub enum SMIMEEncryptionKeyPreference {
    issuerAndSerialNumber(IssuerAndSerialNumber),
    receipentKeyId(RecipientKeyIdentifier),
    subjectAltKeyIdentifier(SubjectKeyIdentifier),
}

impl TryFrom<&X690Element> for SMIMEEncryptionKeyPreference {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SMIMEEncryptionKeyPreference(el)
    }
}

pub fn _decode_SMIMEEncryptionKeyPreference(
    el: &X690Element,
) -> ASN1Result<SMIMEEncryptionKeyPreference> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(SMIMEEncryptionKeyPreference::issuerAndSerialNumber(
            _decode_IssuerAndSerialNumber(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(SMIMEEncryptionKeyPreference::receipentKeyId(
            _decode_RecipientKeyIdentifier(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(SMIMEEncryptionKeyPreference::subjectAltKeyIdentifier(
            _decode_SubjectKeyIdentifier(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "SMIMEEncryptionKeyPreference",
            ))
        }
    }
}

pub fn _encode_SMIMEEncryptionKeyPreference(
    value_: &SMIMEEncryptionKeyPreference,
) -> ASN1Result<X690Element> {
    match value_ {
        SMIMEEncryptionKeyPreference::issuerAndSerialNumber(v) => {
            |v_1: &IssuerAndSerialNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_IssuerAndSerialNumber(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        SMIMEEncryptionKeyPreference::receipentKeyId(v) => {
            |v_1: &RecipientKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_RecipientKeyIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        SMIMEEncryptionKeyPreference::subjectAltKeyIdentifier(v) => {
            |v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 2;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_SMIMEEncryptionKeyPreference(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "issuerAndSerialNumber",
                ));
            }
            Ok(_validate_IssuerAndSerialNumber(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "receipentKeyId")
                );
            }
            Ok(_validate_RecipientKeyIdentifier(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "subjectAltKeyIdentifier",
                ));
            }
            Ok(_validate_SubjectKeyIdentifier(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "SMIMEEncryptionKeyPreference",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-smime OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs9(9) 16 }
/// ```
///
///
pub fn id_smime() -> OBJECT_IDENTIFIER {
    oid!(
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs9 */ 9, 16) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cap  OBJECT IDENTIFIER ::= { id-smime 11 }
/// ```
///
///
#[inline]
pub fn id_cap () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 16, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cap-preferBinaryInside SMIME-CAPS ::= {
/// -- No value 
/// IDENTIFIED BY id-cap-preferBinaryInside }
/// ```
///
///
pub fn cap_preferBinaryInside() -> SMIME_CAPS {
    SMIME_CAPS {
        id: id_cap_preferBinaryInside(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod cap_preferBinaryInside {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cap-preferBinaryInside  OBJECT IDENTIFIER ::= { id-cap 1 }
/// ```
///
///
#[inline]
pub fn id_cap_preferBinaryInside () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 42, 0x86, 72, 0x86, 0xf7, 13, 1, 9, 16, 11, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cap-RC2CBC SMIME-CAPS ::= {
/// TYPE SMIMECapabilitiesParametersForRC2CBC
/// IDENTIFIED BY rc2-cbc}
/// ```
///
///
pub fn cap_RC2CBC() -> SMIME_CAPS {
    SMIME_CAPS {
        id: rc2_cbc(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod cap_RC2CBC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SMIMECapabilitiesParametersForRC2CBC; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SMIMECapabilitiesParametersForRC2CBC(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SMIMECapabilitiesParametersForRC2CBC(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SMIMECapabilitiesParametersForRC2CBC(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMIMECapabilitiesParametersForRC2CBC  ::=  INTEGER (40 | 128, ...)
/// ```
pub type SMIMECapabilitiesParametersForRC2CBC = INTEGER;

pub fn _decode_SMIMECapabilitiesParametersForRC2CBC(
    el: &X690Element,
) -> ASN1Result<SMIMECapabilitiesParametersForRC2CBC> {
    BER.decode_integer(&el)
}

pub fn _encode_SMIMECapabilitiesParametersForRC2CBC(
    value_: &SMIMECapabilitiesParametersForRC2CBC,
) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_SMIMECapabilitiesParametersForRC2CBC(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}
