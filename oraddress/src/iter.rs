#![allow(non_upper_case_globals)]
use std::iter::{Fuse, FusedIterator, Iterator};
use std::borrow::Cow;
use std::path::Iter;
use wildboar_asn1::{is_printable_char, ASN1ErrorCode, ASN1Result};
use x690::{X690Codec, X690Value, BER};
use teletex::teletex_to_utf8;

use crate::asn1::{
    ExtensionAttribute, ORAddress, TerminalType, TerminalType_g3_facsimile, TerminalType_g4_facsimile, TerminalType_ia5_terminal, TerminalType_teletex, TerminalType_telex, TerminalType_videotex, UniversalOrBMPString_character_encoding, _decode_PDSParameter, _decode_UniversalOrBMPString,
    _decode_PostalCode,
    _decode_UnformattedPostalAddress,
    _decode_UniversalUnformattedPostalAddress,
};

const id_common_name: u8 = crate::asn1::common_name().id as u8;
const id_teletex_common_name: u8 = crate::asn1::teletex_common_name().id as u8;
const id_universal_common_name: u8 =
    crate::asn1::universal_common_name().id as u8;
const id_teletex_organization_name: u8 =
    crate::asn1::teletex_organization_name().id as u8;
const id_universal_organization_name: u8 =
    crate::asn1::universal_organization_name().id as u8;
const id_teletex_personal_name: u8 =
    crate::asn1::teletex_personal_name().id as u8;
const id_universal_personal_name: u8 =
    crate::asn1::universal_personal_name().id as u8;
const id_teletex_organizational_unit_names: u8 =
    crate::asn1::teletex_organizational_unit_names().id as u8;
const id_universal_organizational_unit_names: u8 =
    crate::asn1::universal_organizational_unit_names().id as u8;
const id_teletex_domain_defined_attributes: u8 =
    crate::asn1::teletex_domain_defined_attributes().id as u8;
const id_universal_domain_defined_attributes: u8 =
    crate::asn1::universal_domain_defined_attributes().id as u8;
const id_pds_name: u8 = crate::asn1::pds_name().id as u8;
const id_physical_delivery_country_name: u8 =
    crate::asn1::physical_delivery_country_name().id as u8;
const id_postal_code: u8 = crate::asn1::postal_code().id as u8;
const id_physical_delivery_office_name: u8 =
    crate::asn1::physical_delivery_office_name().id as u8;
const id_universal_physical_delivery_office_name: u8 =
    crate::asn1::universal_physical_delivery_office_name().id as u8;
const id_physical_delivery_office_number: u8 =
    crate::asn1::physical_delivery_office_number().id as u8;
const id_universal_physical_delivery_office_number: u8 =
    crate::asn1::universal_physical_delivery_office_number().id as u8;
const id_extension_OR_address_components: u8 =
    crate::asn1::extension_OR_address_components().id as u8;
const id_universal_extension_OR_address_components: u8 =
    crate::asn1::universal_extension_OR_address_components().id as u8;
const id_physical_delivery_personal_name: u8 =
    crate::asn1::physical_delivery_personal_name().id as u8;
const id_universal_physical_delivery_personal_name: u8 =
    crate::asn1::universal_physical_delivery_personal_name().id as u8;
const id_physical_delivery_organization_name: u8 =
    crate::asn1::physical_delivery_organization_name().id as u8;
const id_universal_physical_delivery_organization_name: u8 =
    crate::asn1::universal_physical_delivery_organization_name().id as u8;
const id_extension_physical_delivery_address_components: u8 =
    crate::asn1::extension_physical_delivery_address_components().id as u8;
const id_universal_extension_physical_delivery_address_components: u8 =
    crate::asn1::universal_extension_physical_delivery_address_components().id as u8;
const id_unformatted_postal_address: u8 =
    crate::asn1::unformatted_postal_address().id as u8;
const id_universal_unformatted_postal_address: u8 =
    crate::asn1::universal_unformatted_postal_address().id as u8;
const id_street_address: u8 = crate::asn1::street_address().id as u8;
const id_universal_street_address: u8 =
    crate::asn1::universal_street_address().id as u8;
const id_post_office_box_address: u8 =
    crate::asn1::post_office_box_address().id as u8;
const id_universal_post_office_box_address: u8 =
    crate::asn1::universal_post_office_box_address().id as u8;
const id_poste_restante_address: u8 =
    crate::asn1::poste_restante_address().id as u8;
const id_universal_poste_restante_address: u8 =
    crate::asn1::universal_poste_restante_address().id as u8;
const id_unique_postal_name: u8 = crate::asn1::unique_postal_name().id as u8;
const id_universal_unique_postal_name: u8 =
    crate::asn1::universal_unique_postal_name().id as u8;
const id_local_postal_attributes: u8 =
    crate::asn1::local_postal_attributes().id as u8;
const id_universal_local_postal_attributes: u8 =
    crate::asn1::universal_local_postal_attributes().id as u8;
const id_extended_network_address: u8 =
    crate::asn1::extended_network_address().id as u8;
const id_terminal_type: u8 = crate::asn1::terminal_type().id as u8;

#[inline]
pub(crate) const fn term_type_to_str(tt: u16) -> &'static str {
    match tt {
        crate::asn1::TerminalType_g3_facsimile => "g3fax",
        crate::asn1::TerminalType_g4_facsimile => "g4fax",
        crate::asn1::TerminalType_ia5_terminal => "ia5",
        crate::asn1::TerminalType_teletex => "ttx",
        crate::asn1::TerminalType_telex => "tlx",
        crate::asn1::TerminalType_videotex => "vtx",
        _ => "?",
    }
}

#[inline]
pub(crate) fn term_type_from_str(tt: &str) -> Option<TerminalType> {
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

// #[derive(Debug)]
// pub struct ORAddressInfo {
//     // All of these come from the built-in attributes.
//     pub prmd: Option<String>, // P
//     pub admd: Option<String>, // A

//     /// This should only be the ISO-3166 code.
//     pub country: Option<CountryCode>, // C
//     pub x121_net_addr: Option<String>, // X.121
//     pub num_id: Option<NumericString>, // N-ID
//     pub term_id: Option<String>,       // T-ID

//     pub personal_name: Option<PersonalNameInfo>, // G, I, S, Q
//     pub common_name: Option<String>,             // CN
//     pub org_name: Option<String>,                // O
//     pub ous: Vec<String>,                        // OU1, OU2, OU3, OU4
//     pub pd_person: Option<String>,               // PD-PN
//     pub pd_ea: Option<String>,                   // PD-EA
//     pub pd_ed: Option<String>,                   // PD-ED
//     pub pdo_number: Option<String>,              // PD-OFN
//     pub pdo_name: Option<String>,                // PD-OF
//     pub pd_org_name: Option<String>,             // PD-O
//     pub street: Option<String>,                  // PD-S
//     pub pd_address: Vec<String>,                 // PD-A1 - PD-A6
//     pub pd_unique: Option<String>,               // PD-U
//     pub pd_local: Option<String>,                // PD-L
//     pub postal_restante_arr: Option<String>,     // PD-R
//     pub po_box_addr: Option<String>,             // PD-B
//     pub postal_code: Option<String>,             // PD-PC
//     pub pd_svc_name: Option<String>,             // PD-SN
//     pub pd_country_name: Option<CountryCode>,    // PD-C
//     pub isdn: Option<String>,                    // ISDN (This is the E.163 or E.164 address)
//     pub psap: Option<PresentationAddress>,       // PSAP
//     pub term_type: Option<TerminalType>,         // T-TY
//     pub dda: HashMap<String, String>,            // DDA: Domain-Defined Attribute
// }

/// These are only the O/R address fields that could have duplicate fields due
/// to them being present in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ORAddressStringField {
    OrganizationName,
    // OrganizationalUnitNames,
    // PersonalName,
    // DomainDefinedAttribute,
    CommonName,
    PDOfficeName,
    PDOfficeNumber,
    // PDPersonalName,
    PDOrganizationName,
    ExtensionPDAddressComponents,
    ExtensionORAddressComponents,
    StreetAddress,
    POBoxAddress,
    PosteRestanteAddress,
    UniquePostalName,
    LocalPostalAttributes,
    PDSName,
    // UnformattedPostalAddress,
}

// TODO: Should I change this iterator to just return a union of all attribute
// types so that it can be single-pass?
// Maybe you should just keep it and implement single-pass just for Display
pub struct ORAddressStrings<'a> {
    addr: &'a ORAddress,
    i: usize,
    field: ORAddressStringField,
    ext_attrs: Option<&'a [ExtensionAttribute]>,
}

impl <'a> Iterator for ORAddressStrings<'a> {
    type Item = wildboar_asn1::ASN1Result<Cow<'a, str>>;

    fn next(&mut self) -> Option<Self::Item> {
        // This is the only field in this iterator that also has a corresponding
        // field in the `BuiltInStandardAttributes`.
        if self.field == ORAddressStringField::OrganizationName {
            if self.i == 0 {
                self.i += 1;
                if let Some(o) = self.addr.built_in_standard_attributes.organization_name.as_ref() {
                    return Some(Ok(Cow::Borrowed(o.as_str())));
                }
            }
        }

        // TODO: Instead of skipping each time, just shorten the extensions slice.
        let ext_attrs = self.ext_attrs?;
        let ext_attr_idx = ext_attrs
            .iter()
            .position(|ea| match self.field {
                ORAddressStringField::OrganizationName
                    => match ea.extension_attribute_type {
                        id_teletex_organization_name
                        | id_universal_organization_name => true,
                        _ => false,
                    }
                ORAddressStringField::CommonName
                    => match ea.extension_attribute_type {
                        id_teletex_common_name
                        | id_universal_common_name => true,
                        _ => false,
                    }
                ORAddressStringField::PDOfficeName
                    => match ea.extension_attribute_type {
                        id_physical_delivery_office_name
                        | id_universal_physical_delivery_office_name => true,
                        _ => false,
                    }
                ORAddressStringField::PDOfficeNumber
                    => match ea.extension_attribute_type {
                        id_physical_delivery_office_number
                        | id_universal_physical_delivery_office_number => true,
                        _ => false,
                    }
                ORAddressStringField::PDOrganizationName
                    => match ea.extension_attribute_type {
                        id_physical_delivery_organization_name
                        | id_universal_physical_delivery_organization_name => true,
                        _ => false,
                    }
                ORAddressStringField::ExtensionPDAddressComponents
                    => match ea.extension_attribute_type {
                        id_extension_physical_delivery_address_components
                        | id_universal_extension_physical_delivery_address_components => true,
                        _ => false,
                    }
                ORAddressStringField::ExtensionORAddressComponents
                    => match ea.extension_attribute_type {
                        id_extension_OR_address_components
                        | id_universal_extension_OR_address_components => true,
                        _ => false,
                    }
                ORAddressStringField::StreetAddress
                    => match ea.extension_attribute_type {
                        id_street_address
                        | id_universal_street_address => true,
                        _ => false,
                    }
                ORAddressStringField::POBoxAddress
                    => match ea.extension_attribute_type {
                        id_post_office_box_address
                        | id_universal_post_office_box_address => true,
                        _ => false,
                    }
                ORAddressStringField::PosteRestanteAddress
                    => match ea.extension_attribute_type {
                        id_poste_restante_address
                        | id_universal_poste_restante_address => true,
                        _ => false,
                    }
                ORAddressStringField::UniquePostalName
                    => match ea.extension_attribute_type {
                        id_unique_postal_name
                        | id_universal_unique_postal_name => true,
                        _ => false,
                    }
                ORAddressStringField::LocalPostalAttributes
                    => match ea.extension_attribute_type {
                        id_local_postal_attributes
                        | id_universal_local_postal_attributes => true,
                        _ => false,
                    }
                ORAddressStringField::PDSName
                    => ea.extension_attribute_type == id_pds_name,
            })?
            ;
        let ext_attr = &ext_attrs[ext_attr_idx];
        let new_ext_attrs = &ext_attrs[ext_attr_idx+1..];
        if new_ext_attrs.len() == 0 {
            self.ext_attrs = None;
        } else {
            self.ext_attrs = Some(new_ext_attrs);
        }
        match ext_attr.extension_attribute_type {
            id_teletex_organization_name
            | id_teletex_common_name => {
                let s = match BER.decode_t61_string(&ext_attr.extension_attribute_value) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e)),
                };
                if s.is_ascii() {
                    let s = unsafe { String::from_utf8_unchecked(s) };
                    return Some(Ok(Cow::Owned(s)));
                }
                let s: String = teletex_to_utf8(s.as_slice()).collect();
                Some(Ok(Cow::Owned(s)))
            },
            id_common_name
            | id_pds_name => {
                match &ext_attr.extension_attribute_value.value {
                    X690Value::Primitive(prim) => {
                        let maybe_invalid = prim.iter().position(|b| !is_printable_char(*b));
                        if let Some(invalid) = maybe_invalid {
                            let code = ASN1ErrorCode::prohibited_character(
                                prim.as_ref()[invalid] as u32,
                                invalid,
                            );
                            return Some(Err(ext_attr.extension_attribute_value.to_asn1_error(code)));
                        }
                        let s = unsafe { str::from_utf8_unchecked(&prim) };
                        return Some(Ok(Cow::Borrowed(s)));
                    },
                    _ => (),
                };
                let s = match BER.decode_printable_string(&ext_attr.extension_attribute_value) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok(Cow::Owned(s)))
            },
            id_physical_delivery_office_name
            | id_physical_delivery_office_number
            | id_extension_OR_address_components
            | id_physical_delivery_personal_name
            | id_physical_delivery_organization_name
            | id_extension_physical_delivery_address_components
            | id_street_address
            | id_post_office_box_address
            | id_poste_restante_address
            | id_unique_postal_name
            | id_local_postal_attributes
            => {
                let param = match _decode_PDSParameter(&ext_attr.extension_attribute_value) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e)),
                };
                if let Some(ps) = param.printable_string {
                    return Some(Ok(Cow::Owned(ps)));
                }
                if let Some(ts) = param.teletex_string {
                    let s: String = teletex_to_utf8(ts.as_slice()).collect();
                    return Some(Ok(Cow::Owned(s)));
                }
                // This is not technically an error, so we just return ""...
                return Some(Ok(Cow::Borrowed("")));
            },
            id_universal_common_name
            | id_universal_organization_name
            | id_universal_physical_delivery_office_name
            | id_universal_physical_delivery_office_number
            | id_universal_extension_OR_address_components
            | id_universal_physical_delivery_personal_name
            | id_universal_physical_delivery_organization_name
            | id_universal_extension_physical_delivery_address_components
            | id_universal_unformatted_postal_address
            | id_universal_street_address
            | id_universal_post_office_box_address
            | id_universal_poste_restante_address
            | id_universal_unique_postal_name
            | id_universal_local_postal_attributes
            => {
                let param = match _decode_UniversalOrBMPString(&ext_attr.extension_attribute_value) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e)),
                };
                match param.character_encoding {
                    UniversalOrBMPString_character_encoding::four_octets(univ) => {
                        let s = univ.to_string_lossy();
                        Some(Ok(Cow::Owned(s)))
                    },
                    UniversalOrBMPString_character_encoding::two_octets(bmp) => {
                        let s = bmp.to_string_lossy();
                        Some(Ok(Cow::Owned(s)))
                    },
                }
            },
            // TODO:
            // id_postal_code => {
            //     // PostalCode
            //     let pc = match _decode_PostalCode(&ext_attr.extension_attribute_value) {
            //         Ok(pc) => pc,
            //         Err(e) => return Some(Err(e)),
            //     };
            // },
            // Multiple pseudo-attributes for this.
            // id_unformatted_postal_address => {
            //     let upa = match _decode_UnformattedPostalAddress(&ext_attr.extension_attribute_value) {
            //         Ok(upa) => upa,
            //         Err(e) => return Some(Err(e)),
            //     };
            //     if let Some(ps) = upa.printable_address {
            //         return Some(Ok(Cow::Owned(ps)));
            //     }
            //     if let Some(ts) = upa.teletex_string {
            //         let s: String = teletex_to_utf8(ts.as_slice()).collect();
            //         return Some(Ok(Cow::Owned(s)));
            //     }
            //     // This is not technically an error, so we just return ""...
            //     return Some(Ok(Cow::Borrowed("")));
            // },
            _ => None,
        }
    }
}

impl <'a> FusedIterator for ORAddressStrings<'a> {}

// pub struct PersonalNameRef <'a> {
//     surname: Cow<'a, str>,
//     given_name: Option<Cow<'a, str>>,
//     initials: Option<Cow<'a, str>>,
//     gen_qual: Option<Cow<'a, str>>,
// }

// pub struct ORAddressPersonalNames<'a> {
//     addr: &'a ORAddress,
//     i: usize,
//     field: ORAddressStringField,
//     ext_attrs: Option<&'a [ExtensionAttribute]>,
// }

// impl <'a> Iterator for ORAddressPersonalNames<'a> {
//     type Item = ASN1Result<PersonalNameRef<'a>>;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

// impl <'a> FusedIterator for ORAddressPersonalNames<'a> {}

// pub struct ORAddressOrgUnitNames<'a> {
//     addr: &'a ORAddress,
//     i: usize,
//     ext_attrs: Option<&'a [ExtensionAttribute]>,
// }

// impl <'a> Iterator for ORAddressOrgUnitNames<'a> {
//     type Item = ASN1Result<()>;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

// impl <'a> FusedIterator for ORAddressOrgUnitNames<'a> {}

// pub struct ORAddressDomainDefinedAttributes<'a> {
//     addr: &'a ORAddress,
//     read_built_in: bool,
//     ext_attrs: Option<&'a [ExtensionAttribute]>,
// }

// impl <'a> Iterator for ORAddressDomainDefinedAttributes<'a> {
//     type Item = ASN1Result<()>;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

// impl <'a> FusedIterator for ORAddressDomainDefinedAttributes<'a> {}

// TODO: size_hint
// I don't think any other iterators are needed
