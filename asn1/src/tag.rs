//! ASN.1 tags
use crate::ASN1Value;
use std::fmt;
use std::sync::Arc;

/// ASN.1 tag number
///
/// Based on an analysis of thousands of ASN.1 modules, no tag number ever
/// exceeds this maximum. The largest tag number found in any ASN.1 specification
/// is 12787. This fits within 14 bits, which means that, for X.690 encodings,
/// it would be acceptable to only tolerate two bytes of long-length tag numbers.
pub type TagNumber = u16;

/// ASN.1 tag class
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord)]
pub enum TagClass {
    /// `UNIVERSAL`
    UNIVERSAL,
    /// `APPLICATION`
    APPLICATION,
    /// Context-specific
    CONTEXT,
    /// `PRIVATE`
    PRIVATE,
}

/// ASN.1 tag
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Tag {

    /// Tag class
    pub tag_class: TagClass,

    /// Tag number
    pub tag_number: TagNumber,
}

impl PartialOrd for TagClass {

    /// From ITU Recommendation X.680 (2021), Section 8.6:
    ///
    /// > The canonical order for tags is based on the outermost tag of each type and
    /// > is defined as follows:
    /// >
    /// > a) those elements or alternatives with universal class tags shall appear
    /// > first, followed by those with application class tags, followed by those with
    /// > context-specific tags, followed by those with private class tags;
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

    /// Construct a new ASN.1 tag
    #[inline]
    pub const fn new(tag_class: TagClass, tag_number: TagNumber) -> Self {
        Tag {
            tag_class,
            tag_number,
        }
    }
}

impl fmt::Display for Tag {

    /// Display a tag as it would appear in ASN.1 (e.g. `[ APPLICATION 4 ]`)
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

impl PartialOrd for Tag {

    /// From ITU Recommendation X.680 (2021), Section 8.6:
    ///
    /// > The canonical order for tags is based on the outermost tag of each type and
    /// > is defined as follows:
    /// >
    /// > a) those elements or alternatives with universal class tags shall appear
    /// > first, followed by those with application class tags, followed by those with
    /// > context-specific tags, followed by those with private class tags;
    /// >
    /// > b) within each class of tags, the elements or alternatives shall appear in
    /// > ascending order of their tag numbers.
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

// TODO: Make generic?
/// Tagged ASN.1 Value
#[derive(Debug, Clone, PartialEq)]
pub struct TaggedASN1Value {

    /// The tag
    pub tag: Tag,

    /// `true` if tagged `EXPLICIT`, or `false` if `IMPLICIT`
    pub explicit: bool,

    /// The tagged value itself
    pub value: Arc<ASN1Value>,
}

impl TaggedASN1Value {

    /// Construct a new [TaggedASN1Value]
    #[inline]
    pub const fn new(
        tag_class: TagClass,
        tag_number: TagNumber,
        explicit: bool,
        value: Arc<ASN1Value>,
    ) -> Self {
        TaggedASN1Value {
            tag: Tag::new(tag_class, tag_number),
            explicit,
            value,
        }
    }
}
