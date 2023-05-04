use std::{collections::HashMap, task::{Context, Waker, Poll}};
use std::future::Future;
use std::pin::Pin;
use rose::{
    AsyncRoseClient,
    InvokeIdInt,
    ROSEReceiver,
    ROSETransmitter,
    BindParameters,
    BindOutcome,
    RequestParameters,
    OperationOutcome,
    UnbindParameters,
    UnbindOutcome,
    StartTLSParameters,
    StartTLSOutcome,
    RosePDU,
    AbortReason,
    RejectParameters,
    RejectReason,
};
use x690::X690Element;
use async_trait::async_trait;
use std::io::{Error, ErrorKind, Result};
use tower_service::Service;
use tokio::sync::*;
use x500::CommonProtocolSpecification::InvokeId;
use asn1::read_i64;
use std::sync::Arc;

enum LoopMode {
    #[allow(dead_code)]
    SingleOp,
    Continuous,
}

// TODO: Somehow make this generic over the ROSEReceiver's ParameterType.
pub struct RoseClient <Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> + Send> {
    pub transport: Arc<Mutex<Transport>>,
    pub outstanding_requests: HashMap<InvokeIdInt, oneshot::Sender<OperationOutcome<X690Element, X690Element>>>,
    pub concurrency: u32,

    bound: bool,
    outstanding_bind_req: Option<oneshot::Sender<BindOutcome<X690Element, X690Element>>>,
    outstanding_unbind_req: Option<oneshot::Sender<UnbindOutcome<X690Element, X690Element>>>,
    outstanding_starttls_req: Option<oneshot::Sender<StartTLSOutcome>>,
}

impl <Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> + Send> RoseClient <Transport> {

    pub fn new (transport: Transport, concurrency: u32) -> Self {
        RoseClient {
            transport: Arc::new(Mutex::new(transport)),
            outstanding_requests: HashMap::new(),
            bound: false,
            outstanding_bind_req: None,
            outstanding_unbind_req: None,
            outstanding_starttls_req: None,
            concurrency,
        }
    }

    pub fn reset (&mut self) {
        self.bound = false;
        self.outstanding_bind_req = None;
        self.outstanding_requests.clear();
    }

    pub async fn read_pdu (&mut self) -> Result<Option<RosePDU<X690Element>>> {
        println!("before lock");
        let t = self.transport.clone();
        let mut transport = t.lock().await;
            // .map_err(|_| Error::from(ErrorKind::Other))?;
        println!("after lock");
        transport.read_pdu().await
    }

    pub async fn abort (&mut self, reason: AbortReason) -> Result<usize> {
        // TODO: .write_abort() should close the connection.
        self.reset();
        let t = self.transport.clone();
        let mut transport = t.lock().await;
            // .map_err(|_| Error::from(ErrorKind::Other))?;
        transport.write_abort(reason).await
    }

    /// Repeatedly poll the connection until it exits.
    pub async fn drive(&mut self) -> Result<()> {
        self.turn(LoopMode::Continuous).await.map(|_| ())
    }

    async fn turn(&mut self, mode: LoopMode) -> Result<()> {
        loop {
            tokio::select! {
                fallible_rose_pdu_or_not = self.read_pdu() => {
                    let rose_pdu_or_not = fallible_rose_pdu_or_not?;
                    // TODO: Auto-abort on encountering giant invoke IDs or
                    if let Some(rose_pdu) = rose_pdu_or_not {
                        match rose_pdu {
                            // TODO: Is there any way to make this work as a server too?
                            // Maybe just make this forward the RosePDUs to a receiver.
                            RosePDU::Bind(params) => {
                                // TODO: Abort with invalidProtocol. We are the client.
                            },
                            /* It seems to be implied in X.519 that only one
                            bind request may be outstanding at any time, so we'll run with that. */
                            RosePDU::BindResult(params) => {
                                if let Some(pending) = self.outstanding_bind_req.take() {
                                    // It sucks cloning this much, but these are not
                                    // too expensive to clone unless the user
                                    // supplied freakishly huge AE-titles, protocol
                                    // IDs, opcodes, etc.
                                    pending.send(BindOutcome::Result(params))
                                        .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                                } else {
                                    self.abort(AbortReason::InvalidProtocol).await?;
                                }
                            },
                            RosePDU::BindError(params) => {
                                if let Some(pending) = self.outstanding_bind_req.take() {
                                    // It sucks cloning this much, but these are not
                                    // too expensive to clone unless the user
                                    // supplied freakishly huge AE-titles, protocol
                                    // IDs, opcodes, etc.
                                    pending.send(BindOutcome::Error(params))
                                        .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                                } else {
                                    self.abort(AbortReason::InvalidProtocol).await?;
                                }
                            },
                            RosePDU::Request(params) => {
                                // TODO: Abort with invalidProtocol. We are the client.
                            },
                            RosePDU::Result(ref params) => {
                                let rejection = RejectParameters {
                                    invoke_id: params.invoke_id.clone(),
                                    reason: RejectReason::UnknownInvokeIdResult,
                                };
                                let invoke_id = match &params.invoke_id {
                                    InvokeId::present(i) => i,
                                    _ => {
                                        let t = self.transport.clone();
                                        let mut transport = t.lock().await;
                                        transport.write_reject(rejection).await?;
                                        continue;
                                    },
                                };
                                let invoke_id_int = match read_i64(&invoke_id) {
                                    Ok(i) => i,
                                    _ => { // Usually if the invokeID was huge.
                                        {
                                            let t = self.transport.clone();
                                            let mut transport = t.lock().await;
                                            transport.write_reject(rejection).await?;
                                        }
                                        continue;
                                    },
                                };
                                let outstanding_req = match self.outstanding_requests.remove(&invoke_id_int) {
                                    Some(o) => o,
                                    _ => { // No request with this InvokeID.
                                        {
                                            let t = self.transport.clone();
                                            let mut transport = t.lock().await;
                                            transport.write_reject(rejection).await?;
                                        }
                                        continue;
                                    },
                                };
                                outstanding_req.send(OperationOutcome::Result(params.clone()))
                                    .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                            },
                            RosePDU::Error(ref params) => {
                                let rejection = RejectParameters {
                                    invoke_id: params.invoke_id.clone(),
                                    reason: RejectReason::UnknownInvokeIdError,
                                };
                                let invoke_id = match &params.invoke_id {
                                    InvokeId::present(i) => i,
                                    _ => {
                                        {
                                            let t = self.transport.clone();
                                            let mut transport = t.lock().await;
                                            transport.write_reject(rejection).await?;
                                        }
                                        continue;
                                    },
                                };
                                let invoke_id_int = match read_i64(&invoke_id) {
                                    Ok(i) => i,
                                    _ => { // Usually if the invokeID was huge.
                                        {
                                            let t = self.transport.clone();
                                            let mut transport = t.lock().await;
                                            transport.write_reject(rejection).await?;
                                        }
                                        continue;
                                    },
                                };
                                let outstanding_req = match self.outstanding_requests.remove(&invoke_id_int) {
                                    Some(o) => o,
                                    _ => { // No request with this InvokeID.
                                        {
                                            let t = self.transport.clone();
                                            let mut transport = t.lock().await;
                                            transport.write_reject(rejection).await?;
                                        }
                                        continue;
                                    },
                                };
                                outstanding_req.send(OperationOutcome::Error(params.clone()))
                                    .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                            },
                            RosePDU::Reject(ref params) => {
                                let invoke_id = match &params.invoke_id {
                                    InvokeId::present(i) => i,
                                    _ => {
                                        self.abort(AbortReason::Other).await?;
                                        continue;
                                    },
                                };
                                let invoke_id_int = match read_i64(&invoke_id) {
                                    Ok(i) => i,
                                    _ => { // Usually if the invokeID was huge.
                                        self.abort(AbortReason::Other).await?;
                                        continue;
                                    },
                                };
                                let outstanding_req = match self.outstanding_requests.remove(&invoke_id_int) {
                                    Some(o) => o,
                                    _ => { // No request with this InvokeID.
                                        self.abort(AbortReason::Other).await?;
                                        continue;
                                    },
                                };
                                outstanding_req.send(OperationOutcome::Reject(params.clone()))
                                    .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                            },
                            RosePDU::Unbind(params) => {

                            },
                            RosePDU::UnbindResult(params) => {
                                if let Some(pending) = self.outstanding_unbind_req.take() {
                                    // It sucks cloning this much, but these are not
                                    // too expensive to clone unless the user
                                    // supplied freakishly huge AE-titles, protocol
                                    // IDs, opcodes, etc.
                                    pending.send(UnbindOutcome::Result(params.parameter.clone()))
                                        .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                                } else {
                                    self.abort(AbortReason::InvalidProtocol).await?;
                                }
                            },
                            RosePDU::UnbindError(params) => {
                                if let Some(pending) = self.outstanding_unbind_req.take() {
                                    // It sucks cloning this much, but these are not
                                    // too expensive to clone unless the user
                                    // supplied freakishly huge AE-titles, protocol
                                    // IDs, opcodes, etc.
                                    pending.send(UnbindOutcome::Error(params.parameter.clone()))
                                        .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                                } else {
                                    self.abort(AbortReason::InvalidProtocol).await?;
                                }
                            },
                            RosePDU::Abort(params) => {
                                // TODO: The connection should be automatically closed when an abort is received.
                                self.reset();
                            },
                            _ => {}, // NOOP for StartTLS and StartTLSResponse. Those are handled at lower layers.
                            // RosePDU::StartTLS(params) => {

                            // },
                            // RosePDU::StartTLSResponse(params) => {

                            // },
                        }
                    } else { // We simply do not have a ROSE PDU ready yet.
                        break;
                    }
                },
            };
            if let LoopMode::SingleOp = mode {
                break;
            }
        }
        Ok(())
    }

}

#[macro_export]
macro_rules! drive {
    ($conn:expr) => {
        $crate::tokio::spawn(async move {
            if let Err(e) = $conn.drive().await {
                println!("ROSE transport error: {}", e);
            }
        });
    };
}

// impl <Transport> Service<BindParameters<X690Element>> for RoseClient<Transport>
// where Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> + Send + 'static {
//     type Response = BindOutcome<X690Element, X690Element>;
//     type Error = std::io::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

//     fn poll_ready(
//         &mut self,
//         cx: &mut Context<'_>,
//     ) -> Poll<std::result::Result<(), Self::Error>> {
//         Poll::Ready(Ok(())) // TODO: Set concurrency limit.
//     }

//     fn call(&mut self, req: BindParameters<X690Element>) -> Self::Future {
//         let (tx, rx) = oneshot::channel();
//         self.outstanding_bind_req = Some(tx);
//         let transport_mutex = self.transport.clone();
//         Box::pin(async move {
//             {
//                 let mut transport = transport_mutex.lock().await;
//                 transport.write_bind(req).await?;
//             }
//             return rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe));
//         })
//     }

// }

// impl <Transport> Service<RequestParameters<X690Element>> for RoseClient<Transport>
// where Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> + Send {
//     type Response = OperationOutcome<X690Element, X690Element>;
//     type Error = std::io::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

//     fn poll_ready(
//         &mut self,
//         cx: &mut Context<'_>,
//     ) -> Poll<std::result::Result<(), Self::Error>> {
//         if self.outstanding_requests.len() >= self.concurrency as usize {
//             return Poll::Pending;
//         }
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: RequestParameters<X690Element>) -> Self::Future {
//         Box::pin(async move {
//             if !self.bound {
//                 return Err(Error::from(ErrorKind::InvalidInput));
//             }
//             let invoke_id = match &req.invoke_id {
//                 InvokeId::present(i) => i,
//                 _ => {
//                     self.abort(AbortReason::Other);
//                     return Err(Error::from(ErrorKind::InvalidInput));
//                 },
//             };
//             let invoke_id_int = match read_i64(&invoke_id) {
//                 Ok(i) => i,
//                 _ => { // Usually if the invokeID was huge.
//                     self.abort(AbortReason::Other);
//                     return Err(Error::from(ErrorKind::InvalidInput));
//                 },
//             };
//             let (tx, rx) = oneshot::channel();
//             self.outstanding_requests.insert(invoke_id_int, tx);
//             self.transport.write_request(req).await;
//             return rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe));
//         })
//     }

// }

// impl <Transport> Service<UnbindParameters<X690Element>> for RoseClient<Transport>
// where Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> + Send {
//     type Response = UnbindOutcome<X690Element, X690Element>;
//     type Error = std::io::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

//     fn poll_ready(
//         &mut self,
//         cx: &mut Context<'_>,
//     ) -> Poll<std::result::Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: UnbindParameters<X690Element>) -> Self::Future {
//         Box::pin(async move {
//             let (tx, rx) = oneshot::channel();
//             self.outstanding_unbind_req = Some(tx);
//             self.transport.write_unbind(req).await;
//             return rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe));
//         })
//     }

// }

// impl <Transport> Service<StartTLSParameters> for RoseClient<Transport>
// where Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> {
//     type Response = StartTLSOutcome;
//     type Error = std::io::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response>>>>;

//     fn poll_ready(
//         &mut self,
//         cx: &mut Context<'_>,
//     ) -> Poll<std::result::Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: StartTLSParameters) -> Self::Future {
//         Box::pin(async move {
//             let (tx, rx) = oneshot::channel();
//             self.outstanding_starttls_req = Some(tx);
//             self.transport.write_start_tls(req).await;
//             return rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe));
//         })
//     }

// }

// #[async_trait]
// pub trait AsyncRoseClient<ParameterType = ASN1Value>
// where
//     ParameterType: Send
// {
//     async fn bind(self: &mut Self, params: BindParameters<ParameterType>) -> Result<BindOutcome<ParameterType, ParameterType>>
//     where
//         ParameterType: 'async_trait;

//     async fn request(self: &mut Self, params: RequestParameters<ParameterType>) -> Result<OperationOutcome<ParameterType, ParameterType>>
//     where
//         ParameterType: 'async_trait;

//     async fn unbind(self: &mut Self, params: UnbindParameters<ParameterType>) -> Result<UnbindOutcome<ParameterType, ParameterType>>
//     where
//         ParameterType: 'async_trait;

//     async fn start_tls(self: &mut Self, params: StartTLSParameters) -> Result<StartTLSOutcome>
//     where
//         ParameterType: 'async_trait;
// }

// #[async_trait]
// impl <Transport> AsyncRoseClient<X690Element> for RoseClient<Transport>
// where Transport: ROSEReceiver<X690Element> + ROSETransmitter<X690Element> + Send {

//     async fn bind(self: &mut Self, params: BindParameters<X690Element>) -> Result<BindOutcome<X690Element, X690Element>> {
//         let (tx, rx) = oneshot::channel();
//         self.outstanding_bind_req = Some(tx);
//         let transport_mutex = self.transport.clone();
//         {
//             let mut transport = transport_mutex.lock().await;
//             transport.write_bind(params).await?;
//         }
//         self.turn(LoopMode::SingleOp).await?;
//         return rx.await.map_err(|_| Error::from(ErrorKind::BrokenPipe));
//     }

//     async fn request(self: &mut Self, params: RequestParameters<X690Element>) -> Result<OperationOutcome<X690Element, X690Element>> {
//         Err(Error::from(ErrorKind::Unsupported))
//     }

//     async fn unbind(self: &mut Self, params: UnbindParameters<X690Element>) -> Result<UnbindOutcome<X690Element, X690Element>> {
//         Err(Error::from(ErrorKind::Unsupported))
//     }

//     async fn start_tls(self: &mut Self, params: StartTLSParameters) -> Result<StartTLSOutcome> {
//         Err(Error::from(ErrorKind::Unsupported))
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;
    use idm::IdmStream;
    use rose_idm::RoseIdmStream;
    use rose::BindParameters;
    use std::net::ToSocketAddrs;
    // use tokio::net::TcpSocket;
    use x500::DirectoryAbstractService::{DirectoryBindArgument, _encode_DirectoryBindArgument, _decode_DirectoryBindResult};
    use x500::DirectoryIDMProtocols::id_idm_dap;
    pub use tokio;

    #[tokio::test]
    async fn it_works() {
        let mut addrs = "localhost:4632"
            .to_socket_addrs()
            .unwrap();
        let socket = tokio::net::TcpSocket::new_v4().unwrap();
        let stream = socket.connect(addrs.next().unwrap()).await.unwrap();

        let idm = RoseIdmStream(IdmStream::new(stream));
        let mut rose = RoseClient::new(idm, 5);
        // tokio::spawn(async move {
        //     if let Err(e) = rose.drive().await {
        //         println!("ROSE transport error: {}", e);
        //     }
        // });
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
            BindOutcome::Result(result) => {
                let dbr = _decode_DirectoryBindResult(&result.parameter).unwrap();
                println!("Made it, big dawg. Versions supported: {:?}", dbr.versions);
            },
            _ => panic!(),
        };
    }
}
