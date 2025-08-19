#![doc = include_str!("../README.md")]
#![no_std]
use core::iter::{Iterator, FusedIterator};
use core::error::Error;
use core::fmt::Display;

/// Size of a Version 1 IDM Frame
pub const IDMV1_FRAME_SIZE: usize = 6;
/// Size of a Version 2 IDM Frame
pub const IDMV2_FRAME_SIZE: usize = 8;
/// Absolute smallest valid IDM frame
pub const MIN_FRAME_SIZE: usize = IDMV1_FRAME_SIZE;

/// Values of the IDM `version` field
pub type IDMVersion = u8;
/// IDM Protocol Version 1
pub const IDM_VERSION_1: IDMVersion = 1;
/// IDM Protocol Version 2
pub const IDM_VERSION_2: IDMVersion = 2;

/// Values of the IDM `final` field
pub type IDMFinality = u8;
/// Not a final frame relaying the last bytes of a complete IDM-PDU
pub const IDM_NOT_FINAL: IDMFinality = 0;
/// A final frame relaying the last bytes of a complete IDM-PDU
pub const IDM_FINAL: IDMFinality = 1;

/// Mask for the `encoding` field of a version 2 IDM frame.
pub type EncodingMask = u16;
/// Distinguished Encoding Rules (DER) as described in ITU-T Recommendation X.690.
pub const IDM_ENCODING_DER: EncodingMask = 0b1000_0000_0000_0000;
/// Basic Aligned variant of Packed Encoding Rules (PER) as described in ITU-T Recommendation X.691.
pub const IDM_ENCODING_PER_ALIGNED: EncodingMask = 0b0100_0000_0000_0000;
/// Basic Unaligned variant of Packed Encoding Rules (PER) as described in ITU-T Recommendation X.691.
pub const IDM_ENCODING_PER_UNALIGNED: EncodingMask = 0b0010_0000_0000_0000;
/// XML Encoding Rules (XER) as described in ITU-T Recommendation X.693.
pub const IDM_ENCODING_XER: EncodingMask = 0b0001_0000_0000_0000;

/// An error pertaining to IDM frame decoding
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IdmError {
    /// Unsupported version
    UnsupportedVersion(u8),
}

impl Display for IdmError {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IdmError::UnsupportedVersion(v) => write!(f, "unrecognized idm version ({})", v),
        }
    }

}

impl Error for IdmError {}

/// An IDM Frame
#[derive(Debug)]
pub struct IdmFrame <'a> {
    /// The version field. Only version 1 or 2 are currently defined, which are
    /// encoded as the values 1 and 2, respectively.
    pub version: u8,
    /// The final field. This frame is the final frame containing the complete
    /// IDM PDU if this is set to 1. It is not the final frame if this is set
    /// to 0. All other values are unspecified, but you should probably close
    /// the connection if anything else is used.
    pub final_: u8,
    /// The encoding field of a version 2 IDM frame. This is always set to zero
    /// for version 1 IDM frames.
    pub encoding: u16,
    /// The data fragment carried by the IDM frame.
    pub data: &'a [u8],
}

/// Iterator that reads IDM frames from a buffer, such as a buffer of bytes read
/// in from a TCP socket.
pub struct IdmFrameIter <'a> {
    buffer: &'a [u8],
}

impl <'a> IdmFrameIter<'a> {

    /// Create a new IDM frame iterator
    pub fn new(buffer: &'a [u8]) -> IdmFrameIter<'a> {
        IdmFrameIter { buffer }
    }

    /// Bytes in the buffer not yet read
    pub fn bytes_unread(&self) -> usize {
        self.buffer.len()
    }

}

impl <'a> Iterator for IdmFrameIter<'a> {
    type Item = Result<IdmFrame<'a>, IdmError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.len() < MIN_FRAME_SIZE {
            return None;
        }
        let version: u8 = self.buffer[0];
        let final_: u8 = self.buffer[1];
        let mut encoding: u16 = 0;
        match version {
            1 => {
                let len: usize = u32::from_be_bytes([
                    self.buffer[2],
                    self.buffer[3],
                    self.buffer[4],
                    self.buffer[5],
                ]) as usize;
                let data = self.buffer.get(IDMV1_FRAME_SIZE..IDMV1_FRAME_SIZE + len)?;
                self.buffer = &self.buffer[IDMV1_FRAME_SIZE + len..];
                Some(Ok(IdmFrame {
                    version,
                    final_,
                    encoding,
                    data,
                }))
            },
            2 => {
                if self.buffer.len() < IDMV2_FRAME_SIZE {
                    return None;
                }
                encoding = u16::from_be_bytes([
                    self.buffer[2],
                    self.buffer[3],
                ]);
                let len: usize = u32::from_be_bytes([
                    self.buffer[4],
                    self.buffer[5],
                    self.buffer[6],
                    self.buffer[7],
                ]) as usize;
                let data = self.buffer.get(IDMV2_FRAME_SIZE..IDMV2_FRAME_SIZE + len)?;
                self.buffer = &self.buffer[IDMV2_FRAME_SIZE + len..];
                Some(Ok(IdmFrame {
                    version,
                    final_,
                    encoding,
                    data,
                }))
            },
            _ => {
                self.buffer = &self.buffer[0..0];
                Some(Err(IdmError::UnsupportedVersion(version)))
            },
        }
    }

}

impl <'a> FusedIterator for IdmFrameIter<'a> {}

/// Write an IDM version 1 frame header to `header`
pub fn write_idm_v1_frame_header(
    header: &mut [u8; IDMV1_FRAME_SIZE],
    final_: bool,
    data_len: u32,
) {
    header[0] = IDM_VERSION_1;
    header[1] = if final_ { IDM_FINAL } else { IDM_NOT_FINAL };
    let [l1, l2, l3, l4] = data_len.to_be_bytes();
    header[2] = l1;
    header[3] = l2;
    header[4] = l3;
    header[5] = l4;
}

/// Write an IDM version 2 frame header to `header`
pub fn write_idm_v2_frame_header(
    header: &mut [u8; IDMV2_FRAME_SIZE],
    final_: bool,
    encoding: EncodingMask,
    data_len: u32,
) {
    header[0] = IDM_VERSION_2;
    header[1] = if final_ { IDM_FINAL } else { IDM_NOT_FINAL };
    let [e1, e2] = encoding.to_be_bytes();
    header[2] = e1;
    header[3] = e2;
    let [l1, l2, l3, l4] = data_len.to_be_bytes();
    header[4] = l1;
    header[5] = l2;
    header[6] = l3;
    header[7] = l4;
}

#[cfg(test)]
mod tests {

    extern crate alloc;
    use alloc::vec::Vec;

    use super::{
        IdmFrameIter,
        write_idm_v1_frame_header,
        write_idm_v2_frame_header,
        IDM_ENCODING_XER,
        IDMV1_FRAME_SIZE,
        IDMV2_FRAME_SIZE,
    };

    #[test]
    fn iter_idm_v1_frames_1() {
        let buffer: Vec<u8> = Vec::from([
            1, 1, 0, 0, 0, 5, 1, 2, 3, 4, 5,
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 1);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0);
        assert_eq!(frame.data, [1,2,3,4,5].as_slice());
        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v1_frames_2() {
        let buffer: Vec<u8> = Vec::from([
            1, 1, 0, 0, 0, 5, 1, 2, 3, 4, 5, 1
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 1);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0);
        assert_eq!(frame.data, [1,2,3,4,5].as_slice());
        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v1_frames_3() {
        let buffer: Vec<u8> = Vec::from([ 1, 1, 0, 0, 0, 0 ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 1);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0);
        assert_eq!(frame.data, [].as_slice());
        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v1_frames_4() {
        let buffer: Vec<u8> = Vec::from([
            1, 0, 0, 0, 0, 5, 1, 2, 3, 4, 5,
            1, 1, 0, 0, 0, 4, 5, 6, 7, 8, 9,
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 1);
        assert_eq!(frame.final_, 0);
        assert_eq!(frame.encoding, 0);
        assert_eq!(frame.data, [1,2,3,4,5].as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 1);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0);
        assert_eq!(frame.data, [5,6,7,8].as_slice());

        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v2_frames_1() {
        let buffer: Vec<u8> = Vec::from([
            2, 1, 8, 8, 0, 0, 0, 5, 1, 2, 3, 4, 5,
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 2);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0x0808);
        assert_eq!(frame.data, [1,2,3,4,5].as_slice());
        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v2_frames_2() {
        let buffer: Vec<u8> = Vec::from([
            2, 1, 8, 8, 0, 0, 0, 5, 1, 2, 3, 4, 5, 1
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 2);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0x0808);
        assert_eq!(frame.data, [1,2,3,4,5].as_slice());
        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v2_frames_3() {
        let buffer: Vec<u8> = Vec::from([ 2, 1, 8, 8, 0, 0, 0, 0 ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 2);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0x0808);
        assert_eq!(frame.data, [].as_slice());
        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_v2_frames_4() {
        let buffer: Vec<u8> = Vec::from([
            2, 0, 8, 8, 0, 0, 0, 5, 1, 2, 3, 4, 5,
            2, 1, 8, 8, 0, 0, 0, 4, 5, 6, 7, 8, 9,
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 2);
        assert_eq!(frame.final_, 0);
        assert_eq!(frame.encoding, 0x0808);
        assert_eq!(frame.data, [1,2,3,4,5].as_slice());
        let frame = frames.next().unwrap().unwrap();
        assert_eq!(frame.version, 2);
        assert_eq!(frame.final_, 1);
        assert_eq!(frame.encoding, 0x0808);
        assert_eq!(frame.data, [5,6,7,8].as_slice());

        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn iter_idm_unrecognized_version() {
        let buffer: Vec<u8> = Vec::from([
            3, 1, 8, 8, 0, 0, 0, 4, 5, 6, 7, 8, 9,
        ].as_slice());
        let mut frames = IdmFrameIter::new(buffer.as_slice());
        let frame = frames.next().unwrap();
        assert!(frame.is_err());

        let frame = frames.next();
        assert!(frame.is_none());
        let frame = frames.next();
        assert!(frame.is_none());
    }

    #[test]
    fn test_write_idm_v1_frame_header_1() {
        let mut buf: [u8; IDMV1_FRAME_SIZE] = [0, 0, 0, 0, 0, 0];
        write_idm_v1_frame_header(&mut buf, true, 1024);
        assert_eq!(buf, [1, 1, 0, 0, 4, 0]);
    }

    #[test]
    fn test_write_idm_v2_frame_header_1() {
        let mut buf: [u8; IDMV2_FRAME_SIZE] = [0, 0, 0, 0, 0, 0, 0, 0];
        write_idm_v2_frame_header(&mut buf, true, IDM_ENCODING_XER, 1024);
        assert_eq!(buf, [2, 1, 0x10, 0, 0, 0, 4, 0]);
    }

}
