
use asn1::types::{
    ASN1Value, TagClass, ASN1_UNIVERSAL_TAG_NUMBER_NULL,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE, OBJECT_IDENTIFIER,
};
use std::sync::Arc;
use x690::{X690Element, X690Encoding};
use asn1::construction::{ComponentSpec, TagSelector};
use asn1::error::ASN1Result;
use std::borrow::Borrow;
use x690::{
    ber_decode_any, ber_decode_object_identifier,
    _parse_sequence, X690StructureIterator,
};
use criterion::{criterion_group, criterion_main, Criterion};

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

fn decode_AlgorithmIdentifier2(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
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
            "algorithm" => maybe_algorithm = Some(ber_decode_object_identifier(el)?),
            "parameters" => maybe_parameters = Some(ber_decode_any(el)?),
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

fn decode_algorithm_identifier1() {
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
            .0
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

fn decode_algorithm_identifier2() {
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
    let alg_id = decode_AlgorithmIdentifier2(&root).unwrap();
    assert_eq!(
        alg_id
            .algorithm
            .0
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

fn criterion_benchmark(c: &mut Criterion) {
    /* Finding: the second method--iterating rather than creating a HashMap--is
    about 40% faster. */
    c.bench_function("decode_AlgorithmIdentifier1", |b| b.iter(|| decode_algorithm_identifier1()));
    c.bench_function("decode_AlgorithmIdentifier2", |b| b.iter(|| decode_algorithm_identifier2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
