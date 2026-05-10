pub mod asn1;
pub mod country;
pub mod display;
pub mod iter;
pub mod parse;
pub mod univ_or_bmp;

use std::fmt::Write;
use std::borrow::Cow;

use crate::asn1::{ORAddress, BuiltInStandardAttributes};

pub(crate) fn make_string_safe<'a> (s: &'a str, delim: char) -> Cow<'a, str> {
    let replacement = if delim == ' ' { "?" } else { " " }; // Still PrintableString
    if s.chars().any(|c| c == delim || c.is_control()) {
        return Cow::Owned(s.replace(delim, replacement));
    }
    Cow::Borrowed(s)
}

// impl ORAddress {
//     #[inline]
//     pub fn new(
//         built_in_standard_attributes: BuiltInStandardAttributes,
//         built_in_domain_defined_attributes: OPTIONAL<BuiltInDomainDefinedAttributes>,
//         extension_attributes: OPTIONAL<ExtensionAttributes>,
//     ) -> Self {
//         ORAddress {
//             built_in_standard_attributes,
//             built_in_domain_defined_attributes,
//             extension_attributes,
//         }
//     }

//     /// The delimiter, `delim`, should be either `;` or `/`, but this crate does
//     /// not enforce that in case you need something else.
//     ///
//     /// This DOES write the leading `/` if `/` is used as the delimiter.
//     ///
//     /// This DOES NOT write "X.400: " in front of the string.
//     pub fn fmt_compat(&self, delim: char, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // The recommended sequence is G=;I=;S=;O=;OU1=;OU2=;P=;A=;C=;
//         // Just for conciseness. :)
//         let builtin = &self.built_in_standard_attributes;
//         let ddas = self.built_in_domain_defined_attributes.as_deref();
//         let ext_attrs = self.extension_attributes.as_deref();
//         if delim == '/' {
//             f.write_char('/')?;
//         }
//         if let Some(pn) = builtin.personal_name.as_ref() {
//             if let Some(gn) = pn.given_name.as_ref() {
//                 f.write_str("G=")?;
//                 f.write_str(make_string_safe(gn.as_str(), delim).as_ref())?;
//                 f.write_char(';')?;
//             }
//             if let Some(i) = pn.initials.as_ref() {
//                 f.write_str("I=")?;
//                 f.write_str(make_string_safe(i.as_str(), delim).as_ref())?;
//                 f.write_char(';')?;
//             }
//             if let Some(i) = pn.initials.as_ref() {
//                 f.write_str("I=")?;
//                 f.write_str(make_string_safe(i.as_str(), delim).as_ref())?;
//                 f.write_char(';')?;
//             }
//         }
//         todo!()
//     }

//     pub fn fmt_universally(&self, delim: char, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // The recommended sequence is G=;I=;S=;O=;OU1=;OU2=;P=;A=;C=;
//         // Just for conciseness. :)
//         let builtin = &self.built_in_domain_defined_attributes;
//         let ddas = self.built_in_domain_defined_attributes.as_deref();
//         let ext_attrs = self.extension_attributes.as_deref();
//         if delim == '/' {
//             f.write_char('/')?;
//         }

//         todo!()
//     }
// }
