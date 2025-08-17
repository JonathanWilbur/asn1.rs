use phf::phf_map;
use wildboar_asn1::{
    Tag,
    TagClass,
    UTCTime,
    BIT_STRING,
    OBJECT_IDENTIFIER,
    UNIV_TAG_SEQUENCE,
    UNIV_TAG_UTF8_STRING,
};
use std::str::FromStr;
use x690::{X690Element, X690Codec, X690Value};
use x690::ber::BER;
use std::sync::Arc;
use std::iter::Iterator;
use crate::unescape::parse_postal_address;
use crate::EDIPartyName;
use crate::PKI_Stub::{DistinguishedName, Name, GeneralName, RelativeDistinguishedName, AttributeTypeAndValue};
use ldapdn::parse::dn_from_str;
use ldapdn::escape::unescape_ldap_value_string_cow;
use bytes::Bytes;

/// This is limited to attributes from schemas that are explicitly allowed in
/// IETF RFC 3039 for naming, and a few others for Qualified Certificates.
///
/// This is not out of laziness, but rather, to keep this crate as small as
/// possible while still making a default implementation of
/// `ParseX500AttributeType` that can still be used to decode the vast majority
/// of distinguished names. This is a "good enough" implementation.
///
/// Applications that are expected to go beyond X.509 PKI and support LDAP or
/// X.500 directory features should do a more thorough implementation that
/// includes all object identifiers that have descriptors registered with IANA
/// here: https://www.iana.org/assignments/ldap-parameters/ldap-parameters.xhtml
static OIDS_BY_NAME: phf::Map<&'static str, &'static [u8]> = phf_map! {

    // X.500-series user attributes
    "objectclass" => &[0x55, 4, 0],
    "aliasedentryname" => &[0x55, 4, 1],
    "commonname" => &[0x55, 4, 3],
    "surname" => &[0x55, 4, 4],
    "serialnumber" => &[0x55, 4, 5],
    "countryname" => &[0x55, 4, 6],
    "localityname" => &[0x55, 4, 7],
    "stateorprovincename" => &[0x55, 4, 8],
    "streetaddress" => &[0x55, 4, 9],
    "organizationname" => &[0x55, 4, 10],
    "organizationalunitname" => &[0x55, 4, 11],
    "title" => &[0x55, 4, 12],
    "description" => &[0x55, 4, 13],
    "searchguide" => &[0x55, 4, 14],
    "businesscategory" => &[0x55, 4, 15],
    "postaladdress" => &[0x55, 4, 16],
    "postalcode" => &[0x55, 4, 17],
    "postofficebox" => &[0x55, 4, 18],
    "physicaldeliveryofficename" => &[0x55, 4, 19],
    "telephonenumber" => &[0x55, 4, 20],
    "telexnumber" => &[0x55, 4, 21],
    "facsimiletelephonenumber" => &[0x55, 4, 23],
    "x121address" => &[0x55, 4, 24],
    "internationalisdnnumber" => &[0x55, 4, 25],
    "registeredaddress" => &[0x55, 4, 26],
    "destinationindicator" => &[0x55, 4, 27],
    "preferreddeliverymethod" => &[0x55, 4, 28],
    "presentationaddress" => &[0x55, 4, 29],
    "supportedapplicationcontext" => &[0x55, 4, 30],
    "member" => &[0x55, 4, 31],
    "owner" => &[0x55, 4, 32],
    "roleoccupant" => &[0x55, 4, 33],
    "seealso" => &[0x55, 4, 34],
    "name" => &[0x55, 4, 41],
    "givenname" => &[0x55, 4, 42],
    "initials" => &[0x55, 4, 43],
    "generationqualifier" => &[0x55, 4, 44],
    "x500uniqueidentifier" => &[0x55, 4, 45],
    "dnqualifier" => &[0x55, 4, 46],
    "enhancedsearchguide" => &[0x55, 4, 47],
    "protocolinformation" => &[0x55, 4, 48],
    "distinguishedname" => &[0x55, 4, 49],
    "uniquemember" => &[0x55, 4, 50],
    "houseidentifier" => &[0x55, 4, 51],
    "dmdname" => &[0x55, 4, 54],
    "pseudonym" => &[0x55, 4, 65],
    "communicationsservice" => &[0x55, 4, 66],
    "communicationsnetwork" => &[0x55, 4, 67],
    "uuidpair" => &[0x55, 4, 77],
    "tagoid" => &[0x55, 4, 78],
    "uiiformat" => &[0x55, 4, 79],
    "uiiinurn" => &[0x55, 4, 80],
    "contenturl" => &[0x55, 4, 81],
    "uri" => &[0x55, 4, 83],
    "pwdattribute" => &[0x55, 4, 84],
    "urn" => &[0x55, 4, 86],
    "url" => &[0x55, 4, 87],
    "utmcoordinates" => &[0x55, 4, 88],
    "urnc" => &[0x55, 4, 89],
    "uii" => &[0x55, 4, 90],
    "epc" => &[0x55, 4, 91],
    "tagafi" => &[0x55, 4, 92],
    "epcformat" => &[0x55, 4, 93],
    "epcinurn" => &[0x55, 4, 94],
    "ldapurl" => &[0x55, 4, 95],
    "taglocation" => &[0x55, 4, 96],
    "organizationidentifier" => &[0x55, 4, 97],
    "countrycode3c" => &[0x55, 4, 98],
    "countrycode3n" => &[0x55, 4, 99],
    "dnsname" => &[0x55, 4, 100],
    "intemail" => &[0x55, 4, 104],
    "jid" => &[0x55, 4, 105],
    "objectidentifier" => &[0x55, 4, 106],

    // X.500-series short names
    "cn" => &[0x55, 4, 3],
    "sn" => &[0x55, 4, 4],
    "c" => &[0x55, 4, 6],
    "l" => &[0x55, 4, 7],
    "st" => &[0x55, 4, 8],
    "street" => &[0x55, 4, 9],
    "o" => &[0x55, 4, 10],
    "ou" => &[0x55, 4, 11],
    "gn" => &[0x55, 4, 42],
    "c3" => &[0x55, 4, 98],
    "n3" => &[0x55, 4, 99],

    // X.500-series OID arc attributes
    "oidc1" => &[97, 1, 2, 0],
    "oidc2" => &[97, 1, 2, 1],
    "oidc" => &[97, 1, 2, 2],

    // COSINE attribute names
    "userid" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 1],
    "textencodedoraddress" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 2],
    "rfc822mailbox" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 3],
    "info" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 4],
    "favouritedrink" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 5],
    "roomnumber" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 6],
    "photo" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 7],
    "userclass" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 8],
    "host" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 9],
    "manager" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 10],
    "documentidentifier" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 11],
    "documenttitle" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 12],
    "documentversion" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 13],
    "documentauthor" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 14],
    "documentlocation" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 15],
    "homephone" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 20],
    "secretary" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 21],
    "othermailbox" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 22],
    "lastmodifiedtime" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 23],
    "lastmodifiedby" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 24],
    "domaincomponent" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 25],
    "arecord" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 26],
    "mdrecord" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 27],
    "mxrecord" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 28],
    "nsrecord" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 29],
    "soarecord" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 30],
    "cnamerecord" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 31],
    "associateddomain" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 37],
    "associatedname" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 38],
    "homepostaladdress" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 39],
    "personaltitle" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 40],
    "mobiletelephonenumber" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 41],
    "pagertelephonenumber" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 42],
    "friendlycountryname" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 43],
    "uniqueidentifier" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 44],
    "organizationalstatus" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 45],
    "janetmailbox" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 46],
    "mailpreferenceoption" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 47],
    "buildingname" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 48],
    "dsaquality" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 49],
    "singlelevelquality" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 50],
    "subtreeminimumquality" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 51],
    "subtreemaximumquality" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 52],
    "personalsignature" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 53],
    "ditredirect" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 54],
    "audio" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 55],
    "documentpublisher" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 56],
    "jpegphoto" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 60],

    // COSINE short names
    "uid" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 1],
    "mail" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 3],
    "drink" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 5],
    "dc" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 25],
    "mobile" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 41],
    "pager" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 42],
    "co" => &[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1, 43],

    // PKIX PDA attributes
    "dateofbirth" => &[43, 6, 1, 5, 5, 7, 9, 1],
    "placeofbirth" => &[43, 6, 1, 5, 5, 7, 9, 2],
    "gender" => &[43, 6, 1, 5, 5, 7, 9, 3],
    "countryofcitizenship" => &[43, 6, 1, 5, 5, 7, 9, 4],
    "countryofresidence" => &[43, 6, 1, 5, 5, 7, 9, 5],

};

pub struct DefaultX500ValueParser;

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

impl ParseX500AttributeType for DefaultX500ValueParser {
    #[inline]
    fn attr_type_name_to_oid(&self, s: &str) -> Option<OBJECT_IDENTIFIER> {
        OIDS_BY_NAME
            .get(s.to_ascii_lowercase().as_str())
            // Fine to use unsafe here, because these are hard-coded values we know are correct.
            .map(|bs| unsafe { OBJECT_IDENTIFIER::from_x690_encoding_unchecked(bs.to_vec()) })
    }
}

// TODO: Should this just return ASN1Error instead?
// Exported separately so it can be used by other implementations, if desired.
fn parse_value(
    attr_type: &OBJECT_IDENTIFIER,
    s: &str,
) -> Result<Option<X690Element>, std::fmt::Error> {
    if s.starts_with("#") {
        let bytes = hex::decode(&s[1..]).map_err(|_| std::fmt::Error)?;
        let cst = BER.decode_from_slice(&bytes).map_err(|_| std::fmt::Error)?;
        return Ok(Some(cst.1));
    }
    let x690_slice = attr_type.as_x690_slice();
    let x690_len = x690_slice.len();
    if x690_len == 3 && x690_slice.starts_with(&[0x55, 4]) {
        match x690_slice.last().unwrap() {
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

            0 // objectClass
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

            // DistinguishedName syntax:
            1 // aliasedEntryName
            | 31 // member
            | 32 // owner
            | 33 // roleOccupant
            | 34 // seeAlso
            | 49 // distinguishedName
            => {
                // TODO: Document rationale.
                return Ok(None);
            },

            // BIT STRING
            45 // uniqueIdentifier
            | 90 // uii
            | 91 // epc
            => {
                let starts_with_quote = s
                    .as_bytes()
                    .first()
                    .is_some_and(|f| *f == b'\'');
                if !starts_with_quote || !s.ends_with("'B") {
                    return Err(std::fmt::Error);
                }
                let bs = BIT_STRING::from_bin(&s[1..s.len()-2]);
                return Ok(Some(BER.encode_bit_string(&bs).map_err(|_| std::fmt::Error)?));
            }

            // Postal Address:
            16 // postalAddress
            | 26 // registeredAddress
            => {
                let lines_count = s.split('$').count() + 1;
                let mut address: Vec<X690Element> = Vec::with_capacity(lines_count);
                let lines = parse_postal_address(s)
                    .map(|line| BER.encode_utf8_string(line.as_ref()));
                for maybe_line in lines {
                    let line = maybe_line.map_err(|_| std::fmt::Error)?;
                    address.push(line);
                }
                return Ok(Some(X690Element::new(
                    Tag::new(wildboar_asn1::TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                    x690::X690Value::Constructed(Arc::new(address)),
                )));
            }

            _ => return Ok(None),
        }
    }

    if x690_len == 4 && x690_slice.starts_with(&[97, 1, 2]) {
        return match x690_slice.last().unwrap() {
            0..=2 => { // oidC1, oidC2, oidC
                let i: u128 = s.parse().map_err(|_| std::fmt::Error)?;
                Ok(Some(BER.encode_u128(i).map_err(|_| std::fmt::Error)?))
            },
            _ => Ok(None),
        };
    }
    if x690_len == 10
        && x690_slice.starts_with(&[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1])
    {
        match x690_slice.last().unwrap() {

            // DirectoryString
            1 // userid / uid
            | 2 // textEncodedORAddress
            | 4 // info
            | 5 // favouriteDrink
            | 6 // roomNumber
            | 8 // userClass
            | 9 // host
            | 11 // documentIdentifier
            | 12 // documentTitle
            | 13 // documentVersion
            | 15 // documentLocation
            | 40 // personalTitle
            | 43 // friendlyCountryName
            | 44 // uniqueIdentifier
            | 45 // organizationalStatus
            | 48 // buildingName
            | 56 // documentPublisher
            => return Ok(Some(BER.encode_utf8_string(s).map_err(|_| std::fmt::Error)?)),

            // IA5String
            3 // rfc822mailbox
            | 25..=31 // domainComponent, aRecord, mDRecord, mXRecord, nSRecord, cNAMERecord
            | 37 // associatedDomain
            | 46 // janetMailbox
            => return Ok(Some(BER.encode_ia5_string(s).map_err(|_| std::fmt::Error)?)),

            // DN
            10 // manager
            | 14 // documentAuthor
            | 21 // secretary
            | 24 // lastModifiedBy
            | 38 // associatedName
            | 54 // dITRedirect
            => {
                // TODO: Document rationale.
                return Ok(None);
            }

            // PS
            20 // homePhone
            | 41 // mobileTelephoneNumber
            | 42 // pagerTelephoneNumber
            => return Ok(Some(BER.encode_printable_string(s).map_err(|_| std::fmt::Error)?)),

            22 // otherMailbox
            => {
                let (mbt, mb) = s.split_once('$').ok_or(std::fmt::Error)?;
                let mbt = BER.encode_printable_string(mbt).map_err(|_| std::fmt::Error)?;
                let mb = BER.encode_ia5_string(mb).map_err(|_| std::fmt::Error)?;
                return Ok(Some(X690Element::new(
                    Tag::new(wildboar_asn1::TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                    x690::X690Value::Constructed(Arc::new(vec![ mbt, mb ])),
                )));
            }

            // UTCTime
            23 // lastModifiedTime
            => {
                let t = UTCTime::from_str(s).map_err(|_| std::fmt::Error)?;
                return Ok(Some(BER.encode_utc_time(&t).map_err(|_| std::fmt::Error)?));
            }

            // PostalAddress
            39 // homePostalAddress
            => {
                // FIXME: Handle LDAP value escaping as well.
                let lines_count = s.split('$').count() + 1;
                let mut address: Vec<X690Element> = Vec::with_capacity(lines_count);
                let lines = parse_postal_address(s)
                    .map(|line| BER.encode_utf8_string(line.as_ref()));
                for maybe_line in lines {
                    let line = maybe_line.map_err(|_| std::fmt::Error)?;
                    address.push(line);
                }
                return Ok(Some(X690Element::new(
                    Tag::new(wildboar_asn1::TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                    x690::X690Value::Constructed(Arc::new(address)),
                )));
            }

            _ => return Ok(None),
        }
    }
    Ok(None)

}

impl ParseX500Value<X690Element> for DefaultX500ValueParser {
    #[inline]
    fn parse_value(
        &self,
        attr_type: &OBJECT_IDENTIFIER,
        s: &str,
    ) -> Result<Option<X690Element>, std::fmt::Error> {
        parse_value(attr_type, s)
    }
}

// If T can parse attribute types and values, it can parse whole DNs
impl <T: ParseX500AttributeType + ParseX500Value<X690Element>> ParseX500DistinguishedName for T {

    fn parse_dn(&self, s: &str) -> Result<DistinguishedName, std::fmt::Error> {
        let parser = DefaultX500ValueParser{};
        let mut dn: DistinguishedName = Vec::with_capacity(10);
        for maybe_rdn in dn_from_str(s) {
            let mut rdn_iter = maybe_rdn.map_err(|_| std::fmt::Error)?;
            let mut rdn: RelativeDistinguishedName = Vec::with_capacity(5);
            for maybe_atav in rdn_iter {
                let atav = maybe_atav.map_err(|_| std::fmt::Error)?;
                let (attr_type, attr_val) = atav;
                // If the type starts with a digit, try parsing as a numeric
                // OID, otherwise, assume its a name.
                let attr_type = if attr_type.as_bytes().first().is_some_and(|c| c.is_ascii_digit()) {
                    OBJECT_IDENTIFIER::from_str(attr_type)
                        .map_err(|_| std::fmt::Error)?
                } else {
                    parser.attr_type_name_to_oid(attr_type)
                        .ok_or(std::fmt::Error)?
                };
                let unescaped = unescape_ldap_value_string_cow(attr_val).unwrap();
                let attr_val = unescaped.as_ref();
                let attr_val = parser.parse_value(&attr_type, attr_val).unwrap().unwrap();
                rdn.push(AttributeTypeAndValue::new(
                    attr_type,
                    attr_val,
                    vec![],
                ));
            }
            dn.push(rdn);
        }
        Ok(dn)
    }

}

impl<T> ParseX500DirectoryName for T
where
    T: ParseX500DistinguishedName,
{
    fn parse_x500_name(&self, s: &str) -> Result<Name, std::fmt::Error> {
        let key: String = s.chars()
            .take(12)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if key.starts_with("rdnsequence:") { // TODO: Use likely
            Ok(Name::rdnSequence(self.parse_dn(&s[12..])?))
        } else if key.starts_with("dnsname:") {
            Ok(Name::dnsName(s.split_at(8).1.to_owned()))
        } else if key.starts_with("oid:") {
            let oidname = OBJECT_IDENTIFIER::from_str(s.split_at(4).1)
                .map_err(|_| std::fmt::Error)?;
            Ok(Name::oid(oidname))
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl<T> ParseGeneralName for T
where
    T: ParseX500DirectoryName,
{
    /// This implementation parses:
    /// `directoryName.rdnSequence`
    /// `directoryName.oid` as a numeric OID (e.g. `2.5.4.3`)
    /// `ediPartyName` using a "#" to separate the assigner name and party name.
    /// `ipAddress` in any format that Rust's `IpAddr` supports
    /// `registeredID` as a numeric OID (e.g. `2.5.4.3`)
    fn parse_general_name(&self, s: &str) -> Result<GeneralName, std::fmt::Error> {
        let key: String = s.chars()
            .take(30)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if key.starts_with("othername:") {
            let s = s.split_at("othername:".len()).1;
            todo!() // FIXME:
        }
        else if key.starts_with("rfc822name:") {
            let s = s.split_at("rfc822name:".len()).1;
            return Ok(GeneralName::rfc822Name(s.to_owned()));
        }
        else if key.starts_with("dnsname:") {
            let s = s.split_at("dnsname:".len()).1;
            return Ok(GeneralName::dNSName(s.to_owned()));
        }
        else if key.starts_with("x400address:") {
            // TODO: Make a feature to support this.
            return Err(std::fmt::Error);
        }
        else if key.starts_with("directoryname:") {
            let s = s.split_at("directoryname:".len()).1;
            let dn = self.parse_x500_name(s)
                .map_err(|_| std::fmt::Error)?;
            return Ok(GeneralName::directoryName(dn));
        }
        else if key.starts_with("edipartyname:") {
            let s = s.split_at("edipartyname:".len()).1;
            let (assigner, party) = s.split_once('#').unwrap_or(("", s));
            let assigner_name = if assigner.len() > 0 {
                Some(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING),
                    X690Value::Primitive(Bytes::copy_from_slice(assigner.as_bytes())),
                ))
            } else {
                None
            };
            let edipn = EDIPartyName::new(
                assigner_name,
                X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING),
                    X690Value::Primitive(Bytes::copy_from_slice(party.as_bytes())),
                ),
                vec![],
            );
            return Ok(GeneralName::ediPartyName(edipn));
        }
        else if key.starts_with("uniformresourceidentifier:") {
            let s = s.split_at("uniformresourceidentifier:".len()).1;
            return Ok(GeneralName::uniformResourceIdentifier(s.to_owned()));
        }
        else if key.starts_with("ipaddress:") {
            let s = s.split_at("ipaddress:".len()).1;
            todo!() // TODO: After you make iPAddress an `std::net::IpAddr`.
        }
        else if key.starts_with("registeredid:") {
            let s = s.split_at("registeredid:".len()).1;
            let oidname = OBJECT_IDENTIFIER::from_str(s)
                .map_err(|_| std::fmt::Error)?;
            return Ok(GeneralName::registeredID(oidname));
        } else {
            return Err(std::fmt::Error);
        }
    }
}

#[cfg(test)]
mod tests {

    use wildboar_asn1::{oid, TagClass, OBJECT_IDENTIFIER, UNIV_TAG_BIT_STRING, UNIV_TAG_IA5_STRING, UNIV_TAG_INTEGER, UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_SEQUENCE, UNIV_TAG_UTF8_STRING};
    use ldapdn::parse::dn_from_str;
    use ldapdn::escape::unescape_ldap_value_string_cow;
    use x690::{X690Element, X690Value};
    use crate::parse::{ParseGeneralName, ParseX500DirectoryName, ParseX500DistinguishedName};
    use crate::PKI_Stub::{
        Name,
        GeneralName,
    };
    use super::{ParseX500AttributeType, ParseX500Value, DefaultX500ValueParser};

    #[test]
    fn parse_dn_1() {
        let parser = DefaultX500ValueParser{};
        let input = "givenName=Jonathan+sn=Wilbur,dc=wildboarsoftware,dc=com";

        let mut i = 0;
        for maybe_rdn in dn_from_str(input) {
            let rdn = maybe_rdn.unwrap();
            for maybe_atav in rdn {
                i += 1;
                let (attr_type, attr_val) = maybe_atav.unwrap();
                let attr_type = parser.attr_type_name_to_oid(attr_type).unwrap();
                let attr_val = parser.parse_value(&attr_type, attr_val).unwrap().unwrap();
                match i {
                    1 => {
                        assert_eq!(attr_type, oid!(2,5,4,42));
                        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
                        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_UTF8_STRING);
                        match attr_val.value {
                            X690Value::Primitive(x) => assert_eq!(x.as_ref(), b"Jonathan"),
                            _ => panic!(),
                        };
                    },
                    2 => {
                        assert_eq!(attr_type, oid!(2,5,4,4));
                        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
                        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_UTF8_STRING);
                        match attr_val.value {
                            X690Value::Primitive(x) => assert_eq!(x.as_ref(), b"Wilbur"),
                            _ => panic!(),
                        };
                    },
                    3 => {
                        assert_eq!(attr_type, oid!(0,9,2342,19200300,100,1,25));
                        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
                        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_IA5_STRING);
                        match attr_val.value {
                            X690Value::Primitive(x) => assert_eq!(x.as_ref(), b"wildboarsoftware"),
                            _ => panic!(),
                        };
                    },
                    4 => {
                        assert_eq!(attr_type, oid!(0,9,2342,19200300,100,1,25));
                        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
                        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_IA5_STRING);
                        match attr_val.value {
                            X690Value::Primitive(x) => assert_eq!(x.as_ref(), b"com"),
                            _ => panic!(),
                        };
                    },
                    _ => panic!(),
                };
            }
        }
    }

    #[test]
    fn parse_dn_2() {
        let parser = DefaultX500ValueParser{};
        // The first backslash has to be double escaped, because both LDAP DNs
        // and the postal address syntax use backslash-hex escapes.
        let input = "postalAddress=\\\\24\\,000\\,000 Sweepstakes$123 Main St.,st=FL,c=US";

        let dn: Vec<Vec<(OBJECT_IDENTIFIER, X690Element)>> = dn_from_str(input)
            .map(|maybe_rdn| maybe_rdn.unwrap()
                .map(|maybe_atav| maybe_atav.unwrap())
                .map(|(attr_type, attr_val)| {
                    let attr_type = parser.attr_type_name_to_oid(attr_type).unwrap();
                    let unescaped = unescape_ldap_value_string_cow(attr_val).unwrap();
                    let attr_val = unescaped.as_ref();
                    let attr_val = parser.parse_value(&attr_type, attr_val).unwrap().unwrap();
                    (attr_type, attr_val)
                })
                .collect())
            .collect();
        let (attr_type, attr_val) = &dn[0][0];
        assert_eq!(*attr_type, oid!(2,5,4,16));
        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_SEQUENCE);
        let components = match &attr_val.value {
            X690Value::Constructed(c) => c,
            _ => panic!(),
        };
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(components[0].tag.tag_number, UNIV_TAG_UTF8_STRING);
        assert_eq!(components[1].tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(components[1].tag.tag_number, UNIV_TAG_UTF8_STRING);
        match &components[0].value {
            X690Value::Primitive(s) => assert_eq!(s.as_ref(), b"$,000,000 Sweepstakes"),
            _ => panic!(),
        };
        match &components[1].value {
            X690Value::Primitive(s) => assert_eq!(s.as_ref(), b"123 Main St."),
            _ => panic!(),
        };
    }

    #[test]
    fn parse_dn_3() {
        let parser = DefaultX500ValueParser{};
        let input = "oidC=5,epc='110010101110001'B,communicationsNetwork=1.1.4.5";

        let dn: Vec<Vec<(OBJECT_IDENTIFIER, X690Element)>> = dn_from_str(input)
            .map(|maybe_rdn| maybe_rdn.unwrap()
                .map(|maybe_atav| maybe_atav.unwrap())
                .map(|(attr_type, attr_val)| {
                    let attr_type = parser.attr_type_name_to_oid(attr_type).unwrap();
                    let attr_val = parser.parse_value(&attr_type, attr_val).unwrap().unwrap();
                    (attr_type, attr_val)
                })
                .collect())
            .collect();
        let (attr_type, attr_val) = &dn[0][0];
        assert_eq!(*attr_type, oid!(2,17,1,2,2));
        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_INTEGER);
        match &attr_val.value {
            X690Value::Primitive(x) => assert_eq!(x.as_ref(), &[5]),
            _ => panic!(),
        };

        let (attr_type, attr_val) = &dn[1][0];
        assert_eq!(*attr_type, oid!(2,5,4,91));
        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_BIT_STRING);
        match &attr_val.value {
            X690Value::Primitive(x) => assert_eq!(x.as_ref(), &[1, 0b11001010, 0b11100010]),
            _ => panic!(),
        };

        let (attr_type, attr_val) = &dn[2][0];
        assert_eq!(*attr_type, oid!(2,5,4,67));
        assert_eq!(attr_val.tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(attr_val.tag.tag_number, UNIV_TAG_OBJECT_IDENTIFIER);
        match &attr_val.value {
            X690Value::Primitive(x) => assert_eq!(x.as_ref(), &[41, 4, 5]),
            _ => panic!(),
        };
    }

    #[test]
    fn parse_dn_4() {
        let parser = DefaultX500ValueParser{};
        let input = "cn=Jonathan Wilbur,st=FL,c=US";
        let dn = parser.parse_dn(input).unwrap();
        assert_eq!(dn.len(), 3);
    }

    #[test]
    fn parse_dir_name_1() {
        let parser = DefaultX500ValueParser{};
        let input = "rdnSequence:cn=Jonathan Wilbur,st=FL,c=US";
        let dn = parser.parse_x500_name(input).unwrap();
        match dn {
            Name::rdnSequence(rdns) => {
                assert_eq!(rdns.len(), 3);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_dir_name_2() {
        let parser = DefaultX500ValueParser{};
        let input = "oid:2.5.4.3";
        let dn = parser.parse_x500_name(input).unwrap();
        match dn {
            Name::oid(o) => {
                assert_eq!(o.as_x690_slice(), &[0x55, 4, 3]);
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_dir_name_3() {
        let parser = DefaultX500ValueParser{};
        let input = "DNSNAME:wildboarsoftware.com";
        let dn = parser.parse_x500_name(input).unwrap();
        match dn {
            Name::dnsName(dnsname) => {
                assert_eq!(dnsname.as_str(), "wildboarsoftware.com");
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_gen_name_1() {
        let parser = DefaultX500ValueParser{};
        let input = "rfc822Name:jonathan@wilbur.space";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::rfc822Name(n) => {
                assert_eq!(n.as_str(), "jonathan@wilbur.space");
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_gen_name_2() {
        let parser = DefaultX500ValueParser{};
        let input = "dNSName:wildboarsoftware.com";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::dNSName(n) => {
                assert_eq!(n.as_str(), "wildboarsoftware.com");
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_gen_name_4() {
        let parser = DefaultX500ValueParser{};
        let input = "directoryName:oid:2.5.4.3";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::directoryName(n) => match n {
                Name::oid(oidname) => assert_eq!(oidname.as_x690_slice(), &[0x55, 4, 3]),
                _ => panic!(),
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_gen_name_5() {
        let parser = DefaultX500ValueParser{};
        let input = "ediPartyName:asdf#zxcv";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::ediPartyName(n) => {
                let name_assigner = n.nameAssigner.unwrap();
                let party_name = n.partyName;
                assert_eq!(name_assigner.content_octets().unwrap().as_ref(), b"asdf");
                assert_eq!(party_name.content_octets().unwrap().as_ref(), b"zxcv");
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_gen_name_6() {
        let parser = DefaultX500ValueParser{};
        let input = "ediPartyName:zxcv";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::ediPartyName(n) => {
                let party_name = n.partyName;
                assert!(n.nameAssigner.is_none());
                assert_eq!(party_name.content_octets().unwrap().as_ref(), b"zxcv");
            },
            _ => panic!(),
        };
    }

    #[test]
    fn parse_gen_name_7() {
        let parser = DefaultX500ValueParser{};
        let input = "uniformResourceIdentifier:https://wildboarsoftware.com";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::uniformResourceIdentifier(n) => {
                assert_eq!(n.as_str(), "https://wildboarsoftware.com");
            },
            _ => panic!(),
        };
    }

    // TODO:
    // #[test]
    // fn parse_gen_name_8() {
    //     let parser = DefaultX500ValueParser{};
    //     let input = "iPAddress:1.2.3.4";
    //     let gn = parser.parse_general_name(input).unwrap();
    //     match gn {
    //         GeneralName::iPAddress(ip) => {
    //             assert_eq!(ip, todo!());
    //         },
    //         _ => panic!(),
    //     };
    // }

    #[test]
    fn parse_gen_name_9() {
        let parser = DefaultX500ValueParser{};
        let input = "registeredID:2.5.4.3";
        let gn = parser.parse_general_name(input).unwrap();
        match gn {
            GeneralName::registeredID(n) => {
                assert_eq!(n.as_x690_slice(), &[0x55, 4, 3]);
            },
            _ => panic!(),
        };
    }

    // TODO: otherName
    // TODO: x400Address

}
