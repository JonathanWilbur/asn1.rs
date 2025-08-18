//! Utilities for parsing and encoding LDAP `PostalAddress`
//!
//! The LDAP `PostalAddress` syntax is defined in
//! [IETF RFC 4517, Section 3.3.28](https://datatracker.ietf.org/doc/html/rfc4517#section-3.3.28).
//! Lines of the postal address are separated by dollar signs. Dollar signs and
//! backslashes that appear in the postal address are escaped by being
//! translated to `\24` and `\5C` respectively (case-exact).
//!
//! This crate is `no_std`, but `alloc` is needed if you want to use
//! `unescape_postal_address_line` or `escape_postal_address_line`.
//!
//! You can parse and unescape LDAP postal addresses like so:
//!
//! ```rust
//! use ldappostaladdr::{parse_postal_address, unescape_postal_address_line};
//! let input = "\\241,000,000 Sweepstakes$PO Box 1000000$Anytown, CA 12345$USA";
//! let mut postal_address = parse_postal_address(input);
//! for (line, backslash_escaped, dollar_escaped) in postal_address {
//!   // This line returns Cow::Borrowed() if the line doesn't contain escape sequences.
//!   let unescaped_line = unescape_postal_address_line(line, backslash_escaped, dollar_escaped);
//!   // `unescaped_line` contains the usable postal address line.
//!   // Use `unescaped_line.as_ref()` to read it without allocating.
//! }
//! ```
//!
//! You can create LDAP postal addresses like so:
//!
//! ```rust
//! use ldappostaladdr::escape_postal_address_line;
//! let lines = vec![
//!     String::from("$1,000,000 Sweepstakes"),
//!     String::from("123 Main St."),
//!     String::from("Anytown, PA 12345"),
//!     String::from("USA"),
//! ];
//! let output = lines.iter()
//!     .map(|line| escape_postal_address_line(line).into_owned())
//!     .collect::<Vec<String>>()
//!     .join("$");
//! assert_eq!(output.as_str(), "\\241,000,000 Sweepstakes$123 Main St.$Anytown, PA 12345$USA");
//! ```
//!
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::iter::{Iterator, FusedIterator, DoubleEndedIterator};

/// Unescape an LDAP `PostalAddress` line
///
/// This function converts `\5C` to `\` and `\24` to `$`. A new string is only
/// allocated if one of these escaped characters are encountered, otherwise,
/// `Cow::Borrowed(_)` returns `line` unchanged.
#[cfg(feature = "alloc")]
#[inline]
pub fn unescape_postal_address_line(
    line: &str,
    backslash_escaped: bool,
    dollar_escaped: bool,
) -> Cow<str> {
    // NOTE: I don't believe the casing of 5C matters: IETF RFC 4517 specifically
    // uses %x5C "5C" as the grammatical production for the escaped backslash.
    match (backslash_escaped, dollar_escaped) {
        (true, true) => Cow::Owned(line.replace("\\5C", "\\").replace("\\24", "$")),
        (true, false) => Cow::Owned(line.replace("\\5C", "\\")),
        (false, true) => Cow::Owned(line.replace("\\24", "$")),
        (false, false) => Cow::Borrowed(line),
    }
}

/// Escape an LDAP `PostalAddress` line
///
/// This function converts `\` to `\5C` and `$` to `\24`. A new string is only
/// allocated if one of these escaped characters are encountered, otherwise,
/// `Cow::Borrowed(_)` returns `line` unchanged.
#[cfg(feature = "alloc")]
pub fn escape_postal_address_line(line: &str) -> Cow<str> {
    // Loops to check for escaping we need to do.
    let mut backslash: bool = false;
    let mut dollar: bool = false;
    for c in line.chars() {
        if c == '$' {
            dollar = true;
            continue;
        }
        if c == '\\' {
            backslash = true;
        }
    }
    // NOTE: I don't believe the casing of 5C matters: IETF RFC 4517 specifically
    // uses %x5C "5C" as the grammatical production for the escaped backslash.
    match (backslash, dollar) {
        (true, true) => Cow::Owned(line.replace("\\", "\\5C").replace("$", "\\24")),
        (true, false) => Cow::Owned(line.replace("\\", "\\5C")),
        (false, true) => Cow::Owned(line.replace("$", "\\24")),
        (false, false) => Cow::Borrowed(line),
    }
}

/// An iterator over the lines in an LDAP `PostalAddress`
///
/// **WARNING**: This iterator **DOES NOT** unescape the postal address lines.
/// Instead, it yields a `str` slice of the escaped line and two `bool`s to
/// indicate whether `$` or `\` need unescaping, respectively.
pub struct PostalAddressLineIter<'a> {
    input: &'a str,
}

impl <'a> PostalAddressLineIter<'a> {

    /// Create a new `PostalAddressLineIter`
    #[inline]
    pub(crate) fn new(input: &'a str) -> Self {
        PostalAddressLineIter{ input }
    }

}

impl <'a> Iterator for PostalAddressLineIter<'a> {

    /// a `str` slice of the escaped line, and whether `\` or `$` need unescaping, in that order
    ///
    /// To clarify, if both `bool`s are `false`, you can just use the `str` as
    /// it is; if either is `true`, the `str` contains one of the escape
    /// sequences: `\5C` or `\24`.
    type Item = (&'a str, bool, bool);

    /// Yields the next `(esc_line, needs_backslash_unesc, needs_dollar_unesc)`
    ///
    /// To clarify, if both `bool`s are `false`, you can just use the `str` as
    /// it is; if either is `true`, the `str` contains one of the escape
    /// sequences: `\5C` or `\24`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() == 0 {
            return None;
        }
        let mut backslash_escaped: bool = false;
        let mut dollar_escaped: bool = false;
        for (i, c) in self.input.char_indices() {
            if c == '$' {
                let ret = &self.input[0..i];
                self.input = &self.input[i+1..];
                return Some((ret, backslash_escaped, dollar_escaped));
            }
            if c == '\\' {
                if self.input[i+1..].starts_with("5C") {
                    backslash_escaped = true;
                } else if self.input[i+1..].starts_with("24") {
                    dollar_escaped = true;
                }
                continue;
            }
        }
        let ret = self.input;
        self.input = &self.input[0..0]; // Empty to terminate further iteration.
        Some((ret, backslash_escaped, dollar_escaped))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.input.len() == 0 {
            return (0, Some(0));
        }
        (1, Some(1 + self.input.len()))
    }

}

impl <'a> FusedIterator for PostalAddressLineIter<'a> {}

impl <'a> DoubleEndedIterator for PostalAddressLineIter<'a> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.input.len() == 0 {
            return None;
        }
        let mut backslash_escaped: bool = false;
        let mut dollar_escaped: bool = false;
        for (i, c) in self.input.char_indices().rev() {
            if c == '$' {
                let ret = &self.input[i+1..];
                self.input = &self.input[0..i];
                return Some((ret, backslash_escaped, dollar_escaped));
            }
            if c == '\\' {
                if self.input[i+1..].starts_with("5C") {
                    backslash_escaped = true;
                } else if self.input[i+1..].starts_with("24") {
                    dollar_escaped = true;
                }
                continue;
            }
        }
        let ret = self.input;
        self.input = &self.input[0..0]; // Empty to terminate further iteration.
        Some((ret, backslash_escaped, dollar_escaped))
    }

}

/// Parse an LDAP `PostalAddress`, line-by-line
///
/// This function trivially returns a `PostalAddressLineIter`.
///
/// **WARNING**: The returned iterator **DOES NOT** unescape the postal address
/// lines. Instead, it yields a `str` slice of the escaped line and two `bool`s
/// to indicate whether `$` or `\` need unescaping, respectively.
///
/// ## Example Usage
///
/// ```rust
/// use ldappostaladdr::{parse_postal_address, unescape_postal_address_line};
/// let input = "\\241,000,000 Sweepstakes$PO Box 1000000$Anytown, CA 12345$USA";
/// let mut postal_address = parse_postal_address(input);
/// for (line, backslash_escaped, dollar_escaped) in postal_address {
///   // This line returns Cow::Borrowed() if the line doesn't contain escape sequences.
///   let unescaped_line = unescape_postal_address_line(line, backslash_escaped, dollar_escaped);
///   // `unescaped_line` contains the usable postal address line.
///   // Use `unescaped_line.as_ref()` to read it without allocating.
/// }
/// ```
///
#[inline]
pub fn parse_postal_address<'a>(input: &'a str) -> PostalAddressLineIter<'a> {
    PostalAddressLineIter::new(input)
}

#[cfg(test)]
mod tests {

    extern crate alloc;
    use alloc::borrow::Cow;
    use super::parse_postal_address;
    #[cfg(feature = "alloc")]
    use super::{unescape_postal_address_line, escape_postal_address_line};

    #[test]
    fn iter_postal_addr_1() {
        let input = "1234 Main St.$Anytown, CA 12345$USA";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next(), Some(("1234 Main St.", false, false)));
        assert_eq!(pa.next(), Some(("Anytown, CA 12345", false, false)));
        assert_eq!(pa.next(), Some(("USA", false, false)));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
    }

    #[test]
    fn iter_postal_addr_2() {
        let input = "\\241,000,000 Sweepstakes$PO Box 1000000$Anytown, CA 12345$USA";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next(), Some(("\\241,000,000 Sweepstakes", false, true)));
        assert_eq!(pa.next(), Some(("PO Box 1000000", false, false)));
        assert_eq!(pa.next(), Some(("Anytown, CA 12345", false, false)));
        assert_eq!(pa.next(), Some(("USA", false, false)));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
    }

    #[test]
    fn iter_postal_addr_3() {
        let input = "1\\5C,000\\5C,000 \\24weepstakes$Anytown\\AB, CA 12345\\\\$\\\\USA\\\\5C";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next(), Some(("1\\5C,000\\5C,000 \\24weepstakes", true, true)));
        assert_eq!(pa.next(), Some(("Anytown\\AB, CA 12345\\\\", false, false)));
        assert_eq!(pa.next(), Some(("\\\\USA\\\\5C", true, false)));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
    }

    #[test]
    fn rev_iter_postal_addr_1() {
        let input = "1234 Main St.$Anytown, CA 12345$USA";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next_back(), Some(("USA", false, false)));
        assert_eq!(pa.next(), Some(("1234 Main St.", false, false)));
        assert_eq!(pa.next_back(), Some(("Anytown, CA 12345", false, false)));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next_back(), None);
        assert_eq!(pa.next_back(), None);
    }

    #[test]
    fn rev_iter_postal_addr_2() {
        let input = "\\241,000,000 Sweepstakes$PO Box 1000000$Anytown, CA 12345$USA";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next_back(), Some(("USA", false, false)));
        assert_eq!(pa.next_back(), Some(("Anytown, CA 12345", false, false)));
        assert_eq!(pa.next_back(), Some(("PO Box 1000000", false, false)));
        assert_eq!(pa.next_back(), Some(("\\241,000,000 Sweepstakes", false, true)));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next_back(), None);
        assert_eq!(pa.next_back(), None);
    }

    #[test]
    fn rev_iter_postal_addr_3() {
        let input = "1\\5C,000\\5C,000 \\24weepstakes$Anytown\\AB, CA 12345\\\\$\\\\USA\\\\5C";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next_back(), Some(("\\\\USA\\\\5C", true, false)));
        assert_eq!(pa.next_back(), Some(("Anytown\\AB, CA 12345\\\\", false, false)));
        assert_eq!(pa.next_back(), Some(("1\\5C,000\\5C,000 \\24weepstakes", true, true)));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next_back(), None);
        assert_eq!(pa.next_back(), None);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn unescape_1() {
        let input = "1\\5C,000\\5C,000 \\24weepstakes";
        let pa = unescape_postal_address_line(input, true, true);
        assert!(matches!(pa, Cow::Owned(_)));
        assert_eq!(pa.as_ref(), "1\\,000\\,000 $weepstakes");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn unescape_2() {
        let input = "\\\\USA\\\\5C";
        let pa = unescape_postal_address_line(input, true, false);
        assert!(matches!(pa, Cow::Owned(_)));
        assert_eq!(pa.as_ref(), "\\\\USA\\\\");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn unescape_3() {
        let input = "Anytown, CA 12345";
        let pa = unescape_postal_address_line(input, false, false);
        assert!(matches!(pa, Cow::Borrowed(_)));
        assert_eq!(pa.as_ref(), "Anytown, CA 12345");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn escape_1() {
        let input = "1\\,000\\,000 $weepstakes";
        let pa = escape_postal_address_line(input);
        assert!(matches!(pa, Cow::Owned(_)));
        assert_eq!(pa.as_ref(), "1\\5C,000\\5C,000 \\24weepstakes");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn escape_2() {
        let input = "\\\\USA\\\\";
        let pa = escape_postal_address_line(input);
        assert!(matches!(pa, Cow::Owned(_)));
        assert_eq!(pa.as_ref(), "\\5C\\5CUSA\\5C\\5C");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn escape_3() {
        let input = "Anytown, CA 12345";
        let pa = escape_postal_address_line(input);
        assert!(matches!(pa, Cow::Borrowed(_)));
        assert_eq!(pa.as_ref(), "Anytown, CA 12345");
    }

}
