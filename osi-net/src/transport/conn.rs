use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use smallvec::{SmallVec, smallvec};
use crate::{OsiSelector, ServiceResult, RemoteAndLocalSels, NetworkConnId2};
use crate::transport::UserData;
use crate::transport::service::{
    COTSProvider,
    COTSUser,
    T_CONNECT_Request_Parameters,
    T_CONNECT_Response_Parameters,
    T_DATA_Request_Parameters,
    T_EXPEDITED_DATA_Request_Parameters,
    T_DISCONNECT_Request_Parameters,
    T_CONNECT_Confirm_Parameters,
};
use crate::network::{
    NetworkConnId,
    N_CONNECT_Confirm_Parameters,
    N_DISCONNECT_Request_Parameters,
    N_RESET_Confirm_Parameters, NSProvider,
};
use crate::network::NSUser;
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
use super::IncomingEvent;
use crate::transport::classes::class0and2::dispatch_event as dispatch_event_class_0_or_2;
use crate::stack::OSIApplicationAssociation;

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

#[derive(Debug, Clone)]
pub struct TransportedSegment {
    pub eot: bool,
    pub roa: bool,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TransportUserDataBuffer {
    // NOTE: I don't think this needs to be a VecDeque, because the EOT always
    // ends at the end of a particular TPDU. In other words, this buffer will
    // always be reset to zero-length once a complete TPDU is received.
    // pub buffer: SmallVec<[u8; 128]>, // 18 for DT-TPDU with all options, 7 for SPDU with all options, 2 for simply-encoded User-data PPDU, 2 for RTSE
    pub last_nr: u32,
    pub eot: bool,
    pub roa: bool,
    pub parts: SmallVec<[UserData; 16]>,
    /// This is for TPDUs that are received out of order.
    pub other_tpdus: HashMap<u32, TransportedSegment>, // key is the NR.
}

impl Default for TransportUserDataBuffer {
    fn default() -> Self {
        Self { last_nr: 0, eot: false, roa: false, parts: smallvec![], other_tpdus: HashMap::new() }
    }
}

#[derive(Debug, Clone)]
pub struct X224TransportConnection {
    // pub session: Rc<RefCell<S>>,
    // TODO: I think I am discovering that there is no way to not have a generic parameter for each layer...
    // There may be 0, 1, or more network connections, which may change over time.

    pub state: X224ConnectionState,
    // pub cr: Option<CR_TPDU>, // The original CR-TPDU sent or received.
    // I think just passing in the lower layer into functions that need it will be enough.
    // This actually kind of makes sense, since an N-entity can apparently use the
    // services of one or more (N-1)-entities.
    // network: NetworkLayerService,
    pub remote_ref: u16,
    pub local_ref: u16,
    pub local_t_selector: Option<OsiSelector>,
    pub remote_t_selector: Option<OsiSelector>,
    pub max_tpdu_size: usize,
    pub max_tsdu_size: usize,
    /// X.224 seems to imply that a transport connection may temporarily have
    /// a network connection unassigned.
    pub assigned_network_conn: Option<NetworkConnId2>,
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

    pub buffer: TransportUserDataBuffer,
}

impl Default for X224TransportConnection {
    fn default() -> Self {
        X224TransportConnection {
            state: X224ConnectionState::CLOSED, // TODO: Is this the correct default state?
            // cr: None,
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
            buffer: TransportUserDataBuffer::default(),
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

    // NOTE: Passing a ref to Arc might seem weird, but it means one less clone.

    pub fn receive_TPDU(&mut self, stack: &mut OSIApplicationAssociation, tpdu: &TPDU) -> ServiceResult {
        let event = IncomingEvent::TPDU(tpdu);
        dispatch_event_class_0_or_2(stack, self, &mut stack.session, &event)
    }

    pub fn receive_CR (&mut self, stack: &mut OSIApplicationAssociation, pdu: &CR_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_CC (&mut self, stack: &mut OSIApplicationAssociation, pdu: &CC_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_DR (&mut self, stack: &mut OSIApplicationAssociation, pdu: &DR_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_DC (&mut self, stack: &mut OSIApplicationAssociation, pdu: &DC_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_DT (&mut self, stack: &mut OSIApplicationAssociation, pdu: &DT_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_ED (&mut self, stack: &mut OSIApplicationAssociation, pdu: &ED_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_AK (&mut self, stack: &mut OSIApplicationAssociation, pdu: &AK_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_EA (&mut self, stack: &mut OSIApplicationAssociation, pdu: &EA_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_RJ (&mut self, stack: &mut OSIApplicationAssociation, pdu: &RJ_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }

    pub fn receive_ER (&mut self, stack: &mut OSIApplicationAssociation, pdu: &ER_TPDU) -> ServiceResult {
        // TODO:
        todo!()
    }


}

impl <S: COTSUser<Self>> COTSProvider<S> for X224TransportConnection {

    fn submit_T_CONNECT_request(&mut self, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult {
        if self.class == 0 || self.class == 2 {
            let event = IncomingEvent::TCONreq(params);
            // dispatch_event_class_0_or_2()
            // TODO:
            todo!()
        } else {
            Ok(None)
        }
    }

    fn submit_T_CONNECT_response(&mut self, s: &mut S, params: T_CONNECT_Response_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_DATA_request(&mut self, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_EXPEDITED_DATA_request(&mut self, s: &mut S, params: T_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn submit_T_DISCONNECT_request(&mut self, s: &mut S, params: T_DISCONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_CONNECT_request(&mut self, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_CONNECT_confirm(&mut self, s: &mut S, params: T_CONNECT_Confirm_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_DATA_request(&mut self, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_EXPEDITED_DATA_request(&mut self, s: &mut S, params: T_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_T_DISCONNECT_request(&mut self, s: &mut S, params: T_DISCONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

}

impl <N: NSProvider> NSUser<N> for X224TransportConnection {

    fn receive_nsdu(&mut self, n: &mut N, nsdu: Vec<u8>) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_N_CONNECT_confirm(&mut self, n: &mut N, params: N_CONNECT_Confirm_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_N_DISCONNECT_indication(&mut self, n: &mut N, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

    fn receive_N_RESET_indication(&mut self, n: &mut N, params: N_RESET_Confirm_Parameters) -> ServiceResult {
        // TODO:
        todo!()
    }

}
