use asn1::{BOOLEAN, INTEGER, read_i64};
use bitvec::prelude::*;
use std::io::{Result, ErrorKind, Error};

pub type BitSink = BitVec<u8, Lsb0>;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

pub fn x691_write_boolean (out: &mut BitSink, value: BOOLEAN) -> Result<usize> {
    out.push(value);
    Ok(1)
}

// pub fn x691_write_non_negative_binary_integer (out: &mut BitSink, value: INTEGER) -> Result<usize> {

// }

// pub fn x691_write_twos_complement_binary_integer (out: &mut BitSink, value: INTEGER) -> Result<usize> {

// }

pub fn x691_write_constrained_whole_number (
    out: &mut BitSink,
    value: INTEGER,
    min: i64,
    max: i64,
    aligned: bool,
    value_outside_of_extension_root: bool,
) -> Result<usize> {
    let range = max - min + 1; // 11.5.3
    if range == 1 {
        return Ok(0);
    }
    let mut n = match read_i64(&value) {
        Ok(i) => i,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    let x = n - min;
    match range {
        2 => {
            if x == 1 {
                out.push(true);
            } else {
                out.push(false);
            }
            Ok(1)
        },
        3..=4 => {
            out.extend([
                if (0b0000_0010 & x) > 0 { true } else { false },
                if (0b0000_0001 & x) > 0 { true } else { false },
            ].iter());
            Ok(1)
        },
        5..=8 => {
            out.extend([
                if (0b0000_0100 & x) > 0 { true } else { false },
                if (0b0000_0010 & x) > 0 { true } else { false },
                if (0b0000_0001 & x) > 0 { true } else { false },
            ].iter());
            Ok(1)
        },
        9..=16 => {
            out.extend([
                if (0b0000_1000 & x) > 0 { true } else { false },
                if (0b0000_0100 & x) > 0 { true } else { false },
                if (0b0000_0010 & x) > 0 { true } else { false },
                if (0b0000_0001 & x) > 0 { true } else { false },
            ].iter());
            Ok(1)
        },
        17..=32 => {
            out.extend([
                if (0b0001_0000 & x) > 0 { true } else { false },
                if (0b0000_1000 & x) > 0 { true } else { false },
                if (0b0000_0100 & x) > 0 { true } else { false },
                if (0b0000_0010 & x) > 0 { true } else { false },
                if (0b0000_0001 & x) > 0 { true } else { false },
            ].iter());
            Ok(1)
        },
        33..=64 => {
            out.extend([
                if (0b0010_0000 & x) > 0 { true } else { false },
                if (0b0001_0000 & x) > 0 { true } else { false },
                if (0b0000_1000 & x) > 0 { true } else { false },
                if (0b0000_0100 & x) > 0 { true } else { false },
                if (0b0000_0010 & x) > 0 { true } else { false },
                if (0b0000_0001 & x) > 0 { true } else { false },
            ].iter());
            Ok(1)
        },
        65..=128 => {
            out.extend([
                if (0b0100_0000 & x) > 0 { true } else { false },
                if (0b0010_0000 & x) > 0 { true } else { false },
                if (0b0001_0000 & x) > 0 { true } else { false },
                if (0b0000_1000 & x) > 0 { true } else { false },
                if (0b0000_0100 & x) > 0 { true } else { false },
                if (0b0000_0010 & x) > 0 { true } else { false },
                if (0b0000_0001 & x) > 0 { true } else { false },
            ].iter());
            Ok(1)
        },
        129..=255 => {
            out.extend_from_raw_slice(&[x as i8 as u8]);
            Ok(1)
        },
        256 => {
            // I think this case is the exact case as for 255
            // if aligned && (out.len() % 8) > 0 {
            //     let bits_to_add = 8 - (out.len() % 8);
            //     // TODO: Add padding bits.
            // }
            out.extend_from_raw_slice(&[x as i8 as u8]);
            Ok(1)
        },
        257..=65536 => {
            out.extend_from_raw_slice(&(x as i16).to_be_bytes().as_slice());
            Ok(2)
        },
        65537..=i64::MAX => {

            // let min_octets =
            // FIXME:
            // 11.5.7.4 (The indefinite length case.) Otherwise, the value ("n" – "lb") shall be encoded as a non-negative-binary-integer
            // in a bit-field (octet-aligned in the ALIGNED variant) with the minimum number of octets as specified in 11.3, and the
            // number of octets "len" used in the encoding is used by other clauses that reference this subclause to specify an encoding
            // of the length.
            Ok(0)
        },
        // All other cases are "negative range," which means min and max were reversed.
        _ => return Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn x691_write_normally_small_non_negative_whole_number (out: &mut BitSink, value: INTEGER) -> Result<usize> {

}

pub fn x691_write_semi_constrained_whole_number (out: &mut BitSink, value: INTEGER) -> Result<usize> {

}

pub fn x691_write_unconstrained_whole_number (out: &mut BitSink, value: INTEGER) -> Result<usize> {

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
