

/**
 * ACPM states as defined in table A.2 of ITU Recommendation X.227 (1995),
 * Annex A.
 */
pub enum TableA2AssociationState {
    /** idle: unassociated */
    STA0,
    /** awaiting AARE APDU */
    STA1,
    /** awaiting A-ASSOCIATE response */
    STA2,
    /** awaiting RLRE APDU */
    STA3,
    /** awaiting A-RELEASE response */
    STA4,
    /** associated */
    STA5,
    /** awaiting A-RELEASE response (association initiator) */
    STA6,
    /** awaiting RLRE APDU (association initiator) */
    STA7,
}

pub struct ACPMState {
    state: TableA2AssociationState,
    // outgoingEvents: ACSEOutgoingEventEmitter;
    initiator: bool,
    // presentation: PresentationService;
    // TODO: responding_AP_title: Option<AP_title>,
    // TODO: responding_AE_qualifier: Option<AE_qualifier>,
    // TODO: responding_AP_invocation_identifier: Option<AP_invocation_identifier>,
    // TODO: responding_AE_invocation_identifier: Option<AE_invocation_identifier>,
    // acse_authenticate?: (aarq: AARQ_apdu) => boolean;
}
