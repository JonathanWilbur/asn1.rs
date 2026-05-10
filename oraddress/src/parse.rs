use std::str::FromStr;
use presentation_address::PresentationAddress;
use wildboar_asn1::{is_numeric_char, is_printable_char};
use x690::{ber::BER, X690Codec};

use crate::asn1::{
    BuiltInDomainDefinedAttribute, BuiltInStandardAttributes, CountryName, ExtendedNetworkAddress, ExtendedNetworkAddress_e163_4_address, ORAddress, PDSParameter, PersonalName, PhysicalDeliveryCountryName, PostalCode, TeletexDomainDefinedAttribute, TeletexPersonalName, TerminalType, UnformattedPostalAddress, UniversalDomainDefinedAttribute, UniversalOrBMPString, UniversalPersonalName
};

impl FromStr for BuiltInDomainDefinedAttribute {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut last_was_eq: bool = false;
        let mut replace_needed: bool = false;
        let mut typestr = &s[0..0];
        let mut valuestr = &s[0..0];
        for (i, c) in s.char_indices() {
            if c == '=' {
                if last_was_eq {
                    replace_needed = true;
                } else {
                    typestr = &s[0..i-1];
                    valuestr = &s[i..];
                    break;
                }
                continue;
            }
            last_was_eq = false;
        }
        if typestr.len() == 0 || valuestr.len() == 0 {
            return Err(());
        }
        let typestr: String = if replace_needed {
            typestr.replace("==", "=")
        } else {
            typestr.to_owned()
        };
        Ok(BuiltInDomainDefinedAttribute{
            type_: typestr.as_str().try_into().map_err(|_| ())?,
            value: valuestr.try_into().map_err(|_| ())?,
        })
    }

}
impl FromStr for CountryName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 3 && s.bytes().all(|b| b.is_ascii_digit()) {
            let dcc = u16::from_str(s).map_err(|_| ())?;
            return Ok(CountryName::x121_dcc_code(dcc));
        }
        if s.len() == 2 && s.bytes().all(|b| b.is_ascii_alphabetic()) {
            let sbytes = s.as_bytes();
            let isocc: [u8; 2] = [sbytes[0], sbytes[1]];
            return Ok(CountryName::iso_3166_alpha2_code(isocc));
        }
        Err(())
    }

}


/// ExtendedNetworkAddress  ::=  CHOICE {
///   e163-4-address    SEQUENCE {
///     number       [0]  NumericString(SIZE (1..ub-e163-4-number-length)),
///     sub-address  [1]  NumericString(SIZE (1..ub-e163-4-sub-address-length))
///                   OPTIONAL},
///   psap-address [0]  PresentationAddress }
impl FromStr for ExtendedNetworkAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("+")
            || s.bytes().next().is_some_and(|b| is_numeric_char(b)) {
            // FIXME: Filter out non-digits
            return match s.split_once('x') {
                Some((num, subaddr)) => {
                    let num = BER.decode_numeric_string_value(num.as_bytes())
                        .map_err(|_| ())?;
                    let subaddr = BER.decode_numeric_string_value(subaddr.as_bytes())
                        .map_err(|_| ())?;
                    Ok(ExtendedNetworkAddress::e163_4_address(ExtendedNetworkAddress_e163_4_address::new(
                        ns,
                        Some(subaddr),
                    )))
                },
                None => {
                    let ns = BER.decode_numeric_string_value(s.as_bytes())
                        .map_err(|_| ())?;
                    Ok(ExtendedNetworkAddress::e163_4_address(ExtendedNetworkAddress_e163_4_address::new(
                        ns,
                        None,
                    )))
                },
            };
        } else {
            let paddr = PresentationAddress::from_str(s).map_err(|_| ())?;
            Ok(ExtendedNetworkAddress::psap_address(paddr))
        }
    }

}

// PDSParameter ::= SET {
//   printable-string  PrintableString(SIZE (1..ub-pds-parameter-length)) OPTIONAL,
//   teletex-string    TeletexString(SIZE (1..ub-pds-parameter-length)) OPTIONAL }
impl FromStr for PDSParameter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.bytes().all(|b| is_printable_char(b)) {
            Ok(PDSParameter::new(
                Some(s.to_owned()),
                None,
            ))
        } else if s.is_ascii() { // TODO: Support full conversion to Teletex
            Ok(PDSParameter::new(
                None,
                Some(s.as_bytes().to_vec()),
            ))
        } else {
            Err(())
        }
    }

}
impl FromStr for PersonalName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret: PersonalName = PersonalName::new(
            "".try_into().unwrap(),
            None,
            None,
            None,
        );
        for field in s.split(';') {
            let (label, value) = match field.split_once('=') {
                Some(x) => x,
                None => continue, // Just ignore empty or malformed fields.
            };
            match label {
                "G" => {
                    if ret.given_name.is_some() {
                        return Err(());
                    }
                },
                "I" => {
                    if ret.initials.is_some() {
                        return Err(());
                    }
                },
                "S" => {
                    if ret.surname.len() > 0 {
                        return Err(());
                    }
                },
                "Q" => {
                    if ret.generation_qualifier.is_some() {
                        return Err(());
                    }
                },
                _ => (),
            };
        }
        if ret.surname.len() == 0 {
            return Err(());
        }
        Ok(ret)
    }

}
impl FromStr for PhysicalDeliveryCountryName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for PostalCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for TeletexDomainDefinedAttribute {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for TeletexPersonalName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}

impl FromStr for UnformattedPostalAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for UniversalDomainDefinedAttribute {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for UniversalOrBMPString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for UniversalPersonalName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}
impl FromStr for BuiltInStandardAttributes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

}

// impl FromStr for ORAddress {
//     type Err = ();

//     fn from_str(mut s: &str) -> Result<Self, Self::Err> {
//         let delim = if s.bytes().next().is_some_and(|b| *b == b'/') {
//             s = &s[1..];
//             '/'
//         } else {
//             ';'
//         };
//         for atav in s.split(delim) {
//             let trimmed = atav.trim();
//             if trimmed.len() == 0 {
//                 // Just ignore ";;"
//                 continue;
//             }
//             let (typestr, valuestr) = trimmed.split_once('=').ok_or(())?;
//             match typestr {
//                 "G" => { // Given Name

//                 }
//                 "I" => { // Initial

//                 }
//                 "S" => { // Surname

//                 }
//                 "Q" => { // Generation Qualifier

//                 }
//                 "CN" => { // Common Name

//                 }
//                 "O" => { // Organization

//                 }
//                 "OU1" => { // Organizational Unit 1

//                 }
//                 "OU2" => { // Organizational Unit 2

//                 }
//                 "OU3" => { // Organizational Unit 3

//                 }
//                 "OU4" => { // Organizational Unit 4

//                 }
//                 "P" => { // Private Management Domain Name

//                 }
//                 "A" => { // Administration Management Domain Name

//                 }
//                 "C" => { // Country

//                 }

//             // Physical Delivery Personal Name        PD-person         PD-PN
//             // Extension of Postal O/R Address
//             // Components                             PD-ext.address    PD-EA
//             // Extension of Physical Delivery Address
//             // Components                             PD-ext.delivery   PD-ED
//             // Physical Delivery Office Number        PD-office number  PD-OFN
//             // Physical Delivery Office Name          PD-office         PD-OF
//             // Physical Delivery Organization Name    PD-organization   PD-O
//             // Street Address                         PD-street         PD-S
//             // Unformatted Postal Address             PD-address        PD-A1
//             //                                                         PD-A2
//             // (there are individual labels for                         PD-A3
//             // each line of the address)                                PD-A4
//             //                                                         PD-A5
//             //                                                         PD-A6
//             // Unique Postal Name                     PD-unique         PD-U
//             // Local Postal Attributes                PD-local          PD-L
//             // Postal Restante Address                PD-restante       PD-R
//             // Post Office Box Address                PD-box            PD-B
//             // Postal Code                            PD-code           PD-PC
//             // Physical Delivery Service Name         PD-service        PD-SN
//             // Physical Delivery Country Name         PD-country        PD-C

//             // X.121 Network Address                  X.121             X.121
//             // E.163/E.164 Network Address            ISDN              ISDN
//             // PSAP Network Address                   PSAP              PSAP
//             // User Agent Numeric ID                  N-ID              N-ID
//             // Terminal Identifier                    T-ID              T-ID
//             // Terminal Type                          T-TY              T-TY
//             // Domain Defined Attribute               DDA:<type>        DDA:<type>
//             }
//         }
//     }

// }
