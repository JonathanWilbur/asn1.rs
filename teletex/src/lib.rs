use std::borrow::Cow;

/// Any code points that are unrecognized as Teletex will be returned unchanged
/// to be interpreted directly as a UTF-8 code point. The rationale for this is
/// that there is very high overlap between Teletex and ASCII and UTF-8. If
/// there are any characters that I missed when writing this function, or if any
/// new ones are added to Teletex, they are probably analogous.
pub fn teletex_char_to_utf8_char (c: u8) -> char {
    match c {
        0xA4 => '$',
        0xA6 => '#',
        0xA8 => '¤', // U+00A4
        0xB4 => '×', // U+00D7
        0xB8 => '÷', // U+00F7
        0xE0 => 'Ω', // U+2126
        0xE1 => 'Æ', // U+00C6
        0xE2 => 'Ð', // U+00D0
        0xE3 => 'ª', // U+00AA
        0xE4 => 'Ħ', // U+0126
        0xE6 => 'Ĳ', // U+0132
        0xE7 => 'Ŀ', // U+013F
        0xE8 => 'Ł', // U+0141
        0xE9 => 'Ø', // U+00D8
        0xEA => 'Œ', // U+0152
        0xEB => 'º', // U+00BA
        0xEC => 'Þ', // U+00DE
        0xED => 'Ŧ', // U+0166
        0xEE => 'Ŋ', // U+014A
        0xEF => 'ŉ', // U+0149
        0xF0 => 'ĸ', // U+0138
        0xF1 => 'æ', // U+00E6
        0xF2 => 'đ', // U+0111
        0xF3 => 'ð', // U+00F0
        0xF4 => 'ħ', // U+0127
        0xF5 => 'ı', // U+0131
        0xF6 => 'ĳ', // U+0133
        0xF7 => 'ŀ', // U+0140
        0xF8 => 'ł', // U+0142
        0xF9 => 'ø', // U+00F8
        0xFA => 'œ', // U+0153
        0xFB => 'ß', // U+00DF
        0xFC => 'þ', // U+00FE
        0xFD => 'ŧ', // U+0167
        0xFE => 'ŋ', // U+014B

        // Diacritics
        0xC1 => '\u{0300}',
        0xC2 => '\u{0301}',
        0xC3 => '\u{0302}',
        0xC4 => '\u{0303}',
        0xC5 => '\u{0304}',
        0xC6 => '\u{0306}',
        0xC7 => '\u{0307}',
        0xC8 => '\u{0308}',
        0xC9 => '\u{0308}',
        0xCA => '\u{030A}',
        0xCB => '\u{0327}',
        0xCC => '\u{0332}',
        0xCD => '\u{030B}',
        0xCE => '\u{0328}',
        0xCF => '\u{030C}',

        anything_else => anything_else.into(),
    }
}

pub fn is_teletex_diacritic (c: u8) -> bool {
    (c > 0xC0) && (c <= 0xCF)
}

/// This function does NOT handle unrecognized Teletex codepoints. They will be
/// unchanged and interpreted directly as UTF-8 codepoints.
pub fn teletex_to_utf8 (bytes: &[u8]) -> Cow<str> {
    let mut start_of_non_ascii: Option<usize> = None;
    let mut translation: String = String::new();
    let mut current_diacritic: Option<char> = None;
    for (i, b) in bytes.iter().enumerate() {
        let byte = *b;
        if byte.is_ascii() {
            continue;
        }
        if start_of_non_ascii.is_none() {
            start_of_non_ascii = Some(i);
            /* Design decision: we pre-allocate a String that is twice as
            large as the remaining bytes to accommodate diacritics. */
            translation = String::with_capacity((bytes.len() - i) << 1);
            break;
        }
    }
    if let Some(s) = start_of_non_ascii {
        for b in &bytes[s..] {
            let byte = *b;
            if let Some(diac) = current_diacritic {
                // In UTF-8 the diacritic comes after the letter.
                // In Teletex, it comes before. We swap here.
                translation.push(teletex_char_to_utf8_char(byte));
                translation.push(diac);
                current_diacritic = None;
                continue;
            }
            if is_teletex_diacritic(byte) {
                current_diacritic = Some(teletex_char_to_utf8_char(byte));
                continue;
            }
            translation.push(teletex_char_to_utf8_char(byte));
        }
        Cow::Owned([
            unsafe { std::str::from_utf8_unchecked(&bytes[0..s]) },
            &translation,
        ].concat())
    } else {
        Cow::Borrowed(unsafe { std::str::from_utf8_unchecked(bytes) })
    }
}

#[cfg(test)]
mod tests {
    use super::teletex_to_utf8;
    use std::borrow::Cow;

    #[test]
    fn it_returns_pure_ascii_as_borrowed_string() {
        let input = "Jonathan M. Wilbur";
        match teletex_to_utf8(input.as_bytes()) {
            Cow::Borrowed(s) => assert_eq!(s, input),
            _ => panic!(),
        }
    }

    #[test]
    fn it_translates_unequivalent_chars() {
        let input: Vec<u8> = Vec::from([ b'B', b'i', b'g', 0xA4, b'M', b'o', b'n', b'e', b'y', 0xA4 ]);
        match teletex_to_utf8(&input) {
            Cow::Owned(s) => assert_eq!(&s, "Big$Money$"),
            _ => panic!(),
        }
    }

    #[test]
    fn it_transposes_and_translates_diacritics() {
        let input: Vec<u8> = Vec::from([ b'B', b'i', b'g', b'B', 0xC4, b'o', 0xC5, b'i' ]);
        match teletex_to_utf8(&input) {
            Cow::Owned(s) => assert_eq!(&s, "BigBo\u{0303}i\u{0304}"),
            _ => panic!(),
        }
    }

}
