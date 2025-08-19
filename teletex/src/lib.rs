#![doc = include_str!("../README.md")]
#![no_std]
use core::iter::{Iterator, FusedIterator, ExactSizeIterator};

/// Return `true` if the character `c` is a diacritic in Teletex
#[inline]
pub const fn is_teletex_diacritic (c: u8) -> bool {
    (c > 0xC0) && (c <= 0xCF)
}

/// Convert a single Teletex character to its equivalent Unicode character
pub const fn teletex_char_to_utf8_char (c: u8) -> char {
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

        anything_else => if anything_else.is_ascii() {
            anything_else as char
        } else {
            '\u{FFFD}' // Replacement character �
        },
    }
}

/// Iterator over the conversion of Teletex to UTF-8
pub struct TeletexToUnicodeChars<'a> {
    teletex: &'a [u8],
    diacritic: Option<char>,
}

impl <'a> TeletexToUnicodeChars<'a> {

    #[inline]
    pub(crate) const fn new(teletex: &'a [u8]) -> TeletexToUnicodeChars<'a> {
        TeletexToUnicodeChars{ teletex, diacritic: None }
    }

}

const REPLACEMENT_CHAR: char = '\u{FFFD}';

impl <'a> Iterator for TeletexToUnicodeChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(diac) = self.diacritic.take() {
            return Some(diac);
        }
        let tb = *self.teletex.first()?;
        self.teletex = &self.teletex[1..];
        if !is_teletex_diacritic(tb) {
            return Some(teletex_char_to_utf8_char(tb));
        }
        let tb2 = *self.teletex.first()?;
        self.teletex = &self.teletex[1..];
        if !tb2.is_ascii_alphabetic() {
            // If the diacritic seems misplaced, just return replacement character.
            return Some(REPLACEMENT_CHAR);
        }
        // In UTF-8 the diacritic comes after the letter.
        // In Teletex, it comes before. We swap here.
        self.diacritic = Some(teletex_char_to_utf8_char(tb));
        Some(teletex_char_to_utf8_char(tb2))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.teletex.len() + if self.diacritic.is_some() { 1 } else { 0 };
        (len, Some(len))
    }

}

impl <'a> FusedIterator for TeletexToUnicodeChars<'a> {}
impl <'a> ExactSizeIterator for TeletexToUnicodeChars<'a> {}

/// Convert Teletex to UTF-8, character by character.
///
/// ## Example Usage
///
/// ```rust
/// use teletex::teletex_to_utf8;
/// let input = b"Big\xA4Money\xA4";
/// for utf8char in teletex_to_utf8(input) {
///     // ...utf8char is the UTF-8 equivalent
/// }
/// ```
#[inline]
pub const fn teletex_to_utf8 <'a> (bytes: &'a [u8]) -> TeletexToUnicodeChars<'a> {
    TeletexToUnicodeChars::new(bytes)
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::teletex_to_utf8;
    use alloc::string::String;

    #[test]
    fn it_translates_unequivalent_chars() {
        let input = b"Big\xA4Money\xA4";
        let output: String = teletex_to_utf8(input).collect();
        assert_eq!(output.as_str(), "Big$Money$");
    }

    #[test]
    fn it_transposes_and_translates_diacritics() {
        let input = b"BigB\xC4o\xC5i";
        let output: String = teletex_to_utf8(input).collect();
        assert_eq!(output.as_str(), "BigBo\u{0303}i\u{0304}");
    }

    #[test]
    fn it_decodes_an_empty_string() {
        let input = b"";
        let output: String = teletex_to_utf8(input).collect();
        assert_eq!(output.as_str(), "");
    }

}
