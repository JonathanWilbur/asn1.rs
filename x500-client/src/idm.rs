use crate::RoseStream;
use asn1::ENUMERATED;
use async_trait::async_trait;
use idm::IdmStream;
use rose_transport::{
    AbortReason, BindParameters, BindResultOrErrorParameters, ROSEReceiver, ROSETransmitter,
    RejectParameters, RejectReason, RequestParameters, ResultOrErrorParameters, RosePDU,
    StartTLSParameters, TLSResponseParameters, UnbindParameters, AsyncRoseClient,
    BindOutcome, OperationOutcome, UnbindOutcome, StartTLSOutcome,
};
use std::io::{Error, ErrorKind, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::oneshot;
use x500::{CommonProtocolSpecification::InvokeId, IDMProtocolSpecification::*};
use x690::{ber_cst, write_x690_node, X690Element};
use std::sync::{Arc, RwLock};
use std::future::Future;

pub type X500ROSEPDU = rose_transport::RosePDU<X690Element>;

// TODO: import { protocol_id_to_rose_protocol, app_context_to_protocol_id } from "./utils";

// TODO: pub fn rose_transport_from_idm_socket (idm: &mut IDMSocket) -> ROSETransport {

// }

#[inline]
fn reject_reason_to_integer(rr: RejectReason) -> Option<ENUMERATED> {
    match rr {
        RejectReason::MistypedPDU => Some(IdmReject_reason_mistypedPDU),
        RejectReason::DuplicateInvokeIdRequest => Some(IdmReject_reason_duplicateInvokeIDRequest),
        RejectReason::UnsupportedOperationRequest => {
            Some(IdmReject_reason_unsupportedOperationRequest)
        }
        RejectReason::UnknownOperationRequest => Some(IdmReject_reason_unknownOperationRequest),
        RejectReason::MistypedArgumentRequest => Some(IdmReject_reason_mistypedArgumentRequest),
        RejectReason::ResourceLimitationRequest => Some(IdmReject_reason_resourceLimitationRequest),
        RejectReason::UnknownInvokeIdResult => Some(IdmReject_reason_unknownInvokeIDResult),
        RejectReason::MistypedResultRequest => Some(IdmReject_reason_mistypedResultRequest),
        RejectReason::UnknownInvokeIdError => Some(IdmReject_reason_unknownInvokeIDError),
        RejectReason::UnknownError => Some(IdmReject_reason_unknownError),
        RejectReason::MistypedParameterError => Some(IdmReject_reason_mistypedParameterError),
        RejectReason::UnsupportedIDMVersion => Some(IdmReject_reason_unsupportedIdmVersion),
        RejectReason::UnsuitableIDMVersion => Some(IdmReject_reason_unsuitableIdmVersion),
        RejectReason::InvalidIDMVersion => Some(IdmReject_reason_invalidIdmVersion),
        RejectReason::UnrecognizedPDU => Some(IdmReject_reason_mistypedPDU),
        RejectReason::BadlyStructuredPDU => Some(IdmReject_reason_mistypedPDU),
        RejectReason::ReleaseInProgress => Some(IdmReject_reason_mistypedPDU),
        RejectReason::Other => Some(IdmReject_reason_duplicateInvokeIDRequest),
    }
}

#[inline]
fn integer_to_reject_reason(rr: ENUMERATED) -> Option<RejectReason> {
    match rr {
        IdmReject_reason_duplicateInvokeIDRequest => Some(RejectReason::DuplicateInvokeIdRequest),
        IdmReject_reason_invalidIdmVersion => Some(RejectReason::InvalidIDMVersion),
        IdmReject_reason_mistypedArgumentRequest => Some(RejectReason::MistypedArgumentRequest),
        IdmReject_reason_mistypedParameterError => Some(RejectReason::MistypedParameterError),
        IdmReject_reason_mistypedPDU => Some(RejectReason::MistypedPDU),
        IdmReject_reason_mistypedResultRequest => Some(RejectReason::MistypedResultRequest),
        IdmReject_reason_resourceLimitationRequest => Some(RejectReason::ResourceLimitationRequest),
        IdmReject_reason_unknownError => Some(RejectReason::UnknownError),
        IdmReject_reason_unknownInvokeIDError => Some(RejectReason::UnknownInvokeIdError),
        IdmReject_reason_unknownInvokeIDResult => Some(RejectReason::UnknownInvokeIdResult),
        IdmReject_reason_unknownOperationRequest => Some(RejectReason::UnknownOperationRequest),
        IdmReject_reason_unsuitableIdmVersion => Some(RejectReason::UnsuitableIDMVersion),
        IdmReject_reason_unsupportedIdmVersion => Some(RejectReason::UnsupportedIDMVersion),
        IdmReject_reason_unsupportedOperationRequest => {
            Some(RejectReason::UnsupportedOperationRequest)
        }
        _ => None,
    }
}

#[inline]
fn abort_reason_to_integer(ar: AbortReason) -> Option<ENUMERATED> {
    match ar {
        AbortReason::MistypedPDU => Some(Abort_mistypedPDU),
        AbortReason::UnboundRequest => Some(Abort_unboundRequest),
        AbortReason::InvalidPDU => Some(Abort_invalidPDU),
        AbortReason::ResourceLimitation => Some(Abort_resourceLimitation),
        AbortReason::ConnectionFailed => Some(Abort_connectionFailed),
        AbortReason::InvalidProtocol => Some(Abort_invalidProtocol),
        AbortReason::ReasonNotSpecified => Some(Abort_reasonNotSpecified),
        AbortReason::ProtocolError => Some(Abort_reasonNotSpecified),
        AbortReason::AuthenticationMechanismNameNotRecognized => Some(Abort_reasonNotSpecified),
        AbortReason::AuthenticationMechanismNameRequired => Some(Abort_reasonNotSpecified),
        AbortReason::AuthenticationFailure => Some(Abort_reasonNotSpecified),
        AbortReason::AuthenticationRequired => Some(Abort_reasonNotSpecified),
        AbortReason::Other => Some(Abort_reasonNotSpecified),
    }
}

#[inline]
fn integer_to_abort_reason(ar: ENUMERATED) -> Option<AbortReason> {
    match ar {
        Abort_mistypedPDU => Some(AbortReason::MistypedPDU),
        Abort_unboundRequest => Some(AbortReason::UnboundRequest),
        Abort_invalidPDU => Some(AbortReason::InvalidPDU),
        Abort_resourceLimitation => Some(AbortReason::ResourceLimitation),
        Abort_connectionFailed => Some(AbortReason::ConnectionFailed),
        Abort_invalidProtocol => Some(AbortReason::InvalidProtocol),
        Abort_reasonNotSpecified => Some(AbortReason::ReasonNotSpecified),
        _ => None,
    }
}

#[inline]
async fn write_idm_pdu<W: AsyncWriteExt + AsyncReadExt + Unpin + Send>(
    idm: &mut RoseIdmStream<W>,
    pdu: &IDM_PDU,
) -> Result<usize> {
    // TODO: Do something more useful with these errors.
    match _encode_IDM_PDU(&pdu) {
        Ok(element) => {
            let mut bytes: Vec<u8> = Vec::new();
            if let Err(_e) = write_x690_node(&mut bytes, &element) {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
            idm.0.write_pdu(&bytes, 0).await
        }
        Err(_e) => Err(Error::from(ErrorKind::InvalidInput)),
    }
}

pub struct RoseIdmStream<W: AsyncWriteExt + AsyncReadExt + Unpin + Send>(IdmStream<W>);

#[async_trait]
impl<W: AsyncWriteExt + AsyncReadExt + Unpin + Send> ROSETransmitter<X690Element>
    for RoseIdmStream<W>
{
    async fn write_bind(self: &mut Self, params: BindParameters<X690Element>) -> Result<usize> {
        let idm_bind = IdmBind::new(
            params.protocol_id,
            params.calling_ae_title,
            params.called_ae_title,
            params.parameter,
            vec![],
        );
        let idm_pdu = IDM_PDU::bind(idm_bind);
        write_idm_pdu(&mut self, &idm_pdu).await
    }

    async fn write_bind_result(
        self: &mut Self,
        params: BindResultOrErrorParameters<X690Element>,
    ) -> Result<usize> {
        let idm_br = IdmBindResult::new(
            params.protocol_id,
            params.responding_ae_title,
            params.parameter,
            vec![],
        );
        let idm_pdu = IDM_PDU::bindResult(idm_br);
        write_idm_pdu(&mut self, &idm_pdu).await
    }

    async fn write_bind_error(
        self: &mut Self,
        params: &BindResultOrErrorParameters<X690Element>,
    ) -> Result<usize> {
        let idm_be = IdmBindError::new(
            params.protocol_id,
            params.responding_ae_title,
            None,
            params.parameter,
            vec![],
        );
        let idm_pdu = IDM_PDU::bindError(idm_be);
        write_idm_pdu(&mut self, &idm_pdu).await
    }

    async fn write_request(
        self: &mut Self,
        params: &'static RequestParameters<X690Element>,
    ) -> Result<usize> {
        // TODO: REVIEW: should this be required by this API?
        if let InvokeId::present(iid) = params.invoke_id {
            let request = Request::new(iid, params.code, params.parameter, vec![]);
            let idm_pdu = IDM_PDU::request(request);
            write_idm_pdu(&mut self, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_result(
        self: &mut Self,
        params: ResultOrErrorParameters<X690Element>,
    ) -> Result<usize> {
        if let InvokeId::present(iid) = params.invoke_id {
            let result = IdmResult::new(iid, params.code, params.parameter, vec![]);
            let idm_pdu = IDM_PDU::result(result);
            write_idm_pdu(&mut self, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_error(
        self: &mut Self,
        params: ResultOrErrorParameters<X690Element>,
    ) -> Result<usize> {
        if let InvokeId::present(iid) = params.invoke_id {
            let error = x500::IDMProtocolSpecification::Error::new(
                iid,
                params.code,
                params.parameter,
                vec![],
            );
            let idm_pdu = IDM_PDU::error(error);
            write_idm_pdu(&mut self, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_reject(self: &mut Self, params: RejectParameters) -> Result<usize> {
        if let InvokeId::present(iid) = params.invoke_id {
            let reject = IdmReject::new(
                iid,
                reject_reason_to_integer(params.reason).unwrap_or_default(),
                vec![],
            );
            let idm_pdu = IDM_PDU::reject(reject);
            write_idm_pdu(&mut self, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_unbind(
        self: &mut Self,
        _params: UnbindParameters<X690Element>,
    ) -> Result<usize> {
        let idm_pdu = IDM_PDU::unbind(());
        write_idm_pdu(&mut self, &idm_pdu).await
    }

    async fn write_abort(self: &mut Self, reason: AbortReason) -> Result<usize> {
        let idm_pdu = IDM_PDU::abort(abort_reason_to_integer(reason).unwrap_or_default());
        write_idm_pdu(&mut self, &idm_pdu).await
    }
}

impl<W: AsyncWriteExt + AsyncReadExt + Unpin + Send> RoseIdmStream<W> {
    pub fn receive_idm_pdu(&mut self, encoding_and_bytes: (u16, Vec<u8>)) -> Result<()> {
        let (encoding, idm_pdu_bytes) = encoding_and_bytes;
        if idm_pdu_bytes.len() > 0 // If there is at least a first byte to the framed IDM PDU...
            && idm_pdu_bytes[0] != 0xA0 // ...and the PDU is a bind, and...
            && encoding > 1
        // ...something other than BER or DER encoding is used...
        {
            // We don't support this encoding, so we return an error.
            return Err(Error::from(ErrorKind::InvalidData));
        }
        let idm_pdu_element = match ber_cst(&idm_pdu_bytes) {
            Ok((bytes_read, element)) => {
                if bytes_read != idm_pdu_bytes.len() {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
                element
            }
            Err(_e) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        let pdu = match _decode_IDM_PDU(&idm_pdu_element) {
            Ok(pdu) => pdu,
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        let rose_pdu = match pdu {
            IDM_PDU::bind(bind) => Ok(RosePDU::Bind(BindParameters {
                protocol_id: bind.protocolID,
                calling_ae_title: bind.callingAETitle,
                called_ae_title: bind.calledAETitle,
                parameter: bind.argument,
                implementation_information: None,
                called_ae_invocation_identifier: None,
                called_ap_invocation_identifier: None,
                calling_ae_invocation_identifier: None,
                calling_ap_invocation_identifier: None,
                timeout: 0,
            })),
            IDM_PDU::bindResult(bind_result) => {
                Ok(RosePDU::BindResult(BindResultOrErrorParameters {
                    protocol_id: bind_result.protocolID,
                    parameter: bind_result.result,
                    responding_ae_title: bind_result.respondingAETitle,
                    responding_ae_invocation_identifier: None,
                    responding_ap_invocation_identifier: None,
                }))
            }
            IDM_PDU::bindError(bind_error) => Ok(RosePDU::BindError(BindResultOrErrorParameters {
                protocol_id: bind_error.protocolID,
                parameter: bind_error.error,
                responding_ae_title: bind_error.respondingAETitle,
                responding_ae_invocation_identifier: None,
                responding_ap_invocation_identifier: None,
            })),
            IDM_PDU::request(request) => Ok(RosePDU::Request(RequestParameters {
                code: request.opcode,
                invoke_id: InvokeId::present(request.invokeID),
                parameter: request.argument,
                linked_id: None,
            })),
            IDM_PDU::result(result) => Ok(RosePDU::Result(ResultOrErrorParameters {
                code: result.opcode,
                invoke_id: InvokeId::present(result.invokeID),
                parameter: result.result,
            })),
            IDM_PDU::error(error) => Ok(RosePDU::Error(ResultOrErrorParameters {
                code: error.errcode,
                invoke_id: InvokeId::present(error.invokeID),
                parameter: error.error,
            })),
            IDM_PDU::reject(reject) => Ok(RosePDU::Reject(RejectParameters {
                invoke_id: InvokeId::present(reject.invokeID),
                reason: integer_to_reject_reason(reject.reason).unwrap_or_default(),
            })),
            IDM_PDU::unbind(_unbind) => Ok(RosePDU::Unbind(UnbindParameters {
                parameter: None,
                timeout: 0,
            })),
            IDM_PDU::abort(abort) => Ok(RosePDU::Abort(
                integer_to_abort_reason(abort).unwrap_or_default(),
            )),
            IDM_PDU::startTLS(_start_tls) => Ok(RosePDU::StartTLS(StartTLSParameters::default())),
            IDM_PDU::tLSResponse(tls_response) => {
                Ok(RosePDU::StartTLSResponse(TLSResponseParameters {
                    code: tls_response.max(99) as u8,
                }))
            }
            IDM_PDU::_unrecognized(_unrecognized) => Err(Error::from(ErrorKind::Unsupported)),
        }?;
        self.0.received_pdus.push_back(rose_pdu);
        let future_state = self.0.future_state.lock().unwrap();
        if let Some(waker) = &future_state.waker {
            waker.wake_by_ref(); // TODO: Most inefficient way of waking used here. Hover to see docs.
        }
        Ok(())
    }
}

#[async_trait]
impl<W: AsyncWriteExt + AsyncReadExt + Unpin + Send> ROSEReceiver<X690Element>
    for RoseIdmStream<W>
{
    async fn read_pdu(&mut self) -> Result<Option<rose_transport::RosePDU<X690Element>>> {
        let idm_frame_or_not = self.0.transport.read_pdu().await?;
        let (encoding, idm_pdu_bytes) = match &idm_frame_or_not {
            Some(frame) => frame,
            None => return Ok(None),
        };
        if idm_pdu_bytes.len() > 0 // If there is at least a first byte to the framed IDM PDU...
            && idm_pdu_bytes[0] != 0xA0 // ...and the PDU is a bind, and...
            && *encoding > 1
        // ...something other than BER or DER encoding is used...
        {
            // We don't support this encoding, so we return an error.
            return Err(Error::from(ErrorKind::InvalidData));
        }
        let idm_pdu_element = match ber_cst(&idm_pdu_bytes) {
            Ok((bytes_read, element)) => {
                if bytes_read != idm_pdu_bytes.len() {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
                element
            }
            Err(_e) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        let pdu = match _decode_IDM_PDU(&idm_pdu_element) {
            Ok(pdu) => pdu,
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        let rose_pdu = match pdu {
            IDM_PDU::bind(bind) => Ok(RosePDU::Bind(BindParameters {
                protocol_id: bind.protocolID,
                calling_ae_title: bind.callingAETitle,
                called_ae_title: bind.calledAETitle,
                parameter: bind.argument,
                implementation_information: None,
                called_ae_invocation_identifier: None,
                called_ap_invocation_identifier: None,
                calling_ae_invocation_identifier: None,
                calling_ap_invocation_identifier: None,
                timeout: 0,
            })),
            IDM_PDU::bindResult(bind_result) => {
                Ok(RosePDU::BindResult(BindResultOrErrorParameters {
                    protocol_id: bind_result.protocolID,
                    parameter: bind_result.result,
                    responding_ae_title: bind_result.respondingAETitle,
                    responding_ae_invocation_identifier: None,
                    responding_ap_invocation_identifier: None,
                }))
            }
            IDM_PDU::bindError(bind_error) => Ok(RosePDU::BindError(BindResultOrErrorParameters {
                protocol_id: bind_error.protocolID,
                parameter: bind_error.error,
                responding_ae_title: bind_error.respondingAETitle,
                responding_ae_invocation_identifier: None,
                responding_ap_invocation_identifier: None,
            })),
            IDM_PDU::request(request) => Ok(RosePDU::Request(RequestParameters {
                code: request.opcode,
                invoke_id: InvokeId::present(request.invokeID),
                parameter: request.argument,
                linked_id: None,
            })),
            IDM_PDU::result(result) => Ok(RosePDU::Result(ResultOrErrorParameters {
                code: result.opcode,
                invoke_id: InvokeId::present(result.invokeID),
                parameter: result.result,
            })),
            IDM_PDU::error(error) => Ok(RosePDU::Error(ResultOrErrorParameters {
                code: error.errcode,
                invoke_id: InvokeId::present(error.invokeID),
                parameter: error.error,
            })),
            IDM_PDU::reject(reject) => Ok(RosePDU::Reject(RejectParameters {
                invoke_id: InvokeId::present(reject.invokeID),
                reason: integer_to_reject_reason(reject.reason).unwrap_or_default(),
            })),
            IDM_PDU::unbind(_unbind) => Ok(RosePDU::Unbind(UnbindParameters {
                parameter: None,
                timeout: 0,
            })),
            IDM_PDU::abort(abort) => Ok(RosePDU::Abort(
                integer_to_abort_reason(abort).unwrap_or_default(),
            )),
            IDM_PDU::startTLS(_start_tls) => Ok(RosePDU::StartTLS(StartTLSParameters::default())),
            IDM_PDU::tLSResponse(tls_response) => {
                Ok(RosePDU::StartTLSResponse(TLSResponseParameters {
                    code: tls_response.max(99) as u8,
                }))
            }
            IDM_PDU::_unrecognized(_unrecognized) => {
                Err(Error::from(ErrorKind::Unsupported))
            }
        }?;
        let future_state = self.future_state.lock().unwrap();
        if let Some(waker) = &future_state.waker {
            waker.wake_by_ref(); // TODO: Most inefficient way of waking used here. Hover to see docs.
        }
        Ok(Some(rose_pdu))
    }
}

// #[async_trait]
// impl<W: AsyncWriteExt + AsyncReadExt + Unpin + Send> AsyncRoseClient<X690Element>
//     for RoseStream<X690Element, IdmStream<W>> {

//     async fn bind(self: &mut Self, params: BindParameters<X690Element>) -> Result<BindOutcome<X690Element, X690Element>> {

//     }

//     async fn request(self: &mut Self, params: BindParameters<X690Element>) -> Result<OperationOutcome<X690Element, X690Element>> {
//         let (resp_tx, resp_rx) = oneshot::channel::<OperationOutcome<X690Element, X690Element>>();
//         let req_tx = self.request_tx.clone();
//         tokio::spawn(async move {
//             self.write_request(params);
//             req_tx.send(params);
//         });
//         let resp = resp_rx.await;
//     }

//     async fn unbind(self: &mut Self, params: BindParameters<X690Element>) -> Result<UnbindOutcome<X690Element, X690Element>> {

//     }

//     async fn start_tls(self: &mut Self, params: BindParameters<X690Element>) -> Result<StartTLSOutcome> {

//     }

// }

// impl <W : AsyncWriteExt + AsyncReadExt + Unpin> Future for RoseStream<IdmStream<W>> {
//     type Output = RosePDU<X690Element>;
//     fn poll(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Self::Output> {
//         let mut_self = self.get_mut();
//         match mut_self.read_pdu() {
//             Some(pdu) => Poll::Ready(pdu),
//             None => {
//                 let mut future_state = mut_self.future_state.lock().unwrap();
//                 future_state.waker = Some(cx.waker().clone());
//                 Poll::Pending
//             },
//         }
//     }
// }

// impl <W : AsyncWriteExt + AsyncReadExt + Unpin> Iterator for RoseStream<IdmStream<W>> {
//     type Item = rose_transport::RosePDU<X690Element>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.read_pdu()
//     }
// }

// impl <W : AsyncWriteExt + AsyncReadExt + Unpin> Stream for RoseStream<IdmStream<W>> {
//     type Item = RosePDU<X690Element>;

//     fn poll_next(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>
//     ) -> Poll<Option<Self::Item>> {
//         let mut_self = self.get_mut();
//         match mut_self.read_pdu() {
//             Some(pdu) => Poll::Ready(Some(pdu)),
//             None => {
//                 let mut future_state = mut_self.future_state.lock().unwrap();
//                 future_state.waker = Some(cx.waker().clone());
//                 Poll::Pending
//             },
//         }
//     }
// }

// export
// function rose_transport_from_idm_socket (idm: IDMConnection): ROSETransport {
//     const rose = new_rose_transport(idm.socket);
//     idm.events.on("bind", (params) => {
//         rose.events.emit("bind", {
//             protocol_id: params.protocolID,
//             parameter: params.argument,
//             called_ae_title: params.calledAETitle,
//             calling_ae_title: params.callingAETitle,
//         });
//     });
//     idm.events.on("bindResult", (params) => {
//         rose.is_bound = true;
//         rose.events.emit("bind_result", {
//             protocol_id: params.protocolID,
//             responding_ae_title: params.respondingAETitle,
//             parameter: params.result,
//         });
//     });
//     idm.events.on("bindError", (params) => {
//         rose.events.emit("bind_error", {
//             protocol_id: params.protocolID,
//             responding_ae_title: params.respondingAETitle,
//             parameter: params.error,
//         });
//     });
//     idm.events.on("request", (params) => {
//         rose.events.emit("request", {
//             invoke_id: {
//                 present: params.invokeID,
//             },
//             code: params.opcode,
//             parameter: params.argument,
//         });
//     });
//     idm.events.on("result", (params) => {
//         rose.events.emit("result", {
//             invoke_id: {
//                 present: params.invokeID,
//             },
//             code: params.opcode,
//             parameter: params.result,
//         });
//     });
//     idm.events.on("error_", (params) => {
//         rose.events.emit("error_", {
//             invoke_id: {
//                 present: params.invokeID,
//             },
//             code: params.errcode,
//             parameter: params.error,
//         });
//     });
//     idm.events.on("reject", (params) => {
//         rose.events.emit("reject", {
//             invoke_id: {
//                 present: params.invokeID,
//             },
//             problem: idm_reject_to_rose_reject.get(params.reason)
//                 ?? RejectReason.other,
//         });
//     });
//     idm.events.on("unbind", () => rose.events.emit("unbind"));
//     idm.events.on("abort", (params) => rose.events
//         .emit("abort", (idm_abort_to_rose_abort.get(params) ?? AbortReason.other)));
//     idm.events.on("tLSResponse", (code) => {
//         rose.events.emit("start_tls_response", { code });
//     });
