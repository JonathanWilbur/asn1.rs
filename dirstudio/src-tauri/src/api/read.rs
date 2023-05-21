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
};
use crate::state::{SessionId, ServerSideState};

#[tauri::command]
pub async fn read(
    state: tauri::State<'_, ServerSideState>,
    session_id: &str,
    target: DistinguishedName,
) -> Result<ReadResult, DirectoryClientError> {
    Err(DirectoryClientError::Unimplemented)
}
