use std::io::{ErrorKind, Result, Error};
use std::collections::{HashSet, HashMap};
use asn1::types::{TagClass, TagNumber, Tag, SEQUENCE_OF};
use asn1::construction::{TagSelector, ComponentSpec};
use crate::{X690Element, X690Encoding};

// Return `true` if successfully handled; `false` if error. Parsing will not continue if `false` returned.
pub type ComponentHandler <'a> = &'a dyn FnMut (&X690Element) -> bool;

pub type ComponentHandlers <'a> = HashMap<&'a str, ComponentHandler<'a>>;

pub type AlternativeHandlers <'a> = HashMap<Tag, (&'a str, ComponentHandler<'a>)>;

pub type Decoder <T> = fn (el: &X690Element) -> Result<T>;

pub fn component_is_selected (el: &X690Element, sel: TagSelector) -> bool {
    match sel {
        TagSelector::tag((tc, tn)) => (el.tag_class == tc) && (el.tag_number == tn),
        TagSelector::any => true,
        TagSelector::class(tc) => el.tag_class == tc,
        TagSelector::number(tn) => el.tag_number == tn,
        TagSelector::and(sels) => {
            for s in sels {
                if !component_is_selected(el, **s) {
                    return false;
                }
            }
            return true;
        },
        TagSelector::or(sels) => {
            for s in sels {
                if component_is_selected(el, **s) {
                    return true;
                }
            }
            return false;
        },
        TagSelector::not(n) => !component_is_selected(el, *n),
    }
}

// TODO: Avoid using HashMaps and Sets if the number of components is small.
pub fn _parse_set (
    root: X690Element,
    handlers: ComponentHandlers,
    rctl1: &[ComponentSpec],
    eal: &[ComponentSpec],
    rctl2: &[ComponentSpec],
    unrecognized_handler: ComponentHandler,
    max_elements: usize,
) -> Result<()> {
    let elements = match root.value {
        X690Encoding::Constructed(children) => children,
        _ => return Err(Error::from(ErrorKind::InvalidInput)),
    };
    if elements.len() > max_elements {
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    // Check for duplicates
    let mut encountered_tags: HashSet<Tag> = HashSet::new();
    for el in &elements {
        // TODO: Is checking if a _reference_ to a tag going to work?
        if encountered_tags.contains(&Tag::new(el.tag_class, el.tag_number)) {
            return Err(Error::from(ErrorKind::InvalidInput)); 
        }
        encountered_tags.insert(Tag::new(el.tag_class, el.tag_number));
    }

    let mut encountered_components: HashSet<&str> = HashSet::new();
    let mut encountered_ext_groups: HashSet<u8> = HashSet::new();

    // Instead of iterating over every `ComponentSpec` for each component (O(n^2)),
    // we can pre-index the specs by (tag_class,tag_number)
    let mut tag_to_spec: HashMap<Tag, ComponentSpec> = HashMap::new();
    for spec in rctl1.iter().chain(eal).into_iter().chain(rctl2).into_iter() {
        match spec.selector {
            TagSelector::tag(tag) => {
                tag_to_spec.insert(Tag::new(tag.0, tag.1), *spec);
                ()
            },
            _ => ()
        }
    }

    for el in &elements {
        match tag_to_spec.get(&Tag::new(el.tag_class, el.tag_number)) {
            Some(s) => {
                if encountered_components.contains(s.name) {
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
                encountered_components.insert(s.name);
                if let Some(group_index) = s.group_index {
                    encountered_ext_groups.insert(group_index);
                }
                match handlers.get(s.name) {
                    Some(handler) => {
                        if !(handler)(&el) {
                            break;
                        }
                    },
                    None => {
                        if !unrecognized_handler(&el) {
                            break;
                        }
                    },
                }
            },
            None => {
                if !unrecognized_handler(&el) {
                    break;
                }
            },
        }
    }

    let mut missing_required_components: Vec<&str> = Vec::new();
    for spec in rctl1.iter().chain(rctl2).into_iter() {
        if spec.optional || encountered_components.contains(spec.name) {
            continue;
        }
        missing_required_components.push(spec.name);
    }

    for exg in &encountered_ext_groups {
        for spec in eal {
            // gi = exg && !spec.optional && !encountered_components.has(spec.name): add missing_req_component
            if let Some(group_index) = spec.group_index {
                if *exg == group_index && !spec.optional && !encountered_components.contains(spec.name) {
                    missing_required_components.push(spec.name);
                }   
            }
        }
    }

    if missing_required_components.len() > 0 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    Ok(())
}

pub fn _parse_component_type_list (
    ctl: &[ComponentSpec],
    handlers: &ComponentHandlers,
    elements: &mut [X690Element],
    is_extensions: bool,
) -> Result<usize> {
    let mut e: usize = 0;
    let mut s: usize = 0;
    let mut current_group: Option<u8> = None;
    while s < ctl.len() {
        let spec = ctl[s];
        let el = match elements.get_mut(e) {
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
                            return Err(Error::from(ErrorKind::InvalidInput));
                        } else {
                            s += 1;
                            continue;
                        }
                    } else {
                        s += 1;
                        continue;
                    }
                } else {
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            },
        };
        if component_is_selected(el, spec.selector) {
            el.name = Some(String::from(spec.name));
            match handlers.get(spec.name) {
                Some(handler) => {
                    if !handler(&el) {
                        break;
                    }
                },
                _ => (),
            }
            if let Some(group_index) = spec.group_index {
                current_group = Some(group_index);
            }
            e += 1; // Only if it is a match do you increment the element.
        } else if !spec.optional {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        s += 1;
    }
    Ok(e)
}

fn _get_possible_initial_components (ctl: &[ComponentSpec]) -> usize {
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

pub fn _parse_sequence_with_trailing_rctl (
    root: X690Element,
    handlers: ComponentHandlers,
    rctl1: &[ComponentSpec],
    eal: &[ComponentSpec],
    rctl2: &[ComponentSpec],
    unrecognized_handler: ComponentHandler,
    // max_elements: usize,
) -> Result<()> {
    let mut elements = match root.value {
        X690Encoding::Constructed(children) => children,
        _ => return Err(Error::from(ErrorKind::InvalidInput)),
    };
    let start_of_exts = _parse_component_type_list(rctl1, &handlers, &mut elements, false)?;
    let end_of_possible_rctl2_components = _get_possible_initial_components(rctl2);
    let possible_initial_rctl2_components = &rctl2[0..end_of_possible_rctl2_components];
    // let mut rctl2_entirely_optional: bool = true;
    // for component in rctl2 {
    //     if !component.optional {
    //         rctl2_entirely_optional = false;
    //         break;
    //     }
    // }
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
        if number_of_ext_components.is_some() {
            break;
        }
    }
    // NOTE: I deviated from the TypeScript implementation here. I don't see
    // how the value `startOfExtensions` could ever be -1.
    let start_of_rctl2 = start_of_exts + number_of_ext_components.unwrap_or(0);
    let exts_read = _parse_component_type_list(
        eal,
        &handlers,
        &mut elements[start_of_exts..start_of_rctl2],
        true,
    )?;
    let end_of_recognized_exts = start_of_exts + exts_read;
    for el in &elements[end_of_recognized_exts..start_of_rctl2] {
        if !unrecognized_handler(&el) {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
    }
    let rctl2_components_read = _parse_component_type_list(
        rctl2,
        &handlers,
        &mut elements[start_of_rctl2..],
        false,
    )?;
    if start_of_rctl2 + rctl2_components_read > elements.len() {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    Ok(())
}

pub fn _parse_sequence_without_trailing_rctl (
    root: X690Element,
    handlers: ComponentHandlers,
    rctl1: &[ComponentSpec],
    eal: &[ComponentSpec],
    unrecognized_handler: ComponentHandler,
) -> Result<()> {
    let mut elements = match root.value {
        X690Encoding::Constructed(children) => children,
        _ => return Err(Error::from(ErrorKind::InvalidInput)),
    };
    let start_of_exts = _parse_component_type_list(rctl1, &handlers, &mut elements, false)?;
    let exts_read = _parse_component_type_list(eal, &handlers, &mut elements[start_of_exts..], false)?;
    let end_of_recognized_exts = match start_of_exts.checked_add(exts_read) {
        Some(e) => e,
        None => return Err(Error::from(ErrorKind::InvalidInput)),
    };
    for el in elements[end_of_recognized_exts..].into_iter() {
        unrecognized_handler(&el);
    }
    Ok(())
}

pub fn _parse_sequence (
    root: X690Element,
    handlers: ComponentHandlers,
    rctl1: &[ComponentSpec],
    eal: &[ComponentSpec],
    rctl2: &[ComponentSpec],
    unrecognized_handler: ComponentHandler,
    // max_elements: usize,
) -> Result<()> {
    // TODO: Extract elements here.
    if rctl2.len() > 0 {
        return _parse_sequence_with_trailing_rctl(
            root,
            handlers,
            rctl1,
            eal,
            rctl2,
            unrecognized_handler,
        );
    } else {
        return _parse_sequence_without_trailing_rctl(
            root,
            handlers,
            rctl1,
            eal,
            unrecognized_handler,
        );
    }
}

// pub struct

pub fn _decode_choice (
    el: &mut X690Element,
    handlers: AlternativeHandlers,
    unrecognized_handler: Option<ComponentHandler>,
) -> Result<()> {
    let tag = Tag::new(el.tag_class, el.tag_number);
    match handlers.get(&tag) {
        Some((name, handler)) => {
            el.name = Some(String::from(*name));
            if !handler(&el) {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
        },
        None => {
            if let Some(handler) = unrecognized_handler {
                if !handler(&el) {
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            } else {
                return Err(Error::from(ErrorKind::InvalidInput));

            }
        }
    }
    Ok(())
}

pub fn _decode_sequence_of <T> (el: &X690Element, item_decoder: Decoder<T>) -> Result<Vec<T>> {
    let elements = match &el.value {
        X690Encoding::Constructed(children) => children,
        _ => return Err(Error::from(ErrorKind::InvalidInput)),
    };
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

pub fn _decode_set_of <T> (el: &X690Element, item_decoder: Decoder<T>) -> Result<Vec<T>> {
    let elements = match &el.value {
        X690Encoding::Constructed(children) => children,
        _ => return Err(Error::from(ErrorKind::InvalidInput)),
    };
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

// FIXME:
/*

I think I am going to re-imagine these functions a little bit: instead of
passing in a mapping of closures (getting unwieldy), we merely index X690Elements
by their component type, and let the caller decide what to do with the resulting
values.

The returned values could just be an array that is 1:1 with rctl1 + eal + rctl2.
This would most likely be more performant, but harder to read.

```rust
let algorithm = match components.get("algorithm") {
    Some(c) => ber_decode_object_identifier(c)?,
    None => return Err(e),
};
```

*/

#[cfg(test)]
mod tests {

    use asn1::types::{
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
        ASN1_UNIVERSAL_TAG_NUMBER_NULL, OBJECT_IDENTIFIER, ASN1Value,
    };

    use crate::ber::{
        ber_decode_any,
        ber_decode_object_identifier,
    };

    use super::*;

    #[test]
    fn test_decode_sequence () {
        let rctl1: [&ComponentSpec; 2] = [
            &ComponentSpec::new(
                "algorithm",
                false,
                TagSelector::tag((TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER)),
                None,
                None,
            ),
            &ComponentSpec::new(
                "parameters",
                true,
                TagSelector::any,
                None,
                None,
            ),
        ];
        let root: X690Element = X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            X690Encoding::Constructed(vec![
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                    X690Encoding::IMPLICIT(vec![ 0x55, 0x04, 0x03 ]),
                ),
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_NULL,
                    X690Encoding::IMPLICIT(vec![]),
                ),
            ]),
        );
        let mut algorithm: Option<OBJECT_IDENTIFIER> = None;
        let mut parameters: Option<ASN1Value> = None;
        let _read_algorithm = |el: &X690Element| -> bool {
            match ber_decode_object_identifier(*el) {
                Ok(v) => {
                    algorithm = Some(v);
                    return true;
                },
                Err(e) => return false,
            }
        };
        let mut algorithm: Option<ASN1Value> = None;
        fn _read_parameters (el: &X690Element) -> bool {
            match ber_decode_any(*el) {
                Ok(v) => {
                    parameters = v;
                },
                Err(e) => return false,
            }f
        }
        let handlers: ComponentHandlers = HashMap::new();
        handlers.insert("algorithm", _read_algorithm);
        _parse_sequence(root, handlers, rctl1, eal, rctl2, unrecognized_handler)
    }

}