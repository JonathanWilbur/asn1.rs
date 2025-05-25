use wildboar_asn1::{TRUE, FALSE};
use x500::InformationFramework::AttributeType;
use x500::DistributedOperations::{
    ReferenceType,
    OperationProgress_nameResolutionPhase,
};
use dirstudio_api_types::{
    NavTreeNode,
    DistinguishedName,
    ListResult,
    ReadResult,
    DirectoryClientError,
    DirectoryName,
    FromApiDirectoryName,
    IntoApiDirectoryName,
    EntryInformation,
};
use crate::state::{SessionId, ServerSideState};
use rose::{
    RequestParameters,
    OperationOutcome,
};
use x500_client::{

};
use x500::CommonProtocolSpecification::{
    InvokeId,
    id_opcode_read,
};
use x500::DirectoryAbstractService::{
    ReadArgumentData,
    EntryInformationSelection,
    EntryInformationSelection_attributes,
    EntryInformationSelection_extraAttributes,
    ContextSelection,
    FamilyReturn,
    FamilyReturn_memberSelect_compoundEntry,
};
use x500::EnhancedSecurity::OPTIONALLY_PROTECTED;
use x500::types::{
    ParseX500AttributeType,
    ParseX500Value,
    DisplayX500AttributeType,
    DisplayX500Value,
    ParseX500DirectoryName,
};

#[tauri::command]
pub async fn read(
    state: tauri::State<'_, ServerSideState>,
    session_id: &str,
    object: DirectoryName,
) -> Result<ReadResult, DirectoryClientError> {
    let dap_client_mutex = {
        let sessions = state.sessions.lock().await;
        let maybe_session = sessions.get(session_id);
        if maybe_session.is_none() {
            // FIXME: Need an error for "socket closed" or "session not recognized."
            return Err(DirectoryClientError::SessionAssociationRejected);
        }
        let session = maybe_session.unwrap();
        session.client.clone()
    };

    let mut dap_client = dap_client_mutex.lock().await;
    // FIXME: Need an error: malformed front-end request.
    let object = state.deserialize_x500_name(&object).map_err(|_| DirectoryClientError::MalformedUrl)?;
    let read_outcome = dap_client.read(RequestParameters{
        invoke_id: InvokeId::present(Vec::from([ 0x53 ])), // FIXME: Randomly generate.
        code: id_opcode_read,
        linked_id: None,
        parameter: OPTIONALLY_PROTECTED::unsigned(ReadArgumentData{
            object,
            selection: Some(EntryInformationSelection::new(
                None, // Default is all user attributes
                None, // Default is attributes, not types.
                Some(EntryInformationSelection_extraAttributes::allOperationalAttributes(())),
                Some(ContextSelection::allContexts(())),
                Some(TRUE),
                Some(FamilyReturn::new(
                    FamilyReturn_memberSelect_compoundEntry,
                    None,
                    vec![],
                )),
            )),
            // selection: None,
            modifyRightsRequest: Some(TRUE),
            aliasedRDNs: None,
            criticalExtensions: None,
            entryOnly: None,
            nameResolveOnMaster: None,
            exclusions: None,
            familyGrouping: None,
            operationContexts: None,
            operationProgress: None,
            referenceType: None,
            requestor: None,
            securityParameters: None,
            serviceControls: None,
            _unrecognized: vec![],
        }),
    }).await;
    if let Err(e) = read_outcome {
        // FIXME: Need an error message.
        return Err(DirectoryClientError::TransportAssociationRejected);
    }
    let response = read_outcome.unwrap();
    match response {
        OperationOutcome::Result(result) => {
            let data = match result.parameter {
                OPTIONALLY_PROTECTED::signed(signed) => signed.toBeSigned,
                OPTIONALLY_PROTECTED::unsigned(unsigned) => unsigned,
            };
            // data.
            let returned_name = state.serialize_x500_name(&data.entry.name)
                // For once, this error message is actually correct!
                .map_err(|_| DirectoryClientError::MalformedResponse)?;
            Ok(ReadResult {
                entry: EntryInformation{
                    name: returned_name,
                    // TODO: Get these defaults from the _default methods.
                    derived: data.entry.derivedEntry.unwrap_or(FALSE),
                    from_entry: data.entry.fromEntry.unwrap_or(TRUE),
                    incomplete: data.entry.incompleteEntry.unwrap_or(FALSE),
                    partial_name: data.entry.partialName.unwrap_or(FALSE),
                    info: vec![],
                },
                modify_rights: vec![], // TODO:
                performer: None, // TODO:
                alias_dereferenced: data.aliasDereferenced.unwrap_or(FALSE),
                ldap_message: None, // TODO:
            })
        },
        _ => return Err(DirectoryClientError::MalformedResponse), // FIXME: Use a proper error.
    }
}
