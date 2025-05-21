#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DirectoryOSIProtocols
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DirectoryOSIProtocols`.
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
use crate::OSIProtocolSpecification::*;
use crate::OperationalBindingManagement::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// DAP-OSI-PDUs  ::=  OSI-PDU{directoryAccessAC}
/// ```
pub type DAP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_DAP_OSI_PDUs(el: &X690Element) -> ASN1Result<DAP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_DAP_OSI_PDUs(value_: &DAP_OSI_PDUs) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_DAP_OSI_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DSP-OSI-PDUs  ::=  OSI-PDU{directorySystemAC}
/// ```
pub type DSP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_DSP_OSI_PDUs(el: &X690Element) -> ASN1Result<DSP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_DSP_OSI_PDUs(value_: &DSP_OSI_PDUs) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_DSP_OSI_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DOP-OSI-PDUs  ::=  OSI-PDU{directoryOperationalBindingManagementAC}
/// ```
pub type DOP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_DOP_OSI_PDUs(el: &X690Element) -> ASN1Result<DOP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_DOP_OSI_PDUs(value_: &DOP_OSI_PDUs) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_DOP_OSI_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowSupplierInitiatedDISP-OSI-PDUs  ::=  OSI-PDU{shadowSupplierInitiatedAC}
/// ```
pub type ShadowSupplierInitiatedDISP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_ShadowSupplierInitiatedDISP_OSI_PDUs(
    el: &X690Element,
) -> ASN1Result<ShadowSupplierInitiatedDISP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_ShadowSupplierInitiatedDISP_OSI_PDUs(
    value_: &ShadowSupplierInitiatedDISP_OSI_PDUs,
) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_ShadowSupplierInitiatedDISP_OSI_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowSupplierInitiatedAsynchronousDISP-OSI-PDUs  ::=
///   OSI-PDU{shadowSupplierInitiatedAsynchronousAC}
/// ```
pub type ShadowSupplierInitiatedAsynchronousDISP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_ShadowSupplierInitiatedAsynchronousDISP_OSI_PDUs(
    el: &X690Element,
) -> ASN1Result<ShadowSupplierInitiatedAsynchronousDISP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_ShadowSupplierInitiatedAsynchronousDISP_OSI_PDUs(
    value_: &ShadowSupplierInitiatedAsynchronousDISP_OSI_PDUs,
) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_ShadowSupplierInitiatedAsynchronousDISP_OSI_PDUs(
    el: &X690Element,
) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowConsumerInitiatedDISP-OSI-PDUs  ::=  OSI-PDU{shadowConsumerInitiatedAC}
/// ```
pub type ShadowConsumerInitiatedDISP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_ShadowConsumerInitiatedDISP_OSI_PDUs(
    el: &X690Element,
) -> ASN1Result<ShadowConsumerInitiatedDISP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_ShadowConsumerInitiatedDISP_OSI_PDUs(
    value_: &ShadowConsumerInitiatedDISP_OSI_PDUs,
) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_ShadowConsumerInitiatedDISP_OSI_PDUs(el: &X690Element) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ShadowConsumerInitiatedAsynchronousDISP-OSI-PDUs  ::=
///   OSI-PDU{shadowConsumerInitiatedAsynchronousAC}
/// ```
pub type ShadowConsumerInitiatedAsynchronousDISP_OSI_PDUs = OSI_PDU; // DefinedType

pub fn _decode_ShadowConsumerInitiatedAsynchronousDISP_OSI_PDUs(
    el: &X690Element,
) -> ASN1Result<ShadowConsumerInitiatedAsynchronousDISP_OSI_PDUs> {
    _decode_OSI_PDU(&el)
}

pub fn _encode_ShadowConsumerInitiatedAsynchronousDISP_OSI_PDUs(
    value_: &ShadowConsumerInitiatedAsynchronousDISP_OSI_PDUs,
) -> ASN1Result<X690Element> {
    _encode_OSI_PDU(&value_)
}

pub fn _validate_ShadowConsumerInitiatedAsynchronousDISP_OSI_PDUs(
    el: &X690Element,
) -> ASN1Result<()> {
    _validate_OSI_PDU(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// APPLICATION-CONTEXT ::= CLASS {
///   &bind-operation          OPERATION,
///   &Operations              OPERATION,
///   &applicationContextName  OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   BIND-OPERATION &bind-operation
///   OPERATIONS &Operations
///   APPLICATION CONTEXT NAME &applicationContextName }
/// ```
///
#[derive(Debug)]
pub struct APPLICATION_CONTEXT {
    pub bind_operation: OPERATION,
    pub Operations: Vec<OPERATION>,
    pub applicationContextName: OBJECT_IDENTIFIER,
}
impl APPLICATION_CONTEXT {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryAccessAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            directoryBind
///   OPERATIONS                {read |
///                              compare |
///                              abandon |
///                              list |
///                              search |
///                              addEntry |
///                              removeEntry |
///                              modifyEntry |
///                              modifyDN |
///                              administerPassword |
///                              changePassword }
///   APPLICATION CONTEXT NAME  id-ac-directoryAccessAC }
/// ```
///
///
pub fn directoryAccessAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
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
        applicationContextName: id_ac_directoryAccessAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod directoryAccessAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directorySystemAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            dSABind
///   OPERATIONS                {chainedRead |
///                              chainedCompare |
///                              chainedAbandon |
///                              chainedList |
///                              chainedSearch |
///                              chainedAddEntry |
///                              chainedRemoveEntry |
///                              chainedModifyEntry |
///                              chainedModifyDN |
///                              chainedAdministerPassword |
///                              chainedChangePassword |
///                              chainedLdapTransport |
///                              chainedLinkedLDAP }
///   APPLICATION CONTEXT NAME  id-ac-directorySystemAC }
/// ```
///
///
pub fn directorySystemAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
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
        applicationContextName: id_ac_directorySystemAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod directorySystemAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowSupplierInitiatedAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            dSAShadowBind
///   OPERATIONS                {updateShadow |
///                              coordinateShadowUpdate}
///   APPLICATION CONTEXT NAME  id-ac-shadowSupplierInitiatedAC }
/// ```
///
///
pub fn shadowSupplierInitiatedAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
        bind_operation: dSAShadowBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([updateShadow(), coordinateShadowUpdate()]), /* OBJECT_FIELD_SETTING */
        applicationContextName: id_ac_shadowSupplierInitiatedAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shadowSupplierInitiatedAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowConsumerInitiatedAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            dSAShadowBind
///   OPERATIONS                {requestShadowUpdate |
///                              updateShadow}
///   APPLICATION CONTEXT NAME  id-ac-shadowConsumerInitiatedAC }
/// ```
///
///
pub fn shadowConsumerInitiatedAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
        bind_operation: dSAShadowBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([requestShadowUpdate(), updateShadow()]), /* OBJECT_FIELD_SETTING */
        applicationContextName: id_ac_shadowConsumerInitiatedAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shadowConsumerInitiatedAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowSupplierInitiatedAsynchronousAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            dSAShadowBind
///   OPERATIONS                {updateShadow |
///                              coordinateShadowUpdate}
///   APPLICATION CONTEXT NAME  id-ac-shadowSupplierInitiatedAsynchronousAC }
/// ```
///
///
pub fn shadowSupplierInitiatedAsynchronousAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
        bind_operation: dSAShadowBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([updateShadow(), coordinateShadowUpdate()]), /* OBJECT_FIELD_SETTING */
        applicationContextName: id_ac_shadowSupplierInitiatedAsynchronousAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shadowSupplierInitiatedAsynchronousAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// shadowConsumerInitiatedAsynchronousAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            dSAShadowBind
///   OPERATIONS                {requestShadowUpdate |
///                              updateShadow}
///   APPLICATION CONTEXT NAME  id-ac-shadowConsumerInitiatedAsynchronousAC }
/// ```
///
///
pub fn shadowConsumerInitiatedAsynchronousAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
        bind_operation: dSAShadowBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([requestShadowUpdate(), updateShadow()]), /* OBJECT_FIELD_SETTING */
        applicationContextName: id_ac_shadowConsumerInitiatedAsynchronousAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod shadowConsumerInitiatedAsynchronousAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryOperationalBindingManagementAC APPLICATION-CONTEXT ::= {
///   BIND-OPERATION            dSAOperationalBindingManagementBind
///   OPERATIONS                {establishOperationalBinding |
///                              modifyOperationalBinding |
///                              terminateOperationalBinding}
///   APPLICATION CONTEXT NAME  id-ac-directoryOperationalBindingManagementAC }
/// ```
///
///
pub fn directoryOperationalBindingManagementAC() -> APPLICATION_CONTEXT {
    APPLICATION_CONTEXT {
        bind_operation: dSAOperationalBindingManagementBind(), /* OBJECT_FIELD_SETTING */
        Operations: Vec::from([
            establishOperationalBinding(),
            modifyOperationalBinding(),
            terminateOperationalBinding(),
        ]), /* OBJECT_FIELD_SETTING */
        applicationContextName: id_ac_directoryOperationalBindingManagementAC(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod directoryOperationalBindingManagementAC {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryAccessAS                       OBJECT IDENTIFIER ::= {id-as 1}
/// ```
///
///
#[inline]
pub fn id_as_directoryAccessAS () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_as(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directorySystemAS                       OBJECT IDENTIFIER ::= {id-as 2}
/// ```
///
///
#[inline]
pub fn id_as_directorySystemAS () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_as(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryShadowAS                       OBJECT IDENTIFIER ::= {id-as 3}
/// ```
///
///
#[inline]
pub fn id_as_directoryShadowAS () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_as(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryOperationalBindingManagementAS OBJECT IDENTIFIER ::= {id-as 4}
/// ```
///
///
#[inline]
pub fn id_as_directoryOperationalBindingManagementAS () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_as(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-acseAS                                     OBJECT IDENTIFIER ::= {joint-iso-itu-t association-control(2) abstract-syntax(1) apdus(0) version(1)}
/// ```
///
///
#[inline]
pub fn id_acseAS () -> OBJECT_IDENTIFIER {
	oid!(joint_iso_itu_t,/* association-control */ 2,/* abstract-syntax */ 1,/* apdus */ 0,/* version */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-directoryAccessAC                       OBJECT IDENTIFIER ::= {id-ac 1}
/// ```
///
///
#[inline]
pub fn id_ac_directoryAccessAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-directorySystemAC                       OBJECT IDENTIFIER ::= {id-ac 2}
/// ```
///
///
#[inline]
pub fn id_ac_directorySystemAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-directoryOperationalBindingManagementAC OBJECT IDENTIFIER ::= {id-ac 3}
/// ```
///
///
#[inline]
pub fn id_ac_directoryOperationalBindingManagementAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowConsumerInitiatedAC               OBJECT IDENTIFIER ::= {id-ac 4}
/// ```
///
///
#[inline]
pub fn id_ac_shadowConsumerInitiatedAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowSupplierInitiatedAC               OBJECT IDENTIFIER ::= {id-ac 5}
/// ```
///
///
#[inline]
pub fn id_ac_shadowSupplierInitiatedAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowSupplierInitiatedAsynchronousAC   OBJECT IDENTIFIER ::= {id-ac 8}
/// ```
///
///
#[inline]
pub fn id_ac_shadowSupplierInitiatedAsynchronousAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 8).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowConsumerInitiatedAsynchronousAC   OBJECT IDENTIFIER ::= {id-ac 9}
/// ```
///
///
#[inline]
pub fn id_ac_shadowConsumerInitiatedAsynchronousAC () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ac(), 9).unwrap() // OID_GETTER
}
