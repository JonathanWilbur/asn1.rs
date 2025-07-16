#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(soft_unstable)]

use crate::{X690Element, X690Value};
use wildboar_asn1::TagClass;
use wildboar_asn1::construction::{ComponentSpec, TagSelector};
use wildboar_asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use wildboar_asn1::Tag;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::utils::unlikely;
use std::iter::FusedIterator;

/// A function type for handling components during parsing.
/// 
/// Returns `true` if successfully handled; `false` if error. 
/// Parsing will not continue if `false` is returned.
pub type ComponentHandler<'a> = &'a dyn FnMut(&X690Element) -> bool;

/// A mapping of component names to their handler functions.
pub type ComponentHandlers<'a> = HashMap<&'a str, ComponentHandler<'a>>;

/// A mapping of tags to component names and their handler functions for alternative parsing.
pub type AlternativeHandlers<'a> = HashMap<Tag, (&'a str, ComponentHandler<'a>)>;

/// A function type for decoding ASN.1 elements into a specific type.
pub type Decoder<T> = fn(el: &X690Element) -> ASN1Result<T>;

/// A tuple containing recognized components (by name) and unrecognized components.
pub type IndexedComponents<'a> = (HashMap<&'a str, X690Element>, Vec<X690Element>);

/// Determines if an element matches a given tag selector.
/// 
/// # Arguments
/// * `el` - The X.690 element to check
/// * `sel` - The tag selector to match against
/// 
/// # Returns
/// `true` if the element matches the selector, `false` otherwise.
pub fn component_is_selected(el: &X690Element, sel: TagSelector) -> bool {
    match sel {
        TagSelector::tag((tc, tn)) => (el.tag.tag_class == tc) && (el.tag.tag_number == tn),
        TagSelector::any => true,
        TagSelector::class(tc) => el.tag.tag_class == tc,
        TagSelector::number(tn) => el.tag.tag_number == tn,
        TagSelector::or(sels) => sels.iter().any(|s| component_is_selected(el, **s)),
        TagSelector::not(n) => !component_is_selected(el, *n),
    }
}

/// Parse a `SET` structure according to X.690 rules.
///
/// This function parses a SET by matching elements against component specifications
/// and checking for duplicate tags. It handles required/optional components and
/// extension additions.
///
/// # Arguments
/// * `elements` - The encoded elements to parse
/// * `rctl1` - Root component type list 1
/// * `eal` - Extension additions list
/// * `rctl2` - Root component type list 2
/// * `max_elements` - Maximum number of elements allowed
///
/// # Returns
/// A tuple containing a mapping of the recognized components by name
/// to the corresponding `X690Element`, and a vector of the unrecognized
/// elements, wrapped in an `ASN1Result`.
///
/// The reason this doesn't use an iterator like `_parse_sequence` is because
/// the `SET` components have to be checked for duplicates, and they can
/// appear in any order. This involves creating a `HashMap` in the first place,
/// so this function might as well return the `HashMap` and `Vec` directly.
pub fn _parse_set<'a>(
    elements: &'a [X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
    rctl2: &'a [ComponentSpec],
    max_elements: usize,
) -> ASN1Result<IndexedComponents<'a>> {
    if elements.len() > max_elements {
        let mut err = ASN1Error::new(ASN1ErrorCode::construction_too_complex);
        err.constructed = Some(true);
        return Err(err);
    }

    /* We need to check for any duplicate tags. No, checking for duplicate
    components is not sufficient, because that only works for recognized
    components. We need to make sure the extensions do not have duplicates as
    well, even if we do not recognize them.

    This solution avoids allocation on the heap by using u64s as bitmaps to
    represent the tags that have been encountered for all classes and tag
    numbers less than 64. The HashSet is used otherwise. */
    let mut encountered_univ_tags: u64 = 0;
    let mut encountered_ctxt_tags: u64 = 0;
    let mut encountered_appl_tags: u64 = 0;
    let mut encountered_priv_tags: u64 = 0;
    let mut encountered_ext_groups: u64 = 0;
    let mut big_bois = HashSet::<Tag>::new(); // For tags with numbers greater than 63.
    for el in elements {
        let dup_tag_err = || {
            let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_tags_in_set);
            err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
            err.length = Some(el.len());
            err.constructed = Some(el.is_constructed());
            err
        };
        if unlikely(el.tag.tag_number > 63) {
            if big_bois.contains(&el.tag) {
                return Err(dup_tag_err());
            }
            big_bois.insert(el.tag);
            continue;
        }
        let bit_mask = 1 << el.tag.tag_number;
        match el.tag.tag_class {
            TagClass::UNIVERSAL => {
                if (encountered_univ_tags & bit_mask) > 0 {
                    return Err(dup_tag_err());
                }
                encountered_univ_tags |= bit_mask;
            },
            TagClass::CONTEXT => {
                if (encountered_ctxt_tags & bit_mask) > 0 {
                    return Err(dup_tag_err());
                }
                encountered_ctxt_tags |= bit_mask;
            },
            TagClass::APPLICATION => {
                if (encountered_appl_tags & bit_mask) > 0 {
                    return Err(dup_tag_err());
                }
                encountered_appl_tags |= bit_mask;
            },
            TagClass::PRIVATE => {
                if (encountered_priv_tags & bit_mask) > 0 {
                    return Err(dup_tag_err());
                }
                encountered_priv_tags |= bit_mask;
            },
        }
    }

    // Instead of iterating over every `ComponentSpec` for each component (O(n^2)),
    // we can pre-index the specs by (tag_class,tag_number)
    let mut tag_to_spec: HashMap<Tag, ComponentSpec> = HashMap::with_capacity(rctl1.len() + eal.len() + rctl2.len());
    for spec in rctl1.iter().chain(eal).chain(rctl2) {
        add_to_tag_mapping(&mut tag_to_spec, *spec, spec.selector, false);
    }

    let mut recognized_components = HashMap::with_capacity(elements.len());
    let mut unrecognized_components = Vec::new();
    for el in elements {
        match tag_to_spec.get(&Tag::new(el.tag.tag_class, el.tag.tag_number)) {
            Some(s) => {
                if recognized_components.contains_key(s.name) {
                    let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_components);
                    err.component_name = Some(String::from(s.name));
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(el.is_constructed());
                    return Err(err);
                }
                if let Some(group_index) = s.group_index {
                    if group_index < 63 {
                        encountered_ext_groups |= 1 << group_index;
                    }
                }
                recognized_components.insert(s.name, (*el).clone());
            }
            None => unrecognized_components.push((*el).clone()),
        }
    }

    for spec in rctl1.iter().chain(rctl2) {
        if spec.optional || recognized_components.contains_key(spec.name) {
            continue;
        }
        let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
        err.component_name = Some(String::from(spec.name));
        return Err(err);
    }

    if encountered_ext_groups > 0 {
        for spec in eal {
            if let Some(group_index) = spec.group_index {
                if group_index > 63 {
                    continue;
                }
                let bit_mask = 1 << group_index;
                if ((encountered_ext_groups & bit_mask) > 0)
                    && !spec.optional
                    && !recognized_components.contains_key(spec.name)
                {
                    let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
                    err.component_name = Some(String::from(spec.name));
                    return Err(err);
                }
            }
        }
    }

    Ok((recognized_components, unrecognized_components))
}

/// Parse a component type list according to X.690 rules.
///
/// This function parses components in order, handling optional components
/// and extension additions groups.
///
/// # Arguments
/// * `ctl` - Component type list to parse against
/// * `elements` - The encoded elements to parse
/// * `is_extensions` - Whether this is parsing extension additions
///
/// # Returns
/// A tuple containing the number of elements processed and the parsed components.
pub fn _parse_component_type_list<'a>(
    ctl: &'a [ComponentSpec],
    elements: &'a [&'a X690Element],
    is_extensions: bool,
) -> ASN1Result<(usize, IndexedComponents<'a>)> {
    let mut e: usize = 0;
    let mut s: usize = 0;
    let mut current_group: Option<u8> = None;
    // with_capacity(ctl.len()) because there could only be at most ctl.len() entries.
    let mut ret: IndexedComponents = (HashMap::with_capacity(ctl.len()), Vec::new());
    while s < ctl.len() {
        let spec = ctl[s];
        let el = match elements.get(e) {
            Some(x) => x,
            None => {
                if spec.optional {
                    s += 1;
                    continue;
                /*
                The only difference between parsing an extension additions list
                and a root component type list is that, if you run out of elements
                in the extension additions list, it is only invalid if an
                ExtensionAdditionsGroup had any present elements, but was missing
                REQUIRED elements.
                */
                } else if is_extensions {
                    let curr_group = current_group.unwrap_or(u8::MAX);
                    if let Some(group_index) = spec.group_index {
                        if group_index <= curr_group {
                            let mut err =
                                ASN1Error::new(ASN1ErrorCode::missing_required_components);
                            err.component_name = Some(String::from(spec.name));
                            return Err(err);
                        } else {
                            s += 1;
                            continue;
                        }
                    } else {
                        s += 1;
                        continue;
                    }
                } else {
                    let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
                    err.component_name = Some(String::from(spec.name));
                    return Err(err);
                }
            }
        };
        if component_is_selected(el, spec.selector) {
            ret.0.insert(spec.name, (**el).clone());
            if let Some(group_index) = spec.group_index {
                current_group = Some(group_index);
            }
            e += 1; // Only if it is a match do you increment the element.
        } else if !spec.optional {
            let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
            err.component_name = Some(String::from(spec.name));
            return Err(err);
        }
        s += 1;
    }
    Ok((e, ret))
}

/// An iterator over components in a component type list.
///
/// This iterator yields component names as it processes encoded elements
/// according to the component specifications.
pub struct X690ComponentIterator <'a> {
    /// The component type list to parse against
    pub ctl: &'a [ComponentSpec<'a>],
    /// The encoded elements to iterate over
    pub elements: &'a [X690Element],
    /// Whether this is parsing extension additions
    pub is_extensions: bool,
    e: usize,
    s: usize,
    current_group: Option<u8>,
}

impl <'a> X690ComponentIterator<'a> {

    /// Create a new component iterator.
    ///
    /// # Arguments
    /// * `ctl` - Component type list to parse against
    /// * `elements` - The encoded elements to iterate over
    /// * `is_extensions` - Whether this is parsing extension additions
    pub fn new (
        ctl: &'a [ComponentSpec],
        elements: &'a [X690Element],
        is_extensions: bool,
    ) -> Self {
        X690ComponentIterator {
            ctl,
            elements,
            is_extensions,
            e: 0,
            s: 0,
            current_group: None,
        }
    }

}

impl <'a> Iterator for X690ComponentIterator<'a> {
    type Item = ASN1Result<&'a str>; // str = name of component.

    fn next(&mut self) -> Option<Self::Item> {
        while self.s < self.ctl.len() {
            let spec = self.ctl[self.s];
            let el = match self.elements.get(self.e) {
                Some(x) => x,
                // We reached the end of the encoded values. Now we have to
                // check if any required values are missing.
                None => {
                    if spec.optional {
                        self.s += 1;
                        continue;
                    /*
                    The only difference between parsing an extension additions list
                    and a root component type list is that, if you run out of elements
                    in the extension additions list, it is only invalid if an
                    ExtensionAdditionsGroup had any present elements, but was missing
                    REQUIRED elements.
                    */
                    } else if self.is_extensions {
                        let curr_group = self.current_group.unwrap_or(u8::MAX);
                        if let Some(group_index) = spec.group_index {
                            if group_index <= curr_group {
                                let mut err =
                                    ASN1Error::new(ASN1ErrorCode::missing_required_components);
                                err.component_name = Some(String::from(spec.name));
                                self.s = usize::MAX; // To ensure no further iteration.
                                self.e = usize::MAX; // To ensure no further iteration.
                                return Some(Err(err));
                            } else {
                                self.s += 1;
                                continue;
                            }
                        } else {
                            self.s += 1;
                            continue;
                        }
                    } else {
                        let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
                        err.component_name = Some(String::from(spec.name));
                        self.s = usize::MAX; // To ensure no further iteration.
                        self.e = usize::MAX; // To ensure no further iteration.
                        return Some(Err(err));
                    }
                }
            };
            if component_is_selected(el, spec.selector) {
                if let Some(group_index) = spec.group_index {
                    self.current_group = Some(group_index);
                }
                self.s += 1;
                self.e += 1; // Only if it is a match do you increment the element.
                return Some(Ok(spec.name));
            } else if !spec.optional {
                let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
                err.component_name = Some(String::from(spec.name));
                self.s = usize::MAX; // To ensure no further iteration.
                self.e = usize::MAX; // To ensure no further iteration.
                return Some(Err(err));
            }
            self.s += 1; // The component was optional, but it didn't match. So we just increment and move on.
        }

        // If we are parsing extensions, we can return the rest of them as "unrecognized."
        if self.is_extensions && self.e < self.elements.len() {
            self.e += 1;
            return Some(Ok(""));
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.elements.len()))
    }

}

impl <'a> FusedIterator for X690ComponentIterator<'a> {}

/// Parse a component type list using iteration.
///
/// This is an alternative implementation that uses iteration to parse
/// component type lists. It provides the same functionality as
/// `_parse_component_type_list` but with a different approach.
///
/// # Arguments
/// * `ctl` - Component type list to parse against
/// * `elements` - The encoded elements to parse
/// * `is_extensions` - Whether this is parsing extension additions
///
/// # Returns
/// A tuple containing the number of elements processed and the parsed components.
pub fn _iter_component_type_list<'a>(
    ctl: &'a [ComponentSpec],
    elements: &'a [&'a X690Element],
    is_extensions: bool,
) -> ASN1Result<(usize, IndexedComponents<'a>)> {
    let mut e: usize = 0;
    let mut s: usize = 0;
    let mut current_group: Option<u8> = None;
    // with_capacity(ctl.len()) because there could only be at most ctl.len() entries.
    let mut ret: IndexedComponents = (HashMap::with_capacity(ctl.len()), Vec::new());
    while s < ctl.len() {
        let spec = ctl[s];
        let el = match elements.get(e) {
            Some(x) => x,
            None => {
                if spec.optional {
                    s += 1;
                    continue;
                /*
                The only difference between parsing an extension additions list
                and a root component type list is that, if you run out of elements
                in the extension additions list, it is only invalid if an
                ExtensionAdditionsGroup had any present elements, but was missing
                REQUIRED elements.
                */
                } else if is_extensions {
                    let curr_group = current_group.unwrap_or(u8::MAX);
                    if let Some(group_index) = spec.group_index {
                        if group_index <= curr_group {
                            let mut err =
                                ASN1Error::new(ASN1ErrorCode::missing_required_components);
                            err.component_name = Some(String::from(spec.name));
                            return Err(err);
                        } else {
                            s += 1;
                            continue;
                        }
                    } else {
                        s += 1;
                        continue;
                    }
                } else {
                    let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
                    err.component_name = Some(String::from(spec.name));
                    return Err(err);
                }
            }
        };
        if component_is_selected(el, spec.selector) {
            ret.0.insert(spec.name, (**el).clone());
            if let Some(group_index) = spec.group_index {
                current_group = Some(group_index);
            }
            e += 1; // Only if it is a match do you increment the element.
        } else if !spec.optional {
            let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
            err.component_name = Some(String::from(spec.name));
            return Err(err);
        }
        s += 1;
    }
    Ok((e, ret))
}

/// Get the number of possible initial components in a component type list.
///
/// This function counts the number of required components at the beginning
/// of a component type list.
///
/// # Arguments
/// * `ctl` - The component type list to analyze
///
/// # Returns
/// The number of possible initial components
fn _get_possible_initial_components(ctl: &[ComponentSpec]) -> usize {
    let mut i = 0;
    while i < ctl.len() {
        if !ctl[i].optional {
            i += 1;
            break;
        }
        i += 1;
    }
    i
}

/// Represents the current phase of structure iteration.
enum X690StructureIterationPhase {
    /// Processing root component type list 1
    RCTL1,
    /// Processing extension additions list
    EAL,
    /// Processing root component type list 2
    RCTL2,
}

/// An iterator over components in a SEQUENCE structure.
///
/// This iterator handles the complex parsing of SEQUENCE structures that
/// may contain extension additions between required and optional components.
/// The returned name is "" for unrecognized extensions.
pub struct X690StructureIterator <'a> {
    /// The encoded elements to iterate over
    pub elements: &'a [X690Element],
    /// Root component type list 1 (required components)
    pub rctl1: &'a [ComponentSpec<'a>],
    /// Extension additions list (optional extensions)
    pub eal: &'a [ComponentSpec<'a>],
    /// Root component type list 2 (optional components after extensions)
    pub rctl2: &'a [ComponentSpec<'a>],
    ctl_iterator: X690ComponentIterator<'a>,
    phase: X690StructureIterationPhase,
    i: usize, // The number of encoded values processed.
    start_of_rctl2: usize,
}

impl <'a> X690StructureIterator<'a> {

    /// Create a new structure iterator.
    ///
    /// # Arguments
    /// * `elements` - The encoded elements to iterate over
    /// * `rctl1` - Root component type list 1
    /// * `eal` - Extension additions list
    /// * `rctl2` - Root component type list 2
    pub fn new (
        elements: &'a [X690Element],
        rctl1: &'a [ComponentSpec<'a>],
        eal: &'a [ComponentSpec<'a>],
        rctl2: &'a [ComponentSpec<'a>],
    ) -> Self {
        X690StructureIterator {
            elements,
            rctl1,
            eal,
            rctl2,
            ctl_iterator: X690ComponentIterator::new(rctl1, elements, false),
            phase: X690StructureIterationPhase::RCTL1,
            i: 0,
            start_of_rctl2: 0,
        }
    }

}

impl <'a> FusedIterator for X690StructureIterator<'a> {}

impl <'a> Iterator for X690StructureIterator<'a> {
    type Item = ASN1Result<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fallible_component) = self.ctl_iterator.next() {
            self.i += 1;
            return Some(fallible_component);
        }
        match self.phase {
            X690StructureIterationPhase::RCTL1 => {
                self.phase = X690StructureIterationPhase::EAL;
                // If there is no root component type list 2, then all remaining
                // components must be extensions. Otherwise, we have to find the
                // start of RCTL2, and only iterate over i..start_of_rctl2 as
                // extensions.
                if self.rctl2.len() == 0 {
                    self.ctl_iterator = X690ComponentIterator::new(self.eal, &self.elements[self.i..], true);
                } else {
                    let end_of_possible_initial_rctl2_components = _get_possible_initial_components(self.rctl2);
                    let possible_initial_rctl2_components = &self.rctl2[0..end_of_possible_initial_rctl2_components];
                    let start_of_exts = self.i;
                    let extensions_onwards = &self.elements[start_of_exts..];
                    let mut number_of_ext_components: Option<usize> = None;
                    'outer: for el_index in 0..extensions_onwards.len() {
                        let el = &extensions_onwards[el_index];
                        for possible_rctl2_component in possible_initial_rctl2_components {
                            if component_is_selected(el, possible_rctl2_component.selector) {
                                number_of_ext_components = Some(el_index);
                                break 'outer;
                            }
                        }
                    }
                    let rctl2_found: bool = number_of_ext_components.is_some();
                    let rctl2_entirely_optional: bool = (self.rctl2.len() > 0) && self.rctl2.iter().all(|s| s.optional);
                    if !rctl2_found && !rctl2_entirely_optional {
                        let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
                        err.constructed = Some(true);
                        return Some(Err(err));
                    }

                    // NOTE: I deviated from the TypeScript implementation here. I don't see
                    // how the value `startOfExtensions` could ever be -1.
                    self.start_of_rctl2 = match number_of_ext_components {
                        Some(num) => start_of_exts + num,
                        None => self.elements.len(),
                    };
                    self.ctl_iterator = X690ComponentIterator::new(self.eal, &self.elements[self.i..self.start_of_rctl2], true);
                }
                // This isn't really a recursion risk, because it only iterates
                // three times: once for each of RCTL1, EAL, and RCTL2.
                return self.next();
            },
            X690StructureIterationPhase::EAL => {
                self.phase = X690StructureIterationPhase::RCTL2;
                debug_assert!(self.rctl2.len() == 0 || (self.i == self.start_of_rctl2),
                    "After processing all X.690-encoded extensions, we were not at the start of RCTL2");

                debug_assert!(self.rctl2.len() > 0 || (self.i == self.elements.len()),
                    "After processing all X.690-encoded extensions, we were not at the end of the encoded elements, despite no RCTL2");

                self.ctl_iterator = X690ComponentIterator::new(self.rctl2, &self.elements[self.i..], false);
                return self.next();
            },
            X690StructureIterationPhase::RCTL2 => {
                debug_assert!(self.rctl2.len() == 0 || self.i == self.elements.len(),
                    "After processing all X.690-encoded elements in RCTL2, we were not at the end");
                return None;
            },
        };
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.elements.len()))
    }

}

/// Decode a SEQUENCE OF structure.
///
/// This function decodes a SEQUENCE OF by applying the item decoder
/// to each element in the sequence.
///
/// # Arguments
/// * `el` - The SEQUENCE OF element to decode
/// * `item_decoder` - Function to decode individual items
///
/// # Returns
/// A vector of decoded items
pub fn _decode_sequence_of<T>(el: &X690Element, item_decoder: Decoder<T>) -> ASN1Result<Vec<T>> {
    let elements = el.value.components()
        .map_err(|e| {
            let mut err = ASN1Error::new(e.error_code);
            err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            err
        })?;
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements.iter() {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

/// Decode a SET OF structure.
///
/// This function decodes a SET OF by applying the item decoder
/// to each element in the set.
///
/// # Arguments
/// * `el` - The SET OF element to decode
/// * `item_decoder` - Function to decode individual items
///
/// # Returns
/// A vector of decoded items
pub fn _decode_set_of<T>(el: &X690Element, item_decoder: Decoder<T>) -> ASN1Result<Vec<T>> {
    let elements = el.value.components()
        .map_err(|e| {
            let mut err = ASN1Error::new(e.error_code);
            err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            err
        })?;
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements.iter() {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

/// Encode an element with an explicit tag.
///
/// This function wraps an element in a constructed value with the specified tag.
///
/// # Arguments
/// * `el` - The element to encode
/// * `tag` - The explicit tag to apply
///
/// # Returns
/// The encoded element with explicit tagging
pub fn _encode_explicit(el: X690Element, tag: Tag) -> X690Element {
    X690Element::new(
        tag,
        X690Value::Constructed(Arc::new(vec![ el ])),
    )
}

/// Encode an element with an implicit tag.
///
/// This function changes the tag of an element without wrapping it
/// in a constructed value.
///
/// # Arguments
/// * `el` - The element to encode
/// * `tag` - The implicit tag to apply
///
/// # Returns
/// The encoded element with implicit tagging
pub fn _encode_implicit(el: X690Element, tag: Tag) -> X690Element {
    let mut ret = el.clone();
    ret.tag = tag;
    ret
}

/// Identity function for X.690 elements.
///
/// This function works as both an encode and decode function,
/// simply returning a clone of the input element.
///
/// # Arguments
/// * `el` - The element to process
///
/// # Returns
/// A clone of the input element
pub fn x690_identity(el: &X690Element) -> ASN1Result<X690Element> {
    Ok(el.clone())
}

/// Add or remove component specifications from a tag mapping.
///
/// This helper function recursively processes tag selectors to build
/// or modify a mapping from tags to component specifications.
///
/// # Arguments
/// * `map` - The tag mapping to modify
/// * `spec` - The component specification to add/remove
/// * `selector` - The tag selector to process
/// * `remove` - Whether to remove (true) or add (false) the specification
fn add_to_tag_mapping <'a> (
    map: &mut HashMap<Tag, ComponentSpec<'a>>,
    spec: ComponentSpec<'a>,
    selector: TagSelector<'a>,
    remove: bool,
) {
    match selector {
        TagSelector::tag(tag) => {
            if remove {
                map.remove(&Tag::new(tag.0, tag.1));
            } else {
                map.insert(Tag::new(tag.0, tag.1), spec);
            }
        },
        TagSelector::or(subs) => {
            for sub in subs {
                add_to_tag_mapping(map, spec, **sub, remove);
            }
        },
        TagSelector::not(sub) => {
            add_to_tag_mapping(map, spec, *sub, !remove);
        },
        _ => (),
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    use wildboar_asn1::{
        ASN1Value, TagClass, UNIV_TAG_NULL,
        UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_SEQUENCE,
        UNIV_TAG_UTF8_STRING, OBJECT_IDENTIFIER,
    };
    use std::sync::Arc;
    use crate::parsing::X690StructureIterator;
    use crate::ber::BER;

    use super::*;

    struct AlgorithmIdentifier {
        pub algorithm: OBJECT_IDENTIFIER,
        pub parameters: Option<ASN1Value>,
    }

    const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 2] = &[
        ComponentSpec::new(
            "algorithm",
            false,
            TagSelector::tag((
                TagClass::UNIVERSAL,
                UNIV_TAG_OBJECT_IDENTIFIER,
            )),
            None,
            None,
        ),
        ComponentSpec::new("parameters", true, TagSelector::any, None, None),
        // ComponentSpec::new(
        //     "asdf",
        //     true,
        //     TagSelector::or(&[&TagSelector::any, &TagSelector::any]),
        //     None,
        //     None,
        // ),
    ];

    const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];
    const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

    fn decode_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
        let elements = el.components()?;
        let seq_iter = X690StructureIterator::new(
            &elements,
            _rctl1_components_for_AlgorithmIdentifier,
            _eal_components_for_AlgorithmIdentifier,
            _rctl2_components_for_AlgorithmIdentifier,
        );
        let mut maybe_algorithm: Option<OBJECT_IDENTIFIER> = None;
        let mut parameters: Option<ASN1Value> = None;
        for (i, component) in seq_iter.enumerate() {
            match component.unwrap() {
                "algorithm" => {
                    maybe_algorithm = Some(BER.decode_object_identifier(&elements[i])?);
                },
                "parameters" => {
                    parameters = Some(BER.decode_any(&elements[i])?);
                }
                _ => {
                    return Err(ASN1Error::new(ASN1ErrorCode::unrecognized_components_in_inextensible_type));
                }
            }
        }
        let algorithm: OBJECT_IDENTIFIER = maybe_algorithm.unwrap();
        let parameters: Option<ASN1Value> = parameters;
        Ok(AlgorithmIdentifier {
            algorithm,
            parameters,
        })
    }

    enum DirectoryString {
        UTF8String(String),
    }

    fn decode_DirectoryString(el: &X690Element) -> ASN1Result<DirectoryString> {
        match (el.tag.tag_class, el.tag.tag_number) {
            (TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING) => {
                let v = BER.decode_utf8_string(&el)?;
                return Ok(DirectoryString::UTF8String(v));
            }
            // (TagClass::UNIVERSAL, UNIV_TAG_BMP_STRING) => {
            //     let v = BER.decode_bmp_string(&el)?;
            //     return Ok(DirectoryString::BMPString(v));
            // }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }

    #[test]
    fn test_decode_algorithm_identifier() {
        let root: X690Element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
            X690Value::Constructed(Arc::new(vec![
                X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
                    X690Value::Primitive(Bytes::copy_from_slice(&[0x55, 0x04, 0x03])),
                ),
                X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NULL),
                    X690Value::Primitive(Bytes::copy_from_slice(&[])),
                ),
            ])),
        );
        let alg_id = decode_AlgorithmIdentifier(&root).unwrap();
        assert_eq!(alg_id.algorithm.to_dot_delim_string(), "2.5.4.3");
        if let Some(p) = alg_id.parameters {
            match p {
                ASN1Value::NullValue => (),
                _ => panic!(),
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_directory_string() {
        let root: X690Element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING),
            X690Value::Primitive(Bytes::from("Better Call Saul!")),
        );
        let ds = decode_DirectoryString(&root).unwrap();
        let DirectoryString::UTF8String(s) = ds;
        assert_eq!(s, String::from("Better Call Saul!"));
    }

    pub const rlrq_rctl1: [ComponentSpec; 1] = [ComponentSpec::new(
        "reason",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    )];
    pub const rlrq_rctl2: [ComponentSpec; 1] = [ComponentSpec::new(
        "user-information",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    )];
    pub const rlrq_eal: [ComponentSpec; 2] = [
        ComponentSpec::new(
            "aso-qualifier",
            true,
            TagSelector::tag((TagClass::CONTEXT, 13)),
            None,
            None,
        ),
        ComponentSpec::new(
            "asoi-identifier",
            true,
            TagSelector::tag((TagClass::CONTEXT, 14)),
            None,
            None,
        ),
    ];

    // RLRQ-apdu ::= [APPLICATION 2] IMPLICIT SEQUENCE {
    //     reason              [0] IMPLICIT Release-request-reason OPTIONAL,
    //     ...,
    //     --  Extensions for higher level association FU
    //     aso-qualifier       [13]  ASO-qualifier OPTIONAL,
    //     asoi-identifier     [14] IMPLICIT ASOI-identifier OPTIONAL,
    //     --  End of extensions for higher level association FU
    //     ...,
    //     user-information    [30] IMPLICIT Association-data OPTIONAL
    //   }

    #[test]
    fn test_decode_ACSE_RLRQ_APDU() {
        let encoded_data: Vec<u8> = vec![
            0x62, 0x0f, // RLRQ APDU
            0x80, 0x01, 0x00, // reason: [CONTEXT 0] 0 (normal)
            0x04, 0x0a, 0x8e, 0x44, 0x22, 0x8c, 0x90, 0x52, 0x6d, 0x5a, 0xd3,
            0x8a, // Some unrecognized extension.
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());

        let elements = root.components().unwrap();
        let seq_iter = X690StructureIterator::new(
            &elements,
            rlrq_rctl1.as_slice(),
            rlrq_eal.as_slice(),
            rlrq_rctl2.as_slice(),
        );
        let mut known = 0;
        let mut unknown = 0;
        for component in seq_iter {
            match component.unwrap() {
                "reason" => known += 1,
                "user-information" => known += 1,
                _ => unknown += 1,
            }
        }
        assert_eq!(known, 1);
        assert_eq!(unknown, 1);
    }

    #[test]
    #[should_panic]
    fn test_decode_ACSE_RLRQ_APDU_trailing_unrecognized_component() {
        let elements: Vec<X690Element> = vec![
            X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Primitive(Bytes::new()),
            ),
            X690Element::new(
                Tag::new(TagClass::CONTEXT, 30),
                X690Value::Constructed(Arc::new(vec![])),
            ),
            X690Element::new(
                Tag::new(TagClass::CONTEXT, 21),
                X690Value::Primitive(Bytes::copy_from_slice(&[5])),
            ),
        ];
        let seq_iter = X690StructureIterator::new(
            &elements,
            rlrq_rctl1.as_slice(),
            rlrq_eal.as_slice(),
            rlrq_rctl2.as_slice(),
        );
        for component in seq_iter {
            match component.unwrap() {
                "reason" => (),
                "user-information" => (),
                _ => panic!(),
            }
        }
    }

    const read_arg_data_rctl1: [ComponentSpec; 1] = [ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    )];
    const read_arg_data_rctl2: [ComponentSpec; 1] = [ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    )];
    const read_arg_data_eal: [ComponentSpec; 1] = [ComponentSpec::new(
        "foobar",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    )];

    // ReadArgumentData ::= SET {
    //     object               [0]  Name,
    //     selection            [1]  EntryInformationSelection DEFAULT {},
    //     modifyRightsRequest  [2]  BOOLEAN DEFAULT FALSE,
    //     ...,
    //     ...,
    //     COMPONENTS OF             CommonArguments }

    #[test]
    fn test_decode_read_argument_data_1() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x08,
            0xA0, 0x02, 0x05, 0x00, // [0] NULL
            0xA4, 0x02, 0x05, 0x00, // [4] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 2);
        let (recognized, unrecognized) = _parse_set(
            &elements,
            read_arg_data_rctl1.as_slice(),
            read_arg_data_eal.as_slice(),
            read_arg_data_rctl2.as_slice(),
            elements.len(),
        ).unwrap();
        assert!(recognized.contains_key("object"));
        assert!(recognized.contains_key("serviceControls"));
        assert_eq!(unrecognized.len(), 0);
    }

    #[test]
    fn test_decode_read_argument_data_2() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x04,
            0xA0, 0x02, 0x05, 0x00, // [0] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 1);
        let (recognized, unrecognized) = _parse_set(
            &elements,
            read_arg_data_rctl1.as_slice(),
            read_arg_data_eal.as_slice(),
            read_arg_data_rctl2.as_slice(),
            elements.len(),
        ).unwrap();
        assert!(recognized.contains_key("object"));
        assert!(!recognized.contains_key("serviceControls"));
        assert_eq!(unrecognized.len(), 0);
    }

    // In this test, the ordering of components is reversed from #1.
    #[test]
    fn test_decode_read_argument_data_3() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x08,
            0xA4, 0x02, 0x05, 0x00, // [4] NULL
            0xA0, 0x02, 0x05, 0x00, // [0] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 2);
        let (recognized, unrecognized) = _parse_set(
            &elements,
            read_arg_data_rctl1.as_slice(),
            read_arg_data_eal.as_slice(),
            read_arg_data_rctl2.as_slice(),
            elements.len(),
        ).unwrap();
        assert!(recognized.contains_key("object"));
        assert!(recognized.contains_key("serviceControls"));
        assert_eq!(unrecognized.len(), 0);
    }

    #[test]
    fn test_decode_read_argument_data_4() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x10,
            0xA0, 0x02, 0x05, 0x00, // [0] NULL
            0xA2, 0x02, 0x05, 0x00, // [2] NULL
            0xA3, 0x02, 0x05, 0x00, // [3] NULL
            0xA4, 0x02, 0x05, 0x00, // [4] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 4);
        let (recognized, unrecognized) = _parse_set(
            &elements,
            read_arg_data_rctl1.as_slice(),
            read_arg_data_eal.as_slice(),
            read_arg_data_rctl2.as_slice(),
            elements.len(),
        ).unwrap();
        assert!(recognized.contains_key("object"));
        assert!(recognized.contains_key("serviceControls"));
        assert!(recognized.contains_key("foobar"));
        assert_eq!(unrecognized.len(), 1);
    }

    #[test]
    fn test_duplicate_tags_in_set() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x08,
            0xA0, 0x02, 0x05, 0x00, // [0] NULL
            0xA0, 0x02, 0x05, 0x00, // [0] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 2);
        let parse_result = _parse_set(
            &elements,
            read_arg_data_rctl1.as_slice(),
            read_arg_data_eal.as_slice(),
            read_arg_data_rctl2.as_slice(),
            elements.len(),
        );
        assert!(parse_result.is_err_and(|e| e.error_code == ASN1ErrorCode::duplicate_tags_in_set));
    }

    const _rctl1_components_for_TBSCertificate: &[ComponentSpec; 8] = &[
        ComponentSpec::new(
            "version",
            true,
            TagSelector::tag((TagClass::CONTEXT, 0)),
            None,
            None,
        ),
        ComponentSpec::new(
            "serialNumber",
            false,
            TagSelector::tag((TagClass::UNIVERSAL, 2)),
            None,
            None,
        ),
        ComponentSpec::new(
            "signature",
            false,
            TagSelector::tag((TagClass::UNIVERSAL, 16)),
            None,
            None,
        ),
        ComponentSpec::new("issuer", false, TagSelector::any, None, None),
        ComponentSpec::new(
            "validity",
            false,
            TagSelector::tag((TagClass::UNIVERSAL, 16)),
            None,
            None,
        ),
        ComponentSpec::new("subject", false, TagSelector::any, None, None),
        ComponentSpec::new(
            "subjectPublicKeyInfo",
            false,
            TagSelector::tag((TagClass::UNIVERSAL, 16)),
            None,
            None,
        ),
        ComponentSpec::new(
            "issuerUniqueIdentifier",
            true,
            TagSelector::tag((TagClass::CONTEXT, 1)),
            None,
            None,
        ),
    ];
    
    const _rctl2_components_for_TBSCertificate: &[ComponentSpec; 0] = &[];
    
    const _eal_components_for_TBSCertificate: &[ComponentSpec; 2] = &[
        ComponentSpec::new(
            "subjectUniqueIdentifier",
            true,
            TagSelector::tag((TagClass::CONTEXT, 2)),
            Some(0),
            Some(2),
        ),
        ComponentSpec::new(
            "extensions",
            true,
            TagSelector::tag((TagClass::CONTEXT, 3)),
            Some(1),
            Some(3),
        ),
    ];

    #[test]
    fn test_decode_tbs_certificate_1() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x23,
            0xA0, 0x02, 0x05, 0x00, // [0] NULL version
            0x02, 0x01, 0x01, // serialNumber = 1
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL signature
            0x05, 0x00, // NULL issuer
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL validity
            0x05, 0x00, // NULL subject
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL subjectPublicKeyInfo
            0xA1, 0x02, 0x05, 0x00, // [1] NULL issuerUniqueIdentifier
            0xA2, 0x02, 0x05, 0x00, // [2] NULL subjectUniqueIdentifier
            0xA3, 0x02, 0x05, 0x00, // [3] NULL extensions
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 10);
        let seq_iter = X690StructureIterator::new(
            &elements,
            _rctl1_components_for_TBSCertificate.as_slice(),
            _eal_components_for_TBSCertificate.as_slice(),
            _rctl2_components_for_TBSCertificate.as_slice(),
        );
        let mut version = false;
        let mut serialNumber = false;
        let mut signature = false;
        let mut issuer = false;
        let mut validity = false;
        let mut subject = false;
        let mut subjectPublicKeyInfo = false;
        let mut issuerUniqueIdentifier = false;
        let mut subjectUniqueIdentifier = false;
        let mut extensions = false;
        for component in seq_iter {
            match component.unwrap() {
                "version" => version = true,
                "serialNumber" => serialNumber = true,
                "signature" => signature = true,
                "issuer" => issuer = true,
                "validity" => validity = true,
                "subject" => subject = true,
                "subjectPublicKeyInfo" => subjectPublicKeyInfo = true,
                "issuerUniqueIdentifier" => issuerUniqueIdentifier = true,
                "subjectUniqueIdentifier" => subjectUniqueIdentifier = true,
                "extensions" => extensions = true,
                _ => panic!(),
            }
        }
        assert!(version);
        assert!(serialNumber);
        assert!(signature);
        assert!(issuer);
        assert!(validity);
        assert!(subject);
        assert!(subjectPublicKeyInfo);
        assert!(issuerUniqueIdentifier);
        assert!(subjectUniqueIdentifier);
        assert!(extensions);
    }

    #[test]
    fn test_decode_tbs_certificate_2() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x1B,
            0xA0, 0x02, 0x05, 0x00, // [0] NULL version
            0x02, 0x01, 0x01, // serialNumber = 1
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL signature
            0x05, 0x00, // NULL issuer
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL validity
            0x05, 0x00, // NULL subject
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL subjectPublicKeyInfo
            0xA3, 0x02, 0x05, 0x00, // [3] NULL extensions
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 8);
        let seq_iter = X690StructureIterator::new(
            &elements,
            _rctl1_components_for_TBSCertificate.as_slice(),
            _eal_components_for_TBSCertificate.as_slice(),
            _rctl2_components_for_TBSCertificate.as_slice(),
        );
        let mut version = false;
        let mut serialNumber = false;
        let mut signature = false;
        let mut issuer = false;
        let mut validity = false;
        let mut subject = false;
        let mut subjectPublicKeyInfo = false;
        let mut issuerUniqueIdentifier = false;
        let mut subjectUniqueIdentifier = false;
        let mut extensions = false;
        for component in seq_iter {
            match component.unwrap() {
                "version" => version = true,
                "serialNumber" => serialNumber = true,
                "signature" => signature = true,
                "issuer" => issuer = true,
                "validity" => validity = true,
                "subject" => subject = true,
                "subjectPublicKeyInfo" => subjectPublicKeyInfo = true,
                "issuerUniqueIdentifier" => issuerUniqueIdentifier = true,
                "subjectUniqueIdentifier" => subjectUniqueIdentifier = true,
                "extensions" => extensions = true,
                _ => panic!(),
            }
        }
        assert!(version);
        assert!(serialNumber);
        assert!(signature);
        assert!(issuer);
        assert!(validity);
        assert!(subject);
        assert!(subjectPublicKeyInfo);
        assert!(!issuerUniqueIdentifier);
        assert!(!subjectUniqueIdentifier);
        assert!(extensions);
    }

    #[test]
    fn test_decode_tbs_certificate_3() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x13,
            0x02, 0x01, 0x01, // serialNumber = 1
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL signature
            0x05, 0x00, // NULL issuer
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL validity
            0x05, 0x00, // NULL subject
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL subjectPublicKeyInfo
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 6);
        let seq_iter = X690StructureIterator::new(
            &elements,
            _rctl1_components_for_TBSCertificate.as_slice(),
            _eal_components_for_TBSCertificate.as_slice(),
            _rctl2_components_for_TBSCertificate.as_slice(),
        );
        let mut version = false;
        let mut serialNumber = false;
        let mut signature = false;
        let mut issuer = false;
        let mut validity = false;
        let mut subject = false;
        let mut subjectPublicKeyInfo = false;
        let mut issuerUniqueIdentifier = false;
        let mut subjectUniqueIdentifier = false;
        let mut extensions = false;
        for component in seq_iter {
            match component.unwrap() {
                "version" => version = true,
                "serialNumber" => serialNumber = true,
                "signature" => signature = true,
                "issuer" => issuer = true,
                "validity" => validity = true,
                "subject" => subject = true,
                "subjectPublicKeyInfo" => subjectPublicKeyInfo = true,
                "issuerUniqueIdentifier" => issuerUniqueIdentifier = true,
                "subjectUniqueIdentifier" => subjectUniqueIdentifier = true,
                "extensions" => extensions = true,
                _ => panic!(),
            }
        }
        assert!(!version);
        assert!(serialNumber);
        assert!(signature);
        assert!(issuer);
        assert!(validity);
        assert!(subject);
        assert!(subjectPublicKeyInfo);
        assert!(!issuerUniqueIdentifier);
        assert!(!subjectUniqueIdentifier);
        assert!(!extensions);
    }

    #[test]
    fn test_decode_tbs_certificate_4() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x17,
            0x02, 0x01, 0x01, // serialNumber = 1
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL signature
            0x05, 0x00, // NULL issuer
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL validity
            0x05, 0x00, // NULL subject
            0x30, 0x02, 0x05, 0x00, // SEQUENCE NULL subjectPublicKeyInfo
            0xA4, 0x02, 0x05, 0x00, // [4] NULL unrecognized component
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 7);
        let seq_iter = X690StructureIterator::new(
            &elements,
            _rctl1_components_for_TBSCertificate.as_slice(),
            _eal_components_for_TBSCertificate.as_slice(),
            _rctl2_components_for_TBSCertificate.as_slice(),
        );
        let mut version = false;
        let mut serialNumber = false;
        let mut signature = false;
        let mut issuer = false;
        let mut validity = false;
        let mut subject = false;
        let mut subjectPublicKeyInfo = false;
        let mut issuerUniqueIdentifier = false;
        let mut subjectUniqueIdentifier = false;
        let mut extensions = false;
        let mut unrecognized = false;
        for component in seq_iter {
            match component.unwrap() {
                "version" => version = true,
                "serialNumber" => serialNumber = true,
                "signature" => signature = true,
                "issuer" => issuer = true,
                "validity" => validity = true,
                "subject" => subject = true,
                "subjectPublicKeyInfo" => subjectPublicKeyInfo = true,
                "issuerUniqueIdentifier" => issuerUniqueIdentifier = true,
                "subjectUniqueIdentifier" => subjectUniqueIdentifier = true,
                "extensions" => extensions = true,
                "" => unrecognized = true,
                _ => panic!(),
            }
        }
        assert!(!version);
        assert!(serialNumber);
        assert!(signature);
        assert!(issuer);
        assert!(validity);
        assert!(subject);
        assert!(subjectPublicKeyInfo);
        assert!(!issuerUniqueIdentifier);
        assert!(!subjectUniqueIdentifier);
        assert!(!extensions);
        assert!(unrecognized);
    }


    // PartialOutcomeQualifier ::= SET {
    //     limitProblem                  [0]  LimitProblem OPTIONAL,
    //     unexplored                    [1]  SET SIZE (1..MAX) OF ContinuationReference OPTIONAL,
    //     unavailableCriticalExtensions [2]  BOOLEAN DEFAULT FALSE,
    //     unknownErrors                 [3]  SET SIZE (1..MAX) OF ABSTRACT-SYNTAX.&Type OPTIONAL,
    //     queryReference                [4]  OCTET STRING OPTIONAL,
    //     overspecFilter                [5]  Filter OPTIONAL,
    //     notification                  [6]  SEQUENCE SIZE (1..MAX) OF
    //                                          Attribute{{SupportedAttributes}} OPTIONAL,
    //     entryCount                         CHOICE {
    //       bestEstimate                  [7]  INTEGER,
    //       lowEstimate                   [8]  INTEGER,
    //       exact                         [9]  INTEGER,
    //       ...} OPTIONAL
    //     --                            [10] Not to be used -- }

    const _rctl1_components_for_PartialOutcomeQualifier: &[ComponentSpec; 2] = &[
        ComponentSpec::new(
            "limitProblem",
            true,
            TagSelector::tag((TagClass::CONTEXT, 0)),
            None,
            None,
        ),
        ComponentSpec::new(
            "entryCount",
            true,
            TagSelector::or(&[
                &TagSelector::tag((TagClass::CONTEXT, 7)),
                &TagSelector::tag((TagClass::CONTEXT, 8)),
                &TagSelector::tag((TagClass::CONTEXT, 9)),
            ]),
            None,
            None,
        ),
    ];
    
    const _rctl2_components_for_PartialOutcomeQualifier: &[ComponentSpec; 0] = &[];
    
    const _eal_components_for_PartialOutcomeQualifier: &[ComponentSpec; 0] = &[];

    #[test]
    fn test_duplicate_components_in_set() {
        let encoded_data: Vec<u8> = vec![
            0x31, 0x08,
            0xA7, 0x02, 0x05, 0x00, // [7] NULL
            0xA9, 0x02, 0x05, 0x00, // [9] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 2);
        let parse_result = _parse_set(
            &elements,
            _rctl1_components_for_PartialOutcomeQualifier.as_slice(),
            _eal_components_for_PartialOutcomeQualifier.as_slice(),
            _rctl2_components_for_PartialOutcomeQualifier.as_slice(),
            elements.len(),
        );
        assert!(parse_result.is_err_and(|e| e.error_code == ASN1ErrorCode::duplicate_components));
    }

    #[test]
    fn test_fused_iterator_structure() {
        let encoded_data: Vec<u8> = vec![
            0x30, 0x05,
            0x06, 0x03, 0x55, 0x04, 0x03 // OBJECT IDENTIFIER 2.5.4.3
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 1);
        let mut seq_iter = X690StructureIterator::new(
            &elements,
            _rctl1_components_for_AlgorithmIdentifier,
            _eal_components_for_AlgorithmIdentifier,
            _rctl2_components_for_AlgorithmIdentifier,
        );
        let it1 = seq_iter.next();
        let it2 = seq_iter.next();
        let it3 = seq_iter.next();
        let it4 = seq_iter.next();
        let it5 = seq_iter.next();
        let it6 = seq_iter.next();
        let it7 = seq_iter.next();
        assert!(it1.is_some());
        assert!(it2.is_none());
        assert!(it3.is_none());
        assert!(it4.is_none());
        assert!(it5.is_none());
        assert!(it6.is_none());
        assert!(it7.is_none());
    }

    #[test]
    fn test_fused_iterator_components() {
        let encoded_data: Vec<u8> = vec![
            0x30, 0x05,
            0x06, 0x03, 0x55, 0x04, 0x03 // OBJECT IDENTIFIER 2.5.4.3
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 1);
        let mut seq_iter = X690ComponentIterator::new(
            _rctl1_components_for_AlgorithmIdentifier,
            &elements,
            false,
        );
        let it1 = seq_iter.next();
        let it2 = seq_iter.next();
        let it3 = seq_iter.next();
        let it4 = seq_iter.next();
        let it5 = seq_iter.next();
        let it6 = seq_iter.next();
        let it7 = seq_iter.next();
        assert!(it1.is_some());
        assert!(it2.is_none());
        assert!(it3.is_none());
        assert!(it4.is_none());
        assert!(it5.is_none());
        assert!(it6.is_none());
        assert!(it7.is_none());
    }


    const read_arg_data_rctl1_optional: [ComponentSpec; 1] = [ComponentSpec::new(
        "object",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    )];

    #[test]
    fn test_empty_set() {
        let encoded_data: Vec<u8> = vec![0x31, 0x00];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 0);
        let (recognized, unrecognized) = _parse_set(
            &elements,
            read_arg_data_rctl1_optional.as_slice(),
            read_arg_data_eal.as_slice(),
            read_arg_data_rctl2.as_slice(),
            elements.len(),
        ).unwrap();
        assert_eq!(recognized.len(), 0);
        assert_eq!(unrecognized.len(), 0);
    }

    #[test]
    fn test_empty_seq() {
        let encoded_data: Vec<u8> = vec![0x30, 0x00];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());
        let elements = root.components().unwrap();
        assert_eq!(elements.len(), 0);
        let seq_iter = X690StructureIterator::new(
            &elements,
            rlrq_rctl1.as_slice(),
            rlrq_eal.as_slice(),
            rlrq_rctl2.as_slice(),
        );
        let results = seq_iter.collect::<Vec<_>>();
        assert_eq!(results.len(), 0);
    }

    // Just one more to be sure...
    #[test]
    fn test_decode_ACSE_RLRQ_APDU_2() {
        let encoded_data: Vec<u8> = vec![
            0x62, 0x0B, // RLRQ APDU
            0x80, 0x01, 0x00, // reason: [CONTEXT 0] 0 (normal)
            0xA0 | 13, 0x02, 0x05, 0x00, // aso-qualifier: [13] NULL
            // Skipping asoi-identifier 
            0xA0 | 30, 0x02, 0x05, 0x00, // user-information: [30] NULL
        ];
        let (bytes_read, root) = BER.decode_from_slice(encoded_data.as_slice()).unwrap();
        assert_eq!(bytes_read, encoded_data.len());

        let elements = root.components().unwrap();
        let seq_iter = X690StructureIterator::new(
            &elements,
            rlrq_rctl1.as_slice(),
            rlrq_eal.as_slice(),
            rlrq_rctl2.as_slice(),
        );
        // let mut known = 0;
        // let mut unknown = 0;
        let mut reason: bool = false;
        let mut user_info: bool = false;
        let mut aso_qualifier: bool = false;
        let mut aso_identifier: bool = false;
        for component in seq_iter {
            match component.unwrap() {
                "reason" => reason = true,
                "aso-qualifier" => aso_qualifier = true,
                "aso-identifier" => aso_identifier = true,
                "user-information" => user_info = true,
                _ => (),
            }
        }
        assert!(reason);
        assert!(aso_qualifier);
        assert!(!aso_identifier);
        assert!(user_info);
    }

}
