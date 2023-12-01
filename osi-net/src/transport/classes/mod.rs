use crate::network::NSProvider;
use crate::stack::OSIApplicationAssociation;
use crate::transport::conn::X224TransportConnection;
use crate::transport::pdu::TPDU;
use crate::ServiceResult;
use crate::transport::procedures::treatment_of_protocol_errors_over_cons;

use super::COTSUser;
pub mod class0and2;

pub(crate) type StateTablePredicate <N, S> = fn(
    n: &mut N,
    t: &mut X224TransportConnection,
    s: &mut S,
    pdu: Option<&TPDU>,
) -> bool;

pub(crate) fn transport_noop <N: NSProvider, S: Default + COTSUser<X224TransportConnection>> (
    _n: &mut N,
    _t: &mut X224TransportConnection,
    _s: &mut S,
    _pdu: Option<&TPDU>,
) -> bool {
    false
}

/// From Annex A, Section A.2.3 of
/// [ITU-T Recommendation X.224 (1995)](https://www.itu.int/rec/T-REC-X.224/en):
///
/// > The intersection of each state and event which is invalid is left blank.
/// The action to be taken in this case shall be one of the following:
///
/// > a) for an event related to the transport service (i.e. coming from the
///    TS-user), take no action;
///
/// > b) for an event related to a received TPDU, follow the procedure for
///    treatment of protocol errors (see 6.22) if the state of the supporting
///    network connection makes it possible;
///
/// > c) for an event falling into neither of the above categories (including
///    those which are impossible by the definition of the behaviour of the
///    transport entity or NS-provider), take no action.
///
pub(crate) fn handle_invalid_sequence (
    stack: &mut OSIApplicationAssociation,
    pdu: Option<&TPDU>,
    event_came_from_ts_user: bool,
) -> ServiceResult {
    // Annex A, Section A.2.3.a
    if event_came_from_ts_user {
        return Ok(None);
    }
    // Annex A, Section A.2.3.c
    if pdu.is_none() {
        return Ok(None);
    }
    // Otherwise, Annex A, Section A.2.3.b
    treatment_of_protocol_errors_over_cons(stack, pdu.unwrap(), None, None)
}
