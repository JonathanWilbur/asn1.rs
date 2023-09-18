#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # ProtocolObjectIdentifiers
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `ProtocolObjectIdentifiers`.
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
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-dua OBJECT IDENTIFIER ::= {id-rosObject 1}
/// ```
///
///
pub fn id_rosObject_dua() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-directory OBJECT IDENTIFIER ::= {id-rosObject 2}
/// ```
///
///
pub fn id_rosObject_directory() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-dapDSA OBJECT IDENTIFIER ::= {id-rosObject 3}
/// ```
///
///
pub fn id_rosObject_dapDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-dspDSA OBJECT IDENTIFIER ::= {id-rosObject 4}
/// ```
///
///
pub fn id_rosObject_dspDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-dopDSA OBJECT IDENTIFIER ::= {id-rosObject 7}
/// ```
///
///
pub fn id_rosObject_dopDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-initiatingConsumerDSA OBJECT IDENTIFIER ::= {id-rosObject 8}
/// ```
///
///
pub fn id_rosObject_initiatingConsumerDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-respondingSupplierDSA OBJECT IDENTIFIER ::= {id-rosObject 9}
/// ```
///
///
pub fn id_rosObject_respondingSupplierDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-initiatingSupplierDSA OBJECT IDENTIFIER ::= {id-rosObject 10}
/// ```
///
///
pub fn id_rosObject_initiatingSupplierDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject-respondingConsumerDSA OBJECT IDENTIFIER ::= {id-rosObject 11}
/// ```
///
///
pub fn id_rosObject_respondingConsumerDSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_rosObject().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract-dap OBJECT IDENTIFIER ::= {id-contract 1}
/// ```
///
///
pub fn id_contract_dap() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_contract().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract-dsp OBJECT IDENTIFIER ::= {id-contract 2}
/// ```
///
///
pub fn id_contract_dsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_contract().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract-shadowConsumer OBJECT IDENTIFIER ::= {id-contract 3}
/// ```
///
///
pub fn id_contract_shadowConsumer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_contract().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract-shadowSupplier OBJECT IDENTIFIER ::= {id-contract 4}
/// ```
///
///
pub fn id_contract_shadowSupplier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_contract().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract-dop OBJECT IDENTIFIER ::= {id-contract 5}
/// ```
///
///
pub fn id_contract_dop() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_contract().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-read OBJECT IDENTIFIER ::= {id-package 1}
/// ```
///
///
pub fn id_package_read() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-search OBJECT IDENTIFIER ::= {id-package 2}
/// ```
///
///
pub fn id_package_search() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-modify OBJECT IDENTIFIER ::= {id-package 3}
/// ```
///
///
pub fn id_package_modify() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-chainedRead OBJECT IDENTIFIER ::= {id-package 4}
/// ```
///
///
pub fn id_package_chainedRead() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-chainedSearch OBJECT IDENTIFIER ::= {id-package 5}
/// ```
///
///
pub fn id_package_chainedSearch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-chainedModify OBJECT IDENTIFIER ::= {id-package 6}
/// ```
///
///
pub fn id_package_chainedModify() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-shadowConsumer OBJECT IDENTIFIER ::= {id-package 7}
/// ```
///
///
pub fn id_package_shadowConsumer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-shadowSupplier OBJECT IDENTIFIER ::= {id-package 8}
/// ```
///
///
pub fn id_package_shadowSupplier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-operationalBindingManagement OBJECT IDENTIFIER ::= {id-package 9}
/// ```
///
///
pub fn id_package_operationalBindingManagement() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-dapConnection OBJECT IDENTIFIER ::= {id-package 10}
/// ```
///
///
pub fn id_package_dapConnection() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-dspConnection OBJECT IDENTIFIER ::= {id-package 11}
/// ```
///
///
pub fn id_package_dspConnection() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-dispConnection OBJECT IDENTIFIER ::= {id-package 12}
/// ```
///
///
pub fn id_package_dispConnection() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package-dopConnection OBJECT IDENTIFIER ::= {id-package 13}
/// ```
///
///
pub fn id_package_dopConnection() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_package().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-directoryAccessAC OBJECT IDENTIFIER ::= {id-ac 1}
/// ```
///
///
pub fn id_ac_directoryAccessAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-directorySystemAC OBJECT IDENTIFIER ::= {id-ac 2}
/// ```
///
///
pub fn id_ac_directorySystemAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-directoryOperationalBindingManagementAC OBJECT IDENTIFIER ::= {id-ac 3}
/// ```
///
///
pub fn id_ac_directoryOperationalBindingManagementAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowConsumerInitiatedAC OBJECT IDENTIFIER ::= {id-ac 4}
/// ```
///
///
pub fn id_ac_shadowConsumerInitiatedAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowSupplierInitiatedAC OBJECT IDENTIFIER ::= {id-ac 5}
/// ```
///
///
pub fn id_ac_shadowSupplierInitiatedAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-reliableShadowSupplierInitiatedAC OBJECT IDENTIFIER ::= {id-ac 6}
/// ```
///
///
pub fn id_ac_reliableShadowSupplierInitiatedAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-reliableShadowConsumerInitiatedAC OBJECT IDENTIFIER ::= {id-ac 7}
/// ```
///
///
pub fn id_ac_reliableShadowConsumerInitiatedAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowSupplierInitiatedAsynchronousAC OBJECT IDENTIFIER ::= {id-ac 8}
/// ```
///
///
pub fn id_ac_shadowSupplierInitiatedAsynchronousAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac-shadowConsumerInitiatedAsynchronousAC OBJECT IDENTIFIER ::= {id-ac 9}
/// ```
///
///
pub fn id_ac_shadowConsumerInitiatedAsynchronousAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ac().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryAccessAS OBJECT IDENTIFIER ::= {id-as 1}
/// ```
///
///
pub fn id_as_directoryAccessAS() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directorySystemAS OBJECT IDENTIFIER ::= {id-as 2}
/// ```
///
///
pub fn id_as_directorySystemAS() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryShadowAS OBJECT IDENTIFIER ::= {id-as 3}
/// ```
///
///
pub fn id_as_directoryShadowAS() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryOperationalBindingManagementAS OBJECT IDENTIFIER ::= {id-as 4}
/// ```
///
///
pub fn id_as_directoryOperationalBindingManagementAS() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-directoryReliableShadowAS OBJECT IDENTIFIER ::= {id-as 5}
/// ```
///
///
pub fn id_as_directoryReliableShadowAS() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-reliableShadowBindingAS OBJECT IDENTIFIER ::= {id-as 6}
/// ```
///
///
pub fn id_as_reliableShadowBindingAS() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as-2or3se OBJECT IDENTIFIER ::= {id-as 7}
/// ```
///
///
pub fn id_as_2or3se() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_as().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}
