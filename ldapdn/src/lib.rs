//! https://datatracker.ietf.org/doc/html/rfc4514
//! 
use std::iter::{FusedIterator, DoubleEndedIterator};
#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct EmptyRdnError;

#[cfg(feature = "std")]
impl Display for EmptyRdnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty RDN")
    }
}

#[cfg(feature = "std")]
impl Error for EmptyRdnError {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BadAttrTypeAndValError;

#[cfg(feature = "std")]
impl Display for BadAttrTypeAndValError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Malformed attribute type and value")
    }
}

#[cfg(feature = "std")]
impl Error for BadAttrTypeAndValError {}

/// LDAP Attribute Type and Value
///
/// Note that the attribute descriptor cannot contain escape characters, so
/// we always return the slice. The value may have escapes, though.
pub type AttributeTypeAndValue<'a> = (&'a str, &'a str);

pub fn atav_from_str <'a> (s: &'a str) -> Result<AttributeTypeAndValue<'a>, BadAttrTypeAndValError> {
    let eq_pos = s.find('=');
    if eq_pos.is_none() {
        return Err(BadAttrTypeAndValError);
    }
    let eq_pos = eq_pos.unwrap();
    let (attribute_type, attribute_value) = s.split_at(eq_pos);
    if attribute_type.is_empty() {
        return Err(BadAttrTypeAndValError);
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
}

impl <'a> Iterator for RdnIterator<'a> {
    type Item = Result<AttributeTypeAndValue<'a>, BadAttrTypeAndValError>;

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
                self.s = &s[i+c.len_utf8()..];
                return Some(atav_from_str(atav_str));
            }
            prev_escape = false;
        }
        self.s = "";
        Some(atav_from_str(s))
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
                return Some(atav_from_str(atav_str));
            }
            prev_delim = false;
        }
        self.s = "";
        Some(atav_from_str(s))
    }
}

#[inline]
pub fn rdn_from_str_ex <'a> (s: &'a str, escape: char, atav_delimiter: char) -> RdnIterator<'a> {
    RdnIterator{
        s,
        escape,
        atav_delimiter,
    }
}

#[inline]
pub fn rdn_from_str <'a> (s: &'a str) -> Result<RdnIterator<'a>, EmptyRdnError> {
    if s.is_empty() {
        // Return a fake iterator that will fail.
        return Err(EmptyRdnError);
    }
    Ok(rdn_from_str_ex(s, '\\', '+'))
}

#[derive(Debug)]
pub struct RdnSequenceIterator<'a> {
    s: &'a str,
    escape: char,
    rdn_delimiter: char,
    atav_delimiter: char,
}

impl <'a> Iterator for RdnSequenceIterator<'a> {
    type Item = Result<RdnIterator<'a>, EmptyRdnError>;

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
            if c == self.rdn_delimiter && !prev_escape {
                // We've found the end of the relative distinguished name
                let rdn_str: &str = &s[..i];
                self.s = &s[i+c.len_utf8()..];
                if rdn_str.is_empty() {
                    // We have to handle this here, because the empty string
                    // is not a valid RDN, but it is indistinguishable from
                    // the end of the iterator as well.
                    return Some(Err(EmptyRdnError));
                }
                return Some(Ok(rdn_from_str_ex(rdn_str, self.escape, self.atav_delimiter)));
            }
            prev_escape = false;
        }
        if s.is_empty() {
            // We have to handle this here, because the empty string
            // is not a valid RDN, but it is indistinguishable from
            // the end of the iterator as well.
            return Some(Err(EmptyRdnError));
        }
        self.s = "";
        return Some(Ok(rdn_from_str_ex(s, self.escape, self.atav_delimiter)));
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
                self.s = &s[..i+c.len_utf8()];
                if rdn_str.is_empty() {
                    // We have to handle this here, because the empty string
                    // is not a valid RDN, but it is indistinguishable from
                    // the end of the iterator as well.
                    return Some(Err(EmptyRdnError));
                }
                return Some(Ok(rdn_from_str_ex(rdn_str, self.escape, self.atav_delimiter)));
            }
            prev_delim = false;
        }
        if s.is_empty() {
            // We have to handle this here, because the empty string
            // is not a valid RDN, but it is indistinguishable from
            // the end of the iterator as well.
            return Some(Err(EmptyRdnError));
        }
        self.s = "";
        return Some(Ok(rdn_from_str_ex(s, self.escape, self.atav_delimiter)));
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

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "Wilbur");
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");
        assert_eq!(rdn_iter.next(), None);

        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_dn_2() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=#0c0657696c627572,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "#0c0657696c627572");
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");
        assert_eq!(rdn_iter.next(), None);

        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_dn_3() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III,2.5.4.8=#0c07466c6f72696461,2.5.4.6=\\#US";
        let mut dn_iter = dn_from_str(s);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "Jona\\\\than\\00");
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "von\\ Wilbur\\, III");
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "\\#US");
        assert_eq!(rdn_iter.next(), None);

        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_dn_4() {
        let s = "2.5.4.3=AD\\ Collective\\ Attributes";
        let mut dn_iter = dn_from_str(s);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.3");
        assert_eq!(atav.1, "AD\\ Collective\\ Attributes");
        assert_eq!(rdn_iter.next(), None);

        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_empty_dn() {
        let s = "";
        let mut dn_iter = dn_from_str(s);
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_rdn_1() {
        let s = "2.5.4.3=AD\\ Collective\\ Attributes";
        let mut rdn_iter = rdn_from_str(s).unwrap();

        let atav = rdn_iter.next().unwrap().unwrap();
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
        let mut rdn_iter = rdn_from_str(s).unwrap();

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
        let rdn_iter = rdn_from_str(s);
        assert!(rdn_iter.is_err());
    }

    #[test]
    fn parse_malformed_dn_1() {
        let s = "hello,world,no,equals";
        let mut dn_iter = dn_from_str(s);
        
        for _ in 0..4 {
            let mut rdn_iter = dn_iter.next().unwrap().unwrap();
            let maybe_atav = rdn_iter.next().unwrap();
            assert!(maybe_atav.is_err());
            assert_eq!(rdn_iter.next(), None);
        }
            
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_malformed_dn_2() {
        let s = ",,,,";
        let mut dn_iter = dn_from_str(s);
        
        for _ in 0..4 {
            let maybe_rdn_iter = dn_iter.next().unwrap();
            assert!(maybe_rdn_iter.is_err());
        }

        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_malformed_dn_3() {
        let s = "+,+";
        let mut dn_iter = dn_from_str(s);

        // This malformation manifests as errors when parsing the ATAVs,
        // but that is fine.
        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let maybe_atav = rdn_iter.next().unwrap();
        assert!(maybe_atav.is_err());
        assert_eq!(rdn_iter.next(), None);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let maybe_atav = rdn_iter.next().unwrap();
        assert!(maybe_atav.is_err());
        assert_eq!(rdn_iter.next(), None);

        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_rdn_reverse_1() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III+2.5.4.6=\\#US";
        let mut rdn_iter = rdn_from_str(s).unwrap();

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

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "\\#US");

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");
        assert_eq!(rdn_iter.next_back(), None);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "von\\ Wilbur\\, III");
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "Jona\\\\than\\00");
        assert_eq!(rdn_iter.next_back(), None);

        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
    }

    #[test]
    fn parse_dn_reverse_2() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=Wilbur,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");
        assert_eq!(rdn_iter.next_back(), None);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");
        assert_eq!(rdn_iter.next_back(), None);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "Wilbur");
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");
        assert_eq!(rdn_iter.next_back(), None);

        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
    }

    #[test]
    fn parse_dn_reverse_3() {
        let s = "2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=#0c0657696c627572,2.5.4.8=#0c07466c6f72696461,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");
        assert_eq!(rdn_iter.next_back(), None);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");
        assert_eq!(rdn_iter.next_back(), None);

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "#0c0657696c627572");
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");
        assert_eq!(rdn_iter.next_back(), None);

        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next_back().is_none());
    }

    #[test]
    fn parse_dn_double_ended_1() {
        let s = "2.5.4.8=#0c07466c6f72696461,2.5.4.42=#0c084a6f6e617468616e+2.5.4.4=#0c0657696c627572,2.5.4.6=#0c025553";
        let mut dn_iter = dn_from_str(s);

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.8");
        assert_eq!(atav.1, "#0c07466c6f72696461");

        let mut rdn_iter = dn_iter.next_back().unwrap().unwrap();
        let atav = rdn_iter.next_back().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.6");
        assert_eq!(atav.1, "#0c025553");

        let mut rdn_iter = dn_iter.next().unwrap().unwrap();
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.42");
        assert_eq!(atav.1, "#0c084a6f6e617468616e");
        let atav = rdn_iter.next().unwrap().unwrap();
        assert_eq!(atav.0, "2.5.4.4");
        assert_eq!(atav.1, "#0c0657696c627572");
        assert_eq!(rdn_iter.next(), None);
        assert_eq!(rdn_iter.next_back(), None);

        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next().is_none());
        assert!(dn_iter.next_back().is_none());
        assert!(dn_iter.next().is_none());
    }

    #[test]
    fn parse_rdn_size_hint_1() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur";
        let rdn_iter = rdn_from_str(s).unwrap();
        assert_eq!(rdn_iter.size_hint(), (0, Some(s.len() >> 1)));
    }

    #[test]
    fn parse_dn_size_hint_1() {
        let s = "2.5.4.42=Jona\\\\than\\00+2.5.4.4=von\\ Wilbur\\, III+2.5.4.6=\\#US";
        let dn_iter = dn_from_str(s);
        assert_eq!(dn_iter.size_hint(), (0, Some(s.len() >> 1)));
    }

    #[test]
    fn parse_rdn_size_hint_2() {
        let s = "";
        let rdn_iter = rdn_from_str(s);
        assert!(rdn_iter.is_err());
    }

    #[test]
    fn parse_dn_size_hint_2() {
        let s = "";
        let dn_iter = dn_from_str(s);
        assert_eq!(dn_iter.size_hint(), (0, Some(0)));
    }

}

// TODO: Check for trailing escape character
