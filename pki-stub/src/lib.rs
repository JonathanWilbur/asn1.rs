#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKI-Stub
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKI-Stub`.
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
pub mod PKI_Stub;
pub mod display;
pub mod eq;
pub mod hash;
pub mod oids;
pub mod pki;
pub mod utils;
pub use crate::PKI_Stub::*;
pub use crate::oids::{common_attr_type_to_long_name, common_attr_type_to_short_name};
use std::str::FromStr;
use wildboar_asn1::utils::read_i64;
use wildboar_asn1::{
    ASN1Error, OBJECT_IDENTIFIER, TagClass, UNIV_TAG_BIT_STRING, UNIV_TAG_BMP_STRING,
    UNIV_TAG_BOOLEAN, UNIV_TAG_DATE, UNIV_TAG_DATE_TIME, UNIV_TAG_DURATION, UNIV_TAG_ENUMERATED,
    UNIV_TAG_GENERAL_STRING, UNIV_TAG_GENERALIZED_TIME, UNIV_TAG_GRAPHIC_STRING,
    UNIV_TAG_IA5_STRING, UNIV_TAG_INTEGER, UNIV_TAG_NULL, UNIV_TAG_NUMERIC_STRING,
    UNIV_TAG_OBJECT_DESCRIPTOR, UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_OCTET_STRING,
    UNIV_TAG_OID_IRI, UNIV_TAG_PRINTABLE_STRING, UNIV_TAG_REAL, UNIV_TAG_RELATIVE_OID,
    UNIV_TAG_RELATIVE_OID_IRI, UNIV_TAG_TIME, UNIV_TAG_TIME_OF_DAY, UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_UTC_TIME, UNIV_TAG_UTF8_STRING, UNIV_TAG_VISIBLE_STRING,
};
use x690::{BER, X690Codec, X690Element, x690_write_tlv};

pub trait DisplayX500AttributeType {
    fn attr_type_to_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        self.attr_type_to_long_name(attr_type).or(self
            .attr_type_to_short_name(attr_type)
            .or(self.attr_type_to_descriptor(attr_type)))
    }

    // Same as `attr_type_to_name`, but it prefers shorter names.
    fn attr_type_to_shortest_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        self.attr_type_to_short_name(attr_type).or(self
            .attr_type_to_long_name(attr_type)
            .or(self.attr_type_to_descriptor(attr_type)))
    }

    fn attr_type_to_descriptor(self: &Self, _: &OBJECT_IDENTIFIER) -> Option<&str> {
        None
    }

    fn attr_type_to_long_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        common_attr_type_to_long_name(attr_type)
    }

    fn attr_type_to_short_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        common_attr_type_to_short_name(attr_type)
    }
}

/* I ultimately chose not to implement this using a formatter because this will
likely be used primarily for printing distinguished names, and doing so
requires escaping special characters in attribute values. This cannot happen in
formatters, where the value is written out irreversibly. As such, this returns
strings directly. Unfortunately, this is probably less efficient than using a
formatter. This approach also allows us to return an ASN1Error instead of the
useless std::fmt::Error. */
pub trait DisplayX500Value<OpenType> {
    fn unrecognized_value_to_string(self: &Self, value: &OpenType) -> String;

    // TODO: Should this return an Option<T> to allow the callee to decide whether to use the "unrecognized" syntax?
    fn value_to_string(
        self: &Self,
        attr_type: &OBJECT_IDENTIFIER,
        value: &OpenType,
    ) -> Result<Option<String>, ASN1Error>;
}

pub struct DefaultX500ValueDisplayer;
pub struct DefaultX500ValueParser;

pub fn value_to_string<E, K>(
    _k: &K,
    _attr_type: &OBJECT_IDENTIFIER,
    value: &X690Element,
) -> Result<Option<String>, ASN1Error>
where
    K: DisplayX500AttributeType + DisplayX500Value<E>,
{
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
        }
        UNIV_TAG_INTEGER => {
            let integ = BER.decode_integer(value)?;
            if let Some(i) = read_i64(&integ) {
                return Ok(Some(format!("{}", i).to_string()));
            } else {
                return Ok(None);
            }
        }
        UNIV_TAG_BIT_STRING => {
            let v = BER.decode_bit_string(value)?;
            Ok(Some(format!("{}", v).to_string()))
        }
        UNIV_TAG_OCTET_STRING => {
            let v = BER.decode_octet_string(value)?;
            // NOTE: This is not the LDAP syntax. The LDAP syntax is just the raw octets.
            Ok(Some(hex::encode(v)))
        }
        UNIV_TAG_NULL => Ok(Some(String::from("NULL"))),
        UNIV_TAG_OBJECT_IDENTIFIER => {
            let v = BER.decode_object_identifier(value)?;
            Ok(Some(v.to_string()))
        }
        UNIV_TAG_OBJECT_DESCRIPTOR => {
            let v = BER.decode_object_descriptor(value)?;
            Ok(Some(v))
        }
        // UNIV_TAG_EXTERNAL => {},
        UNIV_TAG_REAL => {
            let v = BER.decode_real(value)?;
            Ok(Some(v.to_string()))
        }
        UNIV_TAG_ENUMERATED => {
            let v = BER.decode_enumerated(value)?;
            Ok(Some(v.to_string()))
        }
        // UNIV_TAG_EMBEDDED_PDV => {},
        UNIV_TAG_UTF8_STRING => {
            let v = BER.decode_utf8_string(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_RELATIVE_OID => {
            let v = BER.decode_relative_oid(value)?;
            Ok(Some(v.to_string()))
        }
        UNIV_TAG_TIME => {
            let v = BER.decode_time(value)?;
            Ok(Some(v))
        }
        // UNIV_TAG_RESERVED_15 => {},
        // UNIV_TAG_SEQUENCE => {},
        // UNIV_TAG_SEQUENCE_OF => {},
        // UNIV_TAG_SET => {},
        // UNIV_TAG_SET_OF => {},
        UNIV_TAG_NUMERIC_STRING => {
            let v = BER.decode_numeric_string(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_PRINTABLE_STRING => {
            let v = BER.decode_printable_string(value)?;
            Ok(Some(v))
        }
        // UNIV_TAG_T61_STRING => {},
        // UNIV_TAG_VIDEOTEX_STRING => {},
        UNIV_TAG_IA5_STRING => {
            let v = BER.decode_ia5_string(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_UTC_TIME => {
            let v = BER.decode_utc_time(value)?;
            Ok(Some(v.to_string()))
        }
        UNIV_TAG_GENERALIZED_TIME => {
            let v = BER.decode_generalized_time(value)?;
            Ok(Some(v.to_string()))
        }
        UNIV_TAG_GRAPHIC_STRING => {
            let v = BER.decode_graphic_string(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_VISIBLE_STRING => {
            let v = BER.decode_visible_string(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_GENERAL_STRING => {
            let v = BER.decode_general_string(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_UNIVERSAL_STRING => {
            let v = BER.decode_universal_string(value)?;
            Ok(Some(v.to_string_lossy()))
        }
        // UNIV_TAG_CHARACTER_STRING => {},
        UNIV_TAG_BMP_STRING => {
            let v = BER.decode_bmp_string(value)?;
            Ok(Some(v.to_string_lossy()))
        }
        UNIV_TAG_DATE => {
            let v = BER.decode_date(value)?;
            Ok(Some(
                format!("{}-{}-{}", v.year, v.month, v.day).to_string(),
            ))
        }
        UNIV_TAG_TIME_OF_DAY => {
            let v = BER.decode_time_of_day(value)?;
            Ok(Some(
                format!("{}:{}:{}", v.hour, v.minute, v.second).to_string(),
            ))
        }
        UNIV_TAG_DATE_TIME => {
            let v = BER.decode_date_time(value)?;
            Ok(Some(
                format!(
                    "{}-{}-{}T{}:{}:{}",
                    v.date.year,
                    v.date.month,
                    v.date.day,
                    v.time.hour,
                    v.time.minute,
                    v.time.second
                )
                .to_string(),
            ))
        }
        UNIV_TAG_DURATION => {
            let v = BER.decode_duration(value)?;
            Ok(Some(v.to_string()))
        }
        UNIV_TAG_OID_IRI => {
            let v = BER.decode_oid_iri(value)?;
            Ok(Some(v))
        }
        UNIV_TAG_RELATIVE_OID_IRI => {
            let v = BER.decode_relative_oid_iri(value)?;
            Ok(Some(v))
        }
        _ => Ok(None),
    }
}

impl DisplayX500Value<X690Element> for DefaultX500ValueDisplayer {
    fn unrecognized_value_to_string(self: &Self, value: &X690Element) -> String {
        let mut encoding: Vec<u8> = Vec::with_capacity(value.len() + 2);
        x690_write_tlv(&mut encoding, &value).unwrap_or_default();
        format!("#{}", hex::encode(&encoding))
    }

    fn value_to_string(
        &self,
        attr_type: &OBJECT_IDENTIFIER,
        value: &X690Element,
    ) -> Result<Option<String>, ASN1Error> {
        value_to_string(self, attr_type, value)
    }
}

impl DisplayX500AttributeType for DefaultX500ValueDisplayer {}

pub trait ParseX500AttributeType {
    fn attr_type_name_to_oid(&self, s: &str) -> Option<OBJECT_IDENTIFIER>;

    // Error is for an invalid attribute type string.
    // Option::None is just for if a name is not recognized.
    // fn parse_attr_type (&self, s: &str) -> Result<Option<OBJECT_IDENTIFIER>, std::fmt::Error>;
    fn parse_attr_type(&self, s: &str) -> Result<Option<OBJECT_IDENTIFIER>, std::fmt::Error> {
        if let Some(first_char) = s.chars().next() {
            if first_char.is_digit(3) {
                // The first digit can only be 0, 1, or 2. Using a radix of 3 handles this.
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

pub trait ParseX500Value<OpenType> {
    fn parse_value(
        &self,
        attr_type: &OBJECT_IDENTIFIER,
        s: &str,
    ) -> Result<Option<OpenType>, std::fmt::Error>;
}

pub trait ParseX500DistinguishedName {
    fn parse_dn(&self, s: &str) -> Result<DistinguishedName, std::fmt::Error>;
}

pub trait ParseX500DirectoryName {
    fn parse_x500_name(&self, s: &str) -> Result<Name, std::fmt::Error>;
}

pub trait ParseGeneralName {
    fn parse_general_name(&self, s: &str) -> Result<GeneralName, std::fmt::Error>;
}

// impl ParseX500AttributeType for DefaultX500ValueParser {  }

// Exported separately so it can be used by other implementations, if desired.
pub fn parse_value<K: ParseX500AttributeType>(
    _k: &K,
    attr_type: &OBJECT_IDENTIFIER,
    s: &str,
) -> Result<Option<X690Element>, std::fmt::Error> {
    if s.starts_with("#") {
        let bytes = hex::decode(&s[1..]).map_err(|_| std::fmt::Error)?;
        let cst = BER.decode_from_slice(&bytes).map_err(|_| std::fmt::Error)?;
        return Ok(Some(cst.1));
    }
    if attr_type.as_x690_slice().len() == 3 && attr_type.as_x690_slice().starts_with(&[0x55, 4]) {
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
    fn parse_value(
        &self,
        attr_type: &OBJECT_IDENTIFIER,
        s: &str,
    ) -> Result<Option<X690Element>, std::fmt::Error> {
        parse_value(self, attr_type, s)
    }
}

impl ParseX500AttributeType for DefaultX500ValueParser {
    fn attr_type_name_to_oid(&self, s: &str) -> Option<OBJECT_IDENTIFIER> {
        // FIXME: Make a better default implementation
        None
    }
}

// TODO: Implement ParseX500DistinguishedName for all types that implement
// ParseX500Value<X690Element> and ParseX500AttributeType

impl<T> ParseX500DistinguishedName for T
where
    T: ParseX500AttributeType + ParseX500Value<X690Element>,
{
    fn parse_dn(&self, s: &str) -> Result<DistinguishedName, std::fmt::Error> {
        // let mut char_start_byte: usize = 0;

        let mut dn: DistinguishedName = Vec::with_capacity(10);
        let mut rdn: RelativeDistinguishedName = Vec::with_capacity(4);
        let mut attr_type: Option<OBJECT_IDENTIFIER> = None;
        let mut start_of_token: usize = 0;
        let mut parsing_value: bool = false;
        let mut escaping: bool = false;
        let mut escaped: bool = false;
        let mut escaped_char: char = char::from_u32(0).unwrap();
        let mut escaped_str: String = String::new();
        for (i, c) in s.char_indices() {
            if parsing_value {
                if escaping {}
                if c == ',' {}
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

impl<T> ParseX500DirectoryName for T
where
    T: ParseX500DistinguishedName,
{
    fn parse_x500_name(&self, s: &str) -> Result<Name, std::fmt::Error> {
        if s.to_lowercase().starts_with("rdnsequence") {
            Ok(Name::rdnSequence(self.parse_dn(&s[11..])?))
        } else {
            Err(std::fmt::Error)
        }
    }
}

// TODO: Implement ParseX500DirectoryName for all types that implement ParseX500DistinguishedName

// TODO: Implement ParseGeneralName for all types that implement ParseX500DirectoryName
