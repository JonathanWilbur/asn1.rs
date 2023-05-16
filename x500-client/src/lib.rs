#![allow(non_upper_case_globals)]
use x500::DirectoryIDMProtocols::id_idm_dap;
use rose::{
    BindParameters,
    BindOutcome,
    BindResultOrErrorParameters,
    UnbindParameters,
    UnbindOutcome,
    StartTLSParameters,
    StartTLSOutcome,
    RequestParameters,
    OperationOutcome,
    ResultOrErrorParameters,
};
use x500::DirectoryAbstractService::{
    DirectoryBindArgument,
    DirectoryBindResult,
    DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
    _encode_DirectoryBindArgument,
    _decode_DirectoryBindResult,
    _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
    ReadArgument,
    ReadResult,
    _encode_ReadArgument,
    _decode_ReadResult,
    CompareArgument,
    CompareResult,
    _encode_CompareArgument,
    _decode_CompareResult,
    AbandonArgument,
    AbandonResult,
    _encode_AbandonArgument,
    _decode_AbandonResult,
    ListArgument,
    ListResult,
    _encode_ListArgument,
    _decode_ListResult,
    SearchArgument,
    SearchResult,
    _encode_SearchArgument,
    _decode_SearchResult,
    AddEntryArgument,
    AddEntryResult,
    _encode_AddEntryArgument,
    _decode_AddEntryResult,
    RemoveEntryArgument,
    RemoveEntryResult,
    _encode_RemoveEntryArgument,
    _decode_RemoveEntryResult,
    ModifyEntryArgument,
    ModifyEntryResult,
    _encode_ModifyEntryArgument,
    _decode_ModifyEntryResult,
    ModifyDNArgument,
    ModifyDNResult,
    _encode_ModifyDNArgument,
    _decode_ModifyDNResult,
    AdministerPasswordArgument,
    AdministerPasswordResult,
    _encode_AdministerPasswordArgument,
    _decode_AdministerPasswordResult,
    ChangePasswordArgument,
    ChangePasswordResult,
    _encode_ChangePasswordArgument,
    _decode_ChangePasswordResult,
    read,
    compare,
    abandon,
    list,
    search,
    addEntry,
    removeEntry,
    modifyEntry,
    modifyDN,
    administerPassword,
    changePassword,
    _decode_AbandonFailedData,
    AbandonFailedData,
};
use x500::EnhancedSecurity::{
    OPTIONALLY_PROTECTED,
    _decode_OPTIONALLY_PROTECTED,
};
use std::io::{Result, Error, ErrorKind};
use rose_stream::RoseClient;
use x690::{X690Element, x690_write_i64_value};
use std::sync::atomic::AtomicI32;
use std::sync::Arc;

#[derive(Clone)]
pub struct DAPClient {
    pub rose_client: RoseClient,
    pub next_invoke_id: Arc<AtomicI32>,
}

pub type DirectoryBindError = OPTIONALLY_PROTECTED<DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1>;
pub type AbandonFailedError = OPTIONALLY_PROTECTED<AbandonFailedData>;

#[macro_export]
macro_rules! encode_operation {
    ($iid:ident, $opname:ident, $arg_encoder:ident, $params:ident) => {
        RequestParameters {
            invoke_id: x500::CommonProtocolSpecification::InvokeId::present($iid),
            code: $opname().operationCode.unwrap(),
            parameter: $arg_encoder(&$params.parameter)
                .map_err(|_| Error::from(ErrorKind::InvalidData))?,
            linked_id: None,
         }
    };
}

#[macro_export]
macro_rules! decode_operation {
    ($outcome:ident, $res_decoder:ident) => {
        match $outcome {
            OperationOutcome::Result(result) => {
                let decoded_result = $res_decoder(&result.parameter)
                    .map_err(|_| Error::from(ErrorKind::InvalidData))?;
                let res = ResultOrErrorParameters {
                    invoke_id: result.invoke_id,
                    code: result.code,
                    parameter: decoded_result,
                };
                Ok(OperationOutcome::Result(res))
            },
            OperationOutcome::Error(error) => Ok(OperationOutcome::Error(error)),
            OperationOutcome::Reject(r) => Ok(OperationOutcome::Reject(r)),
            OperationOutcome::Abort(a) => Ok(OperationOutcome::Abort(a)),
            OperationOutcome::Timeout => Ok(OperationOutcome::Timeout),
            OperationOutcome::Other(o) => Ok(OperationOutcome::Other(o)),
        }
    };
}

impl DAPClient {

    pub fn new (rose_client: RoseClient) -> Self {
        DAPClient {
            rose_client,
            next_invoke_id: Arc::new(AtomicI32::new(1)),
        }
    }

    pub async fn bind(&mut self, params: BindParameters<DirectoryBindArgument>) -> Result<BindOutcome<DirectoryBindResult, DirectoryBindError>> {
        let outcome = self.rose_client.bind(BindParameters {
            protocol_id: id_idm_dap(),
            parameter: _encode_DirectoryBindArgument(&params.parameter)
                .map_err(|_| Error::from(ErrorKind::InvalidData))?,
            timeout: params.timeout,
            calling_ae_title: params.calling_ae_title,
            calling_ae_invocation_identifier: params.calling_ae_invocation_identifier,
            calling_ap_invocation_identifier: params.calling_ap_invocation_identifier,
            called_ae_title: params.called_ae_title,
            called_ae_invocation_identifier: params.called_ae_invocation_identifier,
            called_ap_invocation_identifier: params.called_ap_invocation_identifier,
            implementation_information: params.implementation_information,
        }).await?;
        match outcome {
            BindOutcome::Result(result) => {
                let decoded_result = _decode_DirectoryBindResult(&result.parameter)
                    .map_err(|_| Error::from(ErrorKind::InvalidData))?;
                let dap_result = BindResultOrErrorParameters {
                    parameter: decoded_result,
                    protocol_id: id_idm_dap(),
                    responding_ae_title: result.responding_ae_title,
                    responding_ae_invocation_identifier: result.responding_ae_invocation_identifier,
                    responding_ap_invocation_identifier: result.responding_ap_invocation_identifier,
                };
                Ok(BindOutcome::Result(dap_result))
            },
            BindOutcome::Error(error) => {
                let decoded_error = _decode_OPTIONALLY_PROTECTED(
                    _decode_DirectoryBindError_OPTIONALLY_PROTECTED_Parameter1,
                    &error.parameter,
                )
                    .map_err(|_| Error::from(ErrorKind::InvalidData))?;
                let dap_error = BindResultOrErrorParameters {
                    parameter: decoded_error,
                    protocol_id: id_idm_dap(),
                    responding_ae_title: error.responding_ae_title,
                    responding_ae_invocation_identifier: error.responding_ae_invocation_identifier,
                    responding_ap_invocation_identifier: error.responding_ap_invocation_identifier,
                };
                Ok(BindOutcome::Error(dap_error))
            },
            BindOutcome::Abort(reason) => Ok(BindOutcome::Abort(reason)),
            BindOutcome::Timeout => Ok(BindOutcome::Timeout),
            BindOutcome::Other(other) => Ok(BindOutcome::Other(other)),
        }
    }

    pub async fn read (&mut self, params: RequestParameters<ReadArgument>) -> Result<OperationOutcome<ReadResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, read, _encode_ReadArgument, params)).await?;
        decode_operation!(outcome, _decode_ReadResult)
    }

    pub async fn compare (&mut self, params: RequestParameters<CompareArgument>) -> Result<OperationOutcome<CompareResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, compare, _encode_CompareArgument, params)).await?;
        decode_operation!(outcome, _decode_CompareResult)
    }

    // TODO: abandonFailed is the only legitimate error.
    pub async fn abandon (&mut self, params: RequestParameters<AbandonArgument>) -> Result<OperationOutcome<AbandonResult, AbandonFailedError>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, abandon, _encode_AbandonArgument, params)).await?;
        match outcome {
            OperationOutcome::Result(result) => {
                let decoded_result = _decode_AbandonResult(&result.parameter)
                    .map_err(|_| Error::from(ErrorKind::InvalidData))?;
                let dap_result = ResultOrErrorParameters {
                    invoke_id: result.invoke_id,
                    code: result.code,
                    parameter: decoded_result,
                };
                Ok(OperationOutcome::Result(dap_result))
            },
            OperationOutcome::Error(error) => { // abandonFailed is the only legitimate error.
                let decoded_result = _decode_OPTIONALLY_PROTECTED(_decode_AbandonFailedData, &error.parameter)
                    .map_err(|_| Error::from(ErrorKind::InvalidData))?;
                let e = ResultOrErrorParameters {
                    invoke_id: error.invoke_id,
                    code: error.code,
                    parameter: decoded_result,
                };
                Ok(OperationOutcome::Error(e))
            },
            OperationOutcome::Reject(r) => Ok(OperationOutcome::Reject(r)),
            OperationOutcome::Abort(a) => Ok(OperationOutcome::Abort(a)),
            OperationOutcome::Timeout => Ok(OperationOutcome::Timeout),
            OperationOutcome::Other(o) => Ok(OperationOutcome::Other(o)),
        }
    }

    pub async fn list (&mut self, params: RequestParameters<ListArgument>) -> Result<OperationOutcome<ListResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, list, _encode_ListArgument, params)).await?;
        decode_operation!(outcome, _decode_ListResult)
    }

    pub async fn search (&mut self, params: RequestParameters<SearchArgument>) -> Result<OperationOutcome<SearchResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, search, _encode_SearchArgument, params)).await?;
        decode_operation!(outcome, _decode_SearchResult)
    }

    pub async fn add_entry (&mut self, params: RequestParameters<AddEntryArgument>) -> Result<OperationOutcome<AddEntryResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, addEntry, _encode_AddEntryArgument, params)).await?;
        decode_operation!(outcome, _decode_AddEntryResult)
    }

    pub async fn remove_entry (&mut self, params: RequestParameters<RemoveEntryArgument>) -> Result<OperationOutcome<RemoveEntryResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, removeEntry, _encode_RemoveEntryArgument, params)).await?;
        decode_operation!(outcome, _decode_RemoveEntryResult)
    }

    pub async fn modify_entry (&mut self, params: RequestParameters<ModifyEntryArgument>) -> Result<OperationOutcome<ModifyEntryResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, modifyEntry, _encode_ModifyEntryArgument, params)).await?;
        decode_operation!(outcome, _decode_ModifyEntryResult)
    }

    pub async fn modify_dn (&mut self, params: RequestParameters<ModifyDNArgument>) -> Result<OperationOutcome<ModifyDNResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, modifyDN, _encode_ModifyDNArgument, params)).await?;
        decode_operation!(outcome, _decode_ModifyDNResult)
    }

    pub async fn administer_assword (&mut self, params: RequestParameters<AdministerPasswordArgument>) -> Result<OperationOutcome<AdministerPasswordResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, administerPassword, _encode_AdministerPasswordArgument, params)).await?;
        decode_operation!(outcome, _decode_AdministerPasswordResult)
    }

    pub async fn change_password (&mut self, params: RequestParameters<ChangePasswordArgument>) -> Result<OperationOutcome<ChangePasswordResult, X690Element>> {
        let iid = self.next_invoke_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut iid_bytes = vec![];
        _ = x690_write_i64_value(&mut iid_bytes, iid as i64);
        let outcome = self.rose_client.request(encode_operation!(iid_bytes, changePassword, _encode_ChangePasswordArgument, params)).await?;
        decode_operation!(outcome, _decode_ChangePasswordResult)
    }

    // TODO: Add other convenience methods, optionally configured in the package.

    pub async fn unbind(self: &mut Self, params: UnbindParameters<X690Element>) -> Result<UnbindOutcome<X690Element, X690Element>> {
        self.rose_client.unbind(params).await
    }

    pub async fn start_tls(self: &mut Self, params: StartTLSParameters) -> Result<StartTLSOutcome> {
        self.rose_client.start_tls(params).await
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use ::idm::IdmStream;
    use std::net::ToSocketAddrs;
    use std::time::Duration;
    use tokio::net::TcpSocket;
    use x500::DirectoryAbstractService::{
        DirectoryBindArgument,
        ListArgumentData,
        ListArgument,
        ListResultData,
        ReadArgumentData,
    };
    use x500::InformationFramework::Name;
    use x500::DirectoryIDMProtocols::id_idm_dap;
    use x500::CommonProtocolSpecification::InvokeId;
    use tokio::task::JoinSet;
    use x690::x690_write_i64_value;

    #[tokio::test]
    async fn test_bind_to_x500_dsa_via_idm() {
        let mut addrs = "localhost:4632"
            .to_socket_addrs()
            .unwrap();
        let socket = TcpSocket::new_v4().unwrap();
        let stream = socket.connect(addrs.next().unwrap()).await.unwrap();

        let (rose, _) = RoseClient::from_idm(
            IdmStream::new(stream),
            Duration::from_millis(5000),
            5,
        );
        let mut dap = DAPClient::new(rose);
        let dba = DirectoryBindArgument::new(None, None, vec![]);
        let bind_outcome = dap
            .bind(BindParameters {
                protocol_id: id_idm_dap(),
                timeout: 5,
                parameter: dba,
                calling_ae_title: None,
                calling_ap_invocation_identifier: None,
                calling_ae_invocation_identifier: None,
                called_ae_title: None,
                called_ap_invocation_identifier: None,
                called_ae_invocation_identifier: None,
                implementation_information: None,
            })
            .await
            .unwrap();
        let bind_result = match bind_outcome {
            BindOutcome::Result(r) => r,
            _ => panic!(),
        };
        assert!(bind_result.parameter.versions.is_some());
        let arg = ListArgument::unsigned(ListArgumentData::new(
            Name::rdnSequence(vec![]),
            None,
            None,
            vec![],
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ));
        let outcome = dap.list(RequestParameters {
            invoke_id: InvokeId::present(vec![ 1 ]),
            code: list().operationCode.unwrap(),
            parameter: arg,
            linked_id: None,
        }).await.unwrap();
        let info = match outcome {
            OperationOutcome::Result(res) => {
                let data = match res.parameter {
                    OPTIONALLY_PROTECTED::unsigned(unsigned) => unsigned,
                    OPTIONALLY_PROTECTED::signed(signed) => signed.toBeSigned,
                };
                let list_info = match data {
                    ListResultData::listInfo(info) => info,
                    _ => panic!(),
                };
                list_info
            },
            _ => panic!(),
        };
        println!("The root DSE has {} subordinates.", info.subordinates.len());
        let mut join_handles = JoinSet::new();
        let mut iid: i64 = 1; // FIXME: This is unused.
        for sub in info.subordinates {
            iid += 1;
            let read_arg = ReadArgument::unsigned(ReadArgumentData::new(
                Name::rdnSequence(vec![ sub.rdn ]),
                None,
                None,
                vec![],
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ));
            let mut cloned_dap = dap.clone();
            join_handles.spawn(async move {
                let mut iid_bytes = vec![];
                x690_write_i64_value(&mut iid_bytes, iid).unwrap();
                let read_outcome = cloned_dap.read(RequestParameters {
                    invoke_id: InvokeId::present(iid_bytes),
                    code: read().operationCode.unwrap(),
                    parameter: read_arg,
                    linked_id: None,
                }).await.unwrap();
                match read_outcome {
                    OperationOutcome::Result(res) => {
                        let result = res.parameter;
                        match result {
                            OPTIONALLY_PROTECTED::unsigned(unsigned) => unsigned,
                            OPTIONALLY_PROTECTED::signed(signed) => signed.toBeSigned,
                        };
                        println!("Completed operation {}", iid);
                    },
                    _ => {
                        println!("Failed operation {}", iid);
                    },
                };
            });
        }
        while let Some(_) = join_handles.join_next().await {}
        let unbind_outcome = dap.unbind(UnbindParameters {
            timeout: 5,
            parameter: None,
        }).await.unwrap();
        let ures = match unbind_outcome {
            UnbindOutcome::Result(r) => r,
            _ => panic!(),
        };
        assert_eq!(ures, None);
    }

    // #[tokio::test]
    // async fn test_bind_to_x500_dsa_via_idms() {
    //     use tokio_rustls::TlsConnector;
    //     use tokio_rustls::rustls::{
    //         ClientConfig,
    //         RootCertStore,
    //         ServerName,
    //         Certificate,
    //         Error as TLSError,
    //     };
    //     use tokio_rustls::rustls::client::{
    //         ServerCertVerifier,
    //         ServerCertVerified,
    //     };
    //     use std::time::SystemTime;

    //     struct NoCertVerifier {}

    //     impl ServerCertVerifier for NoCertVerifier {
    //         fn verify_server_cert(
    //             &self,
    //             _: &Certificate,
    //             _: &[Certificate],
    //             _: &ServerName,
    //             _: &mut dyn Iterator<Item = &[u8]>,
    //             _: &[u8],
    //             _: SystemTime,
    //         ) -> std::result::Result<ServerCertVerified, TLSError> {
    //             Ok(ServerCertVerified::assertion())
    //         }
    //     }

    //     let root_store = RootCertStore::empty();
    //     let mut config = ClientConfig::builder()
    //         .with_safe_defaults()
    //         .with_root_certificates(root_store)
    //         .with_no_client_auth()
    //         ;
    //     config
    //         .dangerous()
    //         .set_certificate_verifier(Arc::new(NoCertVerifier {}));

    //     let tls_connector = TlsConnector::from(Arc::new(config));
    //     let server_name = ServerName::try_from("dsa01.root.mkdemo.wildboar.software").unwrap();
    //     let mut addrs = "dsa01.root.mkdemo.wildboar.software:44632"
    //         .to_socket_addrs()
    //         .unwrap();
    //     let socket = TcpSocket::new_v4().unwrap();
    //     let stream = socket.connect(addrs.next().unwrap()).await.unwrap();
    //     let tls_stream = tls_connector.connect(server_name, stream).await.unwrap();
    //     let idm = IdmStream::new(tls_stream);
    //     let mut rose = RoseIdmStream::new(idm);
    //     let dba = DirectoryBindArgument::new(None, None, vec![]);
    //     let encoded_dba = _encode_DirectoryBindArgument(&dba).unwrap();
    //     let bytes_written = rose
    //         .write_bind(BindParameters {
    //             protocol_id: id_idm_dap(),
    //             timeout: 5,
    //             parameter: encoded_dba,
    //             calling_ae_title: None,
    //             calling_ap_invocation_identifier: None,
    //             calling_ae_invocation_identifier: None,
    //             called_ae_title: None,
    //             called_ap_invocation_identifier: None,
    //             called_ae_invocation_identifier: None,
    //             implementation_information: None,
    //         })
    //         .await
    //         .unwrap();
    //     sleep(Duration::new(5, 0)).await;
    //     assert!(bytes_written.gt(&0));
    //     // tokio::time::timeout(Duration::from_millis(10000), async {
    //     //     while let Some(rose_pdu) = rose.read_pdu().await.unwrap() {
    //     //         match &rose_pdu {
    //     //             RosePDU::BindResult(_br) => {
    //     //                 println!("Made it, big dawg.");
    //     //                 return;
    //     //             }
    //     //             _ => panic!(),
    //     //         };
    //     //     }
    //     // })
    //     // .await
    //     // .unwrap();
    // }
}
