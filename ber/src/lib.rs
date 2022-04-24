use std::io::{Write, Result};
use asn1::types::{BOOLEAN};

pub fn ber_encode_boolean <T> (output: &mut T, value: BOOLEAN) -> Result<usize>
    where T : Write {
    if value {
       return output.write(&[ 0x01, 0x01, 0xFF ]);
    } else {
        return output.write(&[ 0x01, 0x01, 0x00 ]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_ber_encode_boolean () {
        let mut output: Vec<u8> = Vec::new();
        crate::ber_encode_boolean(&mut output, true).unwrap();
        crate::ber_encode_boolean(&mut output, false).unwrap();
        assert_eq!(output.len(), 6);
        assert!(output.starts_with(&[ 0x01, 0x01, 0xFF, 0x01, 0x01, 0x00 ]));
    }
}
