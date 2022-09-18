use crate::types::{BIT_STRING};
use std::convert::TryInto;

pub fn join_bit_strings (strs: &[BIT_STRING]) -> BIT_STRING {
    if strs.len() == 0 {
        return BIT_STRING::new();
    }
    // This calculation does not consider trailing bits, both for simplicity,
    // and because doing so might save us only a few bytes of memory at the risk
    // of miscalculations / bugs resulting in reallocation.
    let mut needed_capacity: usize = 0;
    for bs in strs {
        needed_capacity += bs.bytes.len();
    }
    let mut ret: BIT_STRING = BIT_STRING::with_capacity(needed_capacity << 3);
    // The first BIT STRING can be mem-copied in directly, since it always
    // starts off 0-aligned.
    ret.bytes.extend(&strs[0].bytes);

    // let bit_debt: usize = strs[0].trailing_bits as usize;
    let mut bit_index: usize = (strs[0].bytes.len() << 3) - strs[0].trailing_bits as usize;
    for bs in strs[1..].iter() {
        if (bit_index % 8) == 0 { // We are octet-aligned, so we can just do a byte-for-byte copy.
            ret.bytes.extend(&bs.bytes);
            bit_index += (bs.bytes.len() << 3) - bs.trailing_bits as usize;
        } else { // Otherwise, we have to account for the bit shift. Sad!
            let len = bs.bytes.len();
            // These should be the same for each iteration of the following loop.
            let trailing_bits = ((8 - (bit_index % 8)) % 8) as u8;
            let remaining_bits = (bit_index % 8) as u8;
            for byte in bs.bytes[0..len-1].iter() {
                let prev_byte_mask: u8 = byte >> remaining_bits;
                let curr_byte_mask: u8 = byte << trailing_bits;
                let ret_len = ret.bytes.len();
                ret.bytes[ret_len - 1] &= 0xFFu8.overflowing_shl(trailing_bits as u32).0;
                ret.bytes[ret_len - 1] |= prev_byte_mask;
                ret.bytes.push(curr_byte_mask);
            }
            let last_byte = bs.bytes[bs.bytes.len() - 1];
            let bits_in_last_byte = (8 - (bs.trailing_bits % 8)) % 8;
            let bits_left_in_curr_byte = trailing_bits;
            let last_byte_mask = last_byte.overflowing_shr(remaining_bits as u32).0;
            let ret_len = ret.bytes.len();
            ret.bytes[ret_len - 1] &= 0xFFu8.overflowing_shl(trailing_bits as u32).0;
            ret.bytes[ret_len - 1] |= last_byte_mask;
            if bits_in_last_byte > bits_left_in_curr_byte { // We have to overflow to a next byte.
                ret.bytes.push(last_byte.overflowing_shl((8 - (bit_index % 8)).try_into().unwrap()).0);
            }
            bit_index += (bs.bytes.len() << 3) - bs.trailing_bits as usize;
        }
    }
    ret.trailing_bits = ((8 - (bit_index % 8)) % 8) as u8;
    if ret.bytes.len() > 0 { // Zero the remaining bytes
        let ret_len = ret.bytes.len();
        ret.bytes[ret_len - 1] &= 0xFFu8.overflowing_shl(ret.trailing_bits as u32).0;
    }
    ret
}

impl BIT_STRING {

    pub fn new () -> Self {
        BIT_STRING { bytes: Vec::new(), trailing_bits: 0 }
    }

    pub fn with_capacity (bits: usize) -> Self {
        BIT_STRING { bytes: Vec::with_capacity(bits >> 3), trailing_bits: 0 }
    }

    pub fn set (&mut self, index: usize, value: bool) -> bool {
        let byte_index: usize = index >> 3;
        let bit_index: usize = index % 8;
        let extra_bytes_needed: usize = (byte_index + 1) - self.bytes.len();
        let extended: bool = (extra_bytes_needed > 0) || (bit_index > (7 - self.trailing_bits).into());
        for _ in 0..extra_bytes_needed {
            self.bytes.push(0);
        }
        let len = self.bytes.len(); // See: https://stackoverflow.com/questions/30532628/cannot-borrow-as-immutable-string-and-len
        { // Zero remaining bits
            let zeroing_mask: u8 = !(0xFFu8.overflowing_shr((8 - self.trailing_bits).into()).0);
            self.bytes[len - 1] &= zeroing_mask;
        }
        if value {
            let mask: u8 = 1 << (7 - bit_index);
            self.bytes[len - 1] |= mask;
        } else {
            let mask: u8 = !(1 << (7 - bit_index));
            self.bytes[len - 1] &= mask;
        }
        if extended {
            self.trailing_bits = (7 - bit_index) as u8;
        }
        extended
    }

}

#[cfg(test)]
mod tests {

    // use super::*;

    use crate::{types::BIT_STRING, bitstring::join_bit_strings};

    #[test]
    fn test_bit_string_set_1 () {
        let mut bs = BIT_STRING::new();
        bs.set(0, true);
        assert_eq!(bs.bytes.len(), 1);
        assert!(bs.bytes.starts_with(&[ 0b1000_0000 ]));
        assert_eq!(bs.trailing_bits, 7);
    }

    #[test]
    fn test_bit_string_set_2 () {
        let mut bs = BIT_STRING {
            bytes: vec![ 0b0100_0000 ],
            trailing_bits: 5,
        };
        bs.set(6, true);
        assert_eq!(bs.bytes.len(), 1);
        assert_eq!(bs.bytes[0], 0b0100_0010);
        assert_eq!(bs.trailing_bits, 1);
    }

    #[test]
    fn test_bit_string_set_3 () {
        let mut bs = BIT_STRING {
            bytes: vec![ 0b0100_0001 ],
            trailing_bits: 0,
        };
        bs.set(8, true);
        assert_eq!(bs.bytes.len(), 2);
        assert_eq!(bs.bytes[0], 0b0100_0001);
        assert_eq!(bs.bytes[1], 0b1000_0000);
        assert_eq!(bs.trailing_bits, 7);
    }

    #[test]
    fn test_join_bit_strings_1 () {
        let bs1 = BIT_STRING::new();
        let bs2 = BIT_STRING::new();
        let bsout = join_bit_strings(&[ bs1, bs2 ]);
        assert_eq!(bsout.bytes.len(), 0);
        assert_eq!(bsout.trailing_bits, 0);
    }

    #[test]
    fn test_join_bit_strings_2 () {
        let bs1 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 0,
        };
        let bs2 = BIT_STRING::new();
        let bsout = join_bit_strings(&[ bs1, bs2 ]);
        assert_eq!(bsout.bytes.len(), 2);
        assert_eq!(bsout.trailing_bits, 0);
    }

    #[test]
    fn test_join_bit_strings_3 () {
        let bs1 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bs2 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bsout = join_bit_strings(&[ bs1, bs2 ]);
        assert_eq!(bsout.bytes.len(), 4);
        assert_eq!(bsout.trailing_bits, 2);
        assert!(bsout.bytes.starts_with(&[
            0b1010_0101,
            0b1111_0111,
            0b0100_1011,
            0b1110_1100,
        ]));
    }

    #[test]
    fn test_join_bit_strings_4 () {
        let bs1 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bs2 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bs3 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bsout = join_bit_strings(&[ bs1, bs2, bs3 ]);
        assert_eq!(bsout.bytes.len(), 6);
        assert_eq!(bsout.trailing_bits, 3);
        assert!(bsout.bytes.starts_with(&[
            0b1010_0101,
            0b1111_0111,
            0b0100_1011,
            0b1110_1110,
            0b10_010111,
            0b11_011000,
        ]));
    }

    #[test]
    fn test_join_bit_strings_5 () {
        let bs1 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0111 ],
            trailing_bits: 2,
        };
        let bs2 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bs3 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 1,
        };
        let bsout = join_bit_strings(&[ bs1, bs2, bs3 ]);
        assert_eq!(bsout.bytes.len(), 6);
        assert_eq!(bsout.trailing_bits, 4);
        // assert_eq!(bsout.bytes[2], 0b1010_0111);
        assert!(bsout.bytes.starts_with(&[
            0b1010_0101,
            0b1111_0110,
            0b1001_0111,
            0b1101_1101,
            0b0010_1111,
            0b1011_0000,
        ]));
    }

    #[test]
    fn test_join_bit_strings_6 () {
        let bs1 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0111 ],
            trailing_bits: 5,
        };
        let bs2 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 5,
        };
        let bs3 = BIT_STRING {
            bytes: vec![ 0b1010_0101, 0b1111_0110 ],
            trailing_bits: 2,
        };
        let bsout = join_bit_strings(&[ bs1, bs2, bs3 ]);
        assert_eq!(bsout.bytes.len(), 5);
        assert_eq!(bsout.trailing_bits, 4);
        assert!(bsout.bytes.starts_with(&[
            0b1010_0101,
            0b1111_0100,
            0b1011_1110,
            0b1001_0111,
            0b1101_0000,
        ]));
    }
}