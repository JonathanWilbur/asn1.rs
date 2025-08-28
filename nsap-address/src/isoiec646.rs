//! ISO/IEC 646 character encoding per ITU-T Rec. X.213, Section A.5.3.e
//!
//! Quoting ITU-T Rec. X.213, Section A.5.3.e:
//!
//! > when the IDI format is "Local", representing an ISO/IEC 646 character
//! > syntax DSP by converting each character to a number in the range 32-127
//! > using the ISO/IEC 646 encoding, with zero parity and the parity bit in
//! > the most significant position, reducing the value by 32, giving a number
//! > in the range 0-95, encoding this result as a pair of decimal digits, and
//! > using a semi-octet to represent the value of each decimal digit, yielding
//! > a value in the range 0000-1001 for each digit;
//!
//! This just might be the dumbest way to encode characters I've ever heard of.

/// Convert an ISO/IEC 646 byte encoded per ITU-T Rec. X.213 to a UTF-8 `char`
///
/// Returns an `Err` if the byte does not encode a permitted character.
/// Permitted characters are those from ASCII code points 32 to 127, inclusively.
pub const fn local_iso_iec_646_byte_to_char(b: u8) -> Result<char, ()> {
    let ones: u8 = b & 0x0F;
    let tens: u8 = (b & 0xF0) >> 4;
    if ones > 9 || tens > 9 {
        return Err(());
    }
    let cc: u8 = (tens * 10) + ones;
    // Out of precaution, this library additionally prohibits code point 127
    // (`ESC`). This is a control character, and I think it was the ITU's
    // intention to forbid this character from the encoding as well.
    if cc >= 95 {
        return Err(());
    }
    let c = (cc + 32) as char;
    debug_assert!(!c.is_ascii_control());
    Ok(c)
}

/// Convert a UTF-8 `char` to an ISO/IEC 646 byte per ITU-T Rec. X.213
///
/// Returns an `Err` if the `char` is not a permitted character.
/// Permitted characters are those from ASCII code points 32 to 127, inclusively.
///
/// Out of precaution, this library additionally prohibits code point 127
/// (`ESC`). This is a control character, and I think it was the ITU's
/// intention to forbid this character from the encoding as well.
pub const fn char_to_local_iso_iec_646_byte(c: char) -> Result<u8, ()> {
    if c.is_ascii_control() || c > '\x7E' {
        return Err(());
    }
    let cc = (c as u8) - 32;
    let ones: u8 = cc % 10;
    let tens: u8 = cc / 10;
    let b: u8 = (tens << 4) + ones;
    Ok(b)
}

#[cfg(test)]
mod tests {

    use super::{char_to_local_iso_iec_646_byte, local_iso_iec_646_byte_to_char};

    #[test]
    fn test_char_to_local_iso_iec_646_byte() {
        let b = char_to_local_iso_iec_646_byte('a').unwrap();
        assert_eq!(b, 0x65);
    }

    #[test]
    fn test_local_iso_iec_646_byte_to_char() {
        let c = local_iso_iec_646_byte_to_char(0x65).unwrap();
        assert_eq!(c, 'a');
    }

    #[test]
    fn test_iso_iec_646_encode_decode() {
        for c in ' '..='~' {
            let encoded = char_to_local_iso_iec_646_byte(c).unwrap();
            let decoded = local_iso_iec_646_byte_to_char(encoded).unwrap();
            assert_eq!(decoded, c);
        }
    }
}
