#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # Lightweight-Directory-Access-Protocol-V3
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `Lightweight-Directory-Access-Protocol-V3`.
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
use asn1::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPMessage ::= SEQUENCE {
///   messageID   MessageID,
///   protocolOp
///     CHOICE {bindRequest           BindRequest,
///             bindResponse          BindResponse,
///             unbindRequest         UnbindRequest,
///             searchRequest         SearchRequest,
///             searchResEntry        SearchResultEntry,
///             searchResDone         SearchResultDone,
///             searchResRef          SearchResultReference,
///             modifyRequest         ModifyRequest,
///             modifyResponse        ModifyResponse,
///             addRequest            AddRequest,
///             addResponse           AddResponse,
///             delRequest            DelRequest,
///             delResponse           DelResponse,
///             modDNRequest          ModifyDNRequest,
///             modDNResponse         ModifyDNResponse,
///             compareRequest        CompareRequest,
///             compareResponse       CompareResponse,
///             abandonRequest        AbandonRequest,
///             extendedReq           ExtendedRequest,
///             extendedResp          ExtendedResponse,
///             ...,
///             intermediateResponse  IntermediateResponse},
///   controls    [0]  Controls OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LDAPMessage {
		pub messageID: MessageID,
		pub protocolOp: LDAPMessage_protocolOp,
		pub controls: OPTIONAL<Controls>,
		pub _unrecognized: Vec<X690Element>
}
impl  LDAPMessage {
	pub fn new (
		messageID: MessageID,
		protocolOp: LDAPMessage_protocolOp,
		controls: OPTIONAL<Controls>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		LDAPMessage { messageID, protocolOp, controls, _unrecognized}
	}

}
impl TryFrom<&X690Element> for LDAPMessage {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_LDAPMessage(el)
	}
}

pub const _rctl1_components_for_LDAPMessage: &[ComponentSpec; 3] = &[
	ComponentSpec::new("messageID", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("protocolOp", false, TagSelector::any, None, None),
	ComponentSpec::new("controls", true, TagSelector::tag((TagClass::CONTEXT, 0)), None, None)
];

pub const _rctl2_components_for_LDAPMessage: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_LDAPMessage: &[ComponentSpec; 0] = &[

];

pub fn _decode_LDAPMessage (el: &X690Element) -> ASN1Result<LDAPMessage> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LDAPMessage")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_LDAPMessage,
		_eal_components_for_LDAPMessage,
		_rctl2_components_for_LDAPMessage,
	).into_iter();
	let mut _i: usize = 0;
	let mut messageID: OPTIONAL<MessageID> = None;
	let mut protocolOp: OPTIONAL<LDAPMessage_protocolOp> = None;
	let mut controls: OPTIONAL<Controls> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"messageID" => messageID = Some(_decode_MessageID(_el)?),
			"protocolOp" => protocolOp = Some(_decode_LDAPMessage_protocolOp(_el)?),
			"controls" => controls = Some(_decode_Controls(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(LDAPMessage{ messageID: messageID.unwrap(), protocolOp: protocolOp.unwrap(), controls, _unrecognized })

}

pub fn _encode_LDAPMessage (value_: &LDAPMessage) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(13);
	components_.push(_encode_MessageID(&value_.messageID)?);
	components_.push(_encode_LDAPMessage_protocolOp(&value_.protocolOp)?);
	if let Some(v_) = &value_.controls {
		components_.push(|v_1: &Controls| -> ASN1Result<X690Element> { let mut el_1 = _encode_Controls(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_LDAPMessage (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LDAPMessage")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_LDAPMessage,
		_eal_components_for_LDAPMessage,
		_rctl2_components_for_LDAPMessage,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"messageID" => _validate_MessageID(_el)?,
			"protocolOp" => _validate_LDAPMessage_protocolOp(_el)?,
			"controls" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "controls"));
	}
	Ok(_validate_Controls(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageID  ::=  INTEGER(0..maxInt)
/// ```
pub type MessageID = INTEGER;

pub fn _decode_MessageID (el: &X690Element) -> ASN1Result<MessageID> {
	BER.decode_integer(&el)
}

pub fn _encode_MessageID (value_: &MessageID) -> ASN1Result<X690Element> {
	BER.encode_integer(&value_)
}

pub fn _validate_MessageID (el: &X690Element) -> ASN1Result<()> {
	BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// maxInt INTEGER ::= 2147483647
/// ```
///
///
pub const maxInt: i64 = 2147483647;

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPString  ::=  OCTET STRING
/// ```
pub type LDAPString = OCTET_STRING; // OctetStringType

pub fn _decode_LDAPString (el: &X690Element) -> ASN1Result<LDAPString> {
	BER.decode_octet_string(&el)
}

pub fn _encode_LDAPString (value_: &LDAPString) -> ASN1Result<X690Element> {
	BER.encode_octet_string(&value_)
}

pub fn _validate_LDAPString (el: &X690Element) -> ASN1Result<()> {
	BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPOID  ::=  OCTET STRING
/// ```
pub type LDAPOID = OCTET_STRING; // OctetStringType

pub fn _decode_LDAPOID (el: &X690Element) -> ASN1Result<LDAPOID> {
	BER.decode_octet_string(&el)
}

pub fn _encode_LDAPOID (value_: &LDAPOID) -> ASN1Result<X690Element> {
	BER.encode_octet_string(&value_)
}

pub fn _validate_LDAPOID (el: &X690Element) -> ASN1Result<()> {
	BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPDN  ::=  LDAPString
/// ```
pub type LDAPDN = LDAPString; // DefinedType

pub fn _decode_LDAPDN (el: &X690Element) -> ASN1Result<LDAPDN> {
	_decode_LDAPString(&el)
}

pub fn _encode_LDAPDN (value_: &LDAPDN) -> ASN1Result<X690Element> {
	_encode_LDAPString(&value_)
}

pub fn _validate_LDAPDN (el: &X690Element) -> ASN1Result<()> {
	_validate_LDAPString(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RelativeLDAPDN  ::=
///   LDAPString
/// ```
pub type RelativeLDAPDN = LDAPString; // DefinedType

pub fn _decode_RelativeLDAPDN (el: &X690Element) -> ASN1Result<RelativeLDAPDN> {
	_decode_LDAPString(&el)
}

pub fn _encode_RelativeLDAPDN (value_: &RelativeLDAPDN) -> ASN1Result<X690Element> {
	_encode_LDAPString(&value_)
}

pub fn _validate_RelativeLDAPDN (el: &X690Element) -> ASN1Result<()> {
	_validate_LDAPString(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeDescription  ::=  LDAPString
/// ```
pub type AttributeDescription = LDAPString; // DefinedType

pub fn _decode_AttributeDescription (el: &X690Element) -> ASN1Result<AttributeDescription> {
	_decode_LDAPString(&el)
}

pub fn _encode_AttributeDescription (value_: &AttributeDescription) -> ASN1Result<X690Element> {
	_encode_LDAPString(&value_)
}

pub fn _validate_AttributeDescription (el: &X690Element) -> ASN1Result<()> {
	_validate_LDAPString(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValue  ::=  OCTET STRING
/// ```
pub type AttributeValue = OCTET_STRING; // OctetStringType

pub fn _decode_AttributeValue (el: &X690Element) -> ASN1Result<AttributeValue> {
	BER.decode_octet_string(&el)
}

pub fn _encode_AttributeValue (value_: &AttributeValue) -> ASN1Result<X690Element> {
	BER.encode_octet_string(&value_)
}

pub fn _validate_AttributeValue (el: &X690Element) -> ASN1Result<()> {
	BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValueAssertion ::= SEQUENCE {
///   attributeDesc   AttributeDescription,
///   assertionValue  AssertionValue
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeValueAssertion {
		pub attributeDesc: AttributeDescription,
		pub assertionValue: AssertionValue,
		pub _unrecognized: Vec<X690Element>
}
impl  AttributeValueAssertion {
	pub fn new (
		attributeDesc: AttributeDescription,
		assertionValue: AssertionValue,
		_unrecognized: Vec<X690Element>
	) -> Self {
		AttributeValueAssertion { attributeDesc, assertionValue, _unrecognized}
	}

}
impl TryFrom<&X690Element> for AttributeValueAssertion {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_AttributeValueAssertion(el)
	}
}

pub const _rctl1_components_for_AttributeValueAssertion: &[ComponentSpec; 2] = &[
	ComponentSpec::new("attributeDesc", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("assertionValue", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None)
];

pub const _rctl2_components_for_AttributeValueAssertion: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_AttributeValueAssertion: &[ComponentSpec; 0] = &[

];

pub fn _decode_AttributeValueAssertion (el: &X690Element) -> ASN1Result<AttributeValueAssertion> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeValueAssertion")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_AttributeValueAssertion,
		_eal_components_for_AttributeValueAssertion,
		_rctl2_components_for_AttributeValueAssertion,
	).into_iter();
	let mut _i: usize = 0;
	let mut attributeDesc: OPTIONAL<AttributeDescription> = None;
	let mut assertionValue: OPTIONAL<AssertionValue> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"attributeDesc" => attributeDesc = Some(_decode_AttributeDescription(_el)?),
			"assertionValue" => assertionValue = Some(_decode_AssertionValue(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(AttributeValueAssertion{ attributeDesc: attributeDesc.unwrap(), assertionValue: assertionValue.unwrap(), _unrecognized })

}

pub fn _encode_AttributeValueAssertion (value_: &AttributeValueAssertion) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_AttributeDescription(&value_.attributeDesc)?);
	components_.push(_encode_AssertionValue(&value_.assertionValue)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_AttributeValueAssertion (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeValueAssertion")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_AttributeValueAssertion,
		_eal_components_for_AttributeValueAssertion,
		_rctl2_components_for_AttributeValueAssertion,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"attributeDesc" => _validate_AttributeDescription(_el)?,
			"assertionValue" => _validate_AssertionValue(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AssertionValue  ::=  OCTET STRING
/// ```
pub type AssertionValue = OCTET_STRING; // OctetStringType

pub fn _decode_AssertionValue (el: &X690Element) -> ASN1Result<AssertionValue> {
	BER.decode_octet_string(&el)
}

pub fn _encode_AssertionValue (value_: &AssertionValue) -> ASN1Result<X690Element> {
	BER.encode_octet_string(&value_)
}

pub fn _validate_AssertionValue (el: &X690Element) -> ASN1Result<()> {
	BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PartialAttribute ::= SEQUENCE {
///   type  AttributeDescription,
///   vals  SET OF value AttributeValue
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PartialAttribute {
		pub type_: AttributeDescription,
		pub vals: Vec<AttributeValue>,
		pub _unrecognized: Vec<X690Element>
}
impl  PartialAttribute {
	pub fn new (
		type_: AttributeDescription,
		vals: Vec<AttributeValue>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		PartialAttribute { type_, vals, _unrecognized}
	}

}
impl TryFrom<&X690Element> for PartialAttribute {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_PartialAttribute(el)
	}
}

pub const _rctl1_components_for_PartialAttribute: &[ComponentSpec; 2] = &[
	ComponentSpec::new("type", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("vals", false, TagSelector::tag((TagClass::UNIVERSAL, 17)), None, None)
];

pub const _rctl2_components_for_PartialAttribute: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_PartialAttribute: &[ComponentSpec; 0] = &[

];

pub fn _decode_PartialAttribute (el: &X690Element) -> ASN1Result<PartialAttribute> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PartialAttribute")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_PartialAttribute,
		_eal_components_for_PartialAttribute,
		_rctl2_components_for_PartialAttribute,
	).into_iter();
	let mut _i: usize = 0;
	let mut type_: OPTIONAL<AttributeDescription> = None;
	let mut vals: OPTIONAL<Vec<AttributeValue>> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"type" => type_ = Some(_decode_AttributeDescription(_el)?),
			"vals" => vals = Some(|el: &X690Element| -> ASN1Result<SET_OF<AttributeValue>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "vals")),
	};
	let mut items: SET_OF<AttributeValue> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_AttributeValue(el)?);
	}
	Ok(items)
}(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(PartialAttribute{ type_: type_.unwrap(), vals: vals.unwrap(), _unrecognized })

}

pub fn _encode_PartialAttribute (value_: &PartialAttribute) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_AttributeDescription(&value_.type_)?);
	components_.push(|value_: &SET_OF<AttributeValue>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_AttributeValue(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF), X690Value::Constructed(Arc::new(children))))
}(&value_.vals)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_PartialAttribute (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PartialAttribute")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_PartialAttribute,
		_eal_components_for_PartialAttribute,
		_rctl2_components_for_PartialAttribute,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"type" => _validate_AttributeDescription(_el)?,
			"vals" => |el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_AttributeValue(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "vals")),
	}
}(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute  ::=  PartialAttribute(WITH COMPONENTS {
///                                  ...,
///                                  vals  (SIZE (1..MAX))
///                                })
/// ```
pub type Attribute = PartialAttribute; // DefinedType

pub fn _decode_Attribute (el: &X690Element) -> ASN1Result<Attribute> {
	_decode_PartialAttribute(&el)
}

pub fn _encode_Attribute (value_: &Attribute) -> ASN1Result<X690Element> {
	_encode_PartialAttribute(&value_)
}

pub fn _validate_Attribute (el: &X690Element) -> ASN1Result<()> {
	_validate_PartialAttribute(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingRuleId  ::=  LDAPString
/// ```
pub type MatchingRuleId = LDAPString; // DefinedType

pub fn _decode_MatchingRuleId (el: &X690Element) -> ASN1Result<MatchingRuleId> {
	_decode_LDAPString(&el)
}

pub fn _encode_MatchingRuleId (value_: &MatchingRuleId) -> ASN1Result<X690Element> {
	_encode_LDAPString(&value_)
}

pub fn _validate_MatchingRuleId (el: &X690Element) -> ASN1Result<()> {
	_validate_LDAPString(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPResult ::= SEQUENCE {
///   resultCode
///     ENUMERATED {success(0), operationsError(1), protocolError(2),
///                 timeLimitExceeded(3), sizeLimitExceeded(4), compareFalse(5),
///                 compareTrue(6), authMethodNotSupported(7),
///                 strongerAuthRequired(8),
///                 -- 9 reserved
///                 referral(10), adminLimitExceeded(11),
///                 unavailableCriticalExtension(12), confidentialityRequired(13),
///                 saslBindInProgress(14), noSuchAttribute(16),
///                 undefinedAttributeType(17), inappropriateMatching(18),
///                 constraintViolation(19), attributeOrValueExists(20),
///                 invalidAttributeSyntax(21),
///                 -- 22-31 unused
///                 noSuchObject(32), aliasProblem(33),
///                 invalidDNSyntax(34),
///                 -- 35 reserved for undefined isLeaf
///                 aliasDereferencingProblem(36),
///                 -- 37-47 unused
///                 inappropriateAuthentication(48), invalidCredentials(49),
///                 insufficientAccessRights(50), busy(51), unavailable(52),
///                 unwillingToPerform(53),
///                 loopDetect(54),
///                 -- 55-63 unused
///                 namingViolation(64), objectClassViolation(65),
///                 notAllowedOnNonLeaf(66), notAllowedOnRDN(67),
///                 entryAlreadyExists(68),
///                 objectClassModsProhibited(69),
///                 -- 70 reserved for CLDAP
///                 affectsMultipleDSAs(71),
///                 -- 72-79 unused
///                 other(80), ...
///                 },
///   matchedDN          LDAPDN,
///   diagnosticMessage  LDAPString,
///   referral           [3]  Referral OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LDAPResult {
		pub resultCode: LDAPResult_resultCode,
		pub matchedDN: LDAPDN,
		pub diagnosticMessage: LDAPString,
		pub referral: OPTIONAL<Referral>,
		pub _unrecognized: Vec<X690Element>
}
impl  LDAPResult {
	pub fn new (
		resultCode: LDAPResult_resultCode,
		matchedDN: LDAPDN,
		diagnosticMessage: LDAPString,
		referral: OPTIONAL<Referral>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		LDAPResult { resultCode, matchedDN, diagnosticMessage, referral, _unrecognized}
	}

}
impl TryFrom<&X690Element> for LDAPResult {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_LDAPResult(el)
	}
}

pub const _rctl1_components_for_LDAPResult: &[ComponentSpec; 4] = &[
	ComponentSpec::new("resultCode", false, TagSelector::tag((TagClass::UNIVERSAL, 10)), None, None),
	ComponentSpec::new("matchedDN", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("diagnosticMessage", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("referral", true, TagSelector::tag((TagClass::CONTEXT, 3)), None, None)
];

pub const _rctl2_components_for_LDAPResult: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_LDAPResult: &[ComponentSpec; 0] = &[

];

pub fn _decode_LDAPResult (el: &X690Element) -> ASN1Result<LDAPResult> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LDAPResult")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_LDAPResult,
		_eal_components_for_LDAPResult,
		_rctl2_components_for_LDAPResult,
	).into_iter();
	let mut _i: usize = 0;
	let mut resultCode: OPTIONAL<LDAPResult_resultCode> = None;
	let mut matchedDN: OPTIONAL<LDAPDN> = None;
	let mut diagnosticMessage: OPTIONAL<LDAPString> = None;
	let mut referral: OPTIONAL<Referral> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"resultCode" => resultCode = Some(_decode_LDAPResult_resultCode(_el)?),
			"matchedDN" => matchedDN = Some(_decode_LDAPDN(_el)?),
			"diagnosticMessage" => diagnosticMessage = Some(_decode_LDAPString(_el)?),
			"referral" => referral = Some(_decode_Referral(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(LDAPResult{ resultCode: resultCode.unwrap(), matchedDN: matchedDN.unwrap(), diagnosticMessage: diagnosticMessage.unwrap(), referral, _unrecognized })

}

pub fn _encode_LDAPResult (value_: &LDAPResult) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(14);
	components_.push(_encode_LDAPResult_resultCode(&value_.resultCode)?);
	components_.push(_encode_LDAPDN(&value_.matchedDN)?);
	components_.push(_encode_LDAPString(&value_.diagnosticMessage)?);
	if let Some(v_) = &value_.referral {
		components_.push(|v_1: &Referral| -> ASN1Result<X690Element> { let mut el_1 = _encode_Referral(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 3; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_LDAPResult (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LDAPResult")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_LDAPResult,
		_eal_components_for_LDAPResult,
		_rctl2_components_for_LDAPResult,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"resultCode" => _validate_LDAPResult_resultCode(_el)?,
			"matchedDN" => _validate_LDAPDN(_el)?,
			"diagnosticMessage" => _validate_LDAPString(_el)?,
			"referral" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "referral"));
	}
	Ok(_validate_Referral(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Referral  ::=  SEQUENCE SIZE (1..MAX) OF uri URI
/// ```
pub type Referral = Vec<URI>; // SequenceOfType

pub fn _decode_Referral (el: &X690Element) -> ASN1Result<Referral> {
		let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Referral")),
	};
	let mut items: SEQUENCE_OF<URI> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_URI(el)?);
	}
	Ok(items)
}

pub fn _encode_Referral (value_: &Referral) -> ASN1Result<X690Element> {
		let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_URI(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}

pub fn _validate_Referral (el: &X690Element) -> ASN1Result<()> {
		match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_URI(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Referral")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// URI  ::=  LDAPString
/// ```
pub type URI = LDAPString; // DefinedType

pub fn _decode_URI (el: &X690Element) -> ASN1Result<URI> {
	_decode_LDAPString(&el)
}

pub fn _encode_URI (value_: &URI) -> ASN1Result<X690Element> {
	_encode_LDAPString(&value_)
}

pub fn _validate_URI (el: &X690Element) -> ASN1Result<()> {
	_validate_LDAPString(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Controls  ::=  SEQUENCE OF control Control
/// ```
pub type Controls = Vec<Control>; // SequenceOfType

pub fn _decode_Controls (el: &X690Element) -> ASN1Result<Controls> {
		let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Controls")),
	};
	let mut items: SEQUENCE_OF<Control> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_Control(el)?);
	}
	Ok(items)
}

pub fn _encode_Controls (value_: &Controls) -> ASN1Result<X690Element> {
		let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_Control(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}

pub fn _validate_Controls (el: &X690Element) -> ASN1Result<()> {
		match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_Control(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Controls")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Control ::= SEQUENCE {
///   controlType   LDAPOID,
///   criticality   BOOLEAN DEFAULT FALSE,
///   controlValue  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Control {
		pub controlType: LDAPOID,
		pub criticality: OPTIONAL<BOOLEAN>,
		pub controlValue: OPTIONAL<OCTET_STRING>,
		pub _unrecognized: Vec<X690Element>
}
impl  Control {
	pub fn new (
		controlType: LDAPOID,
		criticality: OPTIONAL<BOOLEAN>,
		controlValue: OPTIONAL<OCTET_STRING>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		Control { controlType, criticality, controlValue, _unrecognized}
	}
	pub fn _default_value_for_criticality () -> BOOLEAN { false }
}
impl TryFrom<&X690Element> for Control {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_Control(el)
	}
}

pub const _rctl1_components_for_Control: &[ComponentSpec; 3] = &[
	ComponentSpec::new("controlType", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("criticality", true, TagSelector::tag((TagClass::UNIVERSAL, 1)), None, None),
	ComponentSpec::new("controlValue", true, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None)
];

pub const _rctl2_components_for_Control: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_Control: &[ComponentSpec; 0] = &[

];

pub fn _decode_Control (el: &X690Element) -> ASN1Result<Control> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Control")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_Control,
		_eal_components_for_Control,
		_rctl2_components_for_Control,
	).into_iter();
	let mut _i: usize = 0;
	let mut controlType: OPTIONAL<LDAPOID> = None;
	let mut criticality: OPTIONAL<BOOLEAN> = None;
	let mut controlValue: OPTIONAL<OCTET_STRING> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"controlType" => controlType = Some(_decode_LDAPOID(_el)?),
			"criticality" => criticality = Some(BER.decode_boolean(_el)?),
			"controlValue" => controlValue = Some(BER.decode_octet_string(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(Control{ controlType: controlType.unwrap(), criticality, controlValue, _unrecognized })

}

pub fn _encode_Control (value_: &Control) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(13);
	components_.push(_encode_LDAPOID(&value_.controlType)?);
	if let Some(v_) = &value_.criticality {
		if *v_ != Control::_default_value_for_criticality() {
			components_.push(BER.encode_boolean(&v_)?);
		}
	}
	if let Some(v_) = &value_.controlValue {
		components_.push(BER.encode_octet_string(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_Control (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Control")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_Control,
		_eal_components_for_Control,
		_rctl2_components_for_Control,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"controlType" => _validate_LDAPOID(_el)?,
			"criticality" => BER.validate_boolean(_el)?,
			"controlValue" => BER.validate_octet_string(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BindRequest ::= [APPLICATION 0]  SEQUENCE {
///   version         INTEGER(1..127),
///   name            LDAPDN,
///   authentication  AuthenticationChoice
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BindRequest {
		pub version: INTEGER,
		pub name: LDAPDN,
		pub authentication: AuthenticationChoice,
		pub _unrecognized: Vec<X690Element>
}
impl  BindRequest {
	pub fn new (
		version: INTEGER,
		name: LDAPDN,
		authentication: AuthenticationChoice,
		_unrecognized: Vec<X690Element>
	) -> Self {
		BindRequest { version, name, authentication, _unrecognized}
	}

}
impl TryFrom<&X690Element> for BindRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_BindRequest(el)
	}
}

pub const _rctl1_components_for_BindRequest: &[ComponentSpec; 3] = &[
	ComponentSpec::new("version", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("name", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("authentication", false, TagSelector::any, None, None)
];

pub const _rctl2_components_for_BindRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_BindRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_BindRequest (el: &X690Element) -> ASN1Result<BindRequest> {
	|el: &X690Element| -> ASN1Result<BindRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_BindRequest,
		_eal_components_for_BindRequest,
		_rctl2_components_for_BindRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut version: OPTIONAL<INTEGER> = None;
	let mut name: OPTIONAL<LDAPDN> = None;
	let mut authentication: OPTIONAL<AuthenticationChoice> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"version" => version = Some(BER.decode_integer(_el)?),
			"name" => name = Some(_decode_LDAPDN(_el)?),
			"authentication" => authentication = Some(_decode_AuthenticationChoice(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(BindRequest{ version: version.unwrap(), name: name.unwrap(), authentication: authentication.unwrap(), _unrecognized })
}(&el)
}

pub fn _encode_BindRequest (value_: &BindRequest) -> ASN1Result<X690Element> {
	|v_1: &BindRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &BindRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(13);
	components_.push(BER.encode_integer(&value_.version)?);
	components_.push(_encode_LDAPDN(&value_.name)?);
	components_.push(_encode_AuthenticationChoice(&value_.authentication)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 0; Ok(el_1) }(&value_)
}

pub fn _validate_BindRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_BindRequest,
		_eal_components_for_BindRequest,
		_rctl2_components_for_BindRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"version" => BER.validate_integer(_el)?,
			"name" => _validate_LDAPDN(_el)?,
			"authentication" => _validate_AuthenticationChoice(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticationChoice  ::=  CHOICE {
///   simple  [0]  OCTET STRING,
///   -- 1 and 2 reserved
///   sasl    [3]  SaslCredentials,
///   ...
/// }
/// ```
#[derive(Debug, Clone)]
pub enum AuthenticationChoice {
	simple(OCTET_STRING),
	sasl(SaslCredentials),
	_unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for AuthenticationChoice {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_AuthenticationChoice(el)
	}
}

pub fn _decode_AuthenticationChoice (el: &X690Element) -> ASN1Result<AuthenticationChoice> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(AuthenticationChoice::simple(BER.decode_octet_string(&el)?)),
		(TagClass::CONTEXT, 3) => Ok(AuthenticationChoice::sasl(_decode_SaslCredentials(&el)?)),
		_ => Ok(AuthenticationChoice::_unrecognized(el.clone())),
	}
}

pub fn _encode_AuthenticationChoice (value_: &AuthenticationChoice) -> ASN1Result<X690Element> {
		match value_ {
		AuthenticationChoice::simple(v) => |v_1: &OCTET_STRING| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_octet_string(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v),
		AuthenticationChoice::sasl(v) => |v_1: &SaslCredentials| -> ASN1Result<X690Element> { let mut el_1 = _encode_SaslCredentials(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 3; Ok(el_1) }(&v),
		AuthenticationChoice::_unrecognized(el) => Ok(el.clone()),
	}
}

pub fn _validate_AuthenticationChoice (el: &X690Element) -> ASN1Result<()> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "simple"));
	}
	Ok(BER.validate_octet_string(&el)?)
}(&el),
		(TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "sasl"));
	}
	Ok(_validate_SaslCredentials(&el)?)
}(&el),
		_ => Ok(()),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SaslCredentials ::= SEQUENCE {
///   mechanism    LDAPString,
///   credentials  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SaslCredentials {
		pub mechanism: LDAPString,
		pub credentials: OPTIONAL<OCTET_STRING>,
		pub _unrecognized: Vec<X690Element>
}
impl  SaslCredentials {
	pub fn new (
		mechanism: LDAPString,
		credentials: OPTIONAL<OCTET_STRING>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		SaslCredentials { mechanism, credentials, _unrecognized}
	}

}
impl TryFrom<&X690Element> for SaslCredentials {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_SaslCredentials(el)
	}
}

pub const _rctl1_components_for_SaslCredentials: &[ComponentSpec; 2] = &[
	ComponentSpec::new("mechanism", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("credentials", true, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None)
];

pub const _rctl2_components_for_SaslCredentials: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_SaslCredentials: &[ComponentSpec; 0] = &[

];

pub fn _decode_SaslCredentials (el: &X690Element) -> ASN1Result<SaslCredentials> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SaslCredentials")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SaslCredentials,
		_eal_components_for_SaslCredentials,
		_rctl2_components_for_SaslCredentials,
	).into_iter();
	let mut _i: usize = 0;
	let mut mechanism: OPTIONAL<LDAPString> = None;
	let mut credentials: OPTIONAL<OCTET_STRING> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"mechanism" => mechanism = Some(_decode_LDAPString(_el)?),
			"credentials" => credentials = Some(BER.decode_octet_string(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(SaslCredentials{ mechanism: mechanism.unwrap(), credentials, _unrecognized })

}

pub fn _encode_SaslCredentials (value_: &SaslCredentials) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_LDAPString(&value_.mechanism)?);
	if let Some(v_) = &value_.credentials {
		components_.push(BER.encode_octet_string(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_SaslCredentials (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SaslCredentials")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SaslCredentials,
		_eal_components_for_SaslCredentials,
		_rctl2_components_for_SaslCredentials,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"mechanism" => _validate_LDAPString(_el)?,
			"credentials" => BER.validate_octet_string(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BindResponse ::= [APPLICATION 1]  SEQUENCE {
///   COMPONENTS OF LDAPResult,
///   serverSaslCreds  [7]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BindResponse {
		pub resultCode: LDAPResult_resultCode /* REPLICATED_COMPONENT */,
		pub matchedDN: LDAPDN /* REPLICATED_COMPONENT */,
		pub diagnosticMessage: LDAPString /* REPLICATED_COMPONENT */,
		pub referral: OPTIONAL<Referral> /* REPLICATED_COMPONENT */,
		pub serverSaslCreds: OPTIONAL<OCTET_STRING>,
		pub _unrecognized: Vec<X690Element>
}
impl  BindResponse {
	pub fn new (
		resultCode: LDAPResult_resultCode /* REPLICATED_COMPONENT */,
		matchedDN: LDAPDN /* REPLICATED_COMPONENT */,
		diagnosticMessage: LDAPString /* REPLICATED_COMPONENT */,
		referral: OPTIONAL<Referral> /* REPLICATED_COMPONENT */,
		serverSaslCreds: OPTIONAL<OCTET_STRING>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		BindResponse { resultCode, matchedDN, diagnosticMessage, referral, serverSaslCreds, _unrecognized}
	}

}
impl TryFrom<&X690Element> for BindResponse {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_BindResponse(el)
	}
}

pub const _rctl1_components_for_BindResponse: &[ComponentSpec; 5] = &[
	ComponentSpec::new("resultCode", false, TagSelector::tag((TagClass::UNIVERSAL, 10)), None, None),
	ComponentSpec::new("matchedDN", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("diagnosticMessage", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("referral", true, TagSelector::tag((TagClass::CONTEXT, 3)), None, None),
	ComponentSpec::new("serverSaslCreds", true, TagSelector::tag((TagClass::CONTEXT, 7)), None, None)
];

pub const _rctl2_components_for_BindResponse: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_BindResponse: &[ComponentSpec; 0] = &[

];

pub fn _decode_BindResponse (el: &X690Element) -> ASN1Result<BindResponse> {
	|el: &X690Element| -> ASN1Result<BindResponse> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindResponse")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_BindResponse,
		_eal_components_for_BindResponse,
		_rctl2_components_for_BindResponse,
	).into_iter();
	let mut _i: usize = 0;
	let mut resultCode: OPTIONAL<LDAPResult_resultCode> = None;
	let mut matchedDN: OPTIONAL<LDAPDN> = None;
	let mut diagnosticMessage: OPTIONAL<LDAPString> = None;
	let mut referral: OPTIONAL<Referral> = None;
	let mut serverSaslCreds: OPTIONAL<OCTET_STRING> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"resultCode" => resultCode = Some(_decode_LDAPResult_resultCode(_el)?),
			"matchedDN" => matchedDN = Some(_decode_LDAPDN(_el)?),
			"diagnosticMessage" => diagnosticMessage = Some(_decode_LDAPString(_el)?),
			"referral" => referral = Some(_decode_Referral(_el)?),
			"serverSaslCreds" => serverSaslCreds = Some(BER.decode_octet_string(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(BindResponse{ resultCode: resultCode.unwrap(), matchedDN: matchedDN.unwrap(), diagnosticMessage: diagnosticMessage.unwrap(), referral, serverSaslCreds, _unrecognized })
}(&el)
}

pub fn _encode_BindResponse (value_: &BindResponse) -> ASN1Result<X690Element> {
	|v_1: &BindResponse| -> ASN1Result<X690Element> { let mut el_1 = |value_: &BindResponse| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(15);
	components_.push(_encode_LDAPResult_resultCode(&value_.resultCode)?);
	components_.push(_encode_LDAPDN(&value_.matchedDN)?);
	components_.push(_encode_LDAPString(&value_.diagnosticMessage)?);
	if let Some(v_) = &value_.referral {
		components_.push(|v_1: &Referral| -> ASN1Result<X690Element> { let mut el_1 = _encode_Referral(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 3; Ok(el_1) }(&v_)?);
	}
	if let Some(v_) = &value_.serverSaslCreds {
		components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_octet_string(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 7; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 1; Ok(el_1) }(&value_)
}

pub fn _validate_BindResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindResponse"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindResponse")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_BindResponse,
		_eal_components_for_BindResponse,
		_rctl2_components_for_BindResponse,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"resultCode" => _validate_LDAPResult_resultCode(_el)?,
			"matchedDN" => _validate_LDAPDN(_el)?,
			"diagnosticMessage" => _validate_LDAPString(_el)?,
			"referral" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "referral"));
	}
	Ok(_validate_Referral(&el)?)
}(_el)?,
			"serverSaslCreds" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "serverSaslCreds"));
	}
	Ok(BER.validate_octet_string(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnbindRequest  ::=  [APPLICATION 2]  NULL
/// ```
pub type UnbindRequest = NULL; // NullType

pub fn _decode_UnbindRequest (el: &X690Element) -> ASN1Result<UnbindRequest> {
	BER.decode_null(&el)
}

pub fn _encode_UnbindRequest (value_: &UnbindRequest) -> ASN1Result<X690Element> {
	|v_1: &UnbindRequest| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_null(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 2; Ok(el_1) }(&value_)
}

pub fn _validate_UnbindRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 2 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UnbindRequest"));
	}
	Ok(BER.validate_null(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRequest ::= [APPLICATION 3]  SEQUENCE {
///   baseObject    LDAPDN,
///   scope
///     ENUMERATED {baseObject(0), singleLevel(1), wholeSubtree(2), ...
///                 },
///   derefAliases
///     ENUMERATED {neverDerefAliases(0), derefInSearching(1),
///                 derefFindingBaseObj(2), derefAlways(3)},
///   sizeLimit     INTEGER(0..maxInt),
///   timeLimit     INTEGER(0..maxInt),
///   typesOnly     BOOLEAN,
///   filter        Filter,
///   attributes    AttributeSelection
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchRequest {
		pub baseObject: LDAPDN,
		pub scope: SearchRequest_scope,
		pub derefAliases: SearchRequest_derefAliases,
		pub sizeLimit: INTEGER,
		pub timeLimit: INTEGER,
		pub typesOnly: BOOLEAN,
		pub filter: Filter,
		pub attributes: AttributeSelection,
		pub _unrecognized: Vec<X690Element>
}
impl  SearchRequest {
	pub fn new (
		baseObject: LDAPDN,
		scope: SearchRequest_scope,
		derefAliases: SearchRequest_derefAliases,
		sizeLimit: INTEGER,
		timeLimit: INTEGER,
		typesOnly: BOOLEAN,
		filter: Filter,
		attributes: AttributeSelection,
		_unrecognized: Vec<X690Element>
	) -> Self {
		SearchRequest { baseObject, scope, derefAliases, sizeLimit, timeLimit, typesOnly, filter, attributes, _unrecognized}
	}

}
impl TryFrom<&X690Element> for SearchRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_SearchRequest(el)
	}
}

pub const _rctl1_components_for_SearchRequest: &[ComponentSpec; 8] = &[
	ComponentSpec::new("baseObject", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("scope", false, TagSelector::tag((TagClass::UNIVERSAL, 10)), None, None),
	ComponentSpec::new("derefAliases", false, TagSelector::tag((TagClass::UNIVERSAL, 10)), None, None),
	ComponentSpec::new("sizeLimit", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("timeLimit", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("typesOnly", false, TagSelector::tag((TagClass::UNIVERSAL, 1)), None, None),
	ComponentSpec::new("filter", false, TagSelector::any, None, None),
	ComponentSpec::new("attributes", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_SearchRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_SearchRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_SearchRequest (el: &X690Element) -> ASN1Result<SearchRequest> {
	|el: &X690Element| -> ASN1Result<SearchRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SearchRequest,
		_eal_components_for_SearchRequest,
		_rctl2_components_for_SearchRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut baseObject: OPTIONAL<LDAPDN> = None;
	let mut scope: OPTIONAL<SearchRequest_scope> = None;
	let mut derefAliases: OPTIONAL<SearchRequest_derefAliases> = None;
	let mut sizeLimit: OPTIONAL<INTEGER> = None;
	let mut timeLimit: OPTIONAL<INTEGER> = None;
	let mut typesOnly: OPTIONAL<BOOLEAN> = None;
	let mut filter: OPTIONAL<Filter> = None;
	let mut attributes: OPTIONAL<AttributeSelection> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"baseObject" => baseObject = Some(_decode_LDAPDN(_el)?),
			"scope" => scope = Some(_decode_SearchRequest_scope(_el)?),
			"derefAliases" => derefAliases = Some(_decode_SearchRequest_derefAliases(_el)?),
			"sizeLimit" => sizeLimit = Some(BER.decode_integer(_el)?),
			"timeLimit" => timeLimit = Some(BER.decode_integer(_el)?),
			"typesOnly" => typesOnly = Some(BER.decode_boolean(_el)?),
			"filter" => filter = Some(_decode_Filter(_el)?),
			"attributes" => attributes = Some(_decode_AttributeSelection(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(SearchRequest{ baseObject: baseObject.unwrap(), scope: scope.unwrap(), derefAliases: derefAliases.unwrap(), sizeLimit: sizeLimit.unwrap(), timeLimit: timeLimit.unwrap(), typesOnly: typesOnly.unwrap(), filter: filter.unwrap(), attributes: attributes.unwrap(), _unrecognized })
}(&el)
}

pub fn _encode_SearchRequest (value_: &SearchRequest) -> ASN1Result<X690Element> {
	|v_1: &SearchRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SearchRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(18);
	components_.push(_encode_LDAPDN(&value_.baseObject)?);
	components_.push(_encode_SearchRequest_scope(&value_.scope)?);
	components_.push(_encode_SearchRequest_derefAliases(&value_.derefAliases)?);
	components_.push(BER.encode_integer(&value_.sizeLimit)?);
	components_.push(BER.encode_integer(&value_.timeLimit)?);
	components_.push(BER.encode_boolean(&value_.typesOnly)?);
	components_.push(_encode_Filter(&value_.filter)?);
	components_.push(_encode_AttributeSelection(&value_.attributes)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 3; Ok(el_1) }(&value_)
}

pub fn _validate_SearchRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SearchRequest,
		_eal_components_for_SearchRequest,
		_rctl2_components_for_SearchRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"baseObject" => _validate_LDAPDN(_el)?,
			"scope" => _validate_SearchRequest_scope(_el)?,
			"derefAliases" => _validate_SearchRequest_derefAliases(_el)?,
			"sizeLimit" => BER.validate_integer(_el)?,
			"timeLimit" => BER.validate_integer(_el)?,
			"typesOnly" => BER.validate_boolean(_el)?,
			"filter" => _validate_Filter(_el)?,
			"attributes" => _validate_AttributeSelection(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeSelection  ::=  SEQUENCE OF selector LDAPString
/// ```
pub type AttributeSelection = Vec<LDAPString>; // SequenceOfType

pub fn _decode_AttributeSelection (el: &X690Element) -> ASN1Result<AttributeSelection> {
		let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeSelection")),
	};
	let mut items: SEQUENCE_OF<LDAPString> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_LDAPString(el)?);
	}
	Ok(items)
}

pub fn _encode_AttributeSelection (value_: &AttributeSelection) -> ASN1Result<X690Element> {
		let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_LDAPString(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}

pub fn _validate_AttributeSelection (el: &X690Element) -> ASN1Result<()> {
		match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_LDAPString(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeSelection")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Filter  ::=  CHOICE {
///   and              [0]  SET SIZE (1..MAX) OF filter Filter,
///   or               [1]  SET SIZE (1..MAX) OF filter Filter,
///   not              [2]  Filter,
///   equalityMatch    [3]  AttributeValueAssertion,
///   substrings       [4]  SubstringFilter,
///   greaterOrEqual   [5]  AttributeValueAssertion,
///   lessOrEqual      [6]  AttributeValueAssertion,
///   present          [7]  AttributeDescription,
///   approxMatch      [8]  AttributeValueAssertion,
///   extensibleMatch  [9]  MatchingRuleAssertion,
///   ...
/// }
/// ```
// TODO: CHECK_RECURSIVE_DEFINITION
#[derive(Debug, Clone)]
pub enum Filter {
	and(Vec<Filter>),
	or(Vec<Filter>),
	not(Box<Filter>),
	equalityMatch(AttributeValueAssertion),
	substrings(SubstringFilter),
	greaterOrEqual(AttributeValueAssertion),
	lessOrEqual(AttributeValueAssertion),
	present(AttributeDescription),
	approxMatch(AttributeValueAssertion),
	extensibleMatch(MatchingRuleAssertion),
	_unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Filter {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_Filter(el)
	}
}

pub fn _decode_Filter (el: &X690Element) -> ASN1Result<Filter> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(Filter::and(|el: &X690Element| -> ASN1Result<SET_OF<Filter>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")),
	};
	let mut items: SET_OF<Filter> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_Filter(el)?);
	}
	Ok(items)
}(&el)?)),
		(TagClass::CONTEXT, 1) => Ok(Filter::or(|el: &X690Element| -> ASN1Result<SET_OF<Filter>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")),
	};
	let mut items: SET_OF<Filter> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_Filter(el)?);
	}
	Ok(items)
}(&el)?)),
		(TagClass::CONTEXT, 2) => Ok(Filter::not(Box::new(|el: &X690Element| -> ASN1Result<Filter> {
	Ok(_decode_Filter(&el.inner()?)?)
}(&el)?))),
		(TagClass::CONTEXT, 3) => Ok(Filter::equalityMatch(_decode_AttributeValueAssertion(&el)?)),
		(TagClass::CONTEXT, 4) => Ok(Filter::substrings(_decode_SubstringFilter(&el)?)),
		(TagClass::CONTEXT, 5) => Ok(Filter::greaterOrEqual(_decode_AttributeValueAssertion(&el)?)),
		(TagClass::CONTEXT, 6) => Ok(Filter::lessOrEqual(_decode_AttributeValueAssertion(&el)?)),
		(TagClass::CONTEXT, 7) => Ok(Filter::present(_decode_AttributeDescription(&el)?)),
		(TagClass::CONTEXT, 8) => Ok(Filter::approxMatch(_decode_AttributeValueAssertion(&el)?)),
		(TagClass::CONTEXT, 9) => Ok(Filter::extensibleMatch(_decode_MatchingRuleAssertion(&el)?)),
		_ => Ok(Filter::_unrecognized(el.clone())),
	}
}

pub fn _encode_Filter (value_: &Filter) -> ASN1Result<X690Element> {
		match value_ {
		Filter::and(v) => |v_1: &Vec<Filter>| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SET_OF<Filter>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_Filter(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v),
		Filter::or(v) => |v_1: &Vec<Filter>| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SET_OF<Filter>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_Filter(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 1; Ok(el_1) }(&v),
		Filter::not(v) => |v_1: &Filter| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 2), X690Value::from_explicit(&_encode_Filter(&v_1)?))) }(&v),
		Filter::equalityMatch(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> { let mut el_1 = _encode_AttributeValueAssertion(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 3; Ok(el_1) }(&v),
		Filter::substrings(v) => |v_1: &SubstringFilter| -> ASN1Result<X690Element> { let mut el_1 = _encode_SubstringFilter(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 4; Ok(el_1) }(&v),
		Filter::greaterOrEqual(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> { let mut el_1 = _encode_AttributeValueAssertion(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 5; Ok(el_1) }(&v),
		Filter::lessOrEqual(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> { let mut el_1 = _encode_AttributeValueAssertion(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 6; Ok(el_1) }(&v),
		Filter::present(v) => |v_1: &AttributeDescription| -> ASN1Result<X690Element> { let mut el_1 = _encode_AttributeDescription(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 7; Ok(el_1) }(&v),
		Filter::approxMatch(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> { let mut el_1 = _encode_AttributeValueAssertion(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 8; Ok(el_1) }(&v),
		Filter::extensibleMatch(v) => |v_1: &MatchingRuleAssertion| -> ASN1Result<X690Element> { let mut el_1 = _encode_MatchingRuleAssertion(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 9; Ok(el_1) }(&v),
		Filter::_unrecognized(el) => Ok(el.clone()),
	}
}

pub fn _validate_Filter (el: &X690Element) -> ASN1Result<()> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_Filter(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")),
	}
}(&el)?)
}(&el),
		(TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_Filter(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")),
	}
}(&el)?)
}(&el),
		(TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not"));
	}
	Ok(_validate_Filter(&el.inner()?)?)
}(&el),
		(TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "equalityMatch"));
	}
	Ok(_validate_AttributeValueAssertion(&el)?)
}(&el),
		(TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "substrings"));
	}
	Ok(_validate_SubstringFilter(&el)?)
}(&el),
		(TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "greaterOrEqual"));
	}
	Ok(_validate_AttributeValueAssertion(&el)?)
}(&el),
		(TagClass::CONTEXT, 6) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "lessOrEqual"));
	}
	Ok(_validate_AttributeValueAssertion(&el)?)
}(&el),
		(TagClass::CONTEXT, 7) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "present"));
	}
	Ok(_validate_AttributeDescription(&el)?)
}(&el),
		(TagClass::CONTEXT, 8) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "approxMatch"));
	}
	Ok(_validate_AttributeValueAssertion(&el)?)
}(&el),
		(TagClass::CONTEXT, 9) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensibleMatch"));
	}
	Ok(_validate_MatchingRuleAssertion(&el)?)
}(&el),
		_ => Ok(()),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubstringFilter ::= SEQUENCE {
///   type        AttributeDescription,
///   substrings
///     SEQUENCE SIZE (1..MAX) OF substring
///       CHOICE {initial  [0]  AssertionValue, -- can occur at most once--
///               any      [1]  AssertionValue,
///               final    [2]  AssertionValue} -- can occur at most once
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SubstringFilter {
		pub type_: AttributeDescription,
		pub substrings: Vec<SubstringFilter_substrings_substring>,
		pub _unrecognized: Vec<X690Element>
}
impl  SubstringFilter {
	pub fn new (
		type_: AttributeDescription,
		substrings: Vec<SubstringFilter_substrings_substring>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		SubstringFilter { type_, substrings, _unrecognized}
	}

}
impl TryFrom<&X690Element> for SubstringFilter {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_SubstringFilter(el)
	}
}

pub const _rctl1_components_for_SubstringFilter: &[ComponentSpec; 2] = &[
	ComponentSpec::new("type", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("substrings", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_SubstringFilter: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_SubstringFilter: &[ComponentSpec; 0] = &[

];

pub fn _decode_SubstringFilter (el: &X690Element) -> ASN1Result<SubstringFilter> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubstringFilter")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SubstringFilter,
		_eal_components_for_SubstringFilter,
		_rctl2_components_for_SubstringFilter,
	).into_iter();
	let mut _i: usize = 0;
	let mut type_: OPTIONAL<AttributeDescription> = None;
	let mut substrings: OPTIONAL<Vec<SubstringFilter_substrings_substring>> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"type" => type_ = Some(_decode_AttributeDescription(_el)?),
			"substrings" => substrings = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<SubstringFilter_substrings_substring>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "substrings")),
	};
	let mut items: SEQUENCE_OF<SubstringFilter_substrings_substring> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_SubstringFilter_substrings_substring(el)?);
	}
	Ok(items)
}(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(SubstringFilter{ type_: type_.unwrap(), substrings: substrings.unwrap(), _unrecognized })

}

pub fn _encode_SubstringFilter (value_: &SubstringFilter) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_AttributeDescription(&value_.type_)?);
	components_.push(|value_: &SEQUENCE_OF<SubstringFilter_substrings_substring>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_SubstringFilter_substrings_substring(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&value_.substrings)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_SubstringFilter (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubstringFilter")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SubstringFilter,
		_eal_components_for_SubstringFilter,
		_rctl2_components_for_SubstringFilter,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"type" => _validate_AttributeDescription(_el)?,
			"substrings" => |el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_SubstringFilter_substrings_substring(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "substrings")),
	}
}(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingRuleAssertion ::= SEQUENCE {
///   matchingRule  [1]  MatchingRuleId OPTIONAL,
///   type          [2]  AttributeDescription OPTIONAL,
///   matchValue    [3]  AssertionValue,
///   dnAttributes  [4]  BOOLEAN DEFAULT FALSE
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MatchingRuleAssertion {
		pub matchingRule: OPTIONAL<MatchingRuleId>,
		pub type_: OPTIONAL<AttributeDescription>,
		pub matchValue: AssertionValue,
		pub dnAttributes: OPTIONAL<BOOLEAN>,
		pub _unrecognized: Vec<X690Element>
}
impl  MatchingRuleAssertion {
	pub fn new (
		matchingRule: OPTIONAL<MatchingRuleId>,
		type_: OPTIONAL<AttributeDescription>,
		matchValue: AssertionValue,
		dnAttributes: OPTIONAL<BOOLEAN>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		MatchingRuleAssertion { matchingRule, type_, matchValue, dnAttributes, _unrecognized}
	}
	pub fn _default_value_for_dnAttributes () -> BOOLEAN { false }
}
impl TryFrom<&X690Element> for MatchingRuleAssertion {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_MatchingRuleAssertion(el)
	}
}

pub const _rctl1_components_for_MatchingRuleAssertion: &[ComponentSpec; 4] = &[
	ComponentSpec::new("matchingRule", true, TagSelector::tag((TagClass::CONTEXT, 1)), None, None),
	ComponentSpec::new("type", true, TagSelector::tag((TagClass::CONTEXT, 2)), None, None),
	ComponentSpec::new("matchValue", false, TagSelector::tag((TagClass::CONTEXT, 3)), None, None),
	ComponentSpec::new("dnAttributes", true, TagSelector::tag((TagClass::CONTEXT, 4)), None, None)
];

pub const _rctl2_components_for_MatchingRuleAssertion: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_MatchingRuleAssertion: &[ComponentSpec; 0] = &[

];

pub fn _decode_MatchingRuleAssertion (el: &X690Element) -> ASN1Result<MatchingRuleAssertion> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MatchingRuleAssertion")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_MatchingRuleAssertion,
		_eal_components_for_MatchingRuleAssertion,
		_rctl2_components_for_MatchingRuleAssertion,
	).into_iter();
	let mut _i: usize = 0;
	let mut matchingRule: OPTIONAL<MatchingRuleId> = None;
	let mut type_: OPTIONAL<AttributeDescription> = None;
	let mut matchValue: OPTIONAL<AssertionValue> = None;
	let mut dnAttributes: OPTIONAL<BOOLEAN> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"matchingRule" => matchingRule = Some(_decode_MatchingRuleId(_el)?),
			"type" => type_ = Some(_decode_AttributeDescription(_el)?),
			"matchValue" => matchValue = Some(_decode_AssertionValue(_el)?),
			"dnAttributes" => dnAttributes = Some(BER.decode_boolean(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(MatchingRuleAssertion{ matchingRule, type_, matchValue: matchValue.unwrap(), dnAttributes, _unrecognized })

}

pub fn _encode_MatchingRuleAssertion (value_: &MatchingRuleAssertion) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(14);
	if let Some(v_) = &value_.matchingRule {
		components_.push(|v_1: &MatchingRuleId| -> ASN1Result<X690Element> { let mut el_1 = _encode_MatchingRuleId(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 1; Ok(el_1) }(&v_)?);
	}
	if let Some(v_) = &value_.type_ {
		components_.push(|v_1: &AttributeDescription| -> ASN1Result<X690Element> { let mut el_1 = _encode_AttributeDescription(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 2; Ok(el_1) }(&v_)?);
	}
	components_.push(|v_1: &AssertionValue| -> ASN1Result<X690Element> { let mut el_1 = _encode_AssertionValue(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 3; Ok(el_1) }(&value_.matchValue)?);
	if let Some(v_) = &value_.dnAttributes {
		if *v_ != MatchingRuleAssertion::_default_value_for_dnAttributes() {
			components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_boolean(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 4; Ok(el_1) }(&v_)?);
		}
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_MatchingRuleAssertion (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MatchingRuleAssertion")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_MatchingRuleAssertion,
		_eal_components_for_MatchingRuleAssertion,
		_rctl2_components_for_MatchingRuleAssertion,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"matchingRule" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchingRule"));
	}
	Ok(_validate_MatchingRuleId(&el)?)
}(_el)?,
			"type" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "type"));
	}
	Ok(_validate_AttributeDescription(&el)?)
}(_el)?,
			"matchValue" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchValue"));
	}
	Ok(_validate_AssertionValue(&el)?)
}(_el)?,
			"dnAttributes" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dnAttributes"));
	}
	Ok(BER.validate_boolean(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResultEntry ::= [APPLICATION 4]  SEQUENCE {
///   objectName  LDAPDN,
///   attributes  PartialAttributeList
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchResultEntry {
		pub objectName: LDAPDN,
		pub attributes: PartialAttributeList,
		pub _unrecognized: Vec<X690Element>
}
impl  SearchResultEntry {
	pub fn new (
		objectName: LDAPDN,
		attributes: PartialAttributeList,
		_unrecognized: Vec<X690Element>
	) -> Self {
		SearchResultEntry { objectName, attributes, _unrecognized}
	}

}
impl TryFrom<&X690Element> for SearchResultEntry {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_SearchResultEntry(el)
	}
}

pub const _rctl1_components_for_SearchResultEntry: &[ComponentSpec; 2] = &[
	ComponentSpec::new("objectName", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("attributes", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_SearchResultEntry: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_SearchResultEntry: &[ComponentSpec; 0] = &[

];

pub fn _decode_SearchResultEntry (el: &X690Element) -> ASN1Result<SearchResultEntry> {
	|el: &X690Element| -> ASN1Result<SearchResultEntry> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultEntry")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SearchResultEntry,
		_eal_components_for_SearchResultEntry,
		_rctl2_components_for_SearchResultEntry,
	).into_iter();
	let mut _i: usize = 0;
	let mut objectName: OPTIONAL<LDAPDN> = None;
	let mut attributes: OPTIONAL<PartialAttributeList> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"objectName" => objectName = Some(_decode_LDAPDN(_el)?),
			"attributes" => attributes = Some(_decode_PartialAttributeList(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(SearchResultEntry{ objectName: objectName.unwrap(), attributes: attributes.unwrap(), _unrecognized })
}(&el)
}

pub fn _encode_SearchResultEntry (value_: &SearchResultEntry) -> ASN1Result<X690Element> {
	|v_1: &SearchResultEntry| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SearchResultEntry| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_LDAPDN(&value_.objectName)?);
	components_.push(_encode_PartialAttributeList(&value_.attributes)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 4; Ok(el_1) }(&value_)
}

pub fn _validate_SearchResultEntry (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 4 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultEntry"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultEntry")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_SearchResultEntry,
		_eal_components_for_SearchResultEntry,
		_rctl2_components_for_SearchResultEntry,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"objectName" => _validate_LDAPDN(_el)?,
			"attributes" => _validate_PartialAttributeList(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PartialAttributeList  ::=  SEQUENCE OF partialAttribute PartialAttribute
/// ```
pub type PartialAttributeList = Vec<PartialAttribute>; // SequenceOfType

pub fn _decode_PartialAttributeList (el: &X690Element) -> ASN1Result<PartialAttributeList> {
		let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PartialAttributeList")),
	};
	let mut items: SEQUENCE_OF<PartialAttribute> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_PartialAttribute(el)?);
	}
	Ok(items)
}

pub fn _encode_PartialAttributeList (value_: &PartialAttributeList) -> ASN1Result<X690Element> {
		let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_PartialAttribute(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}

pub fn _validate_PartialAttributeList (el: &X690Element) -> ASN1Result<()> {
		match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_PartialAttribute(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PartialAttributeList")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResultReference  ::=  [APPLICATION 19]  SEQUENCE SIZE (1..MAX) OF uri URI
/// ```
pub type SearchResultReference = Vec<URI>; // SequenceOfType

pub fn _decode_SearchResultReference (el: &X690Element) -> ASN1Result<SearchResultReference> {
	|el: &X690Element| -> ASN1Result<SEQUENCE_OF<URI>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultReference")),
	};
	let mut items: SEQUENCE_OF<URI> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_URI(el)?);
	}
	Ok(items)
}(&el)
}

pub fn _encode_SearchResultReference (value_: &SearchResultReference) -> ASN1Result<X690Element> {
	|v_1: &SearchResultReference| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SEQUENCE_OF<URI>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_URI(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 19; Ok(el_1) }(&value_)
}

pub fn _validate_SearchResultReference (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 19 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultReference"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_URI(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultReference")),
	}
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResultDone  ::=  [APPLICATION 5]  LDAPResult
/// ```
pub type SearchResultDone = LDAPResult; // DefinedType

pub fn _decode_SearchResultDone (el: &X690Element) -> ASN1Result<SearchResultDone> {
	_decode_LDAPResult(&el)
}

pub fn _encode_SearchResultDone (value_: &SearchResultDone) -> ASN1Result<X690Element> {
	|v_1: &SearchResultDone| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPResult(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 5; Ok(el_1) }(&value_)
}

pub fn _validate_SearchResultDone (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 5 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchResultDone"));
	}
	Ok(_validate_LDAPResult(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRequest ::= [APPLICATION 6]  SEQUENCE {
///   object   LDAPDN,
///   changes
///     SEQUENCE OF change
///       SEQUENCE {operation     ENUMERATED {add(0), delete(1), replace(2), ...
///                                           },
///                 modification  PartialAttribute}
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyRequest {
		pub object: LDAPDN,
		pub changes: Vec<ModifyRequest_changes_change>,
		pub _unrecognized: Vec<X690Element>
}
impl  ModifyRequest {
	pub fn new (
		object: LDAPDN,
		changes: Vec<ModifyRequest_changes_change>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		ModifyRequest { object, changes, _unrecognized}
	}

}
impl TryFrom<&X690Element> for ModifyRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_ModifyRequest(el)
	}
}

pub const _rctl1_components_for_ModifyRequest: &[ComponentSpec; 2] = &[
	ComponentSpec::new("object", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("changes", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_ModifyRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_ModifyRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_ModifyRequest (el: &X690Element) -> ASN1Result<ModifyRequest> {
	|el: &X690Element| -> ASN1Result<ModifyRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ModifyRequest,
		_eal_components_for_ModifyRequest,
		_rctl2_components_for_ModifyRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut object: OPTIONAL<LDAPDN> = None;
	let mut changes: OPTIONAL<Vec<ModifyRequest_changes_change>> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"object" => object = Some(_decode_LDAPDN(_el)?),
			"changes" => changes = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<ModifyRequest_changes_change>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "changes")),
	};
	let mut items: SEQUENCE_OF<ModifyRequest_changes_change> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_ModifyRequest_changes_change(el)?);
	}
	Ok(items)
}(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(ModifyRequest{ object: object.unwrap(), changes: changes.unwrap(), _unrecognized })
}(&el)
}

pub fn _encode_ModifyRequest (value_: &ModifyRequest) -> ASN1Result<X690Element> {
	|v_1: &ModifyRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &ModifyRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_LDAPDN(&value_.object)?);
	components_.push(|value_: &SEQUENCE_OF<ModifyRequest_changes_change>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_ModifyRequest_changes_change(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&value_.changes)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 6; Ok(el_1) }(&value_)
}

pub fn _validate_ModifyRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 6 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ModifyRequest,
		_eal_components_for_ModifyRequest,
		_rctl2_components_for_ModifyRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"object" => _validate_LDAPDN(_el)?,
			"changes" => |el: &X690Element| -> ASN1Result<()> {	match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_ModifyRequest_changes_change(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "changes")),
	}
}(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyResponse  ::=  [APPLICATION 7]  LDAPResult
/// ```
pub type ModifyResponse = LDAPResult; // DefinedType

pub fn _decode_ModifyResponse (el: &X690Element) -> ASN1Result<ModifyResponse> {
	_decode_LDAPResult(&el)
}

pub fn _encode_ModifyResponse (value_: &ModifyResponse) -> ASN1Result<X690Element> {
	|v_1: &ModifyResponse| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPResult(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 7; Ok(el_1) }(&value_)
}

pub fn _validate_ModifyResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 7 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyResponse"));
	}
	Ok(_validate_LDAPResult(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddRequest ::= [APPLICATION 8]  SEQUENCE {
///   entry       LDAPDN,
///   attributes  AttributeList
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddRequest {
		pub entry: LDAPDN,
		pub attributes: AttributeList,
		pub _unrecognized: Vec<X690Element>
}
impl  AddRequest {
	pub fn new (
		entry: LDAPDN,
		attributes: AttributeList,
		_unrecognized: Vec<X690Element>
	) -> Self {
		AddRequest { entry, attributes, _unrecognized}
	}

}
impl TryFrom<&X690Element> for AddRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_AddRequest(el)
	}
}

pub const _rctl1_components_for_AddRequest: &[ComponentSpec; 2] = &[
	ComponentSpec::new("entry", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("attributes", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_AddRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_AddRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_AddRequest (el: &X690Element) -> ASN1Result<AddRequest> {
	|el: &X690Element| -> ASN1Result<AddRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_AddRequest,
		_eal_components_for_AddRequest,
		_rctl2_components_for_AddRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut entry: OPTIONAL<LDAPDN> = None;
	let mut attributes: OPTIONAL<AttributeList> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"entry" => entry = Some(_decode_LDAPDN(_el)?),
			"attributes" => attributes = Some(_decode_AttributeList(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(AddRequest{ entry: entry.unwrap(), attributes: attributes.unwrap(), _unrecognized })
}(&el)
}

pub fn _encode_AddRequest (value_: &AddRequest) -> ASN1Result<X690Element> {
	|v_1: &AddRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &AddRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_LDAPDN(&value_.entry)?);
	components_.push(_encode_AttributeList(&value_.attributes)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 8; Ok(el_1) }(&value_)
}

pub fn _validate_AddRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 8 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_AddRequest,
		_eal_components_for_AddRequest,
		_rctl2_components_for_AddRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"entry" => _validate_LDAPDN(_el)?,
			"attributes" => _validate_AttributeList(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeList  ::=  SEQUENCE OF attribute Attribute
/// ```
pub type AttributeList = Vec<Attribute>; // SequenceOfType

pub fn _decode_AttributeList (el: &X690Element) -> ASN1Result<AttributeList> {
		let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeList")),
	};
	let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_Attribute(el)?);
	}
	Ok(items)
}

pub fn _encode_AttributeList (value_: &AttributeList) -> ASN1Result<X690Element> {
		let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_Attribute(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}

pub fn _validate_AttributeList (el: &X690Element) -> ASN1Result<()> {
		match &el.value {
		X690Value::Constructed(subs) => {
			for sub in subs.iter() {
				_validate_Attribute(&sub)?;
			}
			Ok(())
		},
		_ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeList")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddResponse  ::=  [APPLICATION 9]  LDAPResult
/// ```
pub type AddResponse = LDAPResult; // DefinedType

pub fn _decode_AddResponse (el: &X690Element) -> ASN1Result<AddResponse> {
	_decode_LDAPResult(&el)
}

pub fn _encode_AddResponse (value_: &AddResponse) -> ASN1Result<X690Element> {
	|v_1: &AddResponse| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPResult(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 9; Ok(el_1) }(&value_)
}

pub fn _validate_AddResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 9 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddResponse"));
	}
	Ok(_validate_LDAPResult(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DelRequest  ::=  [APPLICATION 10]  LDAPDN
/// ```
pub type DelRequest = LDAPDN; // DefinedType

pub fn _decode_DelRequest (el: &X690Element) -> ASN1Result<DelRequest> {
	_decode_LDAPDN(&el)
}

pub fn _encode_DelRequest (value_: &DelRequest) -> ASN1Result<X690Element> {
	|v_1: &DelRequest| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPDN(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 10; Ok(el_1) }(&value_)
}

pub fn _validate_DelRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 10 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelRequest"));
	}
	Ok(_validate_LDAPDN(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DelResponse  ::=  [APPLICATION 11]  LDAPResult
/// ```
pub type DelResponse = LDAPResult; // DefinedType

pub fn _decode_DelResponse (el: &X690Element) -> ASN1Result<DelResponse> {
	_decode_LDAPResult(&el)
}

pub fn _encode_DelResponse (value_: &DelResponse) -> ASN1Result<X690Element> {
	|v_1: &DelResponse| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPResult(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 11; Ok(el_1) }(&value_)
}

pub fn _validate_DelResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 11 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelResponse"));
	}
	Ok(_validate_LDAPResult(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNRequest ::= [APPLICATION 12]  SEQUENCE {
///   entry         LDAPDN,
///   newrdn        RelativeLDAPDN,
///   deleteoldrdn  BOOLEAN,
///   newSuperior   [0]  LDAPDN OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyDNRequest {
		pub entry: LDAPDN,
		pub newrdn: RelativeLDAPDN,
		pub deleteoldrdn: BOOLEAN,
		pub newSuperior: OPTIONAL<LDAPDN>,
		pub _unrecognized: Vec<X690Element>
}
impl  ModifyDNRequest {
	pub fn new (
		entry: LDAPDN,
		newrdn: RelativeLDAPDN,
		deleteoldrdn: BOOLEAN,
		newSuperior: OPTIONAL<LDAPDN>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		ModifyDNRequest { entry, newrdn, deleteoldrdn, newSuperior, _unrecognized}
	}

}
impl TryFrom<&X690Element> for ModifyDNRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_ModifyDNRequest(el)
	}
}

pub const _rctl1_components_for_ModifyDNRequest: &[ComponentSpec; 4] = &[
	ComponentSpec::new("entry", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("newrdn", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("deleteoldrdn", false, TagSelector::tag((TagClass::UNIVERSAL, 1)), None, None),
	ComponentSpec::new("newSuperior", true, TagSelector::tag((TagClass::CONTEXT, 0)), None, None)
];

pub const _rctl2_components_for_ModifyDNRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_ModifyDNRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_ModifyDNRequest (el: &X690Element) -> ASN1Result<ModifyDNRequest> {
	|el: &X690Element| -> ASN1Result<ModifyDNRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyDNRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ModifyDNRequest,
		_eal_components_for_ModifyDNRequest,
		_rctl2_components_for_ModifyDNRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut entry: OPTIONAL<LDAPDN> = None;
	let mut newrdn: OPTIONAL<RelativeLDAPDN> = None;
	let mut deleteoldrdn: OPTIONAL<BOOLEAN> = None;
	let mut newSuperior: OPTIONAL<LDAPDN> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"entry" => entry = Some(_decode_LDAPDN(_el)?),
			"newrdn" => newrdn = Some(_decode_RelativeLDAPDN(_el)?),
			"deleteoldrdn" => deleteoldrdn = Some(BER.decode_boolean(_el)?),
			"newSuperior" => newSuperior = Some(_decode_LDAPDN(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(ModifyDNRequest{ entry: entry.unwrap(), newrdn: newrdn.unwrap(), deleteoldrdn: deleteoldrdn.unwrap(), newSuperior, _unrecognized })
}(&el)
}

pub fn _encode_ModifyDNRequest (value_: &ModifyDNRequest) -> ASN1Result<X690Element> {
	|v_1: &ModifyDNRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &ModifyDNRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(14);
	components_.push(_encode_LDAPDN(&value_.entry)?);
	components_.push(_encode_RelativeLDAPDN(&value_.newrdn)?);
	components_.push(BER.encode_boolean(&value_.deleteoldrdn)?);
	if let Some(v_) = &value_.newSuperior {
		components_.push(|v_1: &LDAPDN| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPDN(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 12; Ok(el_1) }(&value_)
}

pub fn _validate_ModifyDNRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 12 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyDNRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyDNRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ModifyDNRequest,
		_eal_components_for_ModifyDNRequest,
		_rctl2_components_for_ModifyDNRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"entry" => _validate_LDAPDN(_el)?,
			"newrdn" => _validate_RelativeLDAPDN(_el)?,
			"deleteoldrdn" => BER.validate_boolean(_el)?,
			"newSuperior" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "newSuperior"));
	}
	Ok(_validate_LDAPDN(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNResponse  ::=  [APPLICATION 13]  LDAPResult
/// ```
pub type ModifyDNResponse = LDAPResult; // DefinedType

pub fn _decode_ModifyDNResponse (el: &X690Element) -> ASN1Result<ModifyDNResponse> {
	_decode_LDAPResult(&el)
}

pub fn _encode_ModifyDNResponse (value_: &ModifyDNResponse) -> ASN1Result<X690Element> {
	|v_1: &ModifyDNResponse| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPResult(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 13; Ok(el_1) }(&value_)
}

pub fn _validate_ModifyDNResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 13 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyDNResponse"));
	}
	Ok(_validate_LDAPResult(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareRequest ::= [APPLICATION 14]  SEQUENCE {
///   entry  LDAPDN,
///   ava    AttributeValueAssertion
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CompareRequest {
		pub entry: LDAPDN,
		pub ava: AttributeValueAssertion,
		pub _unrecognized: Vec<X690Element>
}
impl  CompareRequest {
	pub fn new (
		entry: LDAPDN,
		ava: AttributeValueAssertion,
		_unrecognized: Vec<X690Element>
	) -> Self {
		CompareRequest { entry, ava, _unrecognized}
	}

}
impl TryFrom<&X690Element> for CompareRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_CompareRequest(el)
	}
}

pub const _rctl1_components_for_CompareRequest: &[ComponentSpec; 2] = &[
	ComponentSpec::new("entry", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("ava", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_CompareRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_CompareRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_CompareRequest (el: &X690Element) -> ASN1Result<CompareRequest> {
	|el: &X690Element| -> ASN1Result<CompareRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CompareRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_CompareRequest,
		_eal_components_for_CompareRequest,
		_rctl2_components_for_CompareRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut entry: OPTIONAL<LDAPDN> = None;
	let mut ava: OPTIONAL<AttributeValueAssertion> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"entry" => entry = Some(_decode_LDAPDN(_el)?),
			"ava" => ava = Some(_decode_AttributeValueAssertion(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(CompareRequest{ entry: entry.unwrap(), ava: ava.unwrap(), _unrecognized })
}(&el)
}

pub fn _encode_CompareRequest (value_: &CompareRequest) -> ASN1Result<X690Element> {
	|v_1: &CompareRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &CompareRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_LDAPDN(&value_.entry)?);
	components_.push(_encode_AttributeValueAssertion(&value_.ava)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 14; Ok(el_1) }(&value_)
}

pub fn _validate_CompareRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 14 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CompareRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CompareRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_CompareRequest,
		_eal_components_for_CompareRequest,
		_rctl2_components_for_CompareRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"entry" => _validate_LDAPDN(_el)?,
			"ava" => _validate_AttributeValueAssertion(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareResponse  ::=  [APPLICATION 15]  LDAPResult
/// ```
pub type CompareResponse = LDAPResult; // DefinedType

pub fn _decode_CompareResponse (el: &X690Element) -> ASN1Result<CompareResponse> {
	_decode_LDAPResult(&el)
}

pub fn _encode_CompareResponse (value_: &CompareResponse) -> ASN1Result<X690Element> {
	|v_1: &CompareResponse| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPResult(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 15; Ok(el_1) }(&value_)
}

pub fn _validate_CompareResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 15 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CompareResponse"));
	}
	Ok(_validate_LDAPResult(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonRequest  ::=  [APPLICATION 16]  MessageID
/// ```
pub type AbandonRequest = MessageID; // DefinedType

pub fn _decode_AbandonRequest (el: &X690Element) -> ASN1Result<AbandonRequest> {
	_decode_MessageID(&el)
}

pub fn _encode_AbandonRequest (value_: &AbandonRequest) -> ASN1Result<X690Element> {
	|v_1: &AbandonRequest| -> ASN1Result<X690Element> { let mut el_1 = _encode_MessageID(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 16; Ok(el_1) }(&value_)
}

pub fn _validate_AbandonRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 16 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AbandonRequest"));
	}
	Ok(_validate_MessageID(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedRequest ::= [APPLICATION 23]  SEQUENCE {
///   requestName   [0]  LDAPOID,
///   requestValue  [1]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtendedRequest {
		pub requestName: LDAPOID,
		pub requestValue: OPTIONAL<OCTET_STRING>,
		pub _unrecognized: Vec<X690Element>
}
impl  ExtendedRequest {
	pub fn new (
		requestName: LDAPOID,
		requestValue: OPTIONAL<OCTET_STRING>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		ExtendedRequest { requestName, requestValue, _unrecognized}
	}

}
impl TryFrom<&X690Element> for ExtendedRequest {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_ExtendedRequest(el)
	}
}

pub const _rctl1_components_for_ExtendedRequest: &[ComponentSpec; 2] = &[
	ComponentSpec::new("requestName", false, TagSelector::tag((TagClass::CONTEXT, 0)), None, None),
	ComponentSpec::new("requestValue", true, TagSelector::tag((TagClass::CONTEXT, 1)), None, None)
];

pub const _rctl2_components_for_ExtendedRequest: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_ExtendedRequest: &[ComponentSpec; 0] = &[

];

pub fn _decode_ExtendedRequest (el: &X690Element) -> ASN1Result<ExtendedRequest> {
	|el: &X690Element| -> ASN1Result<ExtendedRequest> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ExtendedRequest,
		_eal_components_for_ExtendedRequest,
		_rctl2_components_for_ExtendedRequest,
	).into_iter();
	let mut _i: usize = 0;
	let mut requestName: OPTIONAL<LDAPOID> = None;
	let mut requestValue: OPTIONAL<OCTET_STRING> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"requestName" => requestName = Some(_decode_LDAPOID(_el)?),
			"requestValue" => requestValue = Some(BER.decode_octet_string(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(ExtendedRequest{ requestName: requestName.unwrap(), requestValue, _unrecognized })
}(&el)
}

pub fn _encode_ExtendedRequest (value_: &ExtendedRequest) -> ASN1Result<X690Element> {
	|v_1: &ExtendedRequest| -> ASN1Result<X690Element> { let mut el_1 = |value_: &ExtendedRequest| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(|v_1: &LDAPOID| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPOID(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&value_.requestName)?);
	if let Some(v_) = &value_.requestValue {
		components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_octet_string(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 1; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 23; Ok(el_1) }(&value_)
}

pub fn _validate_ExtendedRequest (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 23 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedRequest"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedRequest")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ExtendedRequest,
		_eal_components_for_ExtendedRequest,
		_rctl2_components_for_ExtendedRequest,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"requestName" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "requestName"));
	}
	Ok(_validate_LDAPOID(&el)?)
}(_el)?,
			"requestValue" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "requestValue"));
	}
	Ok(BER.validate_octet_string(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedResponse ::= [APPLICATION 24]  SEQUENCE {
///   COMPONENTS OF LDAPResult,
///   responseName   [10]  LDAPOID OPTIONAL,
///   responseValue  [11]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtendedResponse {
		pub resultCode: LDAPResult_resultCode /* REPLICATED_COMPONENT */,
		pub matchedDN: LDAPDN /* REPLICATED_COMPONENT */,
		pub diagnosticMessage: LDAPString /* REPLICATED_COMPONENT */,
		pub referral: OPTIONAL<Referral> /* REPLICATED_COMPONENT */,
		pub responseName: OPTIONAL<LDAPOID>,
		pub responseValue: OPTIONAL<OCTET_STRING>,
		pub _unrecognized: Vec<X690Element>
}
impl  ExtendedResponse {
	pub fn new (
		resultCode: LDAPResult_resultCode /* REPLICATED_COMPONENT */,
		matchedDN: LDAPDN /* REPLICATED_COMPONENT */,
		diagnosticMessage: LDAPString /* REPLICATED_COMPONENT */,
		referral: OPTIONAL<Referral> /* REPLICATED_COMPONENT */,
		responseName: OPTIONAL<LDAPOID>,
		responseValue: OPTIONAL<OCTET_STRING>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		ExtendedResponse { resultCode, matchedDN, diagnosticMessage, referral, responseName, responseValue, _unrecognized}
	}

}
impl TryFrom<&X690Element> for ExtendedResponse {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_ExtendedResponse(el)
	}
}

pub const _rctl1_components_for_ExtendedResponse: &[ComponentSpec; 6] = &[
	ComponentSpec::new("resultCode", false, TagSelector::tag((TagClass::UNIVERSAL, 10)), None, None),
	ComponentSpec::new("matchedDN", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("diagnosticMessage", false, TagSelector::tag((TagClass::UNIVERSAL, 4)), None, None),
	ComponentSpec::new("referral", true, TagSelector::tag((TagClass::CONTEXT, 3)), None, None),
	ComponentSpec::new("responseName", true, TagSelector::tag((TagClass::CONTEXT, 10)), None, None),
	ComponentSpec::new("responseValue", true, TagSelector::tag((TagClass::CONTEXT, 11)), None, None)
];

pub const _rctl2_components_for_ExtendedResponse: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_ExtendedResponse: &[ComponentSpec; 0] = &[

];

pub fn _decode_ExtendedResponse (el: &X690Element) -> ASN1Result<ExtendedResponse> {
	|el: &X690Element| -> ASN1Result<ExtendedResponse> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedResponse")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ExtendedResponse,
		_eal_components_for_ExtendedResponse,
		_rctl2_components_for_ExtendedResponse,
	).into_iter();
	let mut _i: usize = 0;
	let mut resultCode: OPTIONAL<LDAPResult_resultCode> = None;
	let mut matchedDN: OPTIONAL<LDAPDN> = None;
	let mut diagnosticMessage: OPTIONAL<LDAPString> = None;
	let mut referral: OPTIONAL<Referral> = None;
	let mut responseName: OPTIONAL<LDAPOID> = None;
	let mut responseValue: OPTIONAL<OCTET_STRING> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"resultCode" => resultCode = Some(_decode_LDAPResult_resultCode(_el)?),
			"matchedDN" => matchedDN = Some(_decode_LDAPDN(_el)?),
			"diagnosticMessage" => diagnosticMessage = Some(_decode_LDAPString(_el)?),
			"referral" => referral = Some(_decode_Referral(_el)?),
			"responseName" => responseName = Some(_decode_LDAPOID(_el)?),
			"responseValue" => responseValue = Some(BER.decode_octet_string(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(ExtendedResponse{ resultCode: resultCode.unwrap(), matchedDN: matchedDN.unwrap(), diagnosticMessage: diagnosticMessage.unwrap(), referral, responseName, responseValue, _unrecognized })
}(&el)
}

pub fn _encode_ExtendedResponse (value_: &ExtendedResponse) -> ASN1Result<X690Element> {
	|v_1: &ExtendedResponse| -> ASN1Result<X690Element> { let mut el_1 = |value_: &ExtendedResponse| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(16);
	components_.push(_encode_LDAPResult_resultCode(&value_.resultCode)?);
	components_.push(_encode_LDAPDN(&value_.matchedDN)?);
	components_.push(_encode_LDAPString(&value_.diagnosticMessage)?);
	if let Some(v_) = &value_.referral {
		components_.push(|v_1: &Referral| -> ASN1Result<X690Element> { let mut el_1 = _encode_Referral(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 3; Ok(el_1) }(&v_)?);
	}
	if let Some(v_) = &value_.responseName {
		components_.push(|v_1: &LDAPOID| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPOID(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 10; Ok(el_1) }(&v_)?);
	}
	if let Some(v_) = &value_.responseValue {
		components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_octet_string(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 11; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 24; Ok(el_1) }(&value_)
}

pub fn _validate_ExtendedResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 24 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedResponse"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendedResponse")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ExtendedResponse,
		_eal_components_for_ExtendedResponse,
		_rctl2_components_for_ExtendedResponse,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"resultCode" => _validate_LDAPResult_resultCode(_el)?,
			"matchedDN" => _validate_LDAPDN(_el)?,
			"diagnosticMessage" => _validate_LDAPString(_el)?,
			"referral" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "referral"));
	}
	Ok(_validate_Referral(&el)?)
}(_el)?,
			"responseName" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "responseName"));
	}
	Ok(_validate_LDAPOID(&el)?)
}(_el)?,
			"responseValue" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 11 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "responseValue"));
	}
	Ok(BER.validate_octet_string(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IntermediateResponse ::= [APPLICATION 25]  SEQUENCE {
///   responseName   [0]  LDAPOID OPTIONAL,
///   responseValue  [1]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IntermediateResponse {
		pub responseName: OPTIONAL<LDAPOID>,
		pub responseValue: OPTIONAL<OCTET_STRING>,
		pub _unrecognized: Vec<X690Element>
}
impl  IntermediateResponse {
	pub fn new (
		responseName: OPTIONAL<LDAPOID>,
		responseValue: OPTIONAL<OCTET_STRING>,
		_unrecognized: Vec<X690Element>
	) -> Self {
		IntermediateResponse { responseName, responseValue, _unrecognized}
	}

}
impl Default for IntermediateResponse {
	fn default () -> Self {
		IntermediateResponse { responseName: None, responseValue: None, _unrecognized: vec![] }
	}
}
impl TryFrom<&X690Element> for IntermediateResponse {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_IntermediateResponse(el)
	}
}

pub const _rctl1_components_for_IntermediateResponse: &[ComponentSpec; 2] = &[
	ComponentSpec::new("responseName", true, TagSelector::tag((TagClass::CONTEXT, 0)), None, None),
	ComponentSpec::new("responseValue", true, TagSelector::tag((TagClass::CONTEXT, 1)), None, None)
];

pub const _rctl2_components_for_IntermediateResponse: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_IntermediateResponse: &[ComponentSpec; 0] = &[

];

pub fn _decode_IntermediateResponse (el: &X690Element) -> ASN1Result<IntermediateResponse> {
	|el: &X690Element| -> ASN1Result<IntermediateResponse> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntermediateResponse")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_IntermediateResponse,
		_eal_components_for_IntermediateResponse,
		_rctl2_components_for_IntermediateResponse,
	).into_iter();
	let mut _i: usize = 0;
	let mut responseName: OPTIONAL<LDAPOID> = None;
	let mut responseValue: OPTIONAL<OCTET_STRING> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"responseName" => responseName = Some(_decode_LDAPOID(_el)?),
			"responseValue" => responseValue = Some(BER.decode_octet_string(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(IntermediateResponse{ responseName, responseValue, _unrecognized })
}(&el)
}

pub fn _encode_IntermediateResponse (value_: &IntermediateResponse) -> ASN1Result<X690Element> {
	|v_1: &IntermediateResponse| -> ASN1Result<X690Element> { let mut el_1 = |value_: &IntermediateResponse| -> ASN1Result<X690Element> {
	let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	if let Some(v_) = &value_.responseName {
		components_.push(|v_1: &LDAPOID| -> ASN1Result<X690Element> { let mut el_1 = _encode_LDAPOID(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v_)?);
	}
	if let Some(v_) = &value_.responseValue {
		components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> { let mut el_1 = BER.encode_octet_string(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 1; Ok(el_1) }(&v_)?);
	}
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))
}(&v_1)?; el_1.tag.tag_class = TagClass::APPLICATION; el_1.tag.tag_number = 25; Ok(el_1) }(&value_)
}

pub fn _validate_IntermediateResponse (el: &X690Element) -> ASN1Result<()> {
	|el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 25 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntermediateResponse"));
	}
	Ok(|el: &X690Element| -> ASN1Result<()> {
	let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IntermediateResponse")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_IntermediateResponse,
		_eal_components_for_IntermediateResponse,
		_rctl2_components_for_IntermediateResponse,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"responseName" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "responseName"));
	}
	Ok(_validate_LDAPOID(&el)?)
}(_el)?,
			"responseValue" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "responseValue"));
	}
	Ok(BER.validate_octet_string(&el)?)
}(_el)?,
			_ => (),
		}
	}
	Ok(())
}(&el)?)
}(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPMessage-protocolOp ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum LDAPMessage_protocolOp {
	bindRequest(BindRequest),
	bindResponse(BindResponse),
	unbindRequest(UnbindRequest),
	searchRequest(SearchRequest),
	searchResEntry(SearchResultEntry),
	searchResDone(SearchResultDone),
	searchResRef(SearchResultReference),
	modifyRequest(ModifyRequest),
	modifyResponse(ModifyResponse),
	addRequest(AddRequest),
	addResponse(AddResponse),
	delRequest(DelRequest),
	delResponse(DelResponse),
	modDNRequest(ModifyDNRequest),
	modDNResponse(ModifyDNResponse),
	compareRequest(CompareRequest),
	compareResponse(CompareResponse),
	abandonRequest(AbandonRequest),
	extendedReq(ExtendedRequest),
	extendedResp(ExtendedResponse),
	intermediateResponse(IntermediateResponse), /* CHOICE_ALT_EXT */
	_unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for LDAPMessage_protocolOp {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_LDAPMessage_protocolOp(el)
	}
}

pub fn _decode_LDAPMessage_protocolOp (el: &X690Element) -> ASN1Result<LDAPMessage_protocolOp> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::APPLICATION, 0) => Ok(LDAPMessage_protocolOp::bindRequest(_decode_BindRequest(&el)?)),
		(TagClass::APPLICATION, 1) => Ok(LDAPMessage_protocolOp::bindResponse(_decode_BindResponse(&el)?)),
		(TagClass::APPLICATION, 2) => Ok(LDAPMessage_protocolOp::unbindRequest(_decode_UnbindRequest(&el)?)),
		(TagClass::APPLICATION, 3) => Ok(LDAPMessage_protocolOp::searchRequest(_decode_SearchRequest(&el)?)),
		(TagClass::APPLICATION, 4) => Ok(LDAPMessage_protocolOp::searchResEntry(_decode_SearchResultEntry(&el)?)),
		(TagClass::APPLICATION, 5) => Ok(LDAPMessage_protocolOp::searchResDone(_decode_SearchResultDone(&el)?)),
		(TagClass::APPLICATION, 19) => Ok(LDAPMessage_protocolOp::searchResRef(_decode_SearchResultReference(&el)?)),
		(TagClass::APPLICATION, 6) => Ok(LDAPMessage_protocolOp::modifyRequest(_decode_ModifyRequest(&el)?)),
		(TagClass::APPLICATION, 7) => Ok(LDAPMessage_protocolOp::modifyResponse(_decode_ModifyResponse(&el)?)),
		(TagClass::APPLICATION, 8) => Ok(LDAPMessage_protocolOp::addRequest(_decode_AddRequest(&el)?)),
		(TagClass::APPLICATION, 9) => Ok(LDAPMessage_protocolOp::addResponse(_decode_AddResponse(&el)?)),
		(TagClass::APPLICATION, 10) => Ok(LDAPMessage_protocolOp::delRequest(_decode_DelRequest(&el)?)),
		(TagClass::APPLICATION, 11) => Ok(LDAPMessage_protocolOp::delResponse(_decode_DelResponse(&el)?)),
		(TagClass::APPLICATION, 12) => Ok(LDAPMessage_protocolOp::modDNRequest(_decode_ModifyDNRequest(&el)?)),
		(TagClass::APPLICATION, 13) => Ok(LDAPMessage_protocolOp::modDNResponse(_decode_ModifyDNResponse(&el)?)),
		(TagClass::APPLICATION, 14) => Ok(LDAPMessage_protocolOp::compareRequest(_decode_CompareRequest(&el)?)),
		(TagClass::APPLICATION, 15) => Ok(LDAPMessage_protocolOp::compareResponse(_decode_CompareResponse(&el)?)),
		(TagClass::APPLICATION, 16) => Ok(LDAPMessage_protocolOp::abandonRequest(_decode_AbandonRequest(&el)?)),
		(TagClass::APPLICATION, 23) => Ok(LDAPMessage_protocolOp::extendedReq(_decode_ExtendedRequest(&el)?)),
		(TagClass::APPLICATION, 24) => Ok(LDAPMessage_protocolOp::extendedResp(_decode_ExtendedResponse(&el)?)),
		(TagClass::APPLICATION, 25) => Ok(LDAPMessage_protocolOp::intermediateResponse(_decode_IntermediateResponse(&el)?)),
		_ => Ok(LDAPMessage_protocolOp::_unrecognized(el.clone())),
	}
}

pub fn _encode_LDAPMessage_protocolOp (value_: &LDAPMessage_protocolOp) -> ASN1Result<X690Element> {
		match value_ {
		LDAPMessage_protocolOp::bindRequest(v) => _encode_BindRequest(&v),
		LDAPMessage_protocolOp::bindResponse(v) => _encode_BindResponse(&v),
		LDAPMessage_protocolOp::unbindRequest(v) => _encode_UnbindRequest(&v),
		LDAPMessage_protocolOp::searchRequest(v) => _encode_SearchRequest(&v),
		LDAPMessage_protocolOp::searchResEntry(v) => _encode_SearchResultEntry(&v),
		LDAPMessage_protocolOp::searchResDone(v) => _encode_SearchResultDone(&v),
		LDAPMessage_protocolOp::searchResRef(v) => _encode_SearchResultReference(&v),
		LDAPMessage_protocolOp::modifyRequest(v) => _encode_ModifyRequest(&v),
		LDAPMessage_protocolOp::modifyResponse(v) => _encode_ModifyResponse(&v),
		LDAPMessage_protocolOp::addRequest(v) => _encode_AddRequest(&v),
		LDAPMessage_protocolOp::addResponse(v) => _encode_AddResponse(&v),
		LDAPMessage_protocolOp::delRequest(v) => _encode_DelRequest(&v),
		LDAPMessage_protocolOp::delResponse(v) => _encode_DelResponse(&v),
		LDAPMessage_protocolOp::modDNRequest(v) => _encode_ModifyDNRequest(&v),
		LDAPMessage_protocolOp::modDNResponse(v) => _encode_ModifyDNResponse(&v),
		LDAPMessage_protocolOp::compareRequest(v) => _encode_CompareRequest(&v),
		LDAPMessage_protocolOp::compareResponse(v) => _encode_CompareResponse(&v),
		LDAPMessage_protocolOp::abandonRequest(v) => _encode_AbandonRequest(&v),
		LDAPMessage_protocolOp::extendedReq(v) => _encode_ExtendedRequest(&v),
		LDAPMessage_protocolOp::extendedResp(v) => _encode_ExtendedResponse(&v),
		LDAPMessage_protocolOp::intermediateResponse(v) => _encode_IntermediateResponse(&v),
		LDAPMessage_protocolOp::_unrecognized(el) => Ok(el.clone()),
	}
}

pub fn _validate_LDAPMessage_protocolOp (el: &X690Element) -> ASN1Result<()> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::APPLICATION, 0) => _validate_BindRequest(&el),
		(TagClass::APPLICATION, 1) => _validate_BindResponse(&el),
		(TagClass::APPLICATION, 2) => _validate_UnbindRequest(&el),
		(TagClass::APPLICATION, 3) => _validate_SearchRequest(&el),
		(TagClass::APPLICATION, 4) => _validate_SearchResultEntry(&el),
		(TagClass::APPLICATION, 5) => _validate_SearchResultDone(&el),
		(TagClass::APPLICATION, 19) => _validate_SearchResultReference(&el),
		(TagClass::APPLICATION, 6) => _validate_ModifyRequest(&el),
		(TagClass::APPLICATION, 7) => _validate_ModifyResponse(&el),
		(TagClass::APPLICATION, 8) => _validate_AddRequest(&el),
		(TagClass::APPLICATION, 9) => _validate_AddResponse(&el),
		(TagClass::APPLICATION, 10) => _validate_DelRequest(&el),
		(TagClass::APPLICATION, 11) => _validate_DelResponse(&el),
		(TagClass::APPLICATION, 12) => _validate_ModifyDNRequest(&el),
		(TagClass::APPLICATION, 13) => _validate_ModifyDNResponse(&el),
		(TagClass::APPLICATION, 14) => _validate_CompareRequest(&el),
		(TagClass::APPLICATION, 15) => _validate_CompareResponse(&el),
		(TagClass::APPLICATION, 16) => _validate_AbandonRequest(&el),
		(TagClass::APPLICATION, 23) => _validate_ExtendedRequest(&el),
		(TagClass::APPLICATION, 24) => _validate_ExtendedResponse(&el),
		(TagClass::APPLICATION, 25) => _validate_IntermediateResponse(&el),
		_ => Ok(()),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPResult-resultCode ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type LDAPResult_resultCode = ENUMERATED;

pub const LDAPResult_resultCode_success: LDAPResult_resultCode = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_operationsError: LDAPResult_resultCode = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_protocolError: LDAPResult_resultCode = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_timeLimitExceeded: LDAPResult_resultCode = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_sizeLimitExceeded: LDAPResult_resultCode = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_compareFalse: LDAPResult_resultCode = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_compareTrue: LDAPResult_resultCode = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_authMethodNotSupported: LDAPResult_resultCode = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_strongerAuthRequired: LDAPResult_resultCode = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_referral: LDAPResult_resultCode = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_adminLimitExceeded: LDAPResult_resultCode = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_unavailableCriticalExtension: LDAPResult_resultCode = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_confidentialityRequired: LDAPResult_resultCode = 13; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_saslBindInProgress: LDAPResult_resultCode = 14; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_noSuchAttribute: LDAPResult_resultCode = 16; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_undefinedAttributeType: LDAPResult_resultCode = 17; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_inappropriateMatching: LDAPResult_resultCode = 18; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_constraintViolation: LDAPResult_resultCode = 19; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_attributeOrValueExists: LDAPResult_resultCode = 20; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_invalidAttributeSyntax: LDAPResult_resultCode = 21; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_noSuchObject: LDAPResult_resultCode = 32; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_aliasProblem: LDAPResult_resultCode = 33; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_invalidDNSyntax: LDAPResult_resultCode = 34; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_aliasDereferencingProblem: LDAPResult_resultCode = 36; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_inappropriateAuthentication: LDAPResult_resultCode = 48; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_invalidCredentials: LDAPResult_resultCode = 49; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_insufficientAccessRights: LDAPResult_resultCode = 50; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_busy: LDAPResult_resultCode = 51; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_unavailable: LDAPResult_resultCode = 52; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_unwillingToPerform: LDAPResult_resultCode = 53; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_loopDetect: LDAPResult_resultCode = 54; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_namingViolation: LDAPResult_resultCode = 64; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_objectClassViolation: LDAPResult_resultCode = 65; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_notAllowedOnNonLeaf: LDAPResult_resultCode = 66; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_notAllowedOnRDN: LDAPResult_resultCode = 67; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_entryAlreadyExists: LDAPResult_resultCode = 68; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_objectClassModsProhibited: LDAPResult_resultCode = 69; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_affectsMultipleDSAs: LDAPResult_resultCode = 71; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_other: LDAPResult_resultCode = 80; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_LDAPResult_resultCode (el: &X690Element) -> ASN1Result<LDAPResult_resultCode> {
	BER.decode_enumerated(&el)
}

pub fn _encode_LDAPResult_resultCode (value_: &LDAPResult_resultCode) -> ASN1Result<X690Element> {
	BER.encode_enumerated(&value_)
}

pub fn _validate_LDAPResult_resultCode (el: &X690Element) -> ASN1Result<()> {
	BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRequest-scope ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type SearchRequest_scope = ENUMERATED;

pub const SearchRequest_scope_baseObject: SearchRequest_scope = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_scope_singleLevel: SearchRequest_scope = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_scope_wholeSubtree: SearchRequest_scope = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SearchRequest_scope (el: &X690Element) -> ASN1Result<SearchRequest_scope> {
	BER.decode_enumerated(&el)
}

pub fn _encode_SearchRequest_scope (value_: &SearchRequest_scope) -> ASN1Result<X690Element> {
	BER.encode_enumerated(&value_)
}

pub fn _validate_SearchRequest_scope (el: &X690Element) -> ASN1Result<()> {
	BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRequest-derefAliases ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type SearchRequest_derefAliases = ENUMERATED;

pub const SearchRequest_derefAliases_neverDerefAliases: SearchRequest_derefAliases = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_derefAliases_derefInSearching: SearchRequest_derefAliases = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_derefAliases_derefFindingBaseObj: SearchRequest_derefAliases = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_derefAliases_derefAlways: SearchRequest_derefAliases = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SearchRequest_derefAliases (el: &X690Element) -> ASN1Result<SearchRequest_derefAliases> {
	BER.decode_enumerated(&el)
}

pub fn _encode_SearchRequest_derefAliases (value_: &SearchRequest_derefAliases) -> ASN1Result<X690Element> {
	BER.encode_enumerated(&value_)
}

pub fn _validate_SearchRequest_derefAliases (el: &X690Element) -> ASN1Result<()> {
	BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubstringFilter-substrings-substring ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum SubstringFilter_substrings_substring {
	initial(AssertionValue),
	any(AssertionValue),
	final_(AssertionValue),
	_unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for SubstringFilter_substrings_substring {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_SubstringFilter_substrings_substring(el)
	}
}

pub fn _decode_SubstringFilter_substrings_substring (el: &X690Element) -> ASN1Result<SubstringFilter_substrings_substring> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(SubstringFilter_substrings_substring::initial(_decode_AssertionValue(&el)?)),
		(TagClass::CONTEXT, 1) => Ok(SubstringFilter_substrings_substring::any(_decode_AssertionValue(&el)?)),
		(TagClass::CONTEXT, 2) => Ok(SubstringFilter_substrings_substring::final_(_decode_AssertionValue(&el)?)),
		_ => Ok(SubstringFilter_substrings_substring::_unrecognized(el.clone())),
	}
}

pub fn _encode_SubstringFilter_substrings_substring (value_: &SubstringFilter_substrings_substring) -> ASN1Result<X690Element> {
		match value_ {
		SubstringFilter_substrings_substring::initial(v) => |v_1: &AssertionValue| -> ASN1Result<X690Element> { let mut el_1 = _encode_AssertionValue(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v),
		SubstringFilter_substrings_substring::any(v) => |v_1: &AssertionValue| -> ASN1Result<X690Element> { let mut el_1 = _encode_AssertionValue(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 1; Ok(el_1) }(&v),
		SubstringFilter_substrings_substring::final_(v) => |v_1: &AssertionValue| -> ASN1Result<X690Element> { let mut el_1 = _encode_AssertionValue(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 2; Ok(el_1) }(&v),
		SubstringFilter_substrings_substring::_unrecognized(el) => Ok(el.clone()),
	}
}

pub fn _validate_SubstringFilter_substrings_substring (el: &X690Element) -> ASN1Result<()> {
		match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initial"));
	}
	Ok(_validate_AssertionValue(&el)?)
}(&el),
		(TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "any"));
	}
	Ok(_validate_AssertionValue(&el)?)
}(&el),
		(TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "final"));
	}
	Ok(_validate_AssertionValue(&el)?)
}(&el),
		_ => Ok(()),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRequest-changes-change-operation ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ModifyRequest_changes_change_operation = ENUMERATED;

pub const ModifyRequest_changes_change_operation_add: ModifyRequest_changes_change_operation = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ModifyRequest_changes_change_operation_delete: ModifyRequest_changes_change_operation = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ModifyRequest_changes_change_operation_replace: ModifyRequest_changes_change_operation = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ModifyRequest_changes_change_operation (el: &X690Element) -> ASN1Result<ModifyRequest_changes_change_operation> {
	BER.decode_enumerated(&el)
}

pub fn _encode_ModifyRequest_changes_change_operation (value_: &ModifyRequest_changes_change_operation) -> ASN1Result<X690Element> {
	BER.encode_enumerated(&value_)
}

pub fn _validate_ModifyRequest_changes_change_operation (el: &X690Element) -> ASN1Result<()> {
	BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRequest-changes-change ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyRequest_changes_change {
		pub operation: ModifyRequest_changes_change_operation,
		pub modification: PartialAttribute,
		pub _unrecognized: Vec<X690Element>
}
impl  ModifyRequest_changes_change {
	pub fn new (
		operation: ModifyRequest_changes_change_operation,
		modification: PartialAttribute,
		_unrecognized: Vec<X690Element>
	) -> Self {
		ModifyRequest_changes_change { operation, modification, _unrecognized}
	}

}
impl TryFrom<&X690Element> for ModifyRequest_changes_change {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_ModifyRequest_changes_change(el)
	}
}

pub const _rctl1_components_for_ModifyRequest_changes_change: &[ComponentSpec; 2] = &[
	ComponentSpec::new("operation", false, TagSelector::tag((TagClass::UNIVERSAL, 10)), None, None),
	ComponentSpec::new("modification", false, TagSelector::tag((TagClass::UNIVERSAL, 16)), None, None)
];

pub const _rctl2_components_for_ModifyRequest_changes_change: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_ModifyRequest_changes_change: &[ComponentSpec; 0] = &[

];

pub fn _decode_ModifyRequest_changes_change (el: &X690Element) -> ASN1Result<ModifyRequest_changes_change> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyRequest-changes-change")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ModifyRequest_changes_change,
		_eal_components_for_ModifyRequest_changes_change,
		_rctl2_components_for_ModifyRequest_changes_change,
	).into_iter();
	let mut _i: usize = 0;
	let mut operation: OPTIONAL<ModifyRequest_changes_change_operation> = None;
	let mut modification: OPTIONAL<PartialAttribute> = None;
	let mut _unrecognized: Vec<X690Element> = vec![];
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"operation" => operation = Some(_decode_ModifyRequest_changes_change_operation(_el)?),
			"modification" => modification = Some(_decode_PartialAttribute(_el)?),
			_ => _unrecognized.push(_el.clone()),
		}
	}
	Ok(ModifyRequest_changes_change{ operation: operation.unwrap(), modification: modification.unwrap(), _unrecognized })

}

pub fn _encode_ModifyRequest_changes_change (value_: &ModifyRequest_changes_change) -> ASN1Result<X690Element> {
		let mut components_: Vec<X690Element> = Vec::with_capacity(12);
	components_.push(_encode_ModifyRequest_changes_change_operation(&value_.operation)?);
	components_.push(_encode_PartialAttribute(&value_.modification)?);
	Ok(X690Element::new(
		Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
		X690Value::Constructed(Arc::new([ components_, value_._unrecognized.clone() ].concat())),
	))

}

pub fn _validate_ModifyRequest_changes_change (el: &X690Element) -> ASN1Result<()> {
		let _elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ModifyRequest-changes-change")),
	};
	let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_ModifyRequest_changes_change,
		_eal_components_for_ModifyRequest_changes_change,
		_rctl2_components_for_ModifyRequest_changes_change,
	).into_iter();
	let mut _i: usize = 0;
	for _fallible_component_name in _seq_iter {
		let _component_name = _fallible_component_name?;
		let _maybe_el = _elements.get(_i);
		_i += 1;
		let _el = _maybe_el.unwrap();
		match _component_name {
			"operation" => _validate_ModifyRequest_changes_change_operation(_el)?,
			"modification" => _validate_PartialAttribute(_el)?,
			_ => (),
		}
	}
	Ok(())

}

