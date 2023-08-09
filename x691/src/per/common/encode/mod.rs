//! ASN.1 PER Encoding common functions

use bitvec::prelude::*;

use crate::per::{PerCodecData, PerCodecError};
use asn1::OBJECT_IDENTIFIER;
use x690::x690_write_object_identifier_value;

mod encode_internal;

#[allow(unused)]
use encode_internal::*;

// Functions defined in this module are called by the respective API functions in the codecs. For
// example, `crate::aper::encode::encode_choice_index` would call `encode_choice_index_common`
// with `aligned` as `true`.

// Common function to encode a Choice Index
pub(crate) fn encode_choice_idx_common(
    data: &mut PerCodecData,
    lb: i128,
    ub: i128,
    is_extensible: bool,
    idx: i128,
    extended: bool,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended choice not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }

    encode_integer_common(data, Some(lb), Some(ub), false, idx, false, aligned)
}

// Common function to encode a sequence header.
pub(crate) fn encode_sequence_header_common(
    data: &mut PerCodecData,
    is_extensible: bool,
    optionals: &BitSlice<u8, Msb0>,
    extended: bool,
    _aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended sequence not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }

    data.append_bits(optionals);

    data.dump_encode();

    Ok(())
}

// Common function to encode an integer
pub(crate) fn encode_integer_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    value: i128,
    extended: bool,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended integer not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }

    match (lb, ub) {
        (None, _) => encode_unconstrained_whole_number_common(data, value, aligned)?,
        (Some(lb), None) => encode_semi_constrained_whole_number_common(data, lb, value, aligned)?,
        (Some(lb), Some(ub)) => {
            encode_constrained_whole_number_common(data, lb, ub, value, aligned)?
        }
    };

    data.dump_encode();

    Ok(())
}

// Common function to encode a BOOLEAN Value
pub(crate) fn encode_bool_common(
    data: &mut PerCodecData,
    value: bool,
    _aligned: bool,
) -> Result<(), PerCodecError> {
    data.encode_bool(value);

    data.dump_encode();
    Ok(())
}

// Common function to encode an ENUMERATED Value
pub(crate) fn encode_enumerated_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    value: i128,
    extended: bool,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended enumerated not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }

    encode_integer_common(data, lb, ub, false, value, false, aligned)?;

    data.dump();

    Ok(())
}

// Common function to encode a bitstring
// Refer to Section 15.
pub(crate) fn encode_bitstring_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    bit_string: &BitSlice<u8, Msb0>,
    extended: bool,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended bitstring not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }

    let length = bit_string.len();
    if length >= 16384 {
        return Err(PerCodecError::new(
            "Encode of fragmented bitstring not yet implemented",
        ));
    }

    encode_length_determinent_common(data, lb, ub, false, length, aligned)?;
    if length > 0 {
        if length > 16 {
            if aligned {
                data.align();
            }
        }
        data.append_bits(bit_string);
    }

    // TODO: Not sure if 15.11 is handled correctly?
    data.dump_encode();

    Ok(())
}

// Common function to encode an OCTET STRING
pub(crate) fn encode_octet_string_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    octet_string: &[u8],
    extended: bool,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended octetstring not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }

    let length = octet_string.len();
    if length >= 16384 {
        return Err(PerCodecError::new(
            "Encode of fragmented octetstring not yet implemented",
        ));
    }

    encode_length_determinent_common(data, lb, ub, false, length, aligned)?;

    if length > 0 {
        if length > 2 {
            if aligned {
                data.align();
            }
        }
        data.append_bits(octet_string.view_bits());
    }

    data.dump_encode();
    Ok(())
}

// Encode a Length Determinent
pub(crate) fn encode_length_determinent_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    normally_small: bool,
    value: usize,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if normally_small {
        encode_normally_small_length_determinent_common(data, value, aligned)?;
        data.dump_encode();

        return Ok(());
    }

    match ub {
        Some(ub) if ub < 65_536 => encode_constrained_whole_number_common(
            data,
            lb.unwrap_or(0),
            ub,
            value as i128,
            aligned,
        )?,
        _ => {
            if let Some(u) = ub {
                if value > u as usize {
                    return Err(PerCodecError::new(format!(
                        "Cannot encode length determinent {} - greater than upper bound {}",
                        value, u,
                    )));
                }
            }

            if let Some(l) = lb {
                if value < l as usize {
                    return Err(PerCodecError::new(format!(
                        "Cannot encode length determinent {} - less than lower bound {}",
                        value, l,
                    )));
                }
            }

            encode_indefinite_length_determinent_common(data, value, aligned)?
        }
    };

    data.dump_encode();

    Ok(())
}

// Common function to encode string value.
pub(crate) fn encode_string_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    value: &String,
    extended: bool,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if extended {
        return Err(PerCodecError::new(
            "Encode of extended visible string not yet implemented",
        ));
    }

    if is_extensible {
        data.encode_bool(extended);
    }
    encode_length_determinent_common(data, lb, ub, false, value.len(), aligned)?;
    if value.len() > 2 {
        if aligned {
            data.align();
        }
    }
    data.append_bits(value.as_bits());

    data.dump_encode();
    Ok(())
}

pub(crate) fn encode_object_identifier_common(
    data: &mut PerCodecData,
    value: &OBJECT_IDENTIFIER,
    aligned: bool,
) -> Result<(), PerCodecError> {
    let mut oid_out: Vec<u8> = Vec::new();
    let len = match x690_write_object_identifier_value(&mut oid_out, value) {
        Ok(bytes_written) => bytes_written,
        Err(_) => return Err(PerCodecError::new("Could not encode object identifier")),
    };
    encode_length_determinent_common(data, Some(0), None, false, len, aligned)?;
    data.bits.extend_from_raw_slice(&oid_out);
    data.dump_encode();
    Ok(())
}
