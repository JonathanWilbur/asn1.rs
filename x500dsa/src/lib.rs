mod dsae;
use wildboar_asn1::{OBJECT_IDENTIFIER, ASN1Error, OPTIONAL, UTF8String, BOOLEAN, ASN1Result, SEQUENCE_OF};
// use x500::{InformationFramework::{
//     ObjectClassKind,
//     AttributeUsage,
//     AttributeType,
//     RelativeDistinguishedName,
//     Attribute,
//     DistinguishedName,
//     AttributeValueAssertion,
//     AttributeTypeAndValue,
// }, DirectoryAbstractService::FamilyReturn_memberSelect};
use wildboar_ldap::AttributeValue as LDAPAttributeValue;
use x690::X690Element;
// use thiserror::Error;
use std::{io::Error as IoError, collections::{HashSet, HashMap}};
// use async_trait::async_trait;
// use x500::DirectoryAbstractService::{
//     EntryInformationSelection,
//     EntryInformation_information_Item as EntryInfoItem,
//     EntryModification,
// };
// use tokio_stream::Stream;

// MAPPING-BASED-MATCHING
//     {SelectedBy, BOOLEAN:combinable, MappingResult, OBJECT IDENTIFIER:matchingRule} ::=
// CLASS {
//     &selectBy             SelectedBy OPTIONAL,
//     &ApplicableTo         ATTRIBUTE,
//     &subtypesIncluded     BOOLEAN DEFAULT TRUE,
//     &combinable           BOOLEAN(combinable),
//     &mappingResults       MappingResult OPTIONAL,
//     &userControl          BOOLEAN DEFAULT FALSE,
//     &exclusive            BOOLEAN DEFAULT TRUE,
//     &matching-rule        MATCHING-RULE.&id(matchingRule),
//     &id                   OBJECT IDENTIFIER UNIQUE }
// WITH SYNTAX {
//     [SELECT BY            &selectBy]
//     APPLICABLE TO         &ApplicableTo
//     [SUBTYPES INCLUDED    &subtypesIncluded]
//     COMBINABLE            &combinable
//     [MAPPING RESULTS      &mappingResults]
//     [USER CONTROL         &userControl]
//     [EXCLUSIVE            &exclusive]
//     MATCHING RULE         &matching-rule
//     ID                    &id }

// ALGORITHM ::= CLASS {
//     &Type          OPTIONAL,
//     &id            OBJECT IDENTIFIER UNIQUE }
// WITH SYNTAX {
//     [PARMS         &Type]
//     IDENTIFIED BY  &id }

// EXTENSION ::= CLASS {
//     &id           OBJECT IDENTIFIER UNIQUE,
//     &ExtnType }
// WITH SYNTAX {
//     SYNTAX        &ExtnType
//     IDENTIFIED BY &id }

pub struct Vertex<T> {
    pub superior: Option<T>,
    pub item: T,
}

pub struct RefVertex<'a, T> {
    pub superior: Option<&'a T>,
    pub item: &'a T,
}

pub type DseDatabaseId = u64;

// pub struct DSE {
//     pub id: DseDatabaseId,
//     pub rdn: RelativeDistinguishedName,
// }

// pub type DITVertex = Vertex<DSE>;

#[derive(Debug)]
pub enum DSAStorageDriverError {
    IoError(IoError),
    Corrupted,
    AlreadyExists,
}

pub type DSAStorageDriverResult <T> = Result<T, DSAStorageDriverError>;
pub type TimeLimit = u32;
pub type Priority = u8;

pub type DSAStorageDriverType = u16;
pub const STORAGE_DRIVER_NULL: DSAStorageDriverType = 0; // Writes are NOOPs. No entries, except a minimal Root DSE.
pub const STORAGE_DRIVER_TEST: DSAStorageDriverType = 1; // Sometimes IO errors, invalid syntax, relational violations, etc.
pub const STORAGE_DRIVER_MEMORY: DSAStorageDriverType = 2;
pub const STORAGE_DRIVER_EDB: DSAStorageDriverType = 3;
pub const STORAGE_DRIVER_GIT: DSAStorageDriverType = 4;

// pub struct DSEInformation {
//     pub info: Vec<EntryInfoItem>,
//     pub shadow_subordinates_complete: bool,
//     pub shadow_attributes_complete: bool,
//     pub shadow_attribute_values_incomplete: Vec<AttributeType>,
//     pub subordinates_storage_driver: DSAStorageDriverType,
// }

pub struct BaseStorageDriverRequest <Arg> {
    pub time_limit: TimeLimit,
    pub priority: Priority,
    pub arg: Arg,
}

// pub struct DITWriteRequest <Arg> {
//     pub target: DseDatabaseId,
//     pub time_limit: TimeLimit,
//     pub priority: Priority,
//     pub arg: Arg,
// }

// pub struct NewDSE <'a> {
//     pub rdn: &'a [AttributeTypeAndValue],
//     pub info: &'a [Attribute],
// }

pub type OidKey = Vec<u8>;
pub type AttributeTypeKey = OidKey;
pub type ContextTypeKey = OidKey;
pub type ObjectClassTypeKey = OidKey;

pub struct ContextAssertion {
    pub context_type: OBJECT_IDENTIFIER,
    pub values: Vec<X690Element>,
}

/// ```asn1
/// ContextAssertion ::= SEQUENCE {
///   contextType    CONTEXT.&id({SupportedContexts}),
///   contextValues  SET SIZE (1..MAX) OF
///       CONTEXT.&Assertion({SupportedContexts}{@contextType}),
///   ... }
/// ```
///
pub enum ContextAssertions {
    All(HashMap<ContextTypeKey, ContextAssertion>),
    Preference(SEQUENCE_OF<ContextAssertion>),
}

// FamilyReturn ::= SEQUENCE {
//     memberSelect   ENUMERATED {
//       contributingEntriesOnly   (1),
//       participatingEntriesOnly  (2),
//       compoundEntry             (3),
//       ...},
//     familySelect   SEQUENCE SIZE (1..MAX) OF OBJECT-CLASS.&id OPTIONAL,
//     ... }



// pub struct Selection {
//     pub attributes: HashSet<OidKey>,
//     pub types_only: bool,
//     pub all_user_attrs: bool,
//     pub all_op_attrs: bool,
//     pub select_all_contexts: bool,
//     pub selected_contexts: HashMap<AttributeTypeKey, ContextAssertions>,
//     pub return_contexts: bool,
//     pub member_select: FamilyReturn_memberSelect,
//     pub family_select: HashSet<ObjectClassTypeKey>,
// }

// export
// interface ReadValuesOptions {
//     readonly selection?: EntryInformationSelection;
//     readonly relevantSubentries?: Vertex[];
//     readonly operationContexts?: ContextSelection;
//     readonly noSubtypeSelection?: boolean;
//     readonly dontSelectFriends?: boolean;

//     /**
//      * This is used by service administrative areas to further constrain the
//      * returned attribute types.
//      */
//     readonly outputAttributeTypes?: Map<IndexableOID, ResultAttribute>;
// }

// pub struct StorageDriverDITReadRequest <Arg> {
//     pub time_limit: TimeLimit,
//     pub priority: Priority,
//     pub attribute_size_limit: u32,
//     pub arg: Arg,
//     // noSubtypeSelection
//     // dontSelectFriends
// }

pub type DSEFlags = u64;

pub const DSE_FLAG_ROOT: DSEFlags = 1 << 0;
pub const DSE_FLAG_GLUE: DSEFlags = 1 << 1;
pub const DSE_FLAG_CP: DSEFlags = 1 << 2;
pub const DSE_FLAG_ENTRY: DSEFlags = 1 << 3;
pub const DSE_FLAG_ALIAS: DSEFlags = 1 << 4;
pub const DSE_FLAG_SUBR: DSEFlags = 1 << 5;
pub const DSE_FLAG_NSSR: DSEFlags = 1 << 6;
pub const DSE_FLAG_SUPR: DSEFlags = 1 << 7;
pub const DSE_FLAG_XR: DSEFlags = 1 << 8;
pub const DSE_FLAG_ADMIN_POINT: DSEFlags = 1 << 9;
pub const DSE_FLAG_SUBENTRY: DSEFlags = 1 << 10;
pub const DSE_FLAG_SHADOW: DSEFlags = 1 << 11;
pub const DSE_FLAG_IMM_SUPR: DSEFlags = 1 << 13;
pub const DSE_FLAG_RHOB: DSEFlags = 1 << 14;
pub const DSE_FLAG_SA: DSEFlags = 1 << 15;
pub const DSE_FLAG_DS_SUBENTRY: DSEFlags = 1 << 16;
pub const DSE_FLAG_FAMILY_MEMBER: DSEFlags = 1 << 17;
pub const DSE_FLAG_DIT_BRIDGE: DSEFlags = 1 << 18;
pub const DSE_FLAG_WRITEABLE_COPY: DSEFlags = 1 << 19;
// flags 20-24 are reserved for future DSETypes

// Important object classes have their own flags to accelerate queries
// that filter by object classes.
// Only structural object classes are used, because they can't change
// once an entry is created.
pub const DSE_FLAG_COUNTRY: DSEFlags = 1 << 25;
pub const DSE_FLAG_LOCALITY: DSEFlags = 1 << 26;
pub const DSE_FLAG_ORG: DSEFlags = 1 << 27;
pub const DSE_FLAG_ORG_UNIT: DSEFlags = 1 << 28;
pub const DSE_FLAG_PERSON: DSEFlags = 1 << 29; // Subtypes of PERSON not supported.
pub const DSE_FLAG_ROLE: DSEFlags = 1 << 30; // organizationalRole
pub const DSE_FLAG_GROUP: DSEFlags = 1 << 31; // groupOfNames or groupOfUniqueNames
pub const DSE_FLAG_APPL_PROCESS: DSEFlags = 1 << 32;
pub const DSE_FLAG_APPL_ENTITY: DSEFlags = 1 << 33;
pub const DSE_FLAG_DSA: DSEFlags = 1 << 34;
pub const DSE_FLAG_DEVICE: DSEFlags = 1 << 35;
pub const DSE_FLAG_DMD: DSEFlags = 1 << 36;
pub const DSE_FLAG_OID_ARC: DSEFlags = 1 << 37; // oidC1obj, oidC2obj, or oidCobj
pub const DSE_FLAG_URNC: DSEFlags = 1 << 38; // urnCobj
pub const DSE_FLAG_CRLDP: DSEFlags = 1 << 39; // cRLDistributionPoint

// These flags apply to both admin points and subentries.
pub const DSE_FLAG_ACCESS_CONTROL: DSEFlags = 1 << 25;
pub const DSE_FLAG_COLL_ATTR: DSEFlags = 1 << 26;
pub const DSE_FLAG_CTXT_ASSN: DSEFlags = 1 << 27;
pub const DSE_FLAG_SVC_ADMIN: DSEFlags = 1 << 28;
pub const DSE_FLAG_PWD_ADMIN: DSEFlags = 1 << 29;
pub const DSE_FLAG_SUBSCHEMA: DSEFlags = 1 << 30;
pub const DSE_FLAG_AC_INNER: DSEFlags = 1 << 31; // Only applies to admin points
pub const DSE_FLAG_CA_INNER: DSEFlags = 1 << 32; // Only applies to admin points
pub const DSE_FLAG_AUTONOMOUS: DSEFlags = 1 << 33; // Only applies to admin points

pub const DSE_FLAG_SHADOW_SUBS_COMPLETE: DSEFlags = 1 << 34;
pub const DSE_FLAG_SHADOW_ATTRS_COMPLETE: DSEFlags = 1 << 35;
pub const DSE_FLAG_LEAF: DSEFlags = 1 << 36; // Has no non-family subordinates.
pub const DSE_FLAG_LIST_ACI: DSEFlags = 1 << 37; // Has entryACI that affect list operations.

pub const DSE_FLAG_DELETED: DSEFlags = 1 << 63; // For future support of soft deletion.

pub type AttrValueFlags = u16;

pub const ATTR_VALUE_FLAG_DISTINGUISHED: AttrValueFlags = 1 << 1;
pub const ATTR_VALUE_FLAG_OPERATIONAL: AttrValueFlags = 1 << 2; // TODO: Should this exist?
pub const ATTR_VALUE_HAS_CONTEXTS: AttrValueFlags = 1 << 3;
pub const ATTR_VALUE_CONTEXT_TEMPORAL: AttrValueFlags = 1 << 4;
pub const ATTR_VALUE_CONTEXT_LOCALE: AttrValueFlags = 1 << 5;
pub const ATTR_VALUE_CONTEXT_LANGUAGE: AttrValueFlags = 1 << 6;
pub const ATTR_VALUE_CONTEXT_LDAP_OPTIONS: AttrValueFlags = 1 << 7;
pub const ATTR_VALUE_CONTEXT_SEC_LABEL: AttrValueFlags = 1 << 8;
pub const ATTR_VALUE_CONTEXT_INTEG_INFO: AttrValueFlags = 1 << 9;
pub const ATTR_VALUE_CONTEXT_CONFIDENCE: AttrValueFlags = 1 << 10;
pub const ATTR_VALUE_CONTEXT_SOURCE: AttrValueFlags = 1 << 11;
pub const ATTR_VALUE_CONTEXT_NOTES: AttrValueFlags = 1 << 12;

// pub enum DseDeletionType {
//     HardDeleteAll,
//     HardDeleteAttributes,
//     SoftDelete,
// }

// pub struct NextSubordinateRequest {
//     pub last_id: DseDatabaseId,
//     pub flags: DSEFlags,
// }

// pub struct FindRDN <'a> {
//     pub superior_id: DseDatabaseId,
//     pub rdn: &'a [AttributeTypeAndValue],
// }

// pub struct FindDN <'a> {
//     pub rdn: &'a [RelativeDistinguishedName],
// }

// pub struct FindID {
//     pub id: DseDatabaseId,
//     pub superior: Option<DseDatabaseId>,
// }

// #[async_trait]
pub trait DSAStorageDriver {

    // TODO: priority, sizeLimit, timeLimit, manageDSAITPlaneRef?, attributeSizeLimit
    // TODO: I think you'll have to pass in name knowledge and other things.

    // async fn find_rdn <'a> (
    //     &self,
    //     req: BaseStorageDriverRequest<FindRDN<'a>>,
    // ) -> DSAStorageDriverResult<Option<DseDatabaseId>>;

    // async fn find_dn <'a> (
    //     &self,
    //     req: BaseStorageDriverRequest<FindDN<'a>>,
    // ) -> DSAStorageDriverResult<Option<DseDatabaseId>> {
    //     // TODO: Naive implementation that calls find_rdn recursively.
    //     todo!()
    // }

    // async fn get_rdn <'a> (
    //     &self,
    //     req: BaseStorageDriverRequest<FindID>,
    // ) -> DSAStorageDriverResult<Option<RelativeDistinguishedName>>;

    // async fn get_dn <'a> (
    //     &self,
    //     req: BaseStorageDriverRequest<FindID>,
    // ) -> DSAStorageDriverResult<Option<DistinguishedName>>;

    // async fn get_next_subordinate_dse (
    //     &self,
    //     req: BaseStorageDriverRequest<DseDatabaseId>,
    //     // target: DseDatabaseId,
    //     // time_limit: TimeLimit,
    //     // priority: Priority,
    //     // last: DseDatabaseId,
    //     // TODO: BloomFilter
    // ) -> DSAStorageDriverResult<Option<DITVertex>>;

    // async fn read_dse <'a> (
    //     &self,
    //     target: DseDatabaseId,
    //     time_limit: TimeLimit,
    //     selection: &'a Selection,
    // ) -> DSAStorageDriverResult<DSEInformation>;

    // async fn add_dse <'a> (
    //     &self,
    //     req: &DITWriteRequest<NewDSE<'a>>,
    // ) -> DSAStorageDriverResult<DseDatabaseId>;

    // async fn modify_dse <'a> (
    //     &self,
    //     req: &DITWriteRequest<&'a [EntryModification]>,
    // ) -> DSAStorageDriverResult<()>;

    // async fn remove_dse (
    //     &self,
    //     req: &DITWriteRequest<()>,
    // ) -> DSAStorageDriverResult<()>;

    // async fn modify_rdn <'a> (
    //     &self,
    //     req: &DITWriteRequest<&'a [AttributeTypeAndValue]>,
    // ) -> DSAStorageDriverResult<()>;

    // async fn move_dse <'a> (
    //     &self,
    //     req: &DITWriteRequest<&'a [RelativeDistinguishedName]>, // DN of the new immediate superior
    // ) -> DSAStorageDriverResult<()>;

    // // Naive implementation for when transactions are not supported.
    // async fn modify_dn <'a> (
    //     &self,
    //     req: &DITWriteRequest<&'a [RelativeDistinguishedName]>,
    // ) -> DSAStorageDriverResult<()> {
    //     let len = req.arg.len();
    //     if let Some(rdn) = req.arg.last() {
    //         self.modify_rdn(&DITWriteRequest{
    //             arg: &rdn.as_slice(),
    //             priority: req.priority,
    //             target: req.target,
    //             time_limit: req.time_limit,
    //         }).await?;
    //         self.move_dse(&DITWriteRequest{
    //             arg: &req.arg[0..len - 1],
    //             priority: req.priority,
    //             target: req.target,
    //             time_limit: req.time_limit,
    //         }).await
    //     } else {
    //         Err(DSAStorageDriverError::AlreadyExists)
    //     }
    // }

    // TODO: load_operational_bindings
    // TODO: save_operational_bindings

    // TODO: Naive default implementation that just appends results to the array.
    // async fn list_subordinate_dses (
    //     &self, target: DseDatabaseId,
    //     time_limit: TimeLimit,
    //     size_limit: usize,
    // ) -> DSAStorageDriverResult<Vec<DITVertex>> {
    //     let mut last_id: DseDatabaseId = 0;
    //     let mut results: Vec<DITVertex> = Vec::with_capacity(size_limit);
    //     let mut i = 0;
    //     while let Some(dse) = self.get_next_subordinate_dse(target, last_id, time_limit).await? {
    //         last_id = dse.item.id;
    //         results.push(dse);
    //         i += 1;
    //         if i >= size_limit {
    //             break;
    //         }
    //     }
    //     Ok(results)
    // }

}

// TODO: Implement null storage driver.
// TODO: Implement in-memory storage driver.
// TODO: Implement test storage driver.

/*

Format of values stored in ReDB:

RDNs Table:
Key: | u32 superior entry ID | DER-encoded, normalized RDN |
Value: | u32 DSE ID | Alias bit, FromEntry bit | (Byte including the two bits, list operations can run with a single query*)
*Then again, you still have to query ACI, object classes, DSE type, and probably more to actually perform a list anyway.
*Still, having the whole DSE type in this table might save a lot of computing. DSE type would be a 24-bit field. Subentries could be ignored, for instance.

DSEs Table:
Key: u32 entry ID
Value: ?
  subordinate_completeness Boolean?
  attribute_completeness   Boolean?
  subordinates_driver u16 or u32? (Probably u16)
You could probably get rid of this table entirely if you just define operational attributes for the above values.
- Entry attribute values that are incomplete from shadowing

Format of values:
Key: | Entry ID u32 | 1 byte: type OID length, N | N bytes: DER-encoded type OID, reversed* | X.690 tag class + constructed flag | X.690 tag number | DER-encoded, normalized value
Value: Non-normalized value with contexts
*Reversing gives better performance because the last arc varies more than the first.
Does this table need to have a bit flag for operational?
The key innately limits the value size. However, this might not be a problem,
because directories are not good with handling large values anyway. You might
WANT a low limit (e.g. 65535 bytes) on value sizes.
Values that don't have a normalizer will just have to be queried by attribute, then filtered in-memory.

EnqueuedSearchResults Table:
Key: | Connection ID u128 | Result Index u32 |
Value: The BER-encoded EntryInformation

EnqueuedListResults Table:
Key: | Connection ID u128 | Result Index u32 |
Value: The BER-encoded EntryInformation

PasswordDictionaryItem Table:
Key: | Normalized Password |
Value: | Flags: geo, person, dictionary, etc. |

Operational Binding Table:
Key: | binding_type, binding_identifier, binding_version |
Value: | The entire BER-encoded DOP argument |

NameToOID: Key = normalized name, value = binary OID
OidToName: Key = binary OID, value = normalized name
One small objection I have to storing OIDs in the database is that it might be
really hard to modify them if you've found an error in your data. This could be
solved by something in the web admin console, but it will still be a pain in the
ass.

AccessPoint table?
AccessPointCredentials table?

Version history table: Key = installed version ID, Value = time

Access Point Credentials Table?

This seems like it could be really simple and elegant.
To every value, you could (optionally) add a createTime INTEGER field.

Can you use multithreading with a storage engine???

*/
