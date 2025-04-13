use crate::types::{Tag, TagClass, TagNumber};
use crate::utils::unlikely;
use std::{
    fmt,
    io::{Error, ErrorKind},
    str::FromStr,
};

/*

From ITU Recommendation X.680 (2021), Section 8.6:

> The canonical order for tags is based on the outermost tag of each type and
> is defined as follows:
>
> a) those elements or alternatives with universal class tags shall appear
> first, followed by those with application class tags, followed by those with
> context-specific tags, followed by those with private class tags;
>
*/
impl PartialOrd for TagClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_class_ord: u8 = match self {
            TagClass::UNIVERSAL => 0,
            TagClass::APPLICATION => 1,
            TagClass::CONTEXT => 2,
            TagClass::PRIVATE => 3,
        };
        let other_class_ord: u8 = match other {
            TagClass::UNIVERSAL => 0,
            TagClass::APPLICATION => 1,
            TagClass::CONTEXT => 2,
            TagClass::PRIVATE => 3,
        };
        Some(self_class_ord.cmp(&other_class_ord))
    }
}

impl Tag {
    #[inline]
    pub fn new(tag_class: TagClass, tag_number: TagNumber) -> Self {
        Tag {
            tag_class,
            tag_number,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tag_class {
            TagClass::UNIVERSAL => write!(f, "[ UNIVERSAL ")?,
            TagClass::CONTEXT => write!(f, "[ CONTEXT ")?,
            TagClass::APPLICATION => write!(f, "[ APPLICATION ")?,
            TagClass::PRIVATE => write!(f, "[ PRIVATE ")?,
        };
        if cfg!(feature = "itoa") {
            let mut buf = itoa::Buffer::new();
            f.write_str(buf.format(self.tag_number))?;
            f.write_str(" ]")
        } else {
            write!(f, "{} ]", self.tag_number)
        }
    }
}

impl From<(TagClass, TagNumber)> for Tag {
    #[inline]
    fn from(other: (TagClass, TagNumber)) -> Self {
        Tag {
            tag_class: other.0,
            tag_number: other.1,
        }
    }
}

// TODO: Test this
impl FromStr for Tag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if unlikely(!s.is_ascii()) {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        let mut index: usize = 0;
        for c in s.chars() {
            if c.is_ascii_whitespace() {
                index += 1;
                continue;
            } else if c == '[' {
                index += 1;
                continue;
            } else {
                break;
            }
        }
        let upper = s[index..].trim_start().to_ascii_uppercase();
        let tag_class: TagClass;
        if upper.starts_with("UNIV") {
            tag_class = TagClass::UNIVERSAL;
        } else if upper.contains("X") {
            tag_class = TagClass::CONTEXT;
        } else if upper.starts_with("CO") {
            tag_class = TagClass::CONTEXT;
        } else if upper.starts_with("APPL") {
            tag_class = TagClass::APPLICATION;
        } else if upper.starts_with("PRIV") {
            tag_class = TagClass::PRIVATE;
        } else {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        for c in upper.chars() {
            if !c.is_ascii_whitespace() {
                break;
            }
            index += 1;
        }
        let start_of_digit: usize = index;
        let mut end_of_digit: usize = index;
        for c in s[start_of_digit..].chars() {
            if !c.is_ascii_digit() {
                break;
            }
            end_of_digit += 1;
        }
        let tag_number = if cfg!(feature = "atoi_simd") {
            atoi_simd::parse_pos::<u16>(&s[start_of_digit..end_of_digit].as_bytes())
                .map_err(|_| Error::from(ErrorKind::InvalidInput))?
        } else {
            u16::from_str(&s[start_of_digit..end_of_digit])
                .map_err(|_| Error::from(ErrorKind::InvalidInput))?
        };
        Ok(Tag {
            tag_class,
            tag_number,
        })
    }
}

/*

From ITU Recommendation X.680 (2021), Section 8.6:

> The canonical order for tags is based on the outermost tag of each type and
> is defined as follows:
>
> a) those elements or alternatives with universal class tags shall appear
> first, followed by those with application class tags, followed by those with
> context-specific tags, followed by those with private class tags;
>
> b) within each class of tags, the elements or alternatives shall appear in
> ascending order of their tag numbers.

*/
impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.tag_class != other.tag_class {
            return Some(self.tag_class.cmp(&other.tag_class));
        }
        if self.tag_number == other.tag_number {
            return Some(std::cmp::Ordering::Equal);
        } else if self.tag_number < other.tag_number {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Greater);
        }
    }
}
