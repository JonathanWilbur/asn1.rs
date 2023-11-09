use nom::number::complete::be_u16;
use nom::{self, IResult, number::complete::{be_u8, be_u32}, bytes::complete::take};
use crate::transport::pdu::{
    TPDU_CODE_DR,
    TPDU_CODE_DC,
    TPDU_CODE_ER,
    TPDU_CODE_ED,
    TPDU_CODE_EA,
    FlowControlConfirmation,
};
use crate::transport::service::{
    MaxAndAverageThroughput,
    BidirectionalThroughput,
    Throughput,
    ResidualErrorRate,
    BidirectionalTransitDelay,
    TransitDelay,
    SelectiveAcknowledgement,
};
use super::{
    TPDU, DT_TPDU, TransportRef, CR_TPDU, CC_TPDU, DR_TPDU, DC_TPDU, RJ_TPDU, AK_TPDU, EA_TPDU, ER_TPDU, ED_TPDU,
    PC_CHECKSUM,
    DR_TPDU_PC_ADDITIONAL_INFO,
    PC_CALLING_TRANSPORT_SELECTOR,
    PC_CALLED_TRANSPORT_SELECTOR,
    PC_RESPONDING_TRANSPORT_SELECTOR,
    PC_TPDU_SIZE,
    PC_TPDU_PREF_MAX_TPDU_SIZE,
    PC_TPDU_VERSION_NUMBER,
    PC_TPDU_PROTECTION_PARAMETERS,
    PC_TPDU_ADDITIONAL_OPTION_SELECTION,
    PC_TPDU_ALT_PROTOCOL_CLASSES,
    PC_TPDU_ACK_TIME,
    PC_TPDU_THROUGHPUT,
    PC_TPDU_RESIDUAL_ERROR_RATE,
    PC_TPDU_PRIORITY,
    PC_TPDU_TRANSIT_DELAY,
    PC_TPDU_REASSIGNMENT_TIME,
    PC_INACTIVITY_TIMER,
    PC_SUBSEQUENCE_NUMBER,
    PC_FLOW_CONTROL_CONFIRM,
    PC_SELECTIVE_ACK_PARAMS,
    PC_INVALID_TPDU,
};
use nom::error::Error as NomError;
use nom::error::ErrorKind as NomErrorKind;
use nom::Err as NomErr;
use crate::OsiSelector;
use std::error::Error;
use std::fmt::Display;

fn tpdu_size_to_code (size: usize) -> Option<u8> {
    match size {
        8192 => Some(0b0000_1101),
        4096 => Some(0b0000_1100),
        2048 => Some(0b0000_1011),
        1024 => Some(0b0000_1010),
        512 => Some(0b0000_1001),
        256 => Some(0b0000_1000),
        128 => Some(0b0000_0111),
        _ => None,
    }
}

fn code_to_tpdu_size (code: u8) -> Option<usize> {
    match code {
        0b0000_1101 => Some(8192),
        0b0000_1100 => Some(4096),
        0b0000_1011 => Some(2048),
        0b0000_1010 => Some(1024),
        0b0000_1001 => Some(512),
        0b0000_1000 => Some(256),
        0b0000_0111 => Some(128),
        _ => None,
    }
}

pub struct X224Parameter <'a> {
    pub code: u8,
    pub value: &'a [u8],
}

pub fn parse_x224_parameter <'a>(b: &'a [u8]) -> IResult<&'a [u8], X224Parameter<'a>> {
    let (b, code) = be_u8(b)?;
    let (b, len) = be_u8(b)?;
    let (b, value) = take(len)(b)?;
    let ret = X224Parameter{
        code,
        value,
    };
    Ok((b, ret))
}

// Actually, it is better to avoid an array allocation and implement this as
// an iterator.
/// `var_slice` should be slice of variable part only.
// pub fn parse_x224_variable_part(var_slice: &[u8]) -> IResult<&[u8], Vec<X224Parameter>> {
//     let b = var_slice;
//     if b.len() == 0 {
//         return Ok((var_slice, vec![]));
//     }
//     // The number of parameters expected is b.len() / 4. Each parameter takes
//     // up at least two bytes for the code and length, and the parameter value
//     // probably averages to around two bytes per parameter type. That means we
//     // can guess that there will be about one parameter per every four bytes of
//     // variable part data.
//     let predicted_params = b.len() >> 2;
//     let mut ret = Vec::with_capacity(predicted_params); // b.len() / 4 is
// }

#[derive(Debug, PartialEq)]
pub enum X224TpduParseErrorKind {
    Empty,
    ReservedLengthIndicator, // LI = 255 was used.
    ZeroLength, // LI = 0 was used.
    UnrecognizedTpduType(u8),
    MalformedFixedPart,
    CrDstRefNotZero,
    SrcRefZero,
    TruncatedVariablePart,
    MalformedVariablePart,
    UnrecognizedParameter(u8),
    WrongSizedParameter(u8), // u8 = parameter code
    MalformedParameter(u8), // u8 = parameter code
    InvalidChecksum,
}

impl Display for X224TpduParseErrorKind {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X224TpduParseErrorKind::Empty => f.write_str("LI=?"),
            X224TpduParseErrorKind::ReservedLengthIndicator => f.write_str("LI=0"),
            X224TpduParseErrorKind::ZeroLength => f.write_str("LI=255"),
            X224TpduParseErrorKind::CrDstRefNotZero => f.write_str("CR.DST-REF != 0"),
            X224TpduParseErrorKind::SrcRefZero => f.write_str("SRC-REF=0"),
            X224TpduParseErrorKind::UnrecognizedTpduType(tt) => f.write_fmt(format_args!("unrecognized tpdu type {:#02x}", tt)),
            X224TpduParseErrorKind::MalformedFixedPart => f.write_str("malformed fixed part"),
            X224TpduParseErrorKind::MalformedVariablePart => f.write_str("malformed variable part"),
            X224TpduParseErrorKind::UnrecognizedParameter(pc) => f.write_fmt(format_args!("unrecognized parameter {:#02x}", pc)),
            X224TpduParseErrorKind::WrongSizedParameter(pc) => f.write_fmt(format_args!("wrong-sized parameter {:#02x}", pc)),
            X224TpduParseErrorKind::MalformedParameter(pc) => f.write_fmt(format_args!("malformed parameter {:#02x}", pc)),
            X224TpduParseErrorKind::InvalidChecksum => f.write_str("invalid checksum"),
            X224TpduParseErrorKind::TruncatedVariablePart => f.write_str("truncated variable part"),
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct X224TpduParseError {
    pub kind: X224TpduParseErrorKind,
    pub tpdu_code: Option<u8>,
    pub src_ref: Option<TransportRef>,
    pub dst_ref: Option<TransportRef>,
}

impl X224TpduParseError {

    pub const fn new (
        kind: X224TpduParseErrorKind,
        tpdu_code: Option<u8>,
        src_ref: Option<TransportRef>,
        dst_ref: Option<TransportRef>,
    ) -> Self {
        X224TpduParseError {
            kind,
            tpdu_code,
            src_ref,
            dst_ref,
        }
    }

}

impl From<X224TpduParseErrorKind> for X224TpduParseError {

    fn from(value: X224TpduParseErrorKind) -> Self {
        X224TpduParseError {
            kind: value,
            tpdu_code: None,
            src_ref: None,
            dst_ref: None,
        }
    }

}

impl Display for X224TpduParseError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid X.224 TPDU")?;
        if let Some(src_ref) = self.src_ref {
            f.write_fmt(format_args!("(SRC-REF={:#04x}", src_ref))?;
        }
        if let Some(dst_ref) = self.dst_ref {
            if self.src_ref.is_none() {
                f.write_str("(")?;
            } else {
                f.write_str(" ")?;
            }
            f.write_fmt(format_args!("DST-REF={:#04x}): ", dst_ref))?;
        } else {
            f.write_str("): ")?;
        }
        self.kind.fmt(f)
    }

}

impl Error for X224TpduParseError {}

// impl <'a> From<NomError<&'a [u8]>> for X224TpduParseError {

//     fn from(value: NomError<&'a [u8]>) -> Self {
//         X224TpduParseError {

//         }
//     }

// }

/// TODO: An entire NSDU must be parsed at a time, because there are some TPDUs
/// that are allowed to be concatenated into an NSDU and there are some that
/// are not allowed.
pub fn parse_x224_tpdu <'a> (complete_nsdu: &'a [u8], class: u8, ext_format: bool) -> IResult<&[u8], TPDU<'a>, X224TpduParseError> {
    let b = complete_nsdu;
    let (b, li) = be_u8(b)
        .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::Empty)))?;
    if li == 0 {
        return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::ZeroLength)));
    }
    if li == 0xFF {
        return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::ReservedLengthIndicator)));
    }
    let (mut b, tpdu_type) = be_u8(b)
        .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
    let mut is_dt: bool = false;
    let mut is_cr: bool = false;
    let mut is_cc: bool = false;
    let mut is_ak: bool = false;
    let mut is_rj: bool = false;
    let mut roa: bool = false;
    let nr: u32;
    let mut cdt: u8 = 0;
    let eot: bool;
    if tpdu_type == 0b1111_0000 { // DT with ROA=0
        is_dt = true;
    }
    if tpdu_type == 0b1111_0001 { // DT with ROA=1
        is_dt = true;
        roa = true;
    }
    if (tpdu_type & 0b1111_0000) == 0b1110_0000 {
        is_cr = true;
        cdt = tpdu_type & 0b0000_1111;
    }
    if (tpdu_type & 0b1111_0000) == 0b1101_0000 {
        is_cc = true;
        cdt = tpdu_type & 0b0000_1111;
    }
    if (tpdu_type & 0b1111_0000) == 0b0110_0000 {
        is_ak = true;
        cdt = tpdu_type & 0b0000_1111;
    }
    if (tpdu_type & 0b1111_0000) == 0b0101_0000 {
        is_rj = true;
        cdt = tpdu_type & 0b0000_1111;
    }
    let mut calling_transport_selector: Option<&[u8]> = None;
    let mut called_transport_selector: Option<&[u8]> = None;
    let mut responding_transport_selector: Option<&[u8]> = None;
    let mut protection_parameters: Option<&[u8]> = None;
    let mut checksum: Option<u16> = None;
    let mut additional_info: Option<&[u8]> = None;
    let mut tpdu_size: Option<usize> = None;
    let mut preferred_max_tpdu_size: Option<usize> = None;
    let mut version_number: u8 = 1;
    let mut additional_option_selection: u8 = 0b0000_0001;
    let mut alternative_protocol_classes: u8 = 0;
    let mut acknowledgement_time: Option<u16> = None;
    let mut throughput: Option<MaxAndAverageThroughput> = None;
    let mut residual_error_rate: Option<ResidualErrorRate> = None;
    let mut priority: Option<u16> = None;
    let mut transit_delay: Option<BidirectionalTransitDelay> = None;
    let mut reassignment_time: Option<u16> = None;
    let mut inactivity_timer: Option<u32> = None;
    let mut subsequence_number: Option<u16> = None;
    let mut flow_control_confirmation: Option<FlowControlConfirmation> = None;
    let mut selective_acknowledgement_parameters: Option<Vec<SelectiveAcknowledgement>> = None;
    let mut invalid_tpdu: Option<&[u8]> = None;
    let mut handle_param = |p: X224Parameter<'a>| -> IResult<(), (), NomError<&'a [u8]>> {
        match p.code {
            PC_CHECKSUM => {
                if p.value.len() != 2 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                let c = u16::from_be_bytes([ p.value[0], p.value[1] ]);
                checksum = Some(c);
            },
            DR_TPDU_PC_ADDITIONAL_INFO => {
                additional_info = Some(p.value);
            },
            PC_CALLING_TRANSPORT_SELECTOR => {
                calling_transport_selector = Some(p.value);
            },
            PC_CALLED_TRANSPORT_SELECTOR => {
                called_transport_selector = Some(p.value);
            },
            PC_RESPONDING_TRANSPORT_SELECTOR => {
                responding_transport_selector = Some(p.value);
            },
            PC_TPDU_SIZE => {
                if p.value.len() != 1 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                let ts = code_to_tpdu_size(p.value[0])
                    .ok_or(NomErr::Error(NomError::new(b, NomErrorKind::Verify)))?;
                tpdu_size = Some(ts);
            },
            PC_TPDU_PREF_MAX_TPDU_SIZE => {
                if p.value.len() == 0 || p.value.len() > 4 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                match p.value.len() {
                    1 => preferred_max_tpdu_size = Some((p.value[0] as usize) << 7),
                    2 => preferred_max_tpdu_size = {
                        let ts = u16::from_be_bytes([ p.value[0], p.value[1] ]);
                        Some((ts as usize) << 7)
                    },
                    3 => preferred_max_tpdu_size = {
                        let ts = u32::from_be_bytes([ 0, p.value[0], p.value[1], p.value[2] ]);
                        Some((ts as usize) << 7)
                    },
                    4 => preferred_max_tpdu_size = {
                        let ts = u32::from_be_bytes([ p.value[0], p.value[1], p.value[2], p.value[3] ]);
                        Some((ts as usize) << 7)
                    },
                    _ => panic!(),
                };
            },
            PC_TPDU_VERSION_NUMBER => {
                if p.value.len() != 1 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                version_number = p.value[0];
            },
            PC_TPDU_PROTECTION_PARAMETERS => {
                protection_parameters = Some(p.value);
            },
            PC_TPDU_ADDITIONAL_OPTION_SELECTION => {
                if p.value.len() != 1 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                additional_option_selection = p.value[0];
            },
            PC_TPDU_ALT_PROTOCOL_CLASSES => {
                alternative_protocol_classes = 0;
                for parambyte in p.value {
                    match parambyte {
                        0b0000_0000 => alternative_protocol_classes |= 0b0000_0001,
                        0b0001_0000 => alternative_protocol_classes |= 0b0000_0010,
                        0b0010_0000 => alternative_protocol_classes |= 0b0000_0100,
                        0b0011_0000 => alternative_protocol_classes |= 0b0000_1000,
                        0b0100_0000 => alternative_protocol_classes |= 0b0001_0000,
                        _ => return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify))),
                    };
                }
            },
            PC_TPDU_ACK_TIME => {
                if p.value.len() != 2 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                acknowledgement_time = Some(u16::from_be_bytes([ p.value[0], p.value[1] ]));
            },
            PC_TPDU_THROUGHPUT => {
                if p.value.len() != 12 && p.value.len() != 24 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                let max = BidirectionalThroughput{
                    calling_to_called: Throughput{
                        target: u32::from_be_bytes([ 0, p.value[0], p.value[1], p.value[2] ]),
                        min_acceptable: u32::from_be_bytes([ 0, p.value[3], p.value[4], p.value[5] ]),
                    },
                    called_to_calling: Throughput{
                        target: u32::from_be_bytes([ 0, p.value[6], p.value[7], p.value[8] ]),
                        min_acceptable: u32::from_be_bytes([ 0, p.value[9], p.value[10], p.value[11] ]),
                    },
                };
                let average: Option<BidirectionalThroughput>;
                if p.value.len() == 24 {
                    average = Some(BidirectionalThroughput{
                        calling_to_called: Throughput{
                            target: u32::from_be_bytes([ 0, p.value[12], p.value[13], p.value[14] ]),
                            min_acceptable: u32::from_be_bytes([ 0, p.value[15], p.value[16], p.value[17] ]),
                        },
                        called_to_calling: Throughput{
                            target: u32::from_be_bytes([ 0, p.value[18], p.value[19], p.value[20] ]),
                            min_acceptable: u32::from_be_bytes([ 0, p.value[21], p.value[22], p.value[23] ]),
                        },
                    });
                } else {
                    average = None;
                }
                throughput = Some(MaxAndAverageThroughput{ max, average });
            },
            PC_TPDU_RESIDUAL_ERROR_RATE => {
                if p.value.len() != 3 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                let target = 10usize.pow(p.value[0] as u32);
                let minimum_acceptable = 10usize.pow(p.value[1] as u32);
                let tsdu_size_of_interest = 2usize.pow(p.value[2] as u32);
                residual_error_rate = Some(ResidualErrorRate {
                    target,
                    minimum_acceptable,
                    tsdu_size_of_interest,
                });
            },
            PC_TPDU_PRIORITY => {
                if p.value.len() != 2 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                priority = Some(u16::from_be_bytes([ p.value[0], p.value[1] ]));
            },
            PC_TPDU_TRANSIT_DELAY => {
                if p.value.len() != 8 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                let calling_to_called = TransitDelay {
                    target: u16::from_be_bytes([ p.value[0], p.value[1] ]),
                    max_acceptable: u16::from_be_bytes([ p.value[2], p.value[3] ]),
                };
                let called_to_calling = TransitDelay {
                    target: u16::from_be_bytes([ p.value[4], p.value[5] ]),
                    max_acceptable: u16::from_be_bytes([ p.value[6], p.value[7] ]),
                };
                transit_delay = Some(BidirectionalTransitDelay{
                    called_to_calling,
                    calling_to_called,
                });
            },
            PC_TPDU_REASSIGNMENT_TIME => {
                if p.value.len() != 2 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                reassignment_time = Some(u16::from_be_bytes([ p.value[0], p.value[1] ]));
            },
            PC_INACTIVITY_TIMER => {
                if p.value.len() != 4 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                inactivity_timer = Some(u32::from_be_bytes([ p.value[0], p.value[1], p.value[2], p.value[3] ]));
            },
            PC_SUBSEQUENCE_NUMBER => {
                if p.value.len() != 2 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                subsequence_number = Some(u16::from_be_bytes([ p.value[0], p.value[1] ]));
            },
            PC_FLOW_CONTROL_CONFIRM => {
                if p.value.len() != 8 {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
                let lower_window_edge: u32 = u32::from_be_bytes([ p.value[0], p.value[1], p.value[2], p.value[3] ]);
                let your_subsequence: u16 = u16::from_be_bytes([ p.value[4], p.value[5] ]);
                let your_credit: u16 = u16::from_be_bytes([ p.value[6], p.value[7] ]);
                flow_control_confirmation = Some(FlowControlConfirmation{
                    lower_window_edge,
                    your_subsequence,
                    your_credit,
                });
            },
            PC_SELECTIVE_ACK_PARAMS => {
                if ext_format {
                    if p.value.len() % 8 > 0 {
                        return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                    }
                    let mut i = 0;
                    let mut selacks: Vec<SelectiveAcknowledgement> = Vec::with_capacity(p.value.len() >> 3);
                    while i < p.value.len() {
                        selacks.push(SelectiveAcknowledgement{
                            lower_edge: u32::from_be_bytes([ p.value[0], p.value[1], p.value[2], p.value[3] ]),
                            upper_edge: u32::from_be_bytes([ p.value[4], p.value[5], p.value[6], p.value[7] ]),
                        });
                        i += 8;
                    }
                    selective_acknowledgement_parameters = Some(selacks);
                } else {
                    if p.value.len() % 2 > 0 {
                        return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                    }
                    let mut i = 0;
                    let mut selacks: Vec<SelectiveAcknowledgement> = Vec::with_capacity(p.value.len() >> 1);
                    while i < p.value.len() {
                        selacks.push(SelectiveAcknowledgement{
                            lower_edge: u32::from_be_bytes([ 0, 0, 0, p.value[0] ]),
                            upper_edge: u32::from_be_bytes([ 0, 0, 0, p.value[1] ]),
                        });
                        i += 2;
                    }
                    selective_acknowledgement_parameters = Some(selacks);
                }
            },
            PC_INVALID_TPDU => {
                invalid_tpdu = Some(p.value);
            },
            _ => {
                /* From ITU-T Recommendation X.224 (1995), Section 13.2.3:
                "A parameter not defined in this Recommendation | International
                Standard shall be treated as a protocol error in any received
                TPDU except a CR-TPDU; in a CR-TPDU it shall be ignored." */
                if !is_cr {
                    return Err(NomErr::Error(NomError::new(b, NomErrorKind::Verify)));
                }
            },
        };
        Ok(((), ()))
    };
    let mut handle_var_part = |li_and_fixed_part_len: usize| -> IResult<(), (), X224TpduParseError> {
        let var_len = li as usize - (li_and_fixed_part_len - 1);
        let end_of_var_part = li_and_fixed_part_len + var_len;
        if complete_nsdu.len() < end_of_var_part {
            return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::TruncatedVariablePart)));
        }
        let mut var_part: &'a [u8] = &complete_nsdu[li_and_fixed_part_len..end_of_var_part];
        while var_part.len() > 0 {
            // I was going to check for duplicates here, but X.224 actually says
            // that, if duplicate parameters are provided, the last one is used.
            let (v, param) = parse_x224_parameter(var_part)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedVariablePart)))?;
            let code = param.code;
            handle_param(param)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedParameter(code))))?;
            var_part = v;
        }
        Ok(((), ()))
    };
    // DT TPDUs are guessed first as an optimization, since they are likely to
    // be the most frequent.
    if is_dt {
        let mut dst_ref: Option<TransportRef> = None;
        let b = {
            if class > 1 {
                let (b1, d) = be_u16(b)
                    .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
                dst_ref = Some(d);
                b1
            } else {
                b
            }
        };
        let b = if ext_format && class > 1 {
            let (b1, nr_and_eot) = be_u32(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            eot = (nr_and_eot & 0b10000000_00000000_00000000_00000000) > 0;
            nr  = nr_and_eot & 0b01111111_11111111_11111111_11111111;
            b1
        } else {
            let (b1, nr_and_eot) = be_u8(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            eot = (nr_and_eot & 0b1000_0000) > 0;
            nr  = (nr_and_eot & 0b01111111).into();
            b1
        };
        handle_var_part(complete_nsdu.len() - b.len())?;
        let user_data = &complete_nsdu[1+li as usize..];
        let dt = DT_TPDU {
            roa,
            eot,
            nr,
            dst_ref,
            checksum,
            user_data,
        };
        return Ok((&b[b.len()..], TPDU::DT(dt)));
    }
    if is_cr {
        let (b, dst_ref) = be_u16(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        if dst_ref != 0 {
            return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::CrDstRefNotZero)));
        }
        let (b, src_ref) = be_u16(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        if src_ref == 0 {
            // TODO: Move this error to other PDUs.
            return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::SrcRefZero)));
        }
        let (b, class_option) = be_u8(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        handle_var_part(complete_nsdu.len() - b.len())?;
        let user_data = &complete_nsdu[1+li as usize..];
        let cr = CR_TPDU {
            cdt,
            dst_ref,
            src_ref,
            class_option,
            calling_transport_selector,
            tpdu_size,
            preferred_max_tpdu_size,
            version_number,
            protection_parameters,
            checksum,
            additional_option_selection,
            alternative_protocol_classes,
            acknowledgement_time,
            throughput,
            residual_error_rate,
            priority,
            transit_delay,
            reassignment_time,
            inactivity_timer,
            called_or_responding_transport_selector: called_transport_selector,
            user_data,
        };
        let b = &complete_nsdu[complete_nsdu.len()..];
        debug_assert_eq!(b.len(), 0);
        return Ok((b, TPDU::CR(cr)));
    }
    if is_cc {
        let (b, dst_ref) = be_u16(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        let (b, src_ref) = be_u16(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        let (b, class_option) = be_u8(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        handle_var_part(complete_nsdu.len() - b.len())?;
        let user_data = &complete_nsdu[1+li as usize..];
        let cc = CC_TPDU {
            cdt,
            dst_ref,
            src_ref,
            class_option,
            calling_transport_selector,
            tpdu_size,
            preferred_max_tpdu_size,
            version_number,
            protection_parameters,
            checksum,
            additional_option_selection,
            alternative_protocol_classes,
            acknowledgement_time,
            throughput,
            residual_error_rate,
            priority,
            transit_delay,
            reassignment_time,
            inactivity_timer,
            called_or_responding_transport_selector: responding_transport_selector,
            user_data,
        };
        let b = &complete_nsdu[complete_nsdu.len()..];
        debug_assert_eq!(b.len(), 0);
        return Ok((b, TPDU::CC(cc)));
    }
    if is_ak {
        let yr_tu_nr: u32;
        let mut cdt: u16 = cdt as u16;
        let (b, dst_ref) = be_u16(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        if [2, 3, 4].contains(&class) && ext_format {
            if cdt != 0 {
                return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)));
            }
            let (b, y) = be_u32(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            yr_tu_nr = y;
            let (b, new_cdt) = be_u16(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            cdt = new_cdt;
            handle_var_part(complete_nsdu.len() - b.len())?;
        } else {
            // TODO: Check that YR does not have MSb set.
            let (b, y) = be_u8(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            yr_tu_nr = y as u32;
            handle_var_part(complete_nsdu.len() - b.len())?;
        }
        let ak = AK_TPDU{
            dst_ref,
            cdt,
            nr: yr_tu_nr,
            checksum,
            subsequence_number,
            flow_control_confirmation,
            selective_acknowledgement_parameters,
        };
        let b = &complete_nsdu[1+li as usize..];
        return Ok((b, TPDU::AK(ak)));
    }
    if is_rj {
        // TODO: Check that li == RJ length
        let yr_tu_nr: u32;
        let mut cdt: u16 = cdt as u16;
        let (b, dst_ref) = be_u16(b)
            .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
        if class == 3 && ext_format {
            if cdt != 0 {
                return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)));
            }
            let (b, y) = be_u32(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            yr_tu_nr = y;
            let (_, new_cdt) = be_u16(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            cdt = new_cdt;
        } else {
            // TODO: Check that YR does not have MSb set.
            let (_, y) = be_u8(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            yr_tu_nr = y as u32;
        }
        // There is no variable part for RJ.
        let rj = RJ_TPDU{
            cdt,
            dst_ref,
            yr_tu_nr,
        };
        let b = &complete_nsdu[1+li as usize..];
        return Ok((b, TPDU::RJ(rj)));
    }
    let (mut b, dst_ref) = be_u16(b)
        .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
    match tpdu_type {
        TPDU_CODE_DR => {
            let (b, src_ref) = be_u16(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            let (b, reason) = be_u8(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            handle_var_part(complete_nsdu.len() - b.len())?;
            let user_data = &complete_nsdu[1+li as usize..];
            let dr = DR_TPDU{
                dst_ref,
                src_ref,
                reason,
                additional_info,
                user_data,
                checksum,
            };
            let b = &complete_nsdu[complete_nsdu.len()..];
            return Ok((b, TPDU::DR(dr)));
        },
        TPDU_CODE_DC => {
            let (b, src_ref) = be_u16(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            handle_var_part(complete_nsdu.len() - b.len())?;
            let dc = DC_TPDU{
                dst_ref,
                src_ref,
                checksum,
            };
            let b = &complete_nsdu[1+li as usize..];
            return Ok((b, TPDU::DC(dc)));
        },
        TPDU_CODE_ER => {
            let (b, reject_cause) = be_u8(b)
                .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
            handle_var_part(complete_nsdu.len() - b.len())?;
            let er = ER_TPDU{
                dst_ref,
                checksum,
                invalid_tpdu,
                reject_cause,
            };
            let b = &complete_nsdu[1+li as usize..];
            return Ok((b, TPDU::ER(er)));
        },
        TPDU_CODE_ED => {
            if [2, 3, 4].contains(&class) && ext_format {
                let (b1, nr_and_eot) = be_u32(b)
                    .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
                b = b1;
                eot = (nr_and_eot & 0b10000000_00000000_00000000_00000000) > 0;
                nr  = nr_and_eot & 0b01111111_11111111_11111111_11111111;
            } else {
                let (b1, nr_and_eot) = be_u8(b)
                    .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
                b = b1;
                eot = (nr_and_eot & 0b10000000) > 0;
                nr  = (nr_and_eot & 0b01111111) as u32;
            }
            handle_var_part(complete_nsdu.len() - b.len())?;
            let user_data = &complete_nsdu[1+li as usize..];
            let ed = ED_TPDU{
                dst_ref,
                checksum,
                eot,
                nr,
                user_data,
            };
            let b = &complete_nsdu[complete_nsdu.len()..];
            return Ok((b, TPDU::ED(ed)));
        },
        TPDU_CODE_EA => {
            if [2, 3, 4].contains(&class) && ext_format {
                let (b1, ynr) = be_u32(b)
                    .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
                b = b1;
                nr  = ynr & 0b01111111_11111111_11111111_11111111;
            } else {
                let (b1, ynr) = be_u8(b)
                    .map_err(|_: NomErr<NomError<&[u8]>>| NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::MalformedFixedPart)))?;
                b = b1;
                nr  = (ynr & 0b01111111) as u32;
            }
            handle_var_part(complete_nsdu.len() - b.len())?;
            let ea = EA_TPDU{
                dst_ref,
                checksum,
                nr,
            };
            let b = &complete_nsdu[1+li as usize..];
            return Ok((b, TPDU::EA(ea)));
        },
        _ => {
            return Err(NomErr::Error(X224TpduParseError::from(X224TpduParseErrorKind::UnrecognizedTpduType(tpdu_type))));
        }
    };
    // TODO: Verify checksum. If the checksum is invalid, return None?
}


#[cfg(test)]
mod tests {
    use crate::transport::{DR_REASON_CONGESTION_AT_TSAP, ER_REJECT_CAUSE_INVALID_PARAMETER_CODE};

    use super::*;

    #[test]
    fn parses_cr_tpdu_01 () {
        // Source: https://wiki.wireshark.org/uploads/__moin_import__/attachments/SampleCaptures/p772-transfer-success.pcap
        let nsdu: &[u8] = &[
            0x18, // LI = 24
            0xe0, // CR
            0x00, 0x00, // DST-REF
            0x0d, 0xe0, // SRC-REF
            0x00, // Class 0 / Options
            0xc1, // Parameter: Calling Transport Selector
                0x07, // Length = 7
                    0x58, 0x34, 0x30, 0x30, 0x2d, 0x38, 0x38, // "X400-88"
            0xc2, // Parameter: Called Transport Selector
                0x07, // Length = 7
                    0x58, 0x34, 0x30, 0x30, 0x2d, 0x38, 0x38, // "X400-88"
            // There is no user data.
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::CR(cr) => {
                assert_eq!(cr.src_ref, 0x0de0);
                assert_eq!(cr.dst_ref, 0x0000);
                assert_eq!(cr.cdt, 0);
                assert_eq!(cr.class_option, 0);
                assert_eq!(cr.calling_transport_selector.unwrap(), &[0x58, 0x34, 0x30, 0x30, 0x2d, 0x38, 0x38]);
                assert_eq!(cr.called_or_responding_transport_selector.unwrap(), &[0x58, 0x34, 0x30, 0x30, 0x2d, 0x38, 0x38]);
                assert_eq!(cr.user_data.len(), 0);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_cc_tpdu_01 () {
        // Source: https://wiki.wireshark.org/uploads/__moin_import__/attachments/SampleCaptures/p772-transfer-success.pcap
        let nsdu: &[u8] = &[
            0x06, // LI = 6
            0xd0, // CC
            0x0d, 0xe0, // DST-REF
            0x06, 0x3c, // SRC-REF
            0x00, // Class 0 / Options
            // There is no user data.
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::CC(cc) => {
                assert_eq!(cc.dst_ref, 0x0de0);
                assert_eq!(cc.src_ref, 0x063c);
                assert_eq!(cc.cdt, 0);
                assert_eq!(cc.class_option, 0);
                assert_eq!(cc.user_data.len(), 0);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_dt_tpdu_01 () {
        // Source: https://wiki.wireshark.org/uploads/__moin_import__/attachments/SampleCaptures/p772-transfer-success.pcap
        let nsdu: &[u8] = &[
            0x02, // LI = 2
            0xf0, // DT
            0x80, // EOT / NR
            b'a', b's', b'd', b'f',
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::DT(dt) => {
                assert!(dt.dst_ref.is_none());
                assert!(dt.checksum.is_none());
                assert_eq!(dt.eot, true);
                assert_eq!(dt.roa, false);
                assert_eq!(dt.user_data, b"asdf");
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_dr_tpdu_01 () {
        let nsdu: &[u8] = &[
            0x06, // LI = 6
            TPDU_CODE_DR,
            0x12, 0x34, // DST-REF
            0x98, 0x87, // SRC-REF
            DR_REASON_CONGESTION_AT_TSAP,
            // No user data
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::DR(dr) => {
                assert_eq!(dr.dst_ref, 0x1234);
                assert_eq!(dr.src_ref, 0x9887);
                assert_eq!(dr.reason, DR_REASON_CONGESTION_AT_TSAP);
                assert!(dr.checksum.is_none());
                assert!(dr.additional_info.is_none());
                assert_eq!(dr.user_data.len(), 0);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_dc_tpdu_01 () {
        // TODO: This seems like it is looping forever.
        let nsdu: &[u8] = &[
            0x05, // LI = 5
            TPDU_CODE_DC,
            0x12, 0x34, // DST-REF
            0x98, 0x87, // SRC-REF
            // No variable part or user data.
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::DC(dc) => {
                assert_eq!(dc.dst_ref, 0x1234);
                assert_eq!(dc.src_ref, 0x9887);
                assert!(dc.checksum.is_none());
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_ed_tpdu_01 () {
        let nsdu: &[u8] = &[
            0x04, // LI = 4
            TPDU_CODE_ED,
            0x12, 0x34, // DST-REF
            0b1000_1010, // EOT=TRUE / NR=0b1010
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::ED(ed) => {
                assert_eq!(ed.dst_ref, 0x1234);
                assert_eq!(ed.eot, true);
                assert_eq!(ed.nr, 0b0000_1010);
                assert!(ed.checksum.is_none());
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_ak_tpdu_01 () {
        let nsdu: &[u8] = &[
            0x04, // LI = 4
            0b0110_1011, // AK / CDT = 0b0000_1011
            0x12, 0x34, // DST-REF
            0x06, // NR = 6
            // No variable part included.
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::AK(ak) => {
                assert_eq!(ak.cdt, 0b0000_1011);
                assert_eq!(ak.dst_ref, 0x1234);
                assert_eq!(ak.nr, 6);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_ea_tpdu_01 () {
        let nsdu: &[u8] = &[
            0x04, // LI = 4
            TPDU_CODE_EA,
            0x12, 0x34, // DST-REF
            0x07, // NR = 7
            // No variable part included.
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::EA(ea) => {
                assert_eq!(ea.dst_ref, 0x1234);
                assert_eq!(ea.nr, 7);
                assert!(ea.checksum.is_none());
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_rj_tpdu_01 () {
        let nsdu: &[u8] = &[
            0x04, // LI = 4
            0b0101_0100, // RJ / CDT = 0b0100
            0x12, 0x34, // DST-REF
            11, // NR=11
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::RJ(rj) => {
                assert_eq!(rj.dst_ref, 0x1234);
                assert_eq!(rj.cdt, 0b0100);
                assert_eq!(rj.yr_tu_nr, 11);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parses_er_tpdu_01 () {
        let nsdu: &[u8] = &[
            0x04, // LI = 4
            TPDU_CODE_ER,
            0x12, 0x34, // DST-REF
            ER_REJECT_CAUSE_INVALID_PARAMETER_CODE,
        ];
        let (b, tpdu) = parse_x224_tpdu(nsdu, 0, false).unwrap();
        assert_eq!(b.len(), 0);
        match tpdu {
            TPDU::ER(er) => {
                assert_eq!(er.dst_ref, 0x1234);
                assert_eq!(er.reject_cause, ER_REJECT_CAUSE_INVALID_PARAMETER_CODE);
                assert!(er.checksum.is_none());
            },
            _ => panic!(),
        };
    }

}
