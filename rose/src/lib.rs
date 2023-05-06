use asn1::{ASN1Value, INTEGER, OBJECT_IDENTIFIER};
use async_trait::async_trait;
use std::io::{Error, ErrorKind, Result};
use x500::CertificateExtensions::GeneralName;
use x500::CommonProtocolSpecification::{Code, InvokeId};
use tokio::sync::{oneshot, mpsc};

pub type OtherRoseError = u16;

pub type BindOutcomeSender<ParameterType> = oneshot::Sender<BindOutcome<ParameterType, ParameterType>>;
pub type OperationOutcomeSender<ParameterType> = oneshot::Sender<OperationOutcome<ParameterType, ParameterType>>;
pub type UnbindOutcomeSender<ParameterType> = oneshot::Sender<UnbindOutcome<ParameterType, ParameterType>>;
pub type StartTLSOutcomeSender = oneshot::Sender<StartTLSOutcome>;

pub type BindArgAndTx<ParameterType> = mpsc::UnboundedReceiver<(BindParameters<ParameterType>, BindOutcomeSender<ParameterType>)>;
pub type RequestArgAndTx<ParameterType> = mpsc::UnboundedReceiver<(RequestParameters<ParameterType>, OperationOutcomeSender<ParameterType>)>;
pub type UnbindArgAndTx<ParameterType> = mpsc::UnboundedReceiver<(UnbindParameters<ParameterType>, UnbindOutcomeSender<ParameterType>)>;
pub type StartTlsArgAndTx = mpsc::UnboundedReceiver<(StartTLSParameters, oneshot::Sender<StartTLSOutcome>)>;

pub type BindRequestSender<ParameterType> = mpsc::UnboundedSender<(BindParameters<ParameterType>, BindOutcomeSender<ParameterType>)>;
pub type RequestSender<ParameterType> = mpsc::UnboundedSender<(RequestParameters<ParameterType>, OperationOutcomeSender<ParameterType>)>;
pub type UnbindSender<ParameterType> = mpsc::UnboundedSender<(UnbindParameters<ParameterType>, UnbindOutcomeSender<ParameterType>)>;
pub type StartTlsSender = mpsc::UnboundedSender<(StartTLSParameters, oneshot::Sender<StartTLSOutcome>)>;

pub enum LoopMode {
    #[allow(dead_code)]
    SingleOp,
    Continuous,
}

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
    Result(Option<ResultType>),
    Error(Option<ErrorType>),
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

pub type InvokeIdInt = i64;

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
pub trait ROSEReceiver<ParameterType: Send> {
    async fn read_pdu(self: &Self) -> Result<Option<RosePDU<ParameterType>>>;
}

#[async_trait]
pub trait AsyncRoseClient<ParameterType = ASN1Value>
where
    ParameterType: Send
{
    async fn bind(self: &mut Self, params: BindParameters<ParameterType>) -> Result<BindOutcome<ParameterType, ParameterType>>
    where
        ParameterType: 'async_trait;

    async fn request(self: &mut Self, params: RequestParameters<ParameterType>) -> Result<OperationOutcome<ParameterType, ParameterType>>
    where
        ParameterType: 'async_trait;

    async fn unbind(self: &mut Self, params: UnbindParameters<ParameterType>) -> Result<UnbindOutcome<ParameterType, ParameterType>>
    where
        ParameterType: 'async_trait;

    async fn start_tls(self: &mut Self, params: StartTLSParameters) -> Result<StartTLSOutcome>
    where
        ParameterType: 'async_trait;
}

pub trait Resettable {
    fn reset (&mut self);
}

#[async_trait]
pub trait RoseAbort {
    async fn abort(self: &mut Self, reason: AbortReason) -> Result<usize>;
}

#[async_trait]
pub trait RoseEngine<ParameterType = ASN1Value>
    where
        Self: Sized,
        ParameterType: Sync + Send {

    async fn turn(
        &mut self,
        mode: LoopMode,
        mut outbound_binds: BindArgAndTx<ParameterType>,
        mut outbound_requests: RequestArgAndTx<ParameterType>,
        mut outbound_unbinds: UnbindArgAndTx<ParameterType>,
        mut outbound_start_tls: StartTlsArgAndTx,
    ) -> Result<()>
        where ParameterType: 'async_trait;

    async fn drive(
        &mut self,
        outbound_binds: BindArgAndTx<ParameterType>,
        outbound_requests: RequestArgAndTx<ParameterType>,
        outbound_unbinds: UnbindArgAndTx<ParameterType>,
        outbound_start_tls: StartTlsArgAndTx,
    ) -> Result<()>
    where ParameterType: 'async_trait {
        self.turn(
            LoopMode::Continuous,
            outbound_binds,
            outbound_requests,
            outbound_unbinds,
            outbound_start_tls,
        ).await
    }
}
