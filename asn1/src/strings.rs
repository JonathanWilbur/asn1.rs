use std::borrow::Cow;

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

/// Return `true` if the string `s` is "visible" per the ASN.1 definition of
/// `VisibleString`. A `VisibleString` is ASCII graphic characters.
#[inline]
pub fn is_visible_str (s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_graphic())
}

/// Normalize a `NumericString` by removing the spaces.
pub fn normalize_num_bytes (input: &[u8]) -> Cow<[u8]> {
    if input.contains(&0x20) {
        Cow::Owned(input.iter().copied().filter(|&b| b != 0x20).collect())
    } else {
        Cow::Borrowed(input)
    }
}
