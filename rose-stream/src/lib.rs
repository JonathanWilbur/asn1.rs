use rose::{
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
use tokio::sync::mpsc::UnboundedReceiver;
use x690::X690Element;
use std::io::{Error, ErrorKind, Result};
use std::time::Duration;
use tokio::sync::*;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use rose_idm::RoseIdmStream;
use idm::IdmStream;
use std::sync::Arc;
use tower::timeout::{Timeout, TimeoutLayer, error::Elapsed};
use tower::limit::{ConcurrencyLimit, ConcurrencyLimitLayer};
use tower::{Service, Layer, ServiceExt};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use rose::RosePDU;

struct RequestFuture(
    pub tokio::sync::oneshot::Receiver<OperationOutcome<X690Element, X690Element>>,
    pub std::result::Result<(), tokio::sync::mpsc::error::SendError<(RequestParameters<X690Element>, tokio::sync::oneshot::Sender<OperationOutcome<X690Element, X690Element>>)>>,
);

impl Future for RequestFuture {
    type Output = std::io::Result<OperationOutcome<X690Element, X690Element>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.1.is_err() {
            return Poll::Ready(Err(Error::from(ErrorKind::BrokenPipe)));
        }
        let b = Pin::new(&mut self.0);
        match b.poll(cx) {
            Poll::Ready(r) => Poll::Ready(r.map_err(|_| Error::from(ErrorKind::BrokenPipe))),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Clone)]
struct BindService(pub BindRequestSender<X690Element>);
#[derive(Clone)]
struct RequestService(pub RequestSender<X690Element>);
#[derive(Clone)]
struct UnbindService(pub UnbindSender<X690Element>);
#[derive(Clone)]
struct StartTLSService(pub StartTlsSender);

impl Service<BindParameters<X690Element>> for BindService {

    type Response = BindOutcome<X690Element, X690Element>;
    type Error = std::io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: BindParameters<X690Element>) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        let r = self.0.send((req, tx));
        Box::pin(async {
            r.map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
            rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
        })
    }

}

impl Service<RequestParameters<X690Element>> for RequestService {

    type Response = OperationOutcome<X690Element, X690Element>;
    type Error = std::io::Error;
    type Future = RequestFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestParameters<X690Element>) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        let r = self.0.send((req, tx));
        RequestFuture(rx, r)
    }

}

impl Service<UnbindParameters<X690Element>> for UnbindService {

    type Response = UnbindOutcome<X690Element, X690Element>;
    type Error = std::io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: UnbindParameters<X690Element>) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        let r = self.0.send((req, tx));
        Box::pin(async {
            r.map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
            rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
        })
    }

}

impl Service<StartTLSParameters> for StartTLSService {

    type Response = StartTLSOutcome;
    type Error = std::io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: StartTLSParameters) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        let r = self.0.send((req, tx));
        let fut = async {
            r.map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
            rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe))
        };
        Box::pin(fut)
    }

}

#[derive(Clone)]
pub struct RoseClient {
    bind_service: Timeout<BindService>,
    req_service: ConcurrencyLimit<Timeout<RequestService>>,
    unbind_service: Timeout<UnbindService>,
    start_tls_service: Timeout<StartTLSService>,
}

impl RoseClient {

    pub fn from_idm <W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync + 'static> (
        idm: IdmStream<W>,
        timeout: Duration,
        concurrency: usize,
    ) -> (Self, UnboundedReceiver<RosePDU<X690Element>>) {
        let timeout_layer = TimeoutLayer::new(timeout);
        let concurrency_layer = ConcurrencyLimitLayer::new(concurrency);
        let (outbound_binds_tx, outbound_binds_rx) = mpsc::unbounded_channel();
        let (outbound_requests_tx, outbound_requests_rx) = mpsc::unbounded_channel();
        let (outbound_unbinds_tx, outbound_unbinds_rx) = mpsc::unbounded_channel();
        let (outbound_start_tls_tx, outbound_start_tls_rx) = mpsc::unbounded_channel();
        let (inbound_rose_pdus_tx, inbound_rose_pdus_rx) = mpsc::unbounded_channel();
        let rose_client = RoseClient {
            bind_service: timeout_layer.layer(BindService(outbound_binds_tx)),
            req_service: concurrency_layer.layer(timeout_layer.layer(RequestService(outbound_requests_tx))),
            unbind_service: timeout_layer.layer(UnbindService(outbound_unbinds_tx)),
            start_tls_service: timeout_layer.layer(StartTLSService(outbound_start_tls_tx)),
        };
        let mut engine = RoseIdmStream::new(Arc::new(Mutex::new(idm)));
        tokio::spawn(async move {
            if let Err(e) = engine.drive(
                outbound_binds_rx,
                outbound_requests_rx,
                outbound_unbinds_rx,
                outbound_start_tls_rx,
                inbound_rose_pdus_tx,
            ).await {
                println!("ROSE transport error: {}", e);
            }
        });
        (rose_client, inbound_rose_pdus_rx)
    }

    pub async fn bind(self: &mut Self, params: BindParameters<X690Element>) -> Result<BindOutcome<X690Element, X690Element>> {
        let outcome = self.bind_service.call(params).await;
        if let Err(typeless_error) = outcome.as_ref() {
            if let Some(_) = typeless_error.downcast_ref::<Elapsed>() { // If the error was, in fact, Elapsed...
                return Err(Error::from(ErrorKind::TimedOut));
            }
        }
        outcome.map_err(|e| *e.downcast::<std::io::Error>().unwrap())
    }

    pub async fn request(self: &mut Self, params: RequestParameters<X690Element>) -> Result<OperationOutcome<X690Element, X690Element>> {
        let outcome = self.req_service.ready().await
            // TODO: How would this error (or any error) happen from merely polling for readiness?
            // We do not downcast, because I don't understand the behavior of this, and we don't want a panic.
            .map_err(|_| std::io::Error::from(ErrorKind::Other))?
            .call(params).await;
        if let Err(typeless_error) = outcome.as_ref() {
            if let Some(_) = typeless_error.downcast_ref::<Elapsed>() { // If the error was, in fact, Elapsed...
                return Err(Error::from(ErrorKind::TimedOut));
            }
        }
        outcome.map_err(|e| *e.downcast::<std::io::Error>().unwrap())
    }

    // pub fn request(self: &mut Self, params: RequestParameters<X690Element>) -> RequestFuture {
    //     let mut s = self.clone();
    //     let f = async move {
    //         let outcome = s.req_service.call(params).await;
    //         if let Err(typeless_error) = outcome.as_ref() {
    //             if let Some(_) = typeless_error.downcast_ref::<Elapsed>() { // If the error was, in fact, Elapsed...
    //                 return Err(Error::from(ErrorKind::TimedOut));
    //             }
    //         }
    //         outcome.map_err(|e| *e.downcast::<std::io::Error>().unwrap())
    //         RequestFuture()
    //     };


    //     // outcome.map_err(|e| *e.downcast::<std::io::Error>().unwrap())
    // }

    pub async fn unbind(self: &mut Self, params: UnbindParameters<X690Element>) -> Result<UnbindOutcome<X690Element, X690Element>> {
        let outcome = self.unbind_service.call(params).await;
        if let Err(typeless_error) = outcome.as_ref() {
            if let Some(_) = typeless_error.downcast_ref::<Elapsed>() { // If the error was, in fact, Elapsed...
                return Err(Error::from(ErrorKind::TimedOut));
            }
        }
        outcome.map_err(|e| *e.downcast::<std::io::Error>().unwrap())
    }

    pub async fn start_tls(self: &mut Self, params: StartTLSParameters) -> Result<StartTLSOutcome> {
        let outcome = self.start_tls_service.call(params).await;
        if let Err(typeless_error) = outcome.as_ref() {
            if let Some(_) = typeless_error.downcast_ref::<Elapsed>() { // If the error was, in fact, Elapsed...
                return Err(Error::from(ErrorKind::TimedOut));
            }
        }
        outcome.map_err(|e| *e.downcast::<std::io::Error>().unwrap())
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
        ReadArgument,
        ReadArgumentData,
        _decode_ReadResult,
        read, _encode_ReadArgument,
    };
    use x500::InformationFramework::{
        Name,
    };
    use x500::DirectoryIDMProtocols::id_idm_dap;
    use tokio::task::JoinSet;
    use x690::x690_write_i64_value;

    #[tokio::test]
    async fn it_works() {
        let mut addrs = "localhost:4632"
            .to_socket_addrs()
            .unwrap();
        let socket = tokio::net::TcpSocket::new_v4().unwrap();
        let stream = socket.connect(addrs.next().unwrap()).await.unwrap();
        let (mut rose, _) = RoseClient::from_idm(
            IdmStream::new(stream),
            Duration::from_millis(5000),
            5,
        );
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
        let info = match outcome {
            OperationOutcome::Result(res) => {
                let result = _decode_ListResult(&res.parameter).unwrap();
                let data = match result {
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
        let mut iid: i64 = 5;
        let mut join_handles = JoinSet::new();
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
            let mut r = rose.clone();
            join_handles.spawn(async move {
                let mut iid_bytes = vec![];
                x690_write_i64_value(&mut iid_bytes, iid).unwrap();
                let f = r.request(RequestParameters {
                    invoke_id: InvokeId::present(iid_bytes),
                    code: read().operationCode.unwrap(),
                    parameter: _encode_ReadArgument(&read_arg).unwrap(),
                    linked_id: None,
                });
                let read_outcome = f.await.unwrap();
                match read_outcome {
                    OperationOutcome::Result(res) => {
                        let result = _decode_ReadResult(&res.parameter).unwrap();
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
        let unbind_outcome = rose.unbind(UnbindParameters { timeout: 5, parameter: None }).await.unwrap();
        let ures = match unbind_outcome {
            UnbindOutcome::Result(r) => r,
            _ => panic!(),
        };
        assert_eq!(ures, None);
    }
}
