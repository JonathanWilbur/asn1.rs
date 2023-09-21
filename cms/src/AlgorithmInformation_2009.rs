#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # AlgorithmInformation-2009
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `AlgorithmInformation-2009`.
//!
//! This compilation was produced by the
//! [Wildboar Software](https://wildboarsoftware.com/en)
//! [ASN.1 Compiler](https://wildboarsoftware.com/en/asn1-compilation).
//!
//! Types from the source ASN.1 module are generally available by their original
//! names, but with hyphens replaced by underscores. Encoders and decoders for
//! any given type are available as `_encode_TYPENAME()` and
//! `_decode_TYPENAME()`. Decoders are also available as implementations of
//! the `From<X690Element` and `From<&'a X690Element>` traits for some
//! types.
//!
use asn1::*;
use std::sync::Arc;
use x500::CertificateExtensions::KeyUsage;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ParamOptions  ::=  ENUMERATED {
/// required,         -- Parameters MUST be encoded in structure
/// preferredPresent, -- Parameters SHOULD be encoded in structure
/// preferredAbsent,  -- Parameters SHOULD NOT be encoded in structure
/// absent,           -- Parameters MUST NOT be encoded in structure
/// inheritable,      -- Parameters are inherited if not present
/// optional,         -- Parameters MAY be encoded in the structure
/// ...
/// }
/// ```
pub type ParamOptions = ENUMERATED;

pub const ParamOptions_required: ParamOptions = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ParamOptions_preferredPresent: ParamOptions = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ParamOptions_preferredAbsent: ParamOptions = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ParamOptions_absent: ParamOptions = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ParamOptions_inheritable: ParamOptions = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ParamOptions_optional: ParamOptions = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ParamOptions(el: &X690Element) -> ASN1Result<ParamOptions> {
    BER.decode_enumerated(&el)
}

pub fn _encode_ParamOptions(value_: &ParamOptions) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_ParamOptions(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DIGEST-ALGORITHM ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence ]
/// }
/// ```
///
#[derive(Debug)]
pub struct DIGEST_ALGORITHM {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
}
impl DIGEST_ALGORITHM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SIGNATURE-ALGORITHM ::= CLASS {
/// &id             OBJECT IDENTIFIER UNIQUE,
/// &Value          OPTIONAL,
/// &Params         OPTIONAL,
/// &paramPresence  ParamOptions DEFAULT absent,
/// &HashSet        DIGEST-ALGORITHM OPTIONAL,
/// &PublicKeySet   PUBLIC-KEY OPTIONAL,
/// &smimeCaps      SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [VALUE &Value]
/// [PARAMS [TYPE &Params] ARE &paramPresence ]
/// [HASHES &HashSet]
/// [PUBLIC-KEYS &PublicKeySet]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct SIGNATURE_ALGORITHM {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub HashSet: OPTIONAL<Vec<DIGEST_ALGORITHM>>,
    pub PublicKeySet: OPTIONAL<Vec<PUBLIC_KEY>>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl SIGNATURE_ALGORITHM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PUBLIC-KEY ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &KeyValue		OPTIONAL,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &keyUsage		KeyUsage OPTIONAL,
/// &PrivateKey		OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [KEY &KeyValue]
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [CERT-KEY-USAGE &keyUsage]
/// [PRIVATE-KEY &PrivateKey]
/// }
/// ```
///
#[derive(Debug)]
pub struct PUBLIC_KEY {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub keyUsage: OPTIONAL<KeyUsage>,
}
impl PUBLIC_KEY {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEY-TRANSPORT ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &PublicKeySet	PUBLIC-KEY OPTIONAL,
/// &smimeCaps		SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [PUBLIC-KEYS &PublicKeySet]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct KEY_TRANSPORT {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub PublicKeySet: OPTIONAL<Vec<PUBLIC_KEY>>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl KEY_TRANSPORT {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEY-AGREE ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &PublicKeySet	PUBLIC-KEY OPTIONAL,
/// &Ukm			OPTIONAL,
/// &ukmPresence	ParamOptions DEFAULT absent,
/// &smimeCaps		SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [PUBLIC-KEYS &PublicKeySet]
/// [UKM [TYPE &Ukm] ARE &ukmPresence]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct KEY_AGREE {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub PublicKeySet: OPTIONAL<Vec<PUBLIC_KEY>>,
    pub ukmPresence: OPTIONAL<ParamOptions>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl KEY_AGREE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEY-WRAP ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &smimeCaps		SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct KEY_WRAP {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl KEY_WRAP {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KEY-DERIVATION ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &smimeCaps		SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct KEY_DERIVATION {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl KEY_DERIVATION {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MAC-ALGORITHM ::= CLASS {
/// &id				OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &keyed		BOOLEAN,
/// &smimeCaps		SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// IS-KEYED-MAC &keyed
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct MAC_ALGORITHM {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub keyed: BOOLEAN,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl MAC_ALGORITHM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CONTENT-ENCRYPTION ::= CLASS {
/// &id                OBJECT IDENTIFIER UNIQUE,
/// &Params            OPTIONAL,
/// &paramPresence     ParamOptions DEFAULT absent,
/// &smimeCaps         SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct CONTENT_ENCRYPTION {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl CONTENT_ENCRYPTION {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ALGORITHM ::= CLASS {
/// &id		OBJECT IDENTIFIER UNIQUE,
/// &Params		OPTIONAL,
/// &paramPresence	ParamOptions DEFAULT absent,
/// &smimeCaps	SMIME-CAPS OPTIONAL
/// } WITH SYNTAX {
/// IDENTIFIER &id
/// [PARAMS [TYPE &Params] ARE &paramPresence]
/// [SMIME-CAPS &smimeCaps]
/// }
/// ```
///
#[derive(Debug)]
pub struct ALGORITHM {
    pub id: OBJECT_IDENTIFIER,
    pub paramPresence: OPTIONAL<ParamOptions>,
    pub smimeCaps: OPTIONAL<SMIME_CAPS>,
}
impl ALGORITHM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMIME-CAPS ::= CLASS {
/// &id			OBJECT IDENTIFIER UNIQUE,
/// &Type		OPTIONAL
/// }
/// WITH SYNTAX { [TYPE &Type] IDENTIFIED BY &id }
/// ```
///
#[derive(Debug, Clone)]
pub struct SMIME_CAPS {
    pub id: OBJECT_IDENTIFIER,
}
impl SMIME_CAPS {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMIMECapability{SMIME-CAPS:CapabilitySet} ::= SEQUENCE {
/// capabilityID	SMIME-CAPS.&id({CapabilitySet}),
/// parameters		SMIME-CAPS.&Type({CapabilitySet}{@capabilityID})
/// 				OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct SMIMECapability {
    pub capabilityID: OBJECT_IDENTIFIER,
    pub parameters: OPTIONAL<X690Element>,
}
impl SMIMECapability {
    pub fn new(capabilityID: OBJECT_IDENTIFIER, parameters: OPTIONAL<X690Element>) -> Self {
        SMIMECapability {
            capabilityID,
            parameters,
        }
    }
}
impl TryFrom<&X690Element> for SMIMECapability {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SMIMECapability(el)
    }
}

pub const _rctl1_components_for_SMIMECapability: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "capabilityID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("parameters", true, TagSelector::any, None, None),
];

pub const _rctl2_components_for_SMIMECapability: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SMIMECapability: &[ComponentSpec; 0] = &[];

pub fn _decode_SMIMECapability(el: &X690Element) -> ASN1Result<SMIMECapability> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SMIMECapability"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SMIMECapability,
        _eal_components_for_SMIMECapability,
        _rctl2_components_for_SMIMECapability,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut capabilityID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parameters_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "capabilityID" => capabilityID_ = Some(BER.decode_object_identifier(_el)?),
            "parameters" => parameters_ = Some(x690_identity(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SMIMECapability")
                )
            }
        }
    }
    Ok(SMIMECapability {
        capabilityID: capabilityID_.unwrap(),
        parameters: parameters_,
    })
}

pub fn _encode_SMIMECapability(value_: &SMIMECapability) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.capabilityID)?);
    if let Some(v_) = &value_.parameters {
        components_.push(x690_identity(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SMIMECapability(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SMIMECapability"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SMIMECapability,
        _eal_components_for_SMIMECapability,
        _rctl2_components_for_SMIMECapability,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "capabilityID" => BER.validate_object_identifier(_el)?,
            "parameters" => BER.validate_any(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SMIMECapability")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SMIMECapabilities { SMIME-CAPS:CapabilitySet }  ::=
/// SEQUENCE SIZE (1..MAX) OF SMIMECapability{{CapabilitySet} }
/// ```
pub type SMIMECapabilities = Vec<SMIMECapability>; // SequenceOfType

pub fn _decode_SMIMECapabilities(el: &X690Element) -> ASN1Result<SMIMECapabilities> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SMIMECapabilities")
            )
        }
    };
    let mut items: SEQUENCE_OF<SMIMECapability> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_SMIMECapability(el)?);
    }
    Ok(items)
}

pub fn _encode_SMIMECapabilities(value_: &SMIMECapabilities) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_SMIMECapability(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SMIMECapabilities(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_SMIMECapability(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SMIMECapabilities")),
    }
}
