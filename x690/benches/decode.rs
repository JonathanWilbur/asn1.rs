
use asn1::types::{
    ASN1Value, TagClass, UNIV_TAG_NULL,
    UNIV_TAG_OBJECT_IDENTIFIER,
    UNIV_TAG_SEQUENCE, OBJECT_IDENTIFIER,
    UNIV_TAG_SET,
    Tag,
};
use std::sync::Arc;
use x690::{X690Codec, X690Element, X690Value, _parse_set, BER};
use asn1::construction::{ComponentSpec, TagSelector};
use asn1::error::ASN1Result;
use std::borrow::Borrow;
use x690::{_parse_sequence, X690StructureIterator};
use criterion::{criterion_group, criterion_main, Criterion};
use bytes::Bytes;

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
            UNIV_TAG_OBJECT_IDENTIFIER,
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
        X690Value::Constructed(children) => children,
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
        BER.decode_object_identifier(components.get("algorithm").unwrap())?;
    let parameters: Option<ASN1Value> = match components.get("parameters") {
        Some(c) => Some(BER.decode_any(c)?),
        _ => None,
    };
    Ok(AlgorithmIdentifier {
        algorithm,
        parameters,
    })
}

fn decode_AlgorithmIdentifier2(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let elements = match el.value.borrow() {
        X690Value::Constructed(children) => children,
        _ => panic!(),
    };
    let seq_iter = X690StructureIterator::new(
        elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    ).into_iter();
    let mut i: usize = 0;
    let mut maybe_algorithm: Option<OBJECT_IDENTIFIER> = None;
    let mut maybe_parameters: Option<ASN1Value> = None;
    for fallible_component_name in seq_iter {
        let component_name = fallible_component_name?;
        let maybe_el = elements.get(i);
        i += 1;
        debug_assert!(maybe_el.is_some(), "Number of parsed components exceeded the number of elements!");
        let el = maybe_el.unwrap();
        // We assume there cannot be duplicates. The parser must be sound enough to not do this.
        match component_name {
            "algorithm" => maybe_algorithm = Some(BER.decode_object_identifier(el)?),
            "parameters" => maybe_parameters = Some(BER.decode_any(el)?),
            _ => panic!("There are no extensions in AlgorithmIdentifier!"),
        }
    }
    // NOTE: unwrap() should be fine, because we validate that there is such a component in `_parse_sequence`.
    let algorithm: OBJECT_IDENTIFIER = maybe_algorithm.unwrap();
    let parameters: Option<ASN1Value> = maybe_parameters;
    Ok(AlgorithmIdentifier {
        algorithm,
        parameters,
    })
}

fn decode_AlgorithmIdentifier3(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let elements = match el.value.borrow() {
        X690Value::Constructed(children) => children,
        _ => panic!(),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
        10,
    )?;
    let algorithm: OBJECT_IDENTIFIER = BER.decode_object_identifier(_components.get("algorithm").unwrap())?;
    let parameters: Option<ASN1Value> = match _components.get("parameters") {
        Some(c_) => Some(BER.decode_any(c_)?),
        _ => None,
    };
    Ok(AlgorithmIdentifier {
        algorithm,
        parameters,
    })
}

// This tests an unsafe, but possibly faster and more terse way.
fn decode_AlgorithmIdentifier4(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let elements = match el.value.borrow() {
        X690Value::Constructed(children) => children,
        _ => panic!(),
    };
    let seq_iter = X690StructureIterator::new(
        elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    ).into_iter();
    let mut i: usize = 0;
    let mut algorithm: OBJECT_IDENTIFIER = unsafe { std::mem::uninitialized() };
    let mut parameters: Option<ASN1Value> = None;
    for fallible_component_name in seq_iter {
        let component_name = fallible_component_name?;
        let maybe_el = elements.get(i);
        i += 1;
        debug_assert!(maybe_el.is_some(), "Number of parsed components exceeded the number of elements!");
        let el = maybe_el.unwrap();
        // We assume there cannot be duplicates. The parser must be sound enough to not do this.
        match component_name {
            "algorithm" => algorithm = BER.decode_object_identifier(el)?,
            "parameters" => parameters = Some(BER.decode_any(el)?),
            _ => panic!("There are no extensions in AlgorithmIdentifier!"),
        }
    }
    // NOTE: unwrap() should be fine, because we validate that there is such a component in `_parse_sequence`.
    Ok(AlgorithmIdentifier {
        algorithm,
        parameters,
    })
}

// This tests a more terse way as well as directly indexing elements[i]
fn decode_AlgorithmIdentifier5(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let elements = match el.value.borrow() {
        X690Value::Constructed(children) => children,
        _ => panic!(),
    };
    let seq_iter = X690StructureIterator::new(
        elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    ).into_iter();
    let mut i: usize = 0;
    let mut algorithm: Option<OBJECT_IDENTIFIER> = None;
    let mut parameters: Option<ASN1Value> = None;
    for fallible_component_name in seq_iter {
        let component_name = fallible_component_name?;
        let el = &elements[i];
        i += 1;
        // debug_assert!(maybe_el.is_some(), "Number of parsed components exceeded the number of elements!");
        // let el = maybe_el.unwrap();
        // We assume there cannot be duplicates. The parser must be sound enough to not do this.
        match component_name {
            "algorithm" => algorithm = Some(BER.decode_object_identifier(&el)?),
            "parameters" => parameters = Some(BER.decode_any(&el)?),
            _ => panic!("There are no extensions in AlgorithmIdentifier!"),
        }
    }
    Ok(AlgorithmIdentifier {
        algorithm: algorithm.unwrap(),
        parameters,
    })
}

fn decode_algorithm_identifier1() {
    let root: X690Element = X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(vec![
            X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
                X690Value::Primitive(Bytes::from(vec![0x55, 0x04, 0x03])),
            ),
            X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NULL),
                X690Value::Primitive(Bytes::from(vec![])),
            ),
        ])),
    );
    let alg_id = decode_AlgorithmIdentifier(&root).unwrap();
    if let Some(p) = alg_id.parameters {
        match p {
            ASN1Value::NullValue => (),
            _ => panic!(),
        }
    } else {
        panic!();
    }
}

fn decode_algorithm_identifier2() {
    let root: X690Element = X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(vec![
            X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
                X690Value::Primitive(Bytes::from(vec![0x55, 0x04, 0x03])),
            ),
            X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NULL),
                X690Value::Primitive(Bytes::from(vec![])),
            ),
        ])),
    );
    let alg_id = decode_AlgorithmIdentifier2(&root).unwrap();
    if let Some(p) = alg_id.parameters {
        match p {
            ASN1Value::NullValue => (),
            _ => panic!(),
        }
    } else {
        panic!();
    }
}

/* This example parses the encoded algorithm identifier as a SET. It is for
testing SET decoding. */
fn decode_algorithm_identifier3() {
    let root: X690Element = X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(vec![
            X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
                X690Value::Primitive(Bytes::from(vec![0x55, 0x04, 0x03])),
            ),
            X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NULL),
                X690Value::Primitive(Bytes::from(vec![])),
            ),
        ])),
    );
    decode_AlgorithmIdentifier3(&root).unwrap();
    /* We don't check if the parameters has a value of NULL. In sets, any
    element that is not specified as having a specific tag and class number are
    treated as unrecognized. */
}

fn criterion_benchmark(c: &mut Criterion) {
    /* Finding: the second method--iterating rather than creating a HashMap--is
    about 40% faster. */
    c.bench_function("decode_AlgorithmIdentifier1", |b| b.iter(|| decode_algorithm_identifier1()));
    c.bench_function("decode_AlgorithmIdentifier2", |b| b.iter(|| decode_algorithm_identifier2()));
    c.bench_function("decode_AlgorithmIdentifier3", |b| b.iter(|| decode_algorithm_identifier3()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
