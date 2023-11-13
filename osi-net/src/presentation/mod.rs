mod service;
use crate::{OsiSelector, session::{SerialNumber, ActivityIdentifier}};
// import { Context_list_Item } from '@wildboar/copp/src/lib/modules/ISO8823-PRESENTATION/Context-list-Item.ta';
// import {
//     Result,
//     Result_acceptance,
// } from '@wildboar/copp/src/lib/modules/ISO8823-PRESENTATION/Result.ta';

/// The presentation-layer states defined in ITU Recommendation X.226 (1994),
/// Annex A, Table A.17.
///
pub enum X226ConnectionState {
    /// Idle - no connection
    STAI0,
    /// Await CPA PPDU
    STAI1,
    /// Await P-CONNECT response
    STAI2,
    /// Connected - Data Transfer
    STAt0,
    /// Await ACA PPDU
    STAac0,
    /// Await P-ALTER-CONTEXT response
    STAac1,
    /// Await ACA PPDU or P-ALTER-CONTEXT response
    STAac2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncPointIdentifier {
    NoActMan(SerialNumber),
    WithActMan((SerialNumber, SerialNumber, ActivityIdentifier)),
}

pub struct ContextSets {
    // TODO:
    // proposed_for_addition_initiated_locally: Vec<Context_list_Item>,
    // proposed_for_addition_initiated_remotely: Vec<(Context_list_Item, Result)>,
    // proposed_for_deletion_initiated_locally: Vec<Context_list_Item>,
    // proposed_for_deletion_initiated_remotely: Vec<Context_list_Item>,
    // dcs_agreed_during_connection_establishment: Map<
    //     Presentation_context_identifier,
    //     Context_list_Item,
    // >,
    // inter_activity_dcs: Vec<Context_list_Item>,
    // contents_of_the_dcs_at_synchronization_points: Map<
    //     SyncPointIdentifier,
    //     Vec<Context_list_Item>,
    // >,
    // default_context: Option<Default_context_name>,
}


pub struct X225PresentationConnection {

    /**
     * Function for selecting presentation contexts.
     */
    // get_preferred_context: (
    //     context_list: Context_list_Item[]
    // ) => Context_list_Item;

    /**
     * Function for determining whether a particular presentation context is acceptable.
     */
    // is_context_acceptable: (
    //     abstract_syntax: OBJECT_IDENTIFIER,
    //     transfer_syntax: OBJECT_IDENTIFIER
    // ) => boolean;

    /**
     * This connection's interface to the session-layer.
     */
    // session: SessionLayer;

    /// The state of the connection, as defined in the state table in
    /// ITU Recommendation X.226 (1994), Annex A, Table A.17.
    state: X226ConnectionState,

    /// Whether activity end is pending.
    aep: bool,

    /// Whether the release phase has been started.
    rl: bool,

    /// Whether a collision of release requests has been detected.
    cr: bool,

    /// Whether the context-management function unit is enabled.
    FU_CM: bool,

    /// Whether the context-restoration function unit is enabled.
    FU_CR: bool,

    /// Presentation context sets defined within this presentation connection.
    contextSets: ContextSets,

    /**
     * The CP PPDU issued to create this connection. (This must be preserved
     * because it is used for determining future behavior.)
     */
    // TODO: cp: Option<CP_type>,

    /// The maximum number of contexts that may appear in the context definition
    /// list, or which may exist in total for this presentation connection.
    max_contexts: usize,

    local_selector: Option<OsiSelector>,

    remote_selector: Option<OsiSelector>,
}
