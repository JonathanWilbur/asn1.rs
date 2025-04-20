use crate::types::BIT_STRING;
use std::{convert::TryInto, fmt::{Display, Write}};
use crate::utils::unlikely;

pub fn join_bit_strings(strs: &[BIT_STRING]) -> BIT_STRING {
    if unlikely(strs.len() == 0) {
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
        if (bit_index % 8) == 0 {
            // We are octet-aligned, so we can just do a byte-for-byte copy.
            ret.bytes.extend(&bs.bytes);
            bit_index += (bs.bytes.len() << 3) - bs.trailing_bits as usize;
        } else {
            // Otherwise, we have to account for the bit shift. Sad!
            let len = bs.bytes.len();
            // These should be the same for each iteration of the following loop.
            let trailing_bits = ((8 - (bit_index % 8)) % 8) as u8;
            let remaining_bits = (bit_index % 8) as u8;
            for byte in bs.bytes[0..len - 1].iter() {
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
            if bits_in_last_byte > bits_left_in_curr_byte {
                // We have to overflow to a next byte.
                ret.bytes.push(
                    last_byte
                        .overflowing_shl((8 - (bit_index % 8)).try_into().unwrap())
                        .0,
                );
            }
            bit_index += (bs.bytes.len() << 3) - bs.trailing_bits as usize;
        }
    }
    ret.trailing_bits = ((8 - (bit_index % 8)) % 8) as u8;
    if ret.bytes.len() > 0 {
        // Zero the remaining bytes
        let ret_len = ret.bytes.len();
        ret.bytes[ret_len - 1] &= 0xFFu8.overflowing_shl(ret.trailing_bits as u32).0;
    }
    ret
}

impl BIT_STRING {

    #[inline]
    pub fn new() -> Self {
        BIT_STRING::default()
    }

    #[inline]
    pub fn with_capacity(bits: usize) -> Self {
        BIT_STRING {
            bytes: Vec::with_capacity((bits >> 3) + 1),
            trailing_bits: 0,
        }
    }

    pub fn get(&mut self, index: usize) -> Option<bool> {
        let byte_index: usize = index >> 3;
        let bit_index: usize = index % 8;
        let byte = self.bytes.get(byte_index)?;
        let non_trailing_bits = 7 - (self.trailing_bits % 8);
        if bit_index > non_trailing_bits as usize {
            return None;
        }
        let masked = (1 << (7 - bit_index)) & byte;
        Some(masked > 0)
    }

    pub fn set(&mut self, index: usize, value: bool) -> bool {
        let byte_index: usize = index >> 3;
        let bit_index: usize = index % 8;
        let extra_bytes_needed: usize = (byte_index + 1) - self.bytes.len();
        let extended: bool =
            (extra_bytes_needed > 0) || (bit_index > (7 - self.trailing_bits).into());
        if extended {
            self.bytes.resize(self.bytes.len() + extra_bytes_needed, 0);
        }
        let len = self.bytes.len(); // See: https://stackoverflow.com/questions/30532628/cannot-borrow-as-immutable-string-and-len
        {
            // Zero remaining bits
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

    /// The length IN BITS of the bit string.
    #[inline]
    pub fn len_in_bits (&self) -> usize {
        (self.bytes.len() << 3)
            .checked_sub(self.trailing_bits as usize)
            .unwrap_or(0)
    }

    pub fn with_bits_set(bits_to_set: &[usize]) -> BIT_STRING {
        let mut bit_size: usize = 0;
        for bit in bits_to_set.iter() {
            if *bit > bit_size {
                bit_size = *bit;
            }
        }
        let byte_size = bit_size >> 3;
        let bytes: Vec<u8> = Vec::with_capacity(byte_size);
        let trailing_bits = (8 - (bit_size % 8)) as u8;
        let mut bs = BIT_STRING {
            bytes,
            trailing_bits,
        };
        for bit in bits_to_set.iter() {
            bs.set(*bit, true);
        }
        bs
    }

    pub fn from_bin(bitstr: &str) -> BIT_STRING {
        if unlikely(bitstr.len() == 0) {
            return BIT_STRING {
                bytes: vec![],
                trailing_bits: 0,
            };
        }
        let bit_size = bitstr.len();
        let bytes: Vec<u8> = Vec::with_capacity(bitstr.len() >> 3);
        let trailing_bits = (8 - (bit_size % 8)) as u8; // REVIEW: I am not sure this is right.
        let mut bs = BIT_STRING {
            bytes,
            trailing_bits,
        };
        let str_bytes = bitstr.as_bytes();
        for i in 0..bitstr.len() {
            let char = str_bytes[i];
            if char == '1' as u8 {
                bs.set(i, true);
            }
        }
        bs
    }

    pub fn from_bits(bits: &[u8]) -> BIT_STRING {
        let bit_size =  bits.len();
        let byte_size = bits.len() >> 3;
        let bytes: Vec<u8> = Vec::with_capacity(byte_size);
        let trailing_bits = (8 - (bit_size % 8)) as u8;
        let mut bs = BIT_STRING {
            bytes,
            trailing_bits,
        };
        for (i, bit) in bits.iter().enumerate() {
            bs.set(i, *bit > 0);
        }
        bs
    }

    #[inline]
    pub fn from_bytes(bytes: Vec<u8>) -> BIT_STRING {
        BIT_STRING { bytes, trailing_bits: 0 }
    }

}


impl PartialEq for BIT_STRING {

    fn eq(&self, other: &Self) -> bool {
        if self.trailing_bits != other.trailing_bits {
            return false;
        }
        // TODO: I can't seem to find a SIMD-accelerated compare
        if self.bytes == other.bytes {
            return true;
        }
        let trailing_bits = self.trailing_bits % 8; // Just to make sure.
        if trailing_bits == 0 {
            // Match MUST have failed because the whole bytes did not match.
            return false;
        }
        match (self.bytes.last(), other.bytes.last()) {
            (None, None) => true, // both empty
            (Some(a), Some(b)) => (*a >> trailing_bits) == (*b >> trailing_bits),
            (_, _) => false, // one empty, the other not
        }
    }

}

impl From<&[u8]> for BIT_STRING {
    fn from(other: &[u8]) -> Self {
        BIT_STRING {
            bytes: Vec::from(other.as_ref()),
            trailing_bits: 0,
        }
    }
}

impl From<Vec<u8>> for BIT_STRING {
    fn from(other: Vec<u8>) -> Self {
        BIT_STRING {
            bytes: other,
            trailing_bits: 0,
        }
    }
}

impl Default for BIT_STRING {
    fn default() -> Self {
        BIT_STRING {
            bytes: vec![],
            trailing_bits: 0,
        }
    }
}

fn write_bin(v: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let bins = v.iter().map(|b| format!("{:08b}", b).to_string());
    for bin in bins {
        f.write_str(bin.as_str())?;
    }
    Ok(())
}

impl Display for BIT_STRING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bytes.len() == 0 {
            return f.write_str("''B");
        }
        if self.trailing_bits == 0 {
            f.write_char('\'')?;
            write_bin(&self.bytes, f)?;
            return f.write_str("'B");
        }
        let len = self.bytes.len();

        // Print the trailing bits.
        let last_byte: u8 = self.bytes[len - 1];
        let (zero_mask, _) = (0b1111_1111 as u8).overflowing_shl(self.trailing_bits as u32);
        let last_byte_str = format!("{:08b}", last_byte & zero_mask).to_string();
        let last_bits = unsafe {
            std::str::from_utf8_unchecked(&last_byte_str.as_bytes()[0..8 - self.trailing_bits as usize])
        };

        f.write_char('\'')?;
        write_bin(&self.bytes[..len - 1], f)?;
        f.write_str(last_bits)?;
        f.write_str("'B")
    }
}

#[macro_export]
macro_rules! bits {
    ( $( $x:expr ),* ) => {
        {
            use super::BIT_STRING;
            BIT_STRING::from_bits(&[ $($x,)* ])
        }
    };
}

#[cfg(test)]
mod tests {

    use crate::{bitstring::join_bit_strings, types::BIT_STRING};

    #[test]
    fn test_bit_string_compare_1() {
        let bs1 = BIT_STRING::new();
        let bs2 = BIT_STRING::new();
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn test_bit_string_compare_2() {
        let bs1 = bits!(1,0,1,0,0,1,0,1,1,1,1);
        let bs2 = bits!(1,0,1,0,0,1,0,1,1,1,1);
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn test_bit_string_compare_3() {
        // These only differ by trailing bits
        let bs1 = BIT_STRING{
            bytes: vec![0b1111_0000, 0b1010_0111],
            trailing_bits: 3,
        };
        let bs2 = BIT_STRING{
            bytes: vec![0b1111_0000, 0b1010_0101],
            trailing_bits: 3,
        };
        assert_eq!(bs1, bs2);
    }


    #[test]
    fn test_bit_string_set_1() {
        let mut bs = BIT_STRING::new();
        bs.set(0, true);
        assert_eq!(bs.bytes.len(), 1);
        assert!(bs.bytes.starts_with(&[0b1000_0000]));
        assert_eq!(bs.trailing_bits, 7);
    }

    #[test]
    fn test_bit_string_set_2() {
        let mut bs = BIT_STRING {
            bytes: vec![0b0100_0000],
            trailing_bits: 5,
        };
        bs.set(6, true);
        assert_eq!(bs.bytes.len(), 1);
        assert_eq!(bs.bytes[0], 0b0100_0010);
        assert_eq!(bs.trailing_bits, 1);
    }

    #[test]
    fn test_bit_string_set_3() {
        let mut bs = BIT_STRING {
            bytes: vec![0b0100_0001],
            trailing_bits: 0,
        };
        bs.set(8, true);
        assert_eq!(bs.bytes.len(), 2);
        assert_eq!(bs.bytes[0], 0b0100_0001);
        assert_eq!(bs.bytes[1], 0b1000_0000);
        assert_eq!(bs.trailing_bits, 7);
    }

    #[test]
    fn test_join_bit_strings_1() {
        let bs1 = BIT_STRING::new();
        let bs2 = BIT_STRING::new();
        let bsout = join_bit_strings(&[bs1, bs2]);
        assert_eq!(bsout.bytes.len(), 0);
        assert_eq!(bsout.trailing_bits, 0);
    }

    #[test]
    fn test_join_bit_strings_2() {
        let bs1 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 0,
        };
        let bs2 = BIT_STRING::new();
        let bsout = join_bit_strings(&[bs1, bs2]);
        assert_eq!(bsout.bytes.len(), 2);
        assert_eq!(bsout.trailing_bits, 0);
    }

    #[test]
    fn test_join_bit_strings_3() {
        let bs1 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs2 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bsout = join_bit_strings(&[bs1, bs2]);
        assert_eq!(bsout.bytes.len(), 4);
        assert_eq!(bsout.trailing_bits, 2);
        assert!(bsout
            .bytes
            .starts_with(&[0b1010_0101, 0b1111_0111, 0b0100_1011, 0b1110_1100,]));
    }

    #[test]
    fn test_join_bit_strings_4() {
        let bs1 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs2 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs3 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bsout = join_bit_strings(&[bs1, bs2, bs3]);
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
    fn test_join_bit_strings_5() {
        let bs1 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0111],
            trailing_bits: 2,
        };
        let bs2 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs3 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bsout = join_bit_strings(&[bs1, bs2, bs3]);
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
    fn test_join_bit_strings_6() {
        let bs1 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0111],
            trailing_bits: 5,
        };
        let bs2 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 5,
        };
        let bs3 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 2,
        };
        let bsout = join_bit_strings(&[bs1, bs2, bs3]);
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

    #[test]
    fn test_bit_string_display() {
        let bs1 = BIT_STRING {
            bytes: vec![0b1010_0101, 0b1111_0111],
            trailing_bits: 5,
        };
        assert_eq!(bs1.to_string().as_str(), "'10100101111'B");
    }

    #[test]
    fn test_bits_macro() {
        let bs1 = bits!(1,0,1,0,0,1,0,1,1,1,1);
        // let bs1 = bits!(1010010111110111);
        assert_eq!(bs1.to_string().as_str(), "'10100101111'B");
    }

    #[test]
    fn test_bit_string_get() {
        let mut bs = BIT_STRING {
            bytes: vec![0b0100_0000],
            trailing_bits: 5,
        };
        assert_eq!(bs.get(0), Some(false));
        assert_eq!(bs.get(1), Some(true));
        assert_eq!(bs.get(4), None);
    }

    // No specific expectation for this test, other than that it does not
    // cause a panic.
    #[test]
    fn test_bit_string_get_malformed_1() {
        let mut bs = BIT_STRING {
            bytes: vec![0b0100_0000],
            trailing_bits: 65,
        };
        let bit = bs.get(1);
        assert_eq!(bit, Some(true));
    }

    #[test]
    fn test_bit_string_get_malformed_2() {
        let mut bs = BIT_STRING {
            bytes: vec![],
            trailing_bits: 3,
        };
        let bit = bs.get(1);
        assert_eq!(bit, None);
    }

    #[test]
    fn test_bit_string_len() {
        let bs = BIT_STRING {
            bytes: vec![0b0101_0000],
            trailing_bits: 3,
        };
        assert_eq!(bs.len_in_bits(), 5);
    }

}
