use std::str::FromStr;

use crate::InformationFramework::{AttributeType, DistinguishedName, Name, RelativeDistinguishedName};
use crate::CertificateExtensions::GeneralName;
use wildboar_asn1::{
    ASN1Error,
    TagClass,
    UNIV_TAG_BOOLEAN,
    UNIV_TAG_INTEGER,
    UNIV_TAG_BIT_STRING,
    UNIV_TAG_OCTET_STRING,
    UNIV_TAG_NULL,
    UNIV_TAG_OBJECT_IDENTIFIER,
    UNIV_TAG_OBJECT_DESCRIPTOR,
    UNIV_TAG_REAL,
    UNIV_TAG_ENUMERATED,
    UNIV_TAG_UTF8_STRING,
    UNIV_TAG_RELATIVE_OID,
    UNIV_TAG_TIME,
    UNIV_TAG_NUMERIC_STRING,
    UNIV_TAG_PRINTABLE_STRING,
    UNIV_TAG_IA5_STRING,
    UNIV_TAG_UTC_TIME,
    UNIV_TAG_GENERALIZED_TIME,
    UNIV_TAG_GRAPHIC_STRING,
    UNIV_TAG_VISIBLE_STRING,
    UNIV_TAG_GENERAL_STRING,
    UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_BMP_STRING,
    UNIV_TAG_DATE,
    UNIV_TAG_TIME_OF_DAY,
    UNIV_TAG_DATE_TIME,
    UNIV_TAG_DURATION,
    UNIV_TAG_OID_IRI,
    UNIV_TAG_RELATIVE_OID_IRI, OBJECT_IDENTIFIER,
};
use x690::{
    X690Element,
    x690_write_tlv,
    BER,
    X690Codec,
};
use wildboar_asn1::utils::read_i64;
use crate::SelectedAttributeTypes::{
    id_at_businessCategory,
    id_at_collectiveFacsimileTelephoneNumber,
    id_at_collectiveInternationalISDNNumber,
    id_at_collectiveLocalityName,
    id_at_collectiveOrganizationalUnitName,
    id_at_collectiveOrganizationName,
    id_at_collectivePhysicalDeliveryOfficeName,
    id_at_collectivePostalAddress,
    id_at_collectivePostalCode,
    id_at_collectivePostOfficeBox,
    id_at_collectiveStateOrProvinceName,
    id_at_collectiveStreetAddress,
    id_at_collectiveTelephoneNumber,
    id_at_collectiveTelexNumber,
    id_at_commonName,
    id_at_communicationsNetwork,
    id_at_communicationsService,
    id_at_contentUrl,
    id_at_countryCode3c,
    id_at_countryCode3n,
    id_at_countryName,
    id_at_description,
    id_at_destinationIndicator,
    id_at_distinguishedName,
    id_at_dmdName,
    id_at_dnQualifier,
    id_at_dnsName,
    id_at_enhancedSearchGuide,
    id_at_epc,
    id_at_epcFormat,
    id_at_epcInUrn,
    id_at_facsimileTelephoneNumber,
    id_at_generationQualifier,
    id_at_givenName,
    id_at_houseIdentifier,
    id_at_initials,
    id_at_intEmail,
    id_at_internationalISDNNumber,
    id_at_jid,
    id_at_knowledgeInformation,
    id_at_ldapUrl,
    id_at_localityName,
    id_at_member,
    id_at_name,
    id_at_objectIdentifier,
    id_oidC,
    id_oidC1,
    id_oidC2,
    id_at_organizationalUnitName,
    id_at_organizationIdentifier,
    id_at_organizationName,
    id_at_owner,
    id_at_physicalDeliveryOfficeName,
    id_at_postalAddress,
    id_at_postalCode,
    id_at_postOfficeBox,
    id_at_preferredDeliveryMethod,
    id_at_presentationAddress,
    id_at_protocolInformation,
    id_at_pseudonym,
    id_at_registeredAddress,
    id_at_roleOccupant,
    id_at_searchGuide,
    id_at_seeAlso,
    id_at_serialNumber,
    id_at_stateOrProvinceName,
    id_at_streetAddress,
    id_at_supportedApplicationContext,
    id_at_surname,
    id_at_tagAfi,
    id_at_tagLocation,
    id_at_tagOid,
    id_at_telephoneNumber,
    id_at_telexNumber,
    id_at_title,
    id_at_uii,
    id_at_uiiFormat,
    id_at_uiiInUrn,
    id_at_uniqueIdentifier,
    id_at_uniqueMember,
    id_at_uri,
    id_at_url,
    id_at_urn,
    id_at_urnC,
    id_at_utmCoordinates,
    id_at_uuidpair,
    id_at_x121Address,
    id_coat_uid,
    id_coat_dc,
};
use crate::AttributeCertificateDefinitions::{
    id_at_role,
    id_at_permission,
    id_at_attributeCertificate,
    id_at_aACertificate,
    id_at_attributeDescriptorCertificate,
    id_at_attributeCertificateRevocationList,
    id_at_eeAttrCertificateRevocationList,
    id_at_attributeAuthorityRevocationList,
    id_at_delegationPath,
    id_at_privPolicy,
    id_at_protPrivPolicy,
    id_at_xmlPrivPolicy,
};
use crate::AuthenticationFramework::{
    id_at_userCertificate,
    id_at_cAcertificate,
    id_at_crossCertificatePair,
    id_at_certificateRevocationList,
    id_at_eepkCertificateRevocationList,
    id_at_authorityRevocationList,
    id_at_deltaRevocationList,
    id_at_supportedAlgorithms,
    id_at_certificationPracticeStmt,
    id_at_certificatePolicy,
    id_at_pkiPath,
    id_at_userPassword,
};
use crate::BasicAccessControl::{
    id_aca_accessControlScheme,
    id_aca_prescriptiveACI,
    id_aca_entryACI,
    id_aca_subentryACI,
};
use crate::DirectoryAbstractService::id_at_family_information;
use crate::DSAOperationalAttributeTypes::{
    id_doa_dseType,
    id_doa_myAccessPoint,
    id_doa_superiorKnowledge,
    id_doa_specificKnowledge,
    id_doa_nonSpecificKnowledge,
    id_doa_supplierKnowledge,
    id_doa_consumerKnowledge,
    id_doa_secondaryShadows,
    id_doa_ditBridgeKnowledge,
};
use crate::EnhancedSecurity::{id_at_clearance, id_at_attributeIntegrityInfo};
use crate::ExtensionAttributes::{
    id_ce_a_subjectDirectoryAttributes,
    id_ce_a_keyUsage,
    id_ce_a_privateKeyUsagePeriod,
    id_ce_a_subjectAltName,
    id_ce_a_issuerAltName,
    id_ce_a_basicConstraints,
    id_ce_a_cRLNumber,
    id_ce_a_reasonCode,
    id_ce_a_holdInstructionCode,
    id_ce_a_invalidityDate,
    id_ce_a_deltaCRLIndicator,
    id_ce_a_issuingDistributionPoint,
    id_ce_a_certificateIssuer,
    id_ce_a_nameConstraints,
    id_ce_a_cRLDistributionPoints,
    id_ce_a_certificatePolicies,
    id_ce_a_policyMappings,
    id_ce_a_authorityKeyIdentifier,
    id_ce_a_policyConstraints,
    id_ce_a_extKeyUsage,
    id_ce_a_authorityAttributeIdentifier,
    id_ce_a_roleSpecCertIdentifier,
    id_ce_a_cRLStreamIdentifier,
    id_ce_a_basicAttConstraints,
    id_ce_a_delegatedNameConstraints,
    id_ce_a_timeSpecification,
    id_ce_a_statusReferrals,
    id_ce_a_freshestCRL,
    id_ce_a_orderedList,
    id_ce_a_attributeDescriptor,
    id_ce_a_userNotice,
    id_ce_a_sOAIdentifier,
    id_ce_a_baseUpdateTime,
    id_ce_a_acceptableCertPolicies,
    id_ce_a_deltaInfo,
    id_ce_a_targetingInformation,
    id_ce_a_noRevAvail,
    id_ce_a_acceptablePrivilegePolicies,
    id_ce_a_toBeRevoked,
    id_ce_a_revokedGroups,
    id_ce_a_expiredCertsOnCRL,
    id_ce_a_indirectIssuer,
    id_ce_a_noAssertion,
    id_ce_a_aAissuingDistributionPoint,
    id_ce_a_issuedOnBehalfOf,
    id_ce_a_singleUse,
    id_ce_a_groupAC,
    id_ce_a_allowedAttributeAssignments,
    id_ce_a_attributeMappings,
    id_ce_a_holderNameConstraints,
    id_ce_a_authorizationValidation,
    id_ce_a_protRestrict,
    id_ce_a_subjectAltPublicKeyInfo,
    id_ce_a_altSignatureAlgorithm,
    id_ce_a_altSignatureValue,
};
use crate::InformationFramework::{
    id_at_objectClass,
    id_at_aliasedEntryName,
    id_oa_subtreeSpecification,
    id_oa_administrativeRole,
    id_oa_createTimestamp,
    id_oa_modifyTimestamp,
    id_oa_subschemaTimestamp,
    id_oa_creatorsName,
    id_oa_modifiersName,
    id_oa_subschemaSubentryList,
    id_oa_accessControlSubentryList,
    id_oa_collectiveAttributeSubentryList,
    id_oa_contextDefaultSubentryList,
    id_oa_serviceAdminSubentryList,
    id_oa_pwdAdminSubentryList,
    id_oa_hasSubordinates,
    id_oa_collectiveExclusions,
    id_oa_contextAssertionDefault, // Should end with "s"
    id_oa_searchRules,
    id_at_pwdAttribute,
    id_oa_hierarchyLevel,
    id_oa_hierarchyBelow,
    id_oa_hierarchyParent,
    id_oa_hierarchyTop,
};
use crate::LdapSystemSchema::{
    id_lat_namingContexts,
    id_lat_altServer,
    id_lat_supportedExtension,
    id_lat_supportedControl,
    id_lat_supportedSASLMechanisms,
    id_lat_supportedLDAPVersion,
    id_oat_supportedFeatures,
    id_soa_ldapSyntaxes,
};
use crate::PasswordPolicy::{
    id_at_userPwd,
    id_oa_pwdStartTime,
    id_oa_pwdExpiryTime,
    id_oa_pwdEndTime,
    id_oa_pwdFails,
    id_oa_pwdFailureTime,
    id_oa_pwdGracesUsed,
    id_oa_pwdModifyEntryAllowed,
    id_oa_pwdChangeAllowed,
    id_oa_pwdMaxAge,
    id_oa_pwdExpiryAge,
    id_oa_pwdMinLength,
    id_oa_pwdVocabulary,
    id_oa_pwdAlphabet,
    id_oa_pwdDictionaries,
    id_oa_pwdExpiryWarning,
    id_oa_pwdGraces,
    id_oa_pwdFailureDuration,
    id_oa_pwdLockoutDuration,
    id_oa_pwdMaxFailures,
    id_oa_pwdMaxTimeInHistory,
    id_oa_pwdMinTimeInHistory,
    id_oa_pwdHistorySlots,
    id_oa_pwdRecentlyExpiredDuration,
    id_oa_pwdEncAlg,
};
use crate::PkiPmiWrapper::{id_contentType, id_messageDigest};
use crate::SchemaAdministration::{
    id_soa_dITStructureRule, // Should end with s
    id_soa_dITContentRules,
    id_soa_matchingRules,
    id_soa_attributeTypes,
    id_soa_objectClasses,
    id_soa_nameForms,
    id_soa_matchingRuleUse,
    id_soa_structuralObjectClass,
    id_soa_governingStructureRule,
    id_soa_contextTypes,
    id_soa_dITContextUse,
    id_soa_friends,
};



pub trait DisplayX500AttributeType {

    fn attr_type_to_string (self: &Self, attr_type: &AttributeType) -> String {
        self.attr_type_to_name(attr_type).or(Some(attr_type.to_string())).unwrap()
    }

    fn attr_type_to_name (self: &Self, attr_type: &AttributeType) -> Option<String> {
        self.attr_type_to_long_name(attr_type)
            .or(self.attr_type_to_short_name(attr_type)
                .or(self.attr_type_to_descriptor(attr_type)))
    }

    // Same as `attr_type_to_name`, but it prefers shorter names.
    fn attr_type_to_shortest_name (self: &Self, attr_type: &AttributeType) -> Option<String> {
        self.attr_type_to_short_name(attr_type)
            .or(self.attr_type_to_long_name(attr_type)
                .or(self.attr_type_to_descriptor(attr_type)))
    }

    fn attr_type_to_descriptor (self: &Self, _: &AttributeType) -> Option<String> {
        None
    }

    fn attr_type_to_long_name (self: &Self, attr_type: &AttributeType) -> Option<String> {
        if *attr_type == id_at_commonName() {
            Some(String::from("commonName"))
        }
        else if *attr_type == id_at_organizationalUnitName() {
            Some(String::from("organizationalUnitName"))
        }
        else if *attr_type == id_at_organizationName() {
            Some(String::from("organizationName"))
        }
        else if *attr_type == id_coat_dc() {
            Some(String::from("domainComponent"))
        }
        else if *attr_type == id_at_localityName() {
            Some(String::from("localityName"))
        }
        else if *attr_type == id_at_stateOrProvinceName() {
            Some(String::from("stateOrProvinceName"))
        }
        else if *attr_type == id_at_countryName() {
            Some(String::from("countryName"))
        }
        else if *attr_type == id_at_dmdName() {
            Some(String::from("dmdName"))
        }
        else if *attr_type == id_coat_uid() {
            Some(String::from("uid"))
        }
        else if *attr_type == id_at_surname() {
            Some(String::from("surname"))
        }
        else if *attr_type == id_at_givenName() {
            Some(String::from("givenName"))
        }
        else if *attr_type == id_at_initials() {
            Some(String::from("initials"))
        }
        else if *attr_type == id_at_generationQualifier() {
            Some(String::from("generationQualifier"))
        }
        else if *attr_type == id_at_dnQualifier() {
            Some(String::from("dnQualifier"))
        }
        else if *attr_type == id_at_streetAddress() {
            Some(String::from("streetAddress"))
        }
        else {
            None
        }
    }

    fn attr_type_to_short_name (self: &Self, attr_type: &AttributeType) -> Option<String> {
        if *attr_type == id_at_commonName() {
            Some(String::from("cn"))
        }
        else if *attr_type == id_at_organizationalUnitName() {
            Some(String::from("ou"))
        }
        else if *attr_type == id_at_organizationName() {
            Some(String::from("o"))
        }
        else if *attr_type == id_coat_dc() {
            Some(String::from("dc"))
        }
        else if *attr_type == id_at_localityName() {
            Some(String::from("l"))
        }
        else if *attr_type == id_at_stateOrProvinceName() {
            Some(String::from("st"))
        }
        else if *attr_type == id_at_countryName() {
            Some(String::from("c"))
        }
        else if *attr_type == id_at_dmdName() {
            Some(String::from("dmdName"))
        }
        else if *attr_type == id_coat_uid() {
            Some(String::from("uid"))
        }
        else if *attr_type == id_at_surname() {
            Some(String::from("sn"))
        }
        else if *attr_type == id_at_givenName() {
            Some(String::from("gn"))
        }
        else if *attr_type == id_at_initials() {
            Some(String::from("initials"))
        }
        else if *attr_type == id_at_generationQualifier() {
            Some(String::from("generationQualifier"))
        }
        else if *attr_type == id_at_dnQualifier() {
            Some(String::from("dnQualifier"))
        }
        else if *attr_type == id_at_streetAddress() {
            Some(String::from("street"))
        }
        else {
            None
        }
    }

}

/* I ultimately chose not to implement this using a formatter because this will
likely be used primarily for printing distinguished names, and doing so
requires escaping special characters in attribute values. This cannot happen in
formatters, where the value is written out irreversibly. As such, this returns
strings directly. Unfortunately, this is probably less efficient than using a
formatter. This approach also allows us to return an ASN1Error instead of the
useless std::fmt::Error. */
pub trait DisplayX500Value <OpenType> {

    fn unrecognized_value_to_string (self: &Self, value: &OpenType) -> String;

    // TODO: Should this return an Option<T> to allow the callee to decide whether to use the "unrecognized" syntax?
    fn value_to_string (self: &Self, attr_type: &AttributeType, value: &OpenType) -> Result<Option<String>, ASN1Error>;

}

pub struct DefaultX500ValueDisplayer;
pub struct DefaultX500ValueParser;

pub fn value_to_string <E, K> (
    _k: &K,
    _attr_type: &AttributeType,
    value: &X690Element,
) -> Result<Option<String>, ASN1Error>
    where
        K: DisplayX500AttributeType + DisplayX500Value<E> {
    if value.tag.tag_class != TagClass::UNIVERSAL {
        return Ok(None);
    }
    match value.tag.tag_number {
        // UNIV_TAG_END_OF_CONTENT => {},
        UNIV_TAG_BOOLEAN => {
            let v = BER.decode_boolean(value)?;
            if v {
                Ok(Some(String::from("TRUE")))
            } else {
                Ok(Some(String::from("FALSE")))
            }
        },
        UNIV_TAG_INTEGER => {
            let integ = BER.decode_integer(value)?;
            if let Some(i) = read_i64(&integ) {
                return Ok(Some(format!("{}", i).to_string()));
            } else {
                return Ok(None);
            }
        },
        UNIV_TAG_BIT_STRING => {
            let v = BER.decode_bit_string(value)?;
            Ok(Some(format!("{}", v).to_string()))
        },
        UNIV_TAG_OCTET_STRING => {
            let v = BER.decode_octet_string(value)?;
            // NOTE: This is not the LDAP syntax. The LDAP syntax is just the raw octets.
            Ok(Some(hex::encode(v)))
        },
        UNIV_TAG_NULL => {
            Ok(Some(String::from("NULL")))
        },
        UNIV_TAG_OBJECT_IDENTIFIER => {
            let v = BER.decode_object_identifier(value)?;
            Ok(Some(v.to_string()))
        },
        UNIV_TAG_OBJECT_DESCRIPTOR => {
            let v = BER.decode_object_descriptor(value)?;
            Ok(Some(v))
        },
        // UNIV_TAG_EXTERNAL => {},
        UNIV_TAG_REAL => {
            let v = BER.decode_real(value)?;
            Ok(Some(v.to_string()))
        },
        UNIV_TAG_ENUMERATED => {
            let v = BER.decode_enumerated(value)?;
            Ok(Some(v.to_string()))
        },
        // UNIV_TAG_EMBEDDED_PDV => {},
        UNIV_TAG_UTF8_STRING => {
            let v = BER.decode_utf8_string(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_RELATIVE_OID => {
            let v = BER.decode_relative_oid(value)?;
            Ok(Some(v.to_string()))
        },
        UNIV_TAG_TIME => {
            let v = BER.decode_time(value)?;
            Ok(Some(v))
        },
        // UNIV_TAG_RESERVED_15 => {},
        // UNIV_TAG_SEQUENCE => {},
        // UNIV_TAG_SEQUENCE_OF => {},
        // UNIV_TAG_SET => {},
        // UNIV_TAG_SET_OF => {},
        UNIV_TAG_NUMERIC_STRING => {
            let v = BER.decode_numeric_string(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_PRINTABLE_STRING => {
            let v = BER.decode_printable_string(value)?;
            Ok(Some(v))
        },
        // UNIV_TAG_T61_STRING => {},
        // UNIV_TAG_VIDEOTEX_STRING => {},
        UNIV_TAG_IA5_STRING => {
            let v = BER.decode_ia5_string(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_UTC_TIME => {
            let v = BER.decode_utc_time(value)?;
            Ok(Some(v.to_string()))
        },
        UNIV_TAG_GENERALIZED_TIME => {
            let v = BER.decode_generalized_time(value)?;
            Ok(Some(v.to_string()))
        },
        UNIV_TAG_GRAPHIC_STRING => {
            let v = BER.decode_graphic_string(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_VISIBLE_STRING => {
            let v = BER.decode_visible_string(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_GENERAL_STRING => {
            let v = BER.decode_general_string(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_UNIVERSAL_STRING => {
            let v = BER.decode_universal_string(value)?;
            Ok(Some(v.to_string_lossy()))
        },
        // UNIV_TAG_CHARACTER_STRING => {},
        UNIV_TAG_BMP_STRING => {
            let v = BER.decode_bmp_string(value)?;
            Ok(Some(v.to_string_lossy()))
        },
        UNIV_TAG_DATE => {
            let v = BER.decode_date(value)?;
            Ok(Some(format!("{}-{}-{}", v.year, v.month, v.day).to_string()))
        },
        UNIV_TAG_TIME_OF_DAY => {
            let v = BER.decode_time_of_day(value)?;
            Ok(Some(format!("{}:{}:{}", v.hour, v.minute, v.second).to_string()))
        },
        UNIV_TAG_DATE_TIME => {
            let v = BER.decode_date_time(value)?;
            Ok(Some(format!("{}-{}-{}T{}:{}:{}",
                v.date.year,
                v.date.month,
                v.date.day,
                v.time.hour,
                v.time.minute,
                v.time.second
            ).to_string()))
        },
        UNIV_TAG_DURATION => {
            let v = BER.decode_duration(value)?;
            Ok(Some(v.to_string()))
        },
        UNIV_TAG_OID_IRI => {
            let v = BER.decode_oid_iri(value)?;
            Ok(Some(v))
        },
        UNIV_TAG_RELATIVE_OID_IRI => {
            let v = BER.decode_relative_oid_iri(value)?;
            Ok(Some(v))
        },
        _ => Ok(None)
    }
}

impl DisplayX500Value<X690Element> for DefaultX500ValueDisplayer {

    fn unrecognized_value_to_string (self: &Self, value: &X690Element) -> String {
        let mut encoding: Vec<u8> = Vec::new();
        x690_write_tlv(&mut encoding, &value).unwrap_or_default();
        format!("#{}", hex::encode(&encoding))
    }

    fn value_to_string (
        &self,
        attr_type: &AttributeType,
        value: &X690Element,
    ) -> Result<Option<String>, ASN1Error> {
        value_to_string(self, attr_type, value)
    }

}

impl DisplayX500AttributeType for DefaultX500ValueDisplayer {  }

pub trait ParseX500AttributeType {

    fn attr_type_name_to_oid (&self, s: &str) -> Option<OBJECT_IDENTIFIER> {
        match s.to_lowercase().as_str() {
            "cn" => Some(id_at_commonName()),
            "ou" => Some(id_at_organizationalUnitName()),
            "o" => Some(id_at_organizationName()),
            "dc" => Some(id_coat_dc()),
            "domaincomponent" => Some(id_coat_dc()),
            "l" => Some(id_at_localityName()),
            "st" => Some(id_at_stateOrProvinceName()),
            "c" => Some(id_at_countryName()),
            "sn" => Some(id_at_surname()),
            "gn" => Some(id_at_givenName()),
            "street" => Some(id_at_streetAddress()),
            "businesscategory" => Some(id_at_businessCategory()),
            "collectivefacsimiletelephonenumber" => Some(id_at_collectiveFacsimileTelephoneNumber()),
            "collectiveinternationalisdnnumber" => Some(id_at_collectiveInternationalISDNNumber()),
            "collectivelocalityname" => Some(id_at_collectiveLocalityName()),
            "collectiveorganizationalunitname" => Some(id_at_collectiveOrganizationalUnitName()),
            "collectiveorganizationname" => Some(id_at_collectiveOrganizationName()),
            "collectivephysicaldeliveryofficename" => Some(id_at_collectivePhysicalDeliveryOfficeName()),
            "collectivepostaladdress" => Some(id_at_collectivePostalAddress()),
            "collectivepostalcode" => Some(id_at_collectivePostalCode()),
            "collectivepostofficebox" => Some(id_at_collectivePostOfficeBox()),
            "collectivestateorprovincename" => Some(id_at_collectiveStateOrProvinceName()),
            "collectivestreetaddress" => Some(id_at_collectiveStreetAddress()),
            "collectivetelephonenumber" => Some(id_at_collectiveTelephoneNumber()),
            "collectivetelexnumber" => Some(id_at_collectiveTelexNumber()),
            "commonname" => Some(id_at_commonName()),
            "communicationsnetwork" => Some(id_at_communicationsNetwork()),
            "communicationsservice" => Some(id_at_communicationsService()),
            "contenturl" => Some(id_at_contentUrl()),
            "countrycode3c" => Some(id_at_countryCode3c()),
            "countrycode3n" => Some(id_at_countryCode3n()),
            "countryname" => Some(id_at_countryName()),
            "description" => Some(id_at_description()),
            "destinationindicator" => Some(id_at_destinationIndicator()),
            "distinguishedname" => Some(id_at_distinguishedName()),
            "dmdname" => Some(id_at_dmdName()),
            "dnqualifier" => Some(id_at_dnQualifier()),
            "dnsname" => Some(id_at_dnsName()),
            "enhancedsearchguide" => Some(id_at_enhancedSearchGuide()),
            "epc" => Some(id_at_epc()),
            "epcformat" => Some(id_at_epcFormat()),
            "epcinurn" => Some(id_at_epcInUrn()),
            "facsimiletelephonenumber" => Some(id_at_facsimileTelephoneNumber()),
            "generationqualifier" => Some(id_at_generationQualifier()),
            "givenname" => Some(id_at_givenName()),
            "houseidentifier" => Some(id_at_houseIdentifier()),
            "initials" => Some(id_at_initials()),
            "intemail" => Some(id_at_intEmail()),
            "internationalisdnnumber" => Some(id_at_internationalISDNNumber()),
            "jid" => Some(id_at_jid()),
            "knowledgeinformation" => Some(id_at_knowledgeInformation()),
            "ldapurl" => Some(id_at_ldapUrl()),
            "localityname" => Some(id_at_localityName()),
            "member" => Some(id_at_member()),
            "name" => Some(id_at_name()),
            "objectidentifier" => Some(id_at_objectIdentifier()),
            "oidc" => Some(id_oidC()),
            "oidc1" => Some(id_oidC1()),
            "oidc2" => Some(id_oidC2()),
            "organizationalunitname" => Some(id_at_organizationalUnitName()),
            "organizationidentifier" => Some(id_at_organizationIdentifier()),
            "organizationname" => Some(id_at_organizationName()),
            "owner" => Some(id_at_owner()),
            "physicaldeliveryofficename" => Some(id_at_physicalDeliveryOfficeName()),
            "postaladdress" => Some(id_at_postalAddress()),
            "postalcode" => Some(id_at_postalCode()),
            "postofficebox" => Some(id_at_postOfficeBox()),
            "preferreddeliverymethod" => Some(id_at_preferredDeliveryMethod()),
            "presentationaddress" => Some(id_at_presentationAddress()),
            "protocolinformation" => Some(id_at_protocolInformation()),
            "pseudonym" => Some(id_at_pseudonym()),
            "registeredaddress" => Some(id_at_registeredAddress()),
            "roleoccupant" => Some(id_at_roleOccupant()),
            "searchguide" => Some(id_at_searchGuide()),
            "seealso" => Some(id_at_seeAlso()),
            "serialnumber" => Some(id_at_serialNumber()),
            "stateorprovincename" => Some(id_at_stateOrProvinceName()),
            "streetaddress" => Some(id_at_streetAddress()),
            "supportedapplicationcontext" => Some(id_at_supportedApplicationContext()),
            "surname" => Some(id_at_surname()),
            "tagafi" => Some(id_at_tagAfi()),
            "taglocation" => Some(id_at_tagLocation()),
            "tagoid" => Some(id_at_tagOid()),
            "telephonenumber" => Some(id_at_telephoneNumber()),
            "telexnumber" => Some(id_at_telexNumber()),
            "title" => Some(id_at_title()),
            "uii" => Some(id_at_uii()),
            "uiiformat" => Some(id_at_uiiFormat()),
            "uiiinurn" => Some(id_at_uiiInUrn()),
            "uniqueidentifier" => Some(id_at_uniqueIdentifier()),
            "uniquemember" => Some(id_at_uniqueMember()),
            "uri" => Some(id_at_uri()),
            "url" => Some(id_at_url()),
            "urn" => Some(id_at_urn()),
            "urnc" => Some(id_at_urnC()),
            "utmcoordinates" => Some(id_at_utmCoordinates()),
            "uuidpair" => Some(id_at_uuidpair()),
            "x121address" => Some(id_at_x121Address()),
            "uid" => Some(id_coat_uid()),
            "role" => Some(id_at_role()),
            "permission" => Some(id_at_permission()),
            "attributecertificate" => Some(id_at_attributeCertificate()),
            "aacertificate" => Some(id_at_aACertificate()),
            "attributedescriptorcertificate" => Some(id_at_attributeDescriptorCertificate()),
            "attributecertificaterevocationlist" => Some(id_at_attributeCertificateRevocationList()),
            "eeattrcertificaterevocationlist" => Some(id_at_eeAttrCertificateRevocationList()),
            "attributeauthorityrevocationlist" => Some(id_at_attributeAuthorityRevocationList()),
            "delegationpath" => Some(id_at_delegationPath()),
            "privpolicy" => Some(id_at_privPolicy()),
            "protprivpolicy" => Some(id_at_protPrivPolicy()),
            "xmlprivpolicy" => Some(id_at_xmlPrivPolicy()),
            "usercertificate" => Some(id_at_userCertificate()),
            "cacertificate" => Some(id_at_cAcertificate()),
            "crosscertificatepair" => Some(id_at_crossCertificatePair()),
            "certificaterevocationlist" => Some(id_at_certificateRevocationList()),
            "eepkcertificaterevocationlist" => Some(id_at_eepkCertificateRevocationList()),
            "authorityrevocationlist" => Some(id_at_authorityRevocationList()),
            "deltarevocationlist" => Some(id_at_deltaRevocationList()),
            "supportedalgorithms" => Some(id_at_supportedAlgorithms()),
            "certificationpracticestmt" => Some(id_at_certificationPracticeStmt()),
            "certificatepolicy" => Some(id_at_certificatePolicy()),
            "pkipath" => Some(id_at_pkiPath()),
            "userpassword" => Some(id_at_userPassword()),
            "accesscontrolscheme" => Some(id_aca_accessControlScheme()),
            "prescriptiveaci" => Some(id_aca_prescriptiveACI()),
            "entryaci" => Some(id_aca_entryACI()),
            "subentryaci" => Some(id_aca_subentryACI()),
            "family_information" => Some(id_at_family_information()),
            "dsetype" => Some(id_doa_dseType()),
            "myaccesspoint" => Some(id_doa_myAccessPoint()),
            "superiorknowledge" => Some(id_doa_superiorKnowledge()),
            "specificknowledge" => Some(id_doa_specificKnowledge()),
            "nonspecificknowledge" => Some(id_doa_nonSpecificKnowledge()),
            "supplierknowledge" => Some(id_doa_supplierKnowledge()),
            "consumerknowledge" => Some(id_doa_consumerKnowledge()),
            "secondaryshadows" => Some(id_doa_secondaryShadows()),
            "ditbridgeknowledge" => Some(id_doa_ditBridgeKnowledge()),
            "clearance" => Some(id_at_clearance()),
            "attributeintegrityinfo" => Some(id_at_attributeIntegrityInfo()),
            "subjectdirectoryattributes" => Some(id_ce_a_subjectDirectoryAttributes()),
            "keyusage" => Some(id_ce_a_keyUsage()),
            "privatekeyusageperiod" => Some(id_ce_a_privateKeyUsagePeriod()),
            "subjectaltname" => Some(id_ce_a_subjectAltName()),
            "issueraltname" => Some(id_ce_a_issuerAltName()),
            "basicconstraints" => Some(id_ce_a_basicConstraints()),
            "crlnumber" => Some(id_ce_a_cRLNumber()),
            "reasoncode" => Some(id_ce_a_reasonCode()),
            "holdinstructioncode" => Some(id_ce_a_holdInstructionCode()),
            "invaliditydate" => Some(id_ce_a_invalidityDate()),
            "deltacrlindicator" => Some(id_ce_a_deltaCRLIndicator()),
            "issuingdistributionpoint" => Some(id_ce_a_issuingDistributionPoint()),
            "certificateissuer" => Some(id_ce_a_certificateIssuer()),
            "nameconstraints" => Some(id_ce_a_nameConstraints()),
            "crldistributionpoints" => Some(id_ce_a_cRLDistributionPoints()),
            "certificatepolicies" => Some(id_ce_a_certificatePolicies()),
            "policymappings" => Some(id_ce_a_policyMappings()),
            "authoritykeyidentifier" => Some(id_ce_a_authorityKeyIdentifier()),
            "policyconstraints" => Some(id_ce_a_policyConstraints()),
            "extkeyusage" => Some(id_ce_a_extKeyUsage()),
            "authorityattributeidentifier" => Some(id_ce_a_authorityAttributeIdentifier()),
            "rolespeccertidentifier" => Some(id_ce_a_roleSpecCertIdentifier()),
            "crlstreamidentifier" => Some(id_ce_a_cRLStreamIdentifier()),
            "basicattconstraints" => Some(id_ce_a_basicAttConstraints()),
            "delegatednameconstraints" => Some(id_ce_a_delegatedNameConstraints()),
            "timespecification" => Some(id_ce_a_timeSpecification()),
            "statusreferrals" => Some(id_ce_a_statusReferrals()),
            "freshestcrl" => Some(id_ce_a_freshestCRL()),
            "orderedlist" => Some(id_ce_a_orderedList()),
            "attributedescriptor" => Some(id_ce_a_attributeDescriptor()),
            "usernotice" => Some(id_ce_a_userNotice()),
            "soaidentifier" => Some(id_ce_a_sOAIdentifier()),
            "baseupdatetime" => Some(id_ce_a_baseUpdateTime()),
            "acceptablecertpolicies" => Some(id_ce_a_acceptableCertPolicies()),
            "deltainfo" => Some(id_ce_a_deltaInfo()),
            "targetinginformation" => Some(id_ce_a_targetingInformation()),
            "norevavail" => Some(id_ce_a_noRevAvail()),
            "acceptableprivilegepolicies" => Some(id_ce_a_acceptablePrivilegePolicies()),
            "toberevoked" => Some(id_ce_a_toBeRevoked()),
            "revokedgroups" => Some(id_ce_a_revokedGroups()),
            "expiredcertsoncrl" => Some(id_ce_a_expiredCertsOnCRL()),
            "indirectissuer" => Some(id_ce_a_indirectIssuer()),
            "noassertion" => Some(id_ce_a_noAssertion()),
            "aaissuingdistributionpoint" => Some(id_ce_a_aAissuingDistributionPoint()),
            "issuedonbehalfof" => Some(id_ce_a_issuedOnBehalfOf()),
            "singleuse" => Some(id_ce_a_singleUse()),
            "groupac" => Some(id_ce_a_groupAC()),
            "allowedattributeassignments" => Some(id_ce_a_allowedAttributeAssignments()),
            "attributemappings" => Some(id_ce_a_attributeMappings()),
            "holdernameconstraints" => Some(id_ce_a_holderNameConstraints()),
            "authorizationvalidation" => Some(id_ce_a_authorizationValidation()),
            "protrestrict" => Some(id_ce_a_protRestrict()),
            "subjectaltpublickeyinfo" => Some(id_ce_a_subjectAltPublicKeyInfo()),
            "altsignaturealgorithm" => Some(id_ce_a_altSignatureAlgorithm()),
            "altsignaturevalue" => Some(id_ce_a_altSignatureValue()),
            "objectclass" => Some(id_at_objectClass()),
            "aliasedentryname" => Some(id_at_aliasedEntryName()),
            "subtreespecification" => Some(id_oa_subtreeSpecification()),
            "administrativerole" => Some(id_oa_administrativeRole()),
            "createtimestamp" => Some(id_oa_createTimestamp()),
            "modifytimestamp" => Some(id_oa_modifyTimestamp()),
            "subschematimestamp" => Some(id_oa_subschemaTimestamp()),
            "creatorsname" => Some(id_oa_creatorsName()),
            "modifiersname" => Some(id_oa_modifiersName()),
            "subschemasubentrylist" => Some(id_oa_subschemaSubentryList()),
            "accesscontrolsubentrylist" => Some(id_oa_accessControlSubentryList()),
            "collectiveattributesubentrylist" => Some(id_oa_collectiveAttributeSubentryList()),
            "contextdefaultsubentrylist" => Some(id_oa_contextDefaultSubentryList()),
            "serviceadminsubentrylist" => Some(id_oa_serviceAdminSubentryList()),
            "pwdadminsubentrylist" => Some(id_oa_pwdAdminSubentryList()),
            "hassubordinates" => Some(id_oa_hasSubordinates()),
            "collectiveexclusions" => Some(id_oa_collectiveExclusions()),
            "contextassertiondefaults" => Some(id_oa_contextAssertionDefault()),
            "searchrules" => Some(id_oa_searchRules()),
            "pwdattribute" => Some(id_at_pwdAttribute()),
            "hierarchylevel" => Some(id_oa_hierarchyLevel()),
            "hierarchybelow" => Some(id_oa_hierarchyBelow()),
            "hierarchyparent" => Some(id_oa_hierarchyParent()),
            "hierarchytop" => Some(id_oa_hierarchyTop()),
            "namingcontexts" => Some(id_lat_namingContexts()),
            "altserver" => Some(id_lat_altServer()),
            "supportedextension" => Some(id_lat_supportedExtension()),
            "supportedcontrol" => Some(id_lat_supportedControl()),
            "supportedsaslmechanisms" => Some(id_lat_supportedSASLMechanisms()),
            "supportedldapversion" => Some(id_lat_supportedLDAPVersion()),
            "supportedfeatures" => Some(id_oat_supportedFeatures()),
            "ldapsyntaxes" => Some(id_soa_ldapSyntaxes()),
            "userpwd" => Some(id_at_userPwd()),
            "pwdstarttime" => Some(id_oa_pwdStartTime()),
            "pwdexpirytime" => Some(id_oa_pwdExpiryTime()),
            "pwdendtime" => Some(id_oa_pwdEndTime()),
            "pwdfails" => Some(id_oa_pwdFails()),
            "pwdfailuretime" => Some(id_oa_pwdFailureTime()),
            "pwdgracesused" => Some(id_oa_pwdGracesUsed()),
            "pwdmodifyentryallowed" => Some(id_oa_pwdModifyEntryAllowed()),
            "pwdchangeallowed" => Some(id_oa_pwdChangeAllowed()),
            "pwdmaxage" => Some(id_oa_pwdMaxAge()),
            "pwdexpiryage" => Some(id_oa_pwdExpiryAge()),
            "pwdminlength" => Some(id_oa_pwdMinLength()),
            "pwdvocabulary" => Some(id_oa_pwdVocabulary()),
            "pwdalphabet" => Some(id_oa_pwdAlphabet()),
            "pwddictionaries" => Some(id_oa_pwdDictionaries()),
            "pwdexpirywarning" => Some(id_oa_pwdExpiryWarning()),
            "pwdgraces" => Some(id_oa_pwdGraces()),
            "pwdfailureduration" => Some(id_oa_pwdFailureDuration()),
            "pwdlockoutduration" => Some(id_oa_pwdLockoutDuration()),
            "pwdmaxfailures" => Some(id_oa_pwdMaxFailures()),
            "pwdmaxtimeinhistory" => Some(id_oa_pwdMaxTimeInHistory()),
            "pwdmintimeinhistory" => Some(id_oa_pwdMinTimeInHistory()),
            "pwdhistoryslots" => Some(id_oa_pwdHistorySlots()),
            "pwdrecentlyexpiredduration" => Some(id_oa_pwdRecentlyExpiredDuration()),
            "pwdencalg" => Some(id_oa_pwdEncAlg()),
            "contenttype" => Some(id_contentType()),
            "messagedigest" => Some(id_messageDigest()),
            "ditstructurerules" => Some(id_soa_dITStructureRule()),
            "ditcontentrules" => Some(id_soa_dITContentRules()),
            "matchingrules" => Some(id_soa_matchingRules()),
            "attributetypes" => Some(id_soa_attributeTypes()),
            "objectclasses" => Some(id_soa_objectClasses()),
            "nameforms" => Some(id_soa_nameForms()),
            "matchingruleuse" => Some(id_soa_matchingRuleUse()),
            "structuralobjectclass" => Some(id_soa_structuralObjectClass()),
            "governingstructurerule" => Some(id_soa_governingStructureRule()),
            "contexttypes" => Some(id_soa_contextTypes()),
            "ditcontextuse" => Some(id_soa_dITContextUse()),
            "friends" => Some(id_soa_friends()),
            _ => None,
        }
    }

    // Error is for an invalid attribute type string.
    // Option::None is just for if a name is not recognized.
    // fn parse_attr_type (&self, s: &str) -> Result<Option<AttributeType>, std::fmt::Error>;
    fn parse_attr_type (&self, s: &str) -> Result<Option<AttributeType>, std::fmt::Error> {
        if let Some(first_char) = s.chars().next() {
            if first_char.is_digit(3) { // The first digit can only be 0, 1, or 2. Using a radix of 3 handles this.
                let oid = OBJECT_IDENTIFIER::from_str(s).map_err(|_| std::fmt::Error)?;
                return Ok(Some(oid));
            } else {
                // Attempt to resolve name.
                return Ok(self.attr_type_name_to_oid(s));
            }
        } else {
            return Err(std::fmt::Error);
        }
    }

}

pub trait ParseX500Value <OpenType> {

    fn parse_value (&self, attr_type: &AttributeType, s: &str) -> Result<Option<OpenType>, std::fmt::Error>;

}

pub trait ParseX500DistinguishedName {

    fn parse_dn (&self, s: &str) -> Result<DistinguishedName, std::fmt::Error>;

}

pub trait ParseX500DirectoryName {

    fn parse_x500_name (&self, s: &str) -> Result<Name, std::fmt::Error>;

}

pub trait ParseGeneralName {

    fn parse_general_name (&self, s: &str) -> Result<GeneralName, std::fmt::Error>;

}

impl ParseX500AttributeType for DefaultX500ValueParser {  }

// Exported separately so it can be used by other implementations, if desired.
pub fn parse_value <K: ParseX500AttributeType> (_k: &K, attr_type: &AttributeType, s: &str) -> Result<Option<X690Element>, std::fmt::Error> {
    if s.starts_with("#") {
        let bytes = hex::decode(&s[1..]).map_err(|_| std::fmt::Error)?;
        let cst = BER.decode_from_slice(&bytes).map_err(|_| std::fmt::Error)?;
        return Ok(Some(cst.1));
    }
    if attr_type.as_x690_slice().len() == 3 && attr_type.as_x690_slice().starts_with(&[ 0x55, 4 ]) {
        match attr_type.as_x690_slice().last().unwrap() {
            2 // knowledgeInformation
            | 3 // commonName
            | 4 // surname
            | 7 // localityName
            | 8 // stateOrProvinceName
            | 10 // organizationName
            | 11 // organizationalUnitName
            | 12 // title
            | 13 // description
            | 15 // businessCategory
            | 17 // postalCode
            | 18 // postOfficeBox
            | 19 // physicalDeliveryOfficeName
            | 41 // name
            | 42 // givenName
            | 43 // initials
            | 44 // generationQualifier
            | 51 // houseIdentifier
            | 54 // dmdName
            | 65 // pseudonym
            | 81 // contentUrl
            | 83 // uri
            | 86 // urn
            | 87 // url
            | 97 // organizationIdentifier
            | 100 // dnsName
            | 104 // intEmail
            | 105 // jid
            => return Ok(Some(BER.encode_utf8_string(s).map_err(|_| std::fmt::Error)?)),

            5 // serialNumber
            | 6 // countryName
            | 20 // telephoneNumber
            | 27 // destinationIndicator
            | 46 // dnQualifier
            | 89 // urnC
            | 98 // countryCode3c
            => return Ok(Some(BER.encode_printable_string(s).map_err(|_| std::fmt::Error)?)),

            24 // x121Address
            | 25 // internationalISDNNumber
            | 99 // countryCode3n
            => return Ok(Some(BER.encode_numeric_string(s).map_err(|_| std::fmt::Error)?)),

            0 // TODO: objectClass
            | 30 // supportedApplicationContext
            | 66 // communicationsService
            | 67 // communicationsNetwork
            | 78 // tagOid
            | 84 // pwdAttribute
            | 106 // objectIdentifier
            => {
                let oid = OBJECT_IDENTIFIER::from_str(s).map_err(|_| std::fmt::Error)?;
                return Ok(Some(BER.encode_object_identifier(&oid).map_err(|_| std::fmt::Error)?));
            },

            1 // aliasedEntryName
            | 31 // member
            | 32 // owner
            | 33 // roleOccupant
            | 34 // seeAlso
            | 49 // distinguishedName
            => {
                // TODO: Implement ParseX500DistinguishedName for this type
            },

            // BIT STRING
            // uniqueIdentifier
            // uii
            // epc

            // Fax:
            // facsimileTelephoneNumber

            // OCTET STRING
            // tagAfi

            // Postal Address:
            // postalAddress
            // registeredAddress

            // searchGuide: guide

            // uniqueMember: nameAndOptionalUID

            // presentationAddress?

            _ => return Ok(None),

        }
    }
    Ok(None)
    // "collectivelocalityname" => Some(id_at_collectiveLocalityName()),
    // "collectiveorganizationalunitname" => Some(id_at_collectiveOrganizationalUnitName()),
    // "collectiveorganizationname" => Some(id_at_collectiveOrganizationName()),
    // "collectivephysicaldeliveryofficename" => Some(id_at_collectivePhysicalDeliveryOfficeName()),
    // "collectivepostalcode" => Some(id_at_collectivePostalCode()),
    // "collectivepostofficebox" => Some(id_at_collectivePostOfficeBox()),
    // "collectivestateorprovincename" => Some(id_at_collectiveStateOrProvinceName()),
    // "internationalisdnnumber" => Some(id_at_internationalISDNNumber()),


    // Not ITU.
    // "uid" => Some(id_coat_uid()),

    // Under a different OID. All have INTEGER syntax.
    // oidC1
    // oidC2
    // oidC

    // TODO: Most operational attribute types
}

impl ParseX500Value<X690Element> for DefaultX500ValueParser {

    fn parse_value (&self, attr_type: &AttributeType, s: &str) -> Result<Option<X690Element>, std::fmt::Error> {
        parse_value(self, attr_type, s)
    }

}

// TODO: Implement ParseX500DistinguishedName for all types that implement
// ParseX500Value<X690Element> and ParseX500AttributeType

impl <T> ParseX500DistinguishedName for T
    where T: ParseX500AttributeType + ParseX500Value<X690Element> {

        fn parse_dn (&self, s: &str) -> Result<DistinguishedName, std::fmt::Error> {
            // let mut char_start_byte: usize = 0;

            let mut dn: DistinguishedName = Vec::with_capacity(10);
            let mut rdn: RelativeDistinguishedName = Vec::with_capacity(4);
            let mut attr_type: Option<AttributeType> = None;
            let mut start_of_token: usize = 0;
            let mut parsing_value: bool = false;
            let mut escaping: bool = false;
            let mut escaped: bool = false;
            let mut escaped_char: char = char::from_u32(0).unwrap();
            let mut escaped_str: String = String::new();
            for (i, c) in s.char_indices() {
                if parsing_value {
                    if escaping {

                    }
                    if c == ',' {

                    }
                } else {
                    if c == '=' {
                        attr_type = self.parse_attr_type(&s[start_of_token..i])?;
                        if attr_type.is_none() {
                            // If we cannot convert the type to an OID, fail.
                            return Err(std::fmt::Error);
                        }
                        start_of_token = i + 1;
                        parsing_value = true;
                    }
                }
            }
            Ok(dn)
        }

}

impl <T> ParseX500DirectoryName for T
    where T: ParseX500DistinguishedName {

    fn parse_x500_name (&self, s: &str) -> Result<Name, std::fmt::Error> {
        if s.to_lowercase().starts_with("rdnsequence") {
            Ok(Name::rdnSequence(self.parse_dn(&s[11..])?))
        } else {
            Err(std::fmt::Error)
        }
    }

}


// TODO: Implement ParseX500DirectoryName for all types that implement ParseX500DistinguishedName

// TODO: Implement ParseGeneralName for all types that implement ParseX500DirectoryName
