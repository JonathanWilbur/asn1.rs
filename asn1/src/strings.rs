
pub fn is_printable_char (b: u8) -> bool {
    b.is_ascii_alphanumeric()
    || (b >= b'\x27' && b < b'0' && b != b'*') // '()+,-./ BUT NOT *
    || b == b' '
    || b == b':'
    || b == b'='
    || b == b'?'
}

pub fn is_printable_str (s: &str) -> bool {
    s.as_bytes().iter().all(|b| is_printable_char(*b))
}

pub fn is_numeric_char (b: u8) -> bool {
    b.is_ascii_digit() || b == b' '
}

pub fn is_numeric_str (s: &str) -> bool {
    s.as_bytes().iter().all(|b| is_numeric_char(*b))
}

// TODO: Do more for other string types.
