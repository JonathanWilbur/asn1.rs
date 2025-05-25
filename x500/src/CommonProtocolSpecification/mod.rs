#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CommonProtocolSpecification
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CommonProtocolSpecification`.
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
use crate::OperationalBindingManagement::*;
use wildboar_asn1::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OPERATION ::= CLASS {
///   &ArgumentType   OPTIONAL,
///   &ResultType     OPTIONAL,
///   &Errors         ERROR OPTIONAL,
///   &operationCode  Code UNIQUE OPTIONAL }
/// WITH SYNTAX {
///   [ARGUMENT       &ArgumentType]
///   [RESULT         &ResultType]
///   [ERRORS         &Errors]
///   [CODE           &operationCode] }
/// ```
///
#[derive(Debug)]
pub struct OPERATION {
    pub Errors: OPTIONAL<Vec<ERROR>>,
    pub operationCode: OPTIONAL<Code>,
}
impl OPERATION {}
impl Default for OPERATION {
    fn default() -> Self {
        OPERATION {
            Errors: None,
            operationCode: None,
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ERROR ::= CLASS {
///   &ParameterType,
///   &errorCode      Code UNIQUE OPTIONAL }
/// WITH SYNTAX {
///   PARAMETER       &ParameterType
///   [CODE           &errorCode] }
/// ```
///
#[derive(Debug)]
pub struct ERROR {
    pub errorCode: OPTIONAL<Code>,
}
impl ERROR {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Code  ::=  CHOICE {
///   local   INTEGER,
///   global  OBJECT IDENTIFIER,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Code {
    local(i64),
    global(OBJECT_IDENTIFIER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Code {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Code(el)
    }
}

pub fn _decode_Code(el: &X690Element) -> ASN1Result<Code> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 2) => Ok(Code::local(BER.decode_i64(&el)?)),
        (TagClass::UNIVERSAL, 6) => Ok(Code::global(BER.decode_object_identifier(&el)?)),
        _ => Ok(Code::_unrecognized(el.clone())),
    }
}

pub fn _encode_Code(value_: &Code) -> ASN1Result<X690Element> {
    match value_ {
        Code::local(v) => BER.encode_i64(*v),
        Code::global(v) => BER.encode_object_identifier(&v),
        Code::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Code(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 2) => BER.validate_i64(&el),
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InvokeId  ::=  CHOICE {
///   present  INTEGER,
///   absent   NULL,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum InvokeId {
    present(i64),
    absent(NULL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for InvokeId {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_InvokeId(el)
    }
}

pub fn _decode_InvokeId(el: &X690Element) -> ASN1Result<InvokeId> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 2) => Ok(InvokeId::present(BER.decode_i64(&el)?)),
        (TagClass::UNIVERSAL, 5) => Ok(InvokeId::absent(BER.decode_null(&el)?)),
        _ => Ok(InvokeId::_unrecognized(el.clone())),
    }
}

pub fn _encode_InvokeId(value_: &InvokeId) -> ASN1Result<X690Element> {
    match value_ {
        InvokeId::present(v) => BER.encode_i64(*v),
        InvokeId::absent(v) => BER.encode_null(&v),
        InvokeId::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_InvokeId(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 2) => BER.validate_i64(&el),
        (TagClass::UNIVERSAL, 5) => BER.validate_null(&el),
        _ => Ok(()),
    }
}

impl PartialEq for InvokeId {

    fn eq(&self, other: &Self) -> bool {
        match self {
            InvokeId::present(iid1) => {
                match other {
                    InvokeId::present(iid2) => iid1 == iid2,
                    _ => false,
                }
            },
            InvokeId::absent(_) => {
                match other {
                    InvokeId::absent(_) => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-read                    Code ::= local:1
/// ```
///
///
pub const id_opcode_read: Code = Code::local(1);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-compare                 Code ::= local:2
/// ```
///
///
pub const id_opcode_compare: Code = Code::local(2);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-abandon                 Code ::= local:3
/// ```
///
///
pub const id_opcode_abandon: Code = Code::local(3);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-list                    Code ::= local:4
/// ```
///
///
pub const id_opcode_list: Code = Code::local(4);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-search                  Code ::= local:5
/// ```
///
///
pub const id_opcode_search: Code = Code::local(5);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-addEntry                Code ::= local:6
/// ```
///
///
pub const id_opcode_addEntry: Code = Code::local(6);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-removeEntry             Code ::= local:7
/// ```
///
///
pub const id_opcode_removeEntry: Code = Code::local(7);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-modifyEntry             Code ::= local:8
/// ```
///
///
pub const id_opcode_modifyEntry: Code = Code::local(8);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-modifyDN                Code ::= local:9
/// ```
///
///
pub const id_opcode_modifyDN: Code = Code::local(9);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-changePassword          Code ::= local:10
/// ```
///
///
pub const id_opcode_changePassword: Code = Code::local(10);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-administerPassword      Code ::= local:11
/// ```
///
///
pub const id_opcode_administerPassword: Code = Code::local(11);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-ldapTransport           Code ::= local:12
/// ```
///
///
pub const id_opcode_ldapTransport: Code = Code::local(12);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-linkedLDAP              Code ::= local:13
/// ```
///
///
pub const id_opcode_linkedLDAP: Code = Code::local(13);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-requestShadowUpdate     Code ::= local:1
/// ```
///
///
pub const id_opcode_requestShadowUpdate: Code = Code::local(1);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-updateShadow            Code ::= local:2
/// ```
///
///
pub const id_opcode_updateShadow: Code = Code::local(2);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-opcode-coordinateShadowUpdate  Code ::= local:3
/// ```
///
///
pub const id_opcode_coordinateShadowUpdate: Code = Code::local(3);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-op-establishOperationalBinding Code ::= local:100
/// ```
///
///
pub const id_op_establishOperationalBinding: Code = Code::local(100);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-op-modifyOperationalBinding    Code ::= local:102
/// ```
///
///
pub const id_op_modifyOperationalBinding: Code = Code::local(102);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-op-terminateOperationalBinding Code ::= local:101
/// ```
///
///
pub const id_op_terminateOperationalBinding: Code = Code::local(101);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-attributeError         Code ::= local:1
/// ```
///
///
pub const id_errcode_attributeError: Code = Code::local(1);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-nameError              Code ::= local:2
/// ```
///
///
pub const id_errcode_nameError: Code = Code::local(2);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-serviceError           Code ::= local:3
/// ```
///
///
pub const id_errcode_serviceError: Code = Code::local(3);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-referral               Code ::= local:4
/// ```
///
///
pub const id_errcode_referral: Code = Code::local(4);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-abandoned              Code ::= local:5
/// ```
///
///
pub const id_errcode_abandoned: Code = Code::local(5);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-securityError          Code ::= local:6
/// ```
///
///
pub const id_errcode_securityError: Code = Code::local(6);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-abandonFailed          Code ::= local:7
/// ```
///
///
pub const id_errcode_abandonFailed: Code = Code::local(7);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-updateError            Code ::= local:8
/// ```
///
///
pub const id_errcode_updateError: Code = Code::local(8);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-dsaReferral            Code ::= local:9
/// ```
///
///
pub const id_errcode_dsaReferral: Code = Code::local(9);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-errcode-shadowError            Code ::= local:1
/// ```
///
///
pub const id_errcode_shadowError: Code = Code::local(1);

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-err-operationalBindingError    Code ::= local:100
/// ```
///
///
pub const id_err_operationalBindingError: Code = Code::local(100);

/// ### ASN.1 Definition:
///
/// ```asn1
/// DOP-Invokable OPERATION ::= {establishOperationalBinding |
///    modifyOperationalBinding |
///    terminateOperationalBinding}
/// ```
///
///
pub fn DOP_Invokable() -> Vec<OPERATION> {
    Vec::from([
        establishOperationalBinding(),
        modifyOperationalBinding(),
        terminateOperationalBinding(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DOP-Returnable OPERATION ::= {establishOperationalBinding |
///    modifyOperationalBinding |
///    terminateOperationalBinding}
/// ```
///
///
pub fn DOP_Returnable() -> Vec<OPERATION> {
    Vec::from([
        establishOperationalBinding(),
        modifyOperationalBinding(),
        terminateOperationalBinding(),
    ])
}
