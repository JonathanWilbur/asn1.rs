use std::io::{Result, ErrorKind, Error};
use std::collections::VecDeque;
use tokio::io::AsyncWriteExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};

pub type Bytes = Vec<u8>;

pub const IDM_VERSION_UNSET: u8 = 0;
pub const IDM_VERSION_1: u8 = 1;
pub const IDM_VERSION_2: u8 = 2;
pub const IDM_V1_FRAME_SIZE: u8 = 6;
pub const IDM_V2_FRAME_SIZE: u8 = 8;
pub const IDM_DEFAULT_BYTE_BUFFER_SIZE: usize = 10_000_000;
pub const IDM_DEFAULT_SEGMENT_BUFFER_SIZE: usize = 100;

pub const IDM_ENCODING_BER: u16 = 0;
pub const IDM_ENCODING_DER: u16 = 0b0000_0000_0000_0001;
pub const IDM_ENCODING_BAPER: u16 = 0b0000_0000_0000_0010;
pub const IDM_ENCODING_BUPER: u16 = 0b0000_0000_0000_0100;
pub const IDM_ENCODING_XER: u16 = 0b0000_0000_0000_1000;
pub const IDM_ENCODING_UNKNOWN: u16 = 0xFFFF;

#[cfg(target_pointer_width = "16")]
compile_error!("16-bit not supported, because IDM PDUs cannot be reliably sized down to fit in a u16.");

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct IDMSegment {
    pub version: u8,
    pub final_: bool,
    pub encoding: u16,
    pub data_bounds: [usize; 2],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IDMSocketOptions <W : AsyncWriteExt> {
    pub byte_buffer_size: usize,
    pub segment_buffer_size: usize,
    pub output: W,
}

// Taken from: https://rust-lang.github.io/async-book/02_execution/03_wakeups.html#applied-build-a-timer
pub struct FutureState {
    pub completed: bool,
    pub waker: Option<Waker>,
}

impl Default for FutureState {
    
    fn default() -> Self {
        FutureState { completed: false, waker: None }
    }

}

pub struct IdmStream <W : AsyncWriteExt> {
    pub version: u8, // 0 = unset
    pub encoding: u16,
    pub output: W,

    // I tried an implementation that would not concatenate buffers for the sake
    // avoiding allocations, but I realized the buffers have to be concatenated
    // for the ASN.1-encoded data to be parsed anyway.
    buffer: Vec<u8>,
    segments: VecDeque<IDMSegment>,
    byte_buffer_size: usize,
    segment_buffer_size: usize,

    /// Tokio says you can use the std Mutex in most cases.
    /// See: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use
    future_state: Arc<Mutex<FutureState>>,
}

impl <W : AsyncWriteExt> std::fmt::Debug for IdmStream<W> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "IdmSocket {{ v{}, enc {:#b}, byte_buf_size {}, seg_buf_size {}, buffer {:?}, segments {:?} }}",
            self.version,
            self.encoding,
            self.byte_buffer_size,
            self.segment_buffer_size,
            self.buffer,
            self.segments,
        ))
    }

}

// I think this will have to be a TCP socket so you can configure a waker.
impl <T : AsyncWriteExt + Unpin> IdmStream <T> {

    pub fn new (output: T) -> Self {
        IdmStream {
            version: IDM_VERSION_UNSET,
            encoding: IDM_ENCODING_UNKNOWN,
            buffer: Vec::with_capacity(IDM_DEFAULT_BYTE_BUFFER_SIZE),
            segments: VecDeque::with_capacity(IDM_DEFAULT_SEGMENT_BUFFER_SIZE),
            byte_buffer_size: IDM_DEFAULT_BYTE_BUFFER_SIZE,
            segment_buffer_size: IDM_DEFAULT_SEGMENT_BUFFER_SIZE,
            output,
            future_state: Arc::new(Mutex::new(FutureState::default())),
        }
    }

    pub async fn write_pdu (&mut self, bytes: &[u8], encoding: u16) -> Result<usize> {
        if bytes.len() > u32::MAX as usize {
            // This implementation simply will not allow you to write an IDM PDU
            // larger than 4GB.
            return Err(Error::from(ErrorKind::InvalidInput)); 
        }
        if self.version == IDM_VERSION_UNSET {
            if encoding > IDM_ENCODING_BER {
                self.version = IDM_VERSION_2;
                self.output.write(&[ 0x02, 0x01 ]).await?; // Version 2, Final
                self.output.write(u16::to_be_bytes(encoding).as_slice()).await?;
                self.output.write(u32::to_be_bytes(bytes.len() as u32).as_slice()).await?;
            } else {
                self.output.write(&[ 0x01, 0x01 ]).await?; // Version 1, Final
                self.output.write(u32::to_be_bytes(bytes.len() as u32).as_slice()).await?;
            }
        } else if self.version == IDM_VERSION_1 {
            self.version = IDM_VERSION_1;
            if encoding != IDM_ENCODING_BER {
                return Err(Error::from(ErrorKind::Unsupported));
            }
            self.output.write(&[ 0x01, 0x01 ]).await?; // Version 1, Final
            self.output.write(u32::to_be_bytes(bytes.len() as u32).as_slice()).await?;
        } else if self.version == IDM_VERSION_2 {
            self.output.write(&[ 0x02, 0x01 ]).await?; // Version 2, Final
            self.output.write(u16::to_be_bytes(encoding).as_slice()).await?;
            self.output.write(u32::to_be_bytes(bytes.len() as u32).as_slice()).await?;
        } else {
            return Err(Error::from(ErrorKind::Other));
        }
        self.output.write(bytes).await
    }

    pub fn receive_data (&mut self, bytes: &[u8]) -> Result<usize> {
        let (new_buffer_size, overflowed) = self.buffer.len().overflowing_add(bytes.len());
        if overflowed {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        if new_buffer_size > self.byte_buffer_size {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        std::io::Write::write(&mut self.buffer, &bytes)?;
        let mut i: usize = 0;
        loop {
            match self.chomp_frame(i) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        return Ok(i);
                    }
                    let (new_i, overflowed_i) = i.overflowing_add(bytes_read);
                    if overflowed_i {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    i = new_i;
                },
                Err(e) => return Err(e),
            }
        }
    }

    fn chomp_frame (&mut self, start_index: usize) -> Result<usize> {
        let (bytes_remaining, br_overflowed) = self.buffer.len().overflowing_sub(start_index);
        if br_overflowed {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        if bytes_remaining < IDM_V1_FRAME_SIZE as usize {
            return Ok(0);
        }
        // assert(self.chunks.len() > 1);
        let version = self.buffer[start_index];
        if version == 0 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        if version > 2 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        if self.version == IDM_VERSION_UNSET {
            self.version = version;
        } else if version != self.version {
            return Err(Error::from(ErrorKind::InvalidData)); // version change
        }
        if (version == 2) && (bytes_remaining < IDM_V2_FRAME_SIZE as usize) {
            return Ok(0);
        }
        let final_ = self.buffer[start_index + 1];
        if final_ > 1 {
            return Err(Error::from(ErrorKind::InvalidData)); // Invalid "final" value.
        }
        if version == 1 { // This is already guaranteed to at least have enough bytes for an IDMv1 frame.
            let length: u32 = u32::from_be_bytes([
                self.buffer[start_index + 2],
                self.buffer[start_index + 3],
                self.buffer[start_index + 4],
                self.buffer[start_index + 5],
            ]);
            if length == 0 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            // If we haven't received the full PDU yet...
            let (idm_frame_size, idm_frame_size_overflowed) = length
                .overflowing_add(IDM_V1_FRAME_SIZE as u32);
            if idm_frame_size_overflowed {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            if bytes_remaining < idm_frame_size as usize {
                return Ok(0);
            }
            let (start_of_data, sod_overflowed) = start_index
                .overflowing_add(IDM_V1_FRAME_SIZE as usize);
            if sod_overflowed {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            let (end_of_frame, eof_overflowed) = start_of_data.overflowing_add(length as usize);
            if eof_overflowed {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            let is_final: bool = final_ == 1;
            let seg = IDMSegment {
                version,
                final_: is_final,
                encoding: 0,
                data_bounds: [ start_of_data, end_of_frame ],
            };
            if self.segments.len() >= self.segment_buffer_size {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            if self.encoding == IDM_ENCODING_UNKNOWN {
                self.encoding = IDM_ENCODING_BER;
            }
            self.segments.push_back(seg);
            return Ok(IDM_V1_FRAME_SIZE as usize + length as usize);
        }
        else if version == 2 {
            let length: u32 = u32::from_be_bytes([
                self.buffer[start_index + 4],
                self.buffer[start_index + 5],
                self.buffer[start_index + 6],
                self.buffer[start_index + 7],
            ]);
            if length == 0 {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            // If we haven't received the full PDU yet...
            let (idm_frame_size, idm_frame_size_overflowed) = length
                .overflowing_add(IDM_V2_FRAME_SIZE as u32);
            if idm_frame_size_overflowed {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            if bytes_remaining < idm_frame_size as usize {
                return Ok(0);
            }
            let encoding: u16 = u16::from_be_bytes([
                self.buffer[start_index + 2],
                self.buffer[start_index + 3],
            ]);
            if let Some(last_seg) = self.segments.back() {
                if last_seg.encoding != encoding {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            let (start_of_data, sod_overflowed) = start_index
                .overflowing_add(IDM_V2_FRAME_SIZE as usize);
            if sod_overflowed {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            let (end_of_frame, eof_overflowed) = start_of_data.overflowing_add(length as usize);
            if eof_overflowed {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            let is_final: bool = final_ == 1;
            let seg = IDMSegment {
                version,
                final_: is_final,
                encoding,
                data_bounds: [ start_of_data, end_of_frame ],
            };
            if self.segments.len() >= self.segment_buffer_size {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            self.segments.push_back(seg);
            return Ok(IDM_V2_FRAME_SIZE as usize + length as usize);
        } else {
            // This alternative should never happen.
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }

    pub fn read_pdu (&mut self) -> Option<(u16, Bytes)> {
        let end_index = self.segments.iter().position(|s| s.final_)?;
        let last_seg_of_pdu = self.segments[end_index];
        let end_of_pdu = last_seg_of_pdu.data_bounds[1];
        let data = self.segments
            .make_contiguous()[0..end_index+1]
            .iter()
            .map(|s| &self.buffer[s.data_bounds[0]..s.data_bounds[1]])
            .collect::<Vec<&[u8]>>()
            .concat();
        self.segments.drain(0..=end_index);
        self.buffer.drain(0..end_of_pdu);
        for seg in self.segments.iter_mut() {
            seg.data_bounds[0] -= end_of_pdu;
            seg.data_bounds[1] -= end_of_pdu;
        }
        Some((last_seg_of_pdu.encoding, data))
    }

    pub fn peek_pdu (&mut self) -> Option<(u16, Bytes)> {
        let end_index = self.segments.iter().position(|s| s.final_)?;
        let last_seg_of_pdu = self.segments[end_index];
        let data = self.segments.make_contiguous()[0..end_index+1]
            .iter()
            .map(|s| &self.buffer[s.data_bounds[0]..s.data_bounds[1]])
            .collect::<Vec<&[u8]>>()
            .concat();
        Some((last_seg_of_pdu.encoding, data))
    }

}

impl <W : AsyncWriteExt> From<IDMSocketOptions<W>> for IdmStream<W> {

    fn from(opts: IDMSocketOptions<W>) -> Self {
        IdmStream {
            version: IDM_VERSION_UNSET,
            encoding: IDM_ENCODING_UNKNOWN,
            buffer: Vec::with_capacity(opts.byte_buffer_size),
            segments: VecDeque::with_capacity(opts.segment_buffer_size),
            byte_buffer_size: opts.byte_buffer_size,
            segment_buffer_size: opts.segment_buffer_size,
            output: opts.output,
            future_state: Arc::new(Mutex::new(FutureState::default())),
        }
    }

}

impl <W : AsyncWriteExt + Unpin> Future for IdmStream<W> {
    type Output = (u16, Vec<u8>);
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let mut_self = self.get_mut();
        if let Some(pdu) = mut_self.read_pdu() {
            Poll::Ready(pdu)
        } else {
            let mut future_state = mut_self.future_state.lock().unwrap();
            future_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

/// This ONLY polls for PDUs that are immediately available.
impl <W : AsyncWriteExt + Unpin> Iterator for IdmStream<W> {
    type Item = (u16, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.read_pdu()
    }

}

impl <W : AsyncWriteExt + Unpin> Stream for IdmStream<W> {
    type Item = (u16, Vec<u8>);

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>> {
        let mut_self = self.get_mut();
        if let Some(pdu) = mut_self.read_pdu() {
            Poll::Ready(Some(pdu))
        } else {
            let mut future_state = mut_self.future_state.lock().unwrap();
            future_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }

}

pub trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_idm_v1_decode() {
        let idm_frame: Vec<u8> = vec![
            0x01, // v1
            0x01, // final
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frame) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frame.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 0);
                assert_eq!(data.len(), 4);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v2_decode() {
        let idm_frame: Vec<u8> = vec![
            0x02, // v2
            0x01, // final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frame) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frame.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 1);
                assert_eq!(data.len(), 4);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v1_decode_back_to_back() {
        let idm_frames: Vec<u8> = vec![
            0x01, // v1
            0x01, // final
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
            0x01, // v1
            0x01, // final
            0x00, 0x00, 0x00, 0x05, // length = 5
            0x05, 0x06, 0x07, 0x08, 0x09, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frames) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frames.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 0);
                assert_eq!(data.len(), 4);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 0);
                assert_eq!(data.len(), 5);
                assert!(data.starts_with(&[ 0x05, 0x06, 0x07, 0x08, 0x09 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v2_decode_back_to_back() {
        let idm_frames: Vec<u8> = vec![
            0x02, // v2
            0x01, // final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
            0x02, // v2
            0x01, // final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x05, // length = 5
            0x05, 0x06, 0x07, 0x08, 0x09, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frames) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frames.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 1);
                assert_eq!(data.len(), 4);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 1);
                assert_eq!(data.len(), 5);
                assert!(data.starts_with(&[ 0x05, 0x06, 0x07, 0x08, 0x09 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v1_decode_fragmented() {
        let idm_frames: Vec<u8> = vec![
            0x01, // v1
            0x01, // final
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
            0x01, // v1
            0x01, // final
            0x00, 0x00, 0x00, 0x05, // length = 5
            0x05, 0x06, 0x07, 0x08, 0x09, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frames[0..8]) {
            Ok(bytes_read) => {
                // Zero should be read, because there are no full IDM frames.
                assert_eq!(*bytes_read, 0);
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.receive_data(&idm_frames[8..]) {
            Ok(bytes_read) => {
                // This should return the whole length of all bytes, because
                // both PDUs are completed with this chunk of bytes.
                assert_eq!(*bytes_read, idm_frames.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 0);
                assert_eq!(data.len(), 4);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 0);
                assert_eq!(data.len(), 5);
                assert!(data.starts_with(&[ 0x05, 0x06, 0x07, 0x08, 0x09 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v2_decode_fragmented() {
        let idm_frames: Vec<u8> = vec![
            0x02, // v2
            0x01, // final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
            0x02, // v2
            0x01, // final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x05, // length = 5
            0x05, 0x06, 0x07, 0x08, 0x09, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frames[0..8]) {
            Ok(bytes_read) => {
                // Zero should be read, because there are no full IDM frames.
                assert_eq!(*bytes_read, 0);
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.receive_data(&idm_frames[8..]) {
            Ok(bytes_read) => {
                // This should return the whole length of all bytes, because
                // both PDUs are completed with this chunk of bytes.
                assert_eq!(*bytes_read, idm_frames.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 1);
                assert_eq!(data.len(), 4);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 1);
                assert_eq!(data.len(), 5);
                assert!(data.starts_with(&[ 0x05, 0x06, 0x07, 0x08, 0x09 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v1_decode_segmented() {
        let idm_frame: Vec<u8> = vec![
            0x01, // v1
            0x00, // NOT final
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
            0x01, // v1
            0x01, // NOT final
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x05, 0x06, 0x07, 0x08, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frame) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frame.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 0);
                assert_eq!(data.len(), 8);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v2_decode_segmented() {
        let idm_frame: Vec<u8> = vec![
            0x02, // v2
            0x00, // NOT final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
            0x02, // v2
            0x01, // final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x05, 0x06, 0x07, 0x08, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frame) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frame.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((encoding, data)) => {
                assert_eq!(*encoding, 1);
                assert_eq!(data.len(), 8);
                assert!(data.starts_with(&[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08 ]));
            },
            None => {
                println!("{:?}", idm);
                panic!("No PDU could be read.");
            },
        };
    }

    #[test]
    fn test_idm_v1_decode_incomplete() {
        let idm_frame: Vec<u8> = vec![
            0x01, // v1
            0x00, // NOT final
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frame) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frame.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((_encoding, _data)) => {
                panic!("PDU should NOT have been read.");
            },
            None => {},
        };
    }

    #[test]
    fn test_idm_v2_decode_incomplete() {
        let idm_frame: Vec<u8> = vec![
            0x02, // v2
            0x00, // NOT final
            0x00, 0x01, // encoding = DER only
            0x00, 0x00, 0x00, 0x04, // length = 4
            0x01, 0x02, 0x03, 0x04, // data
        ];
        let mut idm = IdmStream::new(vec![]);
        match &idm.receive_data(&idm_frame) {
            Ok(bytes_read) => {
                assert_eq!(*bytes_read, idm_frame.len());
            },
            Err(e) => {
                println!("{:?}", idm);
                panic!("{}", e);
            },
        };
        match &idm.read_pdu() {
            Some((_encoding, _data)) => {
                panic!("PDU should NOT have been read.");
            },
            None => {},
        };
    }

}