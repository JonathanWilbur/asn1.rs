use wildboar_asn1::{TRUE, FALSE, OBJECT_IDENTIFIER};
use x500::InformationFramework::AttributeType;
use x500::DistributedOperations::{
    ReferenceType,
    OperationProgress_nameResolutionPhase,
};
use dirstudio_api_types::{
    NavTreeNode,
    DistinguishedName,
    ListResult as ApiListResult,
    DirectoryClientError,
    DirectoryName,
    FromApiDirectoryName,
    IntoApiDirectoryName,
    EntryInformation,
    IntoApiRdn,
    SubordinateInfo,
    NavTreeNodeContent,
};
use crate::state::{SessionId, ServerSideState};
use x500::DirectoryAbstractService::{
    ListResult,
    ListResultData,
    ListArgumentData,
    ListResultData_listInfo_subordinates_Item,
};
use x500::EnhancedSecurity::OPTIONALLY_PROTECTED;
use x500::types::{
    ParseX500AttributeType,
    ParseX500Value,
    DisplayX500AttributeType,
    DisplayX500Value,
    ParseX500DirectoryName,
};
use x500::CommonProtocolSpecification::{
    InvokeId,
    id_opcode_list,
};
use rose::{
    RequestParameters,
    OperationOutcome,
};
use std::borrow::BorrowMut;
use std::iter::Iterator;
use std::mem;

struct ListInfoIterator <'a> {
    remaining_subordinates: &'a [&'a ListResultData_listInfo_subordinates_Item],
}

impl <'a> Iterator for ListInfoIterator<'a> {
    type Item = &'a ListResultData_listInfo_subordinates_Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remaining_subordinates.get(0) {
            None => None,
            Some(item) => {
                self.remaining_subordinates = &self.remaining_subordinates[1..];
                Some(item)
            },
        }
    }

}

struct ListResultIterator <'a> {
    parent: Option<Box<ListResultIterator<'a>>>,
    result_sets: &'a [ListResult],
    index_into_current_set: usize,
    // current_set: Option<&'a ListInfoIterator<'a>>,
}

// impl ListResultIterator<'_> {

//     pub fn new (list_result: ListResult) -> Self {
//         ListResultIterator {
//             parent: None,
//             result_sets: Vec::from([list_result]).as_slice(),
//             current_set: None,
//         }
//     }

// }

impl<'a> Default for ListResultIterator<'a> {
    fn default() -> Self {
        ListResultIterator {
            parent: None,
            result_sets: &[],
            index_into_current_set: 0,
        }
    }
}

// TODO: Copy this to the X.500 library.
impl <'a> Iterator for ListResultIterator<'a> {
    type Item = &'a ListResultData_listInfo_subordinates_Item;

    // Code copied from: https://aloso.github.io/2021/03/09/creating-an-iterator
    fn next(&mut self) -> Option<Self::Item> {
        match self.result_sets.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(list_result) => {
                let data = match list_result {
                    OPTIONALLY_PROTECTED::signed(signed) => &signed.toBeSigned,
                    OPTIONALLY_PROTECTED::unsigned(unsigned) => unsigned,
                };
                match data {
                    ListResultData::listInfo(list_info) => {
                        let maybe_item = list_info.subordinates.get(self.index_into_current_set);
                        if let Some(_) = maybe_item {
                            self.index_into_current_set += 1;
                        } else { // TODO: Is this else condition necessary?
                            self.index_into_current_set = 0;
                        }
                        maybe_item
                    },
                    ListResultData::uncorrelatedListInfo(results) => {
                        self.result_sets = &self.result_sets[1..];

                        // start iterating the child trees
                        *self = ListResultIterator {
                            result_sets: results,
                            parent: Some(Box::new(mem::take(self))),
                            index_into_current_set: 0,
                        };
                        self.next()
                    },
                    _unrecognized => {
                        None // FIXME:
                    },
                }
            },
        }
    }

}


#[tauri::command]
pub async fn list(
    state: tauri::State<'_, ServerSideState>,
    session_id: &str,
    object: DirectoryName,
) -> Result<ApiListResult, DirectoryClientError> {
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
    // FIXME: This implementation fails to deserialize.
    let object_name = state.deserialize_x500_name(&object).map_err(|_| DirectoryClientError::MalformedUrl)?;
    let list_outcome = dap_client.list(RequestParameters{
        invoke_id: InvokeId::present(Vec::from([ 0x58 ])), // FIXME: Randomly generate.
        code: id_opcode_list,
        linked_id: None,
        parameter: OPTIONALLY_PROTECTED::unsigned(ListArgumentData{
            object: object_name,
            pagedResults: None,
            listFamily: None,
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
    if let Err(e) = list_outcome {
        // FIXME: Need an error message.
        return Err(DirectoryClientError::TransportAssociationRejected);
    }
    let response = list_outcome.unwrap();
    match response {
        OperationOutcome::Result(result) => {
            // let data = match &result.parameter {
            //     OPTIONALLY_PROTECTED::signed(signed) => signed.toBeSigned,
            //     OPTIONALLY_PROTECTED::unsigned(unsigned) => unsigned,
            // };
            // FIXME: If alias was dereferenced, the name may be different than
            // what was supplied by the user. Unfortunately, this name field
            // appears in each result set, so it could differ if there is a
            // malfunctioning DSA. I think the solution should be to find which
            // result set has a performer set to the local DSA and use that
            // name. If that does not work, just iterate through the results
            // until you find one. For that matter, I don't know how this will
            // work in the user interface if you return a result set whose base
            // object differs from the request.
            let iterator = ListResultIterator{
                result_sets: &[result.parameter],
                ..Default::default()
            };
            let items = iterator
                .map(|r| {
                    let rdn = state.serialize_x500_rdn(&r.rdn).unwrap();
                    let rdn_str = rdn
                        .iter()
                        .map(|atav| {
                            let type_name = atav.attr_type.name
                                .clone()
                                .unwrap_or(OBJECT_IDENTIFIER::new(&atav.attr_type.numeric).to_string());
                            format!("{}={}", type_name, atav.value.replace("+", "\\+"))
                        })
                        .collect::<Vec<String>>()
                        .join("+")
                        ; // FIXME:
                    NavTreeNode {
                        text: rdn_str,
                        content: NavTreeNodeContent::Subordinate(SubordinateInfo{
                            rdn,
                            alias: r.aliasEntry.unwrap_or(FALSE),
                            entry: r.fromEntry.unwrap_or(TRUE),
                        }),
                    }
                })
                .collect::<Vec<NavTreeNode>>();
            Ok(ApiListResult {
                name: object,
                subordinates: items,
                alias_dereferenced: FALSE, // FIXME:
                ldap_message: None, // TODO:
            })
        },
        _ => return Err(DirectoryClientError::MalformedResponse), // FIXME: Use a proper error.
    }
}
