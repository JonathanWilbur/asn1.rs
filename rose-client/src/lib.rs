use rose::{
    AsyncRoseClient,
    BindParameters,
    BindOutcome,
    RequestParameters,
    OperationOutcome,
    UnbindParameters,
    UnbindOutcome,
    StartTLSParameters,
    StartTLSOutcome,
    BindRequestSender,
    RequestSender,
    UnbindSender,
    StartTlsSender,
    RoseEngine,
};
use x690::X690Element;
use async_trait::async_trait;
use std::io::{Error, ErrorKind, Result};
use tokio::sync::*;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use rose_idm::RoseIdmStream;
use idm::IdmStream;
use std::sync::Arc;

// TODO: Somehow make this generic over the ROSEReceiver's ParameterType.
pub struct RoseClient {
    pub outbound_binds: BindRequestSender<X690Element>,
    pub outbound_requests: RequestSender<X690Element>,
    pub outbound_unbinds: UnbindSender<X690Element>,
    pub outbound_start_tls: StartTlsSender,
}

impl RoseClient {

    pub fn from_idm <W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync + 'static> (idm: IdmStream<W>) -> Self {
        let (outbound_binds_tx, outbound_binds_rx) = mpsc::unbounded_channel();
        let (outbound_requests_tx, outbound_requests_rx) = mpsc::unbounded_channel();
        let (outbound_unbinds_tx, outbound_unbinds_rx) = mpsc::unbounded_channel();
        let (outbound_start_tls_tx, outbound_start_tls_rx) = mpsc::unbounded_channel();
        let rose_client = RoseClient {
            outbound_binds: outbound_binds_tx,
            outbound_requests: outbound_requests_tx,
            outbound_unbinds: outbound_unbinds_tx,
            outbound_start_tls: outbound_start_tls_tx,
        };
        let mut engine = RoseIdmStream::new(Arc::new(Mutex::new(idm)));
        tokio::spawn(async move {
            if let Err(e) = engine.drive(outbound_binds_rx, outbound_requests_rx, outbound_unbinds_rx, outbound_start_tls_rx).await {
                println!("ROSE transport error: {}", e);
            }
        });
        rose_client
    }

}

#[async_trait]
impl AsyncRoseClient<X690Element> for RoseClient {

    async fn bind(self: &mut Self, params: BindParameters<X690Element>) -> Result<BindOutcome<X690Element, X690Element>> {
        let (tx, rx) = oneshot::channel();
        self.outbound_binds.send((params, tx))
            .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
        rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
    }

    async fn request(self: &mut Self, params: RequestParameters<X690Element>) -> Result<OperationOutcome<X690Element, X690Element>> {
        let (tx, rx) = oneshot::channel();
        self.outbound_requests.send((params, tx))
            .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
        rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
    }

    async fn unbind(self: &mut Self, params: UnbindParameters<X690Element>) -> Result<UnbindOutcome<X690Element, X690Element>> {
        let (tx, rx) = oneshot::channel();
        self.outbound_unbinds.send((params, tx))
            .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
        rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
    }

    async fn start_tls(self: &mut Self, params: StartTLSParameters) -> Result<StartTLSOutcome> {
        let (tx, rx) = oneshot::channel();
        self.outbound_start_tls.send((params, tx))
            .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
        rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use idm::IdmStream;
    use rose::BindParameters;
    use x500::CommonProtocolSpecification::InvokeId;
    use x500::EnhancedSecurity::OPTIONALLY_PROTECTED;
    use std::net::ToSocketAddrs;
    use x500::DirectoryAbstractService::{
        DirectoryBindArgument,
        _encode_DirectoryBindArgument,
        _decode_DirectoryBindResult,
        ListArgument,
        ListArgumentData,
        _encode_ListArgument,
        _decode_ListResult,
        list,
        ListResultData,
    };
    use x500::InformationFramework::{
        Name,
    };
    use x500::DirectoryIDMProtocols::id_idm_dap;
    pub use tokio;

    #[tokio::test]
    async fn it_works() {
        let mut addrs = "localhost:4632"
            .to_socket_addrs()
            .unwrap();
        let socket = tokio::net::TcpSocket::new_v4().unwrap();
        let stream = socket.connect(addrs.next().unwrap()).await.unwrap();
        let mut rose = RoseClient::from_idm(IdmStream::new(stream));
        let dba = DirectoryBindArgument::new(None, None, vec![]);
        let encoded_dba = _encode_DirectoryBindArgument(&dba).unwrap();
        let bind_outcome = rose.bind(BindParameters {
            protocol_id: id_idm_dap(),
            timeout: 5,
            parameter: encoded_dba,
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
        match bind_outcome {
            BindOutcome::Result(result) => _decode_DirectoryBindResult(&result.parameter).unwrap(),
            _ => panic!(),
        };
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
        let outcome = rose.request(RequestParameters {
            invoke_id: InvokeId::present(vec![ 1 ]),
            code: list().operationCode.unwrap(),
            parameter: _encode_ListArgument(&arg).unwrap(),
            linked_id: None,
        }).await.unwrap();
        match outcome {
            OperationOutcome::Result(res) => {
                let result = _decode_ListResult(&res.parameter).unwrap();
                let data = match result {
                    OPTIONALLY_PROTECTED::unsigned(unsigned) => unsigned,
                    OPTIONALLY_PROTECTED::signed(signed) => signed.toBeSigned,
                };
                let list_info = match &data {
                    ListResultData::listInfo(info) => info,
                    _ => panic!(),
                };
                println!("The root DSE has {} subordinates.", list_info.subordinates.len());
            },
            _ => panic!(),
        };
        let unbind_outcome = rose.unbind(UnbindParameters { timeout: 5, parameter: None }).await.unwrap();
        let ures = match unbind_outcome {
            UnbindOutcome::Result(r) => r,
            _ => panic!(),
        };
        assert_eq!(ures, None);
    }
}
