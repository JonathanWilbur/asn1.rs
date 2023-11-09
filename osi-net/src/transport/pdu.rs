use crate::OsiSelector;
use std::time::SystemTime;
use crate::transport::{
    ParameterCode,
    TransportRef,
    UserData,
};
use bytes::Bytes;
use crate::transport::service::{
    MaxAndAverageThroughput,
    BidirectionalTransitDelay,
    ResidualErrorRate,
    SelectiveAcknowledgement,
};

// #region TPDU codes
// pub const TPDU_CODE_CR: u8 = 7;
// pub const TPDU_CODE_CC: u8 = 7;
pub const TPDU_CODE_DR: u8 = 0b1000_0000;
pub const TPDU_CODE_DC: u8 = 0b1100_0000;
// pub const TPDU_CODE_DT: u8 = 3; // So long as transport class is 0.
pub const TPDU_CODE_ER: u8 = 0b0111_0000;
pub const TPDU_CODE_EA: u8 = 0b0010_0000;
pub const TPDU_CODE_ED: u8 = 0b0001_0000;
// #endregion TPDU codes

pub const CLASS_OPTION_MASK: u8 = 0b1111_0000; // Apply this to byte 7 of the CR TPDU.
pub const CLASS_OPTION_0: u8 = 0b0000_0000;
pub const CLASS_OPTION_1: u8 = 0b0001_0000;
pub const CLASS_OPTION_2: u8 = 0b0010_0000;
pub const CLASS_OPTION_3: u8 = 0b0011_0000;
pub const CLASS_OPTION_4: u8 = 0b0100_0000;

// #region TPDU fixed header lengths
pub const CR_TPDU_FIXED_HEADER_LENGTH: u8 = 7;
pub const CC_TPDU_FIXED_HEADER_LENGTH: u8 = 7;
pub const DR_TPDU_FIXED_HEADER_LENGTH: u8 = 7;
pub const DC_TPDU_FIXED_HEADER_LENGTH: u8 = 6;
pub const DT_TPDU_FIXED_HEADER_LENGTH: u8 = 3; // So long as transport class is 0.
pub const ER_TPDU_FIXED_HEADER_LENGTH: u8 = 5;
// #endregion TPDU fixed header lengths

// #region TPDU parameter codes
pub const PC_CHECKSUM: u8 = 0b1100_0011;
pub const DR_TPDU_PC_ADDITIONAL_INFO: u8 = 0b1110_0000;
pub const PC_CALLING_TRANSPORT_SELECTOR: u8 = 0b1100_0001;
pub const PC_CALLED_TRANSPORT_SELECTOR: u8 = 0b1100_0010;
pub const PC_RESPONDING_TRANSPORT_SELECTOR: u8 = PC_CALLED_TRANSPORT_SELECTOR;
pub const PC_TPDU_SIZE: u8 = 0b1100_0000;
pub const PC_TPDU_PREF_MAX_TPDU_SIZE: u8 = 0b1111_0000;
// TODO: Remove _TPDU from these
pub const PC_TPDU_VERSION_NUMBER: u8 = 0b1100_0100;
pub const PC_TPDU_PROTECTION_PARAMETERS: u8 = 0b1100_0101;
pub const PC_TPDU_ADDITIONAL_OPTION_SELECTION: u8 = 0b1100_0110;
pub const PC_TPDU_ALT_PROTOCOL_CLASSES: u8 = 0b1100_0111;
pub const PC_TPDU_ACK_TIME: u8 = 0b1000_0101;
pub const PC_TPDU_THROUGHPUT: u8 = 0b1000_1001;
pub const PC_TPDU_RESIDUAL_ERROR_RATE: u8 = 0b1000_0110;
pub const PC_TPDU_PRIORITY: u8 = 0b1000_0111;
pub const PC_TPDU_TRANSIT_DELAY: u8 = 0b1000_1000;
pub const PC_TPDU_REASSIGNMENT_TIME: u8 = 0b1000_1011;
pub const PC_INACTIVITY_TIMER: u8 = 0b1111_0010;
pub const PC_SUBSEQUENCE_NUMBER: u8 = 0b1000_1010; // Length = 2
pub const PC_FLOW_CONTROL_CONFIRM: u8 = 0b1000_1100; // Length = 8
pub const PC_SELECTIVE_ACK_PARAMS: u8 = 0b1000_1111;
pub const PC_INVALID_TPDU: u8 = 0b1100_0001;
// #endregion TPDU parameter codes

// #region DR-TPDU reason codes
pub const DR_REASON_NORMAL_DISCONNECT: u8 = 128 + 0;
pub const DR_REASON_REMOTE_CONGESTION: u8 = 128 + 1;
pub const DR_REASON_NEGOTIATION_FAILED: u8 = 128 + 2;
pub const DR_REASON_DUPLICATE_SRC_REF: u8 = 128 + 3;
pub const DR_REASON_MISMATCHED_REFS: u8 = 128 + 4;
pub const DR_REASON_PROTOCOL_ERROR: u8 = 128 + 5;
pub const DR_REASON_REFERENCE_OVERFLOW: u8 = 128 + 7;
pub const DR_REASON_CR_REFUSED: u8 = 128 + 8;
pub const DR_REASON_HEADER_OR_PARAMETER_LENGTH_INVALID: u8 = 128 + 10;
pub const DR_REASON_NOT_SPECIFIED: u8 = 0;
pub const DR_REASON_CONGESTION_AT_TSAP: u8 = 1;
pub const DR_REASON_SESSION_ENTITY_NOT_ATTACHED_TO_TSAP: u8 = 2;
pub const DR_REASON_ADDRESS_UNKNOWN: u8 = 3;
// #endregion

// #region ER-TPDU reject cause codes
pub const ER_REJECT_CAUSE_NOT_SPECIFIED: u8 = 0b0000_0000;
pub const ER_REJECT_CAUSE_INVALID_PARAMETER_CODE: u8 = 0b0000_0001;
pub const ER_REJECT_CAUSE_INVALID_TPDU_TYPE: u8 = 0b0000_0010;
pub const ER_REJECT_CAUSE_INVALID_PARAMETER_VALUE: u8 = 0b0000_0011;
// #endregion ER-TPDU reject cause codes

// #region Return codes
// pub const RETURN_OK: ReturnCode = 0;
// pub const TPDU_PARSE_TRUNCATED: ReturnCode = 1;
// pub const TPDU_PARSE_UNRECOGNIZED_PARAMETER: ReturnCode = -1;
// pub const TPDU_PARSE_RESERVED_LI: ReturnCode = -2;
// pub const TPDU_PARSE_WRONG_PARSER: ReturnCode = -3;
// pub const TPDU_PARSE_WRONG_PARAM_LENGTH: ReturnCode = -4;
// pub const TPDU_PARSE_DUPLICATE_PARAMETER: ReturnCode = -5;
// pub const TPDU_PARSE_MALFORMED_PARAMETER: ReturnCode = -6;
// pub const DISPATCH_NSDU_RETURN_TOO_SHORT: ReturnCode = -7;
// pub const DISPATCH_NSDU_RETURN_INVALID_CONCAT: ReturnCode = -8;
// pub const DISPATCH_NSDU_RETURN_UNRECOGNIZED_TPDU: ReturnCode = -9;
// #endregion Return codes

// #region TPDU validation return codes
// pub const TPDU_VALIDATION_RC_OK: u8 = 0;
// // Positive numbers are for errors are for negotiation failures.
// pub const TPDU_VALIDATION_CR_UNACCEPTABLE: u8 = 1;
// pub const TPDU_VALIDATION_CR_CANNOT_MEET_CLASS_DEMANDED: u8 = 2;
// pub const TPDU_VALIDATION_SELECTOR_NOT_RECOGNIZED: u8 = 3;
// // Negative numbers are for protocol errors / malformed PDUs or fields.
// pub const TPDU_VALIDATION_CR_UNRECOGNIZED_VERSION: u8 = -1;
// pub const TPDU_VALIDATION_CR_UNRECOGNIZED_CLASS: u8 = -2;
// pub const TPDU_VALIDATION_CR_DST_REF_NOT_ZEROED: u8 = -3;
// pub const TPDU_MALFORMED: u8 = -4;
// #endregion TPDU validation return codes

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub code: ParameterCode,
    pub value: Bytes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CR_TPDU <'a> {
    pub cdt: u8,
    pub dst_ref: TransportRef,
    pub src_ref: TransportRef,
    pub class_option: u8,
    pub calling_transport_selector: Option<&'a [u8]>,
    pub tpdu_size: Option<usize>,
    pub preferred_max_tpdu_size: Option<usize>,
    pub version_number: u8, // defaults to 1
    pub protection_parameters: Option<&'a [u8]>, // The value of this field is user-defined.
    pub checksum: Option<u16>,
    pub additional_option_selection: u8,
    pub alternative_protocol_classes: u8, // A bit mask.
    pub acknowledgement_time: Option<u16>,
    pub throughput: Option<MaxAndAverageThroughput>,
    pub residual_error_rate: Option<ResidualErrorRate>,
    pub priority: Option<u16>, // 0 is the highest priority.
    pub transit_delay: Option<BidirectionalTransitDelay>,
    pub reassignment_time: Option<u16>, // TTR in seconds.
    pub inactivity_timer: Option<u32>, // Expressed in milliseconds.
    pub called_or_responding_transport_selector: Option<&'a [u8]>,
    pub user_data: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DR_TPDU <'a> {
    pub dst_ref: TransportRef,
    pub src_ref: TransportRef,
    pub reason: u8,
    pub additional_info: Option<&'a [u8]>,
    pub user_data: &'a [u8],
    pub checksum: Option<u16>,
}

impl <'a> DR_TPDU<'a> {

    fn checksum_param(&self) -> Vec<u8> {
        if let Some(checksum) = self.checksum {
            let cs = checksum.to_be_bytes();
            vec![ PC_CHECKSUM, 2, cs[0], cs[1] ]
        } else {
            vec![]
        }
    }

    fn addl_info_param(&self) -> Vec<u8> {
        if let Some(addinfo) = self.additional_info {
            [
                [ PC_CHECKSUM, 2 ].as_slice(),
                addinfo.to_vec().as_slice(),
            ].concat()
        } else {
            vec![]
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        // TODO: What to do if length overflows?
        let len = 7
            + self.user_data.len()
            + if self.checksum.is_some() { 4 } else { 0 }
            + self.additional_info.map(|a| a.len() + 2).unwrap_or(0);
        [
            [ len as u8, TPDU_CODE_DR ].as_slice(),
            self.dst_ref.to_be_bytes().as_slice(),
            self.src_ref.to_be_bytes().as_slice(),
            [ self.reason ].as_slice(),
            self.user_data,
            self.checksum_param().as_slice(),
            self.addl_info_param().as_slice(),
        ].concat()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DC_TPDU {
    pub dst_ref: TransportRef,
    pub src_ref: TransportRef,
    pub checksum: Option<u16>,
}

impl DC_TPDU {

    fn checksum_param(&self) -> Vec<u8> {
        if let Some(checksum) = self.checksum {
            let cs = checksum.to_be_bytes();
            vec![ PC_CHECKSUM, 2, cs[0], cs[1] ]
        } else {
            vec![]
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let len = DC_TPDU_FIXED_HEADER_LENGTH
            + if self.checksum.is_some() { 4 } else { 0 };
        [
            [ len as u8, TPDU_CODE_DC ].as_slice(),
            self.dst_ref.to_be_bytes().as_slice(),
            self.src_ref.to_be_bytes().as_slice(),
            self.checksum_param().as_slice(),
        ].concat()
    }

}

// TODO: Make this an alias to CR-TPDU?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CC_TPDU <'a> {
    // NOTE: These fields were copied from CR_TPDU.
    pub cdt: u8,
    pub dst_ref: TransportRef,
    pub src_ref: TransportRef,
    pub class_option: u8,
    pub calling_transport_selector: Option<&'a [u8]>,
    pub tpdu_size: Option<usize>,
    pub preferred_max_tpdu_size: Option<usize>,
    pub version_number: u8, // Default is 1
    pub protection_parameters: Option<&'a [u8]>, // The value of this field is user-defined.
    pub checksum: Option<u16>,
    pub additional_option_selection: u8,
    pub alternative_protocol_classes: u8, // A bit mask.
    pub acknowledgement_time: Option<u16>,
    pub throughput: Option<MaxAndAverageThroughput>,
    pub residual_error_rate: Option<ResidualErrorRate>,
    pub priority: Option<u16>, // 0 is the highest priority.
    pub transit_delay: Option<BidirectionalTransitDelay>,
    pub reassignment_time: Option<u16>, // TTR in seconds.
    pub inactivity_timer: Option<u32>, // Expressed in milliseconds.
    pub called_or_responding_transport_selector: Option<&'a [u8]>,
    pub user_data: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ER_TPDU <'a> {
    pub dst_ref: TransportRef,
    pub reject_cause: u8,
    pub invalid_tpdu: Option<&'a [u8]>,
    pub checksum: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DT_TPDU <'a> {
    pub roa: bool,
    pub dst_ref: Option<TransportRef>,
    pub eot: bool,
    pub nr: u32,
    pub user_data: &'a [u8],
    pub checksum: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ED_TPDU <'a> {
    pub dst_ref: TransportRef,
    pub eot: bool,
    pub nr: u32,
    pub user_data: &'a [u8],
    pub checksum: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowControlConfirmation {
    pub lower_window_edge: u32,
    pub your_subsequence: u16,
    pub your_credit: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AK_TPDU {
    pub dst_ref: TransportRef,
    pub cdt: u16,
    pub nr: u32,
    pub checksum: Option<u16>,
    pub subsequence_number: Option<u16>,
    pub flow_control_confirmation: Option<FlowControlConfirmation>,
    pub selective_acknowledgement_parameters: Option<Vec<SelectiveAcknowledgement>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EA_TPDU {
    pub dst_ref: TransportRef,
    pub nr: u32,
    pub checksum: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RJ_TPDU {
    pub dst_ref: TransportRef,
    pub cdt: u16,
    pub yr_tu_nr: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TPDU <'a> {
    CR(CR_TPDU<'a>),
    CC(CC_TPDU<'a>),
    DR(DR_TPDU<'a>),
    DC(DC_TPDU),
    DT(DT_TPDU<'a>),
    ED(ED_TPDU<'a>),
    AK(AK_TPDU),
    EA(EA_TPDU),
    RJ(RJ_TPDU),
    ER(ER_TPDU<'a>),
}

impl <'a> TPDU <'a> {

    pub fn tpdu_type (&self) -> TPDUType {
        match self {
            TPDU::CR(cr) => TPDUType::CR,
            TPDU::CC(cc) => TPDUType::CC,
            TPDU::DR(dr) => TPDUType::DR,
            TPDU::DC(dc) => TPDUType::DC,
            TPDU::DT(dt) => TPDUType::DT,
            TPDU::ED(ed) => TPDUType::ED,
            TPDU::AK(ak) => TPDUType::AK,
            TPDU::EA(ea) => TPDUType::EA,
            TPDU::RJ(rj) => TPDUType::RJ,
            TPDU::ER(er) => TPDUType::ER,
        }
    }

    pub fn src_ref (&self) -> Option<TransportRef> {
        match self {
            TPDU::CR(cr) => Some(cr.src_ref),
            TPDU::CC(cc) => Some(cc.src_ref),
            TPDU::DR(dr) => Some(dr.src_ref),
            TPDU::DC(dc) => Some(dc.src_ref),
            TPDU::DT(dt) => None,
            TPDU::ED(ed) => None,
            TPDU::AK(ak) => None,
            TPDU::EA(ea) => None,
            TPDU::RJ(rj) => None,
            TPDU::ER(er) => None,
        }
    }

    pub fn dst_ref (&self) -> Option<TransportRef> {
        match self {
            TPDU::CR(cr) => Some(cr.dst_ref),
            TPDU::CC(cc) => Some(cc.dst_ref),
            TPDU::DR(dr) => Some(dr.dst_ref),
            TPDU::DC(dc) => Some(dc.dst_ref),
            TPDU::DT(dt) => dt.dst_ref,
            TPDU::ED(ed) => Some(ed.dst_ref),
            TPDU::AK(ak) => Some(ak.dst_ref),
            TPDU::EA(ea) => Some(ea.dst_ref),
            TPDU::RJ(rj) => Some(rj.dst_ref),
            TPDU::ER(er) => Some(er.dst_ref),
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TPDUType {
    CR,
    CC,
    DR,
    DC,
    DT,
    ED,
    AK,
    EA,
    RJ,
    ER,
}
