#![allow(non_upper_case_globals)]

use asn1::ENUMERATED;
use async_trait::async_trait;
use idm::IdmStream;
use rose::{
    AbortReason,
    BindParameters,
    BindResultOrErrorParameters,
    ROSEReceiver,
    ROSETransmitter,
    RejectParameters,
    RejectReason,
    RequestParameters,
    ResultOrErrorParameters,
    RosePDU,
    StartTLSParameters,
    TLSResponseParameters,
    UnbindParameters,
    BindOutcome,
    OperationOutcome,
    UnbindOutcome,
    StartTLSOutcome,
    InvokeIdInt,
    Resettable,
    RoseAbort,
    RoseEngine,
    LoopMode,
    BindArgAndTx,
    RequestArgAndTx,
    UnbindArgAndTx,
    StartTlsArgAndTx,
};
use tokio::sync::mpsc::UnboundedSender;
use std::io::{Error, ErrorKind, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{oneshot, Mutex};
use x500::{CommonProtocolSpecification::InvokeId, IDMProtocolSpecification::*};
use x690::{ber_cst, write_x690_node, X690Element};
use std::collections::HashMap;
use std::sync::Arc;

pub type X500ROSEPDU = rose::RosePDU<X690Element>;

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
async fn write_idm_pdu<W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync>(
    stream: &mut RoseIdmStream<W>,
    pdu: &IDM_PDU,
) -> Result<usize> {
    // TODO: Do something more useful with these errors.
    match _encode_IDM_PDU(&pdu) {
        Ok(element) => {
            let mut bytes: Vec<u8> = Vec::new();
            if let Err(_e) = write_x690_node(&mut bytes, &element) {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
            let mut idm_stream = stream.idm.lock().await;
            idm_stream.write_pdu(&bytes, 0).await
        }
        Err(_e) => Err(Error::from(ErrorKind::InvalidInput)),
    }
}

pub struct RoseIdmStream<W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> {
    pub idm: Arc<Mutex<IdmStream<W>>>,
    pub outstanding_requests: HashMap<InvokeIdInt, oneshot::Sender<OperationOutcome<X690Element, X690Element>>>,
    pub concurrency: u32,
    bound: bool,
    outstanding_bind_req: Option<oneshot::Sender<BindOutcome<X690Element, X690Element>>>,
    outstanding_unbind_req: Option<oneshot::Sender<UnbindOutcome<X690Element, X690Element>>>,
    outstanding_starttls_req: Option<oneshot::Sender<StartTLSOutcome>>,
}

impl <W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> RoseIdmStream<W> {

    pub fn new (idm: Arc<Mutex<IdmStream<W>>>) -> Self {
        RoseIdmStream {
            idm,
            outstanding_requests: HashMap::new(),
            concurrency: 5,
            bound: false,
            outstanding_bind_req: None,
            outstanding_unbind_req: None,
            outstanding_starttls_req: None,
        }
    }

}

#[async_trait]
impl <W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> RoseEngine<X690Element> for RoseIdmStream<W> {

    /// Repeatedly poll the connection until it exits.
    async fn drive(
        &mut self,
        outbound_binds: BindArgAndTx<X690Element>,
        outbound_requests: RequestArgAndTx<X690Element>,
        outbound_unbinds: UnbindArgAndTx<X690Element>,
        outbound_start_tls: StartTlsArgAndTx,
        inbound_rose_pdus_tx: UnboundedSender<RosePDU<X690Element>>,
    ) -> Result<()> {
        self.turn(
            LoopMode::Continuous,
            outbound_binds,
            outbound_requests,
            outbound_unbinds,
            outbound_start_tls,
            inbound_rose_pdus_tx,
        ).await
    }

    async fn turn(
        &mut self,
        mode: LoopMode,
        mut outbound_binds: BindArgAndTx<X690Element>,
        mut outbound_requests: RequestArgAndTx<X690Element>,
        mut outbound_unbinds: UnbindArgAndTx<X690Element>,
        mut outbound_start_tls: StartTlsArgAndTx,
        inbound_rose_pdus_tx: UnboundedSender<RosePDU<X690Element>>,
    ) -> Result<()> {
        loop {
            tokio::select! {
                // TODO: Receive requests
                maybe_outbound_bind = outbound_binds.recv() => {
                    if maybe_outbound_bind.is_none() {
                        continue;
                    }
                    let (bind_req, bind_tx) = maybe_outbound_bind.unwrap();
                    self.outstanding_bind_req = Some(bind_tx);
                    self.write_bind(bind_req).await?;
                },
                maybe_outbound_req = outbound_requests.recv() => {
                    if maybe_outbound_req.is_none() {
                        continue;
                    }
                    let (req, req_tx) = maybe_outbound_req.unwrap();
                    let iid = match &req.invoke_id {
                        InvokeId::present(i) => i,
                        _ => continue, // We simply ignore requests from the client that do not include an InvokeId.
                    };
                    self.outstanding_requests.insert(*iid, req_tx);
                    self.write_request(req).await?;
                },
                maybe_outbound_unbind = outbound_unbinds.recv() => {
                    if maybe_outbound_unbind.is_none() {
                        continue;
                    }
                    let (unbind, unbind_tx) = maybe_outbound_unbind.unwrap();
                    self.outstanding_unbind_req = Some(unbind_tx);
                    self.write_unbind(unbind).await?;
                },
                maybe_outbound_start_tls = outbound_start_tls.recv() => {
                    if maybe_outbound_start_tls.is_none() {
                        continue;
                    }
                    let (start_tls, start_tls_tx) = maybe_outbound_start_tls.unwrap();
                    self.outstanding_starttls_req = Some(start_tls_tx);
                    self.write_start_tls(start_tls).await?;
                },
                fallible_rose_pdu_or_not = self.read_pdu() => {
                    let rose_pdu_or_not = fallible_rose_pdu_or_not?;
                    // TODO: Auto-abort on encountering giant invoke IDs or
                    if let Some(rose_pdu) = rose_pdu_or_not {
                        if !inbound_rose_pdus_tx.is_closed() { // TODO: Is it worth checking, or just ignore the error?
                            // If we drop the receiver, it just means we don't
                            // care about receiving requests (we are a client).
                            _ = inbound_rose_pdus_tx.send(rose_pdu.clone());
                        }
                        match rose_pdu {
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
                            RosePDU::Result(ref params) => {
                                let rejection = RejectParameters {
                                    invoke_id: params.invoke_id.clone(),
                                    reason: RejectReason::UnknownInvokeIdResult,
                                };
                                let invoke_id = match &params.invoke_id {
                                    InvokeId::present(i) => i,
                                    _ => {
                                        self.write_reject(rejection).await?;
                                        continue;
                                    },
                                };
                                let outstanding_req = match self.outstanding_requests.remove(&invoke_id) {
                                    Some(o) => o,
                                    _ => { // No request with this InvokeID.
                                        self.write_reject(rejection).await?;
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
                                        self.write_reject(rejection).await?;
                                        continue;
                                    },
                                };
                                let outstanding_req = match self.outstanding_requests.remove(&invoke_id) {
                                    Some(o) => o,
                                    _ => { // No request with this InvokeID.
                                        self.write_reject(rejection).await?;
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
                                let outstanding_req = match self.outstanding_requests.remove(&invoke_id) {
                                    Some(o) => o,
                                    _ => { // No request with this InvokeID.
                                        self.abort(AbortReason::Other).await?;
                                        continue;
                                    },
                                };
                                outstanding_req.send(OperationOutcome::Reject(params.clone()))
                                    .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
                            },
                            RosePDU::Unbind(_) => {
                                // TODO: The connection should be automatically closed when an unbind is received.
                                // No response is needed. Just close the connection.
                                self.reset();
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
                            RosePDU::Abort(_) => {
                                // TODO: The connection should be automatically closed when an abort is received.
                                self.reset();
                            },
                            _ => {}, // NOOP for StartTLS and StartTLSResponse. Those are handled at lower layers.
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

impl <W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> Resettable for RoseIdmStream<W> {
    fn reset (&mut self) {
        self.bound = false;
        self.outstanding_bind_req = None;
        self.outstanding_requests.clear();
    }
}

#[async_trait]
impl <W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> RoseAbort for RoseIdmStream<W> {

    async fn abort (&mut self, reason: AbortReason) -> Result<usize> {
        // TODO: .write_abort() should close the connection.
        self.reset();
        self.write_abort(reason).await
    }

}

#[async_trait]
impl<W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> ROSETransmitter<X690Element>
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
        write_idm_pdu(self, &idm_pdu).await
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
        write_idm_pdu(self, &idm_pdu).await
    }

    async fn write_bind_error(
        self: &mut Self,
        params: BindResultOrErrorParameters<X690Element>,
    ) -> Result<usize> {
        let idm_be = IdmBindError::new(
            params.protocol_id.clone(),
            params.responding_ae_title.clone(),
            None,
            params.parameter.clone(),
            vec![],
        );
        let idm_pdu = IDM_PDU::bindError(idm_be);
        write_idm_pdu(self, &idm_pdu).await
    }

    async fn write_request(
        self: &mut Self,
        params: RequestParameters<X690Element>,
    ) -> Result<usize> {
        // TODO: REVIEW: should this be required by this API?
        if let InvokeId::present(iid) = params.invoke_id.clone() {
            let request = Request::new(iid, params.code.clone(), params.parameter.clone(), vec![]);
            let idm_pdu = IDM_PDU::request(request);
            write_idm_pdu(self, &idm_pdu).await
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
            write_idm_pdu(self, &idm_pdu).await
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
            write_idm_pdu(self, &idm_pdu).await
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
            write_idm_pdu(self, &idm_pdu).await
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }

    async fn write_unbind(
        self: &mut Self,
        _params: UnbindParameters<X690Element>,
    ) -> Result<usize> {
        let idm_pdu = IDM_PDU::unbind(());
        let bytes_written = write_idm_pdu(self, &idm_pdu).await?;
        if let Some(pending) = self.outstanding_unbind_req.take() {
            // It sucks cloning this much, but these are not
            // too expensive to clone unless the user
            // supplied freakishly huge AE-titles, protocol
            // IDs, opcodes, etc.
            pending.send(UnbindOutcome::Result(None))
                .map_err(|_| Error::from(ErrorKind::BrokenPipe))?;
            Ok(bytes_written)
        } else {
            self.abort(AbortReason::InvalidProtocol).await?;
            Ok(bytes_written)
        }
    }

    // FIXME: Close the connection.
    async fn write_abort(self: &mut Self, reason: AbortReason) -> Result<usize> {
        let idm_pdu = IDM_PDU::abort(abort_reason_to_integer(reason).unwrap_or_default());
        write_idm_pdu(self, &idm_pdu).await
    }
}

#[async_trait]
impl<W: AsyncWriteExt + AsyncReadExt + Unpin + Send + Sync> ROSEReceiver<X690Element>
    for RoseIdmStream<W>
{
    async fn read_pdu(&self) -> Result<Option<rose::RosePDU<X690Element>>> {
        let mut idm_stream = self.idm.lock().await;
        let idm_frame_or_not = idm_stream.read_pdu().await?;
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
            Err(_e) => return Err(Error::from(ErrorKind::AddrNotAvailable)),
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
        Ok(Some(rose_pdu))
    }
}
