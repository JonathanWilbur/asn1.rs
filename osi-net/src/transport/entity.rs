use crate::OsiSelector;
use crate::WakeTime;
use crate::network::NSProvider;
use crate::network::NSUser;
use crate::session::OSIConnectionOrientedSessionService;
use crate::transport::pdu::TPDU;
use std::borrow::Cow;
use std::collections::HashMap;
use crate::transport::conn::X224TransportConnection;
use crate::transport::TransportRef;
use crate::transport::OSIConnectionOrientedTransportService;
use crate::transport::service::{
    T_CONNECT_Request_Parameters,
    T_CONNECT_Response_Parameters,
    T_DATA_Request_Parameters,
    T_EXPEDITED_DATA_Request_Parameters,
    T_DISCONNECT_Request_Parameters,
    T_CONNECT_Confirm_Parameters,
};
use std::io::{Error, ErrorKind};
use crate::ServiceResult;
use rand::rngs::OsRng;
use rand_sequence::RandomSequence;
use smallvec::SmallVec;
use crate::network::{
    NetworkConnId,
    N_CONNECT_Confirm_Parameters,
    N_DISCONNECT_Request_Parameters,
    N_RESET_Confirm_Parameters,
};

use super::COTSProvider;
use super::COTSUser;
use super::parse_x224_nsdu;
use super::parse_x224_tpdu;
use super::{CR_TPDU, CLASS_OPTION_4, DR_TPDU, DR_REASON_NOT_SPECIFIED, DC_TPDU, TPDUType, X224ConnectionState};

pub const SUPPORTS_TP_CLASS_0: bool = true;
pub const SUPPORTS_TP_CLASS_1: bool = true;
pub const SUPPORTS_TP_CLASS_2: bool = true;
pub const SUPPORTS_TP_CLASS_3: bool = true;
pub const SUPPORTS_TP_CLASS_4: bool = true;

/// The SRC-REF from a CR TPDU is not guaranteed to be universally unique, but
/// it should be unique on the originating host. The network connection ID is
/// used to ensure global uniqueness, because repeated CR TPDUs MUST be sent
/// over the same network connection, even if splitting is used.
pub type RemoteRef = (NetworkConnId, TransportRef);

pub struct X224TransportEntity {
    // Instead of RC<conn>, just take from the hashmap and put it back when done.
    pub local_ref_to_conn: HashMap<TransportRef, X224TransportConnection>,
    pub remote_ref_to_local_ref: HashMap<RemoteRef, TransportRef>,

    /// Yes, it is possible for there to be multiple network connections that
    /// service a given transport connection (splitting) and there may be
    /// multiple transport connections that share a network connection
    /// (multiplexing).
    pub n_conn_id_to_local_refs: HashMap<NetworkConnId, SmallVec<[TransportRef; 8]>>,

    // FIXME: Actually use this to verify authorization to a particular transport ref.
    pub local_ref_to_n_conn_ids: HashMap<TransportRef, SmallVec<[NetworkConnId; 8]>>,

    pub selector: Option<OsiSelector>,
    pub ref_iter: RandomSequence<u16>,
}

impl X224TransportEntity {

    pub fn new () -> Self {
        X224TransportEntity {
            local_ref_to_conn: HashMap::new(),
            remote_ref_to_local_ref: HashMap::new(),
            n_conn_id_to_local_refs: HashMap::new(),
            local_ref_to_n_conn_ids: HashMap::new(),
            selector: None,
            ref_iter: RandomSequence::<u16>::rand(&mut OsRng),
        }
    }

    fn create_new_conn <N: NSProvider> (&mut self, n: &N, cr: &CR_TPDU) -> X224TransportConnection {
        let mut next_ref = self.ref_iter.next();
        if next_ref == 0 {
            next_ref = self.ref_iter.next();
        }
        // FIXME: Awaiting info on what to do when unique ids are exhausted.
        X224TransportConnection {
            assigned_network_conn: Some(n.id()),
            remote_ref: cr.src_ref,
            local_t_selector: cr.called_or_responding_transport_selector.as_ref().map(|x| x.as_ref().to_owned()),
            remote_t_selector: cr.calling_transport_selector.as_ref().map(|x| x.as_ref().to_owned()),
            local_ref: next_ref,
            cr_nsaps: Some((n.remote_selector().to_owned(), n.local_selector().to_owned())),
            ..Default::default() // TODO: Can this really implement Default anymore?
        }
    }

    fn receive_TPDU_with_dst_ref_unassigned <N: NSProvider> (&self, n: &mut N, tpdu: TPDU, dst_ref: u16) -> ServiceResult {
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
                user_data: Cow::Owned(vec![]),
            };
            n.write_nsdu(dr.to_vec())?;
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
                n.write_nsdu(dc.to_vec())?;
                return Ok(None);
            }
        }

        // 3) If the TPDU is neither a CC nor a DR it shall be discarded.
        return Ok(None);
    }

    // NOTE: I _think_ there can be only one session conn per transport at a time.
    // ChatGPT disagrees, but can't provide evidence and contradicts itself.
    // I misunderstood how sessions work. Sessions can be created and terminated
    // readily, and instead, activities survive across session connections.
    // X.200: "The Transport Layer uniquely identifies each session-entity by its transport-address."
    // X.200: "Transport-connections provide duplex transmission between a pair of session-entites (through transport-SAPs)."
    // BINGO: X.200: "There is a one-to-one mapping between a session-connection and a transport-connection at any given instant."
    /// Implements X.224, Section 6.9.1.4.2
    fn dispatch_tpdu <N: NSProvider> (&mut self, n: &mut N, tpdu: TPDU) -> ServiceResult {
        if let TPDU::CR(cr) = tpdu {
            if cr.src_ref == 0 {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
            let remote_ref = (n.id(), cr.src_ref);
            // I think this paragraph is written poorly, but I think what it
            // is saying is that "if the SRC-REF + remote NSAP combo is
            // already taken, this TPDU obviously belongs to that connection."
            let maybe_conn = self
                .remote_ref_to_local_ref.get(&remote_ref)
                .and_then(|local_ref| self.local_ref_to_conn.get_mut(local_ref));
            if let Some(conn) = maybe_conn {
                /* "If the received TPDU is a CR-TPDU, and if the SRC-REF
                parameter and the remote NSAP indicate an existing transport
                connection at that receiving entity, then the CR-TPDU is
                associated with that transport connection" */
                if conn.assigned_network_conn.is_some_and(|nc| nc == n.id()) {
                    return conn.receive_CR(n, &cr);
                }
            }
            let already_has_class_0_or_1_transport_conn = n.already_has_class_0_transport_conn()
                || n.already_has_class_0_transport_conn();
            if already_has_class_0_or_1_transport_conn && cr.class_option == CLASS_OPTION_4 {
                // Discard the CR TPDU. It seems to imply "do nothing."
                return Ok(None);
            }
            let mut conn = self.create_new_conn(n, &cr);
            let result = conn.receive_CR(n, &cr);
            if result.is_ok() {
                // FIXME: self.n_conn_id_to_local_refs.insert(n.id(), conn.local_ref);
                self.remote_ref_to_local_ref.insert(remote_ref, conn.local_ref);
                self.local_ref_to_conn.insert(conn.local_ref, conn);
            }
            // TODO: Remote user could be trying to exhaust local refs. Somehow block / punish them.
            return result;
        }
        if let TPDU::DT(dt) = &tpdu {
            // If the received TPDU is a DT-TPDU and the network connection has no TC assigned to it, and the DT-TPDU is a
            // class 0 or class 1 TPDU (as recognized by the absence of a DST-REF field), then the TPDU should be ignored.
            if n.has_no_tc_assigned() && dt.dst_ref.is_none() {
                return Ok(None);
            }

            // If the received TPDU is a DT-TPDU and the network connection has a class 0 or 1 transport connection
            // assigned to it, or an AK-TPDU where a class 1 transport connection is assigned, then the TPDU is
            // associated with the transport connection.
            if n.already_has_class_0_transport_conn() || n.already_has_class_1_transport_conn() {
                // FIXME:
                // if let Some(conn) = self.n_conn_id_to_local_refs
                //     .get(&n.id())
                //     .and_then(|local_ref| self.local_ref_to_conn.get_mut(local_ref)) {
                //     return conn.receive_DT(n, dt);
                // }
            }
            // The DT-TPDU is _required_ to have a DST-REF above class 1.
            // Meaning if we made it here and DST-REF is None, the DT-TPDU is malformed.
            // Search: 0e3887f6-0dde-4b9d-b72c-8267659a36b5
        }
        else if let TPDU::AK(ak) = &tpdu {
            // If the received TPDU is a DT-TPDU and the network connection has a class 0 or 1 transport connection assigned to it,
            // or an AK-TPDU where a class 1 transport connection is assigned, then the TPDU is associated with the transport
            // connection.
            if n.already_has_class_1_transport_conn() {
                // FIXME:
                // if let Some(conn) = self.n_conn_id_to_local_refs
                //     .get(&n.id())
                //     .and_then(|local_ref| self.local_ref_to_conn.get_mut(local_ref)) {
                //     return conn.receive_AK(n, ak);
                // }
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
        if dst_ref == 0 {
            match tpdu.src_ref() {
                Some(s) => {
                    match self.remote_ref_to_local_ref.get(&(n.id(), s)) {
                        Some(d) => dst_ref = *d,
                        None => return Err(Error::from(ErrorKind::InvalidInput)),
                    }
                }
                None => return Err(Error::from(ErrorKind::InvalidInput)),
            };
        }
        // If the DST-REF is not allocated to a transport connection, then no association with a transport connection
        // is made and there are three cases:
        let maybe_conn = self.local_ref_to_conn.get_mut(&dst_ref);
        if maybe_conn.is_none() {
            return self.receive_TPDU_with_dst_ref_unassigned(n, tpdu, dst_ref);
        }
        let conn = maybe_conn.unwrap();
        // If the DST-REF is allocated to a transport connection, but the TPDU
        // is received on a network connection to which this connection has not
        // been assigned, then there are four cases:
        if conn.assigned_network_conn.is_none() || conn.assigned_network_conn.is_some_and(|nc| nc != n.id()) {
            // If the transport connection is of class 4 and if the TPDU is received on a network connection with
            // the same pair of NSAPs as that of the CR-TPDU, then the TPDU is associated with this transport
            // connection and considered as performing assignment.
            if conn.class == 4 && conn.cr_nsaps.as_ref().is_some_and(|refs| (&refs.0, &refs.1) == n.selectors()) {
                return conn.receive_TPDU(n, &tpdu);
            }

            let tpdu_type = tpdu.tpdu_type();
            // If the transport connection is not assigned to any network connection (waiting for reassignment after
            // failure) and if the TPDU is received on a network connection with the same pair of NSAPs as that of
            // the CR-TPDU, then the association with that transport connection is made, except in the case of DC,
            // DR and CC-TPDU s which are respectively described in 6.9.1.4.2 c), d) and e).
            if conn.assigned_network_conn.is_none()
                && conn.cr_nsaps.as_ref().is_some_and(|refs| (&refs.0, &refs.1) == n.selectors())
                && tpdu_type != TPDUType::DC
                && tpdu_type != TPDUType::DR
                && tpdu_type != TPDUType::CC {
                conn.assigned_network_conn = Some(n.id()); // REVIEW: I am not sure this is right.
                return conn.receive_TPDU(n, &tpdu);
            }

            // In classes 1 and 3, it is also possible to receive a TPDU performing reassignment prior to the
            // notification of the disconnect of the current network connection (i.e. the transport connection is
            // assigned to a network connection, but a TPDU containing the appropriate DST-REF is received on
            // another network connection). In this case it is recommended that the transport entity:
            if conn.assigned_network_conn.is_some() && conn.class == 1 || conn.class == 3 {
                // FIXME: This API has no access to the whole network layer, so this section cannot be done now.
                // - issue an N-DISCONNECT request on the network connection to which the transport connection
                // is currently assigned;
                // - apply to all transport connections assigned to this network connection the procedure for
                // processing a received N-DISCONNECT indication; and
                // - then process the TPDU performing reassignment.
            }

            // Otherwise, the TPDU is considered as having a DST-REF not allocated to a transport connection [case a)].
            return self.receive_TPDU_with_dst_ref_unassigned(n, tpdu, dst_ref);
        }

        // If the TPDU is a DC-TPDU, then it is associated with the transport connection to which the DST-REF is
        // allocated, unless the SRC-REF is not the expected one, in which case the DC-TPDU is discarded.
        if let TPDU::DC(dc) = &tpdu {
            if conn.remote_ref == dc.src_ref {
                return conn.receive_DC(n, &dc);
            }
        }

        // If the TPDU is a DR-TPDU , then there are four cases:
        if let TPDU::DR(dr) = &tpdu {
            // 1) if the SRC-REF is not as expected, then a DC-TPDU with DST-REF equal to the SRC-REF of the
            // received DR-TPDU is sent back and no association is made, except that in the case where the
            // transport entity only supports class 0 and cannot transmit a DC-TPDU, it disconnects the network
            // connection instead of transmitting a DC-TPDU;
            if dr.src_ref != conn.remote_ref {
                if SUPPORTS_TP_CLASS_1 || SUPPORTS_TP_CLASS_2 || SUPPORTS_TP_CLASS_3 || SUPPORTS_TP_CLASS_4 {
                    let dc = DC_TPDU {
                        checksum: None,
                        dst_ref: dr.src_ref,
                        src_ref: 0,
                    };
                    n.write_nsdu(dc.to_vec())?;
                } else {
                    n.close()?;
                }
                return Ok(None);
            }

            // 2) if a CR-TPDU is unacknowledged, then the DR-TPDU is associated with the transport connection,
            // regardless of the value of its SRC-REF parameter;
            if conn.state == X224ConnectionState::WFCC {
                return conn.receive_DR(n, &dr);
            }

            // 3) if the transport entity implements class 4 and if the DST-REF is zero and there is an unacknowledged
            // CC-TPDU or T-CONNECT response is awaited, then the DR-TPDU shall be associated with the
            // transport connection holding the SRC-REF as the remote reference;
            if SUPPORTS_TP_CLASS_4
                && dr.dst_ref == 0
                && (
                    conn.state == X224ConnectionState::AKWAIT
                    || conn.state == X224ConnectionState::WFTRESP
            ) {
                let remote_ref = (n.id(), dr.src_ref);
                let maybe_local_ref = self.remote_ref_to_local_ref.get(&remote_ref);
                if let Some(local_ref) = maybe_local_ref {
                    if *local_ref == conn.local_ref {
                        return conn.receive_DR(n, &dr);
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
            if conn.is_open() && cc.src_ref != conn.remote_ref {
                let dr = DR_TPDU {
                    additional_info: None,
                    checksum: None,
                    src_ref: dst_ref,
                    dst_ref: cc.src_ref,
                    reason: DR_REASON_NOT_SPECIFIED,
                    user_data: Cow::Owned(vec![]),
                };
                n.write_nsdu(dr.to_vec())?;
                return Ok(None);
            }
        }

        // If none of the above cases apply, then the TPDU is associated with the transport connection identified by
        // the DST-REF parameter.
        conn.receive_TPDU(n, &tpdu)
    }

}

// impl <S: COTSUser<Self>> COTSProvider<S> for X224TransportEntity<S> where S: Sized + Default {

//     fn submit_T_CONNECT_request(&mut self, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn submit_T_CONNECT_response(&mut self, s: &mut S, params: T_CONNECT_Response_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn submit_T_DATA_request(&mut self, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn submit_T_EXPEDITED_DATA_request(&mut self, s: &mut S, params: T_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn submit_T_DISCONNECT_request(&mut self, s: &mut S, params: T_DISCONNECT_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn receive_T_CONNECT_request(&mut self, s: &mut S, params: T_CONNECT_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn receive_T_CONNECT_confirm(&mut self, s: &mut S, params: T_CONNECT_Confirm_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn receive_T_DATA_request(&mut self, s: &mut S, params: T_DATA_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn receive_T_EXPEDITED_DATA_request(&mut self, s: &mut S, params: T_EXPEDITED_DATA_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

//     fn receive_T_DISCONNECT_request(&mut self, s: &mut S, params: T_DISCONNECT_Request_Parameters) -> ServiceResult {
//         // TODO:
//         todo!()
//     }

// }

// This entity intentionally implements NSUser, but not COTSProvider.
impl <N: NSProvider> NSUser<N> for X224TransportEntity {

    fn receive_nsdu(&mut self, n: &mut N, nsdu: Vec<u8>) -> ServiceResult {
        let mut b = nsdu.as_slice();
        let mut some_succeeded: bool = false;
        let mut last_err: Option<Error> = None;
        let mut wake_time: WakeTime = None;
        // TODO: Check that each TPDU except the last one belongs to a separate connection.
        while b.len() > 0 {
            match parse_x224_tpdu(b) {
                Ok((s, tpdu)) => {
                    match self.dispatch_tpdu(n, tpdu) {
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

    fn receive_N_CONNECT_confirm(&mut self, n: &mut N, params: N_CONNECT_Confirm_Parameters) -> ServiceResult {
        let local_refs = match self.n_conn_id_to_local_refs.get(&n.id()) {
            Some(r) => r,
            None => return Err(Error::from(ErrorKind::NotConnected)),
        };
        let mut ret: WakeTime = None;
        for local_ref in local_refs.iter() {
            let maybe_conn = self.local_ref_to_conn.get_mut(local_ref);
            if maybe_conn.is_none() {
                continue;
            }
            let conn = maybe_conn.unwrap();
            let timer = match conn.receive_N_CONNECT_confirm(n, params.clone()) {
                Ok(t) => t,
                Err(e) => {
                    // Errors are intentionally ignored.
                    println!("Error propagating NCONconf {}", e);
                    continue
                },
            };
            if ret.is_none() || (timer.is_some() && (timer.unwrap() < ret.unwrap())) {
                ret = timer;
            }
        }
        Ok(ret)
    }

    fn receive_N_DISCONNECT_indication(&mut self, n: &mut N, params: N_DISCONNECT_Request_Parameters) -> ServiceResult {
        let local_refs = match self.n_conn_id_to_local_refs.get(&n.id()) {
            Some(r) => r,
            None => return Err(Error::from(ErrorKind::NotConnected)),
        };
        let mut ret: WakeTime = None;
        for local_ref in local_refs.iter() {
            let maybe_conn = self.local_ref_to_conn.get_mut(local_ref);
            if maybe_conn.is_none() {
                continue;
            }
            let conn = maybe_conn.unwrap();
            let timer = match conn.receive_N_DISCONNECT_indication(n, params.clone()) {
                Ok(t) => t,
                Err(e) => {
                    // Errors are intentionally ignored.
                    println!("Error propagating NDISind {}", e);
                    continue
                },
            };
            if ret.is_none() || (timer.is_some() && (timer.unwrap() < ret.unwrap())) {
                ret = timer;
            }
        }
        Ok(ret)
    }

    fn receive_N_RESET_indication(&mut self, n: &mut N, params: N_RESET_Confirm_Parameters) -> ServiceResult {
        let local_refs = match self.n_conn_id_to_local_refs.get(&n.id()) {
            Some(r) => r,
            None => return Err(Error::from(ErrorKind::NotConnected)),
        };
        let mut ret: WakeTime = None;
        for local_ref in local_refs.iter() {
            let maybe_conn = self.local_ref_to_conn.get_mut(local_ref);
            if maybe_conn.is_none() {
                continue;
            }
            let conn = maybe_conn.unwrap();
            let timer = match conn.receive_N_RESET_indication(n, params.clone()) {
                Ok(t) => t,
                Err(e) => {
                    // Errors are intentionally ignored.
                    println!("Error propagating NRSTind {}", e);
                    continue
                },
            };
            if ret.is_none() || (timer.is_some() && (timer.unwrap() < ret.unwrap())) {
                ret = timer;
            }
        }
        Ok(ret)
    }

}
