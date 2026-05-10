use crate::CommonProtocolSpecification::Code;

const CRITICAL: bool = true;
const NON_CRITICAL: bool = false;
const ALL_OPERATIONS: Option<RelevantDAPOperations> = None;
const SEARCH: Option<RelevantDAPOperations> = Some(RelevantDAPOperations{
    search: true,
    read: false,
    compare: false,
    list: false,
    add_entry: false,
    remove_entry: false,
    modify_entry: false,
    modify_dn: false,
    abandon: false,
});

pub const EXT_BIT_SUBENTRIES: usize = 1;
pub const EXT_BIT_COPY_SHALL_DO: usize = 2;
pub const EXT_BIT_ATTRIBUTE_SIZE_LIMIT: usize = 3;
pub const EXT_BIT_EXTRA_ATTRIBUTES: usize = 4;
pub const EXT_BIT_MODIFY_RIGHTS_REQUEST: usize = 5;
pub const EXT_BIT_PAGED_RESULTS_REQUEST: usize = 6;
pub const EXT_BIT_MATCHED_VALUES_ONLY: usize = 7;
pub const EXT_BIT_EXTENDED_FILTER: usize = 8;
pub const EXT_BIT_TARGET_SYSTEM: usize = 9;
pub const EXT_BIT_USE_ALIAS_ON_UPDATE: usize = 10;
pub const EXT_BIT_NEW_SUPERIOR: usize = 11;
pub const EXT_BIT_MANAGE_DSA_IT: usize = 12;
pub const EXT_BIT_USE_OF_CONTEXTS: usize = 13;
pub const EXT_BIT_PARTIAL_NAME_RESOLUTION: usize = 14;
pub const EXT_BIT_OVERSPEC_FILTER: usize = 15;
pub const EXT_BIT_SELECTION_ON_MODIFY: usize = 16;
pub const EXT_BIT_SECURITY_OPERATION_CODE: usize = 18;
pub const EXT_BIT_SECURITY_ATTRIBUTE_CERTIFICATION_PATH: usize = 19;
pub const EXT_BIT_SECURITY_ERROR_PROTECTION: usize = 20;
pub const EXT_BIT_SERVICE_ADMINISTRATION: usize = 25;
pub const EXT_BIT_ENTRY_COUNT: usize = 26;
pub const EXT_BIT_HIERARCHY_SELECTIONS: usize = 27;
pub const EXT_BIT_RELAXATION: usize = 28;
pub const EXT_BIT_FAMILY_GROUPING: usize = 29;
pub const EXT_BIT_FAMILY_RETURN: usize = 30;
pub const EXT_BIT_DN_ATTRIBUTES: usize = 31;
pub const EXT_BIT_FRIEND_ATTRIBUTES: usize = 32;
pub const EXT_BIT_ABANDON_OF_PAGED_RESULTS: usize = 33;
pub const EXT_BIT_PAGED_RESULTS_ON_THE_DSP: usize = 34;
pub const EXT_BIT_REPLACE_VALUES: usize = 35;

/// X.500 directory operations to which critical extensions can apply
pub struct RelevantDAPOperations {
    pub read: bool,
    pub compare: bool,
    pub list: bool,
    pub search: bool,
    pub add_entry: bool,
    pub remove_entry: bool,
    pub modify_entry: bool,
    pub modify_dn: bool,
    pub abandon: bool,
}

/// Information about a Directory Access Protocol (DAP) Critical Extension
pub struct DAPExtensionInfo {

    /// The name of the extension
    pub name: &'static str,

    /// The 1-based index of the bit in the `criticalExtensions` `BIT STRING`
    /// that corresponds to this critical extension.
    pub identifier: usize,

    /// The operations to which this critical extension applies. `None` if
    /// it applies to all of them.
    pub operations: Option<RelevantDAPOperations>,

    /// Whether this extension is critical (which makes you wonder why they are
    /// called critical extensions if this can be `false`)
    pub critical: bool,

    /// The corresponding LDAP control `OBJECT IDENTIFIER`, if one is defined.
    pub ldap_control: Option<&'static [u32]>,
}

pub const DAP_CRITICAL_EXTENSIONS_AS_OF_2025: [DAPExtensionInfo; 30] = [
    DAPExtensionInfo{
        name: "subentries",
        identifier: EXT_BIT_SUBENTRIES,
        operations: ALL_OPERATIONS,
        critical: NON_CRITICAL,
        ldap_control: Some([ 1u32, 3, 6, 1, 4, 1, 4203, 1, 10, 1 ].as_slice()),
    },
    DAPExtensionInfo{
        name: "copyShallDo",
        identifier: EXT_BIT_COPY_SHALL_DO,
        operations: Some(RelevantDAPOperations{
            read: true,
            compare: true,
            list: true,
            search: true,
            abandon: false,
            add_entry: false,
            modify_dn: false,
            modify_entry: false,
            remove_entry: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "attribute size limit",
        identifier: EXT_BIT_ATTRIBUTE_SIZE_LIMIT,
        operations: Some(RelevantDAPOperations{
            read: true,
            search: true,
            abandon: false,
            add_entry: false,
            compare: false,
            list: false,
            modify_dn: false,
            modify_entry: false,
            remove_entry: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "extraAttributes",
        identifier: EXT_BIT_EXTRA_ATTRIBUTES,
        operations: Some(RelevantDAPOperations{
            read: true,
            search: true,
            abandon: false,
            add_entry: false,
            compare: false,
            list: false,
            modify_dn: false,
            modify_entry: false,
            remove_entry: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "modifyRightsRequest",
        identifier: EXT_BIT_MODIFY_RIGHTS_REQUEST,
        operations: Some(RelevantDAPOperations{
            read: true,
            abandon: false,
            add_entry: false,
            compare: false,
            list: false,
            modify_dn: false,
            modify_entry: false,
            remove_entry: false,
            search: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "pagedResultsRequest",
        identifier: EXT_BIT_PAGED_RESULTS_REQUEST,
        operations: Some(RelevantDAPOperations{
            list: true,
            search: true,
            abandon: false,
            add_entry: false,
            compare: false,
            modify_dn: false,
            modify_entry: false,
            read: false,
            remove_entry: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: Some([ 1u32, 2, 840, 113556, 1, 4, 319 ].as_slice()),
    },
    DAPExtensionInfo{
        name: "matchedValuesOnly",
        identifier: EXT_BIT_MATCHED_VALUES_ONLY,
        operations: Some(RelevantDAPOperations{
            read: true,
            abandon: false,
            add_entry: false,
            compare: false,
            list: false,
            modify_dn: false,
            modify_entry: false,
            remove_entry: false,
            search: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: Some([ 1u32, 2, 826, 0, 1, 3344810, 2, 3 ].as_slice()),
    },
    DAPExtensionInfo{
        name: "extendedFilter",
        identifier: EXT_BIT_EXTENDED_FILTER,
        operations: SEARCH,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "targetSystem",
        identifier: EXT_BIT_TARGET_SYSTEM,
        operations: Some(RelevantDAPOperations{
            add_entry: true,
            abandon: false,
            compare: false,
            list: false,
            modify_dn: false,
            modify_entry: false,
            read: false,
            remove_entry: false,
            search: false,
        }),
        critical: CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "useAliasOnUpdate",
        identifier: EXT_BIT_USE_ALIAS_ON_UPDATE,
        operations: Some(RelevantDAPOperations{
            add_entry: true,
            remove_entry: true,
            modify_entry: true,
            abandon: false,
            compare: false,
            list: false,
            modify_dn: false,
            read: false,
            search: false,
        }),
        critical: CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "newSuperior",
        identifier: EXT_BIT_NEW_SUPERIOR,
        operations: Some(RelevantDAPOperations{
            modify_dn: true,
            abandon: false,
            add_entry: false,
            compare: false,
            list: false,
            modify_entry: false,
            read: false,
            remove_entry: false,
            search: false,
        }),
        critical: CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "manageDSAIT",
        identifier: EXT_BIT_MANAGE_DSA_IT,
        operations: ALL_OPERATIONS,
        critical: CRITICAL,
        ldap_control: Some([ 2u32, 16, 840, 1, 113730, 3, 4, 2 ].as_slice()),
    },
    DAPExtensionInfo{
        name: "Use of contexts",
        identifier: EXT_BIT_USE_OF_CONTEXTS,
        operations: Some(RelevantDAPOperations{
            read: true,
            compare: true,
            list: true,
            search: true,
            add_entry: true,
            modify_entry: true,
            modify_dn: true,
            abandon: false,
            remove_entry: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "partialNameResolution",
        identifier: EXT_BIT_PARTIAL_NAME_RESOLUTION,
        operations: Some(RelevantDAPOperations{
            read: true,
            search: true,
            abandon: false,
            add_entry: false,
            compare: false,
            list: false,
            modify_dn: false,
            modify_entry: false,
            remove_entry: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "overspecFilter",
        identifier: EXT_BIT_OVERSPEC_FILTER,
        operations: SEARCH,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "selectionOnModify",
        identifier: EXT_BIT_SELECTION_ON_MODIFY,
        operations: Some(RelevantDAPOperations{
            modify_entry: true,
            read: false,
            compare: false,
            list: false,
            search: false,
            add_entry: false,
            remove_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "Security parameters - Operation code",
        identifier: EXT_BIT_SECURITY_OPERATION_CODE,
        operations: ALL_OPERATIONS,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "Security parameters - Attribute certification path",
        identifier: EXT_BIT_SECURITY_ATTRIBUTE_CERTIFICATION_PATH,
        operations: ALL_OPERATIONS,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "Security parameters - Error Protection",
        identifier: EXT_BIT_SECURITY_ERROR_PROTECTION,
        operations: ALL_OPERATIONS,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "Service administration",
        identifier: EXT_BIT_SERVICE_ADMINISTRATION,
        operations: Some(RelevantDAPOperations{
            read: true,
            search: true,
            modify_entry: true,
            compare: false,
            list: false,
            add_entry: false,
            remove_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "entryCount",
        identifier: EXT_BIT_ENTRY_COUNT,
        operations: SEARCH,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "hierarchySelections",
        identifier: EXT_BIT_HIERARCHY_SELECTIONS,
        operations: SEARCH,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "relaxation",
        identifier: EXT_BIT_RELAXATION,
        operations: SEARCH,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "familyGrouping",
        identifier: EXT_BIT_FAMILY_GROUPING,
        operations: Some(RelevantDAPOperations{
            compare: true,
            search: true,
            remove_entry: true,
            read: false,
            list: false,
            add_entry: false,
            modify_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: CRITICAL, // TODO: Varies per operation
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "familyReturn",
        identifier: EXT_BIT_FAMILY_RETURN,
        operations: Some(RelevantDAPOperations{
            read: true,
            search: true,
            modify_entry: true,
            compare: false,
            list: false,
            add_entry: false,
            remove_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "dnAttributes",
        identifier: EXT_BIT_DN_ATTRIBUTES,
        operations: SEARCH,
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "friend attributes",
        identifier: EXT_BIT_FRIEND_ATTRIBUTES,
        operations: Some(RelevantDAPOperations{
            read: true,
            search: true,
            compare: false,
            list: false,
            add_entry: false,
            remove_entry: false,
            modify_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "Abandon of paged results",
        identifier: EXT_BIT_ABANDON_OF_PAGED_RESULTS,
        operations: Some(RelevantDAPOperations{
            list: true,
            search: true,
            read: false,
            compare: false,
            add_entry: false,
            remove_entry: false,
            modify_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "Paged results on the DSP",
        identifier: EXT_BIT_PAGED_RESULTS_ON_THE_DSP,
        operations: Some(RelevantDAPOperations{
            list: true,
            search: true,
            read: false,
            compare: false,
            add_entry: false,
            remove_entry: false,
            modify_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: NON_CRITICAL,
        ldap_control: None,
    },
    DAPExtensionInfo{
        name: "replaceValues",
        identifier: EXT_BIT_REPLACE_VALUES,
        operations: Some(RelevantDAPOperations{
            modify_entry: true,
            read: false,
            compare: false,
            list: false,
            search: false,
            add_entry: false,
            remove_entry: false,
            modify_dn: false,
            abandon: false,
        }),
        critical: CRITICAL,
        ldap_control: Some([ 1u32, 3, 6, 1, 1, 14 ].as_slice()),
    },
];

/// Determine whether an operation is a modification operation
///
/// Determine whether an operation is a modification operation as opposed to an
/// operation that only reads values. (In some cases, these reads can have
/// auxiliary effects like changing some operational attributes, but they should
/// not have an effect on user attributes.)
///
/// Returns `Some(true)` if it is a modification operation, `Some(false)` if
/// not, and `None` if unknown
pub fn is_modification_operation (op_code: Code) -> Option<bool> {
    match op_code {
        Code::local(code) => {
            if code > 11 {
                return None;
            }
            if code < 6 {
                return Some(false);
            }
            Some(true)
        },
        _ => None,
    }
}
