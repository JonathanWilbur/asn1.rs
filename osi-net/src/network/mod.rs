mod providers;
mod service;
pub use service::*;
use crate::{OsiSelector, OSIConnectionOrientedEntity, OSIConnectionOrientedLayer, OSIConnection, RemoteAndLocalSelRefs, transport::NSDUParts};
use std::io::{Error, ErrorKind};

/// Why an integer ID for a network connection?
/// Why not use the remote and local NSAPs tuple?
/// Because Section 6.9.1.4.2.b.1 of
/// [ITU-T Recommendation X.224 (1995)](https://www.itu.int/rec/T-REC-X.224/en)
/// implies that there may be multiple connections having the same NSAP tuple.
///
/// The network layer MUST ensure that these are unique.
pub type NetworkConnId = u32;

/// This might be changed to alias bytes::Bytes instead.
pub type UserData = Vec<u8>;
pub type Reason = i32;

/// See [ITU-T Recommendation X.213 (2001)](https://www.itu.int/rec/T-REC-X.213/en), Figure 4.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum OSINetworkConnectionState {
    Idle,
    OutgoingConnectionPending,
    IncomingConnectionPending,
    DataTransferReady,
    NSUserInvokedResetPending,
    NSProviderInvokedResetPending,
}

impl TryFrom<OSINetworkConnectionState> for ErrorKind {
    type Error = ();

    fn try_from(value: OSINetworkConnectionState) -> Result<Self, Self::Error> {
        match value {
            OSINetworkConnectionState::Idle => Ok(ErrorKind::NotConnected),
            OSINetworkConnectionState::OutgoingConnectionPending => Ok(ErrorKind::NotConnected),
            OSINetworkConnectionState::IncomingConnectionPending => Ok(ErrorKind::NotConnected),
            OSINetworkConnectionState::DataTransferReady => Err(()),
            OSINetworkConnectionState::NSUserInvokedResetPending => Ok(ErrorKind::ConnectionReset),
            OSINetworkConnectionState::NSProviderInvokedResetPending => Ok(ErrorKind::ConnectionReset),
        }
    }

}

pub trait OSINetworkService {
    fn consume_outgoing_event (&mut self) -> Option<OSINetworkServiceOutgoingEvent>;

    // Actions performed by the local NS-user
    fn submit_CONNECT_request (&mut self, params: N_CONNECT_Request_Parameters) -> Result<(), Error>;
    fn submit_CONNECT_response (&mut self, params: N_CONNECT_Response_Parameters) -> Result<(), Error>;
    fn submit_DATA_request (&mut self, params: N_DATA_Request_Parameters) -> Result<(), Error>;
    fn submit_DATA_ACKNOWLEDGE_request (&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> Result<(), Error>;
    fn submit_EXPEDITED_DATA_request (&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> Result<(), Error>;
    fn submit_RESET_request (&mut self, params: N_RESET_Request_Parameters) -> Result<(), Error>;
    fn submit_RESET_response (&mut self, params: N_RESET_Response_Parameters) -> Result<(), Error>;
    fn submit_DISCONNECT_request (&mut self, params: N_DISCONNECT_Request_Parameters) -> Result<(), Error>;

    // Actions that are performed by the remote NS-user.

    /// For ITOT, this is called upon establishment of the TCP stream.
    fn receive_CONNECT_request(&mut self, params: N_CONNECT_Request_Parameters) -> Result<(), Error>;

    /// For ITOT, this is called upon establishment of the TCP stream.
    fn receive_CONNECT_confirm(&mut self, params: N_CONNECT_Confirm_Parameters) -> Result<(), Error>;

    /// For ITOT, this is called upon receipt of a TPKT containing a DT TPDU.
    fn receive_DATA_request(&mut self, params: N_DATA_Request_Parameters) -> Result<(), Error>;

    /// For ITOT, this is called upon receipt of a TPKT containing a AK or EA TPDU.
    fn receive_DATA_ACKNOWLEDGE_request(&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> Result<(), Error>;

    /// For ITOT, this is called upon receipt of a TPKT containing a ED TPDU.
    fn receive_EXPEDITED_DATA_request(&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> Result<(), Error>;

    /// I don't think network reset exists for ITOT.
    fn receive_RESET_request(&mut self, params: N_RESET_Request_Parameters) -> Result<(), Error>;

    /// I don't think network reset exists for ITOT.
    fn receive_RESET_confirm(&mut self, params: N_RESET_Confirm_Parameters) -> Result<(), Error>;

    /// For ITOT, this is sent upon closure of the TCP stream.
    fn receive_DISCONNECT_request(&mut self, params: N_DISCONNECT_Request_Parameters) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
/// Events emitted by the local network service.
pub enum OSINetworkServiceOutgoingEvent {
    CONNECT_indication(N_CONNECT_Indication_Parameters),
    CONNECT_confirm(N_CONNECT_Confirm_Parameters),
    DATA_indication(N_DATA_Indication_Parameters),
    DATA_ACKNOWLEDGE_indication(N_DATA_ACKNOWLEDGE_Indication_Parameters),
    EXPEDITED_DATA_indication(N_EXPEDITED_DATA_Indication_Parameters),
    RESET_indication(N_RESET_Indication_Parameters),
    RESET_confirm(N_RESET_Confirm_Parameters),
    DISCONNECT_indication(N_DISCONNECT_Indication_Parameters),
}



#[derive(Debug, Clone)]
pub struct N_CONNECT_Parameters {
    // pub receipt_confirmation_selection ?
    // pub expedited_data_selection ?
    // pub qos_parameter_set ?
    pub ns_user_data: UserData,
}

impl From<UserData> for N_CONNECT_Parameters {

    fn from(value: UserData) -> Self {
        N_CONNECT_Parameters { ns_user_data: value }
    }

}

#[derive(Debug, Clone)]
pub struct N_CONNECT_Request_Parameters {
    pub called_address: OsiSelector,
    pub calling_address: OsiSelector,
    pub connect_params: N_CONNECT_Parameters,
}

pub type N_CONNECT_Indication_Parameters = N_CONNECT_Request_Parameters;

#[derive(Debug, Clone)]
pub struct N_CONNECT_Response_Parameters {
    pub responding_address: OsiSelector,
    pub connect_params: N_CONNECT_Parameters,
}

impl From<N_CONNECT_Request_Parameters> for N_CONNECT_Response_Parameters {

    fn from(value: N_CONNECT_Request_Parameters) -> Self {
        N_CONNECT_Response_Parameters {
            connect_params: value.connect_params,
            responding_address: value.called_address,
        }
    }

}

pub type N_CONNECT_Confirm_Parameters = N_CONNECT_Response_Parameters;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct N_DATA_Request_Parameters {
    pub ns_user_data: UserData,
}

impl From<UserData> for N_DATA_Request_Parameters {

    fn from(value: UserData) -> Self {
        N_DATA_Request_Parameters { ns_user_data: value }
    }

}

pub type N_DATA_Indication_Parameters = N_DATA_Request_Parameters;

#[derive(Debug, Clone)]
pub struct N_DATA_ACKNOWLEDGE_Request_Parameters;

#[derive(Debug, Clone)]
pub struct N_DATA_ACKNOWLEDGE_Indication_Parameters;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct N_EXPEDITED_DATA_Request_Parameters {
    pub ns_user_data: UserData,
}

impl From<UserData> for N_EXPEDITED_DATA_Request_Parameters {

    fn from(value: UserData) -> Self {
        N_EXPEDITED_DATA_Request_Parameters { ns_user_data: value }
    }

}

pub type N_EXPEDITED_DATA_Indication_Parameters = N_EXPEDITED_DATA_Request_Parameters;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct N_RESET_Request_Parameters {
    pub reason: Reason,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct N_RESET_Indication_Parameters {
    pub originator: OsiSelector,
    pub reason: Reason,
}

#[derive(Debug, Clone)]
pub struct N_RESET_Response_Parameters;

#[derive(Debug, Clone)]
pub struct N_RESET_Confirm_Parameters;

#[derive(Debug, Clone)]
pub struct N_DISCONNECT_Request_Parameters {
    pub reason: Reason,
    pub ns_user_data: UserData,
    pub responding_address: OsiSelector,
}

impl Default for N_DISCONNECT_Request_Parameters {
    fn default() -> Self {
        N_DISCONNECT_Request_Parameters {
            reason: 0,
            ns_user_data: vec![],
            responding_address: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct N_DISCONNECT_Indication_Parameters {
    pub originator: OsiSelector,
    pub reason: Reason,
    pub ns_user_data: UserData,
    pub responding_address: OsiSelector,
}

// Implementation

// #[derive(Debug, Clone)]
// pub struct OSINetworkConnection {
//     pub state: OSINetworkConnectionState,
//     pub enqueued_outgoing_events: VecDeque<OSINetworkServiceOutgoingEvent>,
//     pub selector: OsiSelector,
// }


// impl OSINetworkService for OSINetworkServiceEntity {

//     fn consume_outgoing_event (&mut self) -> Option<OSINetworkServiceOutgoingEvent> {
//         self.enqueued_outgoing_events.pop_front()
//     }

//     fn submit_CONNECT_request (&mut self, params: N_CONNECT_Request_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::Idle {
//             return Err(Error::from(ErrorKind::AlreadyExists));
//         }
//         // TODO: Validate request params.
//         self.state = OSINetworkEntityState::IncomingConnectionPending;
//         let outgoing_event = OSINetworkServiceOutgoingEvent::CONNECT_indication(params.into());
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     fn submit_CONNECT_response (&mut self, params: N_CONNECT_Response_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::IncomingConnectionPending {
//             return Err(Error::from(ErrorKind::ConnectionAborted));
//         }
//         // TODO: Insert responding NSAP address.
//         self.state = OSINetworkEntityState::DataTransferReady;
//         let outgoing_event = OSINetworkServiceOutgoingEvent::CONNECT_confirm(params.into());
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     fn submit_DATA_request (&mut self, params: N_DATA_Request_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::DataTransferReady {
//             let err_kind = self.state.try_into().unwrap_or(ErrorKind::NotConnected);
//             return Err(Error::from(err_kind));
//         }
//         let outgoing_event = OSINetworkServiceOutgoingEvent::DATA_indication(params.into());
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     fn submit_DATA_ACKNOWLEDGE_request (&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::DataTransferReady {
//             let err_kind = self.state.try_into().unwrap_or(ErrorKind::NotConnected);
//             return Err(Error::from(err_kind));
//         }
//         let outgoing_event = OSINetworkServiceOutgoingEvent::DATA_ACKNOWLEDGE_indication(N_DATA_ACKNOWLEDGE_Indication_Parameters);
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     fn submit_EXPEDITED_DATA_request (&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::DataTransferReady {
//             let err_kind = self.state.try_into().unwrap_or(ErrorKind::NotConnected);
//             return Err(Error::from(err_kind));
//         }
//         let outgoing_event = OSINetworkServiceOutgoingEvent::EXPEDITED_DATA_indication(params.into());
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     // TODO: It is not clear whether this is for a
//     fn submit_RESET_request (&mut self, params: N_RESET_Request_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::DataTransferReady {
//             let err_kind = self.state.try_into().unwrap_or(ErrorKind::NotConnected);
//             return Err(Error::from(err_kind));
//         }
//         let ind = N_RESET_Indication_Parameters {
//             reason: params.reason,
//             originator: self.selector.clone(),
//         };
//         // self.state = OSINetworkEntityState::
//         let outgoing_event = OSINetworkServiceOutgoingEvent::RESET_indication(ind);
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     fn submit_RESET_response (&mut self, _: N_RESET_Response_Parameters) -> Result<(), Error> {
//         if self.state != OSINetworkEntityState::NSUserInvokedResetPending { // TODO: Not sure this is right.
//             let err_kind = self.state.try_into().unwrap_or(ErrorKind::NotConnected);
//             return Err(Error::from(err_kind));
//         }
//         self.state = OSINetworkEntityState::DataTransferReady;
//         let cnf = N_RESET_Confirm_Parameters;
//         let outgoing_event = OSINetworkServiceOutgoingEvent::RESET_confirm(cnf);
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     fn submit_DISCONNECT_request (&mut self, params: N_DISCONNECT_Request_Parameters) -> Result<(), Error> {
//         self.state = OSINetworkEntityState::Idle;
//         let ind = N_DISCONNECT_Indication_Parameters {
//             originator: self.selector.clone(),
//             ns_user_data: params.ns_user_data,
//             reason: params.reason,
//             responding_address: params.responding_address.clone(),
//         };
//         let outgoing_event = OSINetworkServiceOutgoingEvent::DISCONNECT_indication(ind);
//         self.enqueued_outgoing_events.push_back(outgoing_event);
//         Ok(())
//     }

//     /// For ITOT, this is called upon establishment of the TCP stream.
//     fn receive_CONNECT_request(&mut self, params: N_CONNECT_Request_Parameters) -> Result<(), Error> {

//     }

//     /// For ITOT, this is called upon establishment of the TCP stream.
//     fn receive_CONNECT_confirm(&mut self, params: N_CONNECT_Confirm_Parameters) -> Result<(), Error> {

//     }

//     /// For ITOT, this is called upon receipt of a TPKT containing a DT TPDU.
//     fn receive_DATA_request(&mut self, params: N_DATA_Request_Parameters) -> Result<(), Error> {

//     }

//     /// For ITOT, this is called upon receipt of a TPKT containing a AK or EA TPDU.
//     fn receive_DATA_ACKNOWLEDGE_request(&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> Result<(), Error> {

//     }

//     /// For ITOT, this is called upon receipt of a TPKT containing a ED TPDU.
//     fn receive_EXPEDITED_DATA_request(&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> Result<(), Error> {

//     }

//     /// I don't think network reset exists for ITOT.
//     fn receive_RESET_request(&mut self, params: N_RESET_Request_Parameters) -> Result<(), Error> {

//     }

//     /// I don't think network reset exists for ITOT.
//     fn receive_RESET_confirm(&mut self, params: N_RESET_Confirm_Parameters) -> Result<(), Error> {

//     }

//     /// For ITOT, this is sent upon closure of the TCP stream.
//     fn receive_DISCONNECT_request(&mut self, params: N_DISCONNECT_Request_Parameters) -> Result<(), Error> {

//     }

// }


/*

Does the "entity" construct provide any useful service at all? It seems like it
is just another useless layer of indirection. Currently, I am thinking every
subsystem should just have a single entity, and the subsystem should just
implement the (N)-service trait that simply forwards the request to the single
entity. In the future, you can swap out subsystems with those that provide for
multiple entities, if needed.

I just realized a problem with the do-then-read-queue approach: the state could
be set before a corresponding PDU is sent.

So it seems like every function call will need to be something like:

(&mut self, &mut t, &mut s, &mut p, params)

Example:

fn dispatch_ASCreq (&mut p) {
    p.dispatch_PCONreq()?;
}

CONSTransportSubsystem : CONSTransportService + Subsystem
CONSTransportEntity : CONSTransportService + Entity
ITOTStack : PresentationService
LPPStack : PresentationService

How would this work with XOT or TP4?
- I think this means that you will need to abstract away the TCP socket for ITOT
  so that all function calls can take a (&mut n) parameter.

This is all stack-specific. LPP will just have a presentation layer.


so that each layer has access to the layers above it and can immediately enqueue
PDUs to be sent.

Usage:
1. receive bytes over TCP Stream
2. decode the TPDU.
3. Use the REF to get the transport entity from the transport layer.
4. apply the TPDU to the transport entity.
5. The transport entity will look up the transport connection associated with
   that REF and forward the TPDU to it.
4. read outgoing events from the connection TPM and act on them until the queue is empty.

*/

pub trait OSIConnectionOrientedNetworkService {
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
    fn id (&self) -> NetworkConnId;
}

/*

type SpecificAction = fn(&mut self, n: &mut N, s: &mut S, tpdu: TPDU) -> Result<(), Error>;
type Predicate = fn(&mut self, n: &mut N, s: &mut S, tpdu: TPDU) -> bool;

const SPECIFIC_ACTION: [SpecificAction; 5] = [
    ndis_if_no_longer_used,
    receive_er_tpdu,
    data_transfer_procedure,
    exp_data_transfer_procedure,
    maybe_ndis,
    dc_tpdu, // IDG what this is saying, actually.
];

const P: [Predicate; 5] = [
    t_conn_req_unacceptable,
    ...
];

Then the table translates almost 1:1 to code:

fn dispatch_incoming_event(event) {
    match (event, state) {
        (TCONReq(params), CLOSED) => {
            if P[0](n, self, s, tpdu) {
                self.s.apply_TDISind();
                self.state = CLOSED;
            }
            if P[1](n, self, s, tpdu) {
                NCONreq
                self.state = WFNC;
            }
        }
    }
}

*/

pub trait OSIConnectionOrientedNetworkLayer: OSIConnectionOrientedNetworkService + OSIConnectionOrientedLayer {

}

pub trait OSIConnectionOrientedNetworkEntity: OSIConnectionOrientedNetworkService + OSIConnectionOrientedEntity {

}

// pub trait OSINetworkConnection: OSIConnectionOrientedNetworkService + OSIConnection {
//     fn already_has_class_0_transport_conn (&self) -> bool;
//     fn already_has_class_1_transport_conn (&self) -> bool;
//     fn has_no_tc_assigned (&self) -> bool;
// }
