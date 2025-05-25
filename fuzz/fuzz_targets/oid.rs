#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;

fuzz_target!(|data: &[u8]| {
    let _ = wildboar_asn1::OBJECT_IDENTIFIER::validate_x690_encoding(data);
    let oid1 = unsafe {
        wildboar_asn1::OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked(data)
    };
    for arc in oid1.arcs() {}
    let _ = oid1.to_asn1_string();
    let _ = oid1.to_iri_string();
    let _ = oid1.to_string();
    let _ = oid1.to_dot_delim_string();

    let oid2 = wildboar_asn1::OBJECT_IDENTIFIER::from_prefix_and_arc(oid1.clone(), 1).unwrap();
    for arc in oid2.arcs() {}
    let _ = oid2.to_asn1_string();
    let _ = oid2.to_iri_string();
    let _ = oid2.to_string();
    let _ = oid2.to_dot_delim_string();
    assert!(oid2.starts_with(&oid1));

    let roid = wildboar_asn1::RELATIVE_OID::try_from([ 1u32, 5, 7 ].as_slice()).unwrap();
    let oid3 = wildboar_asn1::OBJECT_IDENTIFIER::from_prefix_and_suffix(oid2.clone(), [ 5u32, 7 ].as_slice()).unwrap();
    for arc in oid3.arcs() {}
    let _ = oid3.to_asn1_string();
    let _ = oid3.to_iri_string();
    let _ = oid3.to_string();
    let _ = oid3.to_dot_delim_string();
    assert!(oid3.starts_with(&oid2));
});
