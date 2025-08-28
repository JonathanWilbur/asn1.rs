//! Miscellaneous utilties

/// Convert a u8 to decimal ASCII digits
pub(crate) fn u8_to_decimal_bytes(mut n: u8) -> [u8; 3] {
    let hundreds = n / 100;
    n %= 100;
    let tens = n / 10;
    let ones = n % 10;
    [b'0' + hundreds, b'0' + tens, b'0' + ones]
}

/// Convert a u16 to decimal ASCII digits
pub(crate) fn u16_to_decimal_bytes(mut n: u16) -> [u8; 5] {
    let ten_thousands = (n / 10000) as u8;
    n %= 10000;
    let thousands = (n / 1000) as u8;
    n %= 1000;
    let hundreds = (n / 100) as u8;
    n %= 100;
    let tens = (n / 10) as u8;
    let ones = (n % 10) as u8;
    [
        b'0' + ten_thousands,
        b'0' + thousands,
        b'0' + hundreds,
        b'0' + tens,
        b'0' + ones,
    ]
}
