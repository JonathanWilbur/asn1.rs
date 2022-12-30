use crate::types::ByteSlice;

pub fn read_i64(bytes: ByteSlice) -> Result<i64, ()> {
    let len = bytes.len();
    match len {
        1 => Ok(bytes[0] as i64),
        2 => Ok(i16::from_be_bytes([bytes[0], bytes[1]]) as i64),
        3 => Ok(i32::from_be_bytes([
            if bytes[0] & 0b1000_0000 > 0 {
                0xFF
            } else {
                0x00
            },
            bytes[0],
            bytes[1],
            bytes[2],
        ]) as i64),
        4 => Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64),
        5..=8 => {
            let mut buf: [u8; 8] = [0; 8];
            buf[8 - len..].copy_from_slice(bytes);
            Ok(i64::from_be_bytes(buf))
        }
        _ => Err(()),
    }
}
