use crate::InformationFramework::{AttributeTypeAndValue, RelativeDistinguishedName};
use crate::PkiPmiExternalDataTypes::{
    ub_common_name_length, ub_domain_defined_attribute_type_length,
    ub_domain_defined_attribute_value_length, ub_domain_defined_attributes, ub_domain_name_length,
    ub_e163_4_number_length, ub_generation_qualifier_length, ub_given_name_length,
    ub_initials_length, ub_numeric_user_id_length, ub_organization_name_length,
    ub_organizational_unit_name_length, ub_pds_name_length, ub_pds_parameter_length,
    ub_postal_code_length, ub_surname_length, ub_terminal_id_length,
    ub_universal_generation_qualifier_length, ub_universal_given_name_length,
    ub_universal_initials_length, ub_universal_surname_length, ub_x121_address_length,
    x121_dcc_country_code_to_iso_3166, AdministrationDomainName, BuiltInDomainDefinedAttribute,
    BuiltInDomainDefinedAttributes, BuiltInStandardAttributes, CountryName, ExtendedNetworkAddress,
    ExtendedNetworkAddress_e163_4_address, ExtensionAttribute, ExtensionAttributes, ORAddress,
    PDSParameter, PersonalName, PhysicalDeliveryCountryName, PostalCode, PrivateDomainName,
    TeletexDomainDefinedAttribute, TeletexPersonalName, TerminalType, TerminalType_g3_facsimile,
    TerminalType_g4_facsimile, TerminalType_ia5_terminal, TerminalType_teletex, TerminalType_telex,
    TerminalType_videotex, UnformattedPostalAddress, UniversalDomainDefinedAttribute,
    UniversalOrBMPString, UniversalPersonalName, _decode_ExtendedNetworkAddress,
    _decode_PDSParameter, _decode_PhysicalDeliveryCountryName, _decode_PostalCode,
    _decode_TeletexDomainDefinedAttributes, _decode_TeletexOrganizationalUnitNames,
    _decode_TeletexPersonalName, _decode_UnformattedPostalAddress,
    _decode_UniversalDomainDefinedAttributes, _decode_UniversalOrBMPString,
    _decode_UniversalOrganizationalUnitNames, _decode_UniversalPersonalName,
    _encode_PhysicalDeliveryCountryName, _encode_UniversalOrBMPString,
    _encode_UniversalOrganizationalUnitNames, _encode_UniversalPersonalName,
};
use crate::SelectedAttributeTypes::{
    countryCode3c, countryName, internationalISDNNumber, organizationalUnitName,
    physicalDeliveryOfficeName, postOfficeBox, postalAddress, postalCode, presentationAddress,
    x121Address, PresentationAddress, UnboundedDirectoryString,
    _encode_owned_UnboundedDirectoryString, commonName, countryCode3n, generationQualifier,
    givenName, initials, organizationName, streetAddress, surname, PostalAddress, _decode_UnboundedDirectoryString,
};
use wildboar_asn1::{
    compare_numeric_string, is_numeric_str, is_printable_str, oid, ASN1Error, ASN1ErrorCode,
    NumericString,
};
use cow_utils::CowUtils;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::error::Error;
use std::fmt::{Display, Write};
use std::str::FromStr;
use teletex::teletex_to_utf8;
use x690::{X690Codec, BER};
use isocountry::CountryCode;

use super::_encode_ExtendedNetworkAddress;

pub const id_common_name: i64 = crate::PkiPmiExternalDataTypes::common_name().id;
pub const id_teletex_common_name: i64 = crate::PkiPmiExternalDataTypes::teletex_common_name().id;
pub const id_universal_common_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_common_name().id;
pub const id_teletex_organization_name: i64 =
    crate::PkiPmiExternalDataTypes::teletex_organization_name().id;
pub const id_universal_organization_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_organization_name().id;
pub const id_teletex_personal_name: i64 =
    crate::PkiPmiExternalDataTypes::teletex_personal_name().id;
pub const id_universal_personal_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_personal_name().id;
pub const id_teletex_organizational_unit_names: i64 =
    crate::PkiPmiExternalDataTypes::teletex_organizational_unit_names().id;
pub const id_universal_organizational_unit_names: i64 =
    crate::PkiPmiExternalDataTypes::universal_organizational_unit_names().id;
pub const id_teletex_domain_defined_attributes: i64 =
    crate::PkiPmiExternalDataTypes::teletex_domain_defined_attributes().id;
pub const id_universal_domain_defined_attributes: i64 =
    crate::PkiPmiExternalDataTypes::universal_domain_defined_attributes().id;
pub const id_pds_name: i64 = crate::PkiPmiExternalDataTypes::pds_name().id;
pub const id_physical_delivery_country_name: i64 =
    crate::PkiPmiExternalDataTypes::physical_delivery_country_name().id;
pub const id_postal_code: i64 = crate::PkiPmiExternalDataTypes::postal_code().id;
pub const id_physical_delivery_office_name: i64 =
    crate::PkiPmiExternalDataTypes::physical_delivery_office_name().id;
pub const id_universal_physical_delivery_office_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_physical_delivery_office_name().id;
pub const id_physical_delivery_office_number: i64 =
    crate::PkiPmiExternalDataTypes::physical_delivery_office_number().id;
pub const id_universal_physical_delivery_office_number: i64 =
    crate::PkiPmiExternalDataTypes::universal_physical_delivery_office_number().id;
pub const id_extension_OR_address_components: i64 =
    crate::PkiPmiExternalDataTypes::extension_OR_address_components().id;
pub const id_universal_extension_OR_address_components: i64 =
    crate::PkiPmiExternalDataTypes::universal_extension_OR_address_components().id;
pub const id_physical_delivery_personal_name: i64 =
    crate::PkiPmiExternalDataTypes::physical_delivery_personal_name().id;
pub const id_universal_physical_delivery_personal_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_physical_delivery_personal_name().id;
pub const id_physical_delivery_organization_name: i64 =
    crate::PkiPmiExternalDataTypes::physical_delivery_organization_name().id;
pub const id_universal_physical_delivery_organization_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_physical_delivery_organization_name().id;
pub const id_extension_physical_delivery_address_components: i64 =
    crate::PkiPmiExternalDataTypes::extension_physical_delivery_address_components().id;
pub const id_universal_extension_physical_delivery_address_components: i64 =
    crate::PkiPmiExternalDataTypes::universal_extension_physical_delivery_address_components().id;
pub const id_unformatted_postal_address: i64 =
    crate::PkiPmiExternalDataTypes::unformatted_postal_address().id;
pub const id_universal_unformatted_postal_address: i64 =
    crate::PkiPmiExternalDataTypes::universal_unformatted_postal_address().id;
pub const id_street_address: i64 = crate::PkiPmiExternalDataTypes::street_address().id;
pub const id_universal_street_address: i64 =
    crate::PkiPmiExternalDataTypes::universal_street_address().id;
pub const id_post_office_box_address: i64 =
    crate::PkiPmiExternalDataTypes::post_office_box_address().id;
pub const id_universal_post_office_box_address: i64 =
    crate::PkiPmiExternalDataTypes::universal_post_office_box_address().id;
pub const id_poste_restante_address: i64 =
    crate::PkiPmiExternalDataTypes::poste_restante_address().id;
pub const id_universal_poste_restante_address: i64 =
    crate::PkiPmiExternalDataTypes::universal_poste_restante_address().id;
pub const id_unique_postal_name: i64 = crate::PkiPmiExternalDataTypes::unique_postal_name().id;
pub const id_universal_unique_postal_name: i64 =
    crate::PkiPmiExternalDataTypes::universal_unique_postal_name().id;
pub const id_local_postal_attributes: i64 =
    crate::PkiPmiExternalDataTypes::local_postal_attributes().id;
pub const id_universal_local_postal_attributes: i64 =
    crate::PkiPmiExternalDataTypes::universal_local_postal_attributes().id;
pub const id_extended_network_address: i64 =
    crate::PkiPmiExternalDataTypes::extended_network_address().id;
pub const id_terminal_type: i64 = crate::PkiPmiExternalDataTypes::terminal_type().id;

pub fn term_type_to_str(tt: u16) -> &'static str {
    match tt {
        TerminalType_g3_facsimile => "g3fax",
        TerminalType_g4_facsimile => "g4fax",
        TerminalType_ia5_terminal => "ia5",
        TerminalType_teletex => "ttx",
        TerminalType_telex => "tlx",
        TerminalType_videotex => "vtx",
        _ => "?",
    }
}

pub fn term_type_from_str(tt: &str) -> Option<TerminalType> {
    match tt {
        "g3fax" => Some(TerminalType_g3_facsimile),
        "g4fax" => Some(TerminalType_g4_facsimile),
        "ia5" => Some(TerminalType_ia5_terminal),
        "ttx" => Some(TerminalType_teletex),
        "tlx" => Some(TerminalType_telex),
        "vtx" => Some(TerminalType_videotex),
        _ => None,
    }
}

// TODO: Move all personal name stuff into a separate file.
#[derive(Clone, Debug, Hash)]
pub struct PersonalNameInfo {
    pub surname: String,
    pub given_name: Option<String>,
    pub initials: Option<String>,
    pub generation_qualifier: Option<String>,
}

impl PersonalNameInfo {
    pub fn from_full_name(given_name: String, surname: String) -> Self {
        PersonalNameInfo {
            surname,
            given_name: Some(given_name),
            initials: None,
            generation_qualifier: None,
        }
    }

    pub fn is_printable(&self) -> bool {
        is_printable_str(&self.surname)
            && (self.surname.len() <= ub_surname_length)
            && (self.given_name.is_none()
                || self
                    .given_name
                    .as_ref()
                    .is_some_and(|n| (n.len() <= ub_given_name_length) && is_printable_str(&n)))
            && (self.initials.is_none()
                || self
                    .initials
                    .as_ref()
                    .is_some_and(|n| (n.len() <= ub_initials_length) && is_printable_str(&n)))
            && (self.generation_qualifier.is_none()
                || self.generation_qualifier.as_ref().is_some_and(|n| {
                    (n.len() <= ub_generation_qualifier_length) && is_printable_str(&n)
                }))
    }
}

impl PartialEq for PersonalNameInfo {
    fn eq(&self, other: &Self) -> bool {
        case_ignore_compare_str(&self.surname, &other.surname)
            && case_ignore_compare_opt_str(&self.given_name, &other.given_name)
            && case_ignore_compare_opt_str(&self.initials, &other.initials)
            && case_ignore_compare_opt_str(&self.generation_qualifier, &other.generation_qualifier)
    }
}

impl From<PersonalNameInfo> for UniversalPersonalName {
    fn from(value: PersonalNameInfo) -> Self {
        UniversalPersonalName {
            surname: value.surname.into(),
            given_name: value.given_name.map(|gn| gn.into()),
            initials: value.initials.map(|i| i.into()),
            generation_qualifier: value.generation_qualifier.map(|gq| gq.into()),
        }
    }
}

impl From<PersonalNameInfo> for PersonalName {
    fn from(value: PersonalNameInfo) -> Self {
        PersonalName {
            surname: value.surname,
            given_name: value.given_name,
            initials: value.initials,
            generation_qualifier: value.generation_qualifier,
        }
    }
}

impl Display for CountryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountryName::x121_dcc_code(dcc) => f.write_str(&dcc),
            CountryName::iso_3166_alpha2_code(iso) => {
                f.write_str(remove_oraddress_chars(iso).as_ref())
            }
        }
    }
}

#[derive(Debug)]
pub struct ORAddressInfo {
    // All of these come from the built-in attributes.
    pub prmd: Option<String>, // P
    pub admd: Option<String>, // A

    /// This should only be the ISO-3166 code.
    pub country: Option<CountryCode>, // C
    pub x121_net_addr: Option<String>, // X.121
    pub num_id: Option<NumericString>, // N-ID
    pub term_id: Option<String>,       // T-ID

    pub personal_name: Option<PersonalNameInfo>, // G, I, S, Q
    pub common_name: Option<String>,             // CN
    pub org_name: Option<String>,                // O
    pub ous: Vec<String>,                        // OU1, OU2, OU3, OU4
    pub pd_person: Option<String>,               // PD-PN
    pub pd_ea: Option<String>,                   // PD-EA
    pub pd_ed: Option<String>,                   // PD-ED
    pub pdo_number: Option<String>,              // PD-OFN
    pub pdo_name: Option<String>,                // PD-OF
    pub pd_org_name: Option<String>,             // PD-O
    pub street: Option<String>,                  // PD-S
    pub pd_address: Vec<String>,                 // PD-A1 - PD-A6
    pub pd_unique: Option<String>,               // PD-U
    pub pd_local: Option<String>,                // PD-L
    pub postal_restante_arr: Option<String>,     // PD-R
    pub po_box_addr: Option<String>,             // PD-B
    pub postal_code: Option<String>,             // PD-PC
    pub pd_svc_name: Option<String>,             // PD-SN
    pub pd_country_name: Option<CountryCode>,    // PD-C
    pub isdn: Option<String>,                    // ISDN (This is the E.163 or E.164 address)
    pub psap: Option<PresentationAddress>,       // PSAP
    pub term_type: Option<TerminalType>,         // T-TY
    pub dda: HashMap<String, String>,            // DDA: Domain-Defined Attribute
}

impl ORAddressInfo {

    /// See ITU-T Recommendation X.402 (1999), Section 18.3.1, First NOTE.
    pub fn is_any_admd (&self) -> bool {
        self.admd.as_ref().is_some_and(|admd| admd.as_str() == " ")
    }

    /// See ITU-T Recommendation X.402 (1999), Section 18.3.1, First NOTE.
    pub fn is_unroutable_admd (&self) -> bool {
        self.admd.as_ref().is_some_and(|admd| admd.as_str() == "0")
    }

    pub fn to_rfc1685_string(&self) -> Result<String, ASN1Error> {
        Ok(self.to_string())
    }

    pub fn is_mnem_form(&self) -> bool {
        self.admd.is_some()
        && self.country.is_some()
        && (
            self.personal_name.is_some()
            || self.org_name.is_some()
            || self.common_name.is_some()
        )
    }

    pub fn is_numr_form(&self) -> bool {
        self.admd.is_some()
        && self.country.is_some()
        && self.num_id.is_some()
    }

    pub fn is_post_f_form(&self) -> bool {
        self.admd.is_some()
        && self.country.is_some()
        && self.pd_country_name.is_some()
        && self.postal_code.is_some()
    }

    pub fn is_post_u_form(&self) -> bool {
        self.is_post_f_form()
        && self.pd_address.len() > 0
    }

    pub fn is_term_form(&self) -> bool {
        self.x121_net_addr.is_some()
    }

}

impl Default for ORAddressInfo {
    fn default() -> Self {
        ORAddressInfo {
            prmd: None,
            admd: None,
            country: None,
            x121_net_addr: None,
            num_id: None,
            term_id: None,
            personal_name: None,
            common_name: None,
            org_name: None,
            ous: Vec::new(),
            pd_person: None,
            pd_ea: None,
            pd_ed: None,
            pdo_number: None,
            pdo_name: None,
            pd_org_name: None,
            street: None,
            pd_address: Vec::new(),
            pd_unique: None,
            pd_local: None,
            postal_restante_arr: None,
            po_box_addr: None,
            postal_code: None,
            pd_svc_name: None,
            pd_country_name: None,
            isdn: None,
            psap: None,
            term_type: None,
            dda: HashMap::new(),
        }
    }
}

impl TryFrom<ORAddressInfo> for RelativeDistinguishedName {
    type Error = ASN1Error;

    fn try_from(mut value: ORAddressInfo) -> Result<Self, Self::Error> {
        // There basically can't be more than 40 attributes.
        let mut rdn: RelativeDistinguishedName = Vec::with_capacity(40);
        // id-at = id-mhs-routing 3 = 2 6 10 3
        if let Some(prmd) = value.prmd.take() {
            // mHSPRMDName { 2 6 10 3 25 }
            let uds: UnboundedDirectoryString = prmd.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 25),
                encoded,
                vec![],
            ));
        }
        if let Some(admd) = value.admd.take() {
            // mHSADMDName { 2 6 10 3 9 }
            let uds: UnboundedDirectoryString = admd.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 9),
                encoded,
                vec![],
            ));
        }
        if let Some(country) = value.country.take() {
            // mHSCountryName { 2 6 10 3 11 }
            {
                let uds: UnboundedDirectoryString = country.alpha2().to_owned().into();
                let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
                rdn.push(AttributeTypeAndValue::new(
                    oid!(2, 6, 10, 3, 11),
                    encoded,
                    vec![],
                ));
            }
            {
                let encoded = BER.encode_printable_string(country.alpha2())?;
                rdn.push(AttributeTypeAndValue::new(
                    countryName().id,
                    encoded,
                    vec![],
                ));
            }
            {
                let encoded = BER.encode_printable_string(country.alpha3())?;
                rdn.push(AttributeTypeAndValue::new(
                    countryCode3c().id,
                    encoded,
                    vec![],
                ));
            }
            {
                let encoded = BER.encode_owned_numeric_string(format!("{:0>3}", country.numeric_id()))?;
                rdn.push(AttributeTypeAndValue::new(
                    countryCode3n().id,
                    encoded,
                    vec![],
                ));
            }
            {
                let encoded = BER.encode_utf8_string(country.name())?;
                rdn.push(AttributeTypeAndValue::new(
                    oid!(0,9,2342,19200300,100,1,43),
                    encoded,
                    vec![],
                ));
            }
        }

        if let Some(x121_net_addr) = value.x121_net_addr.take() {
            // mHSNetworkAddressAttribute { 2 6 10 3 18 }
            let encoded = BER.encode_numeric_string(x121_net_addr.as_str())?;
            rdn.push(AttributeTypeAndValue::new(
                x121Address().id,
                encoded,
                vec![],
            ));
            let uds: UnboundedDirectoryString = x121_net_addr.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 18),
                encoded,
                vec![],
            ));
        }
        if let Some(num_id) = value.num_id.take() {
            // mHSNumericUserIdentifierAttribute { 2 6 10 3 20 }
            let uds: UnboundedDirectoryString = num_id.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 20),
                encoded,
                vec![],
            ));
        }
        if let Some(term_id) = value.term_id.take() {
            // mHSTerminalIdentifierAttribute { 2 6 10 3 28 }
        }
        if let Some(mut personal_name) = value.personal_name.take() {
            // mhs-surname { 2 6 10 3 27 }
            // mhs-givenname { 2 6 10 3 15 }
            // mhs-initials { 2 6 10 3 16 }
            // mhs-gq { 2 6 10 3 14 }
            if let Some(gn) = personal_name.given_name.take() {
                let uds: UnboundedDirectoryString = gn.into();
                let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
                rdn.push(AttributeTypeAndValue::new(
                    oid!(2, 6, 10, 3, 14),
                    encoded.clone(),
                    vec![],
                ));
                rdn.push(AttributeTypeAndValue::new(givenName().id, encoded, vec![]));
            }
            if let Some(inits) = personal_name.initials.take() {
                let uds: UnboundedDirectoryString = inits.into();
                let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
                rdn.push(AttributeTypeAndValue::new(
                    oid!(2, 6, 10, 3, 14),
                    encoded.clone(),
                    vec![],
                ));
                rdn.push(AttributeTypeAndValue::new(initials().id, encoded, vec![]));
            }
            if let Some(genq) = personal_name.generation_qualifier.take() {
                let uds: UnboundedDirectoryString = genq.into();
                let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
                rdn.push(AttributeTypeAndValue::new(
                    oid!(2, 6, 10, 3, 14),
                    encoded.clone(),
                    vec![],
                ));
                rdn.push(AttributeTypeAndValue::new(
                    generationQualifier().id,
                    encoded,
                    vec![],
                ));
            }
            let uds: UnboundedDirectoryString = personal_name.surname.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 27),
                encoded.clone(),
                vec![],
            ));
            rdn.push(AttributeTypeAndValue::new(surname().id, encoded, vec![]));
        }
        if let Some(common_name) = value.common_name.take() {
            // mHSCommonNameAttribute { 2 6 10 3 10 }
            let uds: UnboundedDirectoryString = common_name.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 10),
                encoded.clone(),
                vec![],
            ));
            rdn.push(AttributeTypeAndValue::new(commonName().id, encoded, vec![]));
        }
        if let Some(org_name) = value.org_name.take() {
            // mHSOrganizationName { 2 6 10 3 21 }
            let uds: UnboundedDirectoryString = org_name.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 21),
                encoded.clone(),
                vec![],
            ));
            rdn.push(AttributeTypeAndValue::new(
                organizationName().id,
                encoded,
                vec![],
            ));
        }
        for ou in value.ous.iter() {
            // mHSOrganizationalUnitName { 2 6 10 3 22 }
            let uds: UnboundedDirectoryString = ou.to_owned().into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 22),
                encoded.clone(),
                vec![],
            ));
            rdn.push(AttributeTypeAndValue::new(
                organizationalUnitName().id,
                encoded,
                vec![],
            ));
        }
        if let Some(pdo_name) = value.pdo_name.take() {
            // physicalDeliveryOfficeName
            let uds: UnboundedDirectoryString = pdo_name.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                physicalDeliveryOfficeName().id,
                encoded,
                vec![],
            ));
        }
        if let Some(street) = value.street.take() {
            // streetAddress
            let uds: UnboundedDirectoryString = street.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                streetAddress().id,
                encoded,
                vec![],
            ));
        }
        if value.pd_address.len() > 0 {
            let uds_lines: PostalAddress = value
                .pd_address
                .iter()
                .map(|line| line.to_owned().into())
                .collect();
            let encoded = postalAddress::_encode_Type(&uds_lines)?;
            rdn.push(AttributeTypeAndValue::new(
                postalAddress().id,
                encoded,
                vec![],
            ));
        }
        if let Some(po_box_addr) = value.po_box_addr.take() {
            // postOfficeBox
            let uds: UnboundedDirectoryString = po_box_addr.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                postOfficeBox().id,
                encoded,
                vec![],
            ));
        }
        if let Some(postal_code) = value.postal_code.take() {
            // mHSPostalCodeAttribute { 2 6 10 3 24 }
            let uds: UnboundedDirectoryString = postal_code.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 24),
                encoded.clone(),
                vec![],
            ));
            rdn.push(AttributeTypeAndValue::new(postalCode().id, encoded, vec![]));
        }
        if let Some(pd_svc_name) = value.pd_svc_name.take() {
            // id-at-mhs-pds-name-attribute ID ::= {id-at  23}
            let uds: UnboundedDirectoryString = pd_svc_name.into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 23),
                encoded,
                vec![],
            ));
        }
        if let Some(isdn) = value.isdn.take() {
            // mHSExtendedNetworkAddress { 2 6 10 3 13 } (Actually, not clear what the syntax should be.)
            let encoded = BER.encode_owned_numeric_string(isdn)?;
            rdn.push(AttributeTypeAndValue::new(
                internationalISDNNumber().id,
                encoded,
                vec![],
            ));
            // ~~telephoneNumber~~ (TelephoneNumber uses E.123 syntax.)
        }
        if let Some(psap) = value.psap.take() {
            // mHSExtendedNetworkAddress { 2 6 10 3 13 }
            let encoded = presentationAddress::_encode_Type(&psap)?;
            rdn.push(AttributeTypeAndValue::new(
                presentationAddress().id,
                encoded,
                vec![],
            ));
        }
        if let Some(term_type) = value.term_type.take() {
            // mHSExtendedNetworkAddress { 2 6 10 3 29 }
            // The size constraint seems to hint that this is expected to be in string form (e.g. g3fax)
            let uds: UnboundedDirectoryString = term_type_to_str(term_type).to_owned().into();
            let encoded = _encode_owned_UnboundedDirectoryString(uds)?;
            rdn.push(AttributeTypeAndValue::new(
                oid!(2, 6, 10, 3, 29),
                encoded,
                vec![],
            ));
        }
        Ok(rdn)
    }
}

impl TryFrom<ORAddress> for RelativeDistinguishedName {
    type Error = ASN1Error;

    fn try_from(value: ORAddress) -> Result<Self, Self::Error> {
        let info: ORAddressInfo = (&value).try_into()?;
        info.try_into()
    }

}

impl TryFrom<&ORAddress> for RelativeDistinguishedName {
    type Error = ASN1Error;

    fn try_from(value: &ORAddress) -> Result<Self, Self::Error> {
        let info: ORAddressInfo = value.try_into()?;
        info.try_into()
    }

}

impl TryFrom<&RelativeDistinguishedName> for ORAddressInfo {

    type Error = ASN1Error;

    fn try_from(value: &RelativeDistinguishedName) -> Result<Self, Self::Error> {
        // All of these come from the built-in attributes.
        let mut prmd: Option<String> = None; // P
        let mut admd: Option<String> = None; // A

        // Personal Name Fields
        let mut g: Option<String> = None;
        let mut sn: Option<String> = None;
        let mut i: Option<String> = None;
        let mut q: Option<String> = None;

        // This should only be the ISO-3166 code.
        let mut country: Option<CountryCode> = None; // C
        let mut x121_net_addr: Option<String> = None; // X.121
        let mut num_id: Option<NumericString> = None; // N-ID
        let mut term_id: Option<String> = None; // T-ID
        let mut personal_name: Option<PersonalNameInfo> = None; // G, I, S, Q
        let mut common_name: Option<String> = None; // CN
        let mut org_name: Option<String> = None; // O

         // OU2 - OU4 cannot be used, because no ordering can be inferred.
        let mut ous: Vec<String> = vec![]; // Only for OU1

        let mut pdo_name: Option<String> = None; // PD-OF
        let mut street: Option<String> = None; // PD-S
        let mut pd_address: Vec<String> = vec![]; // PD-A1 - PD-A6
        let mut po_box_addr: Option<String> = None; // PD-B
        let mut postal_code: Option<String> = None; // PD-PC
        let mut pd_svc_name: Option<String> = None; // PD-SN
        let pd_country_name: Option<CountryCode> = None; // PD-C
        let mut isdn: Option<String> = None; // ISDN (This is the E.163 or E.164 address)
        let mut psap: Option<PresentationAddress> = None; // PSAP
        let mut term_type: Option<TerminalType> = None; // T-TY
        let dda: HashMap<String, String> = HashMap::new(); // DDA: Domain-Defined Attribute


        for rdn in value.iter() {
            let type_x690_encoding = rdn.type_.as_x690_slice();
            if type_x690_encoding.len() == 4 && rdn.type_.as_x690_slice().starts_with(&[ 0x56, 10, 3 ]) {
                let last_arc = type_x690_encoding.last().unwrap();
                match last_arc {
                    9 => { // id-at-mhs-admd-name ID ::= {id-at  9}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_printable_str(&s) || s.len() > ub_domain_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        admd = Some(s);
                    },
                    10 => { // id-at-mhs-common-name ID ::= {id-at  10}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_printable_str(&s) || s.len() > ub_common_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        common_name = Some(s);
                    },
                    11 => { // id-at-mhs-country-name ID ::= {id-at  11}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        let mut cc = s.as_str();
                        if is_numeric_str(&s) {
                            let dcc = u16::from_str(&s)
                                .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                            let maybe_iso = x121_dcc_country_code_to_iso_3166(dcc);
                            if maybe_iso.is_none() {
                                return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                            }
                            let iso = maybe_iso.unwrap();
                            cc = iso;
                        }
                        else if is_printable_str(&s) {
                            if s.len() < 2 || s.len() > 3 {
                                return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                            }
                        }
                        else {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        let c = CountryCode::for_alpha2_caseless(&cc)
                            .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        country = Some(c);
                    },
                    14 => { // id-at-mhs-generation-qualifier ID ::= {id-at  14}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if s.chars().count() > ub_universal_generation_qualifier_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        q = Some(uds.to_string());
                    },
                    15 => { // id-at-mhs-given-name ID ::= {id-at  15}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if s.chars().count() > ub_universal_given_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        g = Some(uds.to_string());
                    },
                    16 => { // id-at-mhs-initials ID ::= {id-at  16}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if s.chars().count() > ub_universal_initials_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        i = Some(uds.to_string());
                    },
                    18 => { // id-at-mhs-network-address ID ::= {id-at  18}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_numeric_str(&s) {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        x121_net_addr = Some(s);
                    },
                    20 => { // id-at-mhs-numeric-user-identifier ID ::= {id-at  20}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_numeric_str(&s) || s.len() > ub_numeric_user_id_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        num_id = Some(s);
                    },
                    21 => { // id-at-mhs-organization-name ID ::= {id-at  21}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if s.chars().count() > ub_organization_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        org_name = Some(s);
                    },
                    22 => { // id-at-mhs-organizational-unit-name ID ::= {id-at  22}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if s.chars().count() > ub_organizational_unit_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        if ous.len() == 0 {
                            ous.push(uds.to_string());
                        }
                    },
                    23 => { // id-at-mhs-pds-name-attribute ID ::= {id-at  23}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_printable_str(&s) || s.len() > ub_pds_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        pd_svc_name = Some(s);
                    },
                    24 => { // id-at-mhs-postal-code ID ::= {id-at  24}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_printable_str(&s) || s.len() > ub_postal_code_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        postal_code = Some(s);
                    },
                    25 => { // id-at-mhs-prmd-name ID ::= {id-at  25}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_printable_str(&s) || s.len() > ub_domain_name_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        prmd = Some(s);
                    },
                    27 => { // id-at-mhs-surname ID ::= {id-at  27}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if s.chars().count() > ub_universal_surname_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        sn = Some(uds.to_string());
                    },
                    28 => { // id-at-mhs-terminal-identifier ID ::= {id-at  28}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        if !is_printable_str(&s) || s.len() > ub_terminal_id_length {
                            return Err(ASN1Error::new(ASN1ErrorCode::constraint_violation));
                        }
                        term_id = Some(s);
                    },
                    29 => { // id-at-mhs-terminal-type ID ::= {id-at  29}
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        let s = uds.to_string();
                        term_type = term_type_from_str(&s);
                    },
                    _ => {}, // Do nothing for unrecognized attributes.
                }
            }
            if type_x690_encoding.len() == 3 && type_x690_encoding.starts_with(&[ 0x55, 4 ]) {
                let last_arc = type_x690_encoding.last().unwrap();
                match last_arc {
                    3 => { // commonName
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        common_name = Some(uds.to_string());
                    },
                    4 => { // surname
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        sn = Some(uds.to_string());
                    },
                    6 => { // countryName
                        let cc = countryName::_decode_Type(&rdn.value)?;
                        let c = CountryCode::for_alpha2_caseless(cc.as_str())
                            .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        country = Some(c);
                    },
                    9 => { // streetAddress
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        street = Some(uds.to_string());
                    },
                    10 => { // organizationName
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        org_name = Some(uds.to_string());
                    },
                    11 => { // organizationalUnitName
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        if ous.len() == 0 {
                            ous.push(uds.to_string());
                        } else {
                            // TODO: What to do here?
                        }
                    },
                    16 => { // postalAddress
                        let v = postalAddress::_decode_Type(&rdn.value)?.iter()
                            .map(|line| line.to_string())
                            .collect();
                        pd_address = v;
                    },
                    17 => { // postalCode
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        postal_code = Some(uds.to_string());
                    },
                    18 => { // postOfficeBox
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        po_box_addr = Some(uds.to_string());
                    },
                    19 => { // physicalDeliveryOfficeName
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        pdo_name = Some(uds.to_string());
                    },
                    24 => { // x121Address
                        let v = x121Address::_decode_Type(&rdn.value)?;
                        x121_net_addr = Some(v);
                    },
                    25 => { // internationalISDNNumber
                        let v = internationalISDNNumber::_decode_Type(&rdn.value)?;
                        isdn = Some(v);
                    },
                    29 => { // presentationAddress
                        let v = presentationAddress::_decode_Type(&rdn.value)?;
                        psap = Some(v);
                    },
                    42 => { // givenName
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        g = Some(uds.to_string());
                    },
                    43 => { // initials
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        i = Some(uds.to_string());
                    },
                    44 => { // generationQualifier
                        let uds = _decode_UnboundedDirectoryString(&rdn.value)?;
                        q = Some(uds.to_string());
                    },
                    98 => { // countryCode3c
                        let cc = countryCode3c::_decode_Type(&rdn.value)?;
                        let c = CountryCode::for_alpha3_caseless(cc.as_str())
                            .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        country = Some(c);
                    },
                    99 => { // countryCode3n
                        let cc = countryCode3n::_decode_Type(&rdn.value)?;
                        let cc_int = u32::from_str(&cc)
                            .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        let c = CountryCode::for_id(cc_int)
                            .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        country = Some(c);
                    },
                    _ => {}, // Do nothing for unrecognized attributes.
                }
            }
        }
        if let Some(sn) = sn {
            personal_name = Some(PersonalNameInfo::new(sn, g, i, q));
        }
        Ok(ORAddressInfo {
            prmd,
            admd,
            country,
            x121_net_addr,
            num_id,
            term_id,
            personal_name,
            common_name,
            org_name,
            ous,
            pdo_name,
            street,
            pd_address,
            po_box_addr,
            postal_code,
            pd_svc_name,
            pd_country_name,
            isdn,
            psap,
            term_type,
            ..Default::default()
        })
    }

}

impl TryFrom<&RelativeDistinguishedName> for ORAddress {

    type Error = ASN1Error;

    fn try_from(value: &RelativeDistinguishedName) -> Result<Self, Self::Error> {
        let info: ORAddressInfo = value.try_into()?;
        info.try_into()
    }

}

/// This function does not consider diacritics.
fn case_ignore_compare_str(a: &str, b: &str) -> bool {
    let a_trim = a.trim();
    let b_trim = b.trim();
    if a_trim.len() < 32 && a_trim.is_ascii() && a_trim.eq_ignore_ascii_case(b_trim) {
        return true;
    }
    if a_trim.len() != b_trim.len() {
        return false;
    }
    let non_match = a_trim
        .chars()
        .zip(b_trim.chars())
        .any(|(ac, bc)| !ac.to_uppercase().eq(bc.to_uppercase()));
    !non_match
}

/// This function does not consider diacritics.
fn case_ignore_compare_opt_str(a: &Option<String>, b: &Option<String>) -> bool {
    match (a, b) {
        (Some(a_str), Some(b_str)) => case_ignore_compare_str(&a_str, &b_str),
        (None, None) => true,
        _ => false,
    }
}

/// This function does not consider diacritics.
fn compare_opt_num_str(a: &Option<String>, b: &Option<String>) -> bool {
    match (a, b) {
        (Some(a_str), Some(b_str)) => compare_numeric_string(&a_str, &b_str),
        (None, None) => true,
        _ => false,
    }
}

impl PartialEq for ORAddressInfo {
    /// NOTE: This is not technically correct, because it does not remove
    /// multiple consecutive inner whitespaces, but this is "close enough."
    fn eq(&self, other: &Self) -> bool {
        if self.ous.len() != other.ous.len() {
            return false;
        }
        for i in 0..self.ous.len() {
            let ou_a = &self.ous[i];
            let ou_b = &other.ous[i];
            if ou_a.cow_to_uppercase().as_ref().trim() != ou_b.cow_to_uppercase().as_ref().trim() {
                return false;
            }
        }
        if self.pd_address.len() != other.pd_address.len() {
            return false;
        }
        for i in 0..self.pd_address.len() {
            let line_a = &self.pd_address[i];
            let line_b = &other.pd_address[i];
            if line_a.cow_to_uppercase().as_ref().trim()
                != line_b.cow_to_uppercase().as_ref().trim()
            {
                return false;
            }
        }
        if self.dda.len() != other.dda.len() {
            return false;
        }
        // NOTE: The spec does NOT say that DDAs get trimmed or compared without
        // respect to casing.
        for (k, v) in self.dda.iter() {
            if let Some(other_v) = other.dda.get(k) {
                if v != other_v {
                    return false;
                }
            } else {
                return false;
            }
        }
        self.personal_name == other.personal_name
        && self.country == other.country
        && case_ignore_compare_opt_str(&self.prmd, &other.prmd)
        && ( // See ITU-T Rec. X.402 (1999), Section 18.3.1, First NOTE.
            self.admd.as_ref().is_some_and(|admd| admd.as_str() == " ")
            || other.admd.as_ref().is_some_and(|admd| admd.as_str() == " ")
            || case_ignore_compare_opt_str(&self.admd, &other.admd)
        )
        && compare_opt_num_str(&self.x121_net_addr, &other.x121_net_addr)
        && compare_opt_num_str(&self.num_id, &other.num_id)
        && case_ignore_compare_opt_str(&self.term_id, &other.term_id)
        && case_ignore_compare_opt_str(&self.common_name, &other.common_name)
        && case_ignore_compare_opt_str(&self.org_name, &other.org_name)
        && case_ignore_compare_opt_str(&self.pd_person, &other.pd_person)
        && case_ignore_compare_opt_str(&self.pd_ea, &other.pd_ea)
        && case_ignore_compare_opt_str(&self.pd_ed, &other.pd_ed)
        && case_ignore_compare_opt_str(&self.pdo_number, &other.pdo_number)
        && case_ignore_compare_opt_str(&self.pdo_name, &other.pdo_name)
        && case_ignore_compare_opt_str(&self.pd_org_name, &other.pd_org_name)
        && case_ignore_compare_opt_str(&self.street, &other.street)
        && case_ignore_compare_opt_str(&self.pd_unique, &other.pd_unique)
        && case_ignore_compare_opt_str(&self.pd_local, &other.pd_local)
        && case_ignore_compare_opt_str(&self.postal_restante_arr, &other.postal_restante_arr)
        && case_ignore_compare_opt_str(&self.po_box_addr, &other.po_box_addr)
        && case_ignore_compare_opt_str(&self.postal_code, &other.postal_code)
        && case_ignore_compare_opt_str(&self.pd_svc_name, &other.pd_svc_name)
        // NOTE: This string will have a slash in it, but the compare_numeric_string function should still work.
        && compare_opt_num_str(&self.isdn, &other.isdn)
        && self.psap == other.psap
        && self.term_type == other.term_type
        && self.pd_country_name == other.pd_country_name
    }
}

impl TryFrom<&ORAddress> for ORAddressInfo {
    type Error = ASN1Error;

    fn try_from(value: &ORAddress) -> Result<Self, Self::Error> {
        // All of these come from the built-in attributes.
        let prmd: Option<String> = value
            .built_in_standard_attributes
            .private_domain_name
            .as_ref()
            .map(|a| a.to_string()); // P
        let admd: Option<String> = value
            .built_in_standard_attributes
            .administration_domain_name
            .as_ref()
            .map(|a| a.to_string()); // A
        let country: Option<CountryCode> = {
            if let Some(c) = &value.built_in_standard_attributes.country_name {
                match c {
                    CountryName::x121_dcc_code(dcc) => {
                        let dcc_int = u16::from_str(&dcc)
                            .map_err(|_| ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        let iso = x121_dcc_country_code_to_iso_3166(dcc_int)
                            .ok_or(ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                        CountryCode::for_alpha2(&iso).ok()
                    }
                    CountryName::iso_3166_alpha2_code(iso) => CountryCode::for_alpha2_caseless(&iso).ok(),
                }
            } else {
                None
            }
        };
        let x121_net_addr: Option<String> =
            value.built_in_standard_attributes.network_address.clone(); // X.121
        let num_id: Option<NumericString> = value
            .built_in_standard_attributes
            .numeric_user_identifier
            .clone(); // N-ID
        let term_id: Option<String> = value
            .built_in_standard_attributes
            .terminal_identifier
            .clone(); // T-ID

        let mut personal_name: Option<PersonalNameInfo> = None; // G, I, S, Q
        let mut common_name: Option<String> = None; // CN
        let mut org_name: Option<String> = None; // O
        let mut ous: Vec<String> = vec![]; // OU1, OU2, OU3, OU4
        let mut pd_person: Option<String> = None; // PD-PN
        let mut pd_ea: Option<String> = None; // PD-EA
        let mut pd_ed: Option<String> = None; // PD-ED
        let mut pdo_number: Option<String> = None; // PD-OFN
        let mut pdo_name: Option<String> = None; // PD-OF
        let mut pd_org_name: Option<String> = None; // PD-O
        let mut street: Option<String> = None; // PD-S
        let mut pd_address: Vec<String> = vec![]; // PD-A1 - PD-A6
        let mut pd_unique: Option<String> = None; // PD-U
        let mut pd_local: Option<String> = None; // PD-L
        let mut postal_restante_arr: Option<String> = None; // PD-R
        let mut po_box_addr: Option<String> = None; // PD-B
        let mut postal_code: Option<String> = None; // PD-PC
        let mut pd_svc_name: Option<String> = None; // PD-SN
        let mut pd_country_name: Option<CountryCode> = None; // PD-C
        let mut isdn: Option<String> = None; // ISDN (This is the E.163 or E.164 address)
        let mut psap: Option<PresentationAddress> = None; // PSAP
        let mut term_type: Option<TerminalType> = None; // T-TY
        let mut dda: HashMap<String, String> = HashMap::new(); // DDA: Domain-Defined Attribute

        if let Some(exts) = &value.extension_attributes {
            for ext in exts {
                if ext.extension_attribute_type.len() == 0 {
                    continue; // This should never happen, but just making sure.
                }
                let value = &ext.extension_attribute_value;
                // All extensions are < 127, so you can just use the one byte.
                match ext.extension_attribute_type[0] as i64 {
                    id_common_name => {
                        let v = BER.decode_printable_string(value)?;
                        common_name = Some(v);
                    }
                    id_teletex_common_name => {
                        let v = BER.decode_t61_string(value)?;
                        common_name = Some(teletex_to_utf8(&v).into_owned());
                    }
                    id_universal_common_name => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        common_name = Some(v.to_str().into());
                    }
                    id_teletex_organization_name => {
                        let v = BER.decode_t61_string(value)?;
                        org_name = Some(teletex_to_utf8(&v).into_owned());
                    }
                    id_universal_organization_name => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        org_name = Some(v.to_str().into());
                    }
                    id_teletex_personal_name => {
                        let v = _decode_TeletexPersonalName(value)?;
                        personal_name = Some((&v).into());
                    }
                    id_universal_personal_name => {
                        let v = _decode_UniversalPersonalName(value)?;
                        personal_name = Some((&v).into());
                    }
                    id_teletex_organizational_unit_names => {
                        let v = _decode_TeletexOrganizationalUnitNames(value)?;
                        ous = v.iter().map(|s| teletex_to_utf8(&s).into_owned()).collect();
                    }
                    id_universal_organizational_unit_names => {
                        let v = _decode_UniversalOrganizationalUnitNames(value)?;
                        ous = v.iter().map(|s| s.to_str().into()).collect();
                    }
                    id_teletex_domain_defined_attributes => {
                        let v = _decode_TeletexDomainDefinedAttributes(value)?;
                        for a in v {
                            let attr: DomainDefinedAttribute = (&a).into();
                            dda.insert(attr.type_, attr.value);
                        }
                    }
                    id_universal_domain_defined_attributes => {
                        let v = _decode_UniversalDomainDefinedAttributes(value)?;
                        for a in v {
                            let attr: DomainDefinedAttribute = (&a).into();
                            dda.insert(attr.type_, attr.value);
                        }
                    }
                    id_pds_name => {
                        let v = BER.decode_printable_string(value)?;
                        pd_svc_name = Some(v);
                    }
                    id_physical_delivery_country_name => {
                        let v = _decode_PhysicalDeliveryCountryName(value)?;
                        match v {
                            PhysicalDeliveryCountryName::x121_dcc_code(dcc) => {
                                let dcc_int = u16::from_str(&dcc).map_err(|_| {
                                    ASN1Error::new(ASN1ErrorCode::constraint_violation)
                                })?;
                                let iso = x121_dcc_country_code_to_iso_3166(dcc_int)
                                    .ok_or(ASN1Error::new(ASN1ErrorCode::constraint_violation))?;
                                if let Ok(c) = CountryCode::for_alpha2(&iso) {
                                    pd_country_name = Some(c);
                                }
                            }
                            PhysicalDeliveryCountryName::iso_3166_alpha2_code(iso) => {
                                if let Ok(c) = CountryCode::for_alpha2_caseless(&iso) {
                                    pd_country_name = Some(c);
                                }
                            }
                        };
                    }
                    id_postal_code => {
                        let v = _decode_PostalCode(value)?;
                        postal_code = Some(v.to_string());
                    }
                    id_physical_delivery_office_name => {
                        let v = _decode_PDSParameter(value)?;
                        pdo_name = Some(v.to_string());
                    }
                    id_universal_physical_delivery_office_name => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pdo_name = Some(v.to_str().to_owned());
                    }
                    id_physical_delivery_office_number => {
                        let v = _decode_PDSParameter(value)?;
                        pdo_number = Some(v.to_string()); // Yes, it is a string, not a number.
                    }
                    id_universal_physical_delivery_office_number => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pdo_number = Some(v.to_str().to_owned());
                    }
                    id_extension_OR_address_components => {
                        let v = _decode_PDSParameter(value)?;
                        pd_ea = Some(v.to_string());
                    }
                    id_universal_extension_OR_address_components => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_ea = Some(v.to_str().to_owned());
                    }
                    id_physical_delivery_personal_name => {
                        let v = _decode_PDSParameter(value)?;
                        pd_person = Some(v.to_string());
                    }
                    id_universal_physical_delivery_personal_name => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_person = Some(v.to_str().to_owned());
                    }
                    id_physical_delivery_organization_name => {
                        let v = _decode_PDSParameter(value)?;
                        pd_org_name = Some(v.to_string());
                    }
                    id_universal_physical_delivery_organization_name => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_org_name = Some(v.to_str().to_owned());
                    }
                    id_extension_physical_delivery_address_components => {
                        let v = _decode_PDSParameter(value)?;
                        pd_ed = Some(v.to_string());
                    }
                    id_universal_extension_physical_delivery_address_components => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_ed = Some(v.to_str().to_owned());
                    }
                    id_unformatted_postal_address => {
                        let v = _decode_UnformattedPostalAddress(value)?;
                        pd_address = v.into();
                    }
                    id_universal_unformatted_postal_address => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_address = v.to_str().lines().map(|line| line.to_owned()).collect();
                    }
                    id_street_address => {
                        let v = _decode_PDSParameter(value)?;
                        street = Some(v.to_string());
                    }
                    id_universal_street_address => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        street = Some(v.to_str().to_owned());
                    }
                    id_post_office_box_address => {
                        let v = _decode_PDSParameter(value)?;
                        po_box_addr = Some(v.to_string());
                    }
                    id_universal_post_office_box_address => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        po_box_addr = Some(v.to_str().to_owned());
                    }
                    id_poste_restante_address => {
                        let v = _decode_PDSParameter(value)?;
                        postal_restante_arr = Some(v.to_string());
                    }
                    id_universal_poste_restante_address => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        postal_restante_arr = Some(v.to_str().to_owned());
                    }
                    id_unique_postal_name => {
                        let v = _decode_PDSParameter(value)?;
                        pd_unique = Some(v.to_string());
                    }
                    id_universal_unique_postal_name => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_unique = Some(v.to_str().to_owned());
                    }
                    id_local_postal_attributes => {
                        let v = _decode_PDSParameter(value)?;
                        pd_local = Some(v.to_string());
                    }
                    id_universal_local_postal_attributes => {
                        let v = _decode_UniversalOrBMPString(value)?;
                        pd_local = Some(v.to_str().to_owned());
                    }
                    id_extended_network_address => {
                        let v = _decode_ExtendedNetworkAddress(value)?;
                        match v {
                            ExtendedNetworkAddress::e163_4_address(isdn_addr) => {
                                // I cannot find information on how the subaddress
                                // is to be printed, nor whether it is the same
                                // as an extension.
                                if let Some(subaddress) = isdn_addr.sub_address {
                                    isdn = Some(format!("{}/{}", isdn_addr.number, subaddress));
                                } else {
                                    isdn = Some(isdn_addr.number);
                                }
                            }
                            ExtendedNetworkAddress::psap_address(psap_addr) => {
                                psap = Some(psap_addr);
                            }
                        }
                    }
                    id_terminal_type => {
                        let v = BER.decode_u16(value)?;
                        term_type = Some(v);
                    }
                    _ => {}
                };
            }
        }

        Ok(ORAddressInfo {
            prmd,
            admd,
            country,
            x121_net_addr,
            num_id,
            term_id,
            personal_name,
            common_name,
            org_name,
            ous,
            pd_person,
            pd_ea,
            pd_ed,
            pdo_number,
            pdo_name,
            pd_org_name,
            street,
            pd_address,
            pd_unique,
            pd_local,
            postal_restante_arr,
            po_box_addr,
            postal_code,
            pd_svc_name,
            pd_country_name,
            isdn,
            psap,
            term_type,
            dda,
        })
    }
}

impl Display for ORAddressInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pn) = &self.personal_name {
            if let Some(gn) = &pn.given_name {
                f.write_str("G=")?;
                f.write_str(remove_oraddress_chars(gn).as_ref())?;
                f.write_char(';')?;
            }
            if let Some(i) = &pn.initials {
                f.write_str("I=")?;
                f.write_str(remove_oraddress_chars(i).as_ref())?;
                f.write_char(';')?;
            }
            f.write_str("S=")?;
            f.write_str(remove_oraddress_chars(&pn.surname).as_ref())?;
            f.write_char(';')?;
            if let Some(gq) = &pn.generation_qualifier {
                f.write_str("Q=")?;
                f.write_str(remove_oraddress_chars(gq).as_ref())?;
                f.write_char(';')?;
            }
        }
        if let Some(common_name) = &self.common_name {
            f.write_str("CN=")?;
            f.write_str(&common_name)?;
            f.write_char(';')?;
        }
        if let Some(x121) = &self.x121_net_addr {
            f.write_str("X.121=")?;
            f.write_str(&x121)?;
            f.write_char(';')?;
        }
        if let Some(num_id) = &self.num_id {
            f.write_str("N-ID=")?;
            f.write_str(&num_id)?;
            f.write_char(';')?;
        }
        if let Some(term_id) = &self.term_id {
            f.write_str("T-ID=")?;
            f.write_str(&term_id)?;
            f.write_char(';')?;
        }
        if let Some(ttype) = &self.term_type {
            f.write_str("T-TY=")?;
            f.write_str(&term_type_to_str(*ttype))?;
            f.write_char(';')?;
        }
        if let Some(isdn) = &self.isdn {
            f.write_str("ISDN=")?;
            f.write_str(&isdn)?;
            f.write_char(';')?;
        }
        if let Some(org) = &self.org_name {
            f.write_str("O=")?;
            f.write_str(remove_oraddress_chars(org).as_ref())?;
            f.write_char(';')?;
        }
        for (i, ou) in self.ous.iter().take(4).enumerate() {
            f.write_str("OU")?;
            f.write_char(((i as u8 + 1) + 0x30u8).into())?;
            f.write_char('=')?;
            f.write_str(remove_oraddress_chars(ou).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(prmd) = &self.prmd {
            f.write_str("P=")?;
            f.write_str(remove_oraddress_chars(prmd).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(admd) = &self.admd {
            f.write_str("A=")?;
            f.write_str(remove_oraddress_chars(admd).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(country) = &self.country {
            f.write_str("C=")?;
            f.write_str(&country.to_string())?;
            f.write_char(';')?;
        }
        if let Some(psap) = &self.psap {
            f.write_str("PSAP=")?;
            f.write_str(remove_oraddress_chars(&psap.to_string()).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_person) = &self.pd_person {
            f.write_str("PD-PN=")?;
            f.write_str(remove_oraddress_chars(&pd_person).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(po_box_addr) = &self.po_box_addr {
            f.write_str("PD-B=")?;
            f.write_str(remove_oraddress_chars(&po_box_addr).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(street) = &self.street {
            f.write_str("PD-S=")?;
            f.write_str(remove_oraddress_chars(&street).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_org_name) = &self.pd_org_name {
            f.write_str("PD-O=")?;
            f.write_str(remove_oraddress_chars(&pd_org_name).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(postal_code) = &self.postal_code {
            f.write_str("PD-PC=")?;
            f.write_str(remove_oraddress_chars(&postal_code).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pdo_number) = &self.pdo_number {
            f.write_str("PD-OFN=")?;
            f.write_str(remove_oraddress_chars(&pdo_number).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pdo_name) = &self.pdo_name {
            f.write_str("PD-OF=")?;
            f.write_str(remove_oraddress_chars(&pdo_name).as_ref())?;
            f.write_char(';')?;
        }
        for (i, line) in self.pd_address.iter().take(6).enumerate() {
            f.write_str("PD-A")?;
            f.write_str(&(i + 1).to_string())?;
            f.write_char('=')?;
            f.write_str(remove_oraddress_chars(&line).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_unique) = &self.pd_unique {
            f.write_str("PD-U=")?;
            f.write_str(remove_oraddress_chars(&pd_unique).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_local) = &self.pd_local {
            f.write_str("PD-L=")?;
            f.write_str(remove_oraddress_chars(&pd_local).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_svc_name) = &self.pd_svc_name {
            f.write_str("PD-L=")?;
            f.write_str(remove_oraddress_chars(&pd_svc_name).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_country_name) = &self.pd_country_name {
            f.write_str("PD-C=")?;
            f.write_str(&pd_country_name.to_string())?;
            f.write_char(';')?;
        }
        if let Some(postal_restante_arr) = &self.postal_restante_arr {
            f.write_str("PD-R=")?;
            f.write_str(remove_oraddress_chars(&postal_restante_arr).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_ed) = &self.pd_ed {
            f.write_str("PD-ED=")?;
            f.write_str(remove_oraddress_chars(&pd_ed).as_ref())?;
            f.write_char(';')?;
        }
        if let Some(pd_ea) = &self.pd_ea {
            f.write_str("PD-EA=")?;
            f.write_str(remove_oraddress_chars(&pd_ea).as_ref())?;
            f.write_char(';')?;
        }
        for (k, attr) in self.dda.iter() {
            f.write_str("DDA:")?;
            f.write_str(remove_oraddress_chars(k).cow_replace("=", "==").as_ref())?;
            f.write_char('=')?;
            f.write_str(remove_oraddress_chars(&attr).as_ref())?;
            f.write_char(';')?;
        }
        Ok(())
    }
}

impl PersonalNameInfo {
    pub const fn new(
        surname: String,
        given_name: Option<String>,
        initials: Option<String>,
        generation_qualifier: Option<String>,
    ) -> Self {
        PersonalNameInfo {
            surname,
            given_name,
            initials,
            generation_qualifier,
        }
    }

    pub fn to_string(&self) -> String {
        let mut out: String = String::with_capacity(
            2 // For S=
            + self.surname.len()
            + self.given_name.as_ref().and_then(|n| Some(n.len() + 3)).unwrap_or(0)
            + self.initials.as_ref().and_then(|n| Some(n.len() + 3)).unwrap_or(0)
            + self.generation_qualifier.as_ref().and_then(|n| Some(n.len() + 3)).unwrap_or(0),
        );
        if let Some(gn) = &self.given_name {
            out.push_str("G=");
            out.push_str(gn.cow_replace(";", "").as_ref());
            out.push(';');
        }
        if let Some(i) = &self.initials {
            out.push_str("I=");
            out.push_str(i.cow_replace(";", "").as_ref());
            out.push(';');
        }
        out.push_str("S=");
        out.push_str(&self.surname.cow_replace(";", "").as_ref());
        out.push(';');
        if let Some(gq) = &self.generation_qualifier {
            out.push_str("Q=");
            out.push_str(gq.cow_replace(";", "").as_ref());
        }
        out
    }
}

impl Display for PersonalNameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(gn) = &self.given_name {
            f.write_str("G=")?;
            f.write_str(gn.cow_replace(";", "").as_ref())?;
            f.write_char(';')?;
        }
        if let Some(i) = &self.initials {
            f.write_str("I=")?;
            f.write_str(i.cow_replace(";", "").as_ref())?;
            f.write_char(';')?;
        }
        f.write_str("S=")?;
        f.write_str(&self.surname.cow_replace(";", "").as_ref())?;
        f.write_char(';')?;
        if let Some(gq) = &self.generation_qualifier {
            f.write_str("Q=")?;
            f.write_str(gq.cow_replace(";", "").as_ref())?;
        }
        Ok(())
    }
}

impl From<&TeletexPersonalName> for PersonalNameInfo {
    fn from(value: &TeletexPersonalName) -> Self {
        let surname: String = teletex_to_utf8(&value.surname).into_owned();
        let mut given_name: Option<String> = Default::default();
        let mut initials: Option<String> = Default::default();
        let mut generation_qualifier: Option<String> = Default::default();
        if let Some(n) = &value.given_name {
            given_name = Some(teletex_to_utf8(&n).into_owned());
        }
        if let Some(n) = &value.initials {
            initials = Some(teletex_to_utf8(&n).into_owned());
        }
        if let Some(n) = &value.generation_qualifier {
            generation_qualifier = Some(teletex_to_utf8(&n).into_owned());
        }
        PersonalNameInfo {
            surname,
            given_name,
            initials,
            generation_qualifier,
        }
    }
}

impl From<&UniversalPersonalName> for PersonalNameInfo {
    fn from(value: &UniversalPersonalName) -> Self {
        let surname: String = value.surname.to_str().to_owned();
        let given_name: Option<String> = value
            .given_name
            .as_ref()
            .and_then(|n| Some(n.to_str().to_owned()));
        let initials: Option<String> = value
            .initials
            .as_ref()
            .and_then(|n| Some(n.to_str().to_owned()));
        let generation_qualifier: Option<String> = value
            .generation_qualifier
            .as_ref()
            .and_then(|n| Some(n.to_str().to_owned()));
        PersonalNameInfo {
            surname,
            given_name,
            initials,
            generation_qualifier,
        }
    }
}

impl Display for TeletexPersonalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info: PersonalNameInfo = self.into();
        info.fmt(f)
    }
}

impl Display for UniversalPersonalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info: PersonalNameInfo = self.into();
        info.fmt(f)
    }
}

impl PhysicalDeliveryCountryName {
    pub fn to_string(&self) -> String {
        match self {
            PhysicalDeliveryCountryName::iso_3166_alpha2_code(c) => c.to_owned(),
            PhysicalDeliveryCountryName::x121_dcc_code(c) => c.to_owned(),
        }
    }
}

impl Display for PhysicalDeliveryCountryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("C=")?;
        match self {
            PhysicalDeliveryCountryName::iso_3166_alpha2_code(c) => f.write_str(c),
            PhysicalDeliveryCountryName::x121_dcc_code(c) => f.write_str(c),
        }
    }
}

impl AsRef<str> for PhysicalDeliveryCountryName {
    fn as_ref(&self) -> &str {
        match self {
            PhysicalDeliveryCountryName::iso_3166_alpha2_code(c) => c,
            PhysicalDeliveryCountryName::x121_dcc_code(c) => c,
        }
    }
}

impl PDSParameter {
    pub fn to_string(&self) -> String {
        if let Some(s) = &self.printable_string {
            return s.to_owned();
        }
        if let Some(s) = &self.teletex_string {
            return teletex_to_utf8(s).into_owned();
        }
        "".into()
    }
}

impl Display for PDSParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.printable_string {
            return f.write_str(s);
        }
        if let Some(s) = &self.teletex_string {
            return f.write_str(teletex_to_utf8(s).as_ref());
        }
        Ok(())
    }
}

impl Into<Vec<String>> for UnformattedPostalAddress {
    fn into(self) -> Vec<String> {
        if let Some(printable_addr) = self.printable_address {
            return printable_addr;
        }
        if let Some(telex_addr) = self.teletex_string {
            return teletex_to_utf8(&telex_addr)
                .lines()
                .map(|line| line.to_owned())
                .collect();
        }
        vec![]
    }
}

impl Display for UnformattedPostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(printable_addr) = &self.printable_address {
            for line in printable_addr {
                f.write_str(line)?;
                f.write_str("\r\n")?; // X.402 says lines should be separated by CRLF.
            }
            return Ok(());
        }
        if let Some(telex_addr) = &self.teletex_string {
            let utf8_addr = teletex_to_utf8(telex_addr);
            return f.write_str(&utf8_addr);
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct DomainDefinedAttribute {
    pub type_: String,
    pub value: String,
}

impl DomainDefinedAttribute {
    pub fn to_string(&self) -> String {
        let mut out: String = String::with_capacity(
            4 // For DDA:
            + self.type_.len()
            + 1 // for =
            + self.value.len(),
        );
        out.push_str("DDA:");
        out.push_str(&self.type_.cow_replace(";", "").cow_replace("=", ""));
        out.push('=');
        out.push_str(&self.value.cow_replace(";", "").cow_replace("=", ""));
        out
    }
}

impl Display for DomainDefinedAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DDA:")?;
        f.write_str(&self.type_.cow_replace(";", "").cow_replace("=", ""))?;
        f.write_char('=')?;
        f.write_str(&self.value.cow_replace(";", "").cow_replace("=", ""))
    }
}

impl From<&TeletexDomainDefinedAttribute> for DomainDefinedAttribute {
    fn from(value: &TeletexDomainDefinedAttribute) -> Self {
        let type_ = teletex_to_utf8(&value.type_).into_owned();
        let value = teletex_to_utf8(&value.value).into_owned();
        DomainDefinedAttribute { type_, value }
    }
}

impl From<&UniversalDomainDefinedAttribute> for DomainDefinedAttribute {
    fn from(value: &UniversalDomainDefinedAttribute) -> Self {
        let type_ = value.type_.to_str().to_owned();
        let value = value.value.to_str().to_owned();
        DomainDefinedAttribute { type_, value }
    }
}

impl PostalCode {
    pub fn to_string(&self) -> String {
        match self {
            PostalCode::numeric_code(c) => c.to_owned(),
            PostalCode::printable_code(c) => c.to_owned(),
        }
    }
}

impl Display for PostalCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PD-PC=")?;
        match self {
            PostalCode::numeric_code(c) => f.write_str(c),
            PostalCode::printable_code(c) => f.write_str(c),
        }
    }
}

impl AsRef<str> for PostalCode {
    fn as_ref(&self) -> &str {
        match self {
            PostalCode::numeric_code(c) => c,
            PostalCode::printable_code(c) => c,
        }
    }
}

impl Display for AdministrationDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdministrationDomainName::numeric(num_str) => f.write_str(&num_str),
            AdministrationDomainName::printable(print_str) => f.write_str(&print_str),
        }
    }
}

impl Display for PrivateDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivateDomainName::numeric(num_str) => f.write_str(&num_str),
            PrivateDomainName::printable(print_str) => f.write_str(&print_str),
        }
    }
}

#[derive(Debug)]
pub enum ORAddressError {
    // Duplicate values are definitely allowed. See ITU-T Rec. X.402, Section 18.2.
    // But I can't find any documentation on whether duplicate extension attributes are allowed.
    // I will assume that they are not allowed, since the syntax of some of them
    // is SEQUENCE OF, which I don't think would have been done if duplicates were allowed.
    DuplicateExtensionAttribute(i64),
    EncodingError(ASN1Error),
}

impl Display for ORAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ORAddressError::EncodingError(e) => {
                f.write_str("encoding_error:")?;
                e.fmt(f)
            }
            ORAddressError::DuplicateExtensionAttribute(id) => {
                f.write_fmt(format_args!("duplicate_extension:{}", id))
            }
        }
    }
}

impl Error for ORAddressError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ORAddressError::EncodingError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ASN1Error> for ORAddressError {
    fn from(value: ASN1Error) -> Self {
        ORAddressError::EncodingError(value)
    }
}

/// Removes semi-colons and all control characters from the string, including newlines.
fn remove_oraddress_chars<'a>(s: &'a str) -> Cow<'a, str> {
    if s.chars().any(|c| c.is_control()) {
        return Cow::Owned(s.chars().filter(|c| !c.is_control() && *c != ';').collect());
    }
    s.cow_replace(";", "")
}

impl ORAddress {

    /// Returns true if there are no extensions, not domain-defined attributes,
    /// and nothing in the built-in standard attributes.
    pub fn is_empty (&self) -> bool {
        self.extension_attributes.is_none()
        && self.built_in_domain_defined_attributes.is_none()
        && self.built_in_standard_attributes.is_empty()
    }

    /// See ITU-T Recommendation X.402 (1999), Section 18.3.1, First NOTE.
    pub fn is_any_admd (&self) -> bool {
        self.built_in_standard_attributes.administration_domain_name
            .as_ref()
            .is_some_and(|admd| admd.as_ref() == " ")
    }

    /// See ITU-T Recommendation X.402 (1999), Section 18.3.1, First NOTE.
    pub fn is_unroutable_admd (&self) -> bool {
        self.built_in_standard_attributes.administration_domain_name
            .as_ref()
            .is_some_and(|admd| admd.as_ref() == "0")
    }

    pub fn to_rfc1685_string(&self) -> Result<String, ASN1Error> {
        let info: ORAddressInfo = self.try_into()?;
        Ok(info.to_string())
    }

    pub fn is_mnem_form(&self) -> bool {
        self.built_in_standard_attributes
            .administration_domain_name
            .is_some()
            && self.built_in_standard_attributes.country_name.is_some()
            && (self.built_in_standard_attributes.personal_name.is_some()
                || self
                    .built_in_standard_attributes
                    .organization_name
                    .is_some()
                || self.extension_attributes.as_ref().is_some_and(|exts| {
                    exts.iter().any(|ext| {
                        if ext.extension_attribute_type.len() != 1 {
                            return false;
                        }
                        let ext_id = ext.extension_attribute_type[0] as i64;
                        [
                            id_common_name,
                            id_teletex_common_name,
                            id_universal_common_name,
                            id_teletex_organization_name,
                            id_universal_organization_name,
                            id_teletex_personal_name,
                            id_universal_personal_name,
                        ]
                        .contains(&ext_id)
                    })
                }))
    }

    pub fn is_numr_form(&self) -> bool {
        self.built_in_standard_attributes
            .administration_domain_name
            .is_some()
            && self.built_in_standard_attributes.country_name.is_some()
            && self
                .built_in_standard_attributes
                .numeric_user_identifier
                .is_some()
    }

    pub fn is_post_f_form(&self) -> bool {
        self.built_in_standard_attributes
            .administration_domain_name
            .is_some()
            && self.built_in_standard_attributes.country_name.is_some()
            && self.extension_attributes.as_ref().is_some_and(|exts| {
                exts.iter().any(|ext| {
                    if ext.extension_attribute_type.len() != 1 {
                        return false;
                    }
                    let ext_id = ext.extension_attribute_type[0] as i64;
                    ext_id == id_physical_delivery_country_name
                })
            })
            && self.extension_attributes.as_ref().is_some_and(|exts| {
                exts.iter().any(|ext| {
                    if ext.extension_attribute_type.len() != 1 {
                        return false;
                    }
                    let ext_id = ext.extension_attribute_type[0] as i64;
                    ext_id == id_postal_code
                })
            })
    }

    pub fn is_post_u_form(&self) -> bool {
        self.is_post_f_form()
            && self.extension_attributes.as_ref().is_some_and(|exts| {
                exts.iter().any(|ext| {
                    if ext.extension_attribute_type.len() != 1 {
                        return false;
                    }
                    let ext_id = ext.extension_attribute_type[0] as i64;
                    [
                        id_unformatted_postal_address,
                        id_universal_unformatted_postal_address,
                    ]
                    .contains(&ext_id)
                })
            })
    }

    pub fn is_term_form(&self) -> bool {
        self.built_in_standard_attributes.network_address.is_some()
    }
}

impl PartialEq for ORAddress {
    fn eq(&self, other: &Self) -> bool {
        let self_info: ORAddressInfo = match self.try_into() {
            Ok(x) => x,
            Err(_) => return false,
        };
        let other_info: ORAddressInfo = match other.try_into() {
            Ok(x) => x,
            Err(_) => return false,
        };
        self_info == other_info
    }
}

impl Display for ORAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_rfc1685_string() {
            Ok(s) => f.write_str(&s),
            Err(e) => {
                f.write_str("MALFORMED-OR-ADDR=")?;
                f.write_str(&e.to_string())?;
                f.write_char(';')?;
                if let Some(pn) = &self.built_in_standard_attributes.personal_name {
                    if let Some(gn) = &pn.given_name {
                        f.write_str("G=")?;
                        f.write_str(remove_oraddress_chars(gn).as_ref())?;
                        f.write_char(';')?;
                    }
                    if let Some(i) = &pn.initials {
                        f.write_str("I=")?;
                        f.write_str(remove_oraddress_chars(i).as_ref())?;
                        f.write_char(';')?;
                    }
                    f.write_str("S=")?;
                    f.write_str(remove_oraddress_chars(&pn.surname).as_ref())?;
                    f.write_char(';')?;
                    if let Some(gq) = &pn.generation_qualifier {
                        f.write_str("Q=")?;
                        f.write_str(remove_oraddress_chars(gq).as_ref())?;
                        f.write_char(';')?;
                    }
                }
                if let Some(x121) = &self.built_in_standard_attributes.network_address {
                    f.write_str("X.121=")?;
                    f.write_str(&x121)?;
                    f.write_char(';')?;
                }
                if let Some(num_id) = &self.built_in_standard_attributes.numeric_user_identifier {
                    f.write_str("N-ID=")?;
                    f.write_str(&num_id)?;
                    f.write_char(';')?;
                }
                if let Some(term_id) = &self.built_in_standard_attributes.terminal_identifier {
                    f.write_str("T-ID=")?;
                    f.write_str(&term_id)?;
                    f.write_char(';')?;
                }
                if let Some(org) = &self.built_in_standard_attributes.organization_name {
                    f.write_str("O=")?;
                    f.write_str(remove_oraddress_chars(org).as_ref())?;
                    f.write_char(';')?;
                }
                if let Some(ous) = &self.built_in_standard_attributes.organizational_unit_names {
                    for (i, ou) in ous.iter().take(4).enumerate() {
                        f.write_str("OU")?;
                        f.write_char(((i as u8 + 1) + 0x30u8).into())?;
                        f.write_char('=')?;
                        f.write_str(remove_oraddress_chars(ou).as_ref())?;
                        f.write_char(';')?;
                    }
                }
                if let Some(prmd) = &self.built_in_standard_attributes.private_domain_name {
                    f.write_str("P=")?;
                    f.write_str(remove_oraddress_chars(&prmd.to_string()).as_ref())?;
                    f.write_char(';')?;
                }
                if let Some(admd) = &self.built_in_standard_attributes.administration_domain_name {
                    f.write_str("A=")?;
                    f.write_str(remove_oraddress_chars(&admd.to_string()).as_ref())?;
                    f.write_char(';')?;
                }
                if let Some(country) = &self.built_in_standard_attributes.country_name {
                    f.write_str("C=")?;
                    f.write_str(&country.to_string())?;
                    f.write_char(';')?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum ORAddressParseErr {
    Malformed(String),
    Duplicate(String),
    TooLong(String),
    Unrecognized(String),
    Asn1Err(ASN1Error),
    TooManyDomainDefinedAttributes,
}

impl Display for ORAddressParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ORAddressParseErr::Malformed(s) => f.write_fmt(format_args!("malformed={}", s)),
            ORAddressParseErr::TooLong(s) => f.write_fmt(format_args!("too_long={}", s)),
            ORAddressParseErr::Duplicate(s) => f.write_fmt(format_args!("duplicate={}", s)),
            ORAddressParseErr::Unrecognized(s) => f.write_fmt(format_args!("unrecognized={}", s)),
            ORAddressParseErr::Asn1Err(a) => f.write_fmt(format_args!("asn1_error={}", a)),
            ORAddressParseErr::TooManyDomainDefinedAttributes => {
                f.write_str("too_many_domain_defined_attrs")
            }
        }
    }
}

impl Error for ORAddressParseErr {}

impl From<ASN1Error> for ORAddressParseErr {
    fn from(value: ASN1Error) -> Self {
        ORAddressParseErr::Asn1Err(value)
    }
}

impl FromStr for ORAddressInfo {
    type Err = ORAddressParseErr; // The key that caused a problem.

    /// Technically, this is supposed to handle a "=" being in the DDA key name,
    /// but this requirement is intentionally ignored because (1) it would just
    /// be stupid to do this, and (2) handling it makes the underlying code a
    /// lot uglier.
    ///
    /// Note that the keys "ADMD" and "PRMD" are not standard, per se, but they
    /// are customary, so they are supported as equals to "A" and "P" respectively.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // All of these come from the built-in attributes.
        let mut prmd: Option<String> = None; // P
        let mut admd: Option<String> = None; // A

        // Personal Name Fields
        let mut g: Option<String> = None;
        let mut sn: Option<String> = None;
        let mut i: Option<String> = None;
        let mut q: Option<String> = None;

        // Organizational Units
        let mut ou1: Option<String> = None;
        let mut ou2: Option<String> = None;
        let mut ou3: Option<String> = None;
        let mut ou4: Option<String> = None;

        // PD address
        let mut pd_a1: Option<String> = None;
        let mut pd_a2: Option<String> = None;
        let mut pd_a3: Option<String> = None;
        let mut pd_a4: Option<String> = None;
        let mut pd_a5: Option<String> = None;
        let mut pd_a6: Option<String> = None;

        // This should only be the ISO-3166 code.
        let mut country: Option<CountryCode> = None; // C
        let mut x121_net_addr: Option<String> = None; // X.121
        let mut num_id: Option<NumericString> = None; // N-ID
        let mut term_id: Option<String> = None; // T-ID
        let mut personal_name: Option<PersonalNameInfo> = None; // G, I, S, Q
        let mut common_name: Option<String> = None; // CN
        let mut org_name: Option<String> = None; // O
        let mut ous: Vec<String> = vec![]; // OU1, OU2, OU3, OU4

        let mut pd_person: Option<String> = None; // PD-PN
        let mut pd_ea: Option<String> = None; // PD-EA
        let mut pd_ed: Option<String> = None; // PD-ED
        let mut pdo_number: Option<String> = None; // PD-OFN
        let mut pdo_name: Option<String> = None; // PD-OF
        let mut pd_org_name: Option<String> = None; // PD-O
        let mut street: Option<String> = None; // PD-S
        let mut pd_address: Vec<String> = vec![]; // PD-A1 - PD-A6
        let mut pd_unique: Option<String> = None; // PD-U
        let mut pd_local: Option<String> = None; // PD-L
        let mut postal_restante_arr: Option<String> = None; // PD-R
        let mut po_box_addr: Option<String> = None; // PD-B
        let mut postal_code: Option<String> = None; // PD-PC
        let mut pd_svc_name: Option<String> = None; // PD-SN
        let mut pd_country_name: Option<CountryCode> = None; // PD-C
        let mut isdn: Option<String> = None; // ISDN (This is the E.163 or E.164 address)
        let mut psap: Option<PresentationAddress> = None; // PSAP
        let mut term_type: Option<TerminalType> = None; // T-TY
        let mut dda: HashMap<String, String> = HashMap::new(); // DDA: Domain-Defined Attribute

        let mut encountered: HashSet<String> = HashSet::new();
        let splitterator = if s.starts_with("/") {
            s[1..].split("/")
        } else {
            s.split(";")
        };
        for kv in splitterator {
            if kv.trim().len() == 0 {
                // There may be a delimiter at the end of the address.
                continue;
            }
            let maybe_key = kv.split("=").next();
            if maybe_key.is_none() {
                return Err(ORAddressParseErr::Malformed("".into()));
            }
            let untrimmed_key = maybe_key.unwrap();
            let untrimmed_key_len = untrimmed_key.len();
            let key = untrimmed_key.trim().to_uppercase();
            if encountered.contains(key.as_str()) {
                return Err(ORAddressParseErr::Duplicate(key.into()));
            }
            if ["ADMD", "A"].contains(&key.as_str()) {
                encountered.insert("ADMD".into());
                encountered.insert("A".into());
            } else if ["PRMD", "P"].contains(&key.as_str()) {
                encountered.insert("PRMD".into());
                encountered.insert("P".into());
            } else {
                encountered.insert(key.clone());
            }
            let value = &kv[untrimmed_key_len + 1..];
            if key.starts_with("DDA:") {
                if key.len() > ub_domain_defined_attribute_type_length + 4 {
                    return Err(ORAddressParseErr::TooLong(key));
                }
                if value.len() > ub_domain_defined_attribute_value_length {
                    return Err(ORAddressParseErr::TooLong(key));
                }
                if dda.len() > ub_domain_defined_attributes {
                    return Err(ORAddressParseErr::TooManyDomainDefinedAttributes);
                }
                dda.insert(key[4..].into(), value.into());
                continue;
            }
            match key.as_str() {
                "A" | "ADMD" => {
                    admd = {
                        if !is_printable_str(value) {
                            return Err(ORAddressParseErr::Malformed(key));
                        }
                        if value.len() > ub_domain_name_length {
                            return Err(ORAddressParseErr::TooLong(key));
                        }
                        Some(value.into())
                    }
                }
                "P" | "PRMD" => {
                    if !is_printable_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    if value.len() > ub_domain_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    prmd = Some(value.into())
                }
                "X.121" => {
                    if !is_numeric_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    if value.len() > ub_x121_address_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    x121_net_addr = Some(value.into())
                }
                "N-ID" => {
                    if !is_numeric_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    if value.len() > ub_numeric_user_id_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    num_id = Some(value.into())
                }
                "T-ID" => {
                    if !is_numeric_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    if value.len() > ub_terminal_id_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    term_id = Some(value.into())
                }
                "G" => {
                    let chars = value.chars().count();
                    if chars > ub_universal_given_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    g = Some(value.into())
                }
                "I" => {
                    let chars = value.chars().count();
                    if chars > ub_universal_initials_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    i = Some(value.into())
                }
                "S" => {
                    let chars = value.chars().count();
                    if chars > ub_universal_surname_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    sn = Some(value.into())
                }
                "Q" => {
                    let chars = value.chars().count();
                    if chars > ub_universal_generation_qualifier_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    q = Some(value.into())
                }
                "CN" => {
                    if value.len() > ub_common_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    common_name = Some(value.into())
                }
                "O" => {
                    let chars = value.chars().count();
                    if chars > ub_organization_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    org_name = Some(value.into())
                }
                "OU1" => {
                    let chars = value.chars().count();
                    if chars > ub_organizational_unit_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    ou1 = Some(value.into())
                }
                "OU2" => {
                    let chars = value.chars().count();
                    if chars > ub_organizational_unit_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    ou2 = Some(value.into())
                }
                "OU3" => {
                    let chars = value.chars().count();
                    if chars > ub_organizational_unit_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    ou3 = Some(value.into())
                }
                "OU4" => {
                    let chars = value.chars().count();
                    if chars > ub_organizational_unit_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    ou4 = Some(value.into())
                }
                "PD-PN" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_person = Some(value.into())
                }
                "PD-EA" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_ea = Some(value.into())
                }
                "PD-ED" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_ed = Some(value.into())
                }
                "PD-OFN" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pdo_number = Some(value.into())
                }
                "PD-OF" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pdo_name = Some(value.into())
                }
                "PD-O" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_org_name = Some(value.into())
                }
                "PD-S" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    street = Some(value.into())
                }
                "PD-A1" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_a1 = Some(value.into())
                }
                "PD-A2" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_a2 = Some(value.into())
                }
                "PD-A3" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_a3 = Some(value.into())
                }
                "PD-A4" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_a4 = Some(value.into())
                }
                "PD-A5" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_a5 = Some(value.into())
                }
                "PD-A6" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_a6 = Some(value.into())
                }
                "PD-U" => {
                    if value.len() > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_unique = Some(value.into())
                }
                "PD-L" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_local = Some(value.into())
                }
                "PD-R" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    postal_restante_arr = Some(value.into())
                }
                "PD-B" => {
                    let chars = value.chars().count();
                    if chars > ub_pds_parameter_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    po_box_addr = Some(value.into())
                }
                "PD-PC" => {
                    if !is_printable_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    let chars = value.chars().count();
                    if chars > ub_postal_code_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    postal_code = Some(value.into())
                }
                "PD-SN" => {
                    if !is_printable_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    if value.len() > ub_pds_name_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    pd_svc_name = Some(value.into())
                }
                "C" => {
                    let cc: &str = u16::from_str(value)
                        .ok()
                        .and_then(|dcc| x121_dcc_country_code_to_iso_3166(dcc))
                        .unwrap_or(value);
                    match CountryCode::for_alpha2(&cc) {
                        Ok(c) => country = Some(c),
                        Err(_) => return Err(ORAddressParseErr::Malformed(key)),
                    }
                }
                "PD-C" => {
                    let cc: &str = u16::from_str(value)
                        .ok()
                        .and_then(|dcc| x121_dcc_country_code_to_iso_3166(dcc))
                        .unwrap_or(value);
                    match CountryCode::for_alpha2(&cc) {
                        Ok(c) => pd_country_name = Some(c),
                        Err(_) => return Err(ORAddressParseErr::Malformed(key)),
                    }
                }
                "ISDN" => {
                    if !is_numeric_str(value) {
                        return Err(ORAddressParseErr::Malformed(key));
                    }
                    if value.len() > ub_e163_4_number_length {
                        return Err(ORAddressParseErr::TooLong(key));
                    }
                    isdn = Some(value.into())
                }
                "PSAP" => {
                    let paddr = match PresentationAddress::from_str(value) {
                        Ok(p) => p,
                        Err(_) => return Err(ORAddressParseErr::Malformed(key)),
                    };
                    psap = Some(paddr);
                }
                "T-TY" => {
                    let ttype = match term_type_from_str(value) {
                        Some(t) => t,
                        None => return Err(ORAddressParseErr::Malformed(key)),
                    };
                    term_type = Some(ttype);
                }
                _ => return Err(ORAddressParseErr::Unrecognized(key)),
            };
        }
        if let Some(sn) = sn {
            personal_name = Some(PersonalNameInfo::new(sn, g, i, q));
        }
        if ou1.is_some() {
            ous = Vec::with_capacity(4);
            ous.push(ou1.unwrap());
        }
        if ous.len() == 1 && ou2.is_some() {
            ous.push(ou2.unwrap());
        }
        if ous.len() == 2 && ou3.is_some() {
            ous.push(ou3.unwrap());
        }
        if ous.len() == 3 && ou4.is_some() {
            ous.push(ou4.unwrap());
        }
        if pd_a1.is_some() {
            pd_address = Vec::with_capacity(6);
            pd_address.push(pd_a1.unwrap());
        }
        if pd_address.len() == 1 && pd_a2.is_some() {
            pd_address.push(pd_a2.unwrap());
        }
        if pd_address.len() == 2 && pd_a3.is_some() {
            pd_address.push(pd_a3.unwrap());
        }
        if pd_address.len() == 3 && pd_a4.is_some() {
            pd_address.push(pd_a4.unwrap());
        }
        if pd_address.len() == 4 && pd_a5.is_some() {
            pd_address.push(pd_a5.unwrap());
        }
        if pd_address.len() == 5 && pd_a6.is_some() {
            pd_address.push(pd_a6.unwrap());
        }
        Ok(ORAddressInfo {
            admd,
            prmd,
            country,
            x121_net_addr,
            num_id,
            term_id,
            personal_name,
            common_name,
            org_name,
            ous,
            pd_person,
            pd_ea,
            pd_ed,
            pdo_number,
            pdo_name,
            pd_org_name,
            street,
            pd_address,
            pd_unique,
            pd_local,
            postal_restante_arr,
            po_box_addr,
            postal_code,
            pd_svc_name,
            pd_country_name,
            isdn,
            psap,
            term_type,
            dda,
        })
    }
}

impl TryFrom<ORAddressInfo> for ORAddress {
    type Error = ASN1Error;

    fn try_from(mut value: ORAddressInfo) -> Result<Self, Self::Error> {
        let org_name_is_printable = value.org_name.as_ref().is_some_and(|s| is_printable_str(s));
        let ous_are_printable = value.ous.iter().all(|ou| is_printable_str(&ou));
        let person_name_is_printable = value
            .personal_name
            .as_ref()
            .is_some_and(|pn| pn.is_printable());

        // Built-Ins
        let built_ins = BuiltInStandardAttributes::new(
            value
                .country
                .take()
                .map(|c| CountryName::iso_3166_alpha2_code(c.alpha2().into())),
            value
                .admd
                .take()
                .map(|a| AdministrationDomainName::printable(a.into())),
            value.x121_net_addr.take(),
            value.term_id.take(),
            value.prmd.take().map(|p| PrivateDomainName::printable(p)),
            if org_name_is_printable {
                value.org_name.take()
            } else {
                None
            },
            value.num_id.take(),
            if person_name_is_printable {
                value.personal_name.take().map(|pn| pn.into())
            } else {
                None
            },
            if ous_are_printable && value.ous.len() > 0 {
                Some(value.ous.clone())
            } else {
                None
            },
        );
        let dda: BuiltInDomainDefinedAttributes = value
            .dda
            .into_iter()
            .map(|(k, v)| BuiltInDomainDefinedAttribute::new(k, v))
            .collect();
        let mut ext_attrs: ExtensionAttributes = Vec::with_capacity(20);
        if let Some(mut personal_name) = value.personal_name.take() {
            if !person_name_is_printable {
                let param = UniversalPersonalName::new(
                    personal_name.surname.clone().into(),
                    personal_name.given_name.take().map(|gn| gn.into()),
                    personal_name.initials.take().map(|i| i.into()),
                    personal_name
                        .generation_qualifier
                        .take()
                        .map(|gq| gq.into()),
                );
                let ext = ExtensionAttribute::new(
                    Vec::from([id_universal_personal_name as u8]),
                    _encode_UniversalPersonalName(&param)?,
                );
                ext_attrs.push(ext);
            }
        }
        if let Some(common_name) = value.common_name {
            let ext = ExtensionAttribute::new(
                Vec::from([id_common_name as u8]),
                BER.encode_printable_string(&common_name)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(org_name) = value.org_name {
            if !org_name_is_printable {
                let param: UniversalOrBMPString = org_name.into();
                let ext = ExtensionAttribute::new(
                    Vec::from([id_universal_organization_name as u8]),
                    _encode_UniversalOrBMPString(&param)?,
                );
                ext_attrs.push(ext);
            }
        }
        if value.ous.len() > 0 {
            if !ous_are_printable {
                let ext = ExtensionAttribute::new(
                    Vec::from([id_universal_organization_name as u8]),
                    _encode_UniversalOrganizationalUnitNames(
                        &value.ous.into_iter().map(|ou| ou.into()).collect(),
                    )?,
                );
                ext_attrs.push(ext);
            }
        }
        if let Some(pd_person) = value.pd_person {
            let param: UniversalOrBMPString = pd_person.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_physical_delivery_personal_name as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pd_ea) = value.pd_ea {
            let param: UniversalOrBMPString = pd_ea.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_extension_physical_delivery_address_components as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pd_ed) = value.pd_ed {
            let param: UniversalOrBMPString = pd_ed.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_extension_OR_address_components as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pdo_number) = value.pdo_number {
            let param: UniversalOrBMPString = pdo_number.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_physical_delivery_office_number as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pdo_name) = value.pdo_name {
            let param: UniversalOrBMPString = pdo_name.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_physical_delivery_office_name as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pd_org_name) = value.pd_org_name {
            let param: UniversalOrBMPString = pd_org_name.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_physical_delivery_organization_name as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(street) = value.street {
            let param: UniversalOrBMPString = street.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_street_address as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if value.pd_address.len() > 0 {
            let param: UniversalOrBMPString = value.pd_address.join("\r\n").into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_unformatted_postal_address as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pd_unique) = value.pd_unique {
            let param: UniversalOrBMPString = pd_unique.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_unique_postal_name as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pd_local) = value.pd_local {
            let param: UniversalOrBMPString = pd_local.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_local_postal_attributes as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(postal_restante_arr) = value.postal_restante_arr {
            let param: UniversalOrBMPString = postal_restante_arr.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_universal_poste_restante_address as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(po_box_addr) = value.po_box_addr {
            let param: UniversalOrBMPString = po_box_addr.into();
            let ext = ExtensionAttribute::new(
                Vec::from([id_post_office_box_address as u8]),
                _encode_UniversalOrBMPString(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(postal_code) = value.postal_code {
            let param = if is_numeric_str(&postal_code) {
                BER.encode_numeric_string(&postal_code)?
            } else {
                BER.encode_owned_printable_string(postal_code)?
            };
            let ext = ExtensionAttribute::new(Vec::from([id_postal_code as u8]), param);
            ext_attrs.push(ext);
        }
        if let Some(pd_svc_name) = value.pd_svc_name {
            let ext = ExtensionAttribute::new(
                Vec::from([id_pds_name as u8]),
                BER.encode_owned_printable_string(pd_svc_name)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(pd_country_name) = value.pd_country_name {
            let c_str = pd_country_name.alpha2().to_owned();
            let c = PhysicalDeliveryCountryName::iso_3166_alpha2_code(c_str);
            let ext = ExtensionAttribute::new(
                Vec::from([id_physical_delivery_country_name as u8]),
                _encode_PhysicalDeliveryCountryName(&c)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(isdn) = value.isdn {
            let mut part_splitter = isdn.split("/");
            let maybe_number = part_splitter.next();
            let maybe_sub_addr = part_splitter.next();
            if let Some(number) = maybe_number {
                let addr = ExtendedNetworkAddress_e163_4_address::new(
                    number.to_owned(),
                    maybe_sub_addr.map(|s| s.to_owned()),
                );
                let param = ExtendedNetworkAddress::e163_4_address(addr);
                let ext = ExtensionAttribute::new(
                    Vec::from([id_extended_network_address as u8]),
                    _encode_ExtendedNetworkAddress(&param)?,
                );
                ext_attrs.push(ext);
            }
        }
        if let Some(psap) = value.psap {
            let param = ExtendedNetworkAddress::psap_address(psap);
            let ext = ExtensionAttribute::new(
                Vec::from([id_extended_network_address as u8]),
                _encode_ExtendedNetworkAddress(&param)?,
            );
            ext_attrs.push(ext);
        }
        if let Some(term_type) = value.term_type {
            let ext = ExtensionAttribute::new(
                Vec::from([id_terminal_type as u8]),
                BER.encode_u16(term_type)?,
            );
            ext_attrs.push(ext);
        }
        Ok(ORAddress {
            built_in_domain_defined_attributes: if dda.len() > 0 { Some(dda) } else { None },
            built_in_standard_attributes: built_ins,
            extension_attributes: if ext_attrs.len() > 0 {
                Some(ext_attrs)
            } else {
                None
            },
        })
    }
}

impl FromStr for ORAddress {
    type Err = ORAddressParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let info = ORAddressInfo::from_str(s)?;
        Ok(info.try_into()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::PkiPmiExternalDataTypes::{AdministrationDomainName, PrivateDomainName, CountryName};
    use super::{ORAddressInfo, ORAddress, id_extended_network_address};

    #[test]
    fn parses_oraddress_string() {
        let input = "A=locomoco;C=US;P=foobar;O=Wildboar Corporation;G=Jonathan;I=M.;S=Wilbur;OU1=Cybersecurity;ISDN=333444 5555";
        let info: ORAddressInfo = input.parse().expect("Could not parse ORAddress");
        assert!(info.personal_name.is_some());
        let mut addr: ORAddress = info.try_into().expect("Could not convert ORAddressInfo into ORAddress");
        // let mut std_attrs = &addr.built_in_standard_attributes;
        let admd = addr.built_in_standard_attributes.administration_domain_name.take().unwrap();
        let prmd = addr.built_in_standard_attributes.private_domain_name.take().unwrap();
        let c = addr.built_in_standard_attributes.country_name.take().unwrap();
        let pn = addr.built_in_standard_attributes.personal_name.take().unwrap();
        let o = addr.built_in_standard_attributes.organization_name.take().unwrap();
        let ous = addr.built_in_standard_attributes.organizational_unit_names.take().unwrap();
        let exts = addr.extension_attributes.take().unwrap();
        assert!(addr.built_in_standard_attributes.network_address.is_none());
        assert_eq!(exts.len(), 1); // For ISDN
        assert_eq!(admd, AdministrationDomainName::printable("locomoco".into()));
        assert_eq!(prmd, PrivateDomainName::printable("foobar".into()));
        assert_eq!(c, CountryName::iso_3166_alpha2_code("US".into()));
        assert_eq!(o, String::from("Wildboar Corporation"));
        assert_eq!(pn.surname, String::from("Wilbur"));
        assert_eq!(pn.given_name, Some("Jonathan".into()));
        assert_eq!(pn.initials, Some("M.".into()));
        assert!(pn.generation_qualifier.is_none());
        assert_eq!(ous.len(), 1);
        assert_eq!(ous[0], String::from("Cybersecurity"));
        assert_eq!(exts[0].extension_attribute_type, Vec::from([ id_extended_network_address as u8 ]));
    }

    #[test]
    fn equates_oraddresses() {
        let input1 = "A=locomoco;C=US;P=foobar;O=Wildboar Corporation;G=Jonathan;I=M.;S=Wilbur;OU1=Cybersecurity;ISDN=333444 5555";
        let info1: ORAddressInfo = input1.parse().expect("Could not parse ORAddress 1");

        let input2 = "A=LOCOMOCO;C=313;P=foobar; O=wildboar corporation;  G=Jonathan; I=M.;S=wilbur;OU1=Cybersecurity;ISDN=3334445555";
        let info2: ORAddressInfo = input2.parse().expect("Could not parse ORAddress 2");
        assert_eq!(info1, info2);

        let addr1: ORAddress = info1.try_into().expect("Could not convert ORAddressInfo into ORAddress 1");
        let addr2: ORAddress = info2.try_into().expect("Could not convert ORAddressInfo into ORAddress 2");
        assert_eq!(addr1, addr2);
    }

}
