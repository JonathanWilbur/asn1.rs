use bytes::BytesMut;
use nom::{IResult, bytes::complete::{tag, take}, Err, multi::many0, combinator::peek};

pub struct Tpkt {
    pub vrsn: u8,
    pub reserved: u8,
    pub packet_length: u16,
    pub tpdu: BytesMut,
}

pub struct Q931Message {
    pub length_of_call_reference: u8,
    pub call_reference: [u8; 16],
    pub message_type: u8,
}

pub struct Q931OtherInfoElement {
    pub identifier: u8,
    pub contents: Vec<u8>,
}

pub const PROTOCOL_DISCRIMINATOR_Q931: u8                       = 0b1000_1000;
pub const PROTOCOL_DISCRIMINATOR_Q2931: u8                      = 0b0000_1001;
pub const INFO_ELEMENT_SHIFT: u8                                = 0b1001_0000;
pub const INFO_ELEMENT_MORE_DATA: u8                            = 0b1010_0000;
pub const INFO_ELEMENT_SENDING_COMPLETE: u8                     = 0b1010_0001;
pub const INFO_ELEMENT_CONGESTION_LEVEL: u8                     = 0b1011_0000;
pub const INFO_ELEMENT_REPEAT_INDICATOR: u8                     = 0b1101_0000;
pub const INFO_ELEMENT_SEGMENTED_MESSAGE: u8                    = 0b0000_0000;
pub const INFO_ELEMENT_BEARER_CAPABILITY: u8                    = 0b0000_0100;
pub const INFO_ELEMENT_CAUSE: u8                                = 0b0000_1000;
pub const INFO_ELEMENT_CALL_IDENTITY: u8                        = 0b0001_0000;
pub const INFO_ELEMENT_CALL_STATE: u8                           = 0b0001_0100;
pub const INFO_ELEMENT_CHANNEL_ID: u8                           = 0b0001_1000;
pub const INFO_ELEMENT_PROGRESS_INDICATOR: u8                   = 0b0001_1110;
pub const INFO_ELEMENT_NETWORK_SPECIFIC_FACILITIES: u8          = 0b0010_0000;
pub const INFO_ELEMENT_NOTIFICATION_INDICATOR: u8               = 0b0010_0111;
pub const INFO_ELEMENT_DISPLAY: u8                              = 0b0010_1000;
pub const INFO_ELEMENT_DATE_TIME: u8                            = 0b0010_1001;
pub const INFO_ELEMENT_KEYPAD_FACILITY: u8                      = 0b0010_1100;
pub const INFO_ELEMENT_SIGNAL: u8                               = 0b0011_0100;
pub const INFO_ELEMENT_INFORMATION_RATE: u8                     = 0b0100_0000;
pub const INFO_ELEMENT_END_TO_END_TRANSIT_DELAY: u8             = 0b0100_0010;
pub const INFO_ELEMENT_TRANSIT_DELAY_SEL_AND_IND: u8            = 0b0100_0011;
pub const INFO_ELEMENT_PACKET_LAYER_BINARY_PARAMS: u8           = 0b0100_0100;
pub const INFO_ELEMENT_PACKET_LAYER_WINDOW_SIZE: u8             = 0b0100_0101;
pub const INFO_ELEMENT_PACKET_SIZE: u8                          = 0b0100_0110;
pub const INFO_ELEMENT_CLOSED_USER_GROUP: u8                    = 0b0100_0111;
pub const INFO_ELEMENT_REVERSE_CHARGING_INDICATION: u8          = 0b0100_1010;
pub const INFO_ELEMENT_CALLING_PARTY_NUMBER: u8                 = 0b0110_1100;
pub const INFO_ELEMENT_CALLING_PARTY_SUBADDRESS: u8             = 0b0110_1101;
pub const INFO_ELEMENT_CALLED_PARTY_NUMBER: u8                  = 0b0111_0000;
pub const INFO_ELEMENT_CALLED_PARTY_SUBADDRESS: u8              = 0b0111_0001;
pub const INFO_ELEMENT_REDIRECTING_NUMBER: u8                   = 0b0111_0100;
pub const INFO_ELEMENT_TRANSIT_NETWORK_SELECTION: u8            = 0b0111_1000;
pub const INFO_ELEMENT_RESTART_INDICATION: u8                   = 0b0111_1001;
pub const INFO_ELEMENT_LOW_LAYER_COMPATIBILITY: u8              = 0b0111_1100;
pub const INFO_ELEMENT_HIGH_LAYER_COMPATIBILITY: u8             = 0b0111_1101;
pub const INFO_ELEMENT_USER_USER: u8                            = 0b0111_1110;
pub const INFO_ELEMENT_ESCAPE_FOR_EXTENSION: u8                 = 0b0111_1111;
pub const MESSAGE_TYPE_ESCAPE: u8                               = 0b0000_0000;
pub const MESSAGE_TYPE_ALERTING: u8                             = 0b0000_0001;
pub const MESSAGE_TYPE_CALL_PROCEEDING: u8                      = 0b0000_0010;
pub const MESSAGE_TYPE_CONNECT: u8                              = 0b0000_0111;
pub const MESSAGE_TYPE_CONNECT_ACKNOWLEDGE: u8                  = 0b0000_1111;
pub const MESSAGE_TYPE_PROGRESS: u8                             = 0b0000_0011;
pub const MESSAGE_TYPE_SETUP: u8                                = 0b0000_0101;
pub const MESSAGE_TYPE_SETUP_ACKNOWLEDGE: u8                    = 0b0000_1101;
pub const MESSAGE_TYPE_RESUME: u8                               = 0b0010_0110;
pub const MESSAGE_TYPE_RESUME_ACKNOWLEDGE: u8                   = 0b0010_1110;
pub const MESSAGE_TYPE_RESUME_REJECT: u8                        = 0b0010_0010;
pub const MESSAGE_TYPE_SUSPEND: u8                              = 0b0010_0101;
pub const MESSAGE_TYPE_SUSPEND_ACKNOWLEDGE: u8                  = 0b0010_1101;
pub const MESSAGE_TYPE_SUSPEND_REJECT: u8                       = 0b0010_0001;
pub const MESSAGE_TYPE_USER_INFORMATION: u8                     = 0b0010_0000;
pub const MESSAGE_TYPE_DISCONNECT: u8                           = 0b0100_0101;
pub const MESSAGE_TYPE_RELEASE: u8                              = 0b0100_1101;
pub const MESSAGE_TYPE_RELEASE_COMPLETE: u8                     = 0b0101_1010;
pub const MESSAGE_TYPE_RESTART: u8                              = 0b0100_0110;
pub const MESSAGE_TYPE_RESTART_ACKNOWLEDGE: u8                  = 0b0100_1110;
pub const MESSAGE_TYPE_SEGMENT: u8                              = 0b0110_0000;
pub const MESSAGE_TYPE_CONGESTION_CONTROL: u8                   = 0b0111_1001;
pub const MESSAGE_TYPE_INFORMATION: u8                          = 0b0111_1011;
pub const MESSAGE_TYPE_NOTIFY: u8                               = 0b0110_1110;
pub const MESSAGE_TYPE_STATUS: u8                               = 0b0111_1101;
pub const MESSAGE_TYPE_STATUS_ENQUIRY: u8                       = 0b0111_0101;
pub const BEARER_CAP_CODING_STANDARD_ITU_T: u8                  = 0b0000_0000;
pub const BEARER_CAP_CODING_STANDARD_ISO_IEC: u8                = 0b0100_0000;
pub const BEARER_CAP_CODING_STANDARD_NATIONAL: u8               = 0b1000_0000;
pub const BEARER_CAP_CODING_STANDARD_STANDARD: u8               = 0b1100_0000;
pub const BEARER_CAP_INFO_TXFR_SPEECH: u8                       = 0b0000_0000;
pub const BEARER_CAP_INFO_TXFR_UNRESTRICTED_DIGITAL: u8         = 0b0000_1000;
pub const BEARER_CAP_INFO_TXFR_RESTRICTED_DIGITAL: u8           = 0b0000_1001;
pub const BEARER_CAP_INFO_TXFR_31KHZ_AUDIO: u8                  = 0b0001_0000;
pub const BEARER_CAP_INFO_TXFR_UNRESTRICTED_DIGITAL_W_TONES: u8 = 0b0001_0001;
pub const BEARER_CAP_INFO_TXFR_VIDEO: u8                        = 0b0001_1000;
pub const BEARER_CAP_TRANSFER_MODE_CIRCUIT: u8                  = 0b0000_0000;
pub const BEARER_CAP_TRANSFER_MODE_PACKET: u8                   = 0b1000_0000;
pub const BEARER_CAP_INFO_TRANSFER_RATE_PACKET_MODE: u8         = 0b0000_0000;
pub const BEARER_CAP_INFO_TRANSFER_RATE_64_KBPS: u8             = 0b0001_0000;
pub const BEARER_CAP_INFO_TRANSFER_RATE_2_X_64_KBPS: u8         = 0b0001_0001;
pub const BEARER_CAP_INFO_TRANSFER_RATE_384_KBPS: u8            = 0b0001_0011;
pub const BEARER_CAP_INFO_TRANSFER_RATE_1536_KBPS: u8           = 0b0001_0101;
pub const BEARER_CAP_INFO_TRANSFER_RATE_1920_KPBS: u8           = 0b0001_0111;
pub const BEARER_CAP_INFO_TRANSFER_RATE_MULTIRATE: u8           = 0b0001_1000;
pub const BEARER_CAP_USER_INFO_L1_PROTO_V110_I460_X30: u8       = 0b0000_0001;
pub const BEARER_CAP_USER_INFO_L1_PROTO_G711_MU_LAW: u8         = 0b0000_0010;
pub const BEARER_CAP_USER_INFO_L1_PROTO_G711_A_LAW: u8          = 0b0000_0011;
pub const BEARER_CAP_USER_INFO_L1_PROTO_G721: u8                = 0b0000_0100;
pub const BEARER_CAP_USER_INFO_L1_PROTO_H221_AND_H242: u8       = 0b0000_0101;
pub const BEARER_CAP_USER_INFO_L1_PROTO_RECS_H223_AND_H245: u8  = 0b0000_0110;
pub const BEARER_CAP_USER_INFO_L1_PROTO_NON_ITU_RATE_ADAPT: u8  = 0b0000_0111;
pub const BEARER_CAP_USER_INFO_L1_PROTO_V120: u8                = 0b0000_1000;
pub const BEARER_CAP_USER_INFO_L1_PROTO_X31: u8                 = 0b0000_1001;
pub const BEARER_CAP_SYNCHRONOUS: u8                            = 0b0000_0000;
pub const BEARER_CAP_ASYNCHRONOUS: u8                           = 0b1000_0000;
pub const BEARER_CAP_NEGOTIATION_NOT_POSSIBLE: u8               = 0b0000_0000;
pub const BEARER_CAP_NEGOTIATION_POSSIBLE: u8                   = 0b0100_0000;
pub const BEARER_CAP_USER_RATE_X: u8                            = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_6_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_1_2_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_2_4_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_3_6_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_4_8_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_7_2_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_8_KBPS: u8                       = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_9_6_KBPS: u8                     = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_14_4_KBPS: u8                    = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_16_KBPS: u8                      = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_19_2_KBPS: u8                    = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_32_KBPS: u8                      = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_38_4_KBPS: u8                    = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_48_KBPS: u8                      = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_56_KBPS: u8                      = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_57_6_KBPS: u8                    = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_28_8_KBPS: u8                    = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_24_KBPS: u8                      = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_1345_KBPS: u8                  = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_100_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_075_1_2_KBPS: u8               = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_1_2_0_075_KBPS: u8               = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_050_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_075_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_110_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_150_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_200_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_0_300_KBPS: u8                   = 0b0000_0000;
pub const BEARER_CAP_USER_RATE_12_KBPS: u8                      = 0b0000_0000;
pub const BEARER_CAP_INTERMEDIATE_RATE_NOTUSED: u8              = 0b0000_0000;
pub const BEARER_CAP_INTERMEDIATE_RATE_08_KBPS: u8              = 0b0010_0000;
pub const BEARER_CAP_INTERMEDIATE_RATE_16_KBPS: u8              = 0b0100_0000;
pub const BEARER_CAP_INTERMEDIATE_RATE_32_KBPS: u8              = 0b0110_0000;
pub const BEARER_CAP_NIC_TX_NOT_REQUIRED: u8                    = 0b0000_0000;
pub const BEARER_CAP_NIC_TX_REQUIRED: u8                        = 0b0001_0000;
pub const BEARER_CAP_NIC_RX_NOT_REQUIRED: u8                    = 0b0000_0000;
pub const BEARER_CAP_NIC_RX_REQUIRED: u8                        = 0b0000_1000;
pub const BEARER_CAP_FLOW_CONTROL_TX_NOT_REQUIRED: u8           = 0b0000_0000;
pub const BEARER_CAP_FLOW_CONTROL_TX_REQUIRED: u8               = 0b0000_0100;
pub const BEARER_CAP_FLOW_CONTROL_RX_NOT_REQUIRED: u8           = 0b0000_0000;
pub const BEARER_CAP_FLOW_CONTROL_RX_REQUIRED: u8               = 0b0000_0010;
pub const BEARER_CAP_RATE_ADAPTATION_HEADER_NOT_INCLUDED: u8    = 0b0000_0000;
pub const BEARER_CAP_RATE_ADAPTATION_HEADER_INCLUDED: u8        = 0b0100_0000;
pub const BEARER_CAP_MULTIPLE_FRAME_ESTAB_NOT_SUPPORTED: u8     = 0b0000_0000;
pub const BEARER_CAP_MULTIPLE_FRAME_ESTAB_SUPPORTED: u8         = 0b0010_0000;
pub const BEARER_CAP_MODE_OF_OP_BIT_TRANSPARENT: u8             = 0b0000_0000;
pub const BEARER_CAP_MODE_OF_OP_PROTOCOL_SENSITIVE: u8          = 0b0001_0000;
pub const BEARER_CAP_LOGICAL_LINK_ID_NEGOTIATION_DEFAULT: u8    = 0b0000_0000;
pub const BEARER_CAP_LOGICAL_LINK_ID_FULL_PROTOCOL: u8          = 0b0000_1000;
pub const BEARER_CAP_ORIGINATOR_IS_DEFAULT_ASSIGNEE: u8         = 0b0000_0000;
pub const BEARER_CAP_ORIGINATOR_IS_ASSIGNOR_ONLY: u8            = 0b0000_0100;
pub const BEARER_CAP_NEGOT_IS_DONE_W_USER_INFO: u8              = 0b0000_0000;
pub const BEARER_CAP_NEGOT_IS_DONE_IN_BAND_USING_LL0: u8        = 0b0000_0010;
pub const BEARER_CAP_NUMBER_OF_STOP_BITS_UNUSED: u8             = 0b0000_0000;
pub const BEARER_CAP_NUMBER_OF_STOP_BITS_5_BITS: u8             = 0b0010_0000;
pub const BEARER_CAP_NUMBER_OF_STOP_BITS_7_BITS: u8             = 0b0100_0000;
pub const BEARER_CAP_NUMBER_OF_STOP_BITS_8_BITS: u8             = 0b0110_0000;
pub const BEARER_CAP_NUMBER_OF_DATA_BITS_UNUSED: u8             = 0b0000_0000;
pub const BEARER_CAP_NUMBER_OF_DATA_BITS_5_BITS: u8             = 0b0000_1000;
pub const BEARER_CAP_NUMBER_OF_DATA_BITS_7_BITS: u8             = 0b0001_0000;
pub const BEARER_CAP_NUMBER_OF_DATA_BITS_8_BITS: u8             = 0b0001_1000;
pub const BEARER_CAP_PARITY_INFO_ODD: u8                        = 0b0000_0000;
pub const BEARER_CAP_PARITY_INFO_EVEN: u8                       = 0b0000_0010;
pub const BEARER_CAP_PARITY_INFO_NONE: u8                       = 0b0000_0011;
pub const BEARER_CAP_PARITY_INFO_FORCED_TO_0: u8                = 0b0000_0100;
pub const BEARER_CAP_PARITY_INFO_FORCED_TO_1: u8                = 0b0000_0101;
pub const BEARER_CAP_MODE_DUPLEX_HALF: u8                       = 0b0000_0000;
pub const BEARER_CAP_MODE_DUPLEX_FULL: u8                       = 0b0100_0000;
pub const BEARER_CAP_MODEM_TYPE_V_21: u8                        = 0b0001_0001;
pub const BEARER_CAP_MODEM_TYPE_V_22: u8                        = 0b0001_0010;
pub const BEARER_CAP_MODEM_TYPE_V_22_BIS: u8                    = 0b0001_0011;
pub const BEARER_CAP_MODEM_TYPE_V_23: u8                        = 0b0001_0100;
pub const BEARER_CAP_MODEM_TYPE_V_26: u8                        = 0b0001_0101;
pub const BEARER_CAP_MODEM_TYPE_V_26_BIS: u8                    = 0b0001_0110;
pub const BEARER_CAP_MODEM_TYPE_V_26_TER: u8                    = 0b0001_0111;
pub const BEARER_CAP_MODEM_TYPE_V_27: u8                        = 0b0001_1000;
pub const BEARER_CAP_MODEM_TYPE_V_27_BIS: u8                    = 0b0001_1001;
pub const BEARER_CAP_MODEM_TYPE_V_27_TER: u8                    = 0b0001_1010;
pub const BEARER_CAP_MODEM_TYPE_V_29: u8                        = 0b0001_1011;
pub const BEARER_CAP_MODEM_TYPE_V_32: u8                        = 0b0001_1101;
pub const BEARER_CAP_MODEM_TYPE_V_34: u8                        = 0b0001_1110;
pub const BEARER_CAP_USER_INFO_LAYER_2_PROTOCOL_Q_921: u8       = 0b0000_0010;
pub const BEARER_CAP_USER_INFO_LAYER_2_PROTOCOL_X_25_LL: u8     = 0b0000_0110;
pub const BEARER_CAP_USER_INFO_LAYER_2_PROTOCOL_LAN_LLC: u8     = 0b0000_1100;
pub const BEARER_CAP_USER_INFO_LAYER_3_PROTOCOL_Q_931: u8       = 0b0000_0010;
pub const BEARER_CAP_USER_INFO_LAYER_3_PROTOCOL_X_25_PL: u8     = 0b0000_0110;
pub const BEARER_CAP_USER_INFO_LAYER_3_PROTOCOL_ISO_TR_9577: u8 = 0b0000_1011;
pub const BEARER_CAP_7A_ADDL_LAYER_3_PROT_INFO_RFC_791: u8      = 0b0000_1100;
pub const BEARER_CAP_7A_ADDL_LAYER_3_PROT_INFO_RFC_1548: u8     = 0b0000_1100;
pub const BEARER_CAP_7B_ADDL_LAYER_3_PROT_INFO_RFC_791: u8      = 0b0000_1100;
pub const BEARER_CAP_7B_ADDL_LAYER_3_PROT_INFO_RFC_1548: u8     = 0b0000_1111;
pub const CALL_STATE_ITU_STANDARDIZED_CODING: u8                = 0b0000_0000;
pub const CALL_STATE_ISO_IEC_STANDARD: u8                       = 0b0100_0000;
pub const CALL_STATE_NATIONAL_STANDARD: u8                      = 0b1000_0000;
pub const CALL_STATE_NETWORK_STANDARD: u8                       = 0b1100_0000;
pub const CALL_STATE_U00: u8                                    = 0b0000_0000; // NULL
pub const CALL_STATE_U01: u8                                    = 0b0000_0001; // Call initiated
pub const CALL_STATE_U02: u8                                    = 0b0000_0010; // Overlap sending
pub const CALL_STATE_U03: u8                                    = 0b0000_0011; // Outgoing call proceeding
pub const CALL_STATE_U04: u8                                    = 0b0000_0100; // Call delivered
pub const CALL_STATE_U06: u8                                    = 0b0000_0110; // Call present
pub const CALL_STATE_U07: u8                                    = 0b0000_0111; // Call received
pub const CALL_STATE_U08: u8                                    = 0b0000_1000; // Connect request
pub const CALL_STATE_U09: u8                                    = 0b0000_1001; // Incoming call proceeding
pub const CALL_STATE_U10: u8                                    = 0b0000_1010; // Active
pub const CALL_STATE_U11: u8                                    = 0b0000_1011; // Disconnect request
pub const CALL_STATE_U12: u8                                    = 0b0000_1100; // Disconnect indication
pub const CALL_STATE_U15: u8                                    = 0b0000_1111; // Suspend request
pub const CALL_STATE_U17: u8                                    = 0b0001_0001; // Resume request
pub const CALL_STATE_U19: u8                                    = 0b0001_0011; // Release request
pub const CALL_STATE_U25: u8                                    = 0b0001_1001; // Overlap receiving
pub const CALL_STATE_N00: u8                                    = 0b0000_0000; // NULL
pub const CALL_STATE_N01: u8                                    = 0b0000_0001; // Call initiated
pub const CALL_STATE_N02: u8                                    = 0b0000_0010; // Overlap sending
pub const CALL_STATE_N03: u8                                    = 0b0000_0011; // Outgoing call proceeding
pub const CALL_STATE_N04: u8                                    = 0b0000_0100; // Call delivered
pub const CALL_STATE_N06: u8                                    = 0b0000_0110; // Call present
pub const CALL_STATE_N07: u8                                    = 0b0000_0111; // Call received
pub const CALL_STATE_N08: u8                                    = 0b0000_1000; // Connect request
pub const CALL_STATE_N09: u8                                    = 0b0000_1001; // Incoming call proceeding
pub const CALL_STATE_N10: u8                                    = 0b0000_1010; // Active
pub const CALL_STATE_N11: u8                                    = 0b0000_1011; // Disconnect request
pub const CALL_STATE_N12: u8                                    = 0b0000_1100; // Disconnect indication
pub const CALL_STATE_N15: u8                                    = 0b0000_1111; // Suspend request
pub const CALL_STATE_N17: u8                                    = 0b0001_0001; // Resume request
pub const CALL_STATE_N19: u8                                    = 0b0001_0011; // Release request
pub const CALL_STATE_N22: u8                                    = 0b0001_0110; // Call Abort
pub const CALL_STATE_N25: u8                                    = 0b0001_1001; // Overlap receiving
pub const CALL_STATE_GLOBAL_INTERFACE_REST_0: u8                = 0b0000_0000; // Null
pub const CALL_STATE_GLOBAL_INTERFACE_REST_1: u8                = 0b0011_1101; // Restart request
pub const CALL_STATE_GLOBAL_INTERFACE_REST_2: u8                = 0b0011_1110; // Restart
pub const CALLED_PARTY_ADDRESS_TYPE_UNKNOWN: u8                 = 0b0000_0000;
pub const CALLED_PARTY_ADDRESS_TYPE_INTERNATIONAL: u8           = 0b0001_0000;
pub const CALLED_PARTY_ADDRESS_TYPE_NATIONAL: u8                = 0b0010_0000;
pub const CALLED_PARTY_ADDRESS_TYPE_NETWORK: u8                 = 0b0011_0000;
pub const CALLED_PARTY_ADDRESS_TYPE_SUBSCRIBER: u8              = 0b0100_0000;
pub const CALLED_PARTY_ADDRESS_TYPE_ABBREVIATED: u8             = 0b0110_0000;
pub const CALLED_PARTY_ADDRESS_TYPE_RESERVED: u8                = 0b0111_0000;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_UNKNOWN: u8       = 0b0000_0000;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_ISDN: u8          = 0b0000_0001;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_DATA: u8          = 0b0000_0011;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_TELEX: u8         = 0b0000_0100;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_NATIONAL: u8      = 0b0000_1000;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_PRIVATE: u8       = 0b0000_1001;
pub const CALLED_PARTY_ADDRESS_NUMBERING_PLAN_RESERVED: u8      = 0b0000_1111;
pub const CALLED_PARTY_SUBADDR_TYPE_NSAP: u8                    = 0b0000_0000;
pub const CALLED_PARTY_SUBADDR_TYPE_USER: u8                    = 0b0010_0000;
pub const CALLED_PARTY_SUBADDR_EVEN_NUM_OF_ADDR_SIGNALS: u8     = 0b0000_0000;
pub const CALLED_PARTY_SUBADDR_ODD_NUM_OF_ADDR_SIGNALS: u8      = 0b0000_1000;
pub const CALLING_PARTY_ADDRESS_TYPE_UNKNOWN: u8                = 0b0000_0000;
pub const CALLING_PARTY_ADDRESS_TYPE_INTERNATIONAL: u8          = 0b0001_0000;
pub const CALLING_PARTY_ADDRESS_TYPE_NATIONAL: u8               = 0b0010_0000;
pub const CALLING_PARTY_ADDRESS_TYPE_NETWORK: u8                = 0b0011_0000;
pub const CALLING_PARTY_ADDRESS_TYPE_SUBSCRIBER: u8             = 0b0100_0000;
pub const CALLING_PARTY_ADDRESS_TYPE_ABBREVIATED: u8            = 0b0110_0000;
pub const CALLING_PARTY_ADDRESS_TYPE_RESERVED: u8               = 0b0111_0000;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_UNKNOWN: u8      = 0b0000_0000;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_ISDN: u8         = 0b0000_0001;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_DATA: u8         = 0b0000_0011;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_TELEX: u8        = 0b0000_0100;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_NATIONAL: u8     = 0b0000_1000;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_PRIVATE: u8      = 0b0000_1001;
pub const CALLING_PARTY_ADDRESS_NUMBERING_PLAN_RESERVED: u8     = 0b0000_1111;
pub const CALLING_PARTY_PRES_INDICATOR_ALLOWED: u8              = 0b0000_0000;
pub const CALLING_PARTY_PRES_INDICATOR_RESTRICTED: u8           = 0b0010_0000;
pub const CALLING_PARTY_PRES_INDICATOR_NOT_AVAILABLE: u8        = 0b0100_0000;
pub const CALLING_PARTY_PRES_INDICATOR_RESERVED: u8             = 0b0110_0000;
pub const CALLING_PARTY_SCREENING_USER_PROVIDED_NOT: u8         = 0b0000_0000;
pub const CALLING_PARTY_SCREENING_USER_PROVIDED_PASSED: u8      = 0b0000_0001;
pub const CALLING_PARTY_SCREENING_USER_PROVIDED_FAILED: u8      = 0b0000_0010;
pub const CALLING_PARTY_SCREENING_NETWORK_PROVIDED: u8          = 0b0000_0011;
pub const CALLING_PARTY_SUBADDR_TYPE_NSAP: u8                   = 0b0000_0000;
pub const CALLING_PARTY_SUBADDR_TYPE_USER: u8                   = 0b0010_0000;
pub const CALLING_PARTY_SUBADDR_EVEN_NUM_OF_ADDR_SIGNALS: u8    = 0b0000_0000;
pub const CALLING_PARTY_SUBADDR_ODD_NUM_OF_ADDR_SIGNALS: u8     = 0b0000_1000;
pub const CHANNEL_ID_INTERFACE_IMPLICITLY_IDENTIFIED: u8        = 0b0000_0000;
pub const CHANNEL_ID_INTERFACE_EXPLICITLY_IDENTIFIED: u8        = 0b0100_0000;
pub const CHANNEL_ID_INTERFACE_TYPE_BASIC: u8                   = 0b0000_0000;
pub const CHANNEL_ID_INTERFACE_TYPE_OTHER: u8                   = 0b0010_0000;
pub const CHANNEL_ID_PREFERRED: u8                              = 0b0000_0000;
pub const CHANNEL_ID_EXCLUSIVE: u8                              = 0b0000_1000;
pub const CHANNEL_ID_D_CHANNEL_FALSE: u8                        = 0b0000_0000;
pub const CHANNEL_ID_D_CHANNEL_TRUE: u8                         = 0b0000_0100;
pub const CHANNEL_ID_CODING_STANDARD_ITU: u8                    = 0b0000_0000;
pub const CHANNEL_ID_CODING_STANDARD_ISO_IEC: u8                = 0b0010_0000;
pub const CHANNEL_ID_CODING_STANDARD_NATIONAL: u8               = 0b0100_0000;
pub const CHANNEL_ID_CODING_STANDARD_NETWORK: u8                = 0b0110_0000;
pub const CHANNEL_ID_FOLLOWING_OCTET: u8                        = 0b0000_0000;
pub const CHANNEL_ID_SLOT_MAP: u8                               = 0b0001_0000;
pub const CHANNEL_ID_TYPE_B_CHANNEL_UNITS: u8                   = 0b0000_0011;
pub const CHANNEL_ID_TYPE_H0_CHANNEL_UNITS: u8                  = 0b0000_0110;
pub const CHANNEL_ID_TYPE_H11_CHANNEL_UNITS: u8                 = 0b0000_1000;
pub const CHANNEL_ID_TYPE_H12_CHANNEL_UNITS: u8                 = 0b0000_1001;
pub const CONGESTION_LEVEL_RECEIVER_READY: u8                   = 0b0000_0000;
pub const CONGESTION_LEVEL_RECEIVER_NOT_READY: u8               = 0b0000_1111;
// TODO: I left off here. Filling in all of these constants in Q.931.

// TODO: Q.850: Cause parameters

pub fn parse_q931_other_info_element (input: &[u8]) -> IResult<&[u8], Q931OtherInfoElement> {
    let (input, b0) = take(1usize)(input)?;
    if (b0[0] & 0b1000_0000u8) > 0 {
        Ok((input, Q931OtherInfoElement {
            identifier: b0[0],
            contents: vec![],
        }))
    } else {
        let (input, len_of_contents_bytes) = take(1usize)(input)?;
        let len_of_contents = len_of_contents_bytes[0];
        let (input, contents_bytes) = take(len_of_contents)(input)?;
        Ok((input, Q931OtherInfoElement {
            identifier: b0[0],
            contents: Vec::from(contents_bytes),
        }))
    }
}

// TODO: Implement a different error type.
pub fn parse_q931_message (input: &[u8]) -> IResult<&[u8], Q931Message> {
    let (input, _) = tag([PROTOCOL_DISCRIMINATOR_Q931])(input)?;
    let (input, len_of_call_ref_bytes) = take(1usize)(input)?;
    let len_of_call_ref = len_of_call_ref_bytes[0];
    if len_of_call_ref > 15 {
        return Err(Err::Failure(nom::error::Error { input, code: nom::error::ErrorKind::TooLarge }));
    }
    let (input, call_ref_bytes) = take(len_of_call_ref)(input)?;
    let (input, message_type_bytes) = take(1usize)(input)?;
    let message_type = message_type_bytes[0];
    let mut call_reference: [u8; 16] = [0; 16];
    (&mut call_reference[0..call_ref_bytes.len()]).copy_from_slice(call_ref_bytes);
    let (input, info_elements) = many0(parse_q931_other_info_element)(input)?;
    let mut active_codeset: u8 = 0;
    let mut temporary_codeset: Option<u8> = None;
    for ie in info_elements {
        /* It is not clear whether this information element is the same between
        codesets. I assume it is, because you need some way to change into other
        codesets... */
        if (ie.identifier & INFO_ELEMENT_SHIFT) == INFO_ELEMENT_SHIFT {
            let locking: bool = (ie.identifier & 0b0000_1000) == 0;
            if locking {
                temporary_codeset = Some(ie.identifier & 0b0000_0111);
            } else {
                active_codeset = ie.identifier & 0b0000_0111;
            }
        }
        if let Some(temp_codeset) = temporary_codeset {
            temporary_codeset = None;
            if temp_codeset != 0 {
                continue; // Any other codeset is not understood.
            }
        }
        else if active_codeset != 0 { // Any other codeset is not understood.
            continue;
        }
        // match ie.identifier {

        // }
    }
    Ok((input, Q931Message {
        length_of_call_reference: call_ref_bytes.len() as u8,
        call_reference,
        message_type,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
