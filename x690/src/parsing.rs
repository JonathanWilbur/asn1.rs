#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::{X690Element, X690Encoding};
use asn1::construction::{ComponentSpec, TagSelector};
use asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use asn1::types::Tag;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Return `true` if successfully handled; `false` if error. Parsing will not continue if `false` returned.
pub type ComponentHandler<'a> = &'a dyn FnMut(&X690Element) -> bool;

pub type ComponentHandlers<'a> = HashMap<&'a str, ComponentHandler<'a>>;

pub type AlternativeHandlers<'a> = HashMap<Tag, (&'a str, ComponentHandler<'a>)>;

pub type Decoder<T> = fn(el: &X690Element) -> ASN1Result<T>;

pub type IndexedComponents<'a> = (HashMap<&'a str, X690Element>, Vec<X690Element>);

pub fn component_is_selected(el: &X690Element, sel: TagSelector) -> bool {
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
    elements: &'a [&'a X690Element],
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

    // Check for duplicates
    let mut encountered_tags: HashSet<Tag> = HashSet::new();
    for el in elements {
        if encountered_tags.contains(&Tag::new(el.tag_class, el.tag_number)) {
            let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_tags_in_set);
            err.component_name = el.name.clone();
            err.tag = Some(Tag::new(el.tag_class, el.tag_number));
            err.length = Some(el.len());
            err.constructed = Some(el.is_constructed());
            return Err(err);
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
            }
            _ => (),
        }
    }

    let mut ret: IndexedComponents = (HashMap::new(), Vec::new());
    for el in elements {
        match tag_to_spec.get(&Tag::new(el.tag_class, el.tag_number)) {
            Some(s) => {
                if encountered_components.contains(s.name) {
                    let mut err = ASN1Error::new(ASN1ErrorCode::duplicate_components);
                    err.component_name = Some(String::from(s.name));
                    err.tag = Some(Tag::new(el.tag_class, el.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(el.is_constructed());
                    return Err(err);
                }
                encountered_components.insert(s.name);
                if let Some(group_index) = s.group_index {
                    encountered_ext_groups.insert(group_index);
                }
                ret.0.insert(s.name, (**el).clone());
            }
            None => ret.1.push((**el).clone()),
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
                if *exg == group_index
                    && !spec.optional
                    && !encountered_components.contains(spec.name)
                {
                    missing_required_components.push(spec.name);
                }
            }
        }
    }

    if missing_required_components.len() > 0 {
        let mut err = ASN1Error::new(ASN1ErrorCode::missing_required_components);
        err.component_name = Some(String::from(missing_required_components[0]));
        return Err(err);
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

// pub fn _decode_choice (
//     el: &mut X690Element,
//     handlers: AlternativeHandlers,
//     unrecognized_handler: Option<ComponentHandler>,
// ) -> ASN1Result<()> {
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

pub fn _decode_sequence_of<T>(el: &X690Element, item_decoder: Decoder<T>) -> ASN1Result<Vec<T>> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
        _ => {
            let mut err = ASN1Error::new(ASN1ErrorCode::nonsense);
            err.component_name = el.name.clone();
            err.tag = Some(Tag::new(el.tag_class, el.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            return Err(err);
        }
    };
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

pub fn _decode_set_of<T>(el: &X690Element, item_decoder: Decoder<T>) -> ASN1Result<Vec<T>> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
        _ => {
            let mut err = ASN1Error::new(ASN1ErrorCode::nonsense);
            err.component_name = el.name.clone();
            err.tag = Some(Tag::new(el.tag_class, el.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            return Err(err);
        }
    };
    let mut ret: Vec<T> = Vec::with_capacity(elements.len());
    for element in elements {
        let v = item_decoder(&element)?;
        ret.push(v);
    }
    Ok(ret)
}

// pub fn _decode_explicit <T> (decoder: Decoder<T>) -> T {
//     decoder()
// }

// export function _encode_explicit (
//     class_: ASN1TagClass | undefined,
//     tag: number | undefined,
//     encoderGetter: () => ASN1Encoder<any>,
//     outer:  ASN1Encoder<any>,
// ): ASN1Encoder<any> {
//     return function (value: any, elGetter: ASN1Encoder<any>): ASN1Element {
//         const ret: ASN1Element = outer(value, outer);
//         ret.sequence = [ encoderGetter()(value, elGetter) ];
//         ret.construction = ASN1Construction.constructed;
//         if (class_) {
//             ret.tagClass = class_;
//         }
//         if (typeof tag !== "undefined") {
//             ret.tagNumber = tag;
//         }
//         return ret;
//     };
// }

// export function _decode_explicit<T> (decoderGetter: () => (el: ASN1Element) => T): ASN1Decoder<T> {
//     return (el: ASN1Element) => decoderGetter()(el.inner);
// }

pub fn _encode_explicit(el: X690Element, tag: Tag) -> X690Element {
    X690Element::new(
        tag.tag_class,
        tag.tag_number,
        Arc::new(X690Encoding::EXPLICIT(Box::new(el))),
    )
}

pub fn _encode_implicit(el: X690Element, tag: Tag) -> X690Element {
    X690Element::new(
        tag.tag_class,
        tag.tag_number,
        Arc::new(X690Encoding::EXPLICIT(Box::new(el))),
    )
}

pub const ZXCV: fn(X690Element, Tag) -> X690Element = |el: X690Element, tag: Tag| -> X690Element {
    X690Element::new(
        tag.tag_class,
        tag.tag_number,
        Arc::new(X690Encoding::EXPLICIT(Box::new(el))),
    )
};

// export function _encode_implicit (
//     class_: ASN1TagClass | undefined,
//     tag: number | undefined,
//     encoderGetter: () => ASN1Encoder<any>,
//     outer:  ASN1Encoder<any>, // eslint-disable-line
//     // ^ This needs to remain here.
// ): ASN1Encoder<any> {
//     return function (value: any, elGetter: ASN1Encoder<any>): ASN1Element {
//         const ret: ASN1Element = encoderGetter()(value, elGetter);
//         if (class_) {
//             ret.tagClass = class_;
//         }
//         if (typeof tag !== "undefined") {
//             ret.tagNumber = tag;
//         }
//         return ret;
//     };
// }

// export function _decode_implicit<T> (decoderGetter: () => (el: ASN1Element) => T): ASN1Decoder<T> {
//     return (el: ASN1Element) => decoderGetter()(el);
// }

// TODO: Implement a deep_clone that recurses into the value as well.
// This works as an encode and decode function.
pub fn x690_identity(el: &X690Element) -> ASN1Result<X690Element> {
    Ok(el.clone())
}

#[cfg(test)]
mod tests {

    use crate::*;
    use asn1::types::{
        ASN1Value, TagClass, ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING, ASN1_UNIVERSAL_TAG_NUMBER_NULL,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
        ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING, OBJECT_IDENTIFIER,
    };
    use std::sync::Arc;

    use crate::ber::{
        ber_decode_any, ber_decode_bmp_string, ber_decode_object_identifier, ber_decode_utf8_string,
    };

    use super::*;

    struct AlgorithmIdentifier {
        pub algorithm: OBJECT_IDENTIFIER,
        pub parameters: Option<ASN1Value>,
    }

    const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 3] = &[
        ComponentSpec::new(
            "algorithm",
            false,
            TagSelector::tag((
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
            )),
            None,
            None,
        ),
        ComponentSpec::new("parameters", true, TagSelector::any, None, None),
        ComponentSpec::new(
            "asdf",
            true,
            TagSelector::or(&[&TagSelector::any, &TagSelector::any]),
            None,
            None,
        ),
    ];

    const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];
    const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

    fn decode_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => panic!(),
        };
        let el_refs = elements.iter().collect::<Vec<&X690Element>>();
        let (components, _) = _parse_sequence(
            el_refs.as_slice(),
            _rctl1_components_for_AlgorithmIdentifier,
            _eal_components_for_AlgorithmIdentifier,
            _rctl2_components_for_AlgorithmIdentifier,
        )?;
        // NOTE: unwrap() should be fine, because we validate that there is such a component in `_parse_sequence`.
        let algorithm: OBJECT_IDENTIFIER =
            ber_decode_object_identifier(components.get("algorithm").unwrap())?;
        let parameters: Option<ASN1Value> = match components.get("parameters") {
            Some(c) => Some(ber_decode_any(c)?),
            _ => None,
        };
        Ok(AlgorithmIdentifier {
            algorithm,
            parameters,
        })
    }

    enum DirectoryString {
        UTF8String(String),
        BMPString(String),
    }

    fn decode_DirectoryString(el: &X690Element) -> ASN1Result<DirectoryString> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING) => {
                let v = ber_decode_utf8_string(&el)?;
                return Ok(DirectoryString::UTF8String(v));
            }
            (TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING) => {
                let v = ber_decode_bmp_string(&el)?;
                return Ok(DirectoryString::BMPString(v));
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }

    #[test]
    fn test_decode_sequence() {
        let rctl1: Vec<ComponentSpec> = vec![
            ComponentSpec::new(
                "algorithm",
                false,
                TagSelector::tag((
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                )),
                None,
                None,
            ),
            ComponentSpec::new("parameters", true, TagSelector::any, None, None),
        ];
        let eal: Vec<ComponentSpec> = vec![];
        let rctl2: Vec<ComponentSpec> = vec![];

        let root: X690Element = X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(vec![
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                    Arc::new(X690Encoding::IMPLICIT(vec![0x55, 0x04, 0x03])),
                ),
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_NULL,
                    Arc::new(X690Encoding::IMPLICIT(vec![])),
                ),
            ])),
        );
        let elements = match root.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => panic!(),
        };
        let el_refs = elements.iter().collect::<Vec<&X690Element>>();
        let (components, unrecognized) = _parse_sequence(
            el_refs.as_slice(),
            rctl1.as_slice(),
            eal.as_slice(),
            rctl2.as_slice(),
        )
        .unwrap();
        let algorithm: OBJECT_IDENTIFIER =
            ber_decode_object_identifier(components.get("algorithm").unwrap()).unwrap();
        let parameters: Option<ASN1Value> = components
            .get("parameters")
            .and_then(|c| Some(ber_decode_any(c).unwrap()));
        assert_eq!(unrecognized.len(), 0);
        assert_eq!(
            algorithm
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("."),
            "2.5.4.3"
        );
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
    fn test_decode_algorithm_identifier() {
        let root: X690Element = X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(vec![
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
                    Arc::new(X690Encoding::IMPLICIT(vec![0x55, 0x04, 0x03])),
                ),
                X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_NULL,
                    Arc::new(X690Encoding::IMPLICIT(vec![])),
                ),
            ])),
        );
        let alg_id = decode_AlgorithmIdentifier(&root).unwrap();
        assert_eq!(
            alg_id
                .algorithm
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("."),
            "2.5.4.3"
        );
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
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING,
            Arc::new(X690Encoding::IMPLICIT(Vec::from(String::from(
                "Better Call Saul!",
            )))),
        );
        let ds = decode_DirectoryString(&root).unwrap();
        if let DirectoryString::UTF8String(s) = ds {
            assert_eq!(s, String::from("Better Call Saul!"));
        } else {
            panic!();
        }
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

    #[test]
    fn test_decode_ACSE_RLRQ_APDU() {
        let encoded_data: Vec<u8> = vec![
            0x62, 0x0f, // RLRQ APDU
            0x80, 0x01, 0x00, // reason: [CONTEXT 0] 0 (normal)
            0x04, 0x0a, 0x8e, 0x44, 0x22, 0x8c, 0x90, 0x52, 0x6d, 0x5a, 0xd3,
            0x8a, // Some unrecognized extension.
        ];
        let (bytes_read, root) = match ber_cst(encoded_data.as_slice()) {
            Err(_) => panic!("asdf"),
            Ok(result) => result,
        };
        assert_eq!(bytes_read, encoded_data.len());

        let elements = match root.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => panic!(),
        };
        let el_refs = elements.iter().collect::<Vec<&X690Element>>();

        let (components, unrecognized) = _parse_sequence(
            el_refs.as_slice(),
            rlrq_rctl1.as_slice(),
            rlrq_eal.as_slice(),
            rlrq_rctl2.as_slice(),
        )
        .unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(unrecognized.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_decode_ACSE_RLRQ_APDU_trailing_unrecognized_component() {
        let elements: Vec<X690Element> = vec![
            X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::IMPLICIT(vec![0])),
            ),
            X690Element::new(
                TagClass::CONTEXT,
                30,
                Arc::new(X690Encoding::Constructed(vec![])),
            ),
            X690Element::new(
                TagClass::CONTEXT,
                21,
                Arc::new(X690Encoding::IMPLICIT(vec![5])),
            ),
        ];
        let el_refs = elements.iter().collect::<Vec<&X690Element>>();
        let (_, _) = _parse_sequence(
            el_refs.as_slice(),
            rlrq_rctl1.as_slice(),
            rlrq_eal.as_slice(),
            rlrq_rctl2.as_slice(),
        )
        .unwrap();
    }
}
