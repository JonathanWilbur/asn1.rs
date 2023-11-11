use std::time::{Duration, SystemTime};

use crate::{OsiSelector, ServiceResult, RemoteAndLocalSels, RemoteAndLocalSelRefs};
use crate::transport::UserData;
use crate::transport::service::{
    OSIConnectionOrientedTransportService,
    T_CONNECT_Request_Parameters,
    T_CONNECT_Response_Parameters,
    T_DATA_Request_Parameters,
    T_DATA_EXPEDITED_DATA_Request_Parameters,
    T_DATA_DISCONNECT_Request_Parameters,
    T_CONNECT_Confirm_Parameters,
};
use crate::network::{
    OSINetworkConnection,
    NetworkConnId,
    N_CONNECT_Confirm_Parameters,
    N_DISCONNECT_Request_Parameters,
    N_RESET_Confirm_Parameters,
};
use crate::session::OSIConnectionOrientedSessionService;
use crate::transport::pdu::{
    TPDU,
    CR_TPDU,
    CC_TPDU,
    DR_TPDU,
    DC_TPDU,
    DT_TPDU,
    ED_TPDU,
    AK_TPDU,
    EA_TPDU,
    RJ_TPDU,
    ER_TPDU,
};

/// From Table A.2 of ITU Recommendation X.224.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum X224ConnectionState {

    /// Wait for network connection
    WFNC,

    /// Wait for the CC-TPDU
    WFCC,

    /// Wait before releasing (wait for CC-TPDU before sending the TPDU-DR)
    WBCL,

    /// Transport connection is open
    OPEN,

    /// Release in progress
    CLOSING,

    /// Wait for T-CONNECT response
    WFTRESP,

    /// Transport connection is closed
    CLOSED,

    /// Wait for network connection and reassignment in progress
    WFNC_R,

    /// Wait for CC-TPDU and reassignment in progress
    WFCC_R,

    /// Wait before releasing and reassignment in progress
    WBCL_R,

    /// Open and reassignment in progress
    OPEN_R,

    /// Open and wait for reassignment
    OPEN_WR,

    /// Release in progress and reassignment in progress
    CLOSING_R,

    /// Release in progress and wait for reassignment
    CLOSING_WR,

    /// Wait for T-CONNECT response and wait for reassignment
    WFTRESP_WR,

    /// Wait before releasing and wait for reassignment
    WBCL_WR,

    /// Wait before open complete (CC is unacknowledged)
    WBOC,

    /// Wait before open complete and wait for reassignment
    WBOC_WR,

    /// Wait before open complete and release in progress
    CLOSING_BOC,

    /// Idem and wait for reassignment
    CLOSING_BOC_WR,

    /// Waiting for acknowledgement of CC-TPDU
    AKWAIT,

    /// Waiting for frozen reference time
    REFWAIT,
}

impl X224ConnectionState {

    pub fn is_open (&self) -> bool {
        match self {
            X224ConnectionState::OPEN
            | X224ConnectionState::OPEN_R
            | X224ConnectionState::OPEN_WR => true,
            _ => false,
        }
    }

}

pub struct X224TransportConnection {
    pub state: X224ConnectionState,
    // I think just passing in the lower layer into functions that need it will be enough.
    // This actually kind of makes sense, since an N-entity can apparently use the
    // services of one or more (N-1)-entities.
    // network: NetworkLayerService,
    pub buffer: Vec<UserData>,
    pub remote_ref: u16,
    pub local_ref: u16,
    pub local_t_selector: Option<OsiSelector>,
    pub remote_t_selector: Option<OsiSelector>,
    pub max_tpdu_size: usize,
    pub max_tsdu_size: usize,
    /// X.224 seems to imply that a transport connection may temporarily have
    /// a network connection unassigned.
    pub assigned_network_conn: Option<NetworkConnId>,
    pub class: u8,
    pub cr_nsaps: Option<RemoteAndLocalSels>,

    // These two come from the options of the CR-TPDU, and are agreed upon in the CC-TPDU.
    pub use_extended_format: bool,
    pub use_explicit_flow_control_in_class_2: bool,

    // These come from the Additional Option Selection parameter.
    pub use_non_blocking_expedited_data_in_class_4: bool,
    pub use_request_acknowledgement_in_classes_1_3_and_4: bool,
    pub use_selective_acknowledgement_in_class_4: bool,
    pub use_network_expedited_in_class_1: bool,
    pub use_receipt_confirmation_in_class_1: bool,
    pub use_checksum_in_class_4: bool,
    pub use_transport_expedited_data_transfer_service: bool,

    // Configuration options
    pub ts2_duration: Duration,

    // Timers
    pub ts1: Option<SystemTime>,

    /// Set after an ER-TPDU is sent in response to an invalid TPDU. After this
    /// lapses, the connection shall be released normally. If a DR-TPDU is sent,
    /// this shall be unset.
    pub ts2: Option<SystemTime>,
}

impl Default for X224TransportConnection {
    fn default() -> Self {
        X224TransportConnection {
            state: X224ConnectionState::CLOSED, // TODO: Is this the correct default state?
            buffer: vec![],
            remote_ref: 0,
            local_ref: 0,
            local_t_selector: None,
            remote_t_selector: None,
            max_tpdu_size: usize::MAX, // TODO: What is the default max?
            max_tsdu_size: usize::MAX, // FIXME:
            assigned_network_conn: None,
            class: 0,
            cr_nsaps: None,
            use_extended_format: false,
            use_explicit_flow_control_in_class_2: false,
            use_non_blocking_expedited_data_in_class_4: false,
            use_request_acknowledgement_in_classes_1_3_and_4: false,
            use_selective_acknowledgement_in_class_4: false,
            use_network_expedited_in_class_1: false,
            use_receipt_confirmation_in_class_1: false,
            use_checksum_in_class_4: false,
            use_transport_expedited_data_transfer_service: true,
            ts2_duration: Duration::from_secs(5),
            ts1: None,
            ts2: None,
        }
    }
}

// The functions below were added because the abstract service parameters do not
// capture all of the data in a received TPDU.
impl X224TransportConnection {

    pub fn is_open (&self) -> bool {
        self.state.is_open()
    }

    pub fn is_checksummed (&self) -> bool {
        self.class == 4 && self.use_checksum_in_class_4
    }

    pub fn receive_TPDU<N, S>(&mut self, n: &mut N, s: &mut S, tpdu: &TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_CR<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &CR_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_CC<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &CC_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_DR<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &DR_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_DC<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &DC_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_DT<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &DT_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_ED<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &ED_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_AK<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &AK_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_EA<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &EA_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_RJ<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &RJ_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }

    pub fn receive_ER<N, S>(&mut self, n: &mut N, s: &mut S, pdu: &ER_TPDU) -> ServiceResult
        where N: OSINetworkConnection, S: OSIConnectionOrientedSessionService {
        // TODO:
        todo!()
    }


}

impl <N, S> OSIConnectionOrientedTransportService <N, S> for X224TransportConnection
    where N: OSINetworkConnection,
    S: OSIConnectionOrientedSessionService {

    fn receive_nsdu(&mut self, n: &mut N, s: &mut S, nsdu: Vec<u8>) -> ServiceResult {
        // TODO:
        todo!()
    }

    /// When the runtime sees that the soonest timer has been triggered, this
    /// function shall be called. This function shall perform every action that
    /// is due, then reset / nullify the timers that caused them, if necessary.
    /// This returns the time of the next timer to be invoked.
    fn check_timers(&mut self, n: &mut N, s: &mut S) -> ServiceResult {
        // TODO:
        todo!()
    }

    // This might be unnecessary. Every outgoing event can be translated to
    // an invocation of an N-1 or N+1 API.
    // fn get_next_outgoing_event (&mut self) -> Option<OSIConnectionOrientedTransportOutgoingEvent>;

    fn submit_T_CONNECT_request(&mut self, n: &mut N, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_CONNECT_response(&mut self, n: &mut N, s: &mut S, params: T_CONNECT_Response_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_DATA_request(&mut self, n: &mut N, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_EXPEDITED_DATA_request(&mut self, n: &mut N, s: &mut S, params: T_DATA_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_DISCONNECT_request(&mut self, n: &mut N, s: &mut S, params: T_DATA_DISCONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_CONNECT_request(&mut self, n: &mut N, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_CONNECT_confirm(&mut self, n: &mut N, s: &mut S, params: T_CONNECT_Confirm_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_DATA_request(&mut self, n: &mut N, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_EXPEDITED_DATA_request(&mut self, n: &mut N, s: &mut S, params: T_DATA_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_DISCONNECT_request(&mut self, n: &mut N, s: &mut S, params: T_DATA_DISCONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_DISCONNECT_indication(&mut self, n: &mut N, s: &mut S, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_N_CONNECT_confirm(&mut self, n: &mut N, s: &mut S, params: N_CONNECT_Confirm_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_RESET_indication(&mut self, n: &mut N, s: &mut S, params: N_RESET_Confirm_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

}
