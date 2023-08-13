#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(soft_unstable)]

use crate::{X690Element, X690Value};
use asn1::TagClass;
use asn1::construction::{ComponentSpec, TagSelector};
use asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use asn1::types::Tag;
use std::collections::HashMap;
use std::sync::Arc;

// Return `true` if successfully handled; `false` if error. Parsing will not continue if `false` returned.
pub type ComponentHandler<'a> = &'a dyn FnMut(&X690Element) -> bool;

pub type ComponentHandlers<'a> = HashMap<&'a str, ComponentHandler<'a>>;

pub type AlternativeHandlers<'a> = HashMap<Tag, (&'a str, ComponentHandler<'a>)>;

pub type Decoder<T> = fn(el: &X690Element) -> ASN1Result<T>;

pub type IndexedComponents<'a> = (HashMap<&'a str, X690Element>, Vec<X690Element>);

pub fn component_is_selected(el: &X690Element, sel: TagSelector) -> bool {
    match sel {
        TagSelector::tag((tc, tn)) => (el.tag.tag_class == tc) && (el.tag.tag_number == tn),
        TagSelector::any => true,
        TagSelector::class(tc) => el.tag.tag_class == tc,
        TagSelector::number(tn) => el.tag.tag_number == tn,
        TagSelector::and(sels) => {
            for s in sels {
                if !component_is_selected(el, **s) {
                    return false;
                }
            }
            return true;
        }
        TagSelector::or(sels) => {
            for s in sels {
                if component_is_selected(el, **s) {
                    return true;
                }
            }
            return false;
        }
        TagSelector::not(n) => !component_is_selected(el, *n),
    }
}

// TODO: Avoid using HashMaps and Sets if the number of components is small.
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
    well, even if we do not recognized them.

    Instead of using a `HashSet` (as was the case before), this solution avoids
    allocation on the heap entirely by using u64s as bitmaps. Unfortunately,
    this does mean that tags with numbers greater than 63 are not checked for
    duplicates, but this is an unusual case, and the considerable performance
    gains are worth being a little more reckless on this front. */
    let mut encountered_univ_tags: u64 = 0;
    let mut encountered_ctxt_tags: u64 = 0;
    let mut encountered_appl_tags: u64 = 0;
    let mut encountered_priv_tags: u64 = 0;
    let mut encountered_ext_groups: u64 = 0;
    /* Unfortunately, the above results in this copy-paste mess, but it is still
    worth it. */
    for el in elements {
        match el.tag.tag_class {
            TagClass::UNIVERSAL => {
                if el.tag.tag_number > 63 {
                    continue;
                }
                let bit_mask = 1 << el.tag.tag_number;
                if (encountered_univ_tags & bit_mask) > 0 {
                    let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_tags_in_set);
                    // err.component_name = el.name.clone(); // FIXME:
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(el.is_constructed());
                    return Err(err);
                }
                encountered_univ_tags |= bit_mask;
            },
            TagClass::CONTEXT => {
                if el.tag.tag_number > 63 {
                    continue;
                }
                let bit_mask = 1 << el.tag.tag_number;
                if (encountered_ctxt_tags & bit_mask) > 0 {
                    let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_tags_in_set);
                    // err.component_name = el.name.clone(); // FIXME:
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(el.is_constructed());
                    return Err(err);
                }
                encountered_ctxt_tags |= bit_mask;
            },
            TagClass::APPLICATION => {
                if el.tag.tag_number > 63 {
                    continue;
                }
                let bit_mask = 1 << el.tag.tag_number;
                if (encountered_appl_tags & bit_mask) > 0 {
                    let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_tags_in_set);
                    // err.component_name = el.name.clone(); // FIXME:
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(el.is_constructed());
                    return Err(err);
                }
                encountered_appl_tags |= bit_mask;
            },
            TagClass::PRIVATE => {
                if el.tag.tag_number > 63 {
                    continue;
                }
                let bit_mask = 1 << el.tag.tag_number;
                if (encountered_priv_tags & bit_mask) > 0 {
                    let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_tags_in_set);
                    // err.component_name = el.name.clone(); // FIXME:
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(el.is_constructed());
                    return Err(err);
                }
                encountered_priv_tags |= bit_mask;
            },
        }
    }

    // Instead of iterating over every `ComponentSpec` for each component (O(n^2)),
    // we can pre-index the specs by (tag_class,tag_number)
    let mut tag_to_spec: HashMap<Tag, ComponentSpec> = HashMap::with_capacity(rctl1.len() + eal.len() + rctl2.len());
    for spec in rctl1.iter().chain(eal).into_iter().chain(rctl2).into_iter() {
        match spec.selector {
            TagSelector::tag(tag) => {
                tag_to_spec.insert(Tag::new(tag.0, tag.1), *spec);
                ()
            }
            _ => (),
        }
    }

    let mut ret: IndexedComponents = (HashMap::with_capacity(elements.len()), Vec::new());
    for el in elements {
        match tag_to_spec.get(&Tag::new(el.tag.tag_class, el.tag.tag_number)) {
            Some(s) => {
                if ret.0.contains_key(s.name) {
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
                ret.0.insert(s.name, (*el).clone());
            }
            None => ret.1.push((*el).clone()),
        }
    }

    for spec in rctl1.iter().chain(rctl2).into_iter() {
        if spec.optional || ret.0.contains_key(spec.name) {
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
                    && !ret.0.contains_key(spec.name)
                {
                    let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
                    err.component_name = Some(String::from(spec.name));
                    return Err(err);
                }
            }
        }
    }

    Ok(ret)
}

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


pub struct X690ComponentIterator <'a> {
    pub ctl: &'a [ComponentSpec<'a>],
    pub elements: &'a [X690Element],
    pub is_extensions: bool,
    e: usize,
    s: usize,
    current_group: Option<u8>,
}

impl <'a> X690ComponentIterator<'a> {

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

}

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

pub fn _parse_sequence_with_trailing_rctl<'a>(
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
    rctl2: &'a [ComponentSpec],
) -> ASN1Result<IndexedComponents<'a>> {
    let (start_of_exts, rctl1_index) = _parse_component_type_list(rctl1, elements, false)?;
    let end_of_possible_rctl2_components = _get_possible_initial_components(rctl2);
    let possible_initial_rctl2_components = &rctl2[0..end_of_possible_rctl2_components];
    let extensions_onwards = &elements[start_of_exts..];
    let mut number_of_ext_components: Option<usize> = None;
    for el_index in 0..extensions_onwards.len() {
        let el = &extensions_onwards[el_index];
        for possible_rctl2_component in possible_initial_rctl2_components {
            if component_is_selected(el, possible_rctl2_component.selector) {
                number_of_ext_components = Some(el_index);
                break;
            }
        }
        // TODO: Use named-label breaks: https://stackoverflow.com/questions/22905752/named-breaks-in-for-loops-in-rust
        if number_of_ext_components.is_some() {
            break;
        }
    }
    let rctl2_found: bool = number_of_ext_components.is_some();
    let rctl2_entirely_optional: bool = rctl2.iter().all(|s| s.optional);
    if !rctl2_found && !rctl2_entirely_optional {
        let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
        err.constructed = Some(true);
        return Err(err);
    }
    // NOTE: I deviated from the TypeScript implementation here. I don't see
    // how the value `startOfExtensions` could ever be -1.
    // FIXME: I think the below needs to default to the end of elements.
    let start_of_rctl2 = match number_of_ext_components {
        Some(num) => start_of_exts + num,
        None => elements.len(),
    };
    let (eal_components_read, eal_index) =
        _parse_component_type_list(eal, &elements[start_of_exts..start_of_rctl2], true)?;
    let (rctl2_components_read, rctl2_index) =
        _parse_component_type_list(rctl2, &elements[start_of_rctl2..], false)?;
    if start_of_rctl2 + rctl2_components_read > elements.len() {
        let mut err = ASN1Error::new(ASN1ErrorCode::nonsense);
        err.constructed = Some(true);
        return Err(err);
    }
    if start_of_rctl2 + rctl2_components_read != elements.len() {
        let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
        err.constructed = Some(true);
        return Err(err);
    }
    let end_of_recognized_exts = start_of_exts + eal_components_read;
    let mut ret: IndexedComponents = rctl1_index;
    ret.0.extend(eal_index.0);
    ret.1.extend(eal_index.1);
    for ext in &elements[end_of_recognized_exts..start_of_rctl2] {
        ret.1.push((*ext).clone());
    }
    ret.0.extend(rctl2_index.0);
    ret.1.extend(rctl2_index.1);
    Ok(ret)
}

pub fn _parse_sequence_without_trailing_rctl<'a>(
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
) -> ASN1Result<IndexedComponents<'a>> {
    let (start_of_exts, rctl_index) = _parse_component_type_list(rctl1, &elements, false)?;
    let (exts_read, eal_index) = _parse_component_type_list(eal, &elements[start_of_exts..], true)?;
    let end_of_recognized_exts = start_of_exts + exts_read;
    let mut ret: IndexedComponents = rctl_index;
    ret.0.extend(eal_index.0);
    for el in elements[end_of_recognized_exts..].into_iter() {
        ret.1.push((**el).clone());
    }
    Ok(ret)
}

pub fn _parse_sequence<'a>(
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
    rctl2: &'a [ComponentSpec],
) -> ASN1Result<IndexedComponents<'a>> {
    if rctl2.len() > 0 {
        return _parse_sequence_with_trailing_rctl(elements, rctl1, eal, rctl2);
    } else {
        return _parse_sequence_without_trailing_rctl(elements, rctl1, eal);
    }
}

enum X690StructureIterationPhase {
    RCTL1,
    EAL,
    RCTL2,
}

pub struct X690StructureIterator <'a> {
    pub elements: &'a [X690Element],
    pub rctl1: &'a [ComponentSpec<'a>],
    pub eal: &'a [ComponentSpec<'a>],
    pub rctl2: &'a [ComponentSpec<'a>],
    ctl_iterator: X690ComponentIterator<'a>,
    phase: X690StructureIterationPhase,
    i: usize, // The number of encoded values processed.
    start_of_rctl2: usize,
}

impl <'a> X690StructureIterator<'a> {

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

impl <'a> Iterator for X690StructureIterator<'a> {
    type Item = ASN1Result<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fallible_component) = self.ctl_iterator.next() {
            self.i += 1;
            return match fallible_component {
                Ok(component) => Some(Ok(component)),
                Err(e) => Some(Err(e)),
            };
        }
        match self.phase {
            X690StructureIterationPhase::RCTL1 => {
                self.phase = X690StructureIterationPhase::EAL;
                // If there is no root component type list 2, then all remaining
                // components must be extensions. Otherwise, we have to find the
                // start of RCTL2, and only iterate over i..start_of_rctl2 as
                // extensions.
                if self.rctl2.len() == 0 {
                    self.ctl_iterator = X690ComponentIterator::new(self.eal, &self.elements[self.i..], true).into_iter();
                } else {
                    let end_of_possible_initial_rctl2_components = _get_possible_initial_components(self.rctl2);
                    let possible_initial_rctl2_components = &self.rctl2[0..end_of_possible_initial_rctl2_components];
                    let start_of_exts = self.i;
                    let extensions_onwards = &self.elements[start_of_exts..];
                    let mut number_of_ext_components: Option<usize> = None;
                    for el_index in 0..extensions_onwards.len() {
                        let el = &extensions_onwards[el_index];
                        for possible_rctl2_component in possible_initial_rctl2_components {
                            if component_is_selected(el, possible_rctl2_component.selector) {
                                number_of_ext_components = Some(el_index);
                                break;
                            }
                        }
                        // TODO: Use named-label breaks: https://stackoverflow.com/questions/22905752/named-breaks-in-for-loops-in-rust
                        if number_of_ext_components.is_some() {
                            break;
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
                    // FIXME: I think the below needs to default to the end of elements.
                    self.start_of_rctl2 = match number_of_ext_components {
                        Some(num) => start_of_exts + num,
                        None => self.elements.len(),
                    };
                    self.ctl_iterator = X690ComponentIterator::new(self.eal, &self.elements[self.i..self.start_of_rctl2], true).into_iter();
                }
                return self.next();
            },
            X690StructureIterationPhase::EAL => {
                self.phase = X690StructureIterationPhase::RCTL2;
                debug_assert!(self.rctl2.len() == 0 || (self.i != self.start_of_rctl2),
                    "After processing all X.690-encoded extensions, we were not at the start of RCTL2");

                debug_assert!(self.rctl2.len() > 0 || (self.i == self.elements.len()),
                    "After processing all X.690-encoded extensions, we were not at the end of the encoded elements, despite no RCTL2");

                self.ctl_iterator = X690ComponentIterator::new(self.rctl2, &self.elements[self.i..], false).into_iter();
                return self.next();
            },
            X690StructureIterationPhase::RCTL2 => {
                debug_assert!(self.rctl2.len() == 0 || (self.i != self.start_of_rctl2),
                    "After processing all X.690-encoded elements in RCTL2, we were not at the end");
                return None;
            },
        };
    }

}

// pub fn _decode_choice (
//     el: &mut X690Element,
//     handlers: AlternativeHandlers,
//     unrecognized_handler: Option<ComponentHandler>,
// ) -> ASN1Result<()> {
//     let tag = Tag::new(el.tag.tag_class, el.tag.tag_number);
//     match handlers.get(&tag) {
//         Some((name, handler)) => {
//             el.name = Some(String::from(*name));
//             if !handler(&el) {
//                 return Err(Error::from(ErrorKind::InvalidInput));
//             }
//         },
//         None => {
//             if let Some(handler) = unrecognized_handler {
//                 if !handler(&el) {
//                     return Err(Error::from(ErrorKind::InvalidInput));
//                 }
//             } else {
//                 return Err(Error::from(ErrorKind::InvalidInput));

//             }
//         }
//     }
//     Ok(())
// }

pub fn _decode_sequence_of<T>(el: &X690Element, item_decoder: Decoder<T>) -> ASN1Result<Vec<T>> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            let mut err = ASN1Error::new(ASN1ErrorCode::nonsense);
            // err.component_name = el.name.clone(); // FIXME:
            err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            return Err(err);
        }
    };
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements.iter() {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

pub fn _decode_set_of<T>(el: &X690Element, item_decoder: Decoder<T>) -> ASN1Result<Vec<T>> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            let mut err = ASN1Error::new(ASN1ErrorCode::nonsense);
            // err.component_name = el.name.clone(); // FIXME:
            err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            return Err(err);
        }
    };
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements.iter() {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

pub fn _encode_explicit(el: X690Element, tag: Tag) -> X690Element {
    X690Element::new(
        tag,
        X690Value::Constructed(Arc::new(vec![ el ])),
    )
}

pub fn _encode_implicit(el: X690Element, tag: Tag) -> X690Element {
    let mut ret = el.clone();
    ret.tag = tag;
    ret
}

// This works as an encode and decode function.
pub fn x690_identity(el: &X690Element) -> ASN1Result<X690Element> {
    Ok(el.clone())
}

// #[cfg(test)]
// mod tests {

//     use crate::*;
//     use asn1::types::{
//         ASN1Value, TagClass, ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING, ASN1_UNIVERSAL_TAG_NUMBER_NULL,
//         ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
//         ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING, OBJECT_IDENTIFIER,
//     };
//     use std::sync::Arc;

//     use crate::ber::{
//         ber_decode_any, ber_decode_bmp_string, ber_decode_object_identifier, ber_decode_utf8_string,
//     };

//     use super::*;

//     struct AlgorithmIdentifier {
//         pub algorithm: OBJECT_IDENTIFIER,
//         pub parameters: Option<ASN1Value>,
//     }

//     const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 3] = &[
//         ComponentSpec::new(
//             "algorithm",
//             false,
//             TagSelector::tag((
//                 TagClass::UNIVERSAL,
//                 ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
//             )),
//             None,
//             None,
//         ),
//         ComponentSpec::new("parameters", true, TagSelector::any, None, None),
//         ComponentSpec::new(
//             "asdf",
//             true,
//             TagSelector::or(&[&TagSelector::any, &TagSelector::any]),
//             None,
//             None,
//         ),
//     ];

//     const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];
//     const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

//     fn decode_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
//         let elements = match &el.value {
//             X690Value::Constructed(children) => children,
//             _ => panic!(),
//         };
//         let el_refs = elements.iter().collect::<Vec<&X690Element>>();
//         let (components, _) = _parse_sequence(
//             el_refs.as_slice(),
//             _rctl1_components_for_AlgorithmIdentifier,
//             _eal_components_for_AlgorithmIdentifier,
//             _rctl2_components_for_AlgorithmIdentifier,
//         )?;
//         // NOTE: unwrap() should be fine, because we validate that there is such a component in `_parse_sequence`.
//         let algorithm: OBJECT_IDENTIFIER =
//             ber_decode_object_identifier(components.get("algorithm").unwrap())?;
//         let parameters: Option<ASN1Value> = match components.get("parameters") {
//             Some(c) => Some(ber_decode_any(c)?),
//             _ => None,
//         };
//         Ok(AlgorithmIdentifier {
//             algorithm,
//             parameters,
//         })
//     }

//     enum DirectoryString {
//         UTF8String(String),
//         BMPString(String),
//     }

//     fn decode_DirectoryString(el: &X690Element) -> ASN1Result<DirectoryString> {
//         match (el.tag.tag_class, el.tag.tag_number) {
//             (TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING) => {
//                 let v = ber_decode_utf8_string(&el)?;
//                 return Ok(DirectoryString::UTF8String(v));
//             }
//             (TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING) => {
//                 let v = ber_decode_bmp_string(&el)?;
//                 return Ok(DirectoryString::BMPString(v));
//             }
//             _ => {
//                 return Err(ASN1Error::new(
//                     ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
//                 ))
//             }
//         }
//     }

//     #[test]
//     fn test_decode_sequence() {
//         let rctl1: Vec<ComponentSpec> = vec![
//             ComponentSpec::new(
//                 "algorithm",
//                 false,
//                 TagSelector::tag((
//                     TagClass::UNIVERSAL,
//                     ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
//                 )),
//                 None,
//                 None,
//             ),
//             ComponentSpec::new("parameters", true, TagSelector::any, None, None),
//         ];
//         let eal: Vec<ComponentSpec> = vec![];
//         let rctl2: Vec<ComponentSpec> = vec![];

//         let root: X690Element = X690Element::new(
//             Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
//             X690Value::Constructed(Arc::new(vec![
//                 X690Element::new(
//                     Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER),
//                     X690Value::Primitive(Bytes::copy_from_slice(&[0x55, 0x04, 0x03])),
//                 ),
//                 X690Element::new(
//                     Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_NULL),
//                     X690Value::Primitive(Bytes::copy_from_slice(&[])),
//                 ),
//             ])),
//         );
//         let elements = match &root.value {
//             X690Value::Constructed(children) => children,
//             _ => panic!(),
//         };
//         let el_refs = elements.iter().collect::<Vec<&X690Element>>();
//         let (components, unrecognized) = _parse_sequence(
//             el_refs.as_slice(),
//             rctl1.as_slice(),
//             eal.as_slice(),
//             rctl2.as_slice(),
//         )
//         .unwrap();
//         let algorithm: OBJECT_IDENTIFIER =
//             ber_decode_object_identifier(components.get("algorithm").unwrap()).unwrap();
//         let parameters: Option<ASN1Value> = components
//             .get("parameters")
//             .and_then(|c| Some(ber_decode_any(c).unwrap()));
//         assert_eq!(unrecognized.len(), 0);
//         assert_eq!(
//             algorithm
//                 .0
//                 .iter()
//                 .map(|a| a.to_string())
//                 .collect::<Vec<String>>()
//                 .join("."),
//             "2.5.4.3"
//         );
//         if let Some(p) = parameters {
//             match p {
//                 ASN1Value::NullValue => (),
//                 _ => panic!(),
//             }
//         } else {
//             panic!();
//         }
//     }

//     #[test]
//     fn test_decode_algorithm_identifier() {
//         let root: X690Element = X690Element::new(
//             Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
//             X690Value::Constructed(Arc::new(vec![
//                 X690Element::new(
//                     Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER),
//                     X690Value::Primitive(Bytes::copy_from_slice(&[0x55, 0x04, 0x03])),
//                 ),
//                 X690Element::new(
//                     Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_NULL),
//                     X690Value::Primitive(Bytes::copy_from_slice(&[])),
//                 ),
//             ])),
//         );
//         let alg_id = decode_AlgorithmIdentifier(&root).unwrap();
//         assert_eq!(
//             alg_id
//                 .algorithm
//                 .0
//                 .iter()
//                 .map(|a| a.to_string())
//                 .collect::<Vec<String>>()
//                 .join("."),
//             "2.5.4.3"
//         );
//         if let Some(p) = alg_id.parameters {
//             match p {
//                 ASN1Value::NullValue => (),
//                 _ => panic!(),
//             }
//         } else {
//             panic!();
//         }
//     }

//     #[test]
//     fn test_decode_directory_string() {
//         let root: X690Element = X690Element::new(
//             Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING),
//             X690Value::Primitive(Bytes::from("Better Call Saul!")),
//         );
//         let ds = decode_DirectoryString(&root).unwrap();
//         if let DirectoryString::UTF8String(s) = ds {
//             assert_eq!(s, String::from("Better Call Saul!"));
//         } else {
//             panic!();
//         }
//     }

//     pub const rlrq_rctl1: [ComponentSpec; 1] = [ComponentSpec::new(
//         "reason",
//         true,
//         TagSelector::tag((TagClass::CONTEXT, 0)),
//         None,
//         None,
//     )];
//     pub const rlrq_rctl2: [ComponentSpec; 1] = [ComponentSpec::new(
//         "user-information",
//         true,
//         TagSelector::tag((TagClass::CONTEXT, 30)),
//         None,
//         None,
//     )];
//     pub const rlrq_eal: [ComponentSpec; 2] = [
//         ComponentSpec::new(
//             "aso-qualifier",
//             true,
//             TagSelector::tag((TagClass::CONTEXT, 13)),
//             None,
//             None,
//         ),
//         ComponentSpec::new(
//             "asoi-identifier",
//             true,
//             TagSelector::tag((TagClass::CONTEXT, 14)),
//             None,
//             None,
//         ),
//     ];

//     #[test]
//     fn test_decode_ACSE_RLRQ_APDU() {
//         let encoded_data: Vec<u8> = vec![
//             0x62, 0x0f, // RLRQ APDU
//             0x80, 0x01, 0x00, // reason: [CONTEXT 0] 0 (normal)
//             0x04, 0x0a, 0x8e, 0x44, 0x22, 0x8c, 0x90, 0x52, 0x6d, 0x5a, 0xd3,
//             0x8a, // Some unrecognized extension.
//         ];
//         let (bytes_read, root) = match ber_cst(encoded_data.as_slice()) {
//             Err(_) => panic!("asdf"),
//             Ok(result) => result,
//         };
//         assert_eq!(bytes_read, encoded_data.len());

//         let elements = match &root.value {
//             X690Value::Constructed(children) => children,
//             _ => panic!(),
//         };
//         let el_refs = elements.iter().collect::<Vec<&X690Element>>();

//         let (components, unrecognized) = _parse_sequence(
//             el_refs.as_slice(),
//             rlrq_rctl1.as_slice(),
//             rlrq_eal.as_slice(),
//             rlrq_rctl2.as_slice(),
//         )
//         .unwrap();
//         assert_eq!(components.len(), 1);
//         assert_eq!(unrecognized.len(), 1);
//     }

//     #[test]
//     #[should_panic]
//     fn test_decode_ACSE_RLRQ_APDU_trailing_unrecognized_component() {
//         let elements: Vec<X690Element> = vec![
//             X690Element::new(
//                 Tag::new(TagClass::CONTEXT, 0),
//                 X690Value::Primitive(Bytes::new()),
//             ),
//             X690Element::new(
//                 Tag::new(TagClass::CONTEXT, 30),
//                 X690Value::Constructed(Arc::new(vec![])),
//             ),
//             X690Element::new(
//                 Tag::new(TagClass::CONTEXT, 21),
//                 X690Value::Primitive(Bytes::copy_from_slice(&[5])),
//             ),
//         ];
//         let el_refs = elements.iter().collect::<Vec<&X690Element>>();
//         let (_, _) = _parse_sequence(
//             el_refs.as_slice(),
//             rlrq_rctl1.as_slice(),
//             rlrq_eal.as_slice(),
//             rlrq_rctl2.as_slice(),
//         )
//         .unwrap();
//     }
// }
