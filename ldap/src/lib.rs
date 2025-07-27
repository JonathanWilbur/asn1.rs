#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
pub mod ldap_asn1;
pub use ldap_asn1::*;
use wildboar_asn1::{oid, OBJECT_IDENTIFIER};

pub const success: usize = 0;
pub const operationsError: usize = 1;
pub const protocolError: usize = 2;
pub const timeLimitExceeded: usize = 3;
pub const sizeLimitExceeded: usize = 4;
pub const compareFalse: usize = 5;
pub const compareTrue: usize = 6;
pub const authMethodNotSupported: usize = 7;
pub const strongerAuthRequired: usize = 8;
pub const partialResults: usize = 9;
pub const referral: usize = 10;
pub const adminLimitExceeded: usize = 11;
pub const unavailableCriticalExtension: usize = 12;
pub const confidentialityRequired: usize = 13;
pub const saslBindInProgress: usize = 14;
pub const noSuchAttribute: usize = 16;
pub const undefinedAttributeType: usize = 17;
pub const inappropriateMatching: usize = 18;
pub const constraintViolation: usize = 19;
pub const attributeOrValueExists: usize = 20;
pub const invalidAttributeSyntax: usize = 21;
pub const noSuchObject: usize = 32;
pub const aliasProblem: usize = 33;
pub const invalidDNSyntax: usize = 34;
pub const isLeaf: usize = 35;
pub const aliasDereferencingProblem: usize = 36;
pub const inappropriateAuthentication: usize = 48;
pub const invalidCredentials: usize = 49;
pub const insufficientAccessRights: usize = 50;
pub const busy: usize = 51;
pub const unavailable: usize = 52;
pub const unwillingToPerform: usize = 53;
pub const loopDetect: usize = 54;
pub const namingViolation: usize = 64;
pub const objectClassViolation: usize = 65;
pub const notAllowedOnNonLeaf: usize = 66;
pub const notAllowedOnRDN: usize = 67;
pub const entryAlreadyExists: usize = 68;
pub const objectClassModsProhibited: usize = 69;
pub const resultsTooLarge: usize = 70;
pub const other: usize = 80;
pub const lcupResourcesExhausted: usize = 113;
pub const lcupSecurityViolation: usize = 114;
pub const lcupInvalidData: usize = 115;
pub const lcupUnsupportedScheme: usize = 116;
pub const lcupReloadRequired: usize = 117;
pub const canceled: usize = 118;
pub const noSuchOperation: usize = 119;
pub const tooLate: usize = 120;
pub const cannotCancel: usize = 121;
pub const assertionFailed: usize = 122;
pub const authorizationDenied: usize = 123;

/// Matched Values Control [RFC3876]
#[inline]
pub fn matchedValues() -> OBJECT_IDENTIFIER { oid!(1,2,826,0,1,3344810,2,3) }

/// LDAP Simple Paged Results Control [RFC2696]
#[inline]
pub fn simpledPagedResults() -> OBJECT_IDENTIFIER { oid!(1,2,840,113556,1,4,319) }

/// Sort Request [RFC2891]
#[inline]
pub fn sortRequest() -> OBJECT_IDENTIFIER { oid!(1,2,840,113556,1,4,473) }

/// Sort Response [RFC2891]
#[inline]
pub fn sortResponse() -> OBJECT_IDENTIFIER { oid!(1,2,840,113556,1,4,474) }

/// LCUP Sync Request Control [RFC3928]
#[inline]
pub fn lcupSyncRequest() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,7,1) }

/// LCUP Sync Update Control [RFC3928]
#[inline]
pub fn lcupSyncUpdate() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,7,2) }

/// LCUP Sync Done Control [RFC3928]
#[inline]
pub fn lcupSyncDone() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,7,3) }

/// Assertion Control [RFC4528]
#[inline]
pub fn assertion() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,12) }

/// LDAP Pre-read Control [RFC4527]
#[inline]
pub fn preread() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,13,1) }

/// LDAP Post-read Control [RFC4527]
#[inline]
pub fn postread() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,13,2) }

/// Transaction Specification Control [RFC5805]
#[inline]
pub fn transactionSpecification() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,21,2) }

/// LDAP Content Synchronization Control [RFC4533]
#[inline]
pub fn contentSynchronization() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,4,1,4203,1,9,1,1) }

/// Subentries [RFC3672]
#[inline]
pub fn subentries() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,4,1,4203,1,10,1) }

/// ManageDsaIT [RFC3296]
#[inline]
pub fn managedDSAIT() -> OBJECT_IDENTIFIER { oid!(2,16,840,1,113730,3,4,2) }

/// Authorization Identity Response Control [RFC3829]
#[inline]
pub fn authorizationIdentityResponse() -> OBJECT_IDENTIFIER { oid!(2,16,840,1,113730,3,4,15) }

/// Authorization Identity Request Control [RFC3829]
#[inline]
pub fn authorizationIdentityRequest() -> OBJECT_IDENTIFIER { oid!(2,16,840,1,113730,3,4,16) }

/// Proxy Authorization Control [RFC4370]
#[inline]
pub fn proxyAuthorization() -> OBJECT_IDENTIFIER { oid!(2,16,840,1,113730,3,4,18) }

/// LDAP Don't Use Copy Control [RFC6171]
#[inline]
pub fn dontUseCopy() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,22) }

/// IANA-Registered LDAP Controls as of 2025
pub const IANA_REGISTERED_CONTROLS_BY_2025: [fn() -> OBJECT_IDENTIFIER; 18] = [
    matchedValues,
    simpledPagedResults,
    sortRequest,
    sortResponse,
    lcupSyncRequest,
    lcupSyncUpdate,
    lcupSyncDone,
    assertion,
    preread,
    postread,
    transactionSpecification,
    contentSynchronization,
    subentries,
    managedDSAIT,
    authorizationIdentityResponse,
    authorizationIdentityRequest,
    proxyAuthorization,
    dontUseCopy,
];

/// Cancel Operation [RFC3909]
#[inline]
pub fn cancel() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,8) }

/// Start Transaction Extended Request [RFC5805]
#[inline]
pub fn startTransactionExtendedRequest() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,21,1) }

/// End Transaction Extended Request [RFC5805]
#[inline]
pub fn endTransactionExtendedRequest() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,21,3) }

/// Dynamic Refresh [RFC2589]
#[inline]
pub fn dynamicRefresh() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,4,1,1466,101,119,1) }

/// StartTLS [RFC4511][RFC4513]
#[inline]
pub fn startTLS() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,4,1,1466,20037) }

/// Modify Password [RFC3062]
#[inline]
pub fn modifyPassword() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,4,1,4203,1,11,1) }

/// Who am I? [RFC4532]
#[inline]
pub fn whoAmI() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,4,1,4203,1,11,3) }

/// StartLBURPRequest LDAP ExtendedRequest message [RFC4373]
#[inline]
pub fn startLBURPRequest() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,17,1) }

/// StartLBURPResponse LDAP ExtendedResponse message [RFC4373]
#[inline]
pub fn startLBURPResponse() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,17,2) }

/// EndLBURPRequest LDAP ExtendedRequest message [RFC4373]
#[inline]
pub fn endLBURPRequest() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,17,3) }

/// EndLBURPResponse LDAP ExtendedResponse message [RFC4373]
#[inline]
pub fn endLBURPResponse() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,17,4) }

/// LBURPUpdateRequest LDAP ExtendedRequest message [RFC4373]
#[inline]
pub fn lburpUpdateRequest() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,17,5) }

/// LBURPUpdateResponse LDAP ExtendedResponse message [RFC4373]
#[inline]
pub fn lburpUpdateResponse() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,17,6) }

/// LDAP Turn Operation [RFC4531]
#[inline]
pub fn turn() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,19) }

/// IANA-Registered LDAP Extensions as of 2025
pub const IANA_REGISTERED_EXTENSIONS_BY_2025: [fn() -> OBJECT_IDENTIFIER; 14] = [
    cancel,
    startTransactionExtendedRequest,
    endTransactionExtendedRequest,
    dynamicRefresh,
    startTLS,
    modifyPassword,
    whoAmI,
    startLBURPRequest,
    startLBURPResponse,
    endLBURPRequest,
    endLBURPResponse,
    lburpUpdateRequest,
    lburpUpdateResponse,
    turn,
];

/// Modify-Increment [RFC4525]
#[inline]
pub fn modifyIncrement() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,1,14) }

/// All Op Attrs [RFC3673]
#[inline]
pub fn allOpAttrs() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,4,1,4203,1,5,1) }

/// OC AD Lists [RFC4529]
#[inline]
pub fn ocadLists() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,4,1,4203,1,5,2) }

/// True/False filters [RFC4526]
#[inline]
pub fn trueFalseFilters() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,4,1,4203,1,5,3) }

/// Language Tag Options [RFC3866]
#[inline]
pub fn languageTag() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,4,1,4203,1,5,4) }

/// Language Range Options [RFC3866]
#[inline]
pub fn languageRange() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,4,1,4203,1,5,5) }

/// LBURP Incremental Update style OID [RFC4373]
#[inline]
pub fn lburpIncrementalUpdateStyle() -> OBJECT_IDENTIFIER { oid!( 1,3,6,1,1,17,7) }

/// IANA-Registered LDAP Features as of 2025
pub const IANA_REGISTERED_FEATURES_BY_2025: [fn() -> OBJECT_IDENTIFIER; 7] = [
    modifyIncrement,
    allOpAttrs,
    ocadLists,
    trueFalseFilters,
    languageTag,
    languageRange,
    lburpIncrementalUpdateStyle,
];

/// Aborted Transaction Notice [RFC5805]
#[inline]
pub fn abortedTransaction() -> OBJECT_IDENTIFIER { oid!(1,3,6,1,1,21,4) }

/// IANA-Registered LDAP Notices as of 2025
pub const IANA_REGISTERED_NOTICES_BY_2025: [fn() -> OBJECT_IDENTIFIER; 1] = [
    abortedTransaction,
];

/// Check if two LDAP attribute descriptors are the same attribute type
///
/// ## Returns
/// - `true` if the attribute descriptions are equivalent, `false` otherwise
///
/// ## Examples
/// ```rust
/// use wildboar_ldap::compare_attribute_types;
/// 
/// // Same OID, different case
/// assert!(compare_attribute_types(b"cn", b"CN"));
/// 
/// // Same OID with options
/// assert!(compare_attribute_types(b"cn;lang-en", b"CN;lang-fr"));
/// 
/// // Different OIDs
/// assert!(!compare_attribute_types(b"cn", b"sn"));
/// ```
pub fn compare_attribute_types(a: &[u8], b: &[u8]) -> bool {
    const SEMI_COLON: u8 = b';';
    
    // Find semicolon positions
    let a_semi_index = a.iter().position(|&byte| byte == SEMI_COLON);
    let b_semi_index = b.iter().position(|&byte| byte == SEMI_COLON);
    
    // Extract OID portions (everything before semicolon, or entire string if no semicolon)
    let a_oid = match a_semi_index {
        Some(index) => &a[..index],
        None => a,
    };
    
    let b_oid = match b_semi_index {
        Some(index) => &b[..index],
        None => b,
    };
    
    // If OIDs have different lengths, they can't be equal
    if a_oid.len() != b_oid.len() {
        return false;
    }
    
    // Compare OIDs case-insensitively
    a_oid.iter().zip(b_oid.iter()).all(|(&a_byte, &b_byte)| {
        a_byte.eq_ignore_ascii_case(&b_byte)
    })
}
