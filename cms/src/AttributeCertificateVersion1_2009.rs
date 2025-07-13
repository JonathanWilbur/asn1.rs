#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # AttributeCertificateVersion1-2009
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `AttributeCertificateVersion1-2009`.
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
use std::sync::Arc;
use wildboar_asn1::*;
use x690::*;
use x500::AuthenticationFramework::*;
use x500::AttributeCertificateDefinitions::*;
use x500::CertificateExtensions::*;
use x500::SelectedAttributeTypes::{
    UniqueIdentifier,
    _encode_UniqueIdentifier,
    _decode_UniqueIdentifier,
    _validate_UniqueIdentifier,
};
use x500::InformationFramework::{
    ATTRIBUTE,
    Attribute,
    _decode_Attribute,
    _encode_Attribute,
    _validate_Attribute,
};

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificateV1  ::=  SIGNED{AttributeCertificateInfoV1}
/// ```
pub type AttributeCertificateV1 = SIGNED<AttributeCertificateInfoV1>; // DefinedType

pub fn _decode_AttributeCertificateV1 (el: &X690Element) -> ASN1Result<AttributeCertificateV1> {
	_decode_SIGNED::<AttributeCertificateInfoV1>(_decode_AttributeCertificateInfoV1, el)
}

pub fn _encode_AttributeCertificateV1 (value_: &AttributeCertificateV1) -> ASN1Result<X690Element> {
	_encode_SIGNED::<AttributeCertificateInfoV1>(_encode_AttributeCertificateInfoV1, value_)
}

pub fn _validate_AttributeCertificateV1 (el: &X690Element) -> ASN1Result<()> {
	_validate_SIGNED::<AttributeCertificateInfoV1>(_validate_AttributeCertificateInfoV1, el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificateInfoV1 ::= SEQUENCE {
/// version	AttCertVersionV1 DEFAULT v1,
/// subject	CHOICE {
/// 	baseCertificateID	[0] IssuerSerial,
/// 	-- associated with a Public Key Certificate
/// 	subjectName 		[1] GeneralNames },
/// -- associated with a name
/// issuer 			GeneralNames,
/// signature			AlgorithmIdentifier{SIGNATURE-ALGORITHM, {...}},
/// serialNumber 		CertificateSerialNumber,
/// attCertValidityPeriod 	AttCertValidityPeriod,
/// attributes 			SEQUENCE OF AttributeSet{{AttrList}},
/// issuerUniqueID 		UniqueIdentifier OPTIONAL,
/// extensions 			Extensions{{AttributeCertExtensionsV1}} OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct AttributeCertificateInfoV1 {
	pub version: OPTIONAL<AttCertVersionV1>,
	pub subject: AttributeCertificateInfoV1_subject,
	pub issuer: GeneralNames,
	pub signature: AlgorithmIdentifier,
	pub serialNumber: CertificateSerialNumber,
	pub attCertValidityPeriod: AttCertValidityPeriod,
	pub attributes: Vec<Attribute>,
	pub issuerUniqueID: OPTIONAL<UniqueIdentifier>,
	pub extensions: OPTIONAL<Extensions>
}
impl  AttributeCertificateInfoV1 {
	pub fn new (
		version: OPTIONAL<AttCertVersionV1>,
		subject: AttributeCertificateInfoV1_subject,
		issuer: GeneralNames,
		signature: AlgorithmIdentifier,
		serialNumber: CertificateSerialNumber,
		attCertValidityPeriod: AttCertValidityPeriod,
		attributes: Vec<Attribute>,
		issuerUniqueID: OPTIONAL<UniqueIdentifier>,
		extensions: OPTIONAL<Extensions>
	) -> Self {
		AttributeCertificateInfoV1 { version, subject, issuer, signature, serialNumber, attCertValidityPeriod, attributes, issuerUniqueID, extensions }
	}
	pub fn _default_value_for_version () -> AttCertVersionV1 { AttCertVersionV1_v1 }
}
impl TryFrom<&X690Element> for AttributeCertificateInfoV1 {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_AttributeCertificateInfoV1(el)
	}
}

pub const _rctl1_components_for_AttributeCertificateInfoV1: &[ComponentSpec; 9] = &[
	ComponentSpec::new("version", true, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("subject", false, TagSelector::any, None, None),
	ComponentSpec::new("issuer", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None),
	ComponentSpec::new("signature", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None),
	ComponentSpec::new("serialNumber", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("attCertValidityPeriod", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None),
	ComponentSpec::new("attributes", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None),
	ComponentSpec::new("issuerUniqueID", true, TagSelector::tag((TagClass::UNIVERSAL, 3)), None, None),
	ComponentSpec::new("extensions", true, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_AttributeCertificateInfoV1: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_AttributeCertificateInfoV1: &[ComponentSpec; 0] = &[

];

pub fn _decode_AttributeCertificateInfoV1 (el: &X690Element) -> ASN1Result<AttributeCertificateInfoV1> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeCertificateInfoV1")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_AttributeCertificateInfoV1,
		_eal_components_for_AttributeCertificateInfoV1,
		_rctl2_components_for_AttributeCertificateInfoV1,
	).into_iter();
	let mut _i: usize = 0;
	let mut version_: OPTIONAL<AttCertVersionV1> = None;
	let mut subject_: OPTIONAL<AttributeCertificateInfoV1_subject> = None;
	let mut issuer_: OPTIONAL<GeneralNames> = None;
	let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
	let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
	let mut attCertValidityPeriod_: OPTIONAL<AttCertValidityPeriod> = None;
	let mut attributes_: OPTIONAL<Vec<Attribute>> = None;
	let mut issuerUniqueID_: OPTIONAL<UniqueIdentifier> = None;
	let mut extensions_: OPTIONAL<Extensions> = None;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"version" => version_ = Some(_decode_AttCertVersionV1(_el)?),
			"subject" => subject_ = Some(_decode_AttributeCertificateInfoV1_subject(_el)?),
			"issuer" => issuer_ = Some(_decode_GeneralNames(_el)?),
			"signature" => signature_ = Some(_decode_AlgorithmIdentifier(_el)?),
			"serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
			"attCertValidityPeriod" => attCertValidityPeriod_ = Some(_decode_AttCertValidityPeriod(_el)?),
			"attributes" => attributes_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributes")),
	};
	let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_Attribute(el)?);
	}
	Ok(items)
}(_el)?),
			"issuerUniqueID" => issuerUniqueID_ = Some(_decode_UniqueIdentifier(_el)?),
			"extensions" => extensions_ = Some(_decode_Extensions(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeCertificateInfoV1")),
		}
	}
	Ok(AttributeCertificateInfoV1{ version: version_, subject: subject_.unwrap(), issuer: issuer_.unwrap(), signature: signature_.unwrap(), serialNumber: serialNumber_.unwrap(), attCertValidityPeriod: attCertValidityPeriod_.unwrap(), attributes: attributes_.unwrap(), issuerUniqueID: issuerUniqueID_, extensions: extensions_ })

}

pub fn _encode_AttributeCertificateInfoV1 (value_: &AttributeCertificateInfoV1) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(14);
	if let Some(v_) = &value_.version {
		if *v_ != AttributeCertificateInfoV1::_default_value_for_version() {
			components_.push(_encode_AttCertVersionV1(&v_)?);
		}
	}
	components_.push(_encode_AttributeCertificateInfoV1_subject(&value_.subject)?);
	components_.push(_encode_GeneralNames(&value_.issuer)?);
	components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
	components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
	components_.push(_encode_AttCertValidityPeriod(&value_.attCertValidityPeriod)?);
	components_.push(|value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_Attribute(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&value_.attributes)?);
	if let Some(v_) = &value_.issuerUniqueID {
		components_.push(_encode_UniqueIdentifier(&v_)?);
	}
	if let Some(v_) = &value_.extensions {
		components_.push(_encode_Extensions(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new(components_)),
	))

}

pub fn _validate_AttributeCertificateInfoV1 (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeCertificateInfoV1")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_AttributeCertificateInfoV1,
		_eal_components_for_AttributeCertificateInfoV1,
		_rctl2_components_for_AttributeCertificateInfoV1,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"version" => _validate_AttCertVersionV1(_el)?,
			"subject" => _validate_AttributeCertificateInfoV1_subject(_el)?,
			"issuer" => _validate_GeneralNames(_el)?,
			"signature" => _validate_AlgorithmIdentifier(_el)?,
			"serialNumber" => _validate_CertificateSerialNumber(_el)?,
			"attCertValidityPeriod" => _validate_AttCertValidityPeriod(_el)?,
			"attributes" => |el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_Attribute(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributes")),
	}
}(_el)?,
			"issuerUniqueID" => _validate_UniqueIdentifier(_el)?,
			"extensions" => _validate_Extensions(_el)?,
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeCertificateInfoV1")),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertVersionV1  ::=  INTEGER { v1(0) }
/// ```
pub type AttCertVersionV1 = i8;

pub const AttCertVersionV1_v1: AttCertVersionV1 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AttCertVersionV1 (el: &X690Element) -> ASN1Result<AttCertVersionV1> {
	BER.decode_i8(el)
}

pub fn _encode_AttCertVersionV1 (value_: &AttCertVersionV1) -> ASN1Result<X690Element> {
	BER.encode_i8(*value_)
}

pub fn _validate_AttCertVersionV1 (el: &X690Element) -> ASN1Result<()> {
	BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttrList ATTRIBUTE ::= {...}
/// ```
///
///
pub fn AttrList () -> Vec<ATTRIBUTE> {
	Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertExtensionsV1 EXTENSION ::= {...}
/// ```
///
///
pub fn AttributeCertExtensionsV1 () -> Vec<EXTENSION> {
	Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificateInfoV1-subject ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum AttributeCertificateInfoV1_subject {
	baseCertificateID(IssuerSerial),
	subjectName(GeneralNames),
}

impl TryFrom<&X690Element> for AttributeCertificateInfoV1_subject {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_AttributeCertificateInfoV1_subject(el)
	}
}

pub fn _decode_AttributeCertificateInfoV1_subject (el: &X690Element) -> ASN1Result<AttributeCertificateInfoV1_subject> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(AttributeCertificateInfoV1_subject::baseCertificateID(|el: &X690Element| -> ASN1Result<IssuerSerial> {
	Ok(_decode_IssuerSerial(&el.inner()?)?)
}(&el)?)),
		(TagClass::CONTEXT, 1) => Ok(AttributeCertificateInfoV1_subject::subjectName(|el: &X690Element| -> ASN1Result<GeneralNames> {
	Ok(_decode_GeneralNames(&el.inner()?)?)
}(&el)?)),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "AttributeCertificateInfoV1-subject")),
	}
}

pub fn _encode_AttributeCertificateInfoV1_subject (value_: &AttributeCertificateInfoV1_subject) -> ASN1Result<X690Element> {
		match value_ {
		AttributeCertificateInfoV1_subject::baseCertificateID(v) => |v_1: &IssuerSerial| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 0), X690Value::from_explicit(_encode_IssuerSerial(&v_1)?))) }(&v),
		AttributeCertificateInfoV1_subject::subjectName(v) => |v_1: &GeneralNames| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 1), X690Value::from_explicit(_encode_GeneralNames(&v_1)?))) }(&v),
	}
}

pub fn _validate_AttributeCertificateInfoV1_subject (el: &X690Element) -> ASN1Result<()> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "baseCertificateID"));
	}
	Ok(_validate_IssuerSerial(&el.inner()?)?)
}(&el),
		(TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subjectName"));
	}
	Ok(_validate_GeneralNames(&el.inner()?)?)
}(&el),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "AttributeCertificateInfoV1-subject")),
	}
}

