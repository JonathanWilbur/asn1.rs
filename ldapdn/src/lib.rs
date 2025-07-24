//! https://datatracker.ietf.org/doc/html/rfc4514
//! 
use std::{borrow::Cow, iter::{FusedIterator}};

// TODO: Would this API be a lot better if the DN iterator just returned an RDN

/// LDAP Attribute Type and Value
///
/// Note that the attribute descriptor cannot contain escape characters, so
/// we always return the slice. The value may have escapes, though.
pub type AttributeTypeAndValue<'a> = (&'a str, &'a str);

pub fn atav_from_str <'a> (s: &'a str) -> Result<AttributeTypeAndValue<'a>, ()> {
    let eq_pos = s.find('=');
    if eq_pos.is_none() {
        return Err(());
    }
    let eq_pos = eq_pos.unwrap();
    let (attribute_type, attribute_value) = s.split_at(eq_pos);
    if attribute_type.is_empty() {
        return Err(());
    }
    // Skip the '=' character
    let attribute_value = &attribute_value[1..];
    Ok((attribute_type.trim(), attribute_value.trim()))
}

#[derive(Debug)]
pub struct RdnIterator<'a> {
    s: &'a str,
    escape: char,
    atav_delimiter: char,
    had_error: bool,
}

impl <'a> Iterator for RdnIterator<'a> {
    type Item = Result<AttributeTypeAndValue<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.s.trim_start();
        if s.is_empty() {
            return None;
        }
        let mut prev_escape = false;
        for (i, c) in s.char_indices() {
            if c == self.escape && !prev_escape {
                prev_escape = true;
                continue;
            }
            if c == self.atav_delimiter && !prev_escape {
                // We've found the end of the attribute type and value
                let atav_str: &str = &s[..i];
                self.s = &s[i+1..];
                let atav = atav_from_str(atav_str);
                if atav.is_err() {
                    self.had_error = true;
                }
                return Some(atav);
            }
            prev_escape = false;
        }
        self.s = "";
        let atav = atav_from_str(s);
        if atav.is_err() {
            self.had_error = true;
        }
        Some(atav)
    }

    /// Every attribute type and value must take up at least two characters:
    /// one for the attribute type and one for the equals sign. Technically,
    /// I think the value cannot be empty, but to be on the safe side, the
    /// size hint can be trivially computed by halving the length of the string.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.s.len() >> 1))
    }

}

impl <'a> FusedIterator for RdnIterator<'a> {}

impl<'a> DoubleEndedIterator for RdnIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let s = self.s.trim_end();
        if s.is_empty() {
            return None;
        }
        let mut prev_delim = false;
        for (i, c) in s.char_indices().rev() {
            if c == self.atav_delimiter {
                prev_delim = true;
                continue;
            }
            if prev_delim && c != self.escape {
                let atav_str: &str = &s[i+c.len_utf8()+self.escape.len_utf8()..];
                self.s = &s[..i+c.len_utf8()];
                let atav = atav_from_str(atav_str);
                if atav.is_err() {
                    self.had_error = true;
                }
                return Some(atav);
            }
            prev_delim = false;
        }
        self.s = "";
        let atav = atav_from_str(s);
        if atav.is_err() {
            self.had_error = true;
        }
        Some(atav)
    }
}

#[inline]
pub fn rdn_from_str_ex <'a> (s: &'a str, escape: char, atav_delimiter: char) -> RdnIterator<'a> {
    RdnIterator{
        s,
        escape,
        atav_delimiter,
        had_error: false,
    }
}

#[inline]
pub fn rdn_from_str <'a> (s: &'a str) -> RdnIterator<'a> {
    if s.is_empty() {
        // Return a fake iterator that will fail.
        return RdnIterator { s: "a", escape: '\\', atav_delimiter: '+', had_error: false };
    }
    rdn_from_str_ex(s, '\\', '+')
}

#[derive(Debug)]
pub struct RdnSequenceIterator<'a> {
    s: &'a str,
    rdn_iter: RdnIterator<'a>,
    escape: char,
    rdn_delimiter: char,
    atav_delimiter: char,
}

impl <'a> Iterator for RdnSequenceIterator<'a> {
    /// The first boolean indicates whether the RDN incremented / decremented.
    /// The second value is an attribute type and value.
    /// 
    /// One slight problem with this is that the boolean will not be the same
    /// exact value when going forwards and backwards over a multi-valued RDN.
    type Item = (bool, Result<AttributeTypeAndValue<'a>, ()>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rdn_iter.had_error {
            return None;
        }
        if let Some(atav) = self.rdn_iter.next() {
            return Some((false, atav));
        }
        let s = self.s.trim_start();
        if s.is_empty() {
            return None;
        }
        let mut prev_escape = false;
        for (i, c) in s.char_indices() {
            if c == self.escape && !prev_escape {
                prev_escape = true;
                continue;
            }
            if c == self.rdn_delimiter && !prev_escape {
                // We've found the end of the relative distinguished name
                let rdn_str: &str = &s[..i];
                if rdn_str.is_empty() {
                    self.s = "";
                    return Some((true, Err(())));
                }
                self.s = &s[i+1..];
                self.rdn_iter = rdn_from_str_ex(rdn_str, self.escape, self.atav_delimiter);
                if let Some(atav) = self.rdn_iter.next() {
                    return Some((true, atav));
                }
            }
            prev_escape = false;
        }
        self.rdn_iter = rdn_from_str_ex(self.s, self.escape, self.atav_delimiter);
        self.s = "";
        if let Some(atav) = self.rdn_iter.next() {
            return Some((true, atav));
        }
        None
    }

    /// Every attribute type and value must take up at least two characters:
    /// one for the attribute type and one for the equals sign. Technically,
    /// I think the value cannot be empty, but to be on the safe side, the
    /// size hint can be trivially computed by halving the length of the string.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.s.len() >> 1))
    }

}

impl <'a> FusedIterator for RdnSequenceIterator<'a> {}

impl<'a> DoubleEndedIterator for RdnSequenceIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rdn_iter.had_error {
            return None;
        }
        if let Some(atav) = self.rdn_iter.next_back() {
            return Some((false, atav));
        }
        let s = self.s.trim_end();
        if s.is_empty() {
            return None;
        }
        let mut prev_delim = false;
        for (i, c) in s.char_indices().rev() {
            if c == self.rdn_delimiter {
                prev_delim = true;
                continue;
            }
            if prev_delim && c != self.escape {
                let rdn_str: &str = &s[i+c.len_utf8()+self.escape.len_utf8()..];
                if rdn_str.is_empty() {
                    self.s = "";
                    return Some((true, Err(())));
                }
                self.s = &s[..i+c.len_utf8()];
                self.rdn_iter = rdn_from_str_ex(rdn_str, self.escape, self.atav_delimiter);
                if let Some(atav) = self.rdn_iter.next_back() {
                    return Some((true, atav));
                }
            }
            prev_delim = false;
        }
        self.rdn_iter = rdn_from_str_ex(self.s, self.escape, self.atav_delimiter);
        self.s = "";
        if let Some(atav) = self.rdn_iter.next_back() {
            return Some((true, atav));
        }
        None
    }
}

#[inline]
pub fn dn_from_str_ex <'a> (
    s: &'a str,
    escape: char,
    rdn_delimiter: char,
    atav_delimiter: char,
) -> RdnSequenceIterator<'a> {
    RdnSequenceIterator{
        s,
        escape,
        rdn_delimiter,
        atav_delimiter,
        // This is going to get replaced later.
        rdn_iter: RdnIterator { s: "", escape, atav_delimiter, had_error: false },
    }
}

#[inline]
pub fn dn_from_str <'a> (s: &'a str) -> RdnSequenceIterator<'a> {
    dn_from_str_ex(s, '\\', ',', '+')
}

// pub fn atav_to_str <'a> (atav: AttributeTypeAndValue<'a>) -> String {

// }

// pub fn rdn_to_str <'a> (rdn: &[AttributeTypeAndValue<'a>]) -> String {

// }

// pub fn dn_to_str <'a> (rdn: &[AttributeTypeAndValue<'a>]) -> String {

// }

// TODO: getRDN

// pub fn get_rdn_from_dn <'a> (dn: &'a str, rdn_index: usize) -> Option<AttributeTypeAndValue<'a>> {
//     let mut dn_iter = dn_from_str(dn);
//     // for (i, atav) in dn_iter {
//     //     if i == rdn_index {
//     //         return Some(atav);
//     //     }
//     // }
// }



#[cfg(test)]
mod tests {
    use super::{dn_from_str, rdn_from_str};

    #[test]
    fn parse_dn_1() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=Wilbur,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, false);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "Wilbur");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");

        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_dn_2() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=#0c0657696c627572,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, false);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "#0c0657696c627572");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");

        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_dn_3() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III,2.5.4.8=#0c07466c6f72696461,2.5.4.6=\\#US";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "Jona\\\\than\\00");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, false);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "von\\ Wilbur\\, III");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "\\#US");

        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_dn_4() {
        let s = "2.5.4.3=AD\\ Collective\\ Attributes";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.3");
        assert_eq!(atav.1, "AD\\ Collective\\ Attributes");

        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_empty_dn() {
        let s = "";
        let mut dn_iter = dn_from_str(s);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_rdn_1() {
        let s = "2.5.4.3=AD\\ Collective\\ Attributes";
        let mut rdn_iter = rdn_from_str(s);

        let atav = rdn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.3");
        assert_eq!(atav.1, "AD\\ Collective\\ Attributes");

        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
    }

    #[test]
    fn parse_rdn_2() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III+2.5.4.6=\\#US";
        let mut rdn_iter = rdn_from_str(s);

        let atav = rdn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "Jona\\\\than\\00");

        let atav = rdn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "von\\ Wilbur\\, III");

        let atav = rdn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "\\#US");

        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
    }

    #[test]
    fn parse_empty_rdn() {
        let s = "";
        let mut rdn_iter = rdn_from_str(s);
        let atav = rdn_iter.next().unwrap();
        assert!(atav.is_err());
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next(), None);
    }

    #[test]
    fn parse_malformed_dn_1() {
        let s = "hello,world,no,equals";
        let mut dn_iter = dn_from_str(s);
        let (_, atav) = dn_iter.next().unwrap();
        assert!(atav.is_err());
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_malformed_dn_2() {
        let s = ",,,,";
        let mut dn_iter = dn_from_str(s);
        let (_, atav) = dn_iter.next().unwrap();
        assert!(atav.is_err());
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_malformed_dn_3() {
        let s = ",+,+,+,";
        let mut dn_iter = dn_from_str(s);
        let (_, atav) = dn_iter.next().unwrap();
        assert!(atav.is_err());
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next(), None);
    }

    #[test]
    fn parse_rdn_reverse_1() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III+2.5.4.6=\\#US";
        let mut rdn_iter = rdn_from_str(s);

        let atav = rdn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "\\#US");

        let atav = rdn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "von\\ Wilbur\\, III");

        let atav = rdn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "Jona\\\\than\\00");

        assert_eq!(rdn_iter.next_back(), None);
        assert_eq!(rdn_iter.next_back(), None);
        assert_eq!(rdn_iter.next_back(), None);
        assert_eq!(rdn_iter.next_back(), None);
    }

    #[test]
    fn parse_dn_reverse_1() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III,2.5.4.8=#0c07466c6f72696461,2.5.4.6=\\#US";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "\\#US");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "von\\ Wilbur\\, III");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, false);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "Jona\\\\than\\00");

        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
    }

    #[test]
    fn parse_dn_reverse_2() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=Wilbur,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "Wilbur");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, false);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");

        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
    }

    #[test]
    fn parse_dn_reverse_3() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=#0c0657696c627572,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "#0c0657696c627572");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, false);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");

        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next_back(), None);
    }

    // This test is broken because forwards and backwards share an RDN iterator.
    // You'll have to make one for each direction.
    // Check if the string overlaps perfectly. If so, both are in the same RDN.
    // If an ATAV is read from an overlapping RDN, both must be modified.
    //
    // UPDATE: Now the problem is that the RDN is consumed greedily by one iterator.
    #[test]
    fn parse_dn_double_ended_1() {
        let s = "2.5.4.8=#0c07466c6f72696461,2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=#0c0657696c627572,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");

        let (rdn_index, atav) = dn_iter.next().unwrap();
        let atav = atav.unwrap();
        assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");

        let (rdn_index, atav) = dn_iter.next_back().unwrap();
        let atav = atav.unwrap();
        /* This is broken, and that is accepted. It happens because forwards and backwards
        share the same RDN iterator, and the DN iterator returns a `true` to indicate
        the start of a new RDN only if a new RDN iterator is created. It is in the
        documentation that this cannot be trusted when iterating in both directions. */
        // assert_eq!(rdn_index, true);
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "#0c0657696c627572");

        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next(), None);
        assert_eq!(dn_iter.next_back(), None);
        assert_eq!(dn_iter.next(), None);
    }

    // FIXME: If the iterator ends meet at a multi-valued RDN, they could overlap.
    // TODO: Test size_hint

}
