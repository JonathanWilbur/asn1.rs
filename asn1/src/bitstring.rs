use smallvec::{SmallVec, smallvec};
use crate::{types::{BIT_STRING}};
use std::{convert::TryInto, fmt::{Display, Write}};
use crate::utils::unlikely;
use std::hash::{Hash, Hasher};

/// Combine `BIT STRING`s into one bigger `BIT STRING`
pub fn join_bit_strings(strs: &[BIT_STRING]) -> BIT_STRING {
    if unlikely(strs.len() == 0) {
        return BIT_STRING::new();
    }
    if unlikely(strs.len() == 1) {
        return strs[0].clone();
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
    #[cfg(feature = "smallvec")]
    ret.bytes.extend_from_slice(&strs[0].bytes);
    #[cfg(not(feature = "smallvec"))]
    ret.bytes.extend(&strs[0].bytes);

    // let bit_debt: usize = strs[0].trailing_bits as usize;
    let mut bit_index: usize = (strs[0].bytes.len() << 3).saturating_sub((strs[0].trailing_bits % 8) as usize);
    for bs in strs[1..].iter() {
        // This check prevents a panic at 7dbfa994-2139-44a4-a94a-14b0fc9c3470
        if bs.bytes.len() == 0 {
            continue; // Ignore zero-length BIT STRINGs. They have no effect.
        }
        if (bit_index % 8) == 0 {
            // We are octet-aligned, so we can just do a byte-for-byte copy.
            #[cfg(feature = "smallvec")]
            ret.bytes.extend_from_slice(&bs.bytes);
            #[cfg(not(feature = "smallvec"))]
            ret.bytes.extend(&bs.bytes);
            bit_index += (bs.bytes.len() << 3) - bs.trailing_bits as usize;
        } else {
            // Otherwise, we have to account for the bit shift. Sad!
            let len = bs.bytes.len();
            // These should be the same for each iteration of the following loop.
            let trailing_bits = ((8 - (bit_index % 8)) % 8) as u8;
            let remaining_bits = (bit_index % 8) as u8;
            // 7dbfa994-2139-44a4-a94a-14b0fc9c3470
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
    debug_assert!(ret.trailing_bits < 8);
    ret
}

const fn calculate_trailing_bits(total_bits: usize) -> u8 {
    let used_bits_in_last_byte = total_bits % 8;
    if used_bits_in_last_byte == 0 && total_bits != 0 {
        0
    } else {
        (8 - used_bits_in_last_byte) as u8
    }
}

impl BIT_STRING {

    /// Create a new empty `BIT STRING`
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `BIT STRING` with enough capacity to store `bits` number of
    /// bits without any further heap allocations.
    #[inline]
    pub fn with_capacity(bits: usize) -> Self {
        #[cfg(feature = "smallvec")]
        return BIT_STRING {
            bytes: SmallVec::with_capacity((bits >> 3) + 1),
            trailing_bits: 0,
        };
        #[cfg(not(feature = "smallvec"))]
        return BIT_STRING {
            bytes: Vec::with_capacity((bits >> 3) + 1),
            trailing_bits: 0,
        };
    }

    /// Get a bit at the specified `index`, where `index` is zero-indexed.
    pub fn get(&self, index: usize) -> Option<bool> {
        let byte_index: usize = index >> 3; // This is faster "divide by 8"
        let bit_index: usize = index % 8;
        let byte = self.bytes.get(byte_index)?;
        let non_trailing_bits = 7 - (self.trailing_bits % 8);
        // We only check to see if we're outside of the range of bits if we're
        // on the last byte.
        if (byte_index == (self.bytes.len() - 1)) && bit_index > non_trailing_bits as usize {
            return None;
        }
        let masked = (1 << (7 - bit_index)) & byte;
        Some(masked > 0)
    }

    /// Set (or unset) a bit at the specified `index`, where `index` is zero-indexed.
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        let mut len = self.bytes.len();
        let byte_index: usize = index >> 3;
        let bit_index: usize = index % 8;
        let extra_bytes_needed: usize = (byte_index + 1).saturating_sub(self.bytes.len());
        // The bit string is extended if...
        let extended: bool = (extra_bytes_needed > 0) // we had to write more bytes to the end...
            || ( // or we are...
                (byte_index == len - 1) // writing to the last byte...
                && (bit_index > ((8 - self.trailing_bits as usize) - 1)) // past the very last bit
            );
        if extra_bytes_needed > 0 {
            self.bytes.resize(self.bytes.len() + extra_bytes_needed, 0);
            len = self.bytes.len();
        }
        if len > 0 && self.trailing_bits > 0 {
            let zeroing_mask: u8 = 0xFFu8.unbounded_shl(self.trailing_bits as u32);
            self.bytes[len - 1] &= zeroing_mask;
        }
        if value {
            let mask: u8 = 1 << (7 - bit_index);
            self.bytes[byte_index] |= mask;
        } else {
            let mask: u8 = !(1 << (7 - bit_index));
            self.bytes[byte_index] &= mask;
        }
        if extended {
            self.trailing_bits = (7 - bit_index) as u8;
        }
        debug_assert!(self.trailing_bits < 8);
        extended
    }

    /// The length IN BITS of the `BIT STRING`
    #[inline]
    pub fn len_in_bits (&self) -> usize {
        (self.bytes.len() << 3)
            .checked_sub(self.trailing_bits as usize)
            .unwrap_or(0)
    }

    /// The length IN BYTES of the `BIT STRING`
    #[inline]
    pub fn len_in_bytes (&self) -> usize {
        self.bytes.len()
    }

    /// Get the number of trailing bits
    #[inline]
    pub fn get_trailing_bits_count (&self) -> u8 {
        self.trailing_bits & 7 // More efficient than % 8
    }

    /// Get an immutable reference to the bytes on which the bits are stored.
    /// Bit zero is guaranteed to be the most significant bit of the first byte.
    #[inline]
    pub fn get_bytes_ref (&self) -> &[u8] {
        &self.bytes
    }

    /// Get a mutable reference to the bytes on which the bits are stored.
    /// Bit zero is guaranteed to be the most significant bit of the first byte.
    #[inline]
    pub fn get_bytes_ref_mut (&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Create a new `BIT STRING` that is big enough to accommodate all of the
    /// bits being set from `bits_to_set` and set those bits. The resulting
    /// `BIT STRING` will have no trailing zeroes (i.e. the last bit will
    /// always be a 1 unless `bits_to_set` is empty). The values of
    /// `bits_to_set` are indexes into the `BIT STRING`, with `0` being the
    /// first bit.
    pub fn with_bits_set(bits_to_set: &[usize]) -> BIT_STRING {
        let mut bit_size: usize = 0;
        for bit in bits_to_set.iter() {
            if *bit > bit_size {
                bit_size = *bit;
            }
        }
        let trailing_bits = (7 - (bit_size % 8)) as u8;
        #[cfg(feature = "smallvec")]
        let mut bs = BIT_STRING {
            bytes: SmallVec::with_capacity(std::mem::size_of::<usize>()),
            trailing_bits,
        };
        #[cfg(not(feature = "smallvec"))]
        let mut bs = BIT_STRING {
            bytes: Vec::with_capacity(std::mem::size_of::<usize>()),
            trailing_bits,
        };
        for bit in bits_to_set.iter() {
            bs.set(*bit, true);
        }
        debug_assert!(bs.trailing_bits < 8);
        bs
    }

    /// Convert a binary string (only '1' and '0' ASCII characters) into a
    /// `BIT STRING`. In this implementation, only '1' is used to set bits and
    /// all other characters are treated as though they were '0'. This function
    /// is infallible and assumes that the input is truly only ones and zeroes.
    pub fn from_bin(bitstr: &str) -> BIT_STRING {
        if unlikely(bitstr.len() == 0) {
            #[cfg(feature = "smallvec")]
            return BIT_STRING {
                bytes: smallvec![],
                trailing_bits: 0,
            };
            #[cfg(not(feature = "smallvec"))]
            return BIT_STRING {
                bytes: vec![],
                trailing_bits: 0,
            };
        }
        let bit_size = bitstr.len();
        let bytes_len = (bitstr.len() >> 3) + if (bitstr.len() % 8) > 0 { 1 } else { 0 };

        #[cfg(feature = "smallvec")]
        let mut bs = BIT_STRING {
            bytes: smallvec![0; bytes_len],
            trailing_bits: 0,
        };
        #[cfg(not(feature = "smallvec"))]
        let mut bs = BIT_STRING {
            bytes: vec![0; bytes_len],
            trailing_bits: 0,
        };

        for (i, c) in bitstr.chars().enumerate() {
            if c == '1' {
                bs.set(i, true);
            }
        }
        if bytes_len > 0 {
            bs.trailing_bits = calculate_trailing_bits(bit_size);
        }
        debug_assert!(bs.trailing_bits < 8);
        bs
    }

    /// Create a new `BIT STRING` out of `1`s and `0`s. This was implemented
    /// solely for the `bits!` macro, but you could use it if you want.
    pub fn from_bits(bits: &[u8]) -> BIT_STRING {
        let bit_size = bits.len();
        let byte_size = (bits.len() >> 3) + if (bit_size % 8) > 0 { 1 } else { 0 };
        #[cfg(feature = "smallvec")]
        let mut bs = BIT_STRING {
            bytes: SmallVec::with_capacity(byte_size),
            trailing_bits: 0,
        };
        #[cfg(not(feature = "smallvec"))]
        let mut bs = BIT_STRING {
            bytes: vec![0; byte_size],
            trailing_bits: 0,
        };

        for (i, bit) in bits.iter().enumerate() {
            debug_assert!(*bit == 1 || *bit == 0);
            bs.set(i, *bit > 0);
        }
        if byte_size > 0 {
            bs.trailing_bits = calculate_trailing_bits(bit_size);
        }
        debug_assert!(bs.trailing_bits < 8);
        bs
    }

    /// Consume the bytes passed in to produce a `BIT STRING` that has no
    /// trailing bits.
    #[inline]
    pub fn from_bytes(bytes: Vec<u8>) -> BIT_STRING {
        #[cfg(feature = "smallvec")]
        return BIT_STRING { bytes: bytes.into(), trailing_bits: 0 };
        #[cfg(not(feature = "smallvec"))]
        return BIT_STRING { bytes, trailing_bits: 0 };
    }

    /// Create a `BIT STRING` from borrowed bytes.
    #[inline]
    pub fn from_parts_borrowed(bytes: &[u8], trailing_bits: u8) -> BIT_STRING {
        #[cfg(feature = "smallvec")]
        return BIT_STRING { bytes: bytes.into(), trailing_bits };
        #[cfg(not(feature = "smallvec"))]
        return BIT_STRING { bytes: bytes.to_owned(), trailing_bits };
    }

    /// Create a `BIT STRING` from owned bytes.
    /// If the `smallvec` feature is enabled, this doesn't really incur much of
    /// a performance penalty: `smallvec` is smart enough to re-use the
    /// `Vec<u8>`'s pointer rather than copying it, unless it needs to be
    /// copied into the inline space.
    #[inline]
    pub fn from_parts_owned(bytes: Vec<u8>, trailing_bits: u8) -> BIT_STRING {
        #[cfg(feature = "smallvec")]
        return BIT_STRING { bytes: SmallVec::from_vec(bytes), trailing_bits };
        #[cfg(not(feature = "smallvec"))]
        return BIT_STRING { bytes, trailing_bits };
    }

}

// This trait MUST NOT be implemented for `BIT STRING` because its size will
// vary depending on whether BER, CER, or DER is used.
// impl X690KnownSize for BIT_STRING {}

impl PartialEq for BIT_STRING {

    /// Compare bit strings, bit by bit, ignoring the trailing bits in the
    /// comparison as one would expect.
    fn eq(&self, other: &Self) -> bool {
        if self.trailing_bits != other.trailing_bits {
            return false;
        }
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

    /// Make a new `BIT STRING` using an integral number of octets, which will
    /// have no trailing bits.
    #[inline]
    fn from(other: &[u8]) -> Self {
        #[cfg(feature = "smallvec")]
        return BIT_STRING {
            bytes: SmallVec::from_slice(other),
            trailing_bits: 0,
        };
        #[cfg(not(feature = "smallvec"))]
        return BIT_STRING {
            bytes: other.to_owned(),
            trailing_bits: 0,
        };
    }
}

impl From<Vec<u8>> for BIT_STRING {

    /// Make a new `BIT STRING` using an integral number of octets, which will
    /// have no trailing bits.
    fn from(other: Vec<u8>) -> Self {
        BIT_STRING::from_bytes(other)
    }
}

impl Default for BIT_STRING {

    /// Create an empty `BIT STRING`
    #[inline]
    fn default() -> Self {
        #[cfg(feature = "smallvec")]
        return BIT_STRING {
            bytes: smallvec![],
            trailing_bits: 0,
        };
        #[cfg(not(feature = "smallvec"))]
        return BIT_STRING {
            bytes: vec![],
            trailing_bits: 0,
        };
    }
}

fn write_bin(v: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    v.iter()
        .map(|b| format!("{:08b}", b))
        .try_for_each(|s| f.write_str(s.as_str()))
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

        f.write_char('\'')?;
        write_bin(&self.bytes[0..len-1], f)?;
        format!("{:08b}", last_byte)
            .chars()
            .take(8u8.saturating_sub(self.trailing_bits) as usize)
            .try_for_each(|c| f.write_char(c))?;
        f.write_str("'B")
    }
}

impl Hash for BIT_STRING {

    /// Implementation generated by ChatGPT 4o.
    fn hash<H: Hasher>(&self, state: &mut H) {
        let len = self.bytes.len();

        if len == 0 || self.trailing_bits == 0 {
            // Empty bit string
            0u8.hash(state);
            return;
        }

        // Hash all full bytes
        for b in &self.bytes[..len - 1] {
            b.hash(state);
        }

        // Mask and hash the final partial byte
        let final_byte = self.bytes[len - 1];
        let mask = 0xFF << (8 - self.trailing_bits);
        let masked_byte = final_byte & (mask as u8);

        masked_byte.hash(state);
        self.trailing_bits.hash(state); // Hash the trailing_bits too to disambiguate
    }
}

/// Macro to define a bit string from raw bits.
///
/// **HELP WANTED**: I would love it if somebody could make this require 1s and
/// 0s at the macro level, rather than evaluating them at run-time.
///
/// ```rust
/// let bs1 = asn1::bits!(1,0,1,0,0,1,0,1,1,1,1);
/// assert_eq!(bs1.to_string().as_str(), "'10100101111'B");
/// ```
///
#[macro_export]
macro_rules! bits {
    () => {
        {
            use $crate::BIT_STRING;
            BIT_STRING::new()
        }
    };
    ( $( $x:expr ),+ ) => {
        {
            use $crate::BIT_STRING;
            BIT_STRING::from_bits(&[ $($x,)* ])
        }
    };
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "smallvec")]
    use smallvec::smallvec;
    use std::usize;

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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1111_0000, 0b1010_0111],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1111_0000, 0b1010_0111],
            trailing_bits: 3,
        };
        let bs2 = BIT_STRING{
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1111_0000, 0b1010_0101],
            #[cfg(not(feature = "smallvec"))]
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
        assert_eq!(bs.bytes.as_slice(), &[0b1000_0000]);
        assert_eq!(bs.trailing_bits, 7);
    }

    #[test]
    fn test_bit_string_set_2() {
        let mut bs = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b0100_0000],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b0100_0001],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs2 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs2 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs3 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0111],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0111],
            trailing_bits: 2,
        };
        let bs2 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 1,
        };
        let bs3 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0111],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0111],
            trailing_bits: 5,
        };
        let bs2 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0110],
            trailing_bits: 5,
        };
        let bs3 = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0110],
            #[cfg(not(feature = "smallvec"))]
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
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b1010_0101, 0b1111_0111],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b1010_0101, 0b1111_0111],
            trailing_bits: 5,
        };
        assert_eq!(bs1.to_string().as_str(), "'10100101111'B");
    }

    #[test]
    fn test_bits_macro() {
        let bs1 = bits!(1,0,1,0,0,1,0,1,1,1,1);
        assert_eq!(bs1.to_string().as_str(), "'10100101111'B");
    }

    #[test]
    fn test_bit_string_get() {
        let bs = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b0100_0000],
            #[cfg(not(feature = "smallvec"))]
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
        let bs = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b0100_0000],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b0100_0000],
            trailing_bits: 65,
        };
        let bit = bs.get(1);
        assert_eq!(bit, Some(true));
    }

    #[test]
    fn test_bit_string_get_malformed_2() {
        let bs = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![],
            trailing_bits: 3,
        };
        let bit = bs.get(1);
        assert_eq!(bit, None);
    }

    #[test]
    fn test_bit_string_len() {
        let bs = BIT_STRING {
            #[cfg(feature = "smallvec")]
            bytes: smallvec![0b0101_0000],
            #[cfg(not(feature = "smallvec"))]
            bytes: vec![0b0101_0000],
            trailing_bits: 3,
        };
        assert_eq!(bs.len_in_bits(), 5);
    }

    #[test]
    fn test_bit_string_from_bin() {
        let bs = BIT_STRING::from_bin("00001010");
        assert_eq!(bs.len_in_bits(), 8);
        assert_eq!(bs.bytes.as_slice(), &[ 0b0000_1010 ]);
        assert_eq!(bs.trailing_bits, 0);
    }

    #[test]
    fn test_bit_string_reg_1() {
        let bs1 = BIT_STRING::from_bin("11111100");
        assert_eq!(bs1.to_string().as_str(), "'11111100'B");
    }

    #[test]
    fn test_bit_string_reg_2() {
        let bs1 = BIT_STRING::from_bin("00000000");
        assert_eq!(bs1.to_string().as_str(), "'00000000'B");
    }


    #[test]
    fn test_with_bits_set_1() {
        let bs1 = BIT_STRING::with_bits_set(&[ 1, 3, 5, 7, 8, 9, 13 ]);
        assert_eq!(bs1.to_string().as_str(), "'01010101110001'B");
    }

    #[test]
    fn test_with_bits_set_2() {
        let bs1 = BIT_STRING::with_bits_set(&[]);
        assert_eq!(bs1.to_string().as_str(), "''B");
    }

    #[test]
    fn test_bit_string_get_1() {
        let bs1 = BIT_STRING::with_bits_set(&[ 1, 3, 5, 7, 8, 9, 13 ]);
        assert_eq!(bs1.get(0),  Some(false));
        assert_eq!(bs1.get(1),  Some(true));
        assert_eq!(bs1.get(2),  Some(false));
        assert_eq!(bs1.get(3),  Some(true));
        assert_eq!(bs1.get(4),  Some(false));
        assert_eq!(bs1.get(5),  Some(true));
        assert_eq!(bs1.get(6),  Some(false));
        assert_eq!(bs1.get(7),  Some(true));
        assert_eq!(bs1.get(8),  Some(true));
        assert_eq!(bs1.get(9),  Some(true));
        assert_eq!(bs1.get(10), Some(false));
        assert_eq!(bs1.get(11), Some(false));
        assert_eq!(bs1.get(12), Some(false));
        assert_eq!(bs1.get(13), Some(true));
    }

    #[test]
    fn test_bit_string_get_2() {
        let bs1 = BIT_STRING::with_bits_set(&[ 0, 2, 4, 6, 8, 10 ]);
        assert_eq!(bs1.get(0),  Some(true));
        assert_eq!(bs1.get(1),  Some(false));
        assert_eq!(bs1.get(2),  Some(true));
        assert_eq!(bs1.get(3),  Some(false));
        assert_eq!(bs1.get(4),  Some(true));
        assert_eq!(bs1.get(5),  Some(false));
        assert_eq!(bs1.get(6),  Some(true));
        assert_eq!(bs1.get(7),  Some(false));
        assert_eq!(bs1.get(8),  Some(true));
        assert_eq!(bs1.get(9),  Some(false));
    }

    #[test]
    fn test_bit_string_get_3() {
        let bs1 = BIT_STRING::with_bits_set(&[ 0 ]);
        assert_eq!(bs1.get(usize::MAX), None);
    }

    #[test]
    fn test_bit_string_set_4() {
        let mut bs1 = BIT_STRING::new();
        bs1.set(0, true);
        assert_eq!(bs1.get(0), Some(true));
    }


}
