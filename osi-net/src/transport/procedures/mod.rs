use crate::network::N_DISCONNECT_Request_Parameters;
use crate::{network::NSProvider, stack::OSIApplicationAssociation};
use crate::transport::pdu::TPDU;
use crate::ServiceResult;
use std::borrow::Cow;
use std::ops::Add;
use std::time::SystemTime;
use crate::transport::encode::IntoNSDU;
use super::{ER_TPDU, ER_REJECT_CAUSE_NOT_SPECIFIED};

/// Implements the procedures described in Section 6.22 of
/// [ITU-T Recommendation X.224 (1995)](https://www.itu.int/rec/T-REC-X.224/en)
/// related to handling protocol errors.
pub(crate) fn treatment_of_protocol_errors_over_cons <'a> (
    stack: &mut OSIApplicationAssociation,
    pdu: &'a TPDU,
    invalid_tpdu: Option<Cow<'a, [u8]>>,
    reject_cause: Option<u8>,
) -> ServiceResult {

    // FIXME: Check that the invalid_tpdu will fit in the allowed NSDU/TPDU size.
    // To avoid producing an infinite loop, we do nothing in response to an
    // invalid ER-TPDU.
    if let TPDU::ER(_) = pdu {
        return Ok(None);
    }

    // For some reason, only in class 0, the invalid TPDU parameter is required
    // in an ER-TPDU. If we don't have this information (and I doubt this
    // implementation ever will), we just close the network connection for a
    // class 0 transport.
    if stack.transport.class == 0 && invalid_tpdu.is_none() {
        let disc = N_DISCONNECT_Request_Parameters{ // FIXME:
            reason: 0,
            ns_user_data: vec![],
            responding_address: vec![],
        };
        stack.submit_N_DISCONNECT_request(disc)?;
        return Ok(None);
    }
    let use_checksum = stack.transport.class == 4 && stack.transport.use_checksum_in_class_4;
    let er = ER_TPDU {
        dst_ref: stack.transport.remote_ref,
        reject_cause: reject_cause.unwrap_or(ER_REJECT_CAUSE_NOT_SPECIFIED),
        invalid_tpdu,
        checksum: None,
    };
    let nsdu_parts = er.to_nsdu_parts(stack.transport.class, stack.transport.use_extended_format, use_checksum);
    stack.submit_N_DATA_request_parts(nsdu_parts)?;

    // See NOTE 1.
    if let TPDU::CR(_) = pdu {
        // TODO: Kill the network connection.
    }

    // It is recommended that the sender of the ER-TPDU starts an optional timer
    // TS2 to ensure the release of the connection. If the timer expires, the
    // transport entity shall initiate the release procedures appropriate to the
    // class. The timer should be stopped when a DR-TPDU or an N-DISCONNECT
    // indication is received.
    let now = SystemTime::now();
    let ts2 = now.add(stack.transport.ts2_duration);
    stack.transport.ts2 = Some(ts2);
    Ok(Some(ts2))
}
