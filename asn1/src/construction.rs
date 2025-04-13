use crate::types::{TagClass, TagNumber};

// This is recursively-defined using references instead of `Box` so that it can
// be constructed statically / as a constant.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TagSelector<'a> {
    tag((TagClass, TagNumber)),
    class(TagClass),
    number(TagNumber),
    and(&'a [&'a TagSelector<'a>]),
    or(&'a [&'a TagSelector<'a>]),
    not(&'a TagSelector<'a>),
    any,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ComponentSpec<'a> {
    pub name: &'a str,
    pub optional: bool,
    pub selector: TagSelector<'a>,
    pub group_index: Option<u8>,
    pub version_number: Option<u8>,
}

impl<'a> ComponentSpec<'a> {

    #[inline]
    pub const fn new(
        name: &'a str,
        optional: bool,
        selector: TagSelector<'a>,
        group_index: Option<u8>,
        version_number: Option<u8>,
    ) -> Self {
        ComponentSpec::<'a> {
            name,
            optional,
            selector,
            group_index,
            version_number,
        }
    }

    #[inline]
    pub const fn req(
        name: &'a str,
        selector: TagSelector<'a>,
    ) -> Self {
        ComponentSpec::<'a> {
            name,
            optional: false,
            selector,
            group_index: None,
            version_number: None,
        }
    }

    #[inline]
    pub const fn opt(
        name: &'a str,
        selector: TagSelector<'a>,
    ) -> Self {
        ComponentSpec::<'a> {
            name,
            optional: true,
            selector,
            group_index: None,
            version_number: None,
        }
    }
}

// REVIEW: Does this make sense at all?
impl <'a> Default for ComponentSpec<'a> {

    #[inline]
    fn default() -> Self {
        ComponentSpec::<'a> {
            name: "",
            optional: true,
            selector: TagSelector::any,
            group_index: None,
            version_number: None,
        }
    }

}
