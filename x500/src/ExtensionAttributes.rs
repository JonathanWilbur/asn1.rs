#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # ExtensionAttributes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `ExtensionAttributes`.
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
use crate::AttributeCertificateDefinitions::*;
use crate::CertificateExtensions::*;
use crate::InformationFramework::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// A macro for concisely representing extension attributes, as defined in
/// ITU-T Recommendation X.509 (2019), Annex C.
#[macro_export]
macro_rules! ext_attr {
    ( $name:ident $ext:ident $ldapsyn:ident $id:ident $ldapname:expr ) => {
        pub fn $name () -> ATTRIBUTE {
            ATTRIBUTE {
                ldapSyntax: Some($ldapsyn()), /* OBJECT_FIELD_SETTING */
                ldapName: Some(Vec::from([String::from($ldapname)])), /* OBJECT_FIELD_SETTING */
                id: $id(), /* OBJECT_FIELD_SETTING */
                derivation: None,
                equality_match: None,
                ordering_match: None,
                substrings_match: None,
                single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
                collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
                dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
                no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
                usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
                ldapDesc: None,
                obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
            }
        }
        pub mod $name {
            use super::*;
            pub type Type = $ext::ExtnType; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
            pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
                $ext::_decode_ExtnType(el)
            }
            pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
                $ext::_encode_ExtnType(value_)
            }
            pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
                $ext::_validate_ExtnType(el)
            }
        }
    };
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionAttribute ::= SEQUENCE {
///   type            ATTRIBUTE.&id,
///   value           SET SIZE (0..1) OF SEQUENCE {
///     mandatory  [0]  BOOLEAN DEFAULT FALSE,
///     critical   [1]  BOOLEAN DEFAULT FALSE,
///     ext        [2]  EXTENSION.&ExtnType,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ExtensionAttribute {
    pub type_: OBJECT_IDENTIFIER,
    pub value: Vec<ExtensionAttribute_value_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl ExtensionAttribute {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        value: Vec<ExtensionAttribute_value_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ExtensionAttribute {
            type_,
            value,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ExtensionAttribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtensionAttribute(el)
    }
}

pub const _rctl1_components_for_ExtensionAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtensionAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtensionAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtensionAttribute(el: &X690Element) -> ASN1Result<ExtensionAttribute> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtensionAttribute")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtensionAttribute,
        _eal_components_for_ExtensionAttribute,
        _rctl2_components_for_ExtensionAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut value_: OPTIONAL<Vec<ExtensionAttribute_value_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "value" => {
                value_ = Some(
                    |el: &X690Element| -> ASN1Result<SET_OF<ExtensionAttribute_value_Item>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "value",
                                ))
                            }
                        };
                        let mut items: SET_OF<ExtensionAttribute_value_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_ExtensionAttribute_value_Item(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ExtensionAttribute {
        type_: type__.unwrap(),
        value: value_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ExtensionAttribute(value_: &ExtensionAttribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    components_.push(
        |value_: &SET_OF<ExtensionAttribute_value_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_ExtensionAttribute_value_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.value)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ExtensionAttribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtensionAttribute")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtensionAttribute,
        _eal_components_for_ExtensionAttribute,
        _rctl2_components_for_ExtensionAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
            "value" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ExtensionAttribute_value_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "value")),
                }
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

// %FIXME%: COULD_NOT_COMPILE_ASSIGNMENT extensionSyntax PARAMETERIZATION_UNSUPPORTED

// TODO: Are these valid LDAP names at all?
ext_attr!(a_authorityKeyIdentifier authorityKeyIdentifier id_asx_authorityKeyIdentifier id_ce_a_authorityKeyIdentifier "Authority Key Identifier");
ext_attr!(a_keyUsage keyUsage id_asx_keyUsage id_ce_a_keyUsage "Key Usage");
ext_attr!(a_extKeyUsage extKeyUsage id_asx_extKeyUsage id_ce_a_extKeyUsage "Extended Key Usage");
ext_attr!(a_privateKeyUsagePeriod privateKeyUsagePeriod id_asx_privateKeyUsagePeriod id_ce_a_privateKeyUsagePeriod "Private Key Usage Period");
ext_attr!(a_certificatePolicies certificatePolicies id_asx_certificatePolicies id_ce_a_certificatePolicies "Certificate Policies");
ext_attr!(a_policyMappings policyMappings id_asx_policyMappings id_ce_a_policyMappings "Policy Mappings");
ext_attr!(a_authorizationValidation authorizationValidation id_asx_authorizationValidation id_ce_a_authorizationValidation "Authorization Validation");
ext_attr!(a_subjectAltName subjectAltName id_asx_subjectAltName id_ce_a_subjectAltName "Subject Alternative Name");
ext_attr!(a_issuerAltName issuerAltName id_asx_issuerAltName id_ce_a_issuerAltName "Issuer Alternative Name");
ext_attr!(a_subjectDirectoryAttributes subjectDirectoryAttributes id_asx_subjectDirectoryAttributes id_ce_a_subjectDirectoryAttributes "Subject Directory Attributes");
ext_attr!(a_basicConstraints basicConstraints id_asx_basicConstraints id_ce_a_basicConstraints "Basic Constraints");
ext_attr!(a_nameConstraints nameConstraints id_asx_nameConstraints id_ce_a_nameConstraints "Name Constraints");
ext_attr!(a_policyConstraints policyConstraints id_asx_policyConstraints id_ce_a_policyConstraints "Policy Constraints");
ext_attr!(a_cRLNumber cRLNumber id_asx_cRLNumber id_ce_a_cRLNumber "CRL Number");
ext_attr!(a_statusReferrals statusReferrals id_asx_statusReferrals id_ce_a_statusReferrals "Status Referrals");
ext_attr!(a_cRLStreamIdentifier cRLStreamIdentifier id_asx_cRLStreamIdentifier id_ce_a_cRLStreamIdentifier "CRL stream identifier");
ext_attr!(a_orderedList orderedList id_asx_orderedList id_ce_a_orderedList "Ordered list");
ext_attr!(a_deltaInfo deltaInfo id_asx_deltaInfo id_ce_a_deltaInfo "Delta information");
ext_attr!(a_toBeRevoked toBeRevoked id_asx_toBeRevoked id_ce_a_toBeRevoked "To be revoked");
ext_attr!(a_revokedGroups revokedGroups id_asx_revokedGroups id_ce_a_revokedGroups "Revoked group of certificates");
ext_attr!(a_expiredCertsOnCRL expiredCertsOnCRL id_asx_expiredCertsOnCRL id_ce_a_expiredCertsOnCRL "Expired certificates on CRL");
ext_attr!(a_reasonCode reasonCode id_asx_reasonCode id_ce_a_reasonCode "Reason code");
ext_attr!(a_holdInstructionCode holdInstructionCode id_asx_holdInstructionCode id_ce_a_holdInstructionCode "Hold instruction code");
ext_attr!(a_invalidityDate invalidityDate id_asx_invalidityDate id_ce_a_invalidityDate "Invalidity date");
ext_attr!(a_cRLDistributionPoints cRLDistributionPoints id_asx_cRLDistributionPoints id_ce_a_cRLDistributionPoints "CRL distribution points");
ext_attr!(a_issuingDistributionPoint issuingDistributionPoint id_asx_issuingDistributionPoint id_ce_a_issuingDistributionPoint "Issuing distribution point");
ext_attr!(a_certificateIssuer certificateIssuer id_asx_certificateIssuer id_ce_a_certificateIssuer "Certificate issuer");
ext_attr!(a_deltaCRLIndicator deltaCRLIndicator id_asx_deltaCRLIndicator id_ce_a_deltaCRLIndicator "Delta CRL indicator");
ext_attr!(a_baseUpdateTime baseUpdateTime id_asx_baseUpdateTime id_ce_a_baseUpdateTime "Base update time");
ext_attr!(a_freshestCRL freshestCRL id_asx_freshestCRL id_ce_a_freshestCRL "Freshest CRL");
ext_attr!(a_timeSpecification timeSpecification id_asx_timeSpecification id_ce_a_timeSpecification "Time specification");
ext_attr!(a_targetingInformation targetingInformation id_asx_targetingInformation id_ce_a_targetingInformation "Targeting information");
ext_attr!(a_userNotice userNotice id_asx_userNotice id_ce_a_userNotice "User notice");
ext_attr!(a_acceptablePrivilegePolicies acceptablePrivilegePolicies id_asx_acceptablePrivilegePolicies id_ce_a_acceptablePrivilegePolicies "Acceptable Privilege Policies");
ext_attr!(a_singleUse singleUse id_asx_singleUse id_ce_a_singleUse "Single use");
ext_attr!(a_groupAC groupAC id_asx_groupAC id_ce_a_groupAC "Group attribute certificate");
ext_attr!(a_noRevAvail noRevAvail id_asx_noRevAvail id_ce_a_noRevAvail "No revocation information available");
ext_attr!(a_sOAIdentifier sOAIdentifier id_asx_sOAIdentifier id_ce_a_sOAIdentifier "SOA identifier");
ext_attr!(a_attributeDescriptor attributeDescriptor id_asx_attributeDescriptor id_ce_a_attributeDescriptor "Attribute descriptor");
ext_attr!(a_roleSpecCertIdentifier roleSpecCertIdentifier id_asx_roleSpecCertIdentifier id_ce_a_roleSpecCertIdentifier "Role specification certificate identifier");
ext_attr!(a_basicAttConstraints basicAttConstraints id_asx_basicAttConstraints id_ce_a_basicAttConstraints "Basic attribute constraints");
ext_attr!(a_delegatedNameConstraints delegatedNameConstraints id_asx_delegatedNameConstraints id_ce_a_delegatedNameConstraints "Delegated name constraints");
ext_attr!(a_acceptableCertPolicies acceptableCertPolicies id_asx_acceptableCertPolicies id_ce_a_acceptableCertPolicies "Acceptable certificate policiesGroup attribute certificate");
ext_attr!(a_authorityAttributeIdentifier authorityAttributeIdentifier id_asx_authorityAttributeIdentifier id_ce_a_authorityAttributeIdentifier "Authority attribute identifier");
ext_attr!(a_indirectIssuer indirectIssuer id_asx_indirectIssuer id_ce_a_indirectIssuer "Indirect issuer");
ext_attr!(a_issuedOnBehalfOf issuedOnBehalfOf id_asx_issuedOnBehalfOf id_ce_a_issuedOnBehalfOf "Issued on behalf of");
ext_attr!(a_noAssertion noAssertion id_asx_noAssertion id_ce_a_noAssertion "No assertion");
ext_attr!(a_allowedAttributeAssignments allowedAttributeAssignments id_asx_allowedAttributeAssignments id_ce_a_allowedAttributeAssignments "Allowed attribute assignments");
ext_attr!(a_attributeMappings attributeMappings id_asx_attributeMappings id_ce_a_attributeMappings "Attribute mappings");
ext_attr!(a_holderNameConstraints holderNameConstraints id_asx_holderNameConstraints id_ce_a_holderNameConstraints "Holder name constraints");
ext_attr!(a_aAissuingDistributionPoint aAissuingDistributionPoint id_asx_aAissuingDistributionPoint id_ce_a_aAissuingDistributionPoint "AA issuing distribution point");
ext_attr!(a_protRestrict protRestrict id_asx_protRestrict id_ce_a_protRestrict "Protocol restriction");
ext_attr!(a_subjectAltPublicKeyInfo subjectAltPublicKeyInfo id_asx_subjectAltPublicKeyInfo id_ce_a_subjectAltPublicKeyInfo "Subject alternative public key info");
ext_attr!(a_altSignatureAlgorithm altSignatureAlgorithm id_asx_altSignatureAlgorithm id_ce_a_altSignatureAlgorithm "Alternative signature algorithm");
ext_attr!(a_altSignatureValue altSignatureValue id_asx_altSignatureValue id_ce_a_altSignatureValue "Alternative signature value");

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectDirectoryAttributes         OBJECT IDENTIFIER ::= {id-ce 9 1}
/// ```
///
///
pub fn id_ce_a_subjectDirectoryAttributes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([9, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectKeyIdentifier               OBJECT IDENTIFIER ::= {id-ce 14 1}
/// ```
///
///
pub fn id_ce_a_subjectKeyIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([14, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-keyUsage                           OBJECT IDENTIFIER ::= {id-ce 15 1}
/// ```
///
///
pub fn id_ce_a_keyUsage() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([15, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-privateKeyUsagePeriod              OBJECT IDENTIFIER ::= {id-ce 16 1}
/// ```
///
///
pub fn id_ce_a_privateKeyUsagePeriod() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([16, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectAltName                     OBJECT IDENTIFIER ::= {id-ce 17 1}
/// ```
///
///
pub fn id_ce_a_subjectAltName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([17, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-issuerAltName                      OBJECT IDENTIFIER ::= {id-ce 18 1}
/// ```
///
///
pub fn id_ce_a_issuerAltName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([18, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-basicConstraints                   OBJECT IDENTIFIER ::= {id-ce 19 1}
/// ```
///
///
pub fn id_ce_a_basicConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([19, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLNumber                          OBJECT IDENTIFIER ::= {id-ce 20 1}
/// ```
///
///
pub fn id_ce_a_cRLNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([20, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-reasonCode                         OBJECT IDENTIFIER ::= {id-ce 21 1}
/// ```
///
///
pub fn id_ce_a_reasonCode() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([21, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-holdInstructionCode                OBJECT IDENTIFIER ::= {id-ce 23 1}
/// ```
///
///
pub fn id_ce_a_holdInstructionCode() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([23, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-invalidityDate                     OBJECT IDENTIFIER ::= {id-ce 24 1}
/// ```
///
///
pub fn id_ce_a_invalidityDate() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([24, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-deltaCRLIndicator                  OBJECT IDENTIFIER ::= {id-ce 27 1}
/// ```
///
///
pub fn id_ce_a_deltaCRLIndicator() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([27, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-issuingDistributionPoint           OBJECT IDENTIFIER ::= {id-ce 28 1}
/// ```
///
///
pub fn id_ce_a_issuingDistributionPoint() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([28, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-certificateIssuer                  OBJECT IDENTIFIER ::= {id-ce 29 1}
/// ```
///
///
pub fn id_ce_a_certificateIssuer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([29, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-nameConstraints                    OBJECT IDENTIFIER ::= {id-ce 30 1}
/// ```
///
///
pub fn id_ce_a_nameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([30, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLDistributionPoints              OBJECT IDENTIFIER ::= {id-ce 31 1}
/// ```
///
///
pub fn id_ce_a_cRLDistributionPoints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([31, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-certificatePolicies                OBJECT IDENTIFIER ::= {id-ce 32 1}
/// ```
///
///
pub fn id_ce_a_certificatePolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([32, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-policyMappings                     OBJECT IDENTIFIER ::= {id-ce 33 1}
/// ```
///
///
pub fn id_ce_a_policyMappings() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([33, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-authorityKeyIdentifier             OBJECT IDENTIFIER ::= {id-ce 35 1}
/// ```
///
///
pub fn id_ce_a_authorityKeyIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([35, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-policyConstraints                  OBJECT IDENTIFIER ::= {id-ce 36 1}
/// ```
///
///
pub fn id_ce_a_policyConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([36, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-extKeyUsage                        OBJECT IDENTIFIER ::= {id-ce 37 1}
/// ```
///
///
pub fn id_ce_a_extKeyUsage() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([37, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-authorityAttributeIdentifier       OBJECT IDENTIFIER ::= {id-ce 38 1}
/// ```
///
///
pub fn id_ce_a_authorityAttributeIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([38, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-roleSpecCertIdentifier             OBJECT IDENTIFIER ::= {id-ce 39 1}
/// ```
///
///
pub fn id_ce_a_roleSpecCertIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([39, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLStreamIdentifier                OBJECT IDENTIFIER ::= {id-ce 40 1}
/// ```
///
///
pub fn id_ce_a_cRLStreamIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([40, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-basicAttConstraints                OBJECT IDENTIFIER ::= {id-ce 41 1}
/// ```
///
///
pub fn id_ce_a_basicAttConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([41, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-delegatedNameConstraints           OBJECT IDENTIFIER ::= {id-ce 42 1}
/// ```
///
///
pub fn id_ce_a_delegatedNameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([42, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-timeSpecification                  OBJECT IDENTIFIER ::= {id-ce 43 1}
/// ```
///
///
pub fn id_ce_a_timeSpecification() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([43, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLScope                           OBJECT IDENTIFIER ::= {id-ce 44 1}
/// ```
///
///
pub fn id_ce_a_cRLScope() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([44, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-statusReferrals                    OBJECT IDENTIFIER ::= {id-ce 45 1}
/// ```
///
///
pub fn id_ce_a_statusReferrals() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([45, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-freshestCRL                        OBJECT IDENTIFIER ::= {id-ce 46 1}
/// ```
///
///
pub fn id_ce_a_freshestCRL() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([46, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-orderedList                        OBJECT IDENTIFIER ::= {id-ce 47 1}
/// ```
///
///
pub fn id_ce_a_orderedList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([47, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-attributeDescriptor                OBJECT IDENTIFIER ::= {id-ce 48 1}
/// ```
///
///
pub fn id_ce_a_attributeDescriptor() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([48, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-userNotice                         OBJECT IDENTIFIER ::= {id-ce 49 1}
/// ```
///
///
pub fn id_ce_a_userNotice() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([49, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-sOAIdentifier                      OBJECT IDENTIFIER ::= {id-ce 50 1}
/// ```
///
///
pub fn id_ce_a_sOAIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([50, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-baseUpdateTime                     OBJECT IDENTIFIER ::= {id-ce 51 1}
/// ```
///
///
pub fn id_ce_a_baseUpdateTime() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([51, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-acceptableCertPolicies             OBJECT IDENTIFIER ::= {id-ce 52 1}
/// ```
///
///
pub fn id_ce_a_acceptableCertPolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([52, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-deltaInfo                          OBJECT IDENTIFIER ::= {id-ce 53 1}
/// ```
///
///
pub fn id_ce_a_deltaInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([53, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-inhibitAnyPolicy                   OBJECT IDENTIFIER ::= {id-ce 54 1}
/// ```
///
///
pub fn id_ce_a_inhibitAnyPolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([54, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-targetingInformation               OBJECT IDENTIFIER ::= {id-ce 55 1}
/// ```
///
///
pub fn id_ce_a_targetingInformation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([55, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-noRevAvail                         OBJECT IDENTIFIER ::= {id-ce 56 1}
/// ```
///
///
pub fn id_ce_a_noRevAvail() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([56, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-acceptablePrivilegePolicies        OBJECT IDENTIFIER ::= {id-ce 57 1}
/// ```
///
///
pub fn id_ce_a_acceptablePrivilegePolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([57, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-toBeRevoked                        OBJECT IDENTIFIER ::= {id-ce 58 1}
/// ```
///
///
pub fn id_ce_a_toBeRevoked() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([58, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-revokedGroups                      OBJECT IDENTIFIER ::= {id-ce 59 1}
/// ```
///
///
pub fn id_ce_a_revokedGroups() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([59, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-expiredCertsOnCRL                  OBJECT IDENTIFIER ::= {id-ce 60 1}
/// ```
///
///
pub fn id_ce_a_expiredCertsOnCRL() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([60, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-indirectIssuer                     OBJECT IDENTIFIER ::= {id-ce 61 1}
/// ```
///
///
pub fn id_ce_a_indirectIssuer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([61, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-noAssertion                        OBJECT IDENTIFIER ::= {id-ce 62 1}
/// ```
///
///
pub fn id_ce_a_noAssertion() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([62, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-aAissuingDistributionPoint         OBJECT IDENTIFIER ::= {id-ce 63 1}
/// ```
///
///
pub fn id_ce_a_aAissuingDistributionPoint() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([63, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-issuedOnBehalfOf                   OBJECT IDENTIFIER ::= {id-ce 64 1}
/// ```
///
///
pub fn id_ce_a_issuedOnBehalfOf() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([64, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-singleUse                          OBJECT IDENTIFIER ::= {id-ce 65 1}
/// ```
///
///
pub fn id_ce_a_singleUse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([65, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-groupAC                            OBJECT IDENTIFIER ::= {id-ce 66 1}
/// ```
///
///
pub fn id_ce_a_groupAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([66, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-allowedAttributeAssignments        OBJECT IDENTIFIER ::= {id-ce 67 1}
/// ```
///
///
pub fn id_ce_a_allowedAttributeAssignments() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([67, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-attributeMappings                  OBJECT IDENTIFIER ::= {id-ce 68 1}
/// ```
///
///
pub fn id_ce_a_attributeMappings() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([68, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-holderNameConstraints              OBJECT IDENTIFIER ::= {id-ce 69 1}
/// ```
///
///
pub fn id_ce_a_holderNameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([69, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-authorizationValidation            OBJECT IDENTIFIER ::= {id-ce 70 1}
/// ```
///
///
pub fn id_ce_a_authorizationValidation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([70, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-protRestrict                       OBJECT IDENTIFIER ::= {id-ce 71 1}
/// ```
///
///
pub fn id_ce_a_protRestrict() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([71, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectAltPublicKeyInfo            OBJECT IDENTIFIER ::= {id-ce 72 1}
/// ```
///
///
pub fn id_ce_a_subjectAltPublicKeyInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([72, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-altSignatureAlgorithm              OBJECT IDENTIFIER ::= {id-ce 73 1}
/// ```
///
///
pub fn id_ce_a_altSignatureAlgorithm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([73, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-altSignatureValue                  OBJECT IDENTIFIER ::= {id-ce 74 1}
/// ```
///
///
pub fn id_ce_a_altSignatureValue() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([74, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectDirectoryAttributes          OBJECT IDENTIFIER ::= {id-ce 9 2}
/// ```
///
///
pub fn id_asx_subjectDirectoryAttributes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([9, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectKeyIdentifier                OBJECT IDENTIFIER ::= {id-ce 14 2}
/// ```
///
///
pub fn id_asx_subjectKeyIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([14, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-keyUsage                            OBJECT IDENTIFIER ::= {id-ce 15 2}
/// ```
///
///
pub fn id_asx_keyUsage() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([15, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-privateKeyUsagePeriod               OBJECT IDENTIFIER ::= {id-ce 16 2}
/// ```
///
///
pub fn id_asx_privateKeyUsagePeriod() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([16, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectAltName                      OBJECT IDENTIFIER ::= {id-ce 17 2}
/// ```
///
///
pub fn id_asx_subjectAltName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([17, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-issuerAltName                       OBJECT IDENTIFIER ::= {id-ce 18 2}
/// ```
///
///
pub fn id_asx_issuerAltName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([18, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-basicConstraints                    OBJECT IDENTIFIER ::= {id-ce 19 2}
/// ```
///
///
pub fn id_asx_basicConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([19, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLNumber                           OBJECT IDENTIFIER ::= {id-ce 20 2}
/// ```
///
///
pub fn id_asx_cRLNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([20, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-reasonCode                          OBJECT IDENTIFIER ::= {id-ce 21 2}
/// ```
///
///
pub fn id_asx_reasonCode() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([21, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-holdInstructionCode                 OBJECT IDENTIFIER ::= {id-ce 23 2}
/// ```
///
///
pub fn id_asx_holdInstructionCode() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([23, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-invalidityDate                      OBJECT IDENTIFIER ::= {id-ce 24 2}
/// ```
///
///
pub fn id_asx_invalidityDate() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([24, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-deltaCRLIndicator                   OBJECT IDENTIFIER ::= {id-ce 27 2}
/// ```
///
///
pub fn id_asx_deltaCRLIndicator() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([27, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-issuingDistributionPoint            OBJECT IDENTIFIER ::= {id-ce 28 2}
/// ```
///
///
pub fn id_asx_issuingDistributionPoint() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([28, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-certificateIssuer                   OBJECT IDENTIFIER ::= {id-ce 29 2}
/// ```
///
///
pub fn id_asx_certificateIssuer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([29, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-nameConstraints                     OBJECT IDENTIFIER ::= {id-ce 30 2}
/// ```
///
///
pub fn id_asx_nameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([30, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLDistributionPoints               OBJECT IDENTIFIER ::= {id-ce 31 2}
/// ```
///
///
pub fn id_asx_cRLDistributionPoints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([31, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-certificatePolicies                 OBJECT IDENTIFIER ::= {id-ce 32 2}
/// ```
///
///
pub fn id_asx_certificatePolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([32, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-policyMappings                      OBJECT IDENTIFIER ::= {id-ce 33 2}
/// ```
///
///
pub fn id_asx_policyMappings() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([33, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-authorityKeyIdentifier              OBJECT IDENTIFIER ::= {id-ce 35 2}
/// ```
///
///
pub fn id_asx_authorityKeyIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([35, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-policyConstraints                   OBJECT IDENTIFIER ::= {id-ce 36 2}
/// ```
///
///
pub fn id_asx_policyConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([36, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-extKeyUsage                         OBJECT IDENTIFIER ::= {id-ce 37 2}
/// ```
///
///
pub fn id_asx_extKeyUsage() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([37, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-authorityAttributeIdentifier        OBJECT IDENTIFIER ::= {id-ce 38 2}
/// ```
///
///
pub fn id_asx_authorityAttributeIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([38, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-roleSpecCertIdentifier              OBJECT IDENTIFIER ::= {id-ce 39 2}
/// ```
///
///
pub fn id_asx_roleSpecCertIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([39, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLStreamIdentifier                 OBJECT IDENTIFIER ::= {id-ce 40 2}
/// ```
///
///
pub fn id_asx_cRLStreamIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([40, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-basicAttConstraints                 OBJECT IDENTIFIER ::= {id-ce 41 2}
/// ```
///
///
pub fn id_asx_basicAttConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([41, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-delegatedNameConstraints            OBJECT IDENTIFIER ::= {id-ce 42 2}
/// ```
///
///
pub fn id_asx_delegatedNameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([42, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-timeSpecification                   OBJECT IDENTIFIER ::= {id-ce 43 2}
/// ```
///
///
pub fn id_asx_timeSpecification() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([43, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLScope                            OBJECT IDENTIFIER ::= {id-ce 44 2}
/// ```
///
///
pub fn id_asx_cRLScope() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([44, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-statusReferrals                     OBJECT IDENTIFIER ::= {id-ce 45 2}
/// ```
///
///
pub fn id_asx_statusReferrals() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([45, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-freshestCRL                         OBJECT IDENTIFIER ::= {id-ce 46 2}
/// ```
///
///
pub fn id_asx_freshestCRL() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([46, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-orderedList                         OBJECT IDENTIFIER ::= {id-ce 47 2}
/// ```
///
///
pub fn id_asx_orderedList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([47, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-attributeDescriptor                 OBJECT IDENTIFIER ::= {id-ce 48 2}
/// ```
///
///
pub fn id_asx_attributeDescriptor() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([48, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-userNotice                          OBJECT IDENTIFIER ::= {id-ce 49 2}
/// ```
///
///
pub fn id_asx_userNotice() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([49, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-sOAIdentifier                       OBJECT IDENTIFIER ::= {id-ce 50 2}
/// ```
///
///
pub fn id_asx_sOAIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([50, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-baseUpdateTime                      OBJECT IDENTIFIER ::= {id-ce 51 2}
/// ```
///
///
pub fn id_asx_baseUpdateTime() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([51, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-acceptableCertPolicies              OBJECT IDENTIFIER ::= {id-ce 52 2}
/// ```
///
///
pub fn id_asx_acceptableCertPolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([52, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-deltaInfo                           OBJECT IDENTIFIER ::= {id-ce 53 2}
/// ```
///
///
pub fn id_asx_deltaInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([53, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-inhibitAnyPolicy                    OBJECT IDENTIFIER ::= {id-ce 54 2}
/// ```
///
///
pub fn id_asx_inhibitAnyPolicy() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([54, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-targetingInformation                OBJECT IDENTIFIER ::= {id-ce 55 2}
/// ```
///
///
pub fn id_asx_targetingInformation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([55, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-noRevAvail                          OBJECT IDENTIFIER ::= {id-ce 56 2}
/// ```
///
///
pub fn id_asx_noRevAvail() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([56, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-acceptablePrivilegePolicies         OBJECT IDENTIFIER ::= {id-ce 57 2}
/// ```
///
///
pub fn id_asx_acceptablePrivilegePolicies() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([57, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-toBeRevoked                         OBJECT IDENTIFIER ::= {id-ce 58 2}
/// ```
///
///
pub fn id_asx_toBeRevoked() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([58, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-revokedGroups                       OBJECT IDENTIFIER ::= {id-ce 59 2}
/// ```
///
///
pub fn id_asx_revokedGroups() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([59, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-expiredCertsOnCRL                   OBJECT IDENTIFIER ::= {id-ce 60 2}
/// ```
///
///
pub fn id_asx_expiredCertsOnCRL() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([60, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-indirectIssuer                      OBJECT IDENTIFIER ::= {id-ce 61 2}
/// ```
///
///
pub fn id_asx_indirectIssuer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([61, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-noAssertion                         OBJECT IDENTIFIER ::= {id-ce 62 2}
/// ```
///
///
pub fn id_asx_noAssertion() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([62, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-aAissuingDistributionPoint          OBJECT IDENTIFIER ::= {id-ce 63 2}
/// ```
///
///
pub fn id_asx_aAissuingDistributionPoint() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([63, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-issuedOnBehalfOf                    OBJECT IDENTIFIER ::= {id-ce 64 2}
/// ```
///
///
pub fn id_asx_issuedOnBehalfOf() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([64, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-singleUse                           OBJECT IDENTIFIER ::= {id-ce 65 2}
/// ```
///
///
pub fn id_asx_singleUse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([65, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-groupAC                             OBJECT IDENTIFIER ::= {id-ce 66 2}
/// ```
///
///
pub fn id_asx_groupAC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([66, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-allowedAttributeAssignments         OBJECT IDENTIFIER ::= {id-ce 67 2}
/// ```
///
///
pub fn id_asx_allowedAttributeAssignments() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([67, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-attributeMappings                   OBJECT IDENTIFIER ::= {id-ce 68 2}
/// ```
///
///
pub fn id_asx_attributeMappings() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([68, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-holderNameConstraints               OBJECT IDENTIFIER ::= {id-ce 69 2}
/// ```
///
///
pub fn id_asx_holderNameConstraints() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([69, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-authorizationValidation             OBJECT IDENTIFIER ::= {id-ce 70 2}
/// ```
///
///
pub fn id_asx_authorizationValidation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([70, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-protRestrict                        OBJECT IDENTIFIER ::= {id-ce 71 2}
/// ```
///
///
pub fn id_asx_protRestrict() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([71, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectAltPublicKeyInfo             OBJECT IDENTIFIER ::= {id-ce 72 2}
/// ```
///
///
pub fn id_asx_subjectAltPublicKeyInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([72, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-altSignatureAlgorithm               OBJECT IDENTIFIER ::= {id-ce 73 2}
/// ```
///
///
pub fn id_asx_altSignatureAlgorithm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([73, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-altSignatureValue                   OBJECT IDENTIFIER ::= {id-ce 74 2}
/// ```
///
///
pub fn id_asx_altSignatureValue() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_ce().0, Vec::<u32>::from([74, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionAttribute-value-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct ExtensionAttribute_value_Item {
    pub mandatory: OPTIONAL<BOOLEAN>,
    pub critical: OPTIONAL<BOOLEAN>,
    pub ext: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl ExtensionAttribute_value_Item {
    pub fn new(
        mandatory: OPTIONAL<BOOLEAN>,
        critical: OPTIONAL<BOOLEAN>,
        ext: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ExtensionAttribute_value_Item {
            mandatory,
            critical,
            ext,
            _unrecognized,
        }
    }
    pub fn _default_value_for_mandatory() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_critical() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for ExtensionAttribute_value_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtensionAttribute_value_Item(el)
    }
}

pub const _rctl1_components_for_ExtensionAttribute_value_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "mandatory",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "critical",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ext",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtensionAttribute_value_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtensionAttribute_value_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtensionAttribute_value_Item(
    el: &X690Element,
) -> ASN1Result<ExtensionAttribute_value_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ExtensionAttribute-value-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtensionAttribute_value_Item,
        _eal_components_for_ExtensionAttribute_value_Item,
        _rctl2_components_for_ExtensionAttribute_value_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut mandatory_: OPTIONAL<BOOLEAN> = None;
    let mut critical_: OPTIONAL<BOOLEAN> = None;
    let mut ext_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mandatory" => {
                mandatory_ = Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                    Ok(BER.decode_boolean(&el.inner()?)?)
                }(_el)?)
            }
            "critical" => {
                critical_ = Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                    Ok(BER.decode_boolean(&el.inner()?)?)
                }(_el)?)
            }
            "ext" => {
                ext_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ExtensionAttribute_value_Item {
        mandatory: mandatory_,
        critical: critical_,
        ext: ext_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ExtensionAttribute_value_Item(
    value_: &ExtensionAttribute_value_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.mandatory {
        if *v_ != ExtensionAttribute_value_Item::_default_value_for_mandatory() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&BER.encode_boolean(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.critical {
        if *v_ != ExtensionAttribute_value_Item::_default_value_for_critical() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&BER.encode_boolean(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 2),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_.ext)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ExtensionAttribute_value_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ExtensionAttribute-value-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtensionAttribute_value_Item,
        _eal_components_for_ExtensionAttribute_value_Item,
        _rctl2_components_for_ExtensionAttribute_value_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mandatory" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mandatory")
                    );
                }
                Ok(BER.validate_boolean(&el.inner()?)?)
            }(_el)?,
            "critical" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "critical")
                    );
                }
                Ok(BER.validate_boolean(&el.inner()?)?)
            }(_el)?,
            "ext" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ext"));
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}
