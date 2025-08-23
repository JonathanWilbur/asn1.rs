

/// This uses a fixed-length buffer of 20 bytes, because NSAP addresses are
/// forbidden from exceeding 20 bytes, with an exception for URLs established in
/// ITU-T Rec. X.519. Despite this one exception, no decimal encoding of an NSAP
/// addresses that exceeds 20 bytes.
pub struct BCDBuffer {
    pub bytes: [u8; 20],
    pub i: u8,
}

impl BCDBuffer {

    pub fn new() -> Self {
        BCDBuffer { bytes: [0; 20], i: 0 }
    }

    pub fn push_digit_char(&mut self, c: char) {
        debug_assert!(c.is_ascii_digit(), "non-ascii digit passed into push_digit_char");
        // let ascii = unsafe { c.as_ascii_unchecked() };
        self.push_digit_u8(c as u8)
    }

    pub fn push_digit_u8(&mut self, b: u8) {
        debug_assert!(b.is_ascii_digit(), "non-ascii digit passed into push_digit_u8");
        let nybble: u8 = b.saturating_sub(0x30);
        self.push_nybble(nybble);
    }

    pub fn push_nybble(&mut self, n: u8) {
        let byte_index = self.i >> 1;
        if (self.i % 2) > 0 { // least significant nybble
            self.bytes[byte_index as usize] |= n;
        } else {
            self.bytes[byte_index as usize] |= n << 4;
        }
        self.i += 1;
        self.i = self.i.clamp(0, 39);
    }

    pub fn push_byte(&mut self, byte: u8) {
        let byte_index = self.len_in_bytes();
        self.bytes[byte_index] = byte;
        self.i += if (self.i % 2) > 0 { 3 } else { 2 };
        self.i = self.i.clamp(0, 39);
    }

    pub fn len_in_bytes(&self) -> usize {
        ((self.i >> 1) + (self.i % 2)) as usize
    }

}

impl AsRef<[u8]> for BCDBuffer {

    fn as_ref(&self) -> &[u8] {
        &self.bytes[0..self.len_in_bytes()]
    }

}

#[cfg(test)]
mod tests {
    use crate::bcd::BCDBuffer;

    #[test]
    fn test_bcd_buffer_1() {
        let mut bcd = BCDBuffer::new();
        bcd.push_digit_char('9');
        bcd.push_digit_u8(0x37);
        bcd.push_nybble(0x05);
        bcd.push_byte(0x33);
        assert_eq!(bcd.as_ref(), [ 0x97, 0x50, 0x33 ].as_slice());
    }

}
