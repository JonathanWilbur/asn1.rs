// TODO: Move this code to a separate module
// TODO: Also implement double-ended iterator
use std::borrow::Cow;
use std::iter::{Iterator, FusedIterator};

pub struct PostalAddressLineIter<'a> {
    input: &'a str,
}

impl <'a> PostalAddressLineIter<'a> {

    #[inline]
    pub fn new(input: &'a str) -> Self {
        PostalAddressLineIter{ input }
    }

}

impl <'a> Iterator for PostalAddressLineIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() == 0 {
            return None;
        }
        let mut backslash_escaped: bool = false;
        let mut dollar_escaped: bool = false;
        for (i, c) in self.input.char_indices() {
            if c == '$' {
                let ret = match (backslash_escaped, dollar_escaped) {
                    (true, true) => Cow::Owned(self.input[0..i].replace("\\5C", "\\").replace("\\24", "$")),
                    (true, false) => Cow::Owned(self.input[0..i].replace("\\5C", "\\")),
                    (false, true) => Cow::Owned(self.input[0..i].replace("\\24", "$")),
                    (false, false) => Cow::Borrowed(&self.input[0..i]),
                };
                self.input = &self.input[i+1..];
                return Some(ret);
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
        let ret = match (backslash_escaped, dollar_escaped) {
            (true, true) => Cow::Owned(self.input.replace("\\5C", "\\").replace("\\24", "$")),
            (true, false) => Cow::Owned(self.input.replace("\\5C", "\\")),
            (false, true) => Cow::Owned(self.input.replace("\\24", "$")),
            (false, false) => Cow::Borrowed(self.input),
        };
        self.input = &self.input[0..0]; // Empty to terminate further iteration.
        Some(ret)
    }

}

impl <'a> FusedIterator for PostalAddressLineIter<'a> {}

pub(crate) fn parse_postal_address<'a>(input: &'a str) -> PostalAddressLineIter<'a> {
    PostalAddressLineIter::new(input)
}

#[cfg(test)]
mod tests {

    use std::borrow::Cow;
    use super::parse_postal_address;

    #[test]
    fn iter_postal_addr_1() {
        let input = "1234 Main St.$Anytown, CA 12345$USA";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next(), Some(Cow::Borrowed("1234 Main St.")));
        assert_eq!(pa.next(), Some(Cow::Borrowed("Anytown, CA 12345")));
        assert_eq!(pa.next(), Some(Cow::Borrowed("USA")));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
    }

    #[test]
    fn iter_postal_addr_2() {
        let input = "\\241,000,000 Sweepstakes$PO Box 1000000$Anytown, CA 12345$USA";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next(), Some(Cow::Owned("$1,000,000 Sweepstakes".to_owned())));
        assert_eq!(pa.next(), Some(Cow::Borrowed("PO Box 1000000")));
        assert_eq!(pa.next(), Some(Cow::Borrowed("Anytown, CA 12345")));
        assert_eq!(pa.next(), Some(Cow::Borrowed("USA")));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
    }

    #[test]
    fn iter_postal_addr_3() {
        let input = "1\\5C,000\\5C,000 \\24weepstakes$Anytown\\AB, CA 12345\\\\$\\\\USA\\\\5C";
        let mut pa = parse_postal_address(input);
        assert_eq!(pa.next(), Some(Cow::Owned("1\\,000\\,000 $weepstakes".to_owned())));
        assert_eq!(pa.next(), Some(Cow::Borrowed("Anytown\\AB, CA 12345\\\\")));
        assert_eq!(pa.next(), Some(Cow::Owned("\\\\USA\\\\".to_owned())));
        assert_eq!(pa.next(), None);
        assert_eq!(pa.next(), None);
    }

}
