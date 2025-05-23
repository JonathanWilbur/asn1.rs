//! Encoding, decoding, and validation of constructed types such as `SET` or `SEQUENCE`
//!
//! This is not a complete solution, but this library provides `TagSelector` for
//! defining sequences of rules for matching components in a `SEQUENCE` or `SET`
//! data types, and `ComponentSpec` for more data, such as the component's name
//! and whether it is `OPTIONAL`.
use crate::types::{TagClass, TagNumber};

/// A selector for an ASN.1 tag.
/// This is recursively-defined using references instead of `Box` so that it can
/// be constructed statically / as a constant.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TagSelector<'a> {

    /// Matches a tag class and tag number exactly
    tag((TagClass, TagNumber)),

    /// Matches just the tag class, ignoring the tag number
    class(TagClass),

    /// Matches just the tag number, ignore the tag class
    number(TagNumber),

    /// Matches if any of the contained tag selectors match
    or(&'a [&'a TagSelector<'a>]),

    /// Matches if the contained tag selectors DOES NOT match
    not(&'a TagSelector<'a>),

    /// Matches any tag class and tag number
    any,
}

/// Metadata about a component of a `SET` or `SEQUENCE`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ComponentSpec<'a> {

    /// The name of the component
    pub name: &'a str,

    /// Whether the component is `OPTIONAL`. This MUST be `TRUE` when the
    /// component has a `DEFAULT` value.
    pub optional: bool,

    /// The tag selector for this component. This selector is what is used to
    /// match up an encoded component to this metadata.
    pub selector: TagSelector<'a>,

    /// If component groups are used, this is the group in which the component
    /// appears. From experience, this is rarely used.
    pub group_index: Option<u8>,

    /// `SET` or `SEQUENCE` versioning is used, this is the version in which
    /// this component is found. From experience, this is rarely used.
    pub version_number: Option<u8>,
}

impl<'a> ComponentSpec<'a> {

    /// Define a new component spec
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

    /// Define a new non-`OPTIONAL` and non-`DEFAULT`ing component spec
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

    /// Define a new `OPTIONAL` or `DEFAULT`ing component spec
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
