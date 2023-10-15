use crate::PkiPmiExternalDataTypes::{
    UniversalOrBMPString,
    UniversalOrBMPString_character_encoding,
};

// UniversalOrBMPString{INTEGER:ub-string-length} ::= SET {
//     character-encoding     CHOICE {
//       two-octets             BMPString(SIZE (1..ub-string-length)),
//       four-octets            UniversalString(SIZE (1..ub-string-length))},
//     iso-639-language-code  PrintableString(SIZE (2 | 5)) OPTIONAL }
impl UniversalOrBMPString {

    pub fn to_str (&self) -> &str {
        match self.character_encoding {
            UniversalOrBMPString_character_encoding::two_octets(ref s) => s,
            UniversalOrBMPString_character_encoding::four_octets(ref s) => s,
        }
    }

}

impl From<String> for UniversalOrBMPString_character_encoding {

    fn from(value: String) -> Self {
        let is_bmp = value.chars().all(|c| c <= '\u{FFFF}');
        if is_bmp {
            UniversalOrBMPString_character_encoding::two_octets(value)
        } else {
            UniversalOrBMPString_character_encoding::four_octets(value)
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
            UniversalOrBMPString_character_encoding::two_octets(value.to_owned())
        } else {
            UniversalOrBMPString_character_encoding::four_octets(value.to_owned())
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
