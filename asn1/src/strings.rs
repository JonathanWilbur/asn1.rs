//! Functions for comparing, normalizing, and validating string types
//!
//! You can validate strings to be of type `PrintableString` or `NumericString`
//! like so:
//!
//! ```rust
//! use wildboar_asn1::strings::{is_printable_str, is_numeric_str, is_ia5_str, is_visible_str};
//!
//! assert!(is_printable_str("Testeroo"));
//! assert!(!is_printable_str("Book with 'F*ck' in the title"));
//! assert!(is_numeric_str("0280 6082 0502"));
//! assert!(!is_numeric_str("deadbeef"));
//! assert!(is_ia5_str("hello world"));
//! assert!(!is_visible_str("hello world"));
//! ```
//!
//! You can compare `NumericString` values like so:
//!
//! ```rust
//! use wildboar_asn1::strings::compare_numeric_string;
//!
//! let a = "   65535  ";
//! let b = " 655 35   ";
//! let c = "    065535";
//! assert!(compare_numeric_string(a, b));
//! assert!(compare_numeric_string(b, a));
//! assert!(!compare_numeric_string(a, c));
//! ```
//!
//! You can also normalize `NumericString` to remove any space characters like
//! so:
//!
//! ```rust
//! use wildboar_asn1::strings::normalize_num_bytes;
//!
//! assert_eq!(normalize_num_bytes(b" 8 7 6 5309").as_ref(), b"8765309");
//! ```
//!
use std::borrow::Cow;
use std::fmt::Write;
use crate::utils::likely;
use crate::{BMPString, UniversalString};
use std::str::FromStr;
use std::fmt::Display;

/// Return `true` if the character `b` is "printable" per the ASN.1 definition
/// of a `PrintableString`.
#[inline]
pub const fn is_printable_char (b: u8) -> bool {
    b.is_ascii_alphanumeric()
    || (b >= b'\x27' && b < b'0' && b != b'*') // '()+,-./ BUT NOT *
    || b == b' '
    || b == b':'
    || b == b'='
    || b == b'?'
}

/// Return `true` if the string `s` is "printable" per the ASN.1 definition
/// of a `PrintableString`.
#[inline]
pub fn is_printable_str (s: &str) -> bool {
    s.as_bytes().iter().all(|b| is_printable_char(*b))
}

/// Return `true` if the character `b` is "numeric" per the ASN.1 definition
/// of a `NumericString`.
#[inline]
pub const fn is_numeric_char (b: u8) -> bool {
    b.is_ascii_digit() || b == b' '
}

/// Return `true` if the string `s` is "numeric" per the ASN.1 definition
/// of a `NumericString`.
#[inline]
pub fn is_numeric_str (s: &str) -> bool {
    s.as_bytes().iter().all(|b| is_numeric_char(*b))
}

/// Return `true` if the string `s` is "IA5" per the ASN.1 definition of
/// `IA5String`. Since IA5 is just ASCII, this checks that `s` is ASCII and
/// nothing else.
#[inline]
pub const fn is_ia5_str (s: &str) -> bool {
    s.is_ascii()
}

#[inline]
pub fn is_visible_char (b: u8) -> bool {
    b.is_ascii_graphic()
}

/// Return `true` if the string `s` is "visible" per the ASN.1 definition of
/// `VisibleString`. A `VisibleString` is ASCII graphic characters.
#[inline]
pub fn is_visible_str (s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_graphic())
}

/// Defined by ITU X.680 as:
/// `0 1 2 3 4 5 6 7 8 9 + - : . , / C D H M R P S T W Y Z`
#[inline]
pub fn is_tstring_char (b: u8) -> bool {
    // This covers all digits AND + , - . / :
    if likely(b >= b'+' && b <= b':') {
        return true;
    }
    b"CDHMPRSTWYZ".binary_search(&b).is_ok()
}

#[inline]
pub fn is_tstring (s: &str) -> bool {
    s.as_bytes().iter().copied().all(is_tstring_char)
}

/// Normalize a `NumericString` by removing the spaces.
pub fn normalize_num_bytes (mut input: &[u8]) -> Cow<[u8]> {
    while input.get(0) == Some(&b' ') {
        input = &input[1..];
    }
    while input.last() == Some(&b' ') {
        input = &input[0..input.len() - 1];
    }
    if input.contains(&b' ') {
        Cow::Owned(input.iter().copied().filter(|&b| b != b' ').collect())
    } else {
        Cow::Borrowed(input)
    }
}

/// Compare two numeric strings without modifying them. Space characters are
/// ignored in the comparison, so `00100` will match `00 1 00`. Leading and
/// trailing zeroes are significant, so `99` will not match `0099`.
///
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

const UNICODE_REPLACEMENT_CHAR: char = '\u{FFFD}';

impl BMPString {

    /// Create a new empty `BMPString`
    pub fn new() -> Self {
        Default::default()
    }

    /// Converts any unmappable characters to the Unicode replacement character
    /// (`U+FFFD`).
    pub fn to_string_lossy(&self) -> String {
        self.0
            .iter()
            .map(|c| char::from_u32(*c as u32).unwrap_or(UNICODE_REPLACEMENT_CHAR))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<u16>> for BMPString {
    fn from(v: Vec<u16>) -> Self {
        Self(v)
    }
}

impl From<&[u16]> for BMPString {
    fn from(v: &[u16]) -> Self {
        Self(v.to_vec())
    }
}

impl TryFrom<BMPString> for String {

    type Error = u16;

    fn try_from(v: BMPString) -> Result<Self, Self::Error> {
        let mut ret = String::with_capacity(v.0.len());
        for c in v.0.iter() {
            ret.push(char::from_u32(*c as u32).ok_or(*c)?);
        }
        Ok(ret)
    }
}  

impl FromStr for BMPString {

    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| c as u16).collect()))
    }
}

impl Display for BMPString {

    /// Display `BMPString` as you would a string, with no surrounding quotes
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            f.write_char(char::from_u32(*c as u32).ok_or(std::fmt::Error::default())?)?;
        }
        Ok(())
    }
}

impl UniversalString {
    pub fn new() -> Self {
        Default::default()
    }

    /// Converts any unmappable characters to the Unicode replacement character
    /// (`U+FFFD`).
    pub fn to_string_lossy(&self) -> String {
        self.0
            .iter()
            .map(|c| char::from_u32(*c).unwrap_or(UNICODE_REPLACEMENT_CHAR))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

}

impl From<Vec<u32>> for UniversalString {
    fn from(v: Vec<u32>) -> Self {
        Self(v)
    }
}

impl From<&[u32]> for UniversalString {
    fn from(v: &[u32]) -> Self {
        Self(v.to_vec())
    }
}

impl TryFrom<UniversalString> for String {

    type Error = u32;

    fn try_from(v: UniversalString) -> Result<Self, Self::Error> {
        let mut ret = String::with_capacity(v.0.len());
        for c in v.0.iter() {
            ret.push(char::from_u32(*c).ok_or(*c)?);
        }
        Ok(ret)
    }
}

impl FromStr for UniversalString {

    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| c as u32).collect()))
    }
}

impl Display for UniversalString {

    /// Display `UniversalString` as you would a string, with no surrounding quotes
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            f.write_char(char::from_u32(*c).ok_or(std::fmt::Error::default())?)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::{compare_numeric_string, normalize_num_bytes};

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

    #[test]
    fn test_normalize_num_bytes_1 () {
        assert_eq!(normalize_num_bytes(b" 876 5309   ").as_ref(), b"8765309");
    }

    #[test]
    fn test_normalize_num_bytes_2 () {
        assert_eq!(normalize_num_bytes(b" ").as_ref(), b"");
    }

    #[test]
    fn test_normalize_num_bytes_3 () {
        assert_eq!(normalize_num_bytes(b" \x00 \x10 ").as_ref(), b"\x00\x10");
    }

    use super::{BMPString, UniversalString};
    use std::str::FromStr;

    #[test]
    fn test_bmpstring_to_utf8_and_back() {
        let original = "Hello, 世界!";
        let bmp = BMPString::from_str(original).unwrap();
        let utf8: String = bmp.clone().try_into().unwrap();
        assert_eq!(utf8, original);
        let bmp2 = BMPString::from_str(&utf8).unwrap();
        assert_eq!(bmp.0, bmp2.0);
    }

    #[test]
    fn test_universalstring_to_utf8_and_back() {
        let original = "Hello, 世界! 👋";
        let uni = UniversalString::from_str(original).unwrap();
        let utf8: String = uni.clone().try_into().unwrap();
        assert_eq!(utf8, original);
        let uni2 = UniversalString::from_str(&utf8).unwrap();
        assert_eq!(uni.0, uni2.0);
    }

    #[test]
    fn test_bmpstring_invalid_codepoint() {
        // BMPString only supports codepoints up to 0xFFFF
        // Emoji 👋 is U+1F44B, which is outside BMP
        let s = "👋";
        let bmp = BMPString::from_str(s).unwrap();
        // The codepoint will be truncated to lower 16 bits
        // So, round-trip will not be equal
        let utf8: String = bmp.clone().try_into().unwrap();
        assert_ne!(utf8, s);
    }

    #[test]
    fn test_universalstring_surrogate_pair() {
        // UniversalString supports all Unicode codepoints
        let s = "𐍈"; // U+10348, outside BMP
        let uni = UniversalString::from_str(s).unwrap();
        let utf8: String = uni.clone().try_into().unwrap();
        assert_eq!(utf8, s);
    }

}
