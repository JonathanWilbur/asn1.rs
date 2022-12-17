use std::io::{Result, ErrorKind, Error};
use asn1::ENUMERATED;
use idm::IdmStream;
use rose_transport::{
    ROSEReceiver,
    ROSETransmitter,
    BindParameters,
    BindResultOrErrorParameters,
    RequestParameters,
    ResultOrErrorParameters,
    RejectParameters,
    UnbindParameters,
    AbortReason,
    RejectReason,
    RosePDU,
    StartTLSParameters,
    TLSResponseParameters,
};
use tokio::io::AsyncWriteExt;
use x500::{IDMProtocolSpecification::*, CommonProtocolSpecification::InvokeId};
use x690::{X690Element, write_x690_node, ber_cst};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::ROSEClient;

pub type X500ROSEPDU = rose_transport::RosePDU<X690Element>;

// TODO: import { protocol_id_to_rose_protocol, app_context_to_protocol_id } from "./utils";

// TODO: pub fn rose_transport_from_idm_socket (idm: &mut IDMSocket) -> ROSETransport {

// }

#[inline]
fn reject_reason_to_integer (rr: RejectReason) -> Option<ENUMERATED> {
    match rr {
        RejectReason::MistypedPDU => Some(IdmReject_reason_mistypedPDU),
        RejectReason::DuplicateInvokeIdRequest => Some(IdmReject_reason_duplicateInvokeIDRequest),
        RejectReason::UnsupportedOperationRequest => Some(IdmReject_reason_unsupportedOperationRequest),
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
fn integer_to_reject_reason (rr: ENUMERATED) -> Option<RejectReason> {
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
        IdmReject_reason_unsupportedOperationRequest => Some(RejectReason::UnsupportedOperationRequest),
        _ => None,
    }
}

#[inline]
fn abort_reason_to_integer (ar: AbortReason) -> Option<ENUMERATED> {
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
fn integer_to_abort_reason (ar: ENUMERATED) -> Option<AbortReason> {
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
async fn write_idm_pdu <W : AsyncWriteExt + Unpin> (idm: &mut IdmStream<W>, pdu: &IDM_PDU) -> Result<usize> {
    // TODO: Do something more useful with these errors.
    match _encode_IDM_PDU(&pdu) {
        Ok(element) => {
            let mut bytes: Vec<u8> = Vec::new();
            if let Err(_e) = write_x690_node(&mut bytes, &element) {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
            idm.write_pdu(&bytes, 0).await
        },
        Err(_e) => Err(Error::from(ErrorKind::InvalidInput)),
    }
}

// pub struct X500ROSEPDUFuture <'a, T> {
//     pub client: &'a mut ROSEClient<T>,
// }

// impl <'a, T> ROSEClient<T> {

//     fn read (&mut self) -> X500ROSEPDUFuture <'a, T> {
//         return X500ROSEPDUFuture { client: self }
//     }

// }

// impl <'a, T> Future for X500ROSEPDUFuture<'a, T> {
//     type Output = X500ROSEPDUFuture<'a, T>;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        
//         // if Instant::now() >= self.when {
//         //     println!("Hello world");
//         //     Poll::Ready("done")
//         // } else {
//         //     cx.waker().wake_by_ref();
//         //     Poll::Pending
//         // }
//     }
// }

#[async_trait]
impl <W : AsyncWriteExt + Unpin + Send> ROSETransmitter<X690Element> for ROSEClient<IdmStream<W>> {

    async fn write_bind (self: &mut Self, params: BindParameters<X690Element>) -> Result<usize> {
        let idm_bind = IdmBind::new(
            params.protocol_id,
            params.calling_ae_title,
            params.called_ae_title,
            params.parameter,
            vec![],
        );
        let idm_pdu = IDM_PDU::bind(idm_bind);
        write_idm_pdu(&mut self.transport, &idm_pdu).await
    }

    async fn write_bind_result (self: &mut Self, params: BindResultOrErrorParameters<X690Element>) -> Result<usize> {
        let idm_br = IdmBindResult::new(
            params.protocol_id,
            params.responding_ae_title,
            params.parameter,
            vec![],
        );
        let idm_pdu = IDM_PDU::bindResult(idm_br);
        write_idm_pdu(&mut self.transport, &idm_pdu).await
    }

    async fn write_bind_error (self: &mut Self, params: BindResultOrErrorParameters<X690Element>) -> Result<usize> {
        let idm_be = IdmBindError::new(
            params.protocol_id,
            params.responding_ae_title,
            None,
            params.parameter,
            vec![],
        );
        let idm_pdu = IDM_PDU::bindError(idm_be);
        write_idm_pdu(&mut self.transport, &idm_pdu).await
    }

    async fn write_request (self: &mut Self, params: RequestParameters<X690Element>) -> Result<usize> {
        // TODO: REVIEW: should this be required by this API?
        if let InvokeId::present(iid) = params.invoke_id {
            let request = Request::new(
                iid,
                params.code,
                params.parameter,
                vec![],
            );
            let idm_pdu = IDM_PDU::request(request);
            write_idm_pdu(&mut self.transport, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_result (self: &mut Self, params: ResultOrErrorParameters<X690Element>) -> Result<usize> {
        if let InvokeId::present(iid) = params.invoke_id {
            let result = IdmResult::new(
                iid,
                params.code,
                params.parameter,
                vec![],
            );
            let idm_pdu = IDM_PDU::result(result);
            write_idm_pdu(&mut self.transport, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_error (self: &mut Self, params: ResultOrErrorParameters<X690Element>) -> Result<usize> {
        if let InvokeId::present(iid) = params.invoke_id {
            let error = x500::IDMProtocolSpecification::Error::new(
                iid,
                params.code,
                params.parameter,
                vec![],
            );
            let idm_pdu = IDM_PDU::error(error);
            write_idm_pdu(&mut self.transport, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_reject (self: &mut Self, params: RejectParameters) -> Result<usize> {
        if let InvokeId::present(iid) = params.invoke_id {
            let reject = IdmReject::new(
                iid,
                reject_reason_to_integer(params.reason).unwrap_or_default(),
                vec![],
            );
            let idm_pdu = IDM_PDU::reject(reject);
            write_idm_pdu(&mut self.transport, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_unbind (self: &mut Self, _params: UnbindParameters<X690Element>) -> Result<usize> {
        let idm_pdu = IDM_PDU::unbind(());
        write_idm_pdu(&mut self.transport, &idm_pdu).await
    }

    async fn write_abort (self: &mut Self, reason: AbortReason) -> Result<usize> {
        let idm_pdu = IDM_PDU::abort(abort_reason_to_integer(reason).unwrap_or_default());
        write_idm_pdu(&mut self.transport, &idm_pdu).await
    }

}

impl <W : AsyncWriteExt + Unpin> ROSEReceiver<X690Element, std::io::Error> for ROSEClient<IdmStream<W>> {

    fn read_rose_pdu (&mut self) -> Result<Option<rose_transport::RosePDU<X690Element>>> {
        let (encoding, idm_pdu_bytes) = match self.transport.read_pdu() {
            Some((encoding, idm_pdu_bytes)) => (encoding, idm_pdu_bytes),
            None => return Ok(None),
        };
        if
            idm_pdu_bytes.len() > 0 // If there is at least a first byte to the framed IDM PDU...
            && idm_pdu_bytes[0] != 0xA0 // ...and the PDU is a bind, and...
            && encoding > 1 // ...something other than BER or DER encoding is used...
        { // We don't support this encoding, so we return an error.
            return Err(Error::from(ErrorKind::InvalidData));
        }
        let idm_pdu_element = match ber_cst(&idm_pdu_bytes) {
            Ok((bytes_read, element)) => {
                if bytes_read != idm_pdu_bytes.len() {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
                element
            },
            Err(_e) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        let pdu = match _decode_IDM_PDU(&idm_pdu_element) {
            Ok(pdu) => pdu,
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };
        match pdu {
            IDM_PDU::bind(bind) => {
                Ok(Some(RosePDU::Bind(BindParameters {
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
                })))
            },
            IDM_PDU::bindResult(bind_result) => {
                Ok(Some(RosePDU::BindResult(BindResultOrErrorParameters {
                    protocol_id: bind_result.protocolID,
                    parameter: bind_result.result,
                    responding_ae_title: bind_result.respondingAETitle,
                    responding_ae_invocation_identifier: None,
                    responding_ap_invocation_identifier: None,
                })))
            },
            IDM_PDU::bindError(bind_error) => {
                Ok(Some(RosePDU::BindError(BindResultOrErrorParameters {
                    protocol_id: bind_error.protocolID,
                    parameter: bind_error.error,
                    responding_ae_title: bind_error.respondingAETitle,
                    responding_ae_invocation_identifier: None,
                    responding_ap_invocation_identifier: None,
                })))
            },
            IDM_PDU::request(request) => {
                Ok(Some(RosePDU::Request(RequestParameters {
                    code: request.opcode,
                    invoke_id: InvokeId::present(request.invokeID),
                    parameter: request.argument,
                    linked_id: None,
                })))
            },
            IDM_PDU::result(result) => {
                Ok(Some(RosePDU::Result(ResultOrErrorParameters {
                    code: result.opcode,
                    invoke_id: InvokeId::present(result.invokeID),
                    parameter: result.result,
                })))
            },
            IDM_PDU::error(error) => {
                Ok(Some(RosePDU::Error(ResultOrErrorParameters {
                    code: error.errcode,
                    invoke_id: InvokeId::present(error.invokeID),
                    parameter: error.error,
                })))
            },
            IDM_PDU::reject(reject) => {
                Ok(Some(RosePDU::Reject(RejectParameters {
                    invoke_id: InvokeId::present(reject.invokeID),
                    reason: integer_to_reject_reason(reject.reason).unwrap_or_default(),
                })))
            },
            IDM_PDU::unbind(_unbind) => {
                Ok(Some(RosePDU::Unbind(UnbindParameters {
                    parameter: None,
                    timeout: 0,
                })))
            },
            IDM_PDU::abort(abort) => {
                Ok(Some(RosePDU::Abort(integer_to_abort_reason(abort).unwrap_or_default())))
            },
            IDM_PDU::startTLS(_start_tls) => {
                Ok(Some(RosePDU::StartTLS(StartTLSParameters::default())))
            },
            IDM_PDU::tLSResponse(tls_response) => {
                Ok(Some(RosePDU::StartTLSResponse(TLSResponseParameters {
                    code: tls_response.max(99) as u8,
                })))
            },
            IDM_PDU::_unrecognized(_unrecognized) => {
                Err(Error::from(ErrorKind::Unsupported))
            },
        }
    }

}

impl <W : AsyncWriteExt + Unpin> Iterator for ROSEClient<IdmStream<W>> {
    type Item = std::io::Result<rose_transport::RosePDU<X690Element>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.read_rose_pdu() {
            Ok(pdu_or_not) => {
                if let Some(pdu) = pdu_or_not {
                    return Some(Ok(pdu));
                } else {
                    return None;
                }
            },
            Err(e) => return Some(Err(e)),
        }
    }
}

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
