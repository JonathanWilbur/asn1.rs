use base64::prelude::*;
use pki_stub::*;
use rand::RngCore;
use std::str::FromStr;
use wildboar_asn1::*;
use x690::der::DER;
use x690::*;

fn get_alg_id() -> AlgorithmIdentifier {
    AlgorithmIdentifier::new(
        oid!(1, 2, 840, 113549, 1, 1, 1), // sha1WithRSAEncryption
        Some(X690Element::null()),
        vec![],
    )
}

// Yeah, for some reason, `rand` doesn't have this capability...
fn get_random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0; len];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

fn get_random_serial() -> Vec<u8> {
    let mut bytes = get_random_bytes(12);
    bytes[0] &= 0x7f; // So it is always positive.
    bytes
}

fn main() {
    let tbs = TBSAttributeCertificate::new(
        Version_v3,
        Holder::new(
            Some(IssuerSerial::new(
                vec![], // TODO:
                get_random_serial(),
                None,
                vec![],
            )),
            None,
            None,
        ),
        AttCertIssuer::new(
            None,
            Some(IssuerSerial::new(
                vec![], // TODO:
                get_random_serial(),
                None,
                vec![],
            )),
            None,
            vec![],
        ),
        get_alg_id(),
        get_random_serial(),
        AttCertValidityPeriod::new(
            GeneralizedTime::from_str("20250803000000Z").unwrap(),
            GeneralizedTime::from_str("20250803000000Z").unwrap(),
            vec![],
        ),
        vec![Attribute::new(
            oid!(2, 5, 4, 3),
            vec![DER.encode_utf8_string("hi mom").unwrap()],
            None,
            vec![],
        )],
        None,
        vec![],
        None,
    );

    let attr_cert: AttributeCertificate = AttributeCertificate::new(
        tbs,
        get_alg_id(),
        BIT_STRING::from_bytes(get_random_bytes(128)),
        None,
        None,
        vec![],
    );

    let el = _encode_AttributeCertificate(&attr_cert).unwrap();
    let mut buf = Vec::new();
    let bytes_written = x690_write_tlv(&mut buf, &el).unwrap();

    println!("{}", hex::encode(&buf[..bytes_written]));

    let pemcert = pem::Pem::new("ATTRIBUTE CERTIFICATE", &buf[..bytes_written]);
    let pemstr = pem::encode(&pemcert);
    println!("{} bytes: {}", bytes_written, pemstr);
}
