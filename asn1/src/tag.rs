use crate::types::{Tag, TagClass, TagNumber};
use std::fmt;

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
