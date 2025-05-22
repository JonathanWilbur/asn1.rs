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

pub fn read_i128(bytes: ByteSlice) -> Result<i128, ()> {
    let len = bytes.len();
    match len {
        1 => Ok(bytes[0] as i128),
        2 => Ok(i16::from_be_bytes([bytes[0], bytes[1]]) as i128),
        3 => Ok(i32::from_be_bytes([
            if bytes[0] & 0b1000_0000 > 0 {
                0xFF
            } else {
                0x00
            },
            bytes[0],
            bytes[1],
            bytes[2],
        ]) as i128),
        4 => Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i128),
        5..=16 => {
            let mut buf: [u8; 16] = [0; 16];
            buf[16 - len..].copy_from_slice(bytes);
            Ok(i128::from_be_bytes(buf))
        }
        _ => Err(()),
    }
}

pub fn write_oid_arc<W>(output: &mut W, mut num: u128) -> std::io::Result<usize>
where
    W: std::io::Write
{
    if likely(num < 128) {
        return output.write(&[num as u8]);
    }

    // A u128 can take up to 19 bytes. We do 20 just for safety.
    let mut encoded: [u8; 20] = [0; 20];
    let mut byte_count: usize = 0;
    while num > 0b0111_1111 {
        encoded[byte_count] = (num & 0b0111_1111) as u8 | 0b1000_0000;
        byte_count += 1;
        num >>= 7;
    }
    encoded[byte_count] = num as u8;
    output.write(&encoded[0..byte_count+1])
}

/// If you need to remove spaces from numeric strings, consider using the
/// `cow-utils` crate: https://crates.io/crates/cow-utils.
pub const fn compare_numeric_string (a: &str, b: &str) -> bool {
    // This function was made uglier so it could be const.
    let a_trim = a.as_bytes();
    let b_trim = b.as_bytes();
    let mut i = 0;
    let mut j = 0;
    'a_loop: while i < a_trim.len() {
        let a_byte = a_trim[i];
        if a_byte == b' ' {
            i += 1;
            continue;
        }
        while j < b_trim.len() {
            let b_byte = b_trim[j];
            if b_byte == b' ' {
                j += 1;
                continue;
            }
            if b_byte != a_byte {
                return false;
            }
            i += 1;
            j += 1;
            continue 'a_loop;
        }
        // There was a remaining digit in A, but not more characters in B.
        // B cannot possibly match A, so return false.
        return false;
    }
    // Beyond this point, we ran out of A characters.
    // So we need to check if B has any more digits to match.
    while j < b_trim.len() {
        let b_byte = b_trim[j];
        if b_byte == b' ' {
            j += 1;
            continue;
        }
        return false;
    }
    true
}

/// This is not a time library.
#[inline]
pub(crate) const fn get_days_in_month (year: u16, month: u8) -> u8 {
    let is_leap_year = ((year % 4) == 0) && (((year % 100) > 0) || ((year % 400) == 0));
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => if is_leap_year { 29 } else { 28 },
        _ => 30,
    }
}

pub(crate) mod macros {
    macro_rules! parse_uint {
        ( $inttype:ty, $bytes:expr, $string:expr, $errcode:expr ) => {
            if cfg!(feature = "atoi_simd") {
                atoi_simd::parse_pos::<$inttype>($bytes)
                    .map_err(|_| ASN1Error::new($errcode))?
            } else {
                <$inttype>::from_str($string)
                    .map_err(|_| ASN1Error::new($errcode))?
            }
        };
    }

    pub(crate) use parse_uint;
}

#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    if cfg!(feature = "likely_stable") {
        likely_stable::likely(expr)
    } else {
        expr
    }
}

#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    if cfg!(feature = "likely_stable") {
        likely_stable::unlikely(expr)
    } else {
        expr
    }
}

#[cfg(test)]
mod tests {

    use super::compare_numeric_string;

    #[test]
    fn compare_numeric_string_empty_1 () {
        let a = String::from("");
        let b = String::from("");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_empty_2 () {
        let a = String::from("");
        let b = String::from("1");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_empty_3 () {
        let a = String::from("   ");
        let b = String::from(" ");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_single_digit_1 () {
        let a = String::from("3");
        let b = String::from("3");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_single_digit_2 () {
        let a = String::from("3");
        let b = String::from("1");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_multi_digit_1 () {
        let a = String::from("65535");
        let b = String::from("65535");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_multi_digit_2 () {
        let a = String::from("65535");
        let b = String::from("65435");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_multi_digit_3 () {
        let a = String::from("65535555");
        let b = String::from("65535");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_multi_digit_4 () {
        let a = String::from("65535");
        let b = String::from("65535555");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_multi_diff () {
        let a = String::from("65535");
        let b = String::from("65421");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_surrounding_whitespace_1 () {
        let a = String::from("   65535  ");
        let b = String::from(" 65535     ");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_surrounding_whitespace_2 () {
        let a = String::from("   65535  ");
        let b = String::from(" 65536     ");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_1 () {
        let a = String::from("65535");
        let b = String::from("65 535");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_2 () {
        let a = String::from("65535");
        let b = String::from("65 435");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_3 () {
        let a = String::from("65535");
        let b = String::from("65   535");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_4 () {
        let a = String::from("65535");
        let b = String::from("65   435");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_5 () {
        let a = String::from("65   535");
        let b = String::from("65535");
        assert!(compare_numeric_string(&a, &b));
        assert!(compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_6 () {
        let a = String::from("65   535");
        let b = String::from("65435");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

    #[test]
    fn compare_numeric_string_inner_whitespace_7 () {
        let a = String::from("65  5");
        let b = String::from("65535");
        assert!(!compare_numeric_string(&a, &b));
        assert!(!compare_numeric_string(&b, &a));
    }

}
