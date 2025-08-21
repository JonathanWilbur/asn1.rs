use crate::{AFI, DSPSyntax, X213NetworkAddressType};

/// Maps group AFIs to individual ones per Table A.2 in ITU-T Rec. X.213
pub(crate) const fn normalize_afi (afi: AFI) -> AFI {
    match afi {
        0xA1 => 0x11,
        0xA2 => 0x12,
        0xA3 => 0x13,
        0xA4 => 0x14,
        0xA5 => 0x15,
        0xA6 => 0x16,
        0xA7 => 0x17,
        0xA8 => 0x18,
        0xA9 => 0x19,
        0xAB => 0x21,
        0xAC => 0x22,
        0xAD => 0x23,
        0xAE => 0x24,
        0xAF => 0x25,
        0xB0 => 0x26,
        0xB1 => 0x27,
        0xB2 => 0x28,
        0xB3 => 0x29,
        0xB4 => 0x30,
        0xB5 => 0x31,
        0xB6 => 0x32,
        0xB7 => 0x33,
        0xB8 => 0x34,
        0xB9 => 0x35,
        0xBA => 0x36,
        0xBB => 0x37,
        0xBC => 0x38,
        0xBD => 0x39,
        0xBE => 0x40,
        0xBF => 0x41,
        0xC0 => 0x42,
        0xC1 => 0x43,
        0xC2 => 0x44,
        0xC3 => 0x45,
        0xC4 => 0x46,
        0xC5 => 0x47,
        0xC6 => 0x48,
        0xC7 => 0x49,
        0xC8 => 0x50,
        0xC9 => 0x51,
        0xCA => 0x52,
        0xCB => 0x53,
        0xCC => 0x54,
        0xCD => 0x55,
        0xCE => 0x56,
        0xCF => 0x57,
        0xD0 => 0x58,
        0xD1 => 0x59,
        0xD2 => 0x60,
        0xD3 => 0x61,
        0xD4 => 0x62,
        0xD5 => 0x63,
        0xD6 => 0x64,
        0xD7 => 0x65,
        0xD8 => 0x66,
        0xD9 => 0x67,
        0xDA => 0x68,
        0xDB => 0x69,
        0xDC => 0x70,
        0xDD => 0x71,
        0xDE => 0x72,
        0xDF => 0x73,
        0xE0 => 0x74,
        0xE1 => 0x75,
        0xE2 => 0x76,
        0xE3 => 0x77,
        0xE4 => 0x78,
        0xE5 => 0x79,
        0xE6 => 0x80,
        0xE7 => 0x81,
        0xE8 => 0x82,
        0xE9 => 0x83,
        0xEA => 0x84,
        0xEB => 0x85,
        0xEC => 0x86,
        0xED => 0x87,
        0xEE => 0x88,
        0xEF => 0x89,
        0xF0 => 0x90,
        0xF1 => 0x91,
        0xF2 => 0x92,
        0xF3 => 0x93,
        0xF4 => 0x94,
        0xF5 => 0x95,
        0xF6 => 0x96,
        0xF7 => 0x97,
        0xF8 => 0x98,
        0xF9 => 0x99,
        _ => afi,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X213NetworkAddressInfo {
    pub network_type: X213NetworkAddressType,
    pub leading_zeroes_in_idi: bool,
    pub dsp_syntax: DSPSyntax,
}

/// Table of address info about NSAP syntaxes by AFI.
///
/// This table is biased by -34, because the first 34 AFIs are not defined.
///
/// Quoting X.213:
/// "The numerically greater AFI value is used when the first significant digit
/// in the IDI is zero."
const AFI_INFO: [Option<X213NetworkAddressInfo>; 45] = [
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::IANA_ICP,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 34
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::IANA_ICP,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 35
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 36
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 37
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::ISO_DCC,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 38
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::ISO_DCC,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 39
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 40
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 41
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 42
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 43
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 44
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 45
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::ISO_6523_ICD,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 46
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::ISO_6523_ICD,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 47
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::LOCAL,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 48
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::LOCAL,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 49
    None, // 50
    None, // 51
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 52
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::X121,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
    }), // 53
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 54
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::F69,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
    }), // 55
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 56
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E163,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
    }), // 57
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 58
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::E164,
        leading_zeroes_in_idi: true,
        dsp_syntax: DSPSyntax::Binary,
    }), // 59
    None, // 60
    None, // 61
    None, // 62
    None, // 63
    None, // 64
    None, // 65
    None, // 66
    None, // 67
    None, // 68
    None, // 69
    None, // 70
    None, // 71
    None, // 72
    None, // 73
    None, // 74
    None, // 75
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::ITU_T_IND,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Decimal,
    }), // 76
    Some(X213NetworkAddressInfo{
        network_type: X213NetworkAddressType::ITU_T_IND,
        leading_zeroes_in_idi: false,
        dsp_syntax: DSPSyntax::Binary,
    }), // 77
    None, // 78
];

pub const fn get_address_type_info(afi: AFI) -> Option<X213NetworkAddressInfo> {
    let normalized = normalize_afi(afi);
    // Conversion of the BCD to a true binary value.
    let afi_bin: u8 = (((normalized & 0xF0) >> 4) * 10) + (normalized & 0x0F);
    if afi_bin < 34 || afi_bin > 77 {
        return None;
    }
    // The first 34 are undefined
    let afi_bin = afi_bin - 34;
    AFI_INFO[afi_bin as usize]
}
