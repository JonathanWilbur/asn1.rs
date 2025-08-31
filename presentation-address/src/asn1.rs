//! Basic Encoding Rules (BER) functions for encoding, decoding, and validating `PresentationAddress`
#![allow(non_upper_case_globals)]
use crate::PresentationAddress;
use std::sync::Arc;
use wildboar_asn1::*;
use x690::*;

impl TryFrom<&X690Element> for PresentationAddress {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PresentationAddress(el)
    }
}

/// Root Component Type Lists 1 components for `PresentationAddress`
pub const _rctl1_components_for_PresentationAddress: &[ComponentSpec; 4] = &[
    ComponentSpec::opt("pSelector", TagSelector::tag((TagClass::CONTEXT, 0))),
    ComponentSpec::opt("pSelector", TagSelector::tag((TagClass::CONTEXT, 1))),
    ComponentSpec::opt("pSelector", TagSelector::tag((TagClass::CONTEXT, 2))),
    ComponentSpec::req("nAddresses", TagSelector::tag((TagClass::CONTEXT, 3))),
];

/// Root Component Type Lists 2 components for `PresentationAddress`
pub const _rctl2_components_for_PresentationAddress: &[ComponentSpec; 0] = &[];

/// Extension Additions List components for `PresentationAddress`
pub const _eal_components_for_PresentationAddress: &[ComponentSpec; 0] = &[];

/// Decode a `PresentationAddress` from BER / DER encoding
pub fn _decode_PresentationAddress(el: &X690Element) -> ASN1Result<PresentationAddress> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PresentationAddress")
            );
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PresentationAddress,
        _eal_components_for_PresentationAddress,
        _rctl2_components_for_PresentationAddress,
    );
    let mut _i: usize = 0;
    let mut pSelector_: OPTIONAL<OCTET_STRING> = None;
    let mut sSelector_: OPTIONAL<OCTET_STRING> = None;
    let mut tSelector_: OPTIONAL<OCTET_STRING> = None;
    let mut nAddresses_: OPTIONAL<Vec<OCTET_STRING>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pSelector" => pSelector_ = Some(BER.decode_octet_string(&_el.inner()?)?),
            "sSelector" => sSelector_ = Some(BER.decode_octet_string(&_el.inner()?)?),
            "tSelector" => tSelector_ = Some(BER.decode_octet_string(&_el.inner()?)?),
            "nAddresses" => {
                nAddresses_ = {
                    let elements = match _el.inner()?.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "nAddresses",
                            ));
                        }
                    };
                    let mut items: SET_OF<OCTET_STRING> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(BER.decode_octet_string(el)?);
                    }
                    Some(items)
                }
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PresentationAddress {
        pSelector: pSelector_,
        sSelector: sSelector_,
        tSelector: tSelector_,
        nAddresses: nAddresses_.unwrap(),
        _unrecognized,
    })
}

/// Encode a `PresentationAddress` into BER / DER encoding
pub fn _encode_PresentationAddress(value_: &PresentationAddress) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(4);
    if let Some(v_) = &value_.pSelector {
        components_.push(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(BER.encode_octet_string(&v_)?),
        ));
    }
    if let Some(v_) = &value_.sSelector {
        components_.push(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(BER.encode_octet_string(&v_)?),
        ));
    }
    if let Some(v_) = &value_.tSelector {
        components_.push(X690Element::new(
            Tag::new(TagClass::CONTEXT, 2),
            X690Value::from_explicit(BER.encode_octet_string(&v_)?),
        ));
    }
    let mut encoded_naddresses: Vec<X690Element> = Vec::with_capacity(value_.nAddresses.len());
    for naddr in &value_.nAddresses {
        encoded_naddresses.push(BER.encode_octet_string(naddr.as_slice())?);
    }
    components_.push(X690Element::new(
        Tag::new(TagClass::CONTEXT, 3),
        X690Value::from_explicit(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(encoded_naddresses)),
        )),
    ));
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

/// Validate (without decoding) a `PresentationAddress`'s BER / DER encoding
pub fn _validate_PresentationAddress(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PresentationAddress")
            );
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PresentationAddress,
        _eal_components_for_PresentationAddress,
        _rctl2_components_for_PresentationAddress,
    );
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pSelector" => BER.validate_octet_string(&_el.inner()?)?,
            "sSelector" => BER.validate_octet_string(&_el.inner()?)?,
            "tSelector" => BER.validate_octet_string(&_el.inner()?)?,
            "nAddresses" => {
                if _el.tag.tag_class != TagClass::CONTEXT || _el.tag.tag_number != 3 {
                    return Err(
                        _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nAddresses")
                    );
                }
                match el.inner()?.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_octet_string(&sub)?;
                        }
                    }
                    _ => {
                        return Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nAddresses")
                        );
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
