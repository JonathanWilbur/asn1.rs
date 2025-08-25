use core::iter::{Iterator, FusedIterator};

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

    pub fn push_str(&mut self, s: &str) {
        debug_assert!(s.is_ascii(), "non-ascii passed into BCDBuffer::push_str");
        s.bytes().for_each(|b| self.push_digit_u8(b));
    }

    pub fn push_ascii_bytes(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.is_ascii(), "non-ascii passed into BCDBuffer::push_ascii_bytes");
        bytes.iter().for_each(|b| self.push_digit_u8(*b));
    }

    // pub fn push_digit_char(&mut self, c: char) {
    //     debug_assert!(c.is_ascii_digit(), "non-ascii digit passed into BCDBuffer::push_digit_char");
    //     self.push_digit_u8(c as u8)
    // }

    pub fn push_digit_u8(&mut self, b: u8) {
        debug_assert!(b.is_ascii_digit(), "non-ascii digit passed into BCDBuffer::push_digit_u8");
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


/// BCD Digits Iterator
#[derive(Debug, Clone)]
pub struct BCDDigitsIter<'a> {
    idi: &'a [u8],
    least_sig_nybble: bool,
    leading_0_sig: bool,
    processing_leading_digits: bool,
    ignore_last_nybble: bool,
}

impl <'a> BCDDigitsIter<'a> {

    #[inline]
    pub fn new(
        idi: &'a [u8],
        leading_0_sig: bool,
        ignore_last_nybble: bool,
        least_sig_nybble: bool,
        processing_leading_digits: bool,
    ) -> BCDDigitsIter<'a> {
        BCDDigitsIter{
            idi,
            leading_0_sig,
            ignore_last_nybble,
            processing_leading_digits, // Start off handling leading digits
            least_sig_nybble, // Start off on the MSn
        }
    }

}

/// This SHOULD BE an ASCII digit, but might not be. It is on the caller to
/// check this and determine what to do if this has a non-digit value.
pub type ShouldBeASCIIDigit = u8;

impl <'a> Iterator for BCDDigitsIter<'a> {
    type Item = ShouldBeASCIIDigit;

    /// This implementation does NOT handle malformed digits. The caller MUST
    /// check for non-ASCII digits being returned
    fn next(&mut self) -> Option<Self::Item> {
        while self.idi.len() > 0 {
            let nybble: u8 = if self.least_sig_nybble {
                self.idi[0] & 0b0000_1111
            } else {
                (self.idi[0] & 0b1111_0000) >> 4
            };
            if self.least_sig_nybble {
                self.least_sig_nybble = false;
                self.idi = &self.idi[1..];
            } else {
                self.least_sig_nybble = true;
            }
            if self.processing_leading_digits {
                let leading_digit: u8 = if self.leading_0_sig { 1 } else { 0 };
                if nybble == leading_digit {
                    continue;
                } else {
                    self.processing_leading_digits = false;
                }
            }
            // If the last nybble is 0b1111, it is padding.
            // If the DSP is in decimal digits, the last nybble of the
            if self.idi.len() == 0 && (nybble == 0b1111 || self.ignore_last_nybble) {
                return None;
            }
            return Some(nybble);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut max_digits = self.idi.len() << 1; // Double it
        if self.least_sig_nybble {
            max_digits = max_digits.saturating_sub(1);
        }
        if self.ignore_last_nybble {
            max_digits = max_digits.saturating_sub(1);
        }
        // Every digit could be a leading digit
        (0, Some(max_digits))
    }
}

impl <'a> FusedIterator for BCDDigitsIter<'a> {}

#[cfg(test)]
mod tests {
    use crate::bcd::BCDBuffer;

    #[test]
    fn test_bcd_buffer_1() {
        let mut bcd = BCDBuffer::new();
        // bcd.push_digit_char('9');
        bcd.push_digit_u8(0x39);
        bcd.push_digit_u8(0x37);
        bcd.push_nybble(0x05);
        bcd.push_byte(0x33);
        assert_eq!(bcd.as_ref(), [ 0x97, 0x50, 0x33 ].as_slice());
    }

}
