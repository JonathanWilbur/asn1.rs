#![doc = include_str!("../README.md")]
#![no_std]
// Some things in this file were copied from `stringprep` crate.
// License at the time of copying:
// Copyright (c) 2017 The rust-stringprep Developers

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod rfc3454;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

use core::iter::{Filter, FlatMap, FusedIterator, Iterator, Map};
use core::slice::Iter;
use core::str::Chars;
use unicode_normalization::{Recompositions, UnicodeNormalization};

const REPLACEMENT_CHARACTER: char = '\u{FFFD}';

/// Determine if a character is an unassigned code point
///
/// This is not 100% correct: there are many more unassigned ranges; however,
/// this covers a huge range of them. This sloppy approximation is not done
/// out of laziness, but rather, in the interest of performance. Also, many
/// unassigned code points may become assigned in the future, so this less
/// strict check is a little more future-proof.
///
/// No new code points are even proposed for planes 4 to 13. Plane 3 is
/// entirely unassigned.
#[inline]
const fn is_unassigned(c: char) -> bool {
    c >= '\u{30000}' && c <= '\u{DFFFF}'
}

/// Determine if a character is a private use code point
#[inline]
const fn is_private_use(c: char) -> bool {
    matches!(c, '\u{E000}'..='\u{F8FF}' | '\u{F0000}'..='\u{FFFFD}' | '\u{100000}'..='\u{10FFFD}')
}

/// Determine if a character is a non-character code point
#[inline]
const fn is_non_char(c: char) -> bool {
    let bottom_nybble = c as u32 & 0xFFFF;
    if bottom_nybble >= 0xFFFE && bottom_nybble <= 0xFFFF {
        return true;
    }
    matches!(c, '\u{FDD0}'..='\u{FDEF}')
}

/// Determines if `c` is to be removed according to section 7.2 of
/// [ITU-T Recommendation X.520 (2019)](https://www.itu.int/rec/T-REC-X.520-201910-I/en).
#[inline]
fn x520_mapped_to_something(c: &char) -> bool {
    match *c {
        '\u{00AD}'
        | '\u{1806}'
        | '\u{034F}'
        | '\u{180B}'..='\u{180D}'
        | '\u{FE00}'..='\u{FE0F}'
        | '\u{FFFC}'
        | '\u{200B}' => false,
        // Technically control characters, but mapped to whitespace in X.520.
        '\u{09}' | '\u{0A}'..='\u{0D}' | '\u{85}' => true,
        _ => !c.is_control(),
    }
}

#[inline]
fn is_separator(c: char) -> bool {
    match c {
        | '\u{20}' // SPACE
        | '\u{a0}' // NO-BREAK SPACE
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
        | '\u{1680}' // OGHAM SPACE MARK
        | '\u{2000}'..='\u{200a}' // Width-specific spaces
        | '\u{202f}' // NARROW NO-BREAK SPACE
        | '\u{205f}' // MEDIUM MATHEMATICAL SPACE
        | '\u{3000}' // IDEOGRAPHIC SPACE
        => true,
        _ => false,
    }
}

#[inline]
fn x520_map(c: char) -> char {
    match c {
        '\u{09}' | '\u{0A}'..='\u{0D}' | '\u{85}' => ' ',
        c => {
            if is_separator(c) {
                ' '
            } else {
                c
            }
        }
    }
}

/// B.2 Mapping for case-folding used with NFKC.
#[inline]
fn case_fold_for_nfkc(c: char) -> CaseFoldForNfkc {
    let inner = match rfc3454::B_2.binary_search_by_key(&c, |e| e.0) {
        Ok(idx) => FoldInner::Chars(rfc3454::B_2[idx].1.chars()),
        Err(_) => FoldInner::Char(Some(c)),
    };
    CaseFoldForNfkc(inner)
}

enum FoldInner {
    Chars(Chars<'static>),
    Char(Option<char>),
}

/// The iterator returned by `case_fold_for_nfkc`.
struct CaseFoldForNfkc(FoldInner);

impl Iterator for CaseFoldForNfkc {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.0 {
            FoldInner::Chars(ref mut it) => it.next(),
            FoldInner::Char(ref mut ch) => ch.take(),
        }
    }
}

impl FusedIterator for CaseFoldForNfkc {}

pub struct X520CaseExactStringPrepChars<I>
    where I: Iterator<Item = char> {
    s: Recompositions<Map<Filter<I, fn(&char) -> bool>, fn(char) -> char>>,
    previous_was_space: bool,
}

impl<I> X520CaseExactStringPrepChars<I>
    where I: Iterator<Item = char> {
    pub fn new(s: I) -> Self {
        X520CaseExactStringPrepChars {
            previous_was_space: false,
            s: s
                .filter(x520_mapped_to_something as fn(&char) -> bool)
                .map(x520_map as fn(_) -> _)
                .nfkc(),
        }
    }
}

impl<I> Iterator for X520CaseExactStringPrepChars<I>
    where I: Iterator<Item = char> {
    /// The error is returned if a prohibited character is encountered.
    type Item = Result<char, char>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.s.next() {
            if c == ' ' {
                if self.previous_was_space == true {
                    continue;
                } else {
                    self.previous_was_space = true;
                    return Some(Ok(' '));
                }
            }
            self.previous_was_space = false;
            // You don't have to check for Surrogate codes: it is not possible
            // in UTF-8 strings in Rust.
            if is_unassigned(c) || is_private_use(c) || is_non_char(c) || c == '\u{FFFD}' {
                // Prohibited character: matching MUST return UNDEFINED.
                return Some(Err(c));
            }
            return Some(Ok(c));
        }
        None
    }
}

impl<I> FusedIterator for X520CaseExactStringPrepChars<I>
    where I: Iterator<Item = char> {
}

pub struct X520CaseIgnoreStringPrepChars<I>
    where I: Iterator<Item = char> {
    s: Recompositions<
        FlatMap<
            Map<Filter<I, fn(&char) -> bool>, fn(char) -> char>,
            CaseFoldForNfkc,
            fn(char) -> CaseFoldForNfkc,
        >,
    >,
    previous_was_space: bool,
}

impl<I> FusedIterator for X520CaseIgnoreStringPrepChars<I>
    where I: Iterator<Item = char> {
}

impl<I> X520CaseIgnoreStringPrepChars<I>
    where I: Iterator<Item = char> {
    pub fn new(s: I) -> Self {
        X520CaseIgnoreStringPrepChars {
            previous_was_space: false,
            s: s
                .filter(x520_mapped_to_something as fn(&char) -> bool)
                .map(x520_map as fn(_) -> _)
                .flat_map(case_fold_for_nfkc as fn(_) -> _)
                .nfkc(),
        }
    }
}

impl<I> Iterator for X520CaseIgnoreStringPrepChars<I>
    where I: Iterator<Item = char> {
    /// The error is returned if a prohibited character is encountered.
    type Item = Result<char, char>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.s.next() {
            if c == ' ' {
                if self.previous_was_space == true {
                    continue;
                } else {
                    self.previous_was_space = true;
                    return Some(Ok(' '));
                }
            }
            self.previous_was_space = false;
            // You don't have to check for Surrogate codes: it is not possible
            // in UTF-8 strings in Rust.
            if is_unassigned(c) || is_private_use(c) || is_non_char(c) || c == '\u{FFFD}' {
                // Prohibited character: matching MUST return UNDEFINED.
                return Some(Err(c));
            }
            return Some(Ok(c));
        }
        None
    }
}

/// Iterate over characters string-prepped per ITU-T Recommendation X.520 (case-sensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_exact_str<'a>(s: &'a str) -> X520CaseExactStringPrepChars<Chars<'a>> {
    X520CaseExactStringPrepChars::new(s.chars())
}

/// Iterate over characters string-prepped per ITU-T Recommendation X.520 (case-insensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_ignore_str<'a>(s: &'a str) -> X520CaseIgnoreStringPrepChars<Chars<'a>> {
    X520CaseIgnoreStringPrepChars::new(s.chars())
}

/// Iterate over characters of a string-prepped `BMPString` per ITU-T Recommendation X.520 (case-sensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_exact_bmp<'a>(s: &'a [u16]) -> X520CaseExactStringPrepChars<Map<Iter<'a, u16>, fn(&u16) -> char>> {
    let it: Map<Iter<'a, u16>, fn(&u16) -> char> = s
        .iter()
        .map(|c| char::from_u32(*c as u32).unwrap_or(REPLACEMENT_CHARACTER));
    X520CaseExactStringPrepChars::new(it)
}

/// Iterate over characters of a string-prepped `BMPString` per ITU-T Recommendation X.520 (case-insensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_ignore_bmp<'a>(s: &'a [u16]) -> X520CaseIgnoreStringPrepChars<Map<Iter<'a, u16>, fn(&u16) -> char>> {
    let it: Map<Iter<'a, u16>, fn(&u16) -> char> = s
        .iter()
        .map(|c| char::from_u32(*c as u32).unwrap_or(REPLACEMENT_CHARACTER));
    X520CaseIgnoreStringPrepChars::new(it)
}

/// Iterate over characters of a string-prepped `UniversalString` per ITU-T Recommendation X.520 (case-sensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_exact_univ_str<'a>(s: &'a [u32]) -> X520CaseExactStringPrepChars<Map<Iter<'a, u32>, fn(&u32) -> char>> {
    let it: Map<Iter<'a, u32>, fn(&u32) -> char> = s
        .iter()
        .map(|c| char::from_u32(*c as u32).unwrap_or(REPLACEMENT_CHARACTER));
    X520CaseExactStringPrepChars::new(it)
}

/// Iterate over characters of a string-prepped `UniversalString` per ITU-T Recommendation X.520 (case-insensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_ignore_univ_str<'a>(s: &'a [u32]) -> X520CaseIgnoreStringPrepChars<Map<Iter<'a, u32>, fn(&u32) -> char>> {
    let it: Map<Iter<'a, u32>, fn(&u32) -> char> = s
        .iter()
        .map(|c| char::from_u32(*c).unwrap_or(REPLACEMENT_CHARACTER));
    X520CaseIgnoreStringPrepChars::new(it)
}

/// Check if a string is already string-prepped per ITU-T Recommendation X.520 (case-sensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
pub fn is_x520_stringprepped_case_exact_str(s: &str) -> bool {
    let mut chars = s.chars();
    let mut it = x520_stringprep_case_exact_str(s);
    while let Some(c) = it.next() {
        if c.is_err() {
            return false;
        }
        if chars.next() != Some(c.unwrap()) {
            return false;
        }
    }
    true
}

/// Check if a string is already string-prepped per ITU-T Recommendation X.520 (case-insensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
pub fn is_x520_stringprepped_case_ignore_str(s: &str) -> bool {
    let mut chars = s.chars();
    let mut it = x520_stringprep_case_ignore_str(s);
    while let Some(c) = it.next() {
        if c.is_err() {
            return false;
        }
        if chars.next() != Some(c.unwrap()) {
            return false;
        }
    }
    true
}

/// Normalize a string to case-exact form.
/// 
/// Returns an error if the string contains prohibited characters. The error
/// itself contains the prohibited character.
#[cfg(feature = "alloc")]
#[inline]
pub fn x520_stringprep_to_case_exact_string(s: &str) -> Result<String, char> {
    x520_stringprep_case_exact_str(s).collect()
}

/// Normalize a string to case-ignore form.
/// 
/// Returns an error if the string contains prohibited characters. The error
/// itself contains the prohibited character.
#[cfg(feature = "alloc")]
#[inline]
pub fn x520_stringprep_to_case_ignore_string(s: &str) -> Result<String, char> {
    x520_stringprep_case_ignore_str(s).collect()
}

/// Compare two strings for equality, string-prepped per ITU-T Recommendation X.520 (case-sensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_exact_compare(s1: &str, s2: &str) -> bool {
    x520_stringprep_case_exact_str(s1).eq(x520_stringprep_case_exact_str(s2))
}

/// Compare two strings for equality, string-prepped per ITU-T Recommendation X.520 (case-insensitive)
/// 
/// This does NOT trim leading or trailing whitespace: that is left to the caller.
#[inline]
pub fn x520_stringprep_case_ignore_compare(s1: &str, s2: &str) -> bool {
    x520_stringprep_case_ignore_str(s1).eq(x520_stringprep_case_ignore_str(s2))
}

#[cfg(test)]
mod tests {
    use super::{
        x520_stringprep_case_exact_str,
        x520_stringprep_case_ignore_str,
        x520_stringprep_case_exact_bmp,
        x520_stringprep_case_exact_univ_str,
    };
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    #[test]
    fn test_case_exact_stringprep_1() {
        let input = "Jonathan   Wilbur";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output.as_str(), "Jonathan Wilbur");
    }

    #[test]
    fn test_nfkc_normalization() {
        // Test NFKC normalization - combining characters should be normalized
        let input = "e\u{0301}"; // e + combining acute accent
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "é"); // Should be normalized to precomposed é

        // Test with multiple combining characters
        let input = "e\u{0301}\u{0300}"; // e + acute + grave
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "é\u{0300}"); // Should be normalized but may not combine further
    }

    #[test]
    fn test_whitespace_mapping() {
        // Test ASCII whitespace characters
        let input = "Hello\tWorld\nTest\r\nSpace";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World Test Space");

        // Test Unicode separator characters
        let input = "Hello\u{2000}World\u{2001}Test\u{2002}Space"; // Various Unicode spaces
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World Test Space");

        // Test mixed whitespace
        let input = "Hello\t\u{2000}World\n\u{2001}Test\r\u{2002}Space";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World Test Space");
    }

    #[test]
    fn test_space_consolidation() {
        // Test multiple consecutive spaces
        let input = "Hello    World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");

        // Test mixed whitespace consolidation
        let input = "Hello\t\t\n\n\r\rWorld";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");

        // Test Unicode spaces consolidation
        let input = "Hello\u{2000}\u{2001}\u{2002}World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");

        // Test mixed ASCII and Unicode spaces
        let input = "Hello \t\u{2000}\nWorld";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");
    }

    #[test]
    fn test_leading_trailing_spaces() {
        // Test leading spaces - function preserves one leading space
        let input = "   Hello World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, " Hello World");

        // Test trailing spaces - function preserves one trailing space
        let input = "Hello World   ";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World ");

        // Test both leading and trailing spaces
        let input = "   Hello World   ";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, " Hello World ");
    }

    #[test]
    fn test_prohibited_characters() {
        // // Test unassigned characters (should return Err)
        // let input = "Hello\u{0378}World"; // U+0378 is unassigned
        // let result: Result<String, char> = x520_stringprep_case_exact_str(input)
        //     .collect();
        // assert!(result.is_err());
        // assert_eq!(result.unwrap_err(), '\u{0378}');

        // Test private use characters (should return Err)
        let input = "Hello\u{E000}World"; // U+E000 is private use
        let result: Result<String, char> = x520_stringprep_case_exact_str(input).collect();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), '\u{E000}');

        // Test non-character code points (should return Err)
        let input = "Hello\u{FDD0}World"; // U+FDD0 is non-character
        let result: Result<String, char> = x520_stringprep_case_exact_str(input).collect();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), '\u{FDD0}');

        // Test replacement character (should return Err)
        let input = "Hello\u{FFFD}World";
        let result: Result<String, char> = x520_stringprep_case_exact_str(input).collect();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), '\u{FFFD}');
    }

    #[test]
    fn test_control_characters() {
        // Test that control characters are mapped to spaces
        let input = "Hello\u{0009}World"; // Horizontal tab
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");

        // Test line feed
        let input = "Hello\u{000A}World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");

        // Test carriage return
        let input = "Hello\u{000D}World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");

        // Test next line
        let input = "Hello\u{0085}World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "Hello World");
    }

    #[test]
    fn test_filtered_characters() {
        // Test characters that should be filtered out (not mapped to space)
        let input = "Hello\u{00AD}World"; // Soft hyphen - should be filtered out
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "HelloWorld");

        // Test zero-width space
        let input = "Hello\u{200B}World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "HelloWorld");

        // Test object replacement character
        let input = "Hello\u{FFFC}World";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "HelloWorld");
    }

    #[test]
    fn test_complex_normalization() {
        // Test a complex case with multiple transformations
        let input = "  Hello\te\u{0301}\u{2000}Ä\u{FB03}n  ";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, " Hello é Äffin ");

        let output: String = x520_stringprep_case_ignore_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, " hello é äffin ");
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "");

        let output: String = x520_stringprep_case_ignore_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, "");
    }

    #[test]
    fn test_only_spaces() {
        let input = "   \t\n\r   ";
        let output: String = x520_stringprep_case_exact_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, " ");

        let output: String = x520_stringprep_case_ignore_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output, " ");
    }

    #[test]
    fn test_case_ignore_stringprep_1() {
        let input = "Jonathan   Wilbur";
        let output: String = x520_stringprep_case_ignore_str(input)
            .map(|maybe_c| maybe_c.unwrap())
            .collect();
        assert_eq!(output.as_str(), "jonathan wilbur");
    }

    // This test is just to make sure we can use this for `BMPString`.
    #[test]
    fn test_bmp_string_1() {
        let input: Vec<u16> = "Jonathan   Wilbur".encode_utf16().collect();
        let output = x520_stringprep_case_exact_bmp(input.as_slice()).collect::<Result<String, char>>().unwrap();
        assert_eq!(output.as_str(), "Jonathan Wilbur");
    }

    // This test is just to make sure we can use this for `UniversalString`.
    #[test]
    fn test_univ_string_1() {
        let input: Vec<u32> = "Jonathan   Wilbur".chars().map(|c| c as u32).collect();
        let output = x520_stringprep_case_exact_univ_str(input.as_slice()).collect::<Result<String, char>>().unwrap();
        assert_eq!(output.as_str(), "Jonathan Wilbur");
    }

}
