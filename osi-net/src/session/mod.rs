use std::collections::HashMap;
use bytes::BytesMut;
use crate::{OsiSelector, transport::TransportRef};
mod layer;
mod service;
pub use layer::*;
pub use service::*;

/// [ITU-T Recommendation X.225 (1995)](https://www.itu.int/rec/T-REC-X.225/en),
/// Section 8.3.1.10:
///
/// > Serial number can range from 0 to (10**Upper Limit Serial Number) − 1.
///
/// And in section 8.3.1.13:
///
/// > [The Upper Limit Serial Number's] maximum value is 6 and its default value
/// > is 6.
///
/// It is not clear at all what the Large Initial Serial Number is, but I think
/// it exists just because the other Initial Serial Number parameter is limited
/// to six bytes (and each byte encodes a decimal digit), so the serial number
/// without this parameter would be limited to 999,999. However, the only way
/// that numbers above this can be used is if the Upper Limit Serial Number
/// parameter is set to 0, which allows an infinitely large serial number.
///
/// Either way, it seems that a `usize` should be fine.
///
pub type SerialNumber = usize;

// It is limited to 6 bytes.
pub type ActivityIdentifier = u64;

pub const DEFAULT_MAX_SSDU_SIZE: usize = 10_000_000;
/**
 * There is technically no limit on TSDU size if none is specified, but still,
 * I believe ITOT and the full OSI protocol stack only support up to 64K-ish
 * bytes per TSDU, so this implementation will use an implicit default of
 * 64K (minus a few bytes to avoid bugs).
 */
pub const DEFAULT_MAX_TSDU_SIZE: usize = 65500;

// #region protocol_version

pub const SESSION_PROTOCOL_VERSION_1: u8 = 1;
pub const SESSION_PROTOCOL_VERSION_2: u8 = 2;

// #endregion

// #region connection states

// Table A.2 in Annex A of ITU Rec. X.225.
// Yes, there are some numbers missing from the sequence, such as STA07 and STA17.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum X225ConnectionState {

    /// Idle, no transport connection
    STA01,

    /// Wait for the ABORT ACCEPT SPDU
    STA01A,

    /// Wait for the T-CONNECT confirm
    STA01B,

    /// Idle, transport connected
    STA01C,

    /// Wait for the CONNECT DATA OVERFLOW SPDU
    STA01D,

    /// Wait for the ACCEPT SPDU
    STA02A,

    /// Wait for the OVERFLOW ACCEPT SPDU
    STA02B,

    /// Wait for the DISCONNECT SPDU
    STA03,

    /// Wait for the MAJOR SYNC ACK SPDU or PREPARE SPDU
    STA04A,

    /// Wait for the ACTIVITY END ACK SPDU or PREPARE SPDU
    STA04B,

    /// Wait for the RESYNCHRONIZE ACK SPDU or PREPARE SPDU
    STA05A,

    /// Wait for the ACTIVITY INTERRUPT ACK SPDU or PREPARE SPDU
    STA05B,

    /// Wait for the ACTIVITY DISCARD ACK SPDU or PREPARE SPDU
    STA05C,

    /// Wait for the RESYNCHRONIZE SPDU
    STA06,

    /// Wait for S-CONNECT response
    STA08,

    /// Wait for S-RELEASE response
    STA09,

    /// Wait for S-SYNC-MAJOR response
    STA10A,

    /// Wait for S-ACTIVITY-END response
    STA10B,

    /// Wait for S-RESYNCHRONIZE response
    STA11A,

    /// Wait for S-ACTIVITY-INTERRUPT response
    STA11B,

    /// Wait for S-ACTIVITY-DISCARD response
    STA11C,

    /// After PREPARE, wait for MAJOR SYNC ACK SPDU or ACTIVITY END ACK SPDU
    STA15A,

    /// After PREPARE, wait for RESYNCHRONIZE SPDU or ACTIVITY INTERRUPT SPDU or ACTIVITY DISCARD SPDU
    STA15B,

    /// After PREPARE, wait for RESYNCHRONIZE ACK SPDU or ACTIVITY INTERRUPT ACK SPDU or ACTIVITY DISCARD ACK SPDU
    STA15C,

    /// After PREPARE, wait for the ABORT SPDU
    STA15D,

    /// Wait for T-DISCONNECT indication
    STA16,

    /// Wait for the GIVE TOKENS ACK SPDU
    STA18,

    /// Wait for a recovery request or SPDU
    STA19,

    /// Wait for a recovery SPDU or request
    STA20,

    /// Wait for the CAPABILITY DATA ACK SPDU
    STA21,

    /// Wait for S-CAPABILITY-DATA response
    STA22,

    /// Data transfer state
    STA713,
}

// #endregion

// #region error_code

// pub const ERR_MULTIPLE_SPDU_IN_TSDU: u8 = -1;
// pub const ERR_SINGLE_SPDU_IN_TSDU: u8 = -2;
// pub const ERR_UNRECOGNIZED_SPDU: u8 = -3;
// pub const ERR_INVALID_SEQ: u8 = -4;
// pub const ERR_UNRECOGNIZED_PI: u8 = -5;
// pub const ERR_PI_LENGTH: u8 = -6;
// pub const ERR_DUPLICATE_PARAM: u8 = -7;
// pub const ERR_MISSING_REQ_PARAM: u8 = -8;
// pub const ERR_MALFORMED_PARAM: u8 = -9;
// pub const ERR_UNSUPPORTED_SPDU: u8 = -10;
// pub const ERR_UNSUPPORTED_PREPARE_TYPE: u8 = -11;
// pub const ERR_SSDU_TOO_LARGE: u8 = -12;

// #endregion error_code

// #region si

pub type SI = u8;

pub const SI_CN_SPDU: SI = 13; // CONNECT
pub const SI_OA_SPDU: SI = 16; // OVERFLOW ACCEPT
pub const SI_CDO_SPDU: SI = 15; // CONNECT DATA OVERFLOW
pub const SI_AC_SPDU: SI = 14; // ACCEPT
pub const SI_RF_SPDU: SI = 12; // REFUSE
pub const SI_FN_SPDU: SI = 9; // FINISH
pub const SI_DN_SPDU: SI = 10; // DISCONNECT
pub const SI_NF_SPDU: SI = 8; // NOT FINISHED
pub const SI_AB_SPDU: SI = 25; // ABORT // WARNING: DUPLICATE
pub const SI_AA_SPDU: SI = 26; // ABORT ACCEPT // WARNING: DUPLICATE
pub const SI_DT_SPDU: SI = 1; // DATA TRANSFER // WARNING: DUPLICATE
pub const SI_EX_SPDU: SI = 5; // EXPEDITED DATA
pub const SI_TD_SPDU: SI = 33; // TYPED DATA
pub const SI_CD_SPDU: SI = 61; // CAPABILITY DATA
pub const SI_CDA_SPDU: SI = 62; // CAPABILITY DATA ACK
pub const SI_GT_SPDU: SI = 1; // GIVE TOKENS
pub const SI_PT_SPDU: SI = 2; // PLEASE TOKENS
pub const SI_GTC_SPDU: SI = 3; // GIVE TOKENS CONFIRM
pub const SI_GTA_SPDU: SI = 22; // GIVE TOKENS ACK
pub const SI_MIP_SPDU: SI = 49; // MINOR SYNC POINT
pub const SI_MIA_SPDU: SI = 50; // MINOR SYNC ACK
pub const SI_MAP_SPDU: SI = 41; // MAJOR SYNC POINT
pub const SI_MAA_SPDU: SI = 42; // MAJOR SYNC ACK
pub const SI_RS_SPDU: SI = 53; // RESYNCHRONIZE
pub const SI_RA_SPDU: SI = 34; // RESYNCHRONIZE ACK
pub const SI_PR_SPDU: SI = 7; // PREPARE
pub const SI_ER_SPDU: SI = 0; // EXCEPTION REPORT
pub const SI_ED_SPDU: SI = 48; // EXCEPTION DATA
pub const SI_AS_SPDU: SI = 45; // ACTIVITY START
pub const SI_AR_SPDU: SI = 29; // ACTIVITY RESUME
pub const SI_AI_SPDU: SI = 25; // ACTIVITY INTERRUPT
pub const SI_AIA_SPDU: SI = 26; // ACTIVITY INTERRUPT ACK
pub const SI_AD_SPDU: SI = 57; // ACTIVITY DISCARD
pub const SI_ADA_SPDU: SI = 58; // ACTIVITY DISCARD ACK
pub const SI_AE_SPDU: SI = SI_MAP_SPDU; // ACTIVITY END
pub const SI_AEA_SPDU: SI = SI_MAA_SPDU; // ACTIVITY END ACK

// Warning: GT_SPDU_SI = DT_SPDU_SI.
// Warning: AI_SPDU_SI = AB_SPDU_SI.
// Warning: AA_SPDU_SI = AIA_SPDU_SI.

pub fn is_category_0_spdu (si: SI) -> bool {
    si == SI_GT_SPDU || si == SI_PT_SPDU
}

pub fn is_category_1_spdu (si: SI) -> bool {
    [
        SI_CN_SPDU,
        SI_OA_SPDU,
        SI_CDO_SPDU,
        SI_AC_SPDU,
        SI_RF_SPDU,
        SI_FN_SPDU,
        SI_DN_SPDU,
        SI_NF_SPDU,
        SI_AB_SPDU,
        SI_AA_SPDU,
        SI_GTC_SPDU,
        SI_GTA_SPDU,
        SI_PR_SPDU,
        SI_TD_SPDU,
    ].contains(&si)
}

pub fn is_category_2_spdu (si: SI) -> bool {
    [
        SI_DT_SPDU,
        SI_MIP_SPDU,
        SI_MIA_SPDU,
        SI_MAP_SPDU,
        SI_MAA_SPDU,
        SI_RS_SPDU,
        SI_RA_SPDU,
        SI_AS_SPDU,
        SI_AR_SPDU,
        SI_AD_SPDU,
        SI_ADA_SPDU,
        // SI_AI_SPDU, // Same as ABORT
        // SI_AIA_SPDU, // Same as ABORT ACCEPT
        SI_AE_SPDU,
        SI_AEA_SPDU,
        SI_CD_SPDU,
        SI_CDA_SPDU,
        SI_ER_SPDU,
        SI_ED_SPDU,
    ].contains(&si)
}

// #endregion si

// #region pgi

pub type PGI = u8;

pub const PGI_CONNECTION_ID: PGI = 1;
pub const PGI_CONNECT_ACCEPT: PGI = 5;
pub const PGI_USER_DATA: PGI = 193;
pub const PGI_EXTENDED_USER_DATA: PGI = 194;
pub const PGI_LINKING_INFO: PGI = 33;

/// PGIs that only contain a value, not PIs.
pub fn is_value_only_pgi (pgi: PGI) -> bool {
    pgi == PGI_USER_DATA || pgi == PGI_EXTENDED_USER_DATA
}

// #endregion pgi

// #region

pub type PI = u8;

pub const PI_CALLING_SS_USER_REF: PI = 10;
pub const PI_COMMON_REFERENCE: PI = 11;
pub const PI_ADDITIONAL_REF_INFO: PI = 12;
pub const PI_PROTOCOL_OPTIONS: PI = 19;
pub const PI_TSDU_MAX_SIZE: PI = 21;
pub const PI_VERSION_NUMBER: PI = 22;
pub const PI_INITIAL_SERIAL_NUMBER: PI = 23;
pub const PI_TOKEN_SETTING_ITEM: PI = 26;
pub const PI_SECOND_INITIAL_SERIAL_NUMBER: PI = 55;
pub const PI_UPPER_LIMIT_SERIAL_NUMBER: PI = 56;
pub const PI_LARGE_INITIAL_SERIAL_NUMBER: PI = 57;
pub const PI_LARGE_SECOND_INITIAL_SERIAL_NUMBER: PI = 58;
pub const PI_SESSION_USER_REQUIREMENTS: PI = 20;
pub const PI_CALLING_SESSION_SELECTOR: PI = 51;
pub const PI_CALLED_SESSION_SELECTOR: PI = 52;
pub const PI_RESPONDING_SESSION_SELECTOR: PI = 52; // Same as PI_CALLED_SESSION_SELECTOR.
pub const PI_DATA_OVERFLOW: PI = 60;
pub const PI_ENCLOSURE_ITEM: PI = 25;
pub const PI_USER_DATA: PI = 22;
pub const PI_CALLED_SS_USER_REF: PI = 9;
pub const PI_TOKEN_ITEM: PI = 16;
pub const PI_TRANSPORT_DISCONNECT: PI = 17;
pub const PI_REASON_CODE: PI = 50;
pub const PI_REFLECT_PARAMETER_VALUES: PI = 49;
pub const PI_SYNC_TYPE_ITEM: PI = 15;
pub const PI_SERIAL_NUMBER: PI = 42;
pub const PI_USER_DATA_2: PI = 46;
pub const PI_SECOND_SERIAL_NUMBER: PI = 54;
pub const PI_RESYNC_TYPE: PI = 27;
pub const PI_SECOND_RESYNC_TYPE: PI = 53;
pub const PI_PREPARE_TYPE: PI = 24;
pub const PI_ACTIVITY_IDENTIFIER: PI = 41;
pub const PI_OLD_ACTIVITY_IDENTIFIER: PI = 41;
pub const PI_NEW_ACTIVITY_IDENTIFIER: PI = 41; // Same as PI_OLD_ACTIVITY_IDENTIFIER.

// #endregion

// #region serial_number_digits

pub const SERIAL_NUMBER_DIGIT_0: u8 = 0b0011_0000;
pub const SERIAL_NUMBER_DIGIT_1: u8 = 0b0011_0001;
pub const SERIAL_NUMBER_DIGIT_2: u8 = 0b0011_0010;
pub const SERIAL_NUMBER_DIGIT_3: u8 = 0b0011_0011;
pub const SERIAL_NUMBER_DIGIT_4: u8 = 0b0011_0100;
pub const SERIAL_NUMBER_DIGIT_5: u8 = 0b0011_0101;
pub const SERIAL_NUMBER_DIGIT_6: u8 = 0b0011_0110;
pub const SERIAL_NUMBER_DIGIT_7: u8 = 0b0011_0111;
pub const SERIAL_NUMBER_DIGIT_8: u8 = 0b0011_1000;
pub const SERIAL_NUMBER_DIGIT_9: u8 = 0b0011_1001;

pub fn serial_number_digit_to_char (snd: u8) -> Option<char> {
    match snd {
        SERIAL_NUMBER_DIGIT_0 => Some('0'),
        SERIAL_NUMBER_DIGIT_1 => Some('1'),
        SERIAL_NUMBER_DIGIT_2 => Some('2'),
        SERIAL_NUMBER_DIGIT_3 => Some('3'),
        SERIAL_NUMBER_DIGIT_4 => Some('4'),
        SERIAL_NUMBER_DIGIT_5 => Some('5'),
        SERIAL_NUMBER_DIGIT_6 => Some('6'),
        SERIAL_NUMBER_DIGIT_7 => Some('7'),
        SERIAL_NUMBER_DIGIT_8 => Some('8'),
        SERIAL_NUMBER_DIGIT_9 => Some('9'),
        _ => None,
    }
}

pub fn char_to_serial_number_digit (c: char) -> Option<u8> {
    match c {
        '0' => Some(SERIAL_NUMBER_DIGIT_0),
        '1' => Some(SERIAL_NUMBER_DIGIT_1),
        '2' => Some(SERIAL_NUMBER_DIGIT_2),
        '3' => Some(SERIAL_NUMBER_DIGIT_3),
        '4' => Some(SERIAL_NUMBER_DIGIT_4),
        '5' => Some(SERIAL_NUMBER_DIGIT_5),
        '6' => Some(SERIAL_NUMBER_DIGIT_6),
        '7' => Some(SERIAL_NUMBER_DIGIT_7),
        '8' => Some(SERIAL_NUMBER_DIGIT_8),
        '9' => Some(SERIAL_NUMBER_DIGIT_9),
        _ => None,
    }
}

// #endregion

// #region token_setting

pub const TOKEN_SETTING_INITIATOR_SIDE: u8 = 0b00;
pub const TOKEN_SETTING_RESPONDER_SIDE: u8 = 0b01;
pub const TOKEN_SETTING_CALLED_SS_USER_CHOICE: u8 = 0b10;
pub const TOKEN_SETTING_RESERVED: u8 = 0b11;

// #endregion

// #region session_user_reqs

pub const SUR_HALF_DUPLEX: u16 = 1 << 0;
pub const SUR_DUPLEX: u16 = 1 << 1;
pub const SUR_EXPEDITED_DATA: u16 = 1 << 2;
pub const SUR_MINOR_SYNC: u16 = 1 << 3;
pub const SUR_MAJOR_SYNC: u16 = 1 << 4;
pub const SUR_RESYNC: u16 = 1 << 5;
pub const SUR_ACTIVITY_MANAGEMENT: u16 = 1 << 6;
pub const SUR_NEGOTIATED_RELEASE: u16 = 1 << 7;
pub const SUR_CAPABILITY_EXCHANGE: u16 = 1 << 8;
pub const SUR_EXCEPTIONS: u16 = 1 << 9;
pub const SUR_TYPED_DATA: u16 = 1 << 10;
pub const SUR_SYMMETRIC_SYNC: u16 = 1 << 11;
pub const SUR_DATA_SEPARATION: u16 = 1 << 12;

// #endregion session_user_reqs

// #region ABORT_transport_disconnect

pub const TRANSPORT_DISCONNECT_KEPT: u8 = 1 << 0;
pub const TRANSPORT_DISCONNECT_RELEASED: u8 = 1 << 1;
pub const TRANSPORT_DISCONNECT_USER_ABORT: u8 = 1 << 2;
pub const TRANSPORT_DISCONNECT_PROTOCOL_ERROR: u8 = 1 << 3;
pub const TRANSPORT_DISCONNECT_NO_REASON: u8 = 1 << 4;
pub const TRANSPORT_DISCONNECT_IMPLEMENTATION_RESTRICTED: u8 = 1 << 5;

// #endregion

// #region RESYNC_TYPE

pub const RESYNC_TYPE_RESTART: u8 = 0;
pub const RESYNC_TYPE_ABANDON: u8 = 1;
pub const RESYNC_TYPE_SET: u8 = 2;

// #endregion RESYNC_TYPE

// #region PREPARE_TYPE

pub const PREPARE_TYPE_MAJOR_SYNC_ACK: u8 = 1;
pub const PREPARE_TYPE_RESYNC: u8 = 2;
pub const PREPARE_TYPE_RESYNC_ACK: u8 = 3;
pub const PREPARE_TYPE_ABORT: u8 = 4;

// #endregion

// #region reason_code

pub const RC_REASON_NOT_SPECIFIED: u8 = 0;
pub const RC_TEMPORARY_CONGESTION: u8 = 1;
pub const RC_REJECTED_BY_CALLED_SS_USER: u8 = 2;
pub const RC_SESSION_SELECTOR_UNKNOWN: u8 = 128 + 1;
pub const RC_SS_USER_NOT_ATTACHED_SSAP: u8 = 128 + 2;
pub const RC_SPM_CONGESTION_AT_CONNECT: u8 = 128 + 3;
pub const RC_PROPOSED_PROTOCOL_VERSIONS_NOT_SUPPORTED: u8 = 128 + 4;
pub const RC_REJECTION_BY_SPM_REASON_NOT_SPECIFIED: u8 = 128 + 5;
pub const RC_REJECTION_BY_SPM_IMPLEMENTATION_RESTRICTION_STATED_IN_PICS: u8 =
    128 + 6;
pub const RC_NO_REASON: u8 = 0;
pub const RC_TEMPORARILY_UNABLE_TO_CONTINUE: u8 = 1;
pub const RC_RESERVED_2: u8 = 2;
pub const RC_USER_SEQ_ERROR: u8 = 3;
pub const RC_RESERVED_4: u8 = 4;
pub const RC_LOCAL_SS_USER_ERROR: u8 = 5;
pub const RC_UNRECOVERABLE_PROCEDURAL_ERROR: u8 = 6;
pub const RC_DEMAND_DATA_TOKEN: u8 = 128;

// #endregion reason_code

pub struct CONNECT_SPDU {
    // TODO:
}

pub struct OVERFLOW_ACCEPT_SPDU {
    // TODO:
}

pub struct CONNECT_DATA_OVERFLOW_SPDU {
    // TODO:
}

pub struct ACCEPT_SPDU {
    // TODO:
}

pub struct REFUSE_SPDU {
    // TODO:
}

pub struct FINISH_SPDU {
    // TODO:
}

pub struct DISCONNECT_SPDU {
    // TODO:
}

pub struct NOT_FINISHED_SPDU {
    // TODO:
}

pub struct ABORT_SPDU {
    // TODO:
}

pub struct ABORT_ACCEPT_SPDU {
    // TODO:
}

pub struct DATA_TRANSFER_SPDU {
    // TODO:
}

pub struct EXPEDITED_DATA_SPDU {
    // TODO:
}

pub struct TYPED_DATA_SPDU {
    // TODO:
}

pub struct CAPABILITY_DATA_SPDU {
    // TODO:
}

pub struct CAPABILITY_DATA_ACK_SPDU {
    // TODO:
}

pub struct GIVE_TOKENS_SPDU {
    // TODO:
}

pub struct PLEASE_TOKENS_SPDU {
    // TODO:
}

pub struct GIVE_TOKENS_CONFIRM_SPDU {
    // TODO:
}

pub struct GIVE_TOKENS_ACK_SPDU {
    // TODO:
}

pub struct MINOR_SYNC_POINT_SPDU {
    // TODO:
}

pub struct MINOR_SYNC_ACK_SPDU {
    // TODO:
}

pub struct MAJOR_SYNC_POINT_SPDU {
    // TODO:
}

pub struct MAJOR_SYNC_ACK_SPDU {
    // TODO:
}

pub struct RESYNCHRONIZE_SPDU {
    // TODO:
}

pub struct RESYNCHRONIZE_ACK_SPDU {
    // TODO:
}

pub struct PREPARE_SPDU {
    // TODO:
}

pub struct EXCEPTION_REPORT_SPDU {
    // TODO:
}

pub struct EXCEPTION_DATA_SPDU {
    // TODO:
}

pub struct ACTIVITY_START_SPDU {
    // TODO:
}

pub struct ACTIVITY_RESUME_SPDU {
    // TODO:
}

pub struct ACTIVITY_INTERRUPT_SPDU {
    // TODO:
}

pub struct ACTIVITY_INTERRUPT_ACK_SPDU {
    // TODO:
}

pub struct ACTIVITY_DISCARD_SPDU {
    // TODO:
}

pub struct ACTIVITY_DISCARD_ACK_SPDU {
    // TODO:
}

pub struct ACTIVITY_END_SPDU {
    // TODO:
}

pub struct ACTIVITY_END_ACK_SPDU {

}

pub trait OSIConnectionOrientedSessionService {
    // TODO:
}

pub trait OSIConnectionOrientedSessionEntity : OSIConnectionOrientedSessionService {

}

// TODO: ?
// pub struct SessionServicePDUParserState {
//     buffer: Buffer;
//     bufferIndex: number;
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResyncType {
    NO,
    A,
    R,
    S,
    DSC,
    INT,
}

pub enum SPDU {
    CN(CONNECT_SPDU),
    OA(OVERFLOW_ACCEPT_SPDU),
    CDO(CONNECT_DATA_OVERFLOW_SPDU),
    AC(ACCEPT_SPDU),
    RF(REFUSE_SPDU),
    FN(FINISH_SPDU),
    DN(DISCONNECT_SPDU),
    NF(NOT_FINISHED_SPDU),
    AB(ABORT_SPDU),
    AA(ABORT_ACCEPT_SPDU),
    DT(DATA_TRANSFER_SPDU),
    EX(EXCEPTION_DATA_SPDU),
    TD(TYPED_DATA_SPDU),
    CD(CAPABILITY_DATA_SPDU),
    CDA(CAPABILITY_DATA_ACK_SPDU),
    GT(GIVE_TOKENS_SPDU),
    PT(PLEASE_TOKENS_SPDU),
    GTC(GIVE_TOKENS_CONFIRM_SPDU),
    GTA(GIVE_TOKENS_ACK_SPDU),
    MIP(MINOR_SYNC_POINT_SPDU),
    MIA(MINOR_SYNC_ACK_SPDU),
    MAP(MAJOR_SYNC_POINT_SPDU),
    MAA(MAJOR_SYNC_ACK_SPDU),
    RS(RESYNCHRONIZE_SPDU),
    RA(RESYNCHRONIZE_ACK_SPDU),
    PR(PREPARE_SPDU),
    ER(EXCEPTION_REPORT_SPDU),
    ED(EXCEPTION_DATA_SPDU),
    AS(ACTIVITY_START_SPDU),
    AR(ACTIVITY_RESUME_SPDU),
    AI(ACTIVITY_INTERRUPT_SPDU),
    AIA(ACTIVITY_INTERRUPT_ACK_SPDU),
    AD(ACTIVITY_DISCARD_SPDU),
    ADA(ACTIVITY_DISCARD_ACK_SPDU),
    AE(ACTIVITY_END_SPDU),
    AEA(ACTIVITY_END_ACK_SPDU),
}

pub enum PartialSPDU {
    AC(ACCEPT_SPDU),
    RF(REFUSE_SPDU),
    FN(FINISH_SPDU),
    NF(NOT_FINISHED_SPDU),
    DN(DISCONNECT_SPDU),
    DT(DATA_TRANSFER_SPDU),
    AB(ABORT_SPDU),
}

pub struct X225SessionEntity {
    pub local_transport_ref_to_conn: HashMap<TransportRef, X225SessionConnection>,
}

#[derive(Debug, Clone)]
pub struct X225SessionConnection {

    /// The state of this connection
    pub state: X225ConnectionState,

    /// The version of the session protocol used.
    pub version: u8,

    /// Whether this side of the connection initiated the call.
    pub caller: bool,

    /// Whether the data token is owned locally. None if unowned / both.
    pub i_own_data_token: Option<bool>,

    /// Whether the release token is owned locally. None if unowned / both.
    pub i_own_release_token: Option<bool>,

    /// Whether the minor synchronization token is owned locally. None if unowned / both.
    pub i_own_sync_minor_token: Option<bool>,

    /// Whether the major activity token is owned locally. None if unowned / both.
    pub i_own_major_act_token: Option<bool>,

    /// The maximum size in bytes of a received TSDU.
    pub inbound_max_tsdu_size: usize,

    /// The maximum size in bytes of a transmitted TSDU.
    pub outbound_max_tsdu_size: usize,

    /// The local S-selector
    pub local_selector: Option<OsiSelector>,

    /// The remote S-selector
    pub remote_selector: Option<OsiSelector>,

    /// Not sure what this is, actually.
    pub timer_timeout_in_ms: Option<u32>,

    /// The sent or received CONNECT SPDU.
    pub cn: Option<CONNECT_SPDU>,

    /**
     * Detailed in [ITU Recommendation X.225 (1995)](https://www.itu.int/rec/T-REC-X.225/en),
     * Section 7.9.2. This timeout is used to control when the SPM "gives up"
     * waiting for an ABORT ACCEPT or a T-DISCONNECT.
     */
    // TODO:
    // TIM?: NodeJS.Timeout;

    /// Used for buffering connect data both inbound and outbound.
    pub connectData: BytesMut,

    /// Used for buffering user data
    pub userDataBuffer: BytesMut,

    /// The maximum size, in bytes, of a received or sent SSDU.
    pub max_ssdu_size: usize,

    /// This field exists because, according to
    /// [ITU Recommendation X.225 (1995)](https://www.itu.int/rec/T-REC-X.225/en),
    /// Section 7.37.1:
    ///
    /// > Where an SSDU is segmented, the first SPDU contains all the parameters
    /// > which would have been present in the SPDU if the SSDU had not been
    /// > segmented...
    ///
    /// To handle the above requirement, this implementation preserves the first
    /// received SPDU when the Enclosure Item parameter indicates that an SPDU
    /// is the first of multiple segments.
    ///
    pub in_progress_spdu: Option<PartialSPDU>,

    /// An 16-bit field for the supported functional units.
    pub FU: u16,

    /// If true, use of transport expedited service is selected for use on this
    /// session connection.
    pub TEXP: bool,

    /// If true, an activity is in progress.
    pub Vact: bool,

    /// The next value of Vact when a MAJOR SYNC ACK SPDU or an ACTIVITY END ACK
    /// SPDU is sent or received.
    pub Vnextact: bool,

    /// Indicates what kind of resynchronization is currently in progress.
    pub Vrsp: ResyncType,

    /// The serial number in case of resynchronize restart.
    pub Vrspnb: SerialNumber,

    /// The resync type of the sending flow.
    pub Vrsps: ResyncType,

    /// The resync type of the receiving flow.
    pub Vrspr: ResyncType,

    /// The serial number for the SPM's receiving flow in the case of
    /// resynchronize restart.
    pub Vrspnbr: SerialNumber,

    /// The serial number for the SPM's sending flow in the case of
    /// resynchronize restart.
    pub Vrspnbs: SerialNumber,

    /// Which SPM wins in a collision.
    pub SPMWinner: bool,

    /// If true, the SPM received the T-CONNECT indication; if false, the SPM
    /// initiated the T-CONNECT indication. In other words, if this is false,
    /// the local SPM created the transport connection.
    pub Vtca: bool,

    /// Whether the transport connection can be reused by the SPM for another
    /// session connection.
    pub Vtrr: bool,

    /// Whether there has been a collision of FINISH SPDUs.
    pub Vcoll: bool,

    /// Whether a DISCONNECT SPDU has been received in STA09 (collision of
    /// FINISH SPDUs).
    pub Vdnr: bool,

    /// The lowest serial number to which a sync point confirmation is expected.
    pub V_A: SerialNumber,

    /// The next serial number to be used.
    pub V_M: SerialNumber,

    /// The lowest serial number to which resynchronization restart is permitted.
    pub V_R: SerialNumber,

    /// Whether the SS-user has the right to issue minor sync point responses
    /// when V(A) is less than V(M).
    pub Vsc: bool,

    /// The highest sync point serial number which was sent in a MINOR
    /// SYNCHRONIZATION POINT SPDU with the data separation parameter set to
    /// true.
    pub Vado: SerialNumber,

    /// The highest sync point serial number which was received in a MINOR
    /// SYNCHRONIZATION POINT SPDU with the data separation parameter set to
    /// true.
    pub Vadi: SerialNumber,

    /// The lowest serial number on the SPM's sending data flow to which a
    /// sync point confirmation is expected to be received.
    pub VAs: SerialNumber,

    /// The lowest serial number on the SPM's receiving data flow for which a
    /// confirmation has not yet been sent.
    pub VAr: SerialNumber,

    /// The serial number of the next sync point to be sent.
    pub VMs: SerialNumber,

    /// The serial number of the next sync point to be received.
    pub VMr: SerialNumber,

    /// The lowest serial number on the SPM's sending data flow to which resync
    /// restart is permitted.
    pub VRs: SerialNumber,

    /// The lowest serial number on the SPM's receiving data flow to which
    /// resync restart is permitted.
    pub VRr: SerialNumber,

    /// Whether the receiving flow is in the process of resynchronization.
    pub Discard_rcv_flow: bool,

    /// Whether the sending flow is in the process of resynchronization.
    pub Discard_snd_flow: bool,
}

impl Default for X225SessionConnection {

    fn default() -> Self {
        let transport_caller: bool = true;
        X225SessionConnection {
            version: SESSION_PROTOCOL_VERSION_1,
            caller: transport_caller,
            state: X225ConnectionState::STA01,
            i_own_data_token: None,
            i_own_release_token: None,
            i_own_sync_minor_token: None,
            i_own_major_act_token: None,
            inbound_max_tsdu_size: usize::MAX, // Yes, this is the proper default.
            outbound_max_tsdu_size: usize::MAX, // Yes, this is the proper default.
            local_selector: None,
            remote_selector: None,
            timer_timeout_in_ms: None,
            cn: None,
            connectData: BytesMut::new(),
            userDataBuffer: BytesMut::new(),
            max_ssdu_size: usize::MAX, // This doesn't come from the protocol. It's just a config option.
            in_progress_spdu: None,
            FU: 0b0000_0011_0100_1001, // See X.225, Section 8.3.1.16.
            TEXP: false,
            Vact: false,
            Vnextact: false,
            Vrsp: ResyncType::NO,
            Vrspnb: 0,
            Vrsps: ResyncType::NO,
            Vrspr: ResyncType::NO,
            Vrspnbr: 0,
            Vrspnbs: 0,
            SPMWinner: false,
            Vtca: !transport_caller,
            Vtrr: false,
            Vcoll: false,
            Vdnr: false,
            V_A: 0,
            V_M: 0,
            V_R: 0,
            Vsc: false,
            Vado: 0,
            Vadi: 0,
            VAs: 0,
            VAr: 0,
            VMs: 0,
            VMr: 0,
            VRs: 0,
            VRr: 0,
            Discard_rcv_flow: false,
            Discard_snd_flow: false,
        }
    }

}
