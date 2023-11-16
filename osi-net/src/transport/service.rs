use crate::network::{
    N_CONNECT_Confirm_Parameters,
    N_DISCONNECT_Indication_Parameters,
    N_RESET_Indication_Parameters,
    NSProvider,
};
use crate::session::OSIConnectionOrientedSessionService;
use crate::ServiceResult;
use crate::transport::UserData;
use crate::transport::pdu::TPDU;
use std::borrow::Cow;

/// The session entity implements this.
pub trait COTSUser <T: COTSProvider<Self>> where Self: Sized {
    fn receive_nsdu(&mut self, t: &mut T, nsdu: Cow<'_, [u8]>) -> ServiceResult;
    fn receive_T_CONNECT_indication(&mut self, t: &mut T, params: T_CONNECT_Request_Parameters) -> ServiceResult;
    fn receive_T_CONNECT_confirmation(&mut self, t: &mut T, params: T_CONNECT_Request_Parameters) -> ServiceResult;
    fn receive_T_DISCONNECT_indication(&mut self, t: &mut T, params: T_CONNECT_Request_Parameters) -> ServiceResult;
}

/// The transport entity implements this.
pub trait COTSProvider <S: COTSUser<Self>> where Self: Sized {
    fn submit_T_CONNECT_request(&mut self, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult;
    fn submit_T_CONNECT_response(&mut self, s: &mut S, params: T_CONNECT_Response_Parameters) -> ServiceResult;
    fn submit_T_DATA_request(&mut self, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult;
    fn submit_T_EXPEDITED_DATA_request(&mut self, s: &mut S, params: T_EXPEDITED_DATA_Request_Parameters) -> ServiceResult;
    fn submit_T_DISCONNECT_request(&mut self, s: &mut S, params: T_DISCONNECT_Request_Parameters) -> ServiceResult;
    fn receive_T_CONNECT_request(&mut self, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult;
    fn receive_T_CONNECT_confirm(&mut self, s: &mut S, params: T_CONNECT_Confirm_Parameters) -> ServiceResult;
    fn receive_T_DATA_request(&mut self, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult;
    fn receive_T_EXPEDITED_DATA_request(&mut self, s: &mut S, params: T_EXPEDITED_DATA_Request_Parameters) -> ServiceResult;
    fn receive_T_DISCONNECT_request(&mut self, s: &mut S, params: T_DISCONNECT_Request_Parameters) -> ServiceResult;
}

pub trait OSIConnectionOrientedTransportService <N, S> : COTSProvider<Self> + COTSUser<Self>
    where N: NSProvider,
        S: OSIConnectionOrientedSessionService { // FIXME: This needs to be a layer.

    fn receive_nsdu(&mut self, n: &mut N, s: &mut S, nsdu: Vec<u8>) -> ServiceResult;

    /// When the runtime sees that the soonest timer has been triggered, this
    /// function shall be called. This function shall perform every action that
    /// is due, then reset / nullify the timers that caused them, if necessary.
    /// This returns the time of the next timer to be invoked.
    fn check_timers(&mut self, n: &mut N, s: &mut S) -> ServiceResult;

    // This might be unnecessary. Every outgoing event can be translated to
    // an invocation of an N-1 or N+1 API.
    // fn get_next_outgoing_event (&mut self) -> Option<OSIConnectionOrientedTransportOutgoingEvent>;


}

// From Table A.3 of ITU Recommendation X.224.
// pub enum OSIConnectionOrientedTransportOutgoingEvent {
//     TCONind(CR_TPDU),
//     TCONconf(CC_TPDU),
//     TDTind(),
//     TEXind(),
//     TDISind(),
//     NDISreq(),
//     NRSTresp(),
//     NCONreq(),
//     CR(CR_TPDU),
//     CC(CC_TPDU),
//     DR(DR_TPDU),
//     DC(DC_TPDU),
//     AK(),
//     EA(),
//     DT(DT_TPDU),
//     ED(),
//     ER(ER_TPDU),
//     RJ(),
//     TSDU(UserData),
//     // Not specified in Table A.3, but still useful.
// }

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct Throughput {
    pub target: u32,
    pub min_acceptable: u32,
}

/// All fields express the number of octets per second.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct BidirectionalThroughput {
    pub calling_to_called: Throughput,
    pub called_to_calling: Throughput,
}

/// See X.224, Section 13.3.4.j
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct MaxAndAverageThroughput {
    pub max: BidirectionalThroughput,
    pub average: Option<BidirectionalThroughput>,
}

/// Values are expressed in milliseconds.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct TransitDelay {
    pub target: u16,
    pub max_acceptable: u16,
}

/// Values are expressed in milliseconds.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct BidirectionalTransitDelay {
    pub calling_to_called: TransitDelay,
    pub called_to_calling: TransitDelay,
}

/// All values in this data structure are NOT represented as a power of ten or
/// two. They are the calculated product. This is also not well documented
/// anywhere. These are not the abstract parameters, but concrete fields in the
/// X.224 TPDUs, since the abstract parameters are not well-defined.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct ResidualErrorRate {
    pub target: usize,
    pub minimum_acceptable: usize,
    pub tsdu_size_of_interest: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct SelectiveAcknowledgement {
    pub lower_edge: u32,
    pub upper_edge: u32,
}

/// See Section 10 of
/// [ITU-T Recommendation X.224 (1995)](https://www.itu.int/rec/T-REC-X.224/en).
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportQualityOfServiceParameters {
    // pub establishment_delay: usize, ?
    pub throughput: Option<MaxAndAverageThroughput>,
    pub transit_delay: Option<BidirectionalTransitDelay>,
    pub residual_error_rate: Option<ResidualErrorRate>,

    /// This does not seem to be defined or used in X.224.
    /// The units of this will be failures per 100_000_000 TPDUs.
    pub transfer_failure_probability: Option<usize>,

    /// Measured in milliseconds.
    pub tc_release_delay: Option<u16>,

    /// This does not seem to be defined or used in X.224.
    /// The units of this will be failures per 100_000_000 TPDUs.
    pub tc_release_failure_probability: Option<usize>,

    /// The value of this field is user-defined.
    pub protection_parameters: Option<UserData>,

    /// This is is capped at u16::MAX in the X.224 TPDU, but it is theoretically
    /// unbounded.
    pub priority: Option<u16>, // 0 is the highest priority.

    /// This does not seem to be defined or used in X.224.
    /// The units of this will be failures per 100_000_000 TPDUs.
    pub tc_resilience: Option<usize>,
}

impl Default for TransportQualityOfServiceParameters {

    fn default() -> Self {
        TransportQualityOfServiceParameters {
            throughput: None,
            transit_delay: None,
            residual_error_rate: None,
            transfer_failure_probability: None,
            tc_release_delay: None,
            tc_release_failure_probability: None,
            protection_parameters: None,
            priority: None,
            tc_resilience: None,
        }
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct T_CONNECT_Request_Parameters {
    // pub called_address: ?
    // pub calling_address: ?
    pub expedited_data: bool,
    pub qos: Option<TransportQualityOfServiceParameters>,
    pub user_data: UserData,
}
pub type T_CONNECT_Indication_Parameters = T_CONNECT_Request_Parameters;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct T_CONNECT_Response_Parameters {
    // pub responding_address: ?
    pub expedited_data: bool,
    pub qos: Option<TransportQualityOfServiceParameters>,
    pub user_data: UserData,
}
pub type T_CONNECT_Confirm_Parameters = T_CONNECT_Response_Parameters;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct T_DATA_Request_Parameters {
    pub user_data: UserData,
}
pub type T_DATA_Indication_Parameters = T_DATA_Request_Parameters;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct T_EXPEDITED_DATA_Request_Parameters {
    pub user_data: UserData,
}
pub type T_EXPEDITED_DATA_Indication_Parameters = T_EXPEDITED_DATA_Request_Parameters;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct T_DISCONNECT_Request_Parameters {
    pub reason: u8,
    pub user_data: UserData,
}
pub type T_DISCONNECT_Indication_Parameters = T_DISCONNECT_Request_Parameters;

pub enum IncomingEvent <'a> {
    TPDU(TPDU<'a>),
    TCONreq(T_CONNECT_Request_Parameters),
    TCONresp(T_CONNECT_Response_Parameters),
    TDTreq(T_DATA_Request_Parameters),
    TEXreq(T_EXPEDITED_DATA_Request_Parameters),
    TDISreq(T_DISCONNECT_Request_Parameters),
    NDISind(N_DISCONNECT_Indication_Parameters),
    NCONconf(N_CONNECT_Confirm_Parameters),
    NRSTind(N_RESET_Indication_Parameters),
}
