use std::io::{ErrorKind, Result, Error};
use std::collections::{HashSet, HashMap};
use asn1::types::Tag;
use asn1::construction::{TagSelector, ComponentSpec};
use crate::{X690Element, X690Encoding};

// Return `true` if successfully handled; `false` if error. Parsing will not continue if `false` returned.
pub type ComponentHandler <'a> = &'a dyn FnMut (&X690Element) -> bool;

pub type ComponentHandlers <'a> = HashMap<&'a str, ComponentHandler<'a>>;

pub type AlternativeHandlers <'a> = HashMap<Tag, (&'a str, ComponentHandler<'a>)>;

pub type Decoder <T> = fn (el: &X690Element) -> Result<T>;

pub type IndexedComponents <'a, 'b> = (HashMap<&'a str, &'b X690Element>, Vec<&'b X690Element>);

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
pub fn _parse_set <'a> (
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
    rctl2: &'a [ComponentSpec],
    max_elements: usize,
) -> Result<IndexedComponents<'a, 'a>> {
    if elements.len() > max_elements {
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    // Check for duplicates
    let mut encountered_tags: HashSet<Tag> = HashSet::new();
    for el in elements {
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

    let mut ret: IndexedComponents = (HashMap::new(), Vec::new());
    for el in elements {
        match tag_to_spec.get(&Tag::new(el.tag_class, el.tag_number)) {
            Some(s) => {
                if encountered_components.contains(s.name) {
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
                encountered_components.insert(s.name);
                if let Some(group_index) = s.group_index {
                    encountered_ext_groups.insert(group_index);
                }
                ret.0.insert(s.name, el);
            },
            None => ret.1.push(el),
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

    Ok(ret)
}

pub fn _parse_component_type_list <'a> (
    ctl: &'a [ComponentSpec],
    elements: &'a [&'a X690Element],
    is_extensions: bool,
) -> Result<(usize, IndexedComponents<'a, 'a>)> {
    let mut e: usize = 0;
    let mut s: usize = 0;
    let mut current_group: Option<u8> = None;
    let mut ret: IndexedComponents = (HashMap::new(), Vec::new());
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
            ret.0.insert(spec.name, el);
            if let Some(group_index) = spec.group_index {
                current_group = Some(group_index);
            }
            e += 1; // Only if it is a match do you increment the element.
        } else if !spec.optional {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        s += 1;
    }
    Ok((e, ret))
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

pub fn _parse_sequence_with_trailing_rctl <'a> (
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
    rctl2: &'a [ComponentSpec],
) -> Result<IndexedComponents<'a, 'a>> {
    let (start_of_exts, rctl1_index) = _parse_component_type_list(
        rctl1,
        elements,
        false,
    )?;
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
        if number_of_ext_components.is_some() {
            break;
        }
    }
    // NOTE: I deviated from the TypeScript implementation here. I don't see
    // how the value `startOfExtensions` could ever be -1.
    let start_of_rctl2 = start_of_exts + number_of_ext_components.unwrap_or(0);
    let (exts_read, eal_index) = _parse_component_type_list(
        eal,
        &elements[start_of_exts..start_of_rctl2],
        true,
    )?;
    let (rctl2_components_read, rctl2_index) = _parse_component_type_list(
        rctl2,
        &elements[start_of_rctl2..],
        false,
    )?;
    if start_of_rctl2 + rctl2_components_read > elements.len() {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    // let end_of_recognized_exts = start_of_exts + exts_read;
    let mut ret: IndexedComponents = rctl1_index;
    ret.0.extend(eal_index.0);
    ret.1.extend(eal_index.1);
    ret.0.extend(rctl2_index.0);
    ret.1.extend(rctl2_index.1);
    Ok(ret)
}

pub fn _parse_sequence_without_trailing_rctl <'a> (
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
) -> Result<IndexedComponents<'a, 'a>> {
    let (start_of_exts, rctl_index) = _parse_component_type_list(
        rctl1, 
        &elements,
        false,
    )?;
    let (exts_read, eal_index) = _parse_component_type_list(
        eal,
        &elements[start_of_exts..],
        true,
    )?;
    let end_of_recognized_exts = start_of_exts + exts_read;
    let mut ret: IndexedComponents = rctl_index;
    ret.0.extend(eal_index.0);
    for el in elements[end_of_recognized_exts..].into_iter() {
        ret.1.push(el);
    }
    Ok(ret)
}

pub fn _parse_sequence <'a> (
    elements: &'a [&'a X690Element],
    rctl1: &'a [ComponentSpec],
    eal: &'a [ComponentSpec],
    rctl2: &'a [ComponentSpec],
) -> Result<IndexedComponents<'a, 'a>> {
    if rctl2.len() > 0 {
        return _parse_sequence_with_trailing_rctl(
            elements,
            rctl1,
            eal,
            rctl2,
        );
    } else {
        return _parse_sequence_without_trailing_rctl(
            elements,
            rctl1,
            eal,
        );
    }
}

// pub struct

// pub fn _decode_choice (
//     el: &mut X690Element,
//     handlers: AlternativeHandlers,
//     unrecognized_handler: Option<ComponentHandler>,
// ) -> Result<()> {
//     let tag = Tag::new(el.tag_class, el.tag_number);
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

// FIXME: Use Arc<X690Element> instead of cloning.

#[cfg(test)]
mod tests {

    use asn1::types::{
        TagClass,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
        ASN1_UNIVERSAL_TAG_NUMBER_NULL, OBJECT_IDENTIFIER, ASN1Value,
    };

    use crate::ber::{
        ber_decode_any,
        ber_decode_object_identifier,
    };

    use super::*;

    struct AlgorithmIdentifier {
        pub algorithm: OBJECT_IDENTIFIER,
        pub parameters: Option<ASN1Value>,
    }

    const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 2] = &[
        ComponentSpec::new(
            "algorithm",
            false,
            TagSelector::tag((TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER)),
            None,
            None,
        ),
        ComponentSpec::new(
            "parameters",
            true,
            TagSelector::any,
            None,
            None,
        ),
    ];

    const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];
    const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

    fn decode_AlgorithmIdentifier (el: X690Element) -> Result<AlgorithmIdentifier> {
        let elements = match el.value {
            X690Encoding::Constructed(children) => children,
            _ => panic!(),
        };
        let el_refs = elements.iter().collect::<Vec<&X690Element>>();
        let (components, unrecognized) = _parse_sequence(
            el_refs.as_slice(),
            _rctl1_components_for_AlgorithmIdentifier,
            _eal_components_for_AlgorithmIdentifier,
            _rctl2_components_for_AlgorithmIdentifier,
        ).unwrap();
        let algorithm: OBJECT_IDENTIFIER = ber_decode_object_identifier(components.get("algorithm").unwrap()).unwrap();
        let parameters: Option<ASN1Value> = match components.get("parameters") {
            Some(c) => {
                match ber_decode_any(c) {
                    Ok(v) => Some(v),
                    Err(e) => return Err(e),
                }
            },
            None => None,
        };
        Ok(AlgorithmIdentifier {
            algorithm,
            parameters,
        })
    }

    #[test]
    fn test_decode_sequence () {
        let rctl1: Vec<ComponentSpec> = vec![
            ComponentSpec::new(
                "algorithm",
                false,
                TagSelector::tag((TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER)),
                None,
                None,
            ),
            ComponentSpec::new(
                "parameters",
                true,
                TagSelector::any,
                None,
                None,
            ),
        ];
        let eal: Vec<ComponentSpec> = vec![];
        let rctl2: Vec<ComponentSpec> = vec![];

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
        let elements = match root.value {
            X690Encoding::Constructed(children) => children,
            _ => panic!(),
        };
        let el_refs = elements.iter().collect::<Vec<&X690Element>>();
        let (components, unrecognized) = _parse_sequence(
            el_refs.as_slice(),
            rctl1.as_slice(),
            eal.as_slice(),
            rctl2.as_slice(),
        ).unwrap();
        let algorithm: OBJECT_IDENTIFIER = ber_decode_object_identifier(components.get("algorithm").unwrap()).unwrap();
        let parameters: Option<ASN1Value> = components.get("parameters").and_then(|c| Some(ber_decode_any(c).unwrap()));
        assert_eq!(unrecognized.len(), 0);
        assert_eq!(algorithm.iter().map(|a| a.to_string()).collect::<Vec<String>>().join("."), "2.5.4.3");
        if let Some(p) = parameters {
            match p {
                ASN1Value::NullValue => (),
                _ => panic!(),
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_algorithm_identifier () {
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
        let alg_id = decode_AlgorithmIdentifier(root).unwrap();
        assert_eq!(alg_id.algorithm.iter().map(|a| a.to_string()).collect::<Vec<String>>().join("."), "2.5.4.3");
        if let Some(p) = alg_id.parameters {
            match p {
                ASN1Value::NullValue => (),
                _ => panic!(),
            }
        } else {
            panic!();
        }
    }

}