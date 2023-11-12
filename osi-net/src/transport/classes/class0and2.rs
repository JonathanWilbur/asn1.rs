use std::borrow::Cow;

use crate::network::OSINetworkConnection;
use crate::session::OSIConnectionOrientedSessionService;
use crate::transport::procedures::treatment_of_protocol_errors_over_cons;
use crate::transport::{DR_TPDU, DR_REASON_CR_REFUSED, DR_REASON_NOT_SPECIFIED, DR_REASON_NEGOTIATION_FAILED, DC_TPDU, DR_REASON_NORMAL_DISCONNECT, DR_REASON_PROTOCOL_ERROR, ER_REJECT_CAUSE_INVALID_TPDU_TYPE, IncomingEvent, T_CONNECT_Request_Parameters, CR_TPDU, CC_TPDU};
use crate::transport::conn::{X224TransportConnection, X224ConnectionState};
use crate::transport::pdu::TPDU;
use crate::ServiceResult;
use crate::transport::classes::{StateTablePredicate, handle_invalid_sequence};
use crate::transport::encode::IntoNSDU;

// PREDICATES

/// T-CONNECT request unacceptable
pub(crate) fn P0 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    req: &T_CONNECT_Request_Parameters,
) -> bool {
    // TODO: Validate QoS paramters at some point in the future.
    req.user_data.len() > 32
}

/// Unacceptable CR-TPDU
pub(crate) fn P1 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: &CR_TPDU,
) -> bool {
    if pdu.version_number != 1 {
        return true;
    }
    if let Some(local_sel) = &t.local_t_selector {
        if pdu.called_or_responding_transport_selector.as_ref().is_some_and(|sel| sel.as_ref() != local_sel) {
            return true;
        }
    }
    false
}

/// No network connection available
pub(crate) fn P2 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    _n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> bool {
    // When would there not be a network connection available?
    false
}

/// Network connection available and open
pub(crate) fn P3 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> bool {
    n.is_open()
}

/// Network connection available and open in progress
pub(crate) fn P4 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> bool {
    n.is_open_in_progress()
}

/// Class in class 0 (class selected in CC)
pub(crate) fn P5 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    _n: &mut N,
    t: &mut X224TransportConnection,
    _s: &mut S,
    pdu: Option<&TPDU>,
) -> bool {
    if let Some(tpdu) = pdu {
        if let TPDU::CC(cc) = tpdu {
            // I don't know if this is correct.
            return (cc.class_option & 0b1111_0000) == 0;
        }
    }
    t.class == 0
}

/// Unacceptable CC
pub(crate) fn P6 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> bool {
    let pdu = pdu.expect("No TPDU provided to P6 for class 0 or 2 transport.");
    if t.cr.is_none() {
        debug_assert!(false, "No CR to match up to the received CC.");
        return true;
    }
    let cr = t.cr.as_ref().unwrap();
    if let TPDU::CC(cc) = pdu {
        !acceptable_cc(&cr, &cc)
    } else {
        debug_assert!(false, "TPDU provided to P6 for class 0 or 2 transport was not a CC.");
        true
    }
}

/// Class is class 2
pub(crate) fn P7 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    _n: &mut N,
    t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> bool {
    t.class == 2
}

/// Acceptable CC
pub(crate) fn P8 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> bool {
    let pdu = pdu.expect("No TPDU provided to P8 for class 0 or 2 transport.");
    if t.cr.is_none() {
        debug_assert!(false, "No CR to match up to the received CC.");
        return false;
    }
    let cr = t.cr.as_ref().unwrap();
    if let TPDU::CC(cc) = pdu {
        acceptable_cc(&cr, &cc)
    } else {
        debug_assert!(false, "TPDU provided to P8 for class 0 or 2 transport was not a CC.");
        false
    }
}

/// Class 4 CR
pub(crate) fn P9 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    _n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    pdu: Option<&TPDU>,
) -> bool {
    if pdu.is_none() {
        return false;
    }
    let pdu = pdu.unwrap();
    if let TPDU::CR(cr) = pdu {
        (cr.class_option & 0b0100_0000) == 0b0100_0000
    } else {
        false
    }
}

/// Local choice
pub(crate) fn P10 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    _n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> bool {
    true
}

// ACTIONS

/// If the network connection is not used by another transport connection
/// assigned to it, it may be disconnected. (See 6.1.1.3, Note 3).
pub(crate) fn A1 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> ServiceResult {
    if n.has_no_tc_assigned() {
        n.close()?;
    }
    Ok(None)
}

/// See 6.22 (receipt of an ER-TPDU).
pub(crate) fn A2 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> ServiceResult {
    if pdu.is_none() {
        return Ok(None);
    }
    treatment_of_protocol_errors_over_cons(n, t, s, pdu.unwrap(), None, None)
}

/// See data transfer procedures of the class.
pub(crate) fn A3 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> ServiceResult {
    // TODO:
    todo!()
}

/// See expedited data transfer procedure of the class.
pub(crate) fn A4 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> ServiceResult {
    // TODO:
    todo!()
}

/// An N-RESET response has to be issued once for the network
/// connection if the network connection has not been released.
/// In class 0, an N-DISCONNECT request has to be issued.
pub(crate) fn A5 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> ServiceResult {
    // TODO:
    todo!()
}

/// The DC-TPDU contains a src-ref field set to zero and a dst-ref field set
/// to the SRC-REF of the DR-TPDU received.
pub(crate) fn A6 <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    _n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> ServiceResult {
    // I think this should not have been an action at all. I think it was really
    // just a note. I did this in the body of the state table below.
    Ok(None)
}

// STATE TABLE

/// I can't find anywhere in X.224 where it says to treat the TPDU as invalid if
/// it matches none of the conditions in the state table's cell, but I think it
/// must be done. I am defining this function specifically for this case so I
/// can easily remove it by name later on if I turn out to be wrong.
pub(crate) fn implied_handle_invalid <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
    event_came_from_ts_user: bool,
) -> ServiceResult {
    handle_invalid_sequence(n, t, s, pdu, event_came_from_ts_user)
}

pub(crate) fn dispatch_tpdu <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: &TPDU,
) -> ServiceResult {
    match (pdu, t.state) {
        (TPDU::CR(tpdu), X224ConnectionState::OPEN)
        | (TPDU::CR(tpdu), X224ConnectionState::CLOSING)
        | (TPDU::CR(tpdu), X224ConnectionState::WFTRESP) => {
            if !P9(n, t, s, Some(pdu)) {
                return implied_handle_invalid(n, t, s, Some(pdu), false);
            }
            Ok(None)
        },
        (TPDU::CR(tpdu), X224ConnectionState::CLOSED) => {
            if P1(n, t, s, tpdu) {
                let dr = DR_TPDU{
                    dst_ref: t.remote_ref,
                    reason: if t.class == 0 { DR_REASON_NOT_SPECIFIED } else { DR_REASON_NEGOTIATION_FAILED },
                    additional_info: Some(b"unacceptable"),
                    checksum: None,
                    src_ref: t.local_ref,
                    user_data: Cow::Owned(vec![]),
                };
                let nsdu_parts = dr.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
                n.write_nsdu_parts(nsdu_parts)?;
            } else {
                // TODO: Send TCONind to S
                t.state = X224ConnectionState::WFTRESP;
            }
            Ok(None)
        },

        // What does it mean when the state table row is split horizontally?
        (TPDU::DR(tpdu), X224ConnectionState::WFCC) => {
            // TODO: Send TDISind to S
            t.state = X224ConnectionState::CLOSED;
            A1(n, t, s, Some(pdu))
        },
        (TPDU::DR(tpdu), X224ConnectionState::WBCL)
        | (TPDU::DR(tpdu), X224ConnectionState::CLOSING) => {
            t.state = X224ConnectionState::CLOSED;
            A1(n, t, s, Some(pdu))
        },
        (TPDU::DR(tpdu), X224ConnectionState::OPEN) => {
            if P5(n, t, s, Some(pdu)) {
                // At the current time, this will have the effect of simply
                // closing the connection.
                return treatment_of_protocol_errors_over_cons(n, t, s, pdu, None, None);
            }
            else if P7(n, t, s, Some(pdu)) {
                let dc = DC_TPDU{
                    checksum: None,
                    dst_ref: t.remote_ref,
                    src_ref: t.local_ref,
                };
                let nsdu_parts = dc.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
                n.write_nsdu_parts(nsdu_parts)?;
                // TODO: Send TDISind to S
                t.state = X224ConnectionState::CLOSED;
                Ok(None)
            }
            else {
                panic!("Invalid implementation: invalid class {}", t.class);
            }
        },
        (TPDU::DR(tpdu), X224ConnectionState::WFTRESP) => {
            if P10(n, t, s, Some(pdu)) {
                A6(n, t, s, Some(pdu))?;
                let dc = DC_TPDU{
                    checksum: None,
                    dst_ref: tpdu.src_ref,
                    src_ref: 0, // [6]
                };
                let nsdu_parts = dc.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
                n.write_nsdu_parts(nsdu_parts)?;
            }
            // TODO: Send TDISind to S
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        (TPDU::DR(tpdu), X224ConnectionState::CLOSED) => {
            if tpdu.src_ref != 0 {
                let dc = DC_TPDU{
                    checksum: None,
                    dst_ref: tpdu.src_ref,
                    src_ref: 0,
                };
                let nsdu_parts = dc.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
                n.write_nsdu_parts(nsdu_parts)?;
            }
            Ok(None)
        },

        (TPDU::DC(tpdu), X224ConnectionState::CLOSING) => {
            if t.class == 0 {
                return handle_invalid_sequence(n, t, s, Some(pdu), false);
            }
            t.state = X224ConnectionState::CLOSED;
            if P7(n, t, s, Some(pdu)) {
                A1(n, t, s, Some(pdu))
            } else {
                Ok(None)
            }
        },

        (TPDU::CC(tpdu), X224ConnectionState::WFCC) => {
            if P8(n, t, s, Some(pdu)) {
                // TODO: Send TCONconf to S
                t.state = X224ConnectionState::OPEN;
            }
            if P6(n, t, s, Some(pdu)) {
                // TODO: Send TDISind to S
                if P5(n, t, s, Some(pdu)) {
                    // TODO: Send TDISind to S
                    n.close()?; // TODO: NDISreq
                    t.state = X224ConnectionState::CLOSED;
                }
                if P7(n, t, s, Some(pdu)) {
                    let dr = DR_TPDU{
                        dst_ref: t.remote_ref,
                        reason: if t.class == 0 { DR_REASON_NOT_SPECIFIED } else { DR_REASON_NEGOTIATION_FAILED },
                        additional_info: Some(b"unacceptable"),
                        checksum: None,
                        src_ref: t.local_ref,
                        user_data: Cow::Owned(vec![]),
                    };
                    let nsdu_parts = dr.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
                    n.write_nsdu_parts(nsdu_parts)?;
                    t.state = X224ConnectionState::CLOSING;
                }
                Ok(None)
            } else {
                handle_invalid_sequence(n, t, s, Some(pdu), false)
            }
        },
        (TPDU::CC(tpdu), X224ConnectionState::WBCL) => {
            if P5(n, t, s, Some(pdu)) {
                n.close()?; // TODO: NDISreq
                t.state = X224ConnectionState::CLOSED;
            }
            if P7(n, t, s, Some(pdu)) {
                let dr = DR_TPDU{
                    dst_ref: t.remote_ref,
                    reason: if t.class == 0 { DR_REASON_NOT_SPECIFIED } else { DR_REASON_NORMAL_DISCONNECT },
                    additional_info: Some(b"unacceptable"),
                    checksum: None,
                    src_ref: t.local_ref,
                    user_data: Cow::Owned(vec![]),
                };
                let nsdu_parts = dr.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
                n.write_nsdu_parts(nsdu_parts)?;
                t.state = X224ConnectionState::CLOSING;
            }
            Ok(None)
        },
        (TPDU::CC(tpdu), X224ConnectionState::CLOSING) => {
            Ok(None)
        },
        (TPDU::CC(tpdu), X224ConnectionState::CLOSED) => {
            let dr = DR_TPDU{
                dst_ref: t.remote_ref,
                reason: if t.class == 0 { DR_REASON_NOT_SPECIFIED } else { DR_REASON_PROTOCOL_ERROR },
                additional_info: Some(b"idk you"), // These messages are intentionally short to avoid overflowing.
                checksum: None,
                src_ref: t.local_ref,
                user_data: Cow::Owned(vec![]),
            };
            let nsdu_parts = dr.to_nsdu_parts(t.class, t.use_extended_format, t.is_checksummed());
            n.write_nsdu_parts(nsdu_parts)?;
            Ok(None)
        },

        (TPDU::AK(tpdu), X224ConnectionState::OPEN) => {
            if t.class == 0 {
                return treatment_of_protocol_errors_over_cons(n, t, s, pdu, None, Some(ER_REJECT_CAUSE_INVALID_TPDU_TYPE));
            }
            A3(n, t, s, Some(pdu))
        },
        (TPDU::AK(tpdu), X224ConnectionState::CLOSING) => {
            if t.class == 0 {
                return treatment_of_protocol_errors_over_cons(n, t, s, pdu, None, Some(ER_REJECT_CAUSE_INVALID_TPDU_TYPE));
            }
            Ok(None)
        },
        (TPDU::AK(tpdu), X224ConnectionState::CLOSED) => {
            Ok(None)
        },

        (TPDU::EA(_), X224ConnectionState::OPEN)
        | (TPDU::ED(_), X224ConnectionState::OPEN) => {
            if t.class == 0 {
                return treatment_of_protocol_errors_over_cons(n, t, s, pdu, None, Some(ER_REJECT_CAUSE_INVALID_TPDU_TYPE));
            }
            A4(n, t, s, Some(pdu))
        },
        (TPDU::EA(_), X224ConnectionState::CLOSING)
        | (TPDU::ED(_), X224ConnectionState::OPEN) => {
            if t.class == 0 {
                return treatment_of_protocol_errors_over_cons(n, t, s, pdu, None, Some(ER_REJECT_CAUSE_INVALID_TPDU_TYPE));
            }
            Ok(None)
        },
        (TPDU::EA(_), X224ConnectionState::CLOSED)
        | (TPDU::ED(_), X224ConnectionState::OPEN) => {
            Ok(None)
        },


        (TPDU::DT(tpdu), X224ConnectionState::OPEN) => {
            A3(n, t, s, Some(pdu))
        },
        (TPDU::DT(tpdu), X224ConnectionState::CLOSING) => {
            Ok(None)
        },
        (TPDU::DT(tpdu), X224ConnectionState::CLOSED) => {
            Ok(None)
        },

        (TPDU::ER(tpdu), X224ConnectionState::WFCC) => {
            // TODO: Send TDISind to S
            t.state = X224ConnectionState::CLOSED;
            A1(n, t, s, Some(pdu))
        },
        (TPDU::ER(tpdu), X224ConnectionState::WBCL) => {
            t.state = X224ConnectionState::CLOSED;
            A1(n, t, s, Some(pdu))
        },
        (TPDU::ER(tpdu), X224ConnectionState::OPEN)
        | (TPDU::ER(tpdu), X224ConnectionState::CLOSING) => {
            treatment_of_protocol_errors_over_cons(n, t, s, pdu, None, None)
        },
        (TPDU::ER(tpdu), X224ConnectionState::CLOSED) => {
            Ok(None)
        },

        _ => handle_invalid_sequence(n, t, s, Some(pdu), false),
    }
}

pub(crate) fn dispatch_event <N: OSINetworkConnection, S: OSIConnectionOrientedSessionService> (
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    event: &IncomingEvent,
) -> ServiceResult {
    if let IncomingEvent::TPDU(tpdu) = event {
        return dispatch_tpdu(n, t, s, tpdu);
    }
    match (event, t.state) {
        (IncomingEvent::TCONreq(e), X224ConnectionState::CLOSED) => {
            if P0(n, t, s, e) {
                // TODO: Send TDISind to S
                t.state = X224ConnectionState::CLOSED;
                return Ok(None);
            }
            if P2(n, t, s, None) {
                // TODO: Send NCONreq to N.
                t.state = X224ConnectionState::WFNC;
                return Ok(None);
            }
            if P3(n, t, s, None) {
                // TODO: Send CR.
                t.state = X224ConnectionState::WFCC;
                return Ok(None);
            }
            if P4(n, t, s, None) {
                t.state = X224ConnectionState::WFNC;
                return Ok(None);
            }
            handle_invalid_sequence(n, t, s, None, true)
        },
        (IncomingEvent::TCONresp(e), X224ConnectionState::WFTRESP) => {
            // TODO: CC
            t.state = X224ConnectionState::OPEN;
            Ok(None)
        },
        (IncomingEvent::TDTreq(e), X224ConnectionState::OPEN) => {
            // TODO: [3]
            Ok(None)
        },
        (IncomingEvent::TEXreq(e), X224ConnectionState::OPEN) => {
            if t.class == 0 {
                //
            }
            // TODO: [4]
            Ok(None)
        },
        (IncomingEvent::TDISreq(e), X224ConnectionState::WFNC) => {
            // [1]
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        (IncomingEvent::TDISreq(e), X224ConnectionState::WFCC) => {
            if !P7(n, t, s, None) {
                n.close()?; // NDISreq
                t.state = X224ConnectionState::CLOSED;
            } else {
                t.state = X224ConnectionState::WBCL;
            }
            Ok(None)
        },
        (IncomingEvent::TDISreq(e), X224ConnectionState::OPEN) => {
            if P5(n, t, s, None) {
                n.close()?; // NDISreq
                t.state = X224ConnectionState::CLOSED;
            }
            else if P7(n, t, s, None) {
                // TODO: DR
                t.state = X224ConnectionState::CLOSING;
            }
            Ok(None)
        },
        (IncomingEvent::TDISreq(e), X224ConnectionState::WFTRESP) => {
            // DR
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        (IncomingEvent::NCONconf(e), X224ConnectionState::WFNC) => {
            // TODO: CR
            t.state = X224ConnectionState::WFCC;
            Ok(None)
        },
        (IncomingEvent::NRSTind(e), X224ConnectionState::WFCC)
        | (IncomingEvent::NRSTind(e), X224ConnectionState::OPEN)
        | (IncomingEvent::NRSTind(e), X224ConnectionState::WFTRESP) => {
            // TDISind
            // [1] [5]
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        (IncomingEvent::NRSTind(e), X224ConnectionState::WBCL)
        | (IncomingEvent::NRSTind(e), X224ConnectionState::CLOSING) => {
            // [1] [5]
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        (IncomingEvent::NDISind(e), X224ConnectionState::WFNC)
        | (IncomingEvent::NDISind(e), X224ConnectionState::WFCC)
        | (IncomingEvent::NDISind(e), X224ConnectionState::OPEN)
        | (IncomingEvent::NDISind(e), X224ConnectionState::WFTRESP) => {
            // TDISind
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        (IncomingEvent::NDISind(e), X224ConnectionState::WBCL)
        | (IncomingEvent::NDISind(e), X224ConnectionState::CLOSING) => {
            t.state = X224ConnectionState::CLOSED;
            Ok(None)
        },
        _ => handle_invalid_sequence(n, t, s, None, false), // FIXME: This might actually be user-initiated.
    }
}


pub fn acceptable_cc (cr: &CR_TPDU, cc: &CC_TPDU) -> bool {
    // Refs must match.
    if cr.src_ref != cc.dst_ref {
        return false;
    }

    // CC cannot propose options that CR did not present.
    let cr_ext_format: bool = (cr.class_option & 0b0000_0010) > 0;
    let cr_explicit_flow_control: bool = (cr.class_option & 0b0000_0001) > 0;
    let cc_ext_format: bool = (cc.class_option & 0b0000_0010) > 0;
    let cc_explicit_flow_control: bool = (cc.class_option & 0b0000_0001) > 0;
    if !cr_ext_format && cc_ext_format {
        return false;
    }
    if !cr_explicit_flow_control && cc_explicit_flow_control {
        return false;
    }
    match (cr.tpdu_size, cc.tpdu_size) {
        (Some(cr_size), Some(cc_size)) => {
            if cc_size > cr_size {
                return false;
            }
        }
        _ => (),
    };
    match (cr.preferred_max_tpdu_size, cc.preferred_max_tpdu_size) {
        (Some(cr_size), Some(cc_size)) => {
            if cc_size > cr_size {
                return false;
            }
        }
        _ => (),
    };

    let preferred_class = cr.class_option & 0b1111_0000;
    let chosen_class = cc.class_option & 0b1111_0000;

    // See X.224, Table 3.
    if preferred_class == 0 && chosen_class != 0 {
        return false;
    }
    if chosen_class != preferred_class
        // It is valid to choose 0 if 2 is requested, but 0 is an alt.
        && !(preferred_class == 2 && chosen_class == 0 && (cr.alternative_protocol_classes & 1) == 1)
    {
        let bit = 1 << chosen_class;
        let mut alt = cr.alternative_protocol_classes;
        // Anywhere class 1 is proposed, class 0 is allowed as well.
        if (alt & 0b10) == 1 {
            alt |= 1;
        }
        // Anywhere classes 3 or 4 are proposed, class 2 is allowed as well.
        if (alt & 0b1000) == 1 || (alt & 0b0001_0000) == 1 {
            alt |= 0b100;
        }
        // If true, the CR did not propose that class.
        if (cr.alternative_protocol_classes & bit) == 0 {
            return false;
        }
    }

    true
}
