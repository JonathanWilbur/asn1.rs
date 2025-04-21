use std::borrow::Cow;

#[inline]
pub fn is_printable_char (b: u8) -> bool {
    b.is_ascii_alphanumeric()
    || (b >= b'\x27' && b < b'0' && b != b'*') // '()+,-./ BUT NOT *
    || b == b' '
    || b == b':'
    || b == b'='
    || b == b'?'
}

#[inline]
pub fn is_printable_str (s: &str) -> bool {
    s.as_bytes().iter().all(|b| is_printable_char(*b))
}

#[inline]
pub fn is_numeric_char (b: u8) -> bool {
    b.is_ascii_digit() || b == b' '
}

#[inline]
pub fn is_numeric_str (s: &str) -> bool {
    s.as_bytes().iter().all(|b| is_numeric_char(*b))
}

#[inline]
pub fn is_ia5_str (s: &str) -> bool {
    s.is_ascii()
}

#[inline]
pub fn is_general_str (s: &str) -> bool {
    s.is_ascii()
}

#[inline]
pub fn is_visible_str (s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_graphic())
}

pub fn normalize_num_bytes (input: &[u8]) -> Cow<[u8]> {
    if input.contains(&0x20) {
        Cow::Owned(input.iter().copied().filter(|&b| b != 0x20).collect())
    } else {
        Cow::Borrowed(input)
    }
}
