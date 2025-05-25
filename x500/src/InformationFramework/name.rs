#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
use crate::InformationFramework::{
    RDNSequence, _decode_RDNSequence, _encode_RDNSequence, _validate_RDNSequence,
    AttributeTypeAndValue,
};
use crate::SelectedAttributeTypes::{dc, oidC};
use wildboar_asn1::*;
use x690::*;
use x690::X690Codec;

/// ### ASN.1 Definition:
///
/// ```asn1
/// Name  ::=  CHOICE {
///   rdnSequence   RDNSequence,
///   dnsName       DomainName,
///   oid           OBJECT IDENTIFIER }
/// ```
#[derive(Debug, Clone)]
pub enum Name {
    rdnSequence(RDNSequence),
    dnsName(DomainName),
    oid(OBJECT_IDENTIFIER),
}

impl TryFrom<&X690Element> for Name {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Name(el)
    }
}

pub fn _decode_Name(el: &X690Element) -> ASN1Result<Name> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(Name::rdnSequence(_decode_RDNSequence(&el)?)),
        (TagClass::UNIVERSAL, 12) => Ok(Name::dnsName(_decode_DomainName(&el)?)),
        (TagClass::UNIVERSAL, 6) => Ok(Name::oid(BER.decode_object_identifier(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Name",
            ))
        }
    }
}

pub fn _encode_Name(value_: &Name) -> ASN1Result<X690Element> {
    match value_ {
        Name::rdnSequence(v) => _encode_RDNSequence(&v),
        Name::dnsName(v) => _encode_DomainName(&v),
        Name::oid(v) => BER.encode_object_identifier(&v),
    }
}

pub fn _validate_Name(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_RDNSequence(&el),
        (TagClass::UNIVERSAL, 12) => _validate_DomainName(&el),
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Name",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DomainName  ::=  UTF8String (CONSTRAINED BY {
/// -- Conforms to the format of an (internationalized) domain name. -- })
/// ```
pub type DomainName = UTF8String; // UTF8String

pub fn _decode_DomainName(el: &X690Element) -> ASN1Result<DomainName> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_DomainName(value_: &DomainName) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_DomainName(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

impl TryFrom<Name> for RDNSequence {
    type Error = ASN1Error;

    fn try_from(value: Name) -> Result<Self, Self::Error> {
        match value {
            Name::rdnSequence(rdns) => Ok(rdns),
            Name::dnsName(fqdn) => {
                let number_of_labels = fqdn.matches(".").count();
                let mut rdns: RDNSequence = Vec::with_capacity(number_of_labels);
                for label in fqdn.split(".") {
                    if label.chars().all(|c| c.is_ascii()) {
                        rdns.push(
                            vec![
                                AttributeTypeAndValue::new(
                                    dc().id,
                                    BER.encode_ia5_string(label).unwrap(),
                                    vec![],
                                )
                            ]
                        );
                    }
                    if label.starts_with("xn--") {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                    let punycoded_label = punycode::encode(label)
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
                    let new_label = ["xn--",&punycoded_label].concat();
                    rdns.push(
                        vec![
                            AttributeTypeAndValue::new(
                                dc().id,
                                BER.encode_ia5_string(&new_label).unwrap(),
                                vec![],
                            )
                        ]
                    );
                }
                Ok(rdns)
            },
            Name::oid(oid) =>  {
                let mut rdns: RDNSequence = Vec::with_capacity(oid.len());
                for arc in oid.arcs() {
                    rdns.push(vec![
                        AttributeTypeAndValue::new(
                            oidC().id,
                            BER.encode_u128(arc).unwrap(),
                            vec![],
                        )
                    ]);
                }
                Ok(rdns)
            },
        }
    }

}

impl From<RDNSequence> for Name {

    fn from(value: RDNSequence) -> Self {
        Name::rdnSequence(value)
    }

}
