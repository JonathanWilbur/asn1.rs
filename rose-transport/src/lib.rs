use asn1::{ASN1Value, INTEGER, OBJECT_IDENTIFIER};
use async_trait::async_trait;
use std::io::{Error, ErrorKind, Result};
use x500::CertificateExtensions::GeneralName;
use x500::CommonProtocolSpecification::{Code, InvokeId};

pub type OtherRoseError = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AbortReason {
    // From IDM
    MistypedPDU,
    UnboundRequest,
    InvalidPDU,
    ResourceLimitation,
    ConnectionFailed,
    InvalidProtocol,
    ReasonNotSpecified,
    // From ACSE
    ProtocolError,
    AuthenticationMechanismNameNotRecognized,
    AuthenticationMechanismNameRequired,
    AuthenticationFailure,
    AuthenticationRequired,
    // From Me
    Other,
}

impl Default for AbortReason {
    fn default() -> Self {
        AbortReason::ReasonNotSpecified
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RejectReason {
    MistypedPDU,
    DuplicateInvokeIdRequest,
    UnsupportedOperationRequest,
    UnknownOperationRequest,
    MistypedArgumentRequest,
    ResourceLimitationRequest,
    UnknownInvokeIdResult,
    MistypedResultRequest,
    UnknownInvokeIdError,
    UnknownError,
    MistypedParameterError,
    UnsupportedIDMVersion,
    UnsuitableIDMVersion,
    InvalidIDMVersion,
    UnrecognizedPDU,
    BadlyStructuredPDU,
    ReleaseInProgress,
    Other,
}

impl Default for RejectReason {
    fn default() -> Self {
        RejectReason::MistypedPDU
    }
}

#[derive(Debug, Clone)]
pub struct BindParameters<ParameterType = ASN1Value> {
    pub timeout: u32, // 0 = no timeout.
    pub protocol_id: OBJECT_IDENTIFIER,
    pub parameter: ParameterType,
    pub calling_ae_title: Option<GeneralName>,
    pub calling_ap_invocation_identifier: Option<INTEGER>,
    pub calling_ae_invocation_identifier: Option<INTEGER>,
    pub called_ae_title: Option<GeneralName>,
    pub called_ap_invocation_identifier: Option<INTEGER>,
    pub called_ae_invocation_identifier: Option<INTEGER>,
    pub implementation_information: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BindResultOrErrorParameters<ParameterType = ASN1Value> {
    pub protocol_id: OBJECT_IDENTIFIER,
    pub parameter: ParameterType,
    pub responding_ae_title: Option<GeneralName>,
    pub responding_ap_invocation_identifier: Option<INTEGER>,
    pub responding_ae_invocation_identifier: Option<INTEGER>,
}

#[derive(Debug, Clone)]
pub struct RequestParameters<ParameterType = ASN1Value> {
    pub invoke_id: InvokeId,
    pub code: Code,
    pub parameter: ParameterType,
    pub linked_id: Option<InvokeId>,
}

#[derive(Debug, Clone)]
pub struct ResultOrErrorParameters<ParameterType = ASN1Value> {
    pub invoke_id: InvokeId,
    pub code: Code,
    pub parameter: ParameterType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RejectParameters {
    pub invoke_id: InvokeId,
    pub reason: RejectReason,
}

#[derive(Debug, Clone)]
pub struct UnbindParameters<ParameterType = ASN1Value> {
    pub timeout: u32, // 0 = no timeout.
    pub parameter: Option<ParameterType>,
}

#[derive(Debug, Clone)]
pub struct UnbindResultOrErrorParameters<ParameterType = ASN1Value>
where
    ParameterType: Send,
{
    pub parameter: Option<ParameterType>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StartTLSParameters {
    pub timeout: u32, // 0 = no timeout.
}

#[derive(Debug, Clone, Copy)]
pub struct TLSResponseParameters {
    pub code: u8,
}

#[derive(Debug, Clone)]
pub enum RosePDU<ParameterType = ASN1Value>
where
    ParameterType: Send,
{
    Bind(BindParameters<ParameterType>),
    BindResult(BindResultOrErrorParameters<ParameterType>),
    BindError(BindResultOrErrorParameters<ParameterType>),
    Request(RequestParameters<ParameterType>),
    Result(ResultOrErrorParameters<ParameterType>),
    Error(ResultOrErrorParameters<ParameterType>),
    Reject(RejectParameters),
    Unbind(UnbindParameters<ParameterType>),
    UnbindResult(UnbindResultOrErrorParameters<ParameterType>),
    UnbindError(UnbindResultOrErrorParameters<ParameterType>),
    Abort(AbortReason),
    StartTLS(StartTLSParameters),
    StartTLSResponse(TLSResponseParameters),
}

// export
// class ROSETransportEventEmitter extends TypedEmitter<ROSETransportEvents> {

// }

// export
// interface ROSEInvocationEvents {
//     [invoke_id: string]: (outcome: OperationOutcome) -> Result<()>,
// }

// export
// class ROSEInvocationEventEmitter extends TypedEmitter<ROSEInvocationEvents> {

// }

#[derive(Debug, Clone)]
pub enum BindOutcome<BindResultType = ASN1Value, BindErrorType = ASN1Value> {
    Result(BindResultOrErrorParameters<BindResultType>),
    Error(BindResultOrErrorParameters<BindErrorType>),
    Abort(AbortReason),
    Timeout,
    Other(OtherRoseError),
}

#[derive(Debug, Clone)]
pub enum OperationOutcome<ResultType = ASN1Value, ErrorType = ASN1Value> {
    Result(ResultOrErrorParameters<ResultType>),
    Error(ResultOrErrorParameters<ErrorType>),
    Reject(RejectParameters),
    Abort(AbortReason),
    Timeout,
    Other(OtherRoseError),
}

#[derive(Debug, Clone)]
pub enum UnbindOutcome<ResultType = ASN1Value, ErrorType = ASN1Value> {
    Result(ResultType),
    Error(ErrorType),
    Abort(AbortReason),
    Timeout,
    Other(OtherRoseError),
}

#[derive(Debug, Clone)]
pub enum StartTLSOutcome {
    Response(TLSResponseParameters),
    NotSupportedLocally,
    AlreadyInUse,
    Abort(AbortReason),
    Timeout,
    Other(OtherRoseError),
}

#[derive(Debug, Clone)]
pub struct ROSETransportOptions {
    pub operation_timeout_ms: u32,
}

#[derive(Debug, Clone)]
pub struct ROSETransport {
    pub protocol: Option<OBJECT_IDENTIFIER>,
    // events
    pub is_bound: bool,
    pub options: ROSETransportOptions,
}

#[async_trait]
pub trait ROSETransmitter<ParameterType = ASN1Value>
where
    ParameterType: Send,
{
    async fn write_bind(self: &mut Self, params: BindParameters<ParameterType>) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_bind_result(
        self: &mut Self,
        params: BindResultOrErrorParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_bind_error(
        self: &mut Self,
        params: BindResultOrErrorParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_request(
        self: &mut Self,
        params: RequestParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_result(
        self: &mut Self,
        params: ResultOrErrorParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_error(
        self: &mut Self,
        params: ResultOrErrorParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_reject(self: &mut Self, params: RejectParameters) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_unbind(
        self: &mut Self,
        params: UnbindParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait;
    async fn write_abort(self: &mut Self, reason: AbortReason) -> Result<usize>
    where
        ParameterType: 'async_trait;

    // Default implementations, since these are not guaranteed to be defined for
    // all ROSE transports.
    async fn write_unbind_result(
        self: &mut Self,
        _params: UnbindResultOrErrorParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait,
    {
        Ok(0)
    }
    async fn write_unbind_error(
        self: &mut Self,
        _params: UnbindResultOrErrorParameters<ParameterType>,
    ) -> Result<usize>
    where
        ParameterType: 'async_trait,
    {
        Ok(0)
    }
    async fn write_start_tls(self: &mut Self, _params: StartTLSParameters) -> Result<usize> {
        Err(Error::from(ErrorKind::Unsupported))
    }
    async fn write_tls_response(self: &mut Self, _params: TLSResponseParameters) -> Result<usize> {
        Err(Error::from(ErrorKind::Unsupported))
    }

    async fn write_rose_pdu(self: &mut Self, pdu: RosePDU<ParameterType>) -> Result<usize>
    where
        ParameterType: 'async_trait,
    {
        match pdu {
            RosePDU::Bind(params) => Self::write_bind(self, params).await,
            RosePDU::BindResult(params) => Self::write_bind_result(self, params).await,
            RosePDU::BindError(params) => Self::write_bind_error(self, params).await,
            RosePDU::Request(params) => Self::write_request(self, params).await,
            RosePDU::Result(params) => Self::write_result(self, params).await,
            RosePDU::Error(params) => Self::write_error(self, params).await,
            RosePDU::Reject(params) => Self::write_reject(self, params).await,
            RosePDU::Unbind(params) => Self::write_unbind(self, params).await,
            RosePDU::UnbindResult(params) => Self::write_unbind_result(self, params).await,
            RosePDU::UnbindError(params) => Self::write_unbind_error(self, params).await,
            RosePDU::Abort(params) => Self::write_abort(self, params).await,
            RosePDU::StartTLS(params) => Self::write_start_tls(self, params).await,
            RosePDU::StartTLSResponse(params) => Self::write_tls_response(self, params).await,
        }
    }
}

#[async_trait]
pub trait ROSEReceiver<T: Send> {
    async fn read_pdu(self: &mut Self) -> Result<Option<RosePDU<T>>>;
}

// trait AsyncROSEClient <BindArgumentType = ASN1Value, BindResultType = ASN1Value> {
//     fn bind (params: BindParameters<BindArgumentType>) -> BindOutcome<BindResultType>;
// }

// export
// interface AsyncROSEClient <BindArgumentType = ASN1Element, BindResultType = ASN1Element> {
//     // Async/Await Client API
//     bind: (params: BindParameters<BindArgumentType>) => Promise<BindOutcome<BindResultType>>;
//     request: (params: RequestParameters) => Promise<OperationOutcome>;
//     unbind: (param?: UnbindParameters) => Promise<UnbindOutcome>;
//     startTLS?: (params?: StartTLSParameters) => Promise<StartTLSOutcome>;
// }

// export
// interface ROSETransport extends AsyncROSEClient {
//     socket: Socket | TLSSocket | null;
//     protocol?: OBJECT_IDENTIFIER;
//     events: ROSETransportEventEmitter;
//     invocation_events: ROSEInvocationEventEmitter;
//     is_bound: boolean;
//     options?: ROSETransportOptions;

//     // Low-Level API
//     write_bind: (params: BindParameters) -> Result<()>,
//     write_bind_result: (params: BindResultParameters) -> Result<()>,
//     write_bind_error: (params: BindErrorParameters) -> Result<()>,
//     write_request: (params: RequestParameters) -> Result<()>,
//     write_result: (params: ResultParameters) -> Result<()>,
//     write_error: (params: ErrorParameters) -> Result<()>,
//     write_reject: (params: RejectParameters) -> Result<()>,
//     write_unbind: (params?: UnbindParameters) -> Result<()>,
//     write_unbind_result: (param?: ASN1Element) -> Result<()>,
//     write_unbind_error: (param?: ASN1Element) -> Result<()>,
//     write_abort: (reason: AbortReason) -> Result<()>,

//     write_start_tls?: (params?: StartTLSParameters) -> Result<()>,
//     write_tls_response?: (params?: TLSResponseParameters) -> Result<()>,
// }

// export
// function new_rose_transport (socket?: Socket | TLSSocket): ROSETransport {
//     const rose: ROSETransport = {
//         socket: socket ?? null,
//         events: new ROSETransportEventEmitter(),
//         invocation_events: new ROSEInvocationEventEmitter(),
//         is_bound: false,
//         write_bind: () => {},
//         write_bind_result: () => {},
//         write_bind_error: () => {},
//         write_request: () => {},
//         write_result: () => {},
//         write_error: () => {},
//         write_reject: () => {},
//         write_unbind: () => {},
//         write_unbind_result: () => {},
//         write_unbind_error: () => {},
//         write_abort: () => {},
//         bind: (params) => new Promise((resolve) => {
//             const done = (): void => {
//                 rose.events.off("bind_result", result_handler);
//                 rose.events.off("bind_error", error_handler);
//                 rose.events.off("abort", abort_handler);
//                 rose.socket?.off("timeout", timeout_function);
//                 rose.socket?.off("end", end_function);
//                 if (timeout_handle) {
//                     clearTimeout(timeout_handle);
//                     timeout_handle = undefined;
//                 }
//             };
//             const result_handler = (result: BindResultParameters) => (done(), resolve({ result }));
//             const error_handler = (error: BindErrorParameters) => (done(), resolve({ error }));
//             const abort_handler = (reason: AbortReason) => (done(), resolve({ abort: reason }));
//             let timeout_handle: NodeJS.Timeout | undefined;
//             rose.events.once("bind_result", result_handler);
//             rose.events.once("bind_error", error_handler);
//             rose.events.once("abort", abort_handler);
//             const timeout_in_ms: number | undefined = params.timeout ?? rose.options?.operation_timeout_ms;
//             const timeout_function = () => (done(), resolve({ timeout: true }));
//             const end_function = () => (done(), resolve({ other: { message: "Socket closed." } }));
//             if (Number.isSafeInteger(timeout_in_ms) && (timeout_in_ms! > 0)) {
//                 timeout_handle = setTimeout(
//                     timeout_function,
//                     timeout_in_ms,
//                 );
//             }
//             rose.socket?.once("timeout", timeout_function);
//             rose.socket?.once("end", end_function);
//             rose.write_bind(params);
//         }),
//         request: async (params) => new Promise((resolve) => {
//             const done = (): void => {
//                 rose.invocation_events.off(params.invoke_id.toString(), outcome_handler);
//                 rose.events.off("abort", abort_handler);
//                 rose.socket?.off("timeout", timeout_function);
//                 rose.socket?.off("end", end_function);
//                 if (timeout_handle) {
//                     clearTimeout(timeout_handle);
//                     timeout_handle = undefined;
//                 }
//             };
//             const outcome_handler = (outcome: OperationOutcome) => (done(), resolve(outcome));
//             const abort_handler = (reason: AbortReason) => (done(), resolve({ abort: reason }));
//             let timeout_handle: NodeJS.Timeout | undefined;
//             rose.invocation_events.once(params.invoke_id.toString(), outcome_handler);
//             rose.events.once("abort", abort_handler);
//             const timeout_in_ms: number | undefined = params.timeout ?? rose.options?.operation_timeout_ms;
//             const timeout_function = () => (done(), resolve({ timeout: true }));
//             const end_function = () => (done(), resolve({ other: { message: "Socket closed." } }));
//             if (Number.isSafeInteger(timeout_in_ms) && (timeout_in_ms! > 0)) {
//                 timeout_handle = setTimeout(
//                     timeout_function,
//                     timeout_in_ms,
//                 );
//             }
//             rose.socket?.once("timeout", timeout_function);
//             rose.socket?.once("end", end_function);
//             rose.write_request(params);
//         }),
//         unbind: async (params) => new Promise((resolve) => {
//             const done = (): void => {
//                 rose.events.off("unbind_result", result_handler);
//                 rose.events.off("unbind_error", error_handler);
//                 rose.events.off("abort", abort_handler);
//                 rose.socket?.off("timeout", timeout_function);
//                 rose.socket?.off("end", end_function);
//                 if (timeout_handle) {
//                     clearTimeout(timeout_handle);
//                     timeout_handle = undefined;
//                 }
//             };
//             const result_handler = (result?: ASN1Element) => (done(), resolve({ result: result ?? null }));
//             const error_handler = (error?: ASN1Element) => (done(), resolve({ error: error ?? null }));
//             const abort_handler = (reason: AbortReason) => (done(), resolve({ abort: reason }));
//             let timeout_handle: NodeJS.Timeout | undefined;
//             rose.events.once("unbind_result", result_handler);
//             rose.events.once("unbind_error", error_handler);
//             rose.events.once("abort", abort_handler);
//             const timeout_in_ms: number | undefined = params?.timeout ?? rose.options?.operation_timeout_ms;
//             const timeout_function = () => (done(), resolve({ timeout: true }));
//             const end_function = () => (done(), resolve({ other: { message: "Socket closed." } }));
//             if (Number.isSafeInteger(timeout_in_ms) && (timeout_in_ms! > 0)) {
//                 timeout_handle = setTimeout(
//                     timeout_function,
//                     timeout_in_ms,
//                 );
//             }
//             rose.socket?.once("timeout", timeout_function);
//             rose.socket?.once("end", end_function);
//             rose.write_unbind(params);
//         }),
//         startTLS: async (params): Promise<StartTLSOutcome> => new Promise((resolve) => {
//             if (!rose.write_start_tls) {
//                 resolve({ not_supported_locally: null });
//                 return;
//             }
//             if (rose.socket instanceof TLSSocket) {
//                 resolve({ already_in_use: null });
//                 return;
//             }
//             const done = (): void => {
//                 rose.events.off("start_tls_response", response_handler);
//                 rose.events.off("abort", abort_handler);
//                 rose.socket?.off("timeout", timeout_function);
//                 rose.socket?.off("end", end_function);
//                 if (timeout_handle) {
//                     clearTimeout(timeout_handle);
//                     timeout_handle = undefined;
//                 }
//             };
//             let timeout_handle: NodeJS.Timeout | undefined;
//             const response_handler = (params: TLSResponseParameters) => (done(), resolve({ response: params.code }));
//             const abort_handler = (reason: AbortReason) => (done(), resolve({ abort: reason }));
//             const timeout_in_ms: number | undefined = params?.timeout ?? rose.options?.operation_timeout_ms;
//             const timeout_function = () => (done(), resolve({ timeout: true }));
//             const end_function = () => (done(), resolve({ other: { message: "Socket closed." } }));
//             if (Number.isSafeInteger(timeout_in_ms) && (timeout_in_ms! > 0)) {
//                 timeout_handle = setTimeout(
//                     timeout_function,
//                     timeout_in_ms,
//                 );
//             }
//             rose.events.once("start_tls_response", response_handler);
//             rose.events.once("abort", abort_handler);
//             rose.socket?.once("timeout", timeout_function);
//             rose.socket?.once("end", end_function);
//             rose.write_start_tls();
//         }),
//     };
//     rose.events.on("result", (result) => rose
//         .invocation_events.emit(result.invoke_id.toString(), { result }));
//     rose.events.on("error_", (error) => rose
//         .invocation_events.emit(error.invoke_id.toString(), { error }));
//     rose.events.on("reject", (reject) => rose
//         .invocation_events.emit(reject.invoke_id.toString(), { reject }));
//     return rose;
// }
