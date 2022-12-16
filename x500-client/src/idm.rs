use std::io::{Result, ErrorKind, Error};
use asn1::ENUMERATED;
use idm::IDMSocket;
use rose_transport::{
    ROSEReceiver,
    ROSETransport,
    ROSETransmitter,
    BindParameters,
    BindResultOrErrorParameters,
    RequestParameters,
    ResultOrErrorParameters,
    RejectParameters,
    UnbindParameters,
    AbortReason, RejectReason,
};
use x500::{IDMProtocolSpecification::*, CommonProtocolSpecification::InvokeId};
// use x500::IDMProtocolSpecification::{
//     IdmReject_reason_duplicateInvokeIDRequest,
//     IdmReject_reason_invalidIdmVersion,
//     IdmReject_reason_mistypedArgumentRequest,
//     IdmReject_reason_mistypedParameterError,
//     IdmReject_reason_mistypedPDU,
//     IdmReject_reason_mistypedResultRequest,
//     IdmReject_reason_resourceLimitationRequest,
//     IdmReject_reason_unknownError,
//     IdmReject_reason_unknownInvokeIDError,
//     IdmReject_reason_unknownInvokeIDResult,
//     IdmReject_reason_unknownOperationRequest,
//     IdmReject_reason_unsuitableIdmVersion,
//     IdmReject_reason_unsupportedIdmVersion,
//     IdmReject_reason_unsupportedOperationRequest,
// };
use x690::{X690Element, write_x690_node};
use async_trait::async_trait;
// use tokio::net::TcpStream;

use crate::ROSEClient;

// import {
//     ROSETransport,
//     new_rose_transport,
//     RejectReason,
//     AbortReason,
// } from "@wildboar/rose-transport";
// import {
//     IdmReject_reason,
//     IdmReject_reason_mistypedPDU,
//     IdmReject_reason_duplicateInvokeIDRequest,
//     IdmReject_reason_unsupportedOperationRequest,
//     IdmReject_reason_unknownOperationRequest,
//     IdmReject_reason_mistypedArgumentRequest,
//     IdmReject_reason_resourceLimitationRequest,
//     IdmReject_reason_unknownInvokeIDResult,
//     IdmReject_reason_mistypedResultRequest,
//     IdmReject_reason_unknownInvokeIDError,
//     IdmReject_reason_unknownError,
//     IdmReject_reason_mistypedParameterError,
//     IdmReject_reason_unsupportedIdmVersion,
//     IdmReject_reason_unsuitableIdmVersion,
//     IdmReject_reason_invalidIdmVersion,
// } from "@wildboar/x500/src/lib/modules/IDMProtocolSpecification/IdmReject-reason.ta";
// import {
//     Abort,
//     Abort_mistypedPDU,
//     Abort_unboundRequest,
//     Abort_invalidPDU,
//     Abort_resourceLimitation,
//     Abort_connectionFailed,
//     Abort_invalidProtocol,
//     Abort_reasonNotSpecified,
// } from "@wildboar/x500/src/lib/modules/IDMProtocolSpecification/Abort.ta";
// import {
//     _encode_Code,
// } from "@wildboar/x500/src/lib/modules/CommonProtocolSpecification/Code.ta";
// import { BER } from "asn1-ts/dist/node/functional";
// import { protocol_id_to_rose_protocol, app_context_to_protocol_id } from "./utils";

// const idm_reject_to_rose_reject: Map<IdmReject_reason, RejectReason> = new Map([
//     [ IdmReject_reason_mistypedPDU, RejectReason.mistyped_pdu ],
//     [ IdmReject_reason_duplicateInvokeIDRequest, RejectReason.duplicate_invoke_id_request ],
//     [ IdmReject_reason_unsupportedOperationRequest, RejectReason.unsupported_operation_request ],
//     [ IdmReject_reason_unknownOperationRequest, RejectReason.unknown_operation_request ],
//     [ IdmReject_reason_mistypedArgumentRequest, RejectReason.mistyped_argument_request ],
//     [ IdmReject_reason_resourceLimitationRequest, RejectReason.resource_limitation_request ],
//     [ IdmReject_reason_unknownInvokeIDResult, RejectReason.unknown_invoke_id_result ],
//     [ IdmReject_reason_mistypedResultRequest, RejectReason.mistyped_result_request ],
//     [ IdmReject_reason_unknownInvokeIDError, RejectReason.unknown_invoke_id_error ],
//     [ IdmReject_reason_unknownError, RejectReason.unknown_error ],
//     [ IdmReject_reason_mistypedParameterError, RejectReason.mistyped_parameter_error ],
//     [ IdmReject_reason_unsupportedIdmVersion, RejectReason.unsupported_idm_version ],
//     [ IdmReject_reason_unsuitableIdmVersion, RejectReason.unsuitable_idm_version ],
//     [ IdmReject_reason_invalidIdmVersion, RejectReason.invalid_idm_version ],
// ]);

// const rose_reject_to_idm_reject: Map<RejectReason, IdmReject_reason> = new Map([
//     [ RejectReason.mistyped_pdu, IdmReject_reason_mistypedPDU ],
//     [ RejectReason.duplicate_invoke_id_request, IdmReject_reason_duplicateInvokeIDRequest ],
//     [ RejectReason.unsupported_operation_request, IdmReject_reason_unsupportedOperationRequest ],
//     [ RejectReason.unknown_operation_request, IdmReject_reason_unknownOperationRequest ],
//     [ RejectReason.mistyped_argument_request, IdmReject_reason_mistypedArgumentRequest ],
//     [ RejectReason.resource_limitation_request, IdmReject_reason_resourceLimitationRequest ],
//     [ RejectReason.unknown_invoke_id_result, IdmReject_reason_unknownInvokeIDResult ],
//     [ RejectReason.mistyped_result_request, IdmReject_reason_mistypedResultRequest ],
//     [ RejectReason.unknown_invoke_id_error, IdmReject_reason_unknownInvokeIDError ],
//     [ RejectReason.unknown_error, IdmReject_reason_unknownError ],
//     [ RejectReason.mistyped_parameter_error, IdmReject_reason_mistypedParameterError ],
//     [ RejectReason.unsupported_idm_version, IdmReject_reason_unsupportedIdmVersion ],
//     [ RejectReason.unsuitable_idm_version, IdmReject_reason_unsuitableIdmVersion ],
//     [ RejectReason.invalid_idm_version, IdmReject_reason_invalidIdmVersion ],
// ]);

// const idm_abort_to_rose_abort: Map<Abort, AbortReason> = new Map([
//     [ Abort_mistypedPDU, AbortReason.mistyped_pdu ],
//     [ Abort_unboundRequest, AbortReason.unbound_request ],
//     [ Abort_invalidPDU, AbortReason.invalid_pdu ],
//     [ Abort_resourceLimitation, AbortReason.resource_limitation ],
//     [ Abort_connectionFailed, AbortReason.connection_failed ],
//     [ Abort_invalidProtocol, AbortReason.invalid_protocol ],
//     [ Abort_reasonNotSpecified, AbortReason.reason_not_specified ],
// ]);

// const rose_abort_to_idm_abort: Map<Abort, AbortReason> = new Map([
//     [ AbortReason.mistyped_pdu, Abort_mistypedPDU ],
//     [ AbortReason.unbound_request, Abort_unboundRequest ],
//     [ AbortReason.invalid_pdu, Abort_invalidPDU ],
//     [ AbortReason.resource_limitation, Abort_resourceLimitation ],
//     [ AbortReason.connection_failed, Abort_connectionFailed ],
//     [ AbortReason.invalid_protocol, Abort_invalidProtocol ],
//     [ AbortReason.reason_not_specified, Abort_reasonNotSpecified ],
// ]);

// pub fn rose_transport_from_idm_socket (idm: &mut IDMSocket) -> ROSETransport {

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
async fn write_idm_pdu (idm: &mut IDMSocket<Vec<u8>>, pdu: &IDM_PDU) -> Result<usize> {
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

#[async_trait]
impl ROSETransmitter<X690Element> for ROSEClient<IDMSocket<Vec<u8>>> {

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

//     rose.write_bind = (params) => {
//         rose.protocol = protocol_id_to_rose_protocol(params.protocol_id) ?? params.protocol_id;
//         idm.writeBind(
//             app_context_to_protocol_id.get(params.protocol_id.toString()) ?? params.protocol_id,
//             params.parameter,
//             params.calling_ae_title,
//             params.called_ae_title,
//         );
//     };

//     rose.write_bind_result = (params) => {
//         rose.is_bound = true;
//         rose.protocol = protocol_id_to_rose_protocol(params.protocol_id) ?? params.protocol_id;
//         idm.writeBindResult(
//             app_context_to_protocol_id.get(params.protocol_id.toString()) ?? params.protocol_id,
//             params.parameter,
//             params.responding_ae_title,
//         );
//     };

//     rose.write_bind_error = (params) => {
//         rose.is_bound = false;
//         idm.writeBindError(
//             app_context_to_protocol_id.get(params.protocol_id.toString()) ?? params.protocol_id,
//             params.parameter,
//             params.responding_ae_title,
//         );
//     };

//     rose.write_request = (params) => {
//         if (!("present" in params.invoke_id)) {
//             return;
//         }
//         idm.writeRequest(
//             params.invoke_id.present,
//             params.code,
//             params.parameter,
//         );
//     };

//     rose.write_result = (params) => {
//         if (!("present" in params.invoke_id)) {
//             return;
//         }
//         idm.writeResult(
//             params.invoke_id.present,
//             params.code,
//             params.parameter,
//         );
//     };

//     rose.write_error = (params) => {
//         if (!("present" in params.invoke_id)) {
//             return;
//         }
//         idm.writeError(
//             params.invoke_id.present,
//             _encode_Code(params.code, BER),
//             params.parameter,
//         );
//     };

//     rose.write_reject = (params) => {
//         if (!("present" in params.invoke_id)) {
//             return;
//         }
//         idm.writeReject(
//             params.invoke_id.present,
//             rose_reject_to_idm_reject.get(params.problem)
//                 ?? IdmReject_reason_unknownError,
//         );
//     };

//     rose.write_unbind = () => {
//         rose.is_bound = false;
//         idm.writeUnbind();
//     };

//     rose.write_unbind_result = () => {
//         rose.is_bound = false;
//     };

//     rose.write_unbind_error = () => {};

//     rose.write_abort = (params) => {
//         rose.is_bound = false;
//         idm.writeAbort(
//             rose_abort_to_idm_abort.get(params) ?? Abort_reasonNotSpecified,
//         );
//     };

//     rose.write_start_tls = () => {
//         idm.writeStartTLS();
//     };

//     rose.write_tls_response = (params) => {
//         idm.writeTLSResponse(params?.code ?? 0);
//     };

//     return rose;
// }
