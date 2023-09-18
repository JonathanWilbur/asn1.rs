#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DirectoryIDMProtocols
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DirectoryIDMProtocols`.
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
use crate::CommonProtocolSpecification::*;
use crate::DirectoryAbstractService::*;
use crate::DirectoryShadowAbstractService::*;
use crate::DistributedOperations::*;
use crate::IDMProtocolSpecification::*;
use crate::OperationalBindingManagement::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// DAP-IDM-PDUs  ::=  IDM-PDU{dap-ip}
/// ```
pub type DAP_IDM_PDUs = IDM_PDU; // DefinedType

pub fn _decode_DAP_IDM_PDUs(el: &X690Element) -> ASN1Result<DAP_IDM_PDUs> {
    _decode_IDM_PDU(&el)
}

pub fn _encode_DAP_IDM_PDUs(value_: &DAP_IDM_PDUs) -> ASN1Result<X690Element> {
    _encode_IDM_PDU(&value_)
}

pub fn _validate_DAP_IDM_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_IDM_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dap-ip IDM-PROTOCOL ::= {
///   BIND-OPERATION  directoryBind
///   OPERATIONS      {read |
///                    compare |
///                    abandon |
///                    list |
///                    search |
///                    addEntry |
///                    removeEntry |
///                    modifyEntry |
///                    modifyDN |
///                    administerPassword |
///                    changePassword }
///   ID              id-idm-dap }
/// ```
///
///
pub fn dap_ip() -> IDM_PROTOCOL {
    IDM_PROTOCOL {
        bind_operation: directoryBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([
            read(),
            compare(),
            abandon(),
            list(),
            search(),
            addEntry(),
            removeEntry(),
            modifyEntry(),
            modifyDN(),
            administerPassword(),
            changePassword(),
        ]), /* OBJECT_FIELD_SETTING */
        id: id_idm_dap(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod dap_ip {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSP-IDM-PDUs  ::=  IDM-PDU{dsp-ip}
/// ```
pub type DSP_IDM_PDUs = IDM_PDU; // DefinedType

pub fn _decode_DSP_IDM_PDUs(el: &X690Element) -> ASN1Result<DSP_IDM_PDUs> {
    _decode_IDM_PDU(&el)
}

pub fn _encode_DSP_IDM_PDUs(value_: &DSP_IDM_PDUs) -> ASN1Result<X690Element> {
    _encode_IDM_PDU(&value_)
}

pub fn _validate_DSP_IDM_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_IDM_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsp-ip IDM-PROTOCOL ::= {
///   BIND-OPERATION  dSABind
///   OPERATIONS      {chainedRead |
///                    chainedCompare |
///                    chainedAbandon |
///                    chainedList |
///                    chainedSearch |
///                    chainedAddEntry |
///                    chainedRemoveEntry |
///                    chainedModifyEntry |
///                    chainedModifyDN |
///                    chainedAdministerPassword |
///                    chainedChangePassword |
///                    chainedLdapTransport |
///                    chainedLinkedLDAP }
///   ID              id-idm-dsp }
/// ```
///
///
pub fn dsp_ip() -> IDM_PROTOCOL {
    IDM_PROTOCOL {
        bind_operation: dSABind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([
            chainedRead(),
            chainedCompare(),
            chainedAbandon(),
            chainedList(),
            chainedSearch(),
            chainedAddEntry(),
            chainedRemoveEntry(),
            chainedModifyEntry(),
            chainedModifyDN(),
            chainedAdministerPassword(),
            chainedChangePassword(),
            chainedLdapTransport(),
            chainedLinkedLDAP(),
        ]), /* OBJECT_FIELD_SETTING */
        id: id_idm_dsp(),          /* OBJECT_FIELD_SETTING */
    }
}

pub mod dsp_ip {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DISP-IDM-PDUs  ::=  IDM-PDU{disp-ip}
/// ```
pub type DISP_IDM_PDUs = IDM_PDU; // DefinedType

pub fn _decode_DISP_IDM_PDUs(el: &X690Element) -> ASN1Result<DISP_IDM_PDUs> {
    _decode_IDM_PDU(&el)
}

pub fn _encode_DISP_IDM_PDUs(value_: &DISP_IDM_PDUs) -> ASN1Result<X690Element> {
    _encode_IDM_PDU(&value_)
}

pub fn _validate_DISP_IDM_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_IDM_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// disp-ip IDM-PROTOCOL ::= {
///   BIND-OPERATION  dSAShadowBind
///   OPERATIONS      {requestShadowUpdate |
///                    updateShadow |
///                    coordinateShadowUpdate}
///   ID              id-idm-disp }
/// ```
///
///
pub fn disp_ip() -> IDM_PROTOCOL {
    IDM_PROTOCOL {
        bind_operation: dSAShadowBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([
            requestShadowUpdate(),
            updateShadow(),
            coordinateShadowUpdate(),
        ]), /* OBJECT_FIELD_SETTING */
        id: id_idm_disp(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod disp_ip {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DOP-IDM-PDUs  ::=  IDM-PDU{dop-ip}
/// ```
pub type DOP_IDM_PDUs = IDM_PDU; // DefinedType

pub fn _decode_DOP_IDM_PDUs(el: &X690Element) -> ASN1Result<DOP_IDM_PDUs> {
    _decode_IDM_PDU(&el)
}

pub fn _encode_DOP_IDM_PDUs(value_: &DOP_IDM_PDUs) -> ASN1Result<X690Element> {
    _encode_IDM_PDU(&value_)
}

pub fn _validate_DOP_IDM_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_IDM_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dop-ip IDM-PROTOCOL ::= {
///   BIND-OPERATION  dSAOperationalBindingManagementBind
///   OPERATIONS      {establishOperationalBinding |
///                    modifyOperationalBinding |
///                    terminateOperationalBinding}
///   ID              id-idm-dop }
/// ```
///
///
pub fn dop_ip() -> IDM_PROTOCOL {
    IDM_PROTOCOL {
        bind_operation: dSAOperationalBindingManagementBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([
            establishOperationalBinding(),
            modifyOperationalBinding(),
            terminateOperationalBinding(),
        ]), /* OBJECT_FIELD_SETTING */
        id: id_idm_dop(),                                      /* OBJECT_FIELD_SETTING */
    }
}

pub mod dop_ip {
    /* OBJECT_TYPES */
    use super::*;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-idm-dap  OBJECT IDENTIFIER ::= {id-idm 0}
/// ```
///
///
pub fn id_idm_dap() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_idm().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-idm-dsp  OBJECT IDENTIFIER ::= {id-idm 1}
/// ```
///
///
pub fn id_idm_dsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_idm().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-idm-disp OBJECT IDENTIFIER ::= {id-idm 2}
/// ```
///
///
pub fn id_idm_disp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_idm().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-idm-dop  OBJECT IDENTIFIER ::= {id-idm 3}
/// ```
///
///
pub fn id_idm_dop() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_idm().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}
