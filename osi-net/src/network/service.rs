//! The network service interface will not distinguish between CONS and CLNS.
//! Instead, only a CONS-like interface will be provided, and if the underlying
//! implementation is connectionless, `N-DATA`` primitives shall be treated as
//! `N-UNIT-DATA` primitives, and all other primitives shall be no-ops that
//! emulate connection-like behavior.
use std::sync::{Arc, Mutex};

use crate::ServiceResult;
use super::*;
use crate::NetworkConnId2;

pub trait NSUser <N: NSProvider> {
    fn receive_nsdu(&mut self, n: &mut N, nsdu: Vec<u8>) -> ServiceResult;
    // In X.224 COTP, there may be any number (including zero) of network
    // connections associated with a transport connection. As such, we need to
    // pass in the originating network connection.
    fn receive_N_DISCONNECT_indication(&mut self, n: &mut N, params: N_DISCONNECT_Request_Parameters) -> ServiceResult;
    fn receive_N_CONNECT_confirm(&mut self, n: &mut N, params: N_CONNECT_Confirm_Parameters) -> ServiceResult;
    fn receive_N_RESET_indication(&mut self, n: &mut N, params: N_RESET_Confirm_Parameters) -> ServiceResult;
}

pub trait NSProvider {

    // Actions performed by the local NS-user
    fn submit_N_CONNECT_request (&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult;
    fn submit_N_CONNECT_response (&mut self, params: N_CONNECT_Response_Parameters, conn: Arc<Mutex<dyn OSINetworkConn + Send + Sync>>) -> ServiceResult;
    fn submit_N_DATA_request (&mut self, params: N_DATA_Request_Parameters) -> ServiceResult;
    fn submit_N_DATA_request_parts (&mut self, params: NSDUParts) -> ServiceResult;
    fn submit_N_DATA_ACKNOWLEDGE_request (&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult;
    fn submit_N_EXPEDITED_DATA_request (&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult;
    fn submit_N_EXPEDITED_DATA_request_parts (&mut self, params: NSDUParts) -> ServiceResult;
    fn submit_N_RESET_request (&mut self, params: N_RESET_Request_Parameters) -> ServiceResult;
    fn submit_N_RESET_response (&mut self, params: N_RESET_Response_Parameters) -> ServiceResult;
    fn submit_N_DISCONNECT_request (&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult;

    // Actions that are performed by the remote NS-user.

    /// For ITOT, this is called upon establishment of the TCP stream.
    fn receive_N_CONNECT_request(&mut self, params: N_CONNECT_Request_Parameters, conn: Arc<Mutex<dyn OSINetworkConn + Send + Sync>>) -> ServiceResult;

    /// For ITOT, this is called upon establishment of the TCP stream.
    fn receive_N_CONNECT_confirm(&mut self, params: N_CONNECT_Confirm_Parameters, conn: Arc<Mutex<dyn OSINetworkConn + Send + Sync>>) -> ServiceResult;

    /// For ITOT, this is called upon receipt of a TPKT containing a DT TPDU.
    fn receive_N_DATA_request(&mut self, params: N_DATA_Request_Parameters) -> ServiceResult;

    /// For ITOT, this is called upon receipt of a TPKT containing a AK or EA TPDU.
    fn receive_N_DATA_ACKNOWLEDGE_request(&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult;

    /// For ITOT, this is called upon receipt of a TPKT containing a ED TPDU.
    fn receive_N_EXPEDITED_DATA_request(&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult;

    /// I don't think network reset exists for ITOT.
    fn receive_N_RESET_request(&mut self, params: N_RESET_Request_Parameters) -> ServiceResult;

    /// I don't think network reset exists for ITOT.
    fn receive_N_RESET_confirm(&mut self, params: N_RESET_Confirm_Parameters) -> ServiceResult;

    /// For ITOT, this is sent upon closure of the TCP stream.
    fn receive_N_DISCONNECT_request(&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult;

}

pub trait OSINetworkConn {
    fn id (&self) -> NetworkConnId2;
    fn is_available (&self) -> bool;
    fn is_open (&self) -> bool;
    fn is_open_in_progress (&self) -> bool;
    fn transport_connections_served (&self) -> usize;
    fn max_nsdu_size (&self) -> usize;
    fn write_nsdu (&mut self, nsdu: UserData) -> Result<(), Error>;

    /// This was added to avoid unnecessary re-allocation for concatenation when
    /// the data will just be written out to a network buffer anyway, and when
    /// `.write_vectored()` may be used as a performance hack.
    fn write_nsdu_parts (&mut self, parts: NSDUParts) -> Result<(), Error>;

    fn write_exp_nsdu (&mut self, nsdu: UserData) -> Result<(), Error>;
    fn write_exp_nsdu_parts (&mut self, parts: NSDUParts) -> Result<(), Error>;

    fn request_reset (&mut self) -> Result<(), Error>;
    fn reset (&mut self) -> Result<(), Error>;
    fn request_ack (&mut self) -> Result<(), Error>;

    fn accept (&mut self, params: N_CONNECT_Response_Parameters) -> Result<(), Error>;

    // fn close (&self) -> Result<(), Error>; // This might not be necessary.
    fn local_selector (&self) -> &OsiSelector;
    fn remote_selector (&self) -> &OsiSelector;
    fn selectors (&self) -> RemoteAndLocalSelRefs {
        (self.remote_selector(), self.local_selector())
    }

    // These are useful because they can queried without locking the whole system.
    // Only the network connection in question has to be locked for these to be read.
    fn already_has_class_0_transport_conn (&self) -> bool;
    fn already_has_class_1_transport_conn (&self) -> bool;
    fn has_no_tc_assigned (&self) -> bool;
}
