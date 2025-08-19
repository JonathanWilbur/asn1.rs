#![doc = include_str!("../README.md")]
#![no_std]
use core::iter::{Iterator, FusedIterator};

/// Absolute smallest possible valid TPKT
pub const MIN_TPKT_SIZE: usize = 4;

/// Value of the `vrsn` field of a TPKT
pub type TpktVersion = u8;

// One wonders: what made such a simple protocol go through two revisions?
/// TPKT Version 3
pub const TPKT_VERSION_3: TpktVersion = 3;

/// ISO Transport over TCP (ITOT) TPKT
pub struct Tpkt<'a> {
    /// The version field
    pub vrsn: u8,
    /// The reserved byte
    pub reserved: u8,
    /// The TPDU
    pub tpdu: &'a [u8],
}

/// Iterator that reads TPKT packets from a buffer, such as a buffer of bytes
/// read in from a TCP socket.
pub struct TpktIter <'a> {
    buffer: &'a [u8],
}

impl <'a> TpktIter<'a> {

    /// Create a new TPKT iterator
    pub fn new(buffer: &'a [u8]) -> TpktIter<'a> {
        TpktIter { buffer }
    }

    /// Bytes in the buffer not yet read
    pub fn bytes_unread(&self) -> usize {
        self.buffer.len()
    }

}

impl <'a> Iterator for TpktIter<'a> {
    type Item = Tpkt<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.len() < MIN_TPKT_SIZE {
            return None;
        }
        let vrsn: u8 = self.buffer[0];
        let reserved: u8 = self.buffer[1];
        let len: u16 = u16::from_be_bytes([
            self.buffer[2],
            self.buffer[3],
        ]);
        let tpdu = self.buffer.get(MIN_TPKT_SIZE..MIN_TPKT_SIZE + len as usize)?;
        self.buffer = &self.buffer[MIN_TPKT_SIZE + len as usize..];
        Some(Tpkt {
            vrsn,
            reserved,
            tpdu,
        })
    }

}

impl <'a> FusedIterator for TpktIter<'a> {}

#[cfg(test)]
mod tests {

    use super::TpktIter;

    #[test]
    fn test_tpkt_iter_1() {
        let buffer = [ 3u8, 0, 0, 5, 1, 2, 3, 4, 5 ];
        let mut tpkts = TpktIter::new(buffer.as_slice());
        let tpkt = tpkts.next().unwrap();
        assert_eq!(tpkt.vrsn, 3);
        assert_eq!(tpkt.reserved, 0);
        assert_eq!(tpkt.tpdu, [1,2,3,4,5].as_slice());
        assert!(tpkts.next().is_none());
        assert!(tpkts.next().is_none());
    }

    #[test]
    fn test_tpkt_iter_2() {
        let buffer = [
            3u8, 0, 0, 5, 1, 2, 3, 4, 5,
            3u8, 1, 0, 4, 5, 4, 3, 2, 1,
        ];
        let mut tpkts = TpktIter::new(buffer.as_slice());
        let tpkt = tpkts.next().unwrap();
        assert_eq!(tpkt.vrsn, 3);
        assert_eq!(tpkt.reserved, 0);
        assert_eq!(tpkt.tpdu, [1,2,3,4,5].as_slice());
        let tpkt = tpkts.next().unwrap();
        assert_eq!(tpkt.vrsn, 3);
        assert_eq!(tpkt.reserved, 1);
        assert_eq!(tpkt.tpdu, [5,4,3,2].as_slice());
        assert!(tpkts.next().is_none());
        assert!(tpkts.next().is_none());
    }

    #[test]
    fn test_tpkt_iter_3() {
        let buffer = [ 3u8, 0, 0, 0 ];
        let mut tpkts = TpktIter::new(buffer.as_slice());
        let tpkt = tpkts.next().unwrap();
        assert_eq!(tpkt.vrsn, 3);
        assert_eq!(tpkt.reserved, 0);
        assert_eq!(tpkt.tpdu, [].as_slice());
        assert!(tpkts.next().is_none());
        assert!(tpkts.next().is_none());
    }
}
