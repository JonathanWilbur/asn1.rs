use crate::asn1::{
    UniversalOrBMPString,
    UniversalOrBMPString_character_encoding,
};
use wildboar_asn1::{
    UniversalString,
    BMPString,
};
use std::str::FromStr;
use std::fmt::{Display, Write};

// UniversalOrBMPString{INTEGER:ub-string-length} ::= SET {
//     character-encoding     CHOICE {
//       two-octets             BMPString(SIZE (1..ub-string-length)),
//       four-octets            UniversalString(SIZE (1..ub-string-length))},
//     iso-639-language-code  PrintableString(SIZE (2 | 5)) OPTIONAL }
impl UniversalOrBMPString {

    pub fn to_string (&self) -> String {
        match self.character_encoding {
            UniversalOrBMPString_character_encoding::two_octets(ref s) => s.to_string_lossy(),
            UniversalOrBMPString_character_encoding::four_octets(ref s) => s.to_string_lossy(),
        }
    }

    pub fn safe_fmt(&self, delim: char, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let replacement = if delim == ' ' { '?' } else { ' ' };
        match &self.character_encoding {
            UniversalOrBMPString_character_encoding::two_octets(s) => {
                let chars = s.0
                    .iter()
                    .map(|c| char::from_u32(*c as u32).unwrap_or(replacement))
                    .map(|c| if c == delim { replacement } else { c });
                for c in chars {
                    f.write_char(c)?;
                }
            },
            UniversalOrBMPString_character_encoding::four_octets(s) => {
                let chars = s.0
                    .iter()
                    .map(|c| char::from_u32(*c).unwrap_or(replacement))
                    .map(|c| if c == delim { replacement } else { c });
                for c in chars {
                    f.write_char(c)?;
                }
            },
        }
        Ok(())
    }

}

impl From<String> for UniversalOrBMPString_character_encoding {

    fn from(value: String) -> Self {
        let is_bmp = value.chars().all(|c| c <= '\u{FFFF}');
        if is_bmp {
            UniversalOrBMPString_character_encoding::two_octets(BMPString::from_str(value.as_str()).unwrap())
        } else {
            UniversalOrBMPString_character_encoding::four_octets(UniversalString::from_str(value.as_str()).unwrap())
        }
    }

}

impl From<String> for UniversalOrBMPString {

    fn from(value: String) -> Self {
        UniversalOrBMPString{
            character_encoding: value.into(),
            iso_639_language_code: None,
        }
    }

}


impl From<&str> for UniversalOrBMPString_character_encoding {

    fn from(value: &str) -> Self {
        let is_bmp = value.chars().all(|c| c <= '\u{FFFF}');
        if is_bmp {
            UniversalOrBMPString_character_encoding::two_octets(BMPString::from_str(value).unwrap())
        } else {
            UniversalOrBMPString_character_encoding::four_octets(UniversalString::from_str(value).unwrap())
        }
    }

}

impl From<&str> for UniversalOrBMPString {

    fn from(value: &str) -> Self {
        UniversalOrBMPString{
            character_encoding: value.into(),
            iso_639_language_code: None,
        }
    }

}

const UNICODE_REPLACEMENT_CHAR: char = '\u{FFFD}';

impl Display for UniversalOrBMPString_character_encoding {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniversalOrBMPString_character_encoding::two_octets(s) => {
                let chars = s.0
                    .iter()
                    .map(|c| char::from_u32(*c as u32).unwrap_or(UNICODE_REPLACEMENT_CHAR))
                    .filter(|c| !c.is_control());
                for c in chars {
                    f.write_char(c)?;
                }
            },
            UniversalOrBMPString_character_encoding::four_octets(s) => {
                let chars = s.0
                    .iter()
                    .map(|c| char::from_u32(*c).unwrap_or(UNICODE_REPLACEMENT_CHAR))
                    .filter(|c| !c.is_control());
                for c in chars {
                    f.write_char(c)?;
                }
            },
        }
        Ok(())
    }

}

impl Display for UniversalOrBMPString {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.character_encoding.fmt(f)
    }

}
