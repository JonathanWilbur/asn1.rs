use rand_sequence::RandomSequence;

use crate::{ServiceResult, WakeTime, NetworkConnId2, OsiSelector};
use crate::network::{NSProvider, NetworkConnId};
use crate::presentation::X226PresentationConnection;
use crate::session::X225SessionConnection;
use crate::transport::{X224TransportConnection, TransportRef, parse_x224_tpdu, TPDU, CLASS_OPTION_4, CR_TPDU, DR_TPDU, DR_REASON_NOT_SPECIFIED, DC_TPDU, TPDUType, X224ConnectionState};
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::Weak as ArcWeak;
use std::io::{Error, ErrorKind};
use rand::rngs::OsRng;

pub struct TransportConnInfo {
    /// The application assocation.
    pub stack: Weak<RefCell<OSIApplicationAssociation>>,

    /// The network connections used by this transport connection.
    pub netconns: Vec<Rc<RefCell<dyn NSProvider>>>,
}

pub struct OSITransportLayerState {
    pub local_ref_to_conn: HashMap<TransportRef, std::sync::Mutex<OSIApplicationAssociation>>,
    pub next_ref: TransportRef,
    pub remote_ref_to_local_ref: HashMap<(NetworkConnId2, TransportRef), TransportRef>,
    // This is used to track the 1:1 relation between a network conn. and a class 0 or 1 transport conn.
    pub n_conn_id_to_local_refs: HashMap<NetworkConnId2, TransportRef>,
    pub ref_iter: RandomSequence<u16>,
}

pub const SUPPORTS_TP_CLASS_0: bool = true;
pub const SUPPORTS_TP_CLASS_1: bool = true;
pub const SUPPORTS_TP_CLASS_2: bool = true;
pub const SUPPORTS_TP_CLASS_3: bool = true;
pub const SUPPORTS_TP_CLASS_4: bool = true;

#[derive(Clone)]
pub struct OSISharedState(pub Arc<RwLock<OSITransportLayerState>>);

impl OSISharedState {

    fn receive_nsdu<N: NSProvider + Send + Sync + 'static> (&mut self, n: Arc<Mutex<N>>, nsdu: &[u8]) -> ServiceResult {
        let mut b = nsdu;
        let mut some_succeeded: bool = false;
        let mut last_err: Option<Error> = None;
        let mut wake_time: WakeTime = None;
        let n2 = n.lock().unwrap();
        let netconn_id = n2.id();
        let local_selector = n2.local_selector().to_owned();
        let remote_selector = n2.remote_selector().to_owned();
        drop(n2);
        // TODO: Check that each TPDU except the last one belongs to a separate connection.
        while b.len() > 0 {
            match parse_x224_tpdu(b) {
                Ok((s, tpdu)) => {
                    match self.dispatch_tpdu(n.clone(), netconn_id, &local_selector, &remote_selector, tpdu) {
                        Ok(timer) => {
                            some_succeeded = true;
                            if wake_time.is_none() || (timer.is_some() && (timer.unwrap() < wake_time.unwrap())) {
                                wake_time = timer;
                            }
                        },
                        Err(e) => last_err = Some(e),
                    }
                },
                Err(e) => {
                    // TODO: Peek at the LI and fast-forward over the troublesome TPDU.
                },
            };
        }
        if !some_succeeded {
            return Err(last_err.unwrap_or(Error::from(ErrorKind::InvalidInput)));
        }
        Ok(wake_time)
    }

    // NOTE: I _think_ there can be only one session conn per transport at a time.
    // ChatGPT disagrees, but can't provide evidence and contradicts itself.
    // I misunderstood how sessions work. Sessions can be created and terminated
    // readily, and instead, activities survive across session connections.
    // X.200: "The Transport Layer uniquely identifies each session-entity by its transport-address."
    // X.200: "Transport-connections provide duplex transmission between a pair of session-entites (through transport-SAPs)."
    // BINGO: X.200: "There is a one-to-one mapping between a session-connection and a transport-connection at any given instant."
    /// Implements X.224, Section 6.9.1.4.2
    fn dispatch_tpdu <N: NSProvider + Send + Sync + 'static> (
        &mut self,
        mut n: Arc<Mutex<N>>,
        netconn_id: NetworkConnId2,
        local_selector: &OsiSelector,
        remote_selector: &OsiSelector,
        tpdu: TPDU,
    ) -> ServiceResult {
        if let TPDU::CR(cr) = tpdu {
            if cr.src_ref == 0 {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
            let remote_ref = (netconn_id, cr.src_ref);
            // I think this paragraph is written poorly, but I think what it
            // is saying is that "if the SRC-REF + remote NSAP combo is
            // already taken, this TPDU obviously belongs to that connection."
            let system = self.0.read().unwrap();
            let maybe_conn = system
                .remote_ref_to_local_ref.get(&remote_ref)
                .and_then(|local_ref| system.local_ref_to_conn.get(local_ref));
            if let Some(conn) = maybe_conn {
                let mut conn = conn.lock().unwrap();
                /* "If the received TPDU is a CR-TPDU, and if the SRC-REF
                parameter and the remote NSAP indicate an existing transport
                connection at that receiving entity, then the CR-TPDU is
                associated with that transport connection" */
                if conn.transport.assigned_network_conn.is_some_and(|nc| nc == netconn_id) {
                    return conn.transport.receive_CR(&mut conn, &cr);
                }
            }
            drop(system);
            if cr.class_option == CLASS_OPTION_4 { // Check if class is 4 before acquiring the lock.
                let n2 = n.as_ref().lock().unwrap();
                let already_has_class_0_or_1_transport_conn = n2.already_has_class_0_transport_conn()
                    || n2.already_has_class_0_transport_conn();
                drop(n2);
                if already_has_class_0_or_1_transport_conn {
                    // Discard the CR TPDU. It seems to imply "do nothing."
                    return Ok(None);
                }
            }
            let mut conn = self.create_new_conn(n.clone(), netconn_id, local_selector, remote_selector, &cr);
            let result = conn.transport.receive_CR(&mut conn, &cr);
            if result.is_ok() {
                let mut system = self.0.write().unwrap();
                system.remote_ref_to_local_ref.insert(remote_ref, conn.transport.local_ref);
                system.local_ref_to_conn.insert(conn.transport.local_ref, Mutex::new(conn));
            }
            // TODO: Remote user could be trying to exhaust local refs. Somehow block / punish them.
            return result;
        }
        if let TPDU::DT(dt) = &tpdu {
            // TODO: Optimize this code by not locking so much. Locking
            // happening on every received DT TPDU is a big deal.
            let n2 = n.as_ref().lock().unwrap();
            // If the received TPDU is a DT-TPDU and the network connection has no TC assigned to it, and the DT-TPDU is a
            // class 0 or class 1 TPDU (as recognized by the absence of a DST-REF field), then the TPDU should be ignored.
            if dt.dst_ref.is_none() && n2.has_no_tc_assigned() {
                return Ok(None);
            }

            // If the received TPDU is a DT-TPDU and the network connection has a class 0 or 1 transport connection
            // assigned to it, or an AK-TPDU where a class 1 transport connection is assigned, then the TPDU is
            // associated with the transport connection.
            // NOTE: Multiplexing is only allowed in classes 2, 3, and 4, meaning that, if class 0 or 1 is used, there
            // is a 1:1 mapping between a network connection and a transport connection.
            if n2.already_has_class_0_transport_conn() || n2.already_has_class_1_transport_conn() {
                drop(n2);
                let system = self.0.read().unwrap();
                if let Some(conn) = system.n_conn_id_to_local_refs
                    .get(&netconn_id)
                    .and_then(|local_ref| system.local_ref_to_conn.get(local_ref)) {
                    return conn.lock().unwrap().transport.receive_DT(&mut conn.lock().unwrap(), dt);
                }
            }
            // The DT-TPDU is _required_ to have a DST-REF above class 1.
            // Meaning if we made it here and DST-REF is None, the DT-TPDU is malformed.
            // Search: 0e3887f6-0dde-4b9d-b72c-8267659a36b5
        }
        else if let TPDU::AK(ak) = &tpdu {
            // If the received TPDU is a DT-TPDU and the network connection has a class 0 or 1 transport connection assigned to it,
            // or an AK-TPDU where a class 1 transport connection is assigned, then the TPDU is associated with the transport
            // connection.
            if n.as_ref().lock().unwrap().already_has_class_1_transport_conn() {
                let system = self.0.read().unwrap();
                if let Some(conn) = system.n_conn_id_to_local_refs
                    .get(&netconn_id)
                    .and_then(|local_ref| system.local_ref_to_conn.get(local_ref)) {
                    return conn.lock().unwrap().transport.receive_AK(&mut conn.lock().unwrap(), ak);
                }
            }
        }

        // "...otherwise the DST-REF parameter of the TPDU is used to identify the transport connection."

        let mut dst_ref = match &tpdu.dst_ref() {
            Some(d) => *d,
            // Search: 0e3887f6-0dde-4b9d-b72c-8267659a36b5
            None => return Err(Error::from(ErrorKind::InvalidInput)),
        };

        // As DST-REF of 0 is allowed in class 4 to disconnect a connection that
        // is in progress. If received, we look up the DST-REF we assigned to
        // the connection based on its SRC-REF.
        let system = self.0.read().unwrap();
        if dst_ref == 0 {
            match tpdu.src_ref() {
                Some(s) => {
                    match system.remote_ref_to_local_ref.get(&(netconn_id, s)) {
                        Some(d) => dst_ref = *d,
                        None => return Err(Error::from(ErrorKind::InvalidInput)),
                    }
                }
                None => return Err(Error::from(ErrorKind::InvalidInput)),
            };
        }
        // If the DST-REF is not allocated to a transport connection, then no association with a transport connection
        // is made and there are three cases:
        // TODO: Can you take here without race conditions? If so, you can drop the system lock much sooner.
        let maybe_conn = system.local_ref_to_conn.get(&dst_ref);
        if maybe_conn.is_none() {
            return self.receive_TPDU_with_dst_ref_unassigned(&n, tpdu, dst_ref);
        }
        let mut conn = maybe_conn.unwrap().lock().unwrap();
        // If the DST-REF is allocated to a transport connection, but the TPDU
        // is received on a network connection to which this connection has not
        // been assigned, then there are four cases:
        if conn.transport.assigned_network_conn.is_none() || conn.transport.assigned_network_conn.is_some_and(|nc| nc != netconn_id) {
            // If the transport connection is of class 4 and if the TPDU is received on a network connection with
            // the same pair of NSAPs as that of the CR-TPDU, then the TPDU is associated with this transport
            // connection and considered as performing assignment.
            // TODO: Review this.
            let sels = (remote_selector, local_selector);
            if conn.transport.class == 4 && conn.transport.cr_nsaps.as_ref().is_some_and(|refs| (&refs.0, &refs.1) == sels) {
                return conn.transport.receive_TPDU(&mut conn, &tpdu);
            }

            let tpdu_type = tpdu.tpdu_type();
            // If the transport connection is not assigned to any network connection (waiting for reassignment after
            // failure) and if the TPDU is received on a network connection with the same pair of NSAPs as that of
            // the CR-TPDU, then the association with that transport connection is made, except in the case of DC,
            // DR and CC-TPDU s which are respectively described in 6.9.1.4.2 c), d) and e).
            let t = &mut conn.transport;
            if t.assigned_network_conn.is_none()
                && t.cr_nsaps.as_ref().is_some_and(|refs| (&refs.0, &refs.1) == sels)
                && tpdu_type != TPDUType::DC
                && tpdu_type != TPDUType::DR
                && tpdu_type != TPDUType::CC {
                t.assigned_network_conn = Some(netconn_id); // REVIEW: I am not sure this is right.
                return t.receive_TPDU(&mut conn, &tpdu);
            }

            // In classes 1 and 3, it is also possible to receive a TPDU performing reassignment prior to the
            // notification of the disconnect of the current network connection (i.e. the transport connection is
            // assigned to a network connection, but a TPDU containing the appropriate DST-REF is received on
            // another network connection). In this case it is recommended that the transport entity:
            if t.assigned_network_conn.is_some() && t.class == 1 || t.class == 3 {
                // FIXME: This API has no access to the whole network layer, so this section cannot be done now.
                // - issue an N-DISCONNECT request on the network connection to which the transport connection
                // is currently assigned;
                // - apply to all transport connections assigned to this network connection the procedure for
                // processing a received N-DISCONNECT indication; and
                // - then process the TPDU performing reassignment.
            }

            // Otherwise, the TPDU is considered as having a DST-REF not allocated to a transport connection [case a)].
            return self.receive_TPDU_with_dst_ref_unassigned(&mut n, tpdu, dst_ref);
        }

        // If the TPDU is a DC-TPDU, then it is associated with the transport connection to which the DST-REF is
        // allocated, unless the SRC-REF is not the expected one, in which case the DC-TPDU is discarded.
        if let TPDU::DC(dc) = &tpdu {
            if conn.transport.remote_ref == dc.src_ref {
                return conn.transport.receive_DC(&mut conn, &dc);
            }
        }

        // If the TPDU is a DR-TPDU , then there are four cases:
        if let TPDU::DR(dr) = &tpdu {
            // 1) if the SRC-REF is not as expected, then a DC-TPDU with DST-REF equal to the SRC-REF of the
            // received DR-TPDU is sent back and no association is made, except that in the case where the
            // transport entity only supports class 0 and cannot transmit a DC-TPDU, it disconnects the network
            // connection instead of transmitting a DC-TPDU;
            if dr.src_ref != conn.transport.remote_ref {
                if SUPPORTS_TP_CLASS_1 || SUPPORTS_TP_CLASS_2 || SUPPORTS_TP_CLASS_3 || SUPPORTS_TP_CLASS_4 {
                    let dc = DC_TPDU {
                        checksum: None,
                        dst_ref: dr.src_ref,
                        src_ref: 0,
                    };
                    n.lock().unwrap().write_nsdu(dc.to_vec())?;
                } else {
                    n.lock().unwrap().close()?;
                }
                return Ok(None);
            }

            // 2) if a CR-TPDU is unacknowledged, then the DR-TPDU is associated with the transport connection,
            // regardless of the value of its SRC-REF parameter;
            if conn.transport.state == X224ConnectionState::WFCC {
                return conn.transport.receive_DR(&mut conn, &dr);
            }

            // 3) if the transport entity implements class 4 and if the DST-REF is zero and there is an unacknowledged
            // CC-TPDU or T-CONNECT response is awaited, then the DR-TPDU shall be associated with the
            // transport connection holding the SRC-REF as the remote reference;
            if SUPPORTS_TP_CLASS_4
                && dr.dst_ref == 0
                && (
                    conn.transport.state == X224ConnectionState::AKWAIT
                    || conn.transport.state == X224ConnectionState::WFTRESP
            ) {
                let remote_ref = (netconn_id, dr.src_ref);
                let system = self.0.read().unwrap();
                let maybe_local_ref = system.remote_ref_to_local_ref.get(&remote_ref);
                if let Some(local_ref) = maybe_local_ref {
                    if *local_ref == conn.transport.local_ref {
                        return conn.transport.receive_DR(&mut conn, &dr);
                    } else {
                        return Ok(None);
                    }
                } else {
                    return Ok(None);
                }
            }

            // 4) otherwise, the DR-TPDU is associated with the transport connection identified by the DST-REF
            // parameter.
            // There is nothing that needs to be done here. Just fall through.
        }

        // If the TPDU is a CC-TPDU whose DST-REF parameter identifies an open connection (one for which
        // a CC-TPDU has been previously received), and the SRC-REF in the CC-TPDU does not match the remote
        // reference, then a DR-TPDU is sent back with DST-REF equal to the SRC-REF of the received CC-TPDU
        // and no association is made.
        if let TPDU::CC(cc) = &tpdu {
            if conn.transport.is_open() && cc.src_ref != conn.transport.remote_ref {
                let dr = DR_TPDU {
                    additional_info: None,
                    checksum: None,
                    src_ref: dst_ref,
                    dst_ref: cc.src_ref,
                    reason: DR_REASON_NOT_SPECIFIED,
                    user_data: Cow::Owned(vec![]),
                };
                n.lock().unwrap().write_nsdu(dr.to_vec())?;
                return Ok(None);
            }
        }

        // If none of the above cases apply, then the TPDU is associated with the transport connection identified by
        // the DST-REF parameter.
        conn.transport.receive_TPDU(&mut conn, &tpdu)
    }

    fn receive_TPDU_with_dst_ref_unassigned <N: NSProvider> (&self, n: &Arc<Mutex<N>>, tpdu: TPDU, dst_ref: u16) -> ServiceResult {
        // 1) If the TPDU is a CC-TPDU the transport entity shall respond on the same network connection with
        // a DR-TPDU . The SRC-REF of the DR-TPDU may be either 0 or the DST-REF from the received
        // CC-TPDU .
        if let TPDU::CC(cc) = tpdu {
            let dr = DR_TPDU {
                additional_info: None,
                checksum: None,
                src_ref: dst_ref,
                dst_ref: 0,
                reason: DR_REASON_NOT_SPECIFIED,
                user_data: std::borrow::Cow::Owned(vec![]),
            };
            n.lock().unwrap().write_nsdu(dr.to_vec())?;
            return Ok(None);
        }

        // 2) If the TPDU is a DR-TPDU, the transport entity shall respond on the same network connection with
        // a DC-TPDU, except in the case that the DR is carrying an SRC-REF set to zero, then no DC-TPDU
        // shall be sent, or in the case where the transport entity only supports class 0, then the network
        // connection shall be disconnected.
        if let TPDU::DR(dr) = tpdu {
            if dr.src_ref != 0 {
                let dc = DC_TPDU {
                    checksum: None,
                    dst_ref: dr.src_ref,
                    src_ref: 0,
                };
                n.lock().unwrap().write_nsdu(dc.to_vec())?;
                return Ok(None);
            }
        }

        // 3) If the TPDU is neither a CC nor a DR it shall be discarded.
        return Ok(None);
    }

    fn create_new_conn <N: NSProvider + Send + Sync + 'static> (
        &mut self,
        n: Arc<Mutex<N>>,
        netconn_id: NetworkConnId2,
        local_selector: &OsiSelector,
        remote_selector: &OsiSelector,
        cr: &CR_TPDU,
    ) -> OSIApplicationAssociation {
        let mut system = self.0.write().unwrap();
        let mut next_ref = system.ref_iter.next();
        if next_ref == 0 {
            next_ref = system.ref_iter.next();
        }
        // FIXME: Awaiting info on what to do when unique ids are exhausted.
        let transport = X224TransportConnection {
            assigned_network_conn: Some(netconn_id),
            remote_ref: cr.src_ref,
            local_t_selector: cr.called_or_responding_transport_selector.as_ref().map(|x| x.as_ref().to_owned()),
            remote_t_selector: cr.calling_transport_selector.as_ref().map(|x| x.as_ref().to_owned()),
            local_ref: next_ref,
            cr_nsaps: Some((remote_selector.to_owned(), local_selector.to_owned())),
            ..Default::default() // TODO: Can this really implement Default anymore?
        };
        OSIApplicationAssociation {
            network_conns: vec![n.clone()],
            transport,
            session: X225SessionConnection::default(),
            presentation: X226PresentationConnection::default(),
        }
    }

}

pub struct OSIApplicationAssociation {
    // NOTE: Using Mutex instead of RwLock for the NSProvider, because:
    // 1. Mutex is more performant.
    // 2. Mutex does not suffer reader or writer starvation. Meaning that the
    //    remote peer cannot stop this server from responding by sending data.
    // 3. There is not quite as much a lopsidedness in network I/O as there is
    //    with looking up the occupied transport refs from the system stack.
    pub network_conns: Vec<Arc<Mutex<dyn NSProvider + Send + Sync>>>, // TODO: Make this a smallvec.
    pub transport: X224TransportConnection,
    pub session: X225SessionConnection,
    pub presentation: X226PresentationConnection,
}

impl OSIApplicationAssociation {

    pub fn receive_nsdu (&mut self, system: Arc<tokio::sync::Mutex<OSITransportLayerState>>, nsdu: &[u8]) -> ServiceResult {

        Ok(None)
    }

}

impl NSProvider for OSIApplicationAssociation {

    // // Actions performed by the local NS-user
    // fn submit_N_CONNECT_request (&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult {

    // }

    // fn submit_N_CONNECT_response (&mut self, params: N_CONNECT_Response_Parameters) -> ServiceResult {

    // }

    // fn submit_N_DATA_request (&mut self, params: N_DATA_Request_Parameters) -> ServiceResult {

    // }

    // fn submit_N_DATA_ACKNOWLEDGE_request (&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult {

    // }

    // fn submit_N_EXPEDITED_DATA_request (&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {

    // }

    // fn submit_N_RESET_request (&mut self, params: N_RESET_Request_Parameters) -> ServiceResult {

    // }

    // fn submit_N_RESET_response (&mut self, params: N_RESET_Response_Parameters) -> ServiceResult {

    // }

    // fn submit_N_DISCONNECT_request (&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {

    // }


    // // Actions that are performed by the remote NS-user.

    // /// For ITOT, this is called upon establishment of the TCP stream.
    // fn receive_N_CONNECT_request(&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult {

    // }

    // /// For ITOT, this is called upon establishment of the TCP stream.
    // fn receive_N_CONNECT_confirm(&mut self, params: N_CONNECT_Confirm_Parameters) -> ServiceResult {

    // }

    // /// For ITOT, this is called upon receipt of a TPKT containing a DT TPDU.
    // fn receive_N_DATA_request(&mut self, params: N_DATA_Request_Parameters) -> ServiceResult {

    // }

    // /// For ITOT, this is called upon receipt of a TPKT containing a AK or EA TPDU.
    // fn receive_N_DATA_ACKNOWLEDGE_request(&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult {

    // }

    // /// For ITOT, this is called upon receipt of a TPKT containing a ED TPDU.
    // fn receive_N_EXPEDITED_DATA_request(&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {

    // }

    // /// I don't think network reset exists for ITOT.
    // fn receive_N_RESET_request(&mut self, params: N_RESET_Request_Parameters) -> ServiceResult {

    // }

    // /// I don't think network reset exists for ITOT.
    // fn receive_N_RESET_confirm(&mut self, params: N_RESET_Confirm_Parameters) -> ServiceResult {

    // }

    // /// For ITOT, this is sent upon closure of the TCP stream.
    // fn receive_N_DISCONNECT_request(&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {

    // }

    // fn id (&self) -> NetworkConnId2 {

    // }

    // fn is_available (&self) -> bool {

    // }

    // fn is_open (&self) -> bool {

    // }

    // fn is_open_in_progress (&self) -> bool {

    // }

    // fn transport_connections_served (&self) -> usize {

    // }

    // fn max_nsdu_size (&self) -> usize {

    // }

    // fn write_nsdu (&mut self, nsdu: UserData) -> Result<(), Error> {

    // }


    // /// This was added to avoid unnecessary re-allocation for concatenation when
    // /// the data will just be written out to a network buffer anyway, and when
    // /// `.write_vectored()` may be used as a performance hack.
    // fn write_nsdu_parts (&mut self, parts: NSDUParts) -> Result<(), Error> {

    // }

    // fn close (&self) -> Result<(), Error> {

    // } // This might not be necessary.

    // fn local_selector (&self) -> &OsiSelector {

    // }

    // fn remote_selector (&self) -> &OsiSelector {

    // }

    // fn selectors (&self) -> RemoteAndLocalSelRefs {
    //     (self.remote_selector(), self.local_selector())
    // }

    // // These are useful because they can queried without locking the whole system.
    // // Only the network connection in question has to be locked for these to be read.
    // fn already_has_class_0_transport_conn (&self) -> bool {

    // }

    // fn already_has_class_1_transport_conn (&self) -> bool {

    // }

    // fn has_no_tc_assigned (&self) -> bool {

    // }

}

impl OSITransportLayerState {

    pub fn associate (&mut self) -> OSIApplicationAssociation {
        let tref = self.next_ref;
        self.next_ref += 1;
        OSIApplicationAssociation {
            network_conns: vec![],
            transport: X224TransportConnection::default(),
            session: X225SessionConnection::default(),
            presentation: X226PresentationConnection::default(),
        }
    }

}

impl Default for OSITransportLayerState {

    fn default() -> Self {
        OSITransportLayerState {
            ref_iter: RandomSequence::<u16>::rand(&mut OsRng),
            n_conn_id_to_local_refs: HashMap::new(),
            remote_ref_to_local_ref: HashMap::new(),
            local_ref_to_conn: HashMap::new(),
            next_ref: 5,
        }
    }

}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use tokio::net::tcp::OwnedWriteHalf;
    use tokio::sync::Mutex;
    use tokio::net::TcpStream;
    use std::sync::Arc;
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use crate::network::{
        N_CONNECT_Request_Parameters,
        N_CONNECT_Confirm_Parameters,
        N_CONNECT_Indication_Parameters,
        N_CONNECT_Response_Parameters,
        N_DATA_ACKNOWLEDGE_Indication_Parameters,
        N_DATA_ACKNOWLEDGE_Request_Parameters,
        N_DISCONNECT_Indication_Parameters,
        N_DISCONNECT_Request_Parameters,
        N_EXPEDITED_DATA_Indication_Parameters,
        N_EXPEDITED_DATA_Request_Parameters,
        N_RESET_Confirm_Parameters,
        N_RESET_Indication_Parameters,
        N_RESET_Request_Parameters,
        N_RESET_Response_Parameters,
        N_CONNECT_Parameters,
        N_DATA_Indication_Parameters,
        N_DATA_Request_Parameters,
    };
    use crate::transport::{NSDUParts, UserData};
    use crate::RemoteAndLocalSelRefs;

    struct TpktParser {
        buf: BytesMut,
    }

    impl TpktParser {

        pub fn receive_data<'a>(&self, data: &'a [u8]) -> Option<&'a [u8]> {
            if data.len() > 4 {
                return None;
            }
            // actually, just pretend this is correctly implemented.
            Some(&data[4..])
        }

    }

    struct AsyncITOTWriteHalf (pub OwnedWriteHalf);

    impl NSProvider for AsyncITOTWriteHalf {
        // Actions performed by the local NS-user
        fn submit_N_CONNECT_request (&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_CONNECT_response (&mut self, params: N_CONNECT_Response_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_DATA_request (&mut self, params: N_DATA_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_DATA_ACKNOWLEDGE_request (&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_EXPEDITED_DATA_request (&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_RESET_request (&mut self, params: N_RESET_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_RESET_response (&mut self, params: N_RESET_Response_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }
        fn submit_N_DISCONNECT_request (&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        // Actions that are performed by the remote NS-user.

        /// For ITOT, this is called upon establishment of the TCP stream.
        fn receive_N_CONNECT_request(&mut self, params: N_CONNECT_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// For ITOT, this is called upon establishment of the TCP stream.
        fn receive_N_CONNECT_confirm(&mut self, params: N_CONNECT_Confirm_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// For ITOT, this is called upon receipt of a TPKT containing a DT TPDU.
        fn receive_N_DATA_request(&mut self, params: N_DATA_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// For ITOT, this is called upon receipt of a TPKT containing a AK or EA TPDU.
        fn receive_N_DATA_ACKNOWLEDGE_request(&mut self, params: N_DATA_ACKNOWLEDGE_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// For ITOT, this is called upon receipt of a TPKT containing a ED TPDU.
        fn receive_N_EXPEDITED_DATA_request(&mut self, params: N_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// I don't think network reset exists for ITOT.
        fn receive_N_RESET_request(&mut self, params: N_RESET_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// I don't think network reset exists for ITOT.
        fn receive_N_RESET_confirm(&mut self, params: N_RESET_Confirm_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        /// For ITOT, this is sent upon closure of the TCP stream.
        fn receive_N_DISCONNECT_request(&mut self, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {
            // TODO:
            todo!()
        }

        fn id (&self) -> NetworkConnId2 {
            // TODO:
            todo!()
        }

        fn is_available (&self) -> bool {
            // TODO:
            todo!()
        }

        fn is_open (&self) -> bool {
            // TODO:
            todo!()
        }

        fn is_open_in_progress (&self) -> bool {
            // TODO:
            todo!()
        }

        fn transport_connections_served (&self) -> usize {
            // TODO:
            todo!()
        }

        fn max_nsdu_size (&self) -> usize {
            // TODO:
            todo!()
        }

        fn write_nsdu (&mut self, nsdu: UserData) -> Result<(), Error> {
            // TODO:
            todo!()
        }


        /// This was added to avoid unnecessary re-allocation for concatenation when
        /// the data will just be written out to a network buffer anyway, and when
        /// `.write_vectored()` may be used as a performance hack.
        fn write_nsdu_parts (&mut self, parts: NSDUParts) -> Result<(), Error> {
            // TODO:
            todo!()
        }
        fn close (&self) -> Result<(), Error> {
            // TODO:
            todo!()
        } // This might not be necessary.
        fn local_selector (&self) -> &OsiSelector {
            // TODO:
            todo!()
        }
        fn remote_selector (&self) -> &OsiSelector {
            // TODO:
            todo!()
        }
        fn selectors (&self) -> RemoteAndLocalSelRefs {
            (self.remote_selector(), self.local_selector())
        }

        // These are useful because they can queried without locking the whole system.
        // Only the network connection in question has to be locked for these to be read.
        fn already_has_class_0_transport_conn (&self) -> bool {
            // TODO:
            todo!()
        }
        fn already_has_class_1_transport_conn (&self) -> bool {
            // TODO:
            todo!()
        }
        fn has_no_tc_assigned (&self) -> bool {
            // TODO:
            todo!()
        }
    }

    async fn process_conn (
        mut system: OSISharedState,
        socket: TcpStream,
    ) -> std::io::Result<()> {
        let mut buf = vec![0; 1024];
        let (mut r, mut w) = socket.into_split();
        let tpkts = TpktParser{ buf: BytesMut::new() };
        let writer = Arc::new(std::sync::Mutex::new(AsyncITOTWriteHalf(w)));
        // NOTE: the writer MUST be Arcd and Mutexd, because it may be shared among many transports.

        // The only problem now is that it seems like the writer MUST use Tokio's Mutex, because it is held across an await.
        // Actually, I think that is not true. You are not holding the lock over an await. It is just that there is
        // an await below and the writer is defined above. There is no problem here, I think.
        loop {
            match r.read(&mut buf).await {
                // Return value of `Ok(0)` signifies that the remote has closed
                Ok(0) => return Ok(()),
                Ok(n) => {
                    let maybe_nsdu = tpkts.receive_data(&buf[..n]);
                    if maybe_nsdu.is_none() {
                        continue;
                    }
                    let nsdu = maybe_nsdu.unwrap();
                    let wake_time = system.receive_nsdu(writer.clone(), nsdu)?;
                    // TODO: Use the wake_time
                    // TODO: I think you can just parse the NSDU into TPDUs here,
                    // then pass each TPDU into the system stack.
                    // stack.lock().
                    // FIXME: I removed the CR from the Transport TPDU to make this work.
                    // assn.receive_nsdu(stack.clone(), nsdu).unwrap();
                    // Copy the data back to socket
                    // if socket.write_all(&buf[..n]).await.is_err() {
                    //     // Unexpected socket error. There isn't much we can
                    //     // do here so just stop processing.
                    //     return Ok(());
                    // }
                }
                Err(_) => {
                    // Unexpected socket error. There isn't much we can do
                    // here so just stop processing.
                    panic!();
                }
            }
        }
    }

    #[tokio::test]
    async fn works_like_i_think_it_will () -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:6142").await?;
        let stack = OSISharedState(Arc::new(std::sync::RwLock::new(OSITransportLayerState::default())));
        loop {
            let stack = stack.clone();
            // FIXME: Actually, the association shall not be made here.
            // it shall be made by the system stack upon receipt of a CR TPDU.
            // let mut assn = stack.0.lock().unwrap().associate();
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                process_conn(stack, socket).await
            });
        }
    }

}
