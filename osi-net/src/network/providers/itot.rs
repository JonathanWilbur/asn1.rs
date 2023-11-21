use std::net::TcpStream;
use crate::network::{
    N_CONNECT_Request_Parameters,
    N_CONNECT_Response_Parameters,
};

use crate::network::NSProvider;
use nsap_address::X213NetworkAddress;

/// NOTE: You can clone TcpStream using
/// https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.try_clone
///
/// I don't exactly see how this can fail other than running out of file
/// descriptors, or maybe some other freak scenario. You can use one copy to
/// own in this struct and write to and another externally to read from and
/// enqueue NPDUs.
///
/// I think the same think can be done with Tokio using
/// `tokio::net::TcpStream::into_split()`.
///
/// You can create raw sockets currently using the `socket2` crate, which is
/// officially supported by the Rust foundation. But it is only synchronous
/// currently. This means TP4 over IP will only be synchronous.
pub struct SyncITOTStream(Option<TcpStream>);

impl NSProvider for SyncITOTStream {

    // Actions performed by the local NS-user / write half of the socket.
    fn submit_N_CONNECT_request (&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult {
        // TODO: Convert params.called_address to IP address and port
        // TODO: let stream = TcpStream::connect(addr)?;
        // self.0 = Some(stream);
        let naddr = X213NetworkAddress::try_from(params.called_address.as_slice()).unwrap();
        if let Some(socket_addr) = naddr.to_socket_addr() {

        }
    }
    fn submit_N_CONNECT_response (&mut self, params: N_CONNECT_Response_Parameters) -> ServiceResult {
        // TODO: Does anything need to be done here?
        // I think the network connection is already established by the time
        // this is called.
        Ok(None)
    }

    fn submit_N_DATA_request (&mut self, params: N_DATA_Request_Parameters) -> ServiceResult;
    fn submit_N_DATA_ACKNOWLEDGE_request (&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult;
    fn submit_N_EXPEDITED_DATA_request (&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult;
    fn submit_N_RESET_request (&mut self, params: N_RESET_Request_Parameters) -> ServiceResult;
    fn submit_N_RESET_response (&mut self, params: N_RESET_Response_Parameters) -> ServiceResult;
    fn submit_N_DISCONNECT_request (&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult;

    // Actions that are performed by the remote NS-user / read half of the socket.

    /// For ITOT, this is called upon establishment of the TCP stream.
    /// TODO: You need to pass in the originating
    fn receive_N_CONNECT_request(&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult {
        // TODO: Review. You might need a new field for network connection state, Per Figure 4 in ITU X.213.
        Ok(None)
    }

    /// For ITOT, this is called upon establishment of the TCP stream.
    fn receive_N_CONNECT_confirm(&mut self, params: N_CONNECT_Confirm_Parameters) -> ServiceResult {
        // TODO: Review.
        Ok(None)
    }

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

    fn id (&self) -> NetworkConnId;
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
    fn close (&self) -> Result<(), Error>; // This might not be necessary.
    fn local_selector (&self) -> &OsiSelector;
    fn remote_selector (&self) -> &OsiSelector;
    fn selectors (&self) -> RemoteAndLocalSelRefs {
        (self.remote_selector(), self.local_selector())
    }

    // TODO: These might need to die... It might be better to implement this in the transport provider.
    fn already_has_class_0_transport_conn (&self) -> bool;
    fn already_has_class_1_transport_conn (&self) -> bool;
    fn has_no_tc_assigned (&self) -> bool;

}
